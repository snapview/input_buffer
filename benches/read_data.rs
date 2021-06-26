use criterion::*;
use input_buffer::*;
use std::io::Read;

#[derive(Clone)]
struct MockInput {
    bytes: usize,
    chunk: Vec<u8>,
    limit: usize,
}

impl MockInput {
    pub fn new(chunk: usize, limit: usize) -> Self {
        Self {
            bytes: 0,
            chunk: vec![0; chunk],
            limit,
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
            let size = std::cmp::min(self.limit - self.bytes, std::cmp::min(self.chunk.len(), buf.len()));
            self.bytes += size;
            buf[..size].copy_from_slice(&self.chunk[..size]);
            Ok(size)
        }
    }
}

fn input_buffer(mut inp: MockInput) {
    let mut buf = InputBuffer::new();
    loop {
        if buf.read_from(&mut inp).ok() == Some(0) {
            break;
        }
    }
    assert_eq!(buf.as_cursor().get_ref().len(), inp.total_len());
}

fn extend_from_slice(mut inp: MockInput) {
    let mut data = Vec::new();
    let chunk = &mut [0; 8192];
    loop {
        match inp.read(chunk) {
            Ok(0) => { break; }
            Ok(n) => {
                data.extend_from_slice(&chunk[..n])
            }
            Err(_) => unreachable!()
        }
    }
    assert_eq!(data.len(), inp.total_len());
}


fn with_capacity(mut inp: MockInput) {
    let mut data = Vec::with_capacity(inp.total_len());
    let chunk = &mut [0; 8192];
    loop {
        match inp.read(chunk) {
            Ok(0) => { break; }
            Ok(n) => {
                data.extend_from_slice(&chunk[..n])
            }
            Err(_) => unreachable!()
        }
    }
    assert_eq!(data.len(), inp.total_len());
}

fn with_capacity_unsafe(mut inp: MockInput) {
    let mut data = Vec::with_capacity(inp.total_len());
    unsafe {
        data.set_len(inp.total_len());
    }
    let mut pos = 0;

    loop {
        match inp.read(&mut data[pos..]) {
            Ok(0) => { break; }
            Ok(n) => {
                pos += n;
            }
            Err(_) => unreachable!()
        }
    }
    unsafe {
        data.set_len(pos);
    }
    assert_eq!(data.len(), inp.total_len());
}

fn bench(c: &mut Criterion) {
    const DATA_SIZE: usize = 1024 * 1024 * 8;

    let inp = MockInput::new(1400, DATA_SIZE);
    let mut group = c.benchmark_group("throughput");
    group.throughput(Throughput::Bytes(inp.total_len() as u64));
    group.bench_function("input_buffer", |b| b.iter(|| input_buffer(inp.clone())));
    group.bench_function("extend_from_slice", |b| b.iter(|| extend_from_slice(inp.clone())));
    group.bench_function("with_capacity", |b| b.iter(|| with_capacity(inp.clone())));
    group.bench_function("with_capacity_unsafe", |b| b.iter(|| with_capacity_unsafe(inp.clone())));
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);