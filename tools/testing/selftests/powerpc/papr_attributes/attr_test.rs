//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/papr_attributes/attr_test.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// PAPR Energy attributes sniff test
// This checks if the papr folders and contents are populated relating to
// the energy and frequency attributes
//
// Copyright 2022, Pratik Rajesh Sampat, IBM Corp.
//

    enum energy_freq_attrs {
    POWER_PERFORMANCE_MODE = 1,
    IDLE_POWER_SAVER_STATUS = 2,
    MIN_FREQ = 3,
    STAT_FREQ = 4,
    MAX_FREQ = 6,
    PROC_FOLDING_STATUS = 8
    };
    enum type {
    INVALID,
    STR_VAL,
    NUM_VAL
    };
#[no_mangle]
unsafe extern "C" fn value_type(id: c_int) -> c_int {
    static int value_type(int id)
    {
    int val_type;
    switch (id) {
    case POWER_PERFORMANCE_MODE:
    case IDLE_POWER_SAVER_STATUS:
    val_type = STR_VAL;
    break;
    case MIN_FREQ:
    case STAT_FREQ:
    case MAX_FREQ:
    case PROC_FOLDING_STATUS:
    val_type = NUM_VAL;
    break;
    default:
    val_type = INVALID;
    }
    return val_type;
    }
#[no_mangle]
unsafe extern "C" fn verify_energy_info() -> c_int {
    static int verify_energy_info(void)
    {
    const char *path = "/sys/firmware/papr/energy_scale_info";
    struct dirent *entry;
    struct stat s;
    DIR *dirp;
    errno = 0;
    if (stat(path, &s)) {
    SKIP_IF(errno == ENOENT);
    FAIL_IF(errno);
    }
    FAIL_IF(!S_ISDIR(s.st_mode));
    dirp = opendir(path);
    while ((entry = readdir(dirp)) != core::ptr::null_mut()) {
    char file_name[64];
    int id, attr_type;
    FILE *f;
    if (strcmp(entry.d_name, ".") == 0 ||
    strcmp(entry.d_name, "..") == 0)
    continue;
    id = atoi(entry.d_name);
    attr_type = value_type(id);
    FAIL_IF(attr_type == INVALID);
// Check if the files exist and have data in them
    sprintf(file_name, "%s/%d/desc", path, id);
    f = fopen(file_name, "r");
    FAIL_IF(!f);
    FAIL_IF(fgetc(f) == EOF);
    sprintf(file_name, "%s/%d/value", path, id);
    f = fopen(file_name, "r");
    FAIL_IF(!f);
    FAIL_IF(fgetc(f) == EOF);
    if (attr_type == STR_VAL) {
    sprintf(file_name, "%s/%d/value_desc", path, id);
    f = fopen(file_name, "r");
    FAIL_IF(!f);
    FAIL_IF(fgetc(f) == EOF);
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(verify_energy_info, "papr_attributes");
    }
