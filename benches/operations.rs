use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tailcall_chunk::Chunk;

const N: usize = 10000;

fn bench_operations(c: &mut Criterion) {
    // Benchmark append operations
    c.benchmark_group("append")
        .bench_function("chunk_append", |b| {
            b.iter(|| {
                let mut chunk = Chunk::default();
                for i in 0..10000 {
                    chunk = chunk.append(i.to_string());
                }

                black_box(chunk);
            })
        })
        .bench_function("vec_append", |b| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..10000 {
                    vec.push(i.to_string());
                }
                black_box(vec);
            })
        });

    // Benchmark prepend operations
    c.benchmark_group("prepend")
        .bench_function("chunk_prepend", |b| {
            b.iter(|| {
                let mut chunk = Chunk::default();
                for i in 0..10000 {
                    chunk = chunk.prepend(i.to_string());
                }
                black_box(chunk);
            })
        })
        .bench_function("vec_prepend", |b| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..10000 {
                    vec.insert(0, i.to_string());
                }
                black_box(vec);
            })
        });

    // Benchmark concat operations
    c.benchmark_group("concat")
        .bench_function("chunk_concat", |b| {
            let chunk1: Chunk<_> = (0..5000).map(|i| i.to_string()).collect();
            let chunk2: Chunk<_> = (5000..10000).map(|i| i.to_string()).collect();
            b.iter(|| {
                black_box(chunk1.clone().concat(chunk2.clone()));
            })
        })
        .bench_function("vec_concat", |b| {
            let vec1: Vec<_> = (0..5000).map(|i| i.to_string()).collect();
            let vec2: Vec<_> = (5000..10000).map(|i| i.to_string()).collect();
            b.iter(|| {
                let mut result = vec1.clone();
                result.extend(vec2.iter().cloned());
                black_box(result)
            })
        });

    // Benchmark clone operations
    c.benchmark_group("clone")
        .bench_function("chunk_clone", |b| {
            let chunk: Chunk<_> = (0..10000).collect();
            b.iter(|| {
                black_box(chunk.clone());
            })
        })
        .bench_function("vec_clone", |b| {
            let vec: Vec<_> = (0..10000).collect();
            b.iter(|| {
                black_box(vec.clone());
            })
        });

    // Benchmark from_iter operation
    c.benchmark_group("from_iter")
        .bench_function("Chunk", |b| {
            b.iter(|| Chunk::from_iter((0..N)));
        })
        .bench_function("Vec", |b| b.iter(|| Vec::from_iter((0..N))));
}

criterion_group!(benches, bench_operations);
criterion_main!(benches);
