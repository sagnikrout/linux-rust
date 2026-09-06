//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/libarena/selftests/test_spmc.bpf.c
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


// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause

//
// NOTE: These selftests only test for the single-threaded use case, which for
// Lev-Chase queues is obviously the simplest one. Still, it is important to
// exercise the API to ensure it passes verification and basic checks.
//
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_spmc_remove_empty() -> c_int {
    int test_spmc_remove_empty(void)
    {
    u64 val;
    int ret;
    struct spmc __arena *spmc = spmc_create();
    if (!spmc)
    return 1;
    ret = spmc_owned_remove(spmc, &val);
    if (ret != -ENOENT)
    return 1;
    spmc_destroy(spmc);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_spmc_steal_empty() -> c_int {
    int test_spmc_steal_empty(void)
    {
    u64 val;
    int ret;
    struct spmc __arena *spmc = spmc_create();
    if (!spmc)
    return 1;
    ret = spmc_steal(spmc, &val);
    if (ret != -ENOENT)
    return 1;
    spmc_destroy(spmc);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_spmc_steal_one() -> c_int {
    int test_spmc_steal_one(void)
    {
    u64 val, newval;
    int ret, i;
    struct spmc __arena *spmc = spmc_create();
    if (!spmc)
    return 1;
    for (i = 0; i < 10 && can_loop; i++) {
    val = i;
    ret = spmc_owned_add(spmc, val);
    if (ret)
    return 1;
    ret = spmc_steal(spmc, &newval);
    if (ret)
    return 2;
    if (val != newval)
    return 3;
    }
    spmc_destroy(spmc);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_spmc_remove_one() -> c_int {
    int test_spmc_remove_one(void)
    {
    u64 val, newval;
    int ret, i;
    struct spmc __arena *spmc = spmc_create();
    if (!spmc)
    return 1;
    for (i = 0; i < 10 && can_loop; i++) {
    val = i;
    ret = spmc_owned_add(spmc, val);
    if (ret)
    return 1;
    ret = spmc_owned_remove(spmc, &newval);
    if (ret)
    return 2;
    if (val != newval)
    return 3;
    }
    spmc_destroy(spmc);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_spmc_remove_many() -> c_int {
    int test_spmc_remove_many(void)
    {
    u64 val, newval;
    int ret, i;
    u64 expected;
    struct spmc __arena *spmc = spmc_create();
    if (!spmc)
    return 1;
    for (i = 0; i < 500 && can_loop; i++) {
    val = i;
    ret = spmc_owned_add(spmc, val);
    if (ret) {
    arena_stderr("%s:%d error %d\n", __func__, __LINE__, ret);
    return 1;
    }
    }
    for (i = 0; i < 500 && can_loop; i++) {
    ret = spmc_owned_remove(spmc, &newval);
    if (ret) {
    arena_stderr("%s:%d error %d\n", __func__, __LINE__, ret);
    return 1;
    }
    expected = 500 - 1 - i;
    if (newval != expected) {
    arena_stderr("%s:%d expected %llu found %llu\n", __func__, __LINE__, expected, newval);
    return 1;
    }
    }
    spmc_destroy(spmc);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_spmc_steal_many() -> c_int {
    int test_spmc_steal_many(void)
    {
    u64 val, newval;
    int ret, i;
    struct spmc __arena *spmc = spmc_create();
    if (!spmc)
    return 1;
    for (i = 0; i < 500 && can_loop; i++) {
    val = i;
    ret = spmc_owned_add(spmc, val);
    if (ret) {
    arena_stderr("%s:%d error %d\n", __func__, __LINE__, ret);
    return 1;
    }
    }
    for (i = 0; i < 500 && can_loop; i++) {
    ret = spmc_steal(spmc, &newval);
    if (ret) {
    arena_stderr("%s:%d error %d\n", __func__, __LINE__, ret);
    return 1;
    }
    if (newval != i) {
    arena_stderr("%s:%d expected %d found %llu\n", __func__, __LINE__, i, newval);
    return 1;
    }
    }
    spmc_destroy(spmc);
    return 0;
    }
