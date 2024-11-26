use criterion::{criterion_group, criterion_main, Criterion};
use tailcall_chunk::Chunk;

const N: usize = 10000;

fn bench_operations(c: &mut Criterion) {
    // Benchmark append operations
    c.benchmark_group("append")
        .bench_function("Chunk", |b| {
            b.iter(|| {
                let mut chunk = Chunk::default();
                for i in 0..N {
                    chunk = chunk.append(i.to_string());
                }

                chunk
            })
        })
        .bench_function("Vec", |b| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..N {
                    vec.push(i.to_string());
                }
                vec
            })
        });

    // Benchmark prepend operations
    c.benchmark_group("prepend")
        .bench_function("Chunk", |b| {
            b.iter(|| {
                let mut chunk = Chunk::default();
                for i in 0..N {
                    chunk = chunk.prepend(i.to_string());
                }
                chunk
            })
        })
        .bench_function("Vec", |b| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..N {
                    vec.insert(0, i.to_string());
                }
                vec
            })
        });

    // Benchmark concat operations
    c.benchmark_group("concat")
        .bench_function("Chunk", |b| {
            let chunk1: Chunk<_> = (0..N / 2).map(|i| i.to_string()).collect();
            let chunk2: Chunk<_> = (N / 2..N).map(|i| i.to_string()).collect();
            b.iter(|| chunk1.clone().concat(chunk2.clone()))
        })
        .bench_function("Vec", |b| {
            let vec1: Vec<_> = (0..N / 2).map(|i| i.to_string()).collect();
            let vec2: Vec<_> = (N / 2..N).map(|i| i.to_string()).collect();
            b.iter(|| {
                let mut result = vec1.clone();
                result.extend(vec2.iter().cloned());
                result
            })
        });

    // Benchmark clone operations
    c.benchmark_group("clone")
        .bench_function("Chunk", |b| {
            let chunk: Chunk<_> = (0..N).collect();
            b.iter(|| chunk.clone())
        })
        .bench_function("Vec", |b| {
            let vec: Vec<_> = (0..N).collect();
            b.iter(|| vec.clone())
        });

    // Benchmark from_iter operation
    c.benchmark_group("from_iter")
        .bench_function("Chunk", |b| {
            b.iter(|| Chunk::from_iter((0..N).into_iter()));
        })
        .bench_function("Vec", |b| b.iter(|| Vec::from_iter((0..N).into_iter())));
}

criterion_group!(benches, bench_operations);
criterion_main!(benches);
