//! Automatically rewritten from C to Rust
//! Source: mm/memtest.c
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


// SPDX-License-Identifier: GPL-2.0

    static bool early_memtest_done;
    static phys_addr_t early_memtest_bad_size;
    static u64 patterns[] __initdata = {
// The first entry has to be 0 to leave memtest with zeroed memory
    0,
    0xffffffffffffffffULL,
    0x5555555555555555ULL,
    0xaaaaaaaaaaaaaaaaULL,
    0x1111111111111111ULL,
    0x2222222222222222ULL,
    0x4444444444444444ULL,
    0x8888888888888888ULL,
    0x3333333333333333ULL,
    0x6666666666666666ULL,
    0x9999999999999999ULL,
    0xccccccccccccccccULL,
    0x7777777777777777ULL,
    0xbbbbbbbbbbbbbbbbULL,
    0xddddddddddddddddULL,
    0xeeeeeeeeeeeeeeeeULL,
    0x7a6c7258554e494cULL, /* yeah ;-) */
    };
#[no_mangle]
unsafe extern "C" fn reserve_bad_mem(pattern: u64, start_bad: phys_addr_t, end_bad: phys_addr_t) -> void __init {
    static void __init reserve_bad_mem(u64 pattern, phys_addr_t start_bad, phys_addr_t end_bad)
    {
    pr_info("  %016llx bad mem addr %pa - %pa reserved\n",
    cpu_to_be64(pattern), &start_bad, &end_bad);
    memblock_reserve(start_bad, end_bad - start_bad);
    early_memtest_bad_size += (end_bad - start_bad);
    }
#[no_mangle]
unsafe extern "C" fn memtest(pattern: u64, start_phys: phys_addr_t, size: phys_addr_t) -> void __init {
    static void __init memtest(u64 pattern, phys_addr_t start_phys, phys_addr_t size)
    {
    u64 *p, *start, *end;
    phys_addr_t start_bad, last_bad;
    phys_addr_t start_phys_aligned;
    let mut incr: usize = sizeof(pattern);
    start_phys_aligned = ALIGN(start_phys, incr);
    start = __va(start_phys_aligned);
    end = start + (size - (start_phys_aligned - start_phys)) / incr;
    start_bad = 0;
    last_bad = 0;
    VM_WARN_ON_ONCE(size < start_phys_aligned - start_phys);
    for (p = start; p < end; p++)
    WRITE_ONCE(*p, pattern);
    for (p = start; p < end; p++, start_phys_aligned += incr) {
    if (READ_ONCE(*p) == pattern)
    continue;
    if (start_phys_aligned == last_bad + incr) {
    last_bad += incr;
    continue;
    }
    if (start_bad)
    reserve_bad_mem(pattern, start_bad, last_bad + incr);
    start_bad = last_bad = start_phys_aligned;
    }
    if (start_bad)
    reserve_bad_mem(pattern, start_bad, last_bad + incr);
    early_memtest_done = true;
    }
#[no_mangle]
unsafe extern "C" fn do_one_pass(pattern: u64, start: phys_addr_t, end: phys_addr_t) -> void __init {
    static void __init do_one_pass(u64 pattern, phys_addr_t start, phys_addr_t end)
    {
    u64 i;
    phys_addr_t this_start, this_end;
    for_each_free_mem_range(i, NUMA_NO_NODE, MEMBLOCK_NONE, &this_start,
    &this_end, core::ptr::null_mut()) {
    this_start = clamp(this_start, start, end);
    this_end = clamp(this_end, start, end);
    if (this_start < this_end) {
    pr_info("  %pa - %pa pattern %016llx\n",
    &this_start, &this_end, cpu_to_be64(pattern));
    memtest(pattern, this_start, this_end - this_start);
    }
    }
    }
// default is disabled
    static unsigned int memtest_pattern __initdata;
#[no_mangle]
unsafe extern "C" fn parse_memtest(arg: *mut c_char) -> int __init {
    static int __init parse_memtest(char *arg)
    {
    let mut ret: c_int = 0;
    if (arg)
    ret = kstrtouint(arg, 0, &memtest_pattern);
    else
    memtest_pattern = ARRAY_SIZE(patterns);
    return ret;
    }
    early_param("memtest", parse_memtest);
#[no_mangle]
pub unsafe extern "C" fn early_memtest(start: phys_addr_t, end: phys_addr_t) -> void __init {
    void __init early_memtest(phys_addr_t start, phys_addr_t end)
    {
    unsigned int i;
    let mut idx: c_uint = 0;
    if (!memtest_pattern)
    return;
    pr_info("early_memtest: # of tests: %u\n", memtest_pattern);
    for (i = memtest_pattern-1; i < UINT_MAX; --i) {
    idx = i % ARRAY_SIZE(patterns);
    do_one_pass(patterns[idx], start, end);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn memtest_report_meminfo(m: *mut seq_file) {
    void memtest_report_meminfo(struct seq_file *m)
    {
    unsigned long early_memtest_bad_size_kb;
    if (!IS_ENABLED(CONFIG_PROC_FS))
    return;
    if (!early_memtest_done)
    return;
    early_memtest_bad_size_kb = early_memtest_bad_size >> 10;
    if (early_memtest_bad_size && !early_memtest_bad_size_kb)
    early_memtest_bad_size_kb = 1;
// When 0 is reported, it means there actually was a successful test
    seq_printf(m, "EarlyMemtestBad:   %5lu kB\n", early_memtest_bad_size_kb);
    }
