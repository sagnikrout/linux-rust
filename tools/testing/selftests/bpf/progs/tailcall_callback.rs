//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tailcall_callback.c
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

    int classifier_0(struct __sk_buff *skb);
    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 1);
    __uint(key_size, sizeof(__u32));
    __array(values, void (void));
    } jmp_table SEC(".maps") = {
    .values = {
    [0] = (void *) &classifier_0,
    },
    };
    __auxiliary
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn classifier_0(skb: *mut __sk_buff) -> c_int {
    int classifier_0(struct __sk_buff *skb)
    {
    return 0;
    }
    static __noinline
#[no_mangle]
pub unsafe extern "C" fn subprog_tail0(skb: *mut __sk_buff) -> c_int {
    int subprog_tail0(struct __sk_buff *skb)
    {
    let mut ret: c_int = 0;
    bpf_tail_call_static(skb, &jmp_table, 0);
    barrier_var(ret);
    return ret;
    }
    static __noinline
#[no_mangle]
pub unsafe extern "C" fn callback_loop(index: c_int, cb_ctx: *mut c_void) -> c_int {
    int callback_loop(int index, void **cb_ctx)
    {
    int ret;
    ret = subprog_tail0(*cb_ctx);
    barrier_var(ret);
    return ret ? 1 : 0;
    }
    static __noinline
#[no_mangle]
pub unsafe extern "C" fn callback_empty(index: c_int, data: *mut c_void) -> c_int {
    int callback_empty(int index, void *data)
    {
    return 0;
    }
// callback involving subprog with tail call is rejected
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(callback": "cannot tail call within) -> __failure {
    __failure __msg("cannot tail call within callback")
#[no_mangle]
pub unsafe extern "C" fn tailcall_callback_1(skb: *mut __sk_buff) -> c_int {
    int tailcall_callback_1(struct __sk_buff *skb)
    {
    clobber_regs_stack();
    bpf_loop(1, callback_loop, &skb, 0);
    return 0;
    }
// subprogs with tailcall do not affect no-tailcall callback
    SEC("tc")
    __success
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn tailcall_callback_2(skb: *mut __sk_buff) -> c_int {
    int tailcall_callback_2(struct __sk_buff *skb)
    {
    int ret;
    clobber_regs_stack();
    ret = subprog_tail0(skb);
    __sink(ret);
    bpf_loop(1, callback_empty, core::ptr::null_mut(), 0);
    return 0;
    }
    char __license[] SEC("license") = "GPL";
