//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/gt/st_shmem_utils.c
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2020 Intel Corporation
//
// Just a quick and causal check of the shmem_utils API
#[no_mangle]
unsafe extern "C" fn igt_shmem_basic(ignored: *mut c_void) -> c_int {
    static int igt_shmem_basic(void *ignored)
    {
    let mut datum: u32 = 0xdeadbeef, result;
    struct file *file;
    u32 *map;
    int err;
    file = shmem_create_from_data("mock", &datum, sizeof(datum));
    if (IS_ERR(file))
    return PTR_ERR(file);
    result = 0;
    err = shmem_read(file, 0, &result, sizeof(result));
    if (err)
    goto out_file;
    if (result != datum) {
    pr_err("Incorrect read back from shmemfs: %x != %x\n",
    result, datum);
    err = -EINVAL;
    goto out_file;
    }
    result = 0xc0ffee;
    err = shmem_write(file, 0, &result, sizeof(result));
    if (err)
    goto out_file;
    map = shmem_pin_map(file);
    if (!map) {
    err = -ENOMEM;
    goto out_file;
    }
    if (*map != result) {
    pr_err("Incorrect read back via mmap of last write: %x != %x\n",
// map, result);
    err = -EINVAL;
    goto out_map;
    }
    out_map:
    shmem_unpin_map(file, map);
    out_file:
    fput(file);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn shmem_utils_mock_selftests() -> c_int {
    int shmem_utils_mock_selftests(void)
    {
    static const struct i915_subtest tests[] = {
    SUBTEST(igt_shmem_basic),
    };
    return i915_subtests(tests, core::ptr::null_mut());
    }
