//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/csum_diff_test.c
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
// Copyright Amazon.com Inc. or its affiliates

pub const BUFF_SZ: c_int = 512;
// Will be updated by benchmark before program loading
    char to_buff[BUFF_SZ];
    let mut to_buff_len: volatile unsigned int = 0;
    char from_buff[BUFF_SZ];
    let mut from_buff_len: volatile unsigned int = 0;
    let mut seed: c_ushort = 0;
    short result;
    char _license[] SEC("license") = "GPL";
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn compute_checksum(ctx: *mut c_void) -> c_int {
    int compute_checksum(void *ctx)
    {
    let mut to_len_half: c_int = to_buff_len / 2;
    let mut from_len_half: c_int = from_buff_len / 2;
    short result2;
// Calculate checksum in one go
    result2 = bpf_csum_diff((void *)from_buff, from_buff_len,
    (void *)to_buff, to_buff_len, seed);
// Calculate checksum by concatenating bpf_csum_diff()
    result = bpf_csum_diff((void *)from_buff, from_buff_len - from_len_half,
    (void *)to_buff, to_buff_len - to_len_half, seed);
    result = bpf_csum_diff((void *)from_buff + (from_buff_len - from_len_half), from_len_half,
    (void *)to_buff + (to_buff_len - to_len_half), to_len_half, result);
    result = (result == result2) ? result : 0;
    return 0;
    }
