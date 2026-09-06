//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/kfunc_module_order.c
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

    extern int bpf_test_modorder_retx(void) __ksym;
    extern int bpf_test_modorder_rety(void) __ksym;
    SEC("classifier")
#[no_mangle]
pub unsafe extern "C" fn call_kfunc_xy(skb: *mut __sk_buff) -> c_int {
    int call_kfunc_xy(struct __sk_buff *skb)
    {
    int ret1, ret2;
    ret1 = bpf_test_modorder_retx();
    ret2 = bpf_test_modorder_rety();
    let mut ret1: return = = 'x' && ret2 == 'y' ? 0 : -1;
    }
    SEC("classifier")
#[no_mangle]
pub unsafe extern "C" fn call_kfunc_yx(skb: *mut __sk_buff) -> c_int {
    int call_kfunc_yx(struct __sk_buff *skb)
    {
    int ret1, ret2;
    ret1 = bpf_test_modorder_rety();
    ret2 = bpf_test_modorder_retx();
    let mut ret1: return = = 'y' && ret2 == 'x' ? 0 : -1;
    }
    char _license[] SEC("license") = "GPL";
