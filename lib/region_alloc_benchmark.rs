//! Automatically rewritten from C to Rust
//! Source: lib/region_alloc_benchmark.c
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
// Benchmark bitmap, IDA and Maple Tree allocation of variable-sized regions.

pub const REGION_MAX_SIZE: c_int = 32;
    static unsigned long *bitmap __initdata;
// One more request guarantees that even an all-ones trace reaches ENOSPC.
    static u8 *reg_sz __initdata;
    static unsigned long *reg_idx __initdata;
    static unsigned long capacities[64] = { 1000000, 100000, 10000, 1000, 100, 10 };
    let mut cap_cnt: static unsigned int = 6;
    module_param_array(capacities, ulong, &cap_cnt, 0400);
    MODULE_PARM_DESC(capacities, "Region capacities to benchmark");
#[no_mangle]
unsafe extern "C" fn benchmark_bitmap(cap: c_ulong) -> unsigned long __init {
    static unsigned long __init benchmark_bitmap(unsigned long cap)
    {
    unsigned long cnt, idx;
    ktime_t alloc_time, free_time;
    size_t sz;
    bitmap_zero(bitmap, cap);
    alloc_time = ktime_get();
    for (cnt = 0; cnt <= cap; cnt++) {
    idx = bitmap_find_next_zero_area(bitmap, cap, 0, reg_sz[cnt], 0);
    if (idx >= cap)
    break;
    reg_idx[cnt] = idx;
    bitmap_set(bitmap, idx, reg_sz[cnt]);
    }
    alloc_time = ktime_get() - alloc_time;
    idx = cnt;
    free_time = ktime_get();
    while (idx--)
    bitmap_clear(bitmap, reg_idx[idx], reg_sz[idx]);
    free_time = ktime_get() - free_time;
    WARN_ON(!bitmap_empty(bitmap, cap));
    sz = BITS_TO_LONGS(cap) * sizeof(unsigned long);
    pr_err("Bitmap  %12llu  %12llu  %8lu  %8lu  %10zu\n",
    alloc_time, free_time, cnt, cap, sz);
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn ida_size(nr_ids: c_ulong) -> size_t __init {
    static size_t __init ida_size(unsigned long nr_ids)
    {
    let mut entries: c_ulong = DIV_ROUND_UP(nr_ids, IDA_BITMAP_BITS);
    let mut bitmaps: c_ulong = nr_ids / IDA_BITMAP_BITS;
    let mut nodes: c_ulong = 0;
    if (nr_ids % IDA_BITMAP_BITS > BITS_PER_XA_VALUE)
    bitmaps++;
    while (entries > 1) {
    entries = DIV_ROUND_UP(entries, XA_CHUNK_SIZE);
    nodes += entries;
    }
    return sizeof(struct ida) +
    bitmaps * sizeof(struct ida_bitmap) +
    nodes   * sizeof(struct xa_node);
    }
#[no_mangle]
unsafe extern "C" fn benchmark_ida(cap: c_ulong) -> unsigned long __init {
    static unsigned long __init benchmark_ida(unsigned long cap)
    {
    let mut ida: ida = IDA_INIT(ida);
    unsigned long cnt, idx, off, nr_ids = 0;
    ktime_t alloc_time, free_time;
    let mut id: c_int = -ENOSPC;
    alloc_time = ktime_get();
    for (cnt = 0; cnt <= cap; cnt++) {
    for (off = 0; off < reg_sz[cnt]; off++) {
    id = ida_alloc_max(&ida, cap - 1, GFP_KERNEL);
    if (id < 0)
    break;
    if (!off)
    reg_idx[cnt] = id;
    }
    if (id < 0) {
    while (off--)
    ida_free(&ida, reg_idx[cnt] + off);
    break;
    }
    WARN_ON(id != reg_idx[cnt] + reg_sz[cnt] - 1);
    nr_ids += reg_sz[cnt];
    }
    alloc_time = ktime_get() - alloc_time;
    WARN_ON(id != -ENOSPC);
    idx = cnt;
    free_time = ktime_get();
    while (idx--) {
    for (off = 0; off < reg_sz[idx]; off++)
    ida_free(&ida, reg_idx[idx] + off);
    }
    free_time = ktime_get() - free_time;
    WARN_ON(!ida_is_empty(&ida));
    pr_err("IDA     %12llu  %12llu  %8lu  %8lu  %10zu\n",
    alloc_time, free_time, cnt, cap, ida_size(nr_ids));
    ida_destroy(&ida);
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn benchmark_maple_tree(cap: c_ulong) -> unsigned long __init {
    static unsigned long __init benchmark_maple_tree(unsigned long cap)
    {
    let mut mt: maple_tree = MTREE_INIT(mt, MT_FLAGS_ALLOC_RANGE);
    unsigned long cnt, idx;
    ktime_t alloc_time, free_time;
    size_t sz;
    int ret;
    alloc_time = ktime_get();
    for (cnt = 0; cnt <= cap; cnt++) {
    ret = mtree_alloc_range(&mt, &idx, xa_mk_value(cnt + 1),
    reg_sz[cnt], 0, cap - 1, GFP_KERNEL);
    if (ret)
    break;
    reg_idx[cnt] = idx;
    }
    alloc_time = ktime_get() - alloc_time;
    WARN_ON(ret != -EBUSY);
    idx = cnt;
    free_time = ktime_get();
    while (idx--)
    mtree_erase(&mt, reg_idx[idx]);
    free_time = ktime_get() - free_time;
    WARN_ON(!mtree_empty(&mt));
// Minimum storage assuming fully occupied allocation-range leaf nodes.
    sz = sizeof(mt) + DIV_ROUND_UP(cnt, MAPLE_ARANGE64_SLOTS) * sizeof(struct maple_node);
    pr_err("Maple   %12llu  %12llu  %8lu  %8lu  %10zu\n",
    alloc_time, free_time, cnt, cap, sz);
    mtree_destroy(&mt);
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn region_alloc_benchmark() -> int __init {
    static int __init region_alloc_benchmark(void)
    {
    unsigned long bitmap_count, ida_count, maple_count;
    unsigned long i, max_cap = 0;
    let mut ret: c_int = -ENOMEM;
    for (i = 0; i < cap_cnt; i++) {
    if (capacities[i] == 0) {
    pr_err("capacity must be nonzero\n");
    return -EINVAL;
    }
    max_cap = max(max_cap, capacities[i]);
    }
    bitmap = kvmalloc_array(BITS_TO_LONGS(max_cap), sizeof(*bitmap), GFP_KERNEL);
    reg_sz = kvmalloc_array(max_cap + 1, sizeof(*reg_sz), GFP_KERNEL);
    reg_idx = kvmalloc_array(max_cap, sizeof(*reg_idx), GFP_KERNEL);
    if (!bitmap || !reg_sz || !reg_idx)
    goto out;
    pr_err("\nStart testing bitmap vs IDA vs Maple Tree region allocation\n");
    pr_err("memory: bitmap is exact; IDA and Maple Tree are lower bounds\n");
    pr_err("Type      alloc (ns)     free (ns)   regions  capacity  memory (B)\n");
    for (i = 0; i < cap_cnt; i++) {
    unsigned long idx, max_size;
    max_size = min(REGION_MAX_SIZE, capacities[i] / 10) ? : 1;
    for (idx = 0; idx <= capacities[i]; idx++)
    reg_sz[idx] = get_random_u32_below(max_size) + 1;
    bitmap_count = benchmark_bitmap(capacities[i]);
    maple_count  = benchmark_maple_tree(capacities[i]);
    ida_count    = benchmark_ida(capacities[i]);
    WARN_ON(bitmap_count != ida_count);
    WARN_ON(bitmap_count != maple_count);
    }
// Return an error so the benchmark can run repeatedly without rmmod.
    pr_info("Region allocation benchmark complete\n");
    ret = -EAGAIN;
    out:
    kvfree(reg_idx);
    kvfree(reg_sz);
    kvfree(bitmap);
    return ret;
    }
    module_init(region_alloc_benchmark);
    MODULE_AUTHOR("Yury Norov <ynorov@nvidia.com>");
    MODULE_DESCRIPTION("Benchmark bitmap, IDA and Maple Tree region allocation");
    MODULE_LICENSE("GPL");
