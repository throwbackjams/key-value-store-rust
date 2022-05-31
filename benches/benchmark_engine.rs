use criterion::{black_box, criterion_group, criterion_main, Benchmark, Criterion};
use kvs::engines::{KvStore, KvsEngine, SledKvsEngine};
use kvs::error::Result;
use kvs::utils::SLED_FILE_NAME;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::path::PathBuf;
use std::time::Duration;

fn create_random_values(n: u64) -> Vec<(String, String)> {
    let byte_length_range = 1..100_000;

    let number_pairs = n;

    let mut vec = vec![];

    for _ in 0..number_pairs {
        let mut rng = rand::thread_rng();

        let rand_key_length = rng.gen_range(byte_length_range.clone());
        let rand_value_length = rng.gen_range(byte_length_range.clone());

        let rand_key: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(rand_key_length)
            .map(char::from)
            .collect();

        let rand_value: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(rand_value_length)
            .map(char::from)
            .collect();

        vec.push((rand_key, rand_value));
    }
    println!("vec length: {}", vec.len());
    vec
}

fn kvs_set(key_value_pairs: Vec<(String, String)>, kv_store: &mut KvStore) -> Result<()> {
    for (key, value) in key_value_pairs.into_iter() {
        kv_store.set(key, value)?;
    }

    Ok(())
}

fn kvs_get(key_value_pairs: Vec<(String, String)>, kv_store: &mut KvStore) -> Result<()> {
    // let path = PathBuf::from("");
    // let mut kv_store = KvStore::open(path)?;

    println!("inside kvs_get");
    for (key, _value) in key_value_pairs.into_iter() {
        println!("starting iteration");
        kv_store.get(key)?;
        println!("kvs_get iteration complete");
    }

    Ok(())
}

fn sled_set(key_value_pairs: Vec<(String, String)>, sled_engine: &mut SledKvsEngine) -> Result<()> {
    for (key, value) in key_value_pairs.into_iter() {
        sled_engine.set(key, value)?;
    }

    Ok(())
}

fn sled_get(key_value_pairs: Vec<(String, String)>, sled_engine: &mut SledKvsEngine) -> Result<()> {
    for (key, _value) in key_value_pairs.into_iter() {
        sled_engine.get(key)?;
    }

    Ok(())
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let key_value_pairs = create_random_values(100);
    let path = PathBuf::from("");
    println!("path opened");
    let mut kv_store = KvStore::open(path).unwrap();

    println!("starting group kvs");

    let sled_db = SledKvsEngine::open(SLED_FILE_NAME).unwrap();

    let mut sled_engine = SledKvsEngine {
        directory_path: PathBuf::from(SLED_FILE_NAME),
        sled_db: sled_db,
    };

    let mut group = c.benchmark_group("kvs");
    group.sample_size(10);
    group.bench_function("kvs set 10", |b| {
        b.iter(|| kvs_set(key_value_pairs.clone(), &mut kv_store))
    });
    // group.bench_function("kvs get 10", |b| b.iter(|| kvs_get(key_value_pairs.clone(),&mut kv_store)));
    group.finish();

    let mut sled_group = c.benchmark_group("sled");
    sled_group.sample_size(100);
    sled_group.bench_function("sled set 100", |b| {
        b.iter(|| sled_set(key_value_pairs.clone(), &mut sled_engine))
    });
    sled_group.bench_function("sled get 100", |b| {
        b.iter(|| sled_get(key_value_pairs.clone(), &mut sled_engine))
    });
    sled_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
