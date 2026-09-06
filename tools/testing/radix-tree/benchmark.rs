//! Automatically rewritten from C to Rust
//! Source: tools/testing/radix-tree/benchmark.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-only
//
// benchmark.c:
// Author: Konstantin Khlebnikov <koct9i@gmail.com>
//

#[no_mangle]
unsafe extern "C" fn benchmark_iter(root: *mut radix_tree_root, tagged: bool) -> c_longlong {
    static long long benchmark_iter(struct radix_tree_root *root, bool tagged)
    {
    let mut sink: volatile unsigned long = 0;
    struct radix_tree_iter iter;
    struct timespec start, finish;
    long long nsec;
    int l, loops = 1;
    void **slot;

    again:

    clock_gettime(CLOCK_MONOTONIC, &start);
    for (l = 0; l < loops; l++) {
    if (tagged) {
    radix_tree_for_each_tagged(slot, root, &iter, 0, 0)
    sink ^= (unsigned long)slot;
    } else {
    radix_tree_for_each_slot(slot, root, &iter, 0)
    sink ^= (unsigned long)slot;
    }
    }
    clock_gettime(CLOCK_MONOTONIC, &finish);
    nsec = (finish.tv_sec - start.tv_sec) * NSEC_PER_SEC +
    (finish.tv_nsec - start.tv_nsec);

    if (loops == 1 && nsec * 5 < NSEC_PER_SEC) {
    loops = NSEC_PER_SEC / nsec / 4 + 1;
    goto again;
    }

    nsec /= loops;
    return nsec;
    }
    static void benchmark_insert(struct radix_tree_root *root,
    unsigned long size, unsigned long step)
    {
    struct timespec start, finish;
    unsigned long index;
    long long nsec;
    clock_gettime(CLOCK_MONOTONIC, &start);
    for (index = 0 ; index < size ; index += step)
    item_insert(root, index);
    clock_gettime(CLOCK_MONOTONIC, &finish);
    nsec = (finish.tv_sec - start.tv_sec) * NSEC_PER_SEC +
    (finish.tv_nsec - start.tv_nsec);
    printv(2, "Size: %8ld, step: %8ld, insertion: %15lld ns\n",
    size, step, nsec);
    }
    static void benchmark_tagging(struct radix_tree_root *root,
    unsigned long size, unsigned long step)
    {
    struct timespec start, finish;
    unsigned long index;
    long long nsec;
    clock_gettime(CLOCK_MONOTONIC, &start);
    for (index = 0 ; index < size ; index += step)
    radix_tree_tag_set(root, index, 0);
    clock_gettime(CLOCK_MONOTONIC, &finish);
    nsec = (finish.tv_sec - start.tv_sec) * NSEC_PER_SEC +
    (finish.tv_nsec - start.tv_nsec);
    printv(2, "Size: %8ld, step: %8ld, tagging: %17lld ns\n",
    size, step, nsec);
    }
    static void benchmark_delete(struct radix_tree_root *root,
    unsigned long size, unsigned long step)
    {
    struct timespec start, finish;
    unsigned long index;
    long long nsec;
    clock_gettime(CLOCK_MONOTONIC, &start);
    for (index = 0 ; index < size ; index += step)
    item_delete(root, index);
    clock_gettime(CLOCK_MONOTONIC, &finish);
    nsec = (finish.tv_sec - start.tv_sec) * NSEC_PER_SEC +
    (finish.tv_nsec - start.tv_nsec);
    printv(2, "Size: %8ld, step: %8ld, deletion: %16lld ns\n",
    size, step, nsec);
    }
#[no_mangle]
unsafe extern "C" fn benchmark_size(size: c_ulong, step: c_ulong) {
    static void benchmark_size(unsigned long size, unsigned long step)
    {
    RADIX_TREE(tree, GFP_KERNEL);
    long long normal, tagged;
    benchmark_insert(&tree, size, step);
    benchmark_tagging(&tree, size, step);
    tagged = benchmark_iter(&tree, true);
    normal = benchmark_iter(&tree, false);
    printv(2, "Size: %8ld, step: %8ld, tagged iteration: %8lld ns\n",
    size, step, tagged);
    printv(2, "Size: %8ld, step: %8ld, normal iteration: %8lld ns\n",
    size, step, normal);
    benchmark_delete(&tree, size, step);
    item_kill_tree(&tree);
    rcu_barrier();
    }
#[no_mangle]
pub unsafe extern "C" fn benchmark() {
    void benchmark(void)
    {
    unsigned long size[] = {1 << 10, 1 << 20, 0};
    unsigned long step[] = {1, 2, 7, 15, 63, 64, 65,
    128, 256, 512, 12345, 0};
    int c, s;
    printv(1, "starting benchmarks\n");
    printv(1, "RADIX_TREE_MAP_SHIFT = %d\n", RADIX_TREE_MAP_SHIFT);
    for (c = 0; size[c]; c++)
    for (s = 0; step[s]; s++)
    benchmark_size(size[c], step[s]);
    }
