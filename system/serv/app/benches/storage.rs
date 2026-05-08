use std::{collections::HashMap, mem};

use criterion::{Criterion, black_box, criterion_group, criterion_main};

type Entry = [u8; 32];

const ENTRY_COUNTS: &[usize] = &[0, 100_000_000];

fn token_from_counter(counter: u64) -> Entry {
    let mut token = [0u8; 32];
    token[0..8].copy_from_slice(&counter.to_be_bytes());
    token
}

fn build_hashmap(entries: usize) -> HashMap<Entry, ()> {
    let mut map = HashMap::new();

    for idx in 0..entries {
        map.insert(token_from_counter(idx as u64), ());
    }

    map
}

fn estimated_storage_bytes(map: &HashMap<Entry, ()>) -> usize {
    mem::size_of_val(map) + map.capacity() * mem::size_of::<Entry>()
}

fn bench_storage(_c: &mut Criterion) {
    println!();
    println!("HashMap<[u8; 32], ()> estimated storage");
    println!(
        "{:>12} {:>12} {:>16} {:>14}",
        "entries", "capacity", "bytes", "bytes/entry"
    );

    for entries in ENTRY_COUNTS {
        let map = build_hashmap(*entries);
        let bytes = estimated_storage_bytes(&map);
        let bytes_per_entry = if *entries == 0 {
            0.0
        } else {
            bytes as f64 / *entries as f64
        };

        println!(
            "{:>12} {:>12} {:>16} {:>14.2}",
            entries,
            map.capacity(),
            bytes,
            bytes_per_entry
        );

        black_box(map);
    }

    println!();
}

criterion_group!(benches, bench_storage);
criterion_main!(benches);
