//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/strncmp_bench.c
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
// Copyright (C) 2021. Huawei Technologies Co., Ltd

pub const STRNCMP_STR_SZ: c_int = 4096;
// Will be updated by benchmark before program loading
    let mut cmp_str_len: volatile unsigned int = 1;
    const char target[STRNCMP_STR_SZ];
    let mut hits: c_long = 0;
    char str[STRNCMP_STR_SZ];
    char _license[] SEC("license") = "GPL";
    static __always_inline int local_strncmp(const char *s1, unsigned int sz,
    const char *s2)
    {
    let mut ret: c_int = 0;
    unsigned int i;
    for (i = 0; i < sz; i++) {
// E.g. 0xff > 0x31
    ret = (unsigned char)s1[i] - (unsigned char)s2[i];
    if (ret || !s1[i])
    break;
    }
    return ret;
    }
    SEC("tp/syscalls/sys_enter_getpgid")
#[no_mangle]
pub unsafe extern "C" fn strncmp_no_helper(ctx: *mut c_void) -> c_int {
    int strncmp_no_helper(void *ctx)
    {
    const char *target_str = target;
    barrier_var(target_str);
    if (local_strncmp(str, cmp_str_len + 1, target_str) < 0)
    __sync_add_and_fetch(&hits, 1);
    return 0;
    }
    SEC("tp/syscalls/sys_enter_getpgid")
#[no_mangle]
pub unsafe extern "C" fn strncmp_helper(ctx: *mut c_void) -> c_int {
    int strncmp_helper(void *ctx)
    {
    if (bpf_strncmp(str, cmp_str_len + 1, target) < 0)
    __sync_add_and_fetch(&hits, 1);
    return 0;
    }
