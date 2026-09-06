//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/dscr/dscr_sysfs_test.c
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
// POWER Data Stream Control Register (DSCR) sysfs interface test
//
// This test updates to system wide DSCR default through the sysfs interface
// and then verifies that all the CPU specific DSCR defaults are updated as
// well verified from their sysfs interfaces.
//
// Copyright 2015, Anshuman Khandual, IBM Corporation.
//

#[no_mangle]
unsafe extern "C" fn check_cpu_dscr_default(file: *mut c_char, val: c_ulong) -> c_int {
    static int check_cpu_dscr_default(char *file, unsigned long val)
    {
    unsigned long cpu_dscr;
    int err;
    err = read_ulong(file, &cpu_dscr, 16);
    if (err)
    return err;
    if (cpu_dscr != val) {
    printf("DSCR match failed: %ld (system) %ld (cpu)\n",
    val, cpu_dscr);
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_all_cpu_dscr_defaults(val: c_ulong) -> c_int {
    static int check_all_cpu_dscr_defaults(unsigned long val)
    {
    DIR *sysfs;
    struct dirent *dp;
    char file[LEN_MAX];
    sysfs = opendir(CPU_PATH);
    if (!sysfs) {
    perror("opendir() failed");
    return 1;
    }
    while ((dp = readdir(sysfs))) {
    int len;
    if (!(dp.d_type & DT_DIR))
    continue;
    if (!strcmp(dp.d_name, "cpuidle"))
    continue;
    if (!strstr(dp.d_name, "cpu"))
    continue;
    len = snprintf(file, LEN_MAX, "%s%s/dscr", CPU_PATH, dp.d_name);
    if (len >= LEN_MAX)
    continue;
    if (access(file, F_OK))
    continue;
    if (check_cpu_dscr_default(file, val)) {
    closedir(sysfs);
    return 1;
    }
    }
    closedir(sysfs);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dscr_sysfs() -> c_int {
    int dscr_sysfs(void)
    {
    unsigned long orig_dscr_default;
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_DSCR));
    orig_dscr_default = get_default_dscr();
    for (int i = 0; i < DSCR_MAX; i++) {
    set_default_dscr(i);
    if (check_all_cpu_dscr_defaults(i))
    goto fail;
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
    return test_harness(dscr_sysfs, "dscr_sysfs_test");
    }
