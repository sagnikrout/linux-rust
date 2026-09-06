//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/check.c
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

//
// Some BIOSes seem to corrupt the low 64k of memory during events
// like suspend/resume and unplugging an HDMI cable.  Reserve all
// remaining free memory in that area and fill it with a distinct
// pattern.
//
pub const MAX_SCAN_AREAS: c_int = 8;
    let mut memory_corruption_check: static int __read_mostly = -1;
    let mut corruption_check_size: static unsigned __read_mostly = 64*1024;
    static unsigned __read_mostly corruption_check_period = 60; /* seconds */
    static struct scan_area {
    u64 addr;
    u64 size;
    } scan_areas[MAX_SCAN_AREAS];
    static int num_scan_areas;
#[no_mangle]
unsafe extern "C" fn set_corruption_check(arg: *mut c_char) -> __init int {
    static __init int set_corruption_check(char *arg)
    {
    ssize_t ret;
    unsigned long val;
    if (!arg) {
    pr_err("memory_corruption_check config string not provided\n");
    return -EINVAL;
    }
    ret = kstrtoul(arg, 10, &val);
    if (ret)
    return ret;
    memory_corruption_check = val;
    return 0;
    }
    early_param("memory_corruption_check", set_corruption_check);
#[no_mangle]
unsafe extern "C" fn set_corruption_check_period(arg: *mut c_char) -> __init int {
    static __init int set_corruption_check_period(char *arg)
    {
    ssize_t ret;
    unsigned long val;
    if (!arg) {
    pr_err("memory_corruption_check_period config string not provided\n");
    return -EINVAL;
    }
    ret = kstrtoul(arg, 10, &val);
    if (ret)
    return ret;
    corruption_check_period = val;
    return 0;
    }
    early_param("memory_corruption_check_period", set_corruption_check_period);
#[no_mangle]
unsafe extern "C" fn set_corruption_check_size(arg: *mut c_char) -> __init int {
    static __init int set_corruption_check_size(char *arg)
    {
    char *end;
    unsigned size;
    if (!arg) {
    pr_err("memory_corruption_check_size config string not provided\n");
    return -EINVAL;
    }
    size = memparse(arg, &end);
    if (*end == '\0')
    corruption_check_size = size;
    return (size == corruption_check_size) ? 0 : -EINVAL;
    }
    early_param("memory_corruption_check_size", set_corruption_check_size);
#[no_mangle]
pub unsafe extern "C" fn setup_bios_corruption_check() -> void __init {
    void __init setup_bios_corruption_check(void)
    {
    phys_addr_t start, end;
    u64 i;
    if (memory_corruption_check == -1) {
    memory_corruption_check =

    1

    0

    ;
    }
    if (corruption_check_size == 0)
    memory_corruption_check = 0;
    if (!memory_corruption_check)
    return;
    corruption_check_size = round_up(corruption_check_size, PAGE_SIZE);
    for_each_free_mem_range(i, NUMA_NO_NODE, MEMBLOCK_NONE, &start, &end,
    core::ptr::null_mut()) {
    start = clamp_t(phys_addr_t, round_up(start, PAGE_SIZE),
    PAGE_SIZE, corruption_check_size);
    end = clamp_t(phys_addr_t, round_down(end, PAGE_SIZE),
    PAGE_SIZE, corruption_check_size);
    if (start >= end)
    continue;
    memblock_reserve(start, end - start);
    scan_areas[num_scan_areas].addr = start;
    scan_areas[num_scan_areas].size = end - start;
// Assume we've already mapped this early memory
    memset(__va(start), 0, end - start);
    if (++num_scan_areas >= MAX_SCAN_AREAS)
    break;
    }
    if (num_scan_areas)
    pr_info("Scanning %d areas for low memory corruption\n", num_scan_areas);
    }
#[no_mangle]
unsafe extern "C" fn check_for_bios_corruption() {
    static void check_for_bios_corruption(void)
    {
    int i;
    let mut corruption: c_int = 0;
    if (!memory_corruption_check)
    return;
    for (i = 0; i < num_scan_areas; i++) {
    unsigned long *addr = __va(scan_areas[i].addr);
    let mut size: c_ulong = scan_areas[i].size;
    for (; size; addr++, size -= sizeof(unsigned long)) {
    if (!*addr)
    continue;
    pr_err("Corrupted low memory at %p (%lx phys) = %08lx\n", addr, __pa(addr), *addr);
    corruption = 1;
// addr = 0;
    }
    }
    WARN_ONCE(corruption, KERN_ERR "Memory corruption detected in low memory\n");
    }
    static void check_corruption(struct work_struct *dummy);
    static DECLARE_DELAYED_WORK(bios_check_work, check_corruption);
#[no_mangle]
unsafe extern "C" fn check_corruption(dummy: *mut work_struct) {
    static void check_corruption(struct work_struct *dummy)
    {
    check_for_bios_corruption();
    schedule_delayed_work(&bios_check_work,
    round_jiffies_relative(corruption_check_period*HZ));
    }
#[no_mangle]
unsafe extern "C" fn start_periodic_check_for_corruption() -> c_int {
    static int start_periodic_check_for_corruption(void)
    {
    if (!num_scan_areas || !memory_corruption_check || corruption_check_period == 0)
    return 0;
    pr_info("Scanning for low memory corruption every %d seconds\n", corruption_check_period);
// First time we run the checks right away
    schedule_delayed_work(&bios_check_work, 0);
    return 0;
    }
    device_initcall(start_periodic_check_for_corruption);
