use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tailcall_chunk::Chunk;

fn bench_operations(c: &mut Criterion) {
    // Benchmark append operations
    c.benchmark_group("append")
        .bench_function("chunk_append", |b| {
            b.iter(|| {
                let mut chunk = Chunk::default();
                for i in 0..1000 {
                    chunk = chunk.append(i);
                }

                black_box(chunk);
            })
        })
        .bench_function("vec_append", |b| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..1000 {
                    vec.push(i);
                }
                black_box(vec);
            })
        });

    // Benchmark prepend operations
    c.benchmark_group("prepend")
        .bench_function("chunk_prepend", |b| {
            b.iter(|| {
                let mut chunk = Chunk::default();
                for i in 0..1000 {
                    chunk = chunk.prepend(i);
                }
                black_box(chunk);
            })
        })
        .bench_function("vec_prepend", |b| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..1000 {
                    vec.insert(0, i);
                }
                black_box(vec);
            })
        });

    // Benchmark concat operations
    c.benchmark_group("concat")
        .bench_function("chunk_concat", |b| {
            let chunk1: Chunk<_> = (0..500).collect();
            let chunk2: Chunk<_> = (500..1000).collect();
            b.iter(|| {
                black_box(chunk1.clone().concat(chunk2.clone()));
            })
        })
        .bench_function("vec_concat", |b| {
            let vec1: Vec<_> = (0..500).collect();
            let vec2: Vec<_> = (500..1000).collect();
            b.iter(|| {
                let mut result = vec1.clone();
                result.extend(vec2.iter().cloned());
                black_box(result)
            })
        });

    // Benchmark clone operations
    c.benchmark_group("clone")
        .bench_function("chunk_clone", |b| {
            let chunk: Chunk<_> = (0..1000).collect();
            b.iter(|| {
                black_box(chunk.clone());
            })
        })
        .bench_function("vec_clone", |b| {
            let vec: Vec<_> = (0..1000).collect();
            b.iter(|| {
                black_box(vec.clone());
            })
        });
}

criterion_group!(benches, bench_operations);
criterion_main!(benches);
