use std::{collections::HashMap, mem};

use criterion::{Criterion, black_box, criterion_group, criterion_main};

type Entry = [u8; 32];

const ENTRY_COUNTS: &[usize] = &[0, 100_000_000];
const STORAGE_TABLE_TITLE: &str = "HashMap<[u8; 32], ()> estimated storage";

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

fn bytes_per_entry(bytes: usize, entries: usize) -> f64 {
    if entries == 0 {
        0.0
    } else {
        bytes as f64 / entries as f64
    }
}

fn print_storage_header() {
    println!();
    println!("{STORAGE_TABLE_TITLE}");
    println!(
        "{:>12} {:>12} {:>16} {:>14}",
        "entries", "capacity", "bytes", "bytes/entry"
    );
}

fn print_storage_row(entries: usize, capacity: usize, bytes: usize) {
    println!(
        "{:>12} {:>12} {:>16} {:>14.2}",
        entries,
        capacity,
        bytes,
        bytes_per_entry(bytes, entries)
    );
}

fn bench_storage(_c: &mut Criterion) {
    print_storage_header();

    for entries in ENTRY_COUNTS {
        let map = build_hashmap(*entries);
        let bytes = estimated_storage_bytes(&map);
        print_storage_row(*entries, map.capacity(), bytes);
        black_box(map);
    }

    println!();
}

criterion_group!(benches, bench_storage);
criterion_main!(benches);
