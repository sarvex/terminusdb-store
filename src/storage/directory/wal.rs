//! Write-Ahead Log
use crc::{crc32, Hasher32};
use super::*;

struct LabelSetRecord<M:AsRef<[u8]>+Clone> {
    data: M,
}

struct LabelSetEntry {
    label: String,
    layer: [u32;5]
}

impl<M:AsRef<[u8]>+Clone> LabelSetRecord<M> {
    fn index(&self) -> u64 {
        unimplemented!();
    }
    fn entries(&self) -> impl Iterator<Item=LabelSetEntry> {
        LabelSetEntryIterator {
            data: self.data.clone(),
            position: 0
        }
    }
}

struct LabelSetEntryIterator<M:AsRef<[u8]>+Clone> {
    data: M,
    position: usize
}

impl<M:AsRef<[u8]>+Clone> Iterator for LabelSetEntryIterator<M> {
    type Item = LabelSetEntry;

    fn next(&mut self) -> Option<LabelSetEntry> {
        unimplemented!();
    }
}

struct CheckpointRecord<M:AsRef<[u8]>+Clone> {
    data: M
}

impl<M:AsRef<[u8]>+Clone> CheckpointRecord<M> {
    fn index(&self) -> u64 {
        unimplemented!();
    }
}

enum WalRecord<M:AsRef<[u8]>+Clone> {
    LabelSet(LabelSetRecord<M>),
    Checkpoint(CheckpointRecord<M>)
}

impl<M:AsRef<[u8]>+Clone> WalRecord<M> {
    fn data(&self) -> &[u8] {
        match self {
            Self::LabelSet(r) => r.data.as_ref(),
            Self::Checkpoint(r) => r.data.as_ref()
        }
    }

    fn length(&self) -> usize {
        self.data().len()
    }
    fn checksum(&self) -> u32 {
        crc32::checksum_ieee(self.data())
    }
}

const WAL_FILE_NAME: &'static str = "wa.log";
struct WalFile {
    path: PathBuf
}

impl WalFile {
    fn last_record(&self) -> WalRecord<Vec<u8>> {
        unimplemented!();
    }
}
