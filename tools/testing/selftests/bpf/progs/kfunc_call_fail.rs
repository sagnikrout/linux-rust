//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/kfunc_call_fail.c
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
// Copyright (c) 2021 Facebook

    static struct bpf_spin_lock kfunc_call_lock SEC(".data.A");
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn kfunc_call_test_spin_lock_unsafe(skb: *mut __sk_buff) -> c_int {
    int kfunc_call_test_spin_lock_unsafe(struct __sk_buff *skb)
    {
    bpf_spin_lock(&kfunc_call_lock);
    bpf_kfunc_trigger_ctx_check();
    bpf_spin_unlock(&kfunc_call_lock);
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_test_args {
    pub data: [__u8; 16],
    pub size: usize,
}

    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn kfunc_syscall_test_fail(args: *mut syscall_test_args) -> c_int {
    int kfunc_syscall_test_fail(struct syscall_test_args *args)
    {
    bpf_kfunc_call_test_mem_len_pass1(&args.data, sizeof(*args) + 1);
    return 0;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn kfunc_syscall_test_null_fail(args: *mut syscall_test_args) -> c_int {
    int kfunc_syscall_test_null_fail(struct syscall_test_args *args)
    {
// Must be called with args as a NULL pointer
// we do not check for it to have the verifier consider that
// the pointer might not be null, and so we can load it.
//
// So the following can not be added:
//
// if (args)
// return -22;
//
    bpf_kfunc_call_test_mem_len_pass1(args, sizeof(*args));
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn kfunc_call_test_get_mem_fail_rdonly(skb: *mut __sk_buff) -> c_int {
    int kfunc_call_test_get_mem_fail_rdonly(struct __sk_buff *skb)
    {
    struct prog_test_ref_kfunc *pt;
    let mut s: c_ulong = 0;
    int *p = core::ptr::null_mut();
    let mut ret: c_int = 0;
    pt = bpf_kfunc_call_test_acquire(&s);
    if (pt) {
    p = bpf_kfunc_call_test_get_rdonly_mem(pt, 2 * sizeof(int));
    if (p)
    p[0] = 42; /* this is a read-only buffer, so -EACCES */
    else
    ret = -1;
    bpf_kfunc_call_test_release(pt);
    }
    return ret;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn kfunc_call_test_get_mem_fail_use_after_free(skb: *mut __sk_buff) -> c_int {
    int kfunc_call_test_get_mem_fail_use_after_free(struct __sk_buff *skb)
    {
    struct prog_test_ref_kfunc *pt;
    let mut s: c_ulong = 0;
    int *p = core::ptr::null_mut();
    let mut ret: c_int = 0;
    pt = bpf_kfunc_call_test_acquire(&s);
    if (pt) {
    p = bpf_kfunc_call_test_get_rdwr_mem(pt, 2 * sizeof(int));
    if (p) {
    p[0] = 42;
    ret = p[1]; /* 108 */
    } else {
    ret = -1;
    }
    bpf_kfunc_call_test_release(pt);
    }
    if (p)
    ret = p[0]; /* p is not valid anymore */
    return ret;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn kfunc_call_test_get_mem_fail_oob(skb: *mut __sk_buff) -> c_int {
    int kfunc_call_test_get_mem_fail_oob(struct __sk_buff *skb)
    {
    struct prog_test_ref_kfunc *pt;
    let mut s: c_ulong = 0;
    int *p = core::ptr::null_mut();
    let mut ret: c_int = 0;
    pt = bpf_kfunc_call_test_acquire(&s);
    if (pt) {
    p = bpf_kfunc_call_test_get_rdonly_mem(pt, 2 * sizeof(int));
    if (p)
    ret = p[2 * sizeof(int)]; /* oob access, so -EACCES */
    else
    ret = -1;
    bpf_kfunc_call_test_release(pt);
    }
    return ret;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn kfunc_call_test_get_mem_fail_zero_size(skb: *mut __sk_buff) -> c_int {
    int kfunc_call_test_get_mem_fail_zero_size(struct __sk_buff *skb)
    {
    struct prog_test_ref_kfunc *pt;
    let mut s: c_ulong = 0;
    int *p = core::ptr::null_mut();
    let mut ret: c_int = 0;
    pt = bpf_kfunc_call_test_acquire(&s);
    if (pt) {
//
// An explicit rdwr_buf_size of 0 gives R0 a zero-sized buffer,
// so any access is out of bounds, hence -EACCES. Previously the
// verifier treated a zero size as "no size argument" and sized
// R0 after the pointed-to return type, wrongly allowing the read.
//
    p = bpf_kfunc_call_test_get_rdwr_mem(pt, 0);
    if (p)
    ret = p[0];
    else
    ret = -1;
    bpf_kfunc_call_test_release(pt);
    }
    return ret;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn kfunc_call_test_get_mem_fail_oversized(skb: *mut __sk_buff) -> c_int {
    int kfunc_call_test_get_mem_fail_oversized(struct __sk_buff *skb)
    {
    struct prog_test_ref_kfunc *pt;
    let mut s: c_ulong = 0;
    int *p = core::ptr::null_mut();
    let mut ret: c_int = 0;
    pt = bpf_kfunc_call_test_acquire(&s);
    if (pt) {
//
// rdwr_buf_size is a const int, so a C literal is narrowed to
// 32 bits before the call. Force the full 64-bit value 2^64 - 192
// (0xffffffffffffff40, > U32_MAX) into the argument register with
// a 64-bit immediate load. The verifier records r0_size from the
// full register value and must reject it before that value is
// truncated into R0's u32 mem_size.
//
    asm volatile (
    "r1 = %[pt];"
    "r2 = %[oversized] ll;"
    "call %[get_rdwr_mem];"
    "%[p] = r0;"
    : [p] "=r"(p)
    : [pt] "r"(pt),
    [oversized] "i"(0xffffffffffffff40LL),
    [get_rdwr_mem] "i"(bpf_kfunc_call_test_get_rdwr_mem)
    : "r0", "r1", "r2", "r3", "r4", "r5");
    bpf_kfunc_call_test_release(pt);
    }
    return ret;
    }
    let mut not_const_size: c_int = 2 * sizeof(int);
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn kfunc_call_test_get_mem_fail_not_const(skb: *mut __sk_buff) -> c_int {
    int kfunc_call_test_get_mem_fail_not_const(struct __sk_buff *skb)
    {
    struct prog_test_ref_kfunc *pt;
    let mut s: c_ulong = 0;
    int *p = core::ptr::null_mut();
    let mut ret: c_int = 0;
    pt = bpf_kfunc_call_test_acquire(&s);
    if (pt) {
    p = bpf_kfunc_call_test_get_rdonly_mem(pt, not_const_size); /* non const size, -EINVAL */
    if (p)
    ret = p[0];
    else
    ret = -1;
    bpf_kfunc_call_test_release(pt);
    }
    return ret;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn kfunc_call_test_mem_acquire_fail(skb: *mut __sk_buff) -> c_int {
    int kfunc_call_test_mem_acquire_fail(struct __sk_buff *skb)
    {
    struct prog_test_ref_kfunc *pt;
    let mut s: c_ulong = 0;
    int *p = core::ptr::null_mut();
    let mut ret: c_int = 0;
    pt = bpf_kfunc_call_test_acquire(&s);
    if (pt) {
// we are failing on this one, because we are not acquiring a PTR_TO_BTF_ID (a struct ptr)
    p = bpf_kfunc_call_test_acq_rdonly_mem(pt, 2 * sizeof(int));
    if (p)
    ret = p[0];
    else
    ret = -1;
    bpf_kfunc_call_int_mem_release(p);
    bpf_kfunc_call_test_release(pt);
    }
    return ret;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn kfunc_call_test_pointer_arg_type_mismatch(skb: *mut __sk_buff) -> c_int {
    int kfunc_call_test_pointer_arg_type_mismatch(struct __sk_buff *skb)
    {
    bpf_kfunc_call_test_pass_ctx((void *)10);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
