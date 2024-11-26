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

                chunk.as_vec()
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
                chunk.as_vec()
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
            let part: Chunk<_> = (0..100).map(|i| i.to_string()).collect();
            b.iter(|| {
                let mut chunk = Chunk::default();
                for _ in 0..N {
                    chunk = chunk.concat(part.clone());
                }

                chunk.as_vec()
            })
        })
        .bench_function("Vec", |b| {
            let part: Vec<_> = (0..100).map(|i| i.to_string()).collect();
            b.iter(|| {
                let mut result = Vec::new();
                for _ in 0..N {
                    result.extend(part.iter().cloned());
                }

                result
            })
        });

    // Benchmark clone operations
    c.benchmark_group("clone")
        .bench_function("Chunk", |b| {
            let chunk: Chunk<_> = (0..N).collect();
            b.iter(|| chunk.clone().as_vec())
        })
        .bench_function("Vec", |b| {
            let vec: Vec<_> = (0..N).collect();
            b.iter(|| vec.clone())
        });

    // Benchmark from_iter operation
    c.benchmark_group("from_iter")
        .bench_function("Chunk", |b| {
            b.iter(|| Chunk::from_iter((0..N).into_iter()).as_vec());
        })
        .bench_function("Vec", |b| b.iter(|| Vec::from_iter((0..N).into_iter())));
}

criterion_group!(benches, bench_operations);
criterion_main!(benches);
