//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/summarization_freplace.c
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

    SEC("?freplace")
#[no_mangle]
pub unsafe extern "C" fn changes_pkt_data(sk: *mut __sk_buff) -> c_long {
    long changes_pkt_data(struct __sk_buff *sk)
    {
    return bpf_skb_pull_data(sk, 0);
    }
    SEC("?freplace")
#[no_mangle]
pub unsafe extern "C" fn does_not_change_pkt_data(sk: *mut __sk_buff) -> c_long {
    long does_not_change_pkt_data(struct __sk_buff *sk)
    {
    return 0;
    }
    SEC("?freplace")
#[no_mangle]
pub unsafe extern "C" fn might_sleep(ctx: *mut pt_regs) -> c_long {
    long might_sleep(struct pt_regs *ctx)
    {
    int i;
    bpf_copy_from_user(&i, sizeof(i), core::ptr::null_mut());
    return i;
    }
    SEC("?freplace")
#[no_mangle]
pub unsafe extern "C" fn does_not_sleep(ctx: *mut pt_regs) -> c_long {
    long does_not_sleep(struct pt_regs *ctx)
    {
    return 0;
    }
    char _license[] SEC("license") = "GPL";
