use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use library_stdnums::lccn::valid;

fn valid_benchmark(c: &mut Criterion) {
  let lccns = [
    "78-890351",
    "n78-890351",
    "2001-890351",
    "nb78-890351",
    "agr78-890351",
    "n2001-890351",
    "nb2001-890351",
    "n78-89035100444",
    "n78",
    "na078-890351",
    "n078-890351",
    "0an78-890351",
    "n78-89c0351"
  ];

  let mut group = c.benchmark_group("valid");
  for i in lccns.iter() {
    group.bench_with_input(BenchmarkId::new("char", i), i,
      |b, i| b.iter(|| valid(*i, false)));
  }
  group.finish();
}

criterion_group!(valid_benches, valid_benchmark);
criterion_main!(valid_benches);