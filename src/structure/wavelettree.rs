use futures::prelude::*;
use futures::sync::mpsc::channel;
use futures::sync::oneshot::spawn;
use tokio::prelude::*;
use tokio::executor::DefaultExecutor;
use super::logarray::*;
use super::bitarray::*;
use super::bitindex::*;
use super::storage::*;

#[derive(Clone)]
pub struct WaveletTree<'a> {
    bits: BitIndex<'a>,
    layer_ranks: Vec<u64>
}

impl<'a> WaveletTree<'a> {
    pub fn from_parts(bits: BitIndex<'a>, num_layers: usize) -> WaveletTree<'a> {
        if bits.len() % num_layers != 0 {
            panic!("the bitarray length is not a multiple of the number of layers");
        }

        let mut layer_ranks = Vec::with_capacity(num_layers);
        let layer_width = bits.len() / num_layers;

        let mut last_rank = 0;
        for i in 1..=num_layers {
            let rank = bits.rank((i*layer_width) as u64);
            layer_ranks.push(rank - last_rank);
            last_rank = rank;
        }

        WaveletTree { bits, layer_ranks }
    }


    pub fn decode(&self) -> impl Iterator<Item=u64> {
        self.decode_from(0)
    }

    pub fn decode_one(&self, index: usize) -> u64 {
        self.decode_from(index).nth(0).unwrap()
    }

    pub fn decode_from(&self, index: usize) -> impl Iterator<Item=u64> {
        if index >= self.bits.len()/self.layer_ranks.len() {
            panic!("index too high");
        }
        vec![0].into_iter()
    }
}

fn build_wavelet_fragment<S:Stream<Item=u64,Error=std::io::Error>, W:AsyncWrite>(stream: S, write: BitArrayFileBuilder<W>, alphabet: usize, layer: usize, fragment: usize) -> impl Future<Item=BitArrayFileBuilder<W>,Error=std::io::Error> {
    let alphabet_start = (alphabet / (layer + 1) * fragment) as u64;
    let alphabet_end = (alphabet / (layer + 1) * (fragment+1)) as u64;
    let alphabet_mid = ((alphabet_start+alphabet_end)/2) as u64;

    stream.fold(write, move |w, num| {
        let result: Box<Future<Item=BitArrayFileBuilder<W>,Error=std::io::Error>> =
        if num >= alphabet_start && num < alphabet_end {
            Box::new(w.push(num >= alphabet_mid))
        }
        else {
            Box::new(future::ok(w))
        };

        result
    })
}

pub fn build_wavelet_tree<FLoad: 'static+FileLoad+Clone, F1: 'static+FileLoad+FileStore, F2: 'static+FileStore, F3: 'static+FileStore>(source: FLoad, destination_bits: F1, destination_blocks: F2, destination_sblocks: F3) -> impl Future<Item=(),Error=std::io::Error> {
    let bits = BitArrayFileBuilder::new(destination_bits.open_write());

    logarray_file_get_width(source.clone())
        .map(|width| (width as usize, 2_usize.pow(width as u32)))
        .and_then(|(num_layers, alphabet_size)| stream::iter_ok::<_,std::io::Error>((0..num_layers).map(|layer| (0..layer).map(move |fragment| (layer, fragment))).flatten())
                  .fold(bits, move |b, (layer, fragment)| {
                      open_logarray_stream(source.clone())
                          .and_then(move |stream| build_wavelet_fragment(stream, b, alphabet_size, layer, fragment))
                  })
                  .and_then(|b| b.finalize())
                  .and_then(move |_| build_bitindex(destination_bits.open_read(), destination_blocks.open_write(), destination_sblocks.open_write()))
                  .map(|_|()))
}
