//! storage traits that the builders and loaders can rely on

use tokio::prelude::*;
use tokio::fs::*;
use std::sync::{Arc,RwLock};
use std::io::{self,Seek, SeekFrom};
use std::path::PathBuf;
use std::marker::PhantomData;
use std::collections::HashMap;
use memmap::*;

pub trait FileStore {
    type Write: AsyncWrite;
    fn open_write(&self) -> Self::Write {
        self.open_write_from(0)
    }
    fn open_write_from(&self, offset: usize) -> Self::Write;
}

pub trait FileLoad {
    type Read: AsyncRead;
    type Map: AsRef<[u8]>;
    
    fn size(&self) -> usize;
    fn open_read(&self) -> Self::Read {
        self.open_read_from(0)
    }
    fn open_read_from(&self, offset: usize) -> Self::Read;
    fn map(&self) -> Self::Map;
}

pub struct MemoryBackedStoreWriter {
    vec: Arc<RwLock<Vec<u8>>>,
    pos: usize
}

impl Write for MemoryBackedStoreWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        let mut v = self.vec.write().unwrap();
        if v.len() - self.pos < buf.len() {
            v.resize(self.pos + buf.len(), 0);
        }

        v[self.pos..self.pos+buf.len()].copy_from_slice(buf);

        self.pos += buf.len();

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

impl AsyncWrite for MemoryBackedStoreWriter {
    fn shutdown(&mut self) -> Result<Async<()>, io::Error> {
        Ok(Async::Ready(()))
    }
}

pub struct MemoryBackedStoreReader {
    vec: Arc<RwLock<Vec<u8>>>,
    pos: usize
}

impl Read for MemoryBackedStoreReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let v = self.vec.read().unwrap();

        if self.pos >= v.len() {
            return Ok(0);
        }

        let slice = &v[self.pos..];
        if slice.len() >= buf.len() {
            buf.copy_from_slice(&slice[..buf.len()]);
            self.pos += buf.len();

            Ok(buf.len())
        }
        else {
            buf[..slice.len()].copy_from_slice(slice);
            self.pos += slice.len();

            Ok(slice.len())
        }
    }
}

impl AsyncRead for MemoryBackedStoreReader {
}

#[derive(Clone)]
pub struct MemoryBackedStore {
    vec: Arc<RwLock<Vec<u8>>>
}

impl MemoryBackedStore {
    pub fn new() -> MemoryBackedStore {
        MemoryBackedStore { vec: Default::default() }
    }
}

impl FileStore for MemoryBackedStore {
    type Write = MemoryBackedStoreWriter;

    fn open_write_from(&self, pos: usize) -> MemoryBackedStoreWriter {
        MemoryBackedStoreWriter { vec: self.vec.clone(), pos }
    }
}

impl FileLoad for MemoryBackedStore {
    type Read = MemoryBackedStoreReader;
    type Map = Vec<u8>;

    fn size(&self) -> usize {
        self.vec.read().unwrap().len()
    }

    fn open_read_from(&self, offset: usize) -> MemoryBackedStoreReader {
        MemoryBackedStoreReader { vec: self.vec.clone(), pos: offset }
    }

    fn map(&self) -> Vec<u8> {
        self.vec.read().unwrap().clone()
    }
}

pub struct FileBackedStore {
    path: PathBuf
}

impl FileBackedStore {
    pub fn new<P:Into<PathBuf>>(path: P) -> FileBackedStore {
        FileBackedStore { path: path.into() }
    }

    fn open_read_from_std(&self, offset: usize) -> std::fs::File {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        let mut file = options.open(&self.path).unwrap();

        file.seek(SeekFrom::Start(offset as u64)).unwrap();

        file
    }

}

impl FileLoad for FileBackedStore {
    type Read = File;
    type Map = Mmap;

    fn size(&self) -> usize {
        let m = std::fs::metadata(&self.path).unwrap();
        m.len() as usize
    }

    fn open_read_from(&self, offset: usize) -> File {
        let f = self.open_read_from_std(offset);

        File::from_std(f)
    }

    fn map(&self) -> Mmap {
        let f = self.open_read_from_std(0);

        // unsafe justification: we opened this file specifically to do memory mapping, and will do nothing else with it.
        unsafe { Mmap::map(&f) }.unwrap()

    }
}

impl FileStore for FileBackedStore {
    type Write = File;

    fn open_write_from(&self, offset: usize) -> File {
        let mut options = std::fs::OpenOptions::new();
        options.read(true).write(true).create(true);
        let mut file = options.open(&self.path).unwrap();

        file.seek(SeekFrom::Start(offset as u64)).unwrap();

        File::from_std(file)
    }
}

pub trait MultiFileStore {
    type FileBackend: FileLoad+FileStore;

    fn backend_for(&mut self, file: &str) -> Self::FileBackend;
    fn backend_for_ro(&self, file: &str) -> Option<ReadOnlyFile<Self::FileBackend>>;
}

pub struct MemoryBackedMultiFileStore {
    files: HashMap<String,MemoryBackedStore>
}

impl MultiFileStore for MemoryBackedMultiFileStore {
    type FileBackend = MemoryBackedStore;

    fn backend_for_ro(&self, file: &str) -> Option<ReadOnlyFile<MemoryBackedStore>> {
        self.files.get(file).map(|f|ReadOnlyFile::new(f.clone()))
    }

    fn backend_for(&mut self, file: &str) -> MemoryBackedStore {
        match self.files.get(file) {
            None => {
                let store = MemoryBackedStore::new();
                self.files.insert(file.to_owned(), store.clone());

                store
            },
            Some(f) => f.clone()
        }
    }
}

pub struct FileBackedMultiFileStore {
    directory: PathBuf
}

impl FileBackedMultiFileStore {
    pub fn new<P:Into<PathBuf>>(path: P) -> FileBackedMultiFileStore {
        let p:PathBuf = path.into();
        if !p.is_dir() {
            panic!("tried to create a FileBackedMultiFileStore with a path that did not point at a directory");
        }

        FileBackedMultiFileStore { directory: p }
    }
}

impl MultiFileStore for FileBackedMultiFileStore {
    type FileBackend = FileBackedStore;

    fn backend_for_ro(&self, file: &str) -> Option<ReadOnlyFile<FileBackedStore>> {
        let mut full_path = self.directory.clone();
        full_path.push(file);

        match full_path.is_file() {
            true => Some(ReadOnlyFile::new(FileBackedStore::new(full_path))),
            false => None
        }
    }

    fn backend_for(&mut self, file: &str) -> FileBackedStore {
        let mut full_path = self.directory.clone();
        full_path.push(file);

        FileBackedStore::new(full_path)
    }
}

pub struct ReadOnlyFile<T:FileLoad> {
    file: T
}

impl<T:FileLoad> ReadOnlyFile<T> {
    pub fn new(file: T) -> ReadOnlyFile<T> {
        ReadOnlyFile { file }
    }
}

impl<T:FileLoad> FileLoad for ReadOnlyFile<T> {
    type Read = T::Read;
    type Map = T::Map;

    fn size(&self) -> usize {
        self.file.size()
    }

    fn open_read_from(&self, offset: usize) -> Self::Read {
        self.file.open_read_from(offset)
    }

    fn map(&self) -> Self::Map {
        self.file.map()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::sync::oneshot::channel;

    #[test]
    fn write_and_read_memory_backed() {
        let file = MemoryBackedStore::new();

        let w = file.open_write();
        let buf = tokio::io::write_all(w,[1,2,3])
            .and_then(move |_| tokio::io::read_to_end(file.open_read(), Vec::new()))
            .map(|(_,buf)| buf)
            .wait()
            .unwrap();

        assert_eq!(vec![1,2,3], buf);
    }

    #[test]
    fn write_and_map_memory_backed() {
        let file = MemoryBackedStore::new();

        let w = file.open_write();
        tokio::io::write_all(w,[1,2,3])
            .wait()
            .unwrap();

        assert_eq!(vec![1,2,3], file.map());
    }

    #[test]
    fn write_and_read_file_backed() {
        let (tx,rx) = channel::<Result<Vec<u8>, std::io::Error>>();

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("foo");
        let file = FileBackedStore::new(file_path);

        let w = file.open_write();
        let task = tokio::io::write_all(w,[1,2,3])
            .and_then(move |_| tokio::io::read_to_end(file.open_read(), Vec::new()))
            .map(move |(_,buf)| buf)
            .then(|x| tx.send(x))
            .map(|_|())
            .map_err(|_|());

        tokio::run(task);
        let buf = rx.wait().unwrap();

        assert_eq!(vec![1,2,3], buf.unwrap());
    }

    #[test]
    fn write_and_map_file_backed() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("foo");
        let file = FileBackedStore::new(file_path);

        let w = file.open_write();
        let task = tokio::io::write_all(w,[1,2,3])
            .map(|_|())
            .map_err(|_|());

        tokio::run(task);

        assert_eq!(&vec![1,2,3][..], &file.map()[..]);
    }
}
