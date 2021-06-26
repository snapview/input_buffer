use criterion::*;
use input_buffer::*;
use std::io::{Read};
#[derive(Clone)]
struct MockInput {
    bytes: usize,
    chunk: Vec<u8>,
    limit: usize
}
impl MockInput {
    pub fn new(chunk: usize, limit: usize) -> Self {
        Self {
            bytes: 0,
            chunk: vec![0; chunk],
            limit
        }
    }
    pub fn total_len(&self) -> usize {
        self.limit
    }
}
impl Read for MockInput {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.bytes >= self.limit {
            Ok(0)
        } else {
            let size = std::cmp::min(self.chunk.len(), buf.len());
            self.bytes += size;
            buf[..size].copy_from_slice(&self.chunk[..size]);
            Ok(size)
        }
    }
}

fn decode(mut inp: MockInput) {
    let mut buf = InputBuffer::new();
    loop {
        if buf.read_from(&mut inp).ok() == Some(0) {
            break;
        }
    }
}

fn bench(c: &mut Criterion) {
    let inp = MockInput::new(1400, 1024 * 1024 * 24);
    let mut group = c.benchmark_group("throughput");
    group.throughput(Throughput::Bytes(inp.total_len() as u64));
    group.bench_function("decode", |b| b.iter(|| decode(inp.clone())));
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);