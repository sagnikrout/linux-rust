//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_snprintf.c
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
// Copyright (c) 2021 Google LLC.

    let mut pid: __u32 = 0;
    char num_out[64] = {};
    let mut num_ret: c_long = 0;
    char ip_out[64] = {};
    let mut ip_ret: c_long = 0;
    char sym_out[64] = {};
    let mut sym_ret: c_long = 0;
    char addr_out[64] = {};
    let mut addr_ret: c_long = 0;
    char str_out[64] = {};
    let mut str_ret: c_long = 0;
    char over_out[6] = {};
    let mut over_ret: c_long = 0;
    char pad_out[10] = {};
    let mut pad_ret: c_long = 0;
    char noarg_out[64] = {};
    let mut noarg_ret: c_long = 0;
    let mut nobuf_ret: c_long = 0;
    extern const void schedule __ksym;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handler(ctx: *const c_void) -> c_int {
    int handler(const void *ctx)
    {
// Convenient values to pretty-print
    const __u8 ex_ipv4[] = {127, 0, 0, 1};
    const __u8 ex_ipv6[] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1};
    static const char str1[] = "str1";
    static const char longstr[] = "longstr";
    if ((int)bpf_get_current_pid_tgid() != pid)
    return 0;
// Integer types
    num_ret  = BPF_SNPRINTF(num_out, sizeof(num_out),
    "%d %u %x %li %llu %lX",
    -8, 9, 150, -424242, 1337, 0xDABBAD00);
// IP addresses
    ip_ret   = BPF_SNPRINTF(ip_out, sizeof(ip_out), "%pi4 %pI6",
    &ex_ipv4, &ex_ipv6);
// Symbol lookup formatting
    sym_ret  = BPF_SNPRINTF(sym_out,  sizeof(sym_out), "%ps %pS %pB",
    &schedule, &schedule, &schedule);
// Kernel pointers
    addr_ret = BPF_SNPRINTF(addr_out, sizeof(addr_out), "%pK %px %p",
    0, 0xFFFF00000ADD4E55, 0xFFFF00000ADD4E55);
// Strings and single-byte character embedding
    str_ret  = BPF_SNPRINTF(str_out, sizeof(str_out), "%s % 9c %+2c %-3c %04c %0c %+05s",
    str1, 'a', 'b', 'c', 'd', 'e', longstr);
// Overflow
    over_ret = BPF_SNPRINTF(over_out, sizeof(over_out), "%%overflow");
// Padding of fixed width numbers
    pad_ret = BPF_SNPRINTF(pad_out, sizeof(pad_out), "%5d %0900000X", 4, 4);
// No args
    noarg_ret = BPF_SNPRINTF(noarg_out, sizeof(noarg_out), "simple case");
// No buffer
    nobuf_ret = BPF_SNPRINTF(core::ptr::null_mut(), 0, "only interested in length %d", 60);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
