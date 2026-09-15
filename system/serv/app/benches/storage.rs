use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};

use constant_time_utils::bytes::Long;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use dashmap::DashMap;

const ENTRY_COUNTS: &[usize] = &[21_800];
const STORAGE_TABLE_TITLE: &str = "DashMap<[u8; 32], ()> live allocated bytes";

struct CountingAllocator;

static MEASURE_ACTIVE: AtomicBool = AtomicBool::new(false);
static MEASURED_DELTA_BYTES: AtomicIsize = AtomicIsize::new(0);
thread_local! {
    static MEASURE_THIS_THREAD: Cell<bool> = const { Cell::new(false) };
}

#[global_allocator]
static GLOBAL_ALLOCATOR: CountingAllocator = CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() && should_count_this_allocation() {
            MEASURED_DELTA_BYTES.fetch_add(layout.size() as isize, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc_zeroed(layout) };
        if !ptr.is_null() && should_count_this_allocation() {
            MEASURED_DELTA_BYTES.fetch_add(layout.size() as isize, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if should_count_this_allocation() {
            MEASURED_DELTA_BYTES.fetch_sub(layout.size() as isize, Ordering::Relaxed);
        }
        unsafe { System.dealloc(ptr, layout) };
    }

    unsafe fn realloc(&self, ptr: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8 {
        let new_ptr = unsafe { System.realloc(ptr, old_layout, new_size) };
        if !new_ptr.is_null() && should_count_this_allocation() {
            MEASURED_DELTA_BYTES.fetch_add(
                new_size as isize - old_layout.size() as isize,
                Ordering::Relaxed,
            );
        }
        new_ptr
    }
}

fn should_count_this_allocation() -> bool {
    MEASURE_ACTIVE.load(Ordering::Relaxed) && MEASURE_THIS_THREAD.with(Cell::get)
}

fn build_dashmap(entries: usize) -> DashMap<Long, ()> {
    let map = DashMap::new();
    for idx in 0..entries {
        let mut token = [0u8; 32];
        token[..8].copy_from_slice(&(idx as u64).to_be_bytes());
        map.insert(token, ());
    }
    map
}

fn bench_storage(_c: &mut Criterion) {
    let warmup = build_dashmap(32);
    black_box(&warmup);
    drop(warmup);

    println!();
    println!("{STORAGE_TABLE_TITLE}");
    println!(
        "{:>12} {:>12} {:>16} {:>14}",
        "entries", "capacity", "alloc_delta", "bytes/entry"
    );

    for &entries in ENTRY_COUNTS {
        MEASURED_DELTA_BYTES.store(0, Ordering::Relaxed);
        MEASURE_THIS_THREAD.with(|flag| flag.set(true));
        MEASURE_ACTIVE.store(true, Ordering::Relaxed);

        let map = build_dashmap(entries);
        let capacity = map.capacity();

        MEASURE_ACTIVE.store(false, Ordering::Relaxed);
        MEASURE_THIS_THREAD.with(|flag| flag.set(false));
        let allocated = MEASURED_DELTA_BYTES.load(Ordering::Relaxed).max(0) as usize;

        println!(
            "{:>12} {:>12} {:>16} {:>14.2}",
            entries,
            capacity,
            allocated,
            allocated as f64 / entries as f64
        );
        black_box(&map);
        drop(map);
    }

    println!();
}

criterion_group!(benches, bench_storage);
criterion_main!(benches);
