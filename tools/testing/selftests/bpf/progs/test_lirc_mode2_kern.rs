//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_lirc_mode2_kern.c
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
// test ir decoder
//
// Copyright (C) 2018 Sean Young <sean@mess.org>

    SEC("lirc_mode2")
#[no_mangle]
pub unsafe extern "C" fn bpf_decoder(sample: *mut c_uint) -> c_int {
    int bpf_decoder(unsigned int *sample)
    {
    if (LIRC_IS_PULSE(*sample)) {
    let mut duration: c_uint = LIRC_VALUE(*sample);
    if (duration & 0x1000)
    bpf_rc_keydown(sample, 0x40, duration & 0xffff, 0);
    if (duration & 0x2000)
    bpf_rc_pointer_rel(sample, (duration >> 8) & 0xff,
    duration & 0xff);
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
