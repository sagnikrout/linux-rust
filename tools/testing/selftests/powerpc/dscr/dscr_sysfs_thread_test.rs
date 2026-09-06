//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/dscr/dscr_sysfs_thread_test.c
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
// POWER Data Stream Control Register (DSCR) sysfs thread test
//
// This test updates the system wide DSCR default value through
// sysfs interface which should then update all the CPU specific
// DSCR default values which must also be then visible to threads
// executing on individual CPUs on the system.
//
// Copyright 2015, Anshuman Khandual, IBM Corporation.
//
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
unsafe extern "C" fn test_thread_dscr(val: c_ulong) -> c_int {
    static int test_thread_dscr(unsigned long val)
    {
    unsigned long cur_dscr, cur_dscr_usr;
    cur_dscr = get_dscr();
    cur_dscr_usr = get_dscr_usr();
    if (val != cur_dscr) {
    printf("[cpu %d] Kernel DSCR should be %ld but is %ld\n",
    sched_getcpu(), val, cur_dscr);
    return 1;
    }
    if (val != cur_dscr_usr) {
    printf("[cpu %d] User DSCR should be %ld but is %ld\n",
    sched_getcpu(), val, cur_dscr_usr);
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_cpu_dscr_thread(val: c_ulong) -> c_int {
    static int check_cpu_dscr_thread(unsigned long val)
    {
    cpu_set_t mask;
    int cpu;
    for (cpu = 0; cpu < CPU_SETSIZE; cpu++) {
    CPU_ZERO(&mask);
    CPU_SET(cpu, &mask);
    if (sched_setaffinity(0, sizeof(mask), &mask))
    continue;
    if (test_thread_dscr(val))
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dscr_sysfs_thread() -> c_int {
    int dscr_sysfs_thread(void)
    {
    unsigned long orig_dscr_default;
    int i, j;
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_DSCR));
    orig_dscr_default = get_default_dscr();
    for (i = 0; i < COUNT; i++) {
    for (j = 0; j < DSCR_MAX; j++) {
    set_default_dscr(j);
    if (check_cpu_dscr_thread(j))
    goto fail;
    }
    }
    set_default_dscr(orig_dscr_default);
    return 0;
    fail:
    set_default_dscr(orig_dscr_default);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    return test_harness(dscr_sysfs_thread, "dscr_sysfs_thread_test");
    }
