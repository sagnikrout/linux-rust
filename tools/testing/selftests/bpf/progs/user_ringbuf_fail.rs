//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/user_ringbuf_fail.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sample {
    pub pid: c_int,
    pub seq: c_int,
    pub value: c_long,
    pub comm: [c_char; 16],
}

    struct {
    __uint(type, BPF_MAP_TYPE_USER_RINGBUF);
    __uint(max_entries, 4096);
    } user_ringbuf SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 2);
    } ringbuf SEC(".maps");
    static int map_value;
    static long
    bad_access1(struct bpf_dynptr *dynptr, void *context)
    {
    const struct sample *sample;
    sample = bpf_dynptr_data(dynptr - 1, 0, sizeof(*sample));
    bpf_printk("Was able to pass bad pointer %lx\n", (__u64)dynptr - 1);
    return 0;
    }
// A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
// not be able to read before the pointer.
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(ptr": "negative offset dynptr_ptr) -> __failure {
    __failure __msg("negative offset dynptr_ptr ptr")
#[no_mangle]
pub unsafe extern "C" fn user_ringbuf_callback_bad_access1(ctx: *mut c_void) -> c_int {
    int user_ringbuf_callback_bad_access1(void *ctx)
    {
    bpf_user_ringbuf_drain(&user_ringbuf, bad_access1, core::ptr::null_mut(), 0);
    return 0;
    }
    static long
    bad_access2(struct bpf_dynptr *dynptr, void *context)
    {
    const struct sample *sample;
    sample = bpf_dynptr_data(dynptr + 1, 0, sizeof(*sample));
    bpf_printk("Was able to pass bad pointer %lx\n", (__u64)dynptr + 1);
    return 0;
    }
// A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
// not be able to read past the end of the pointer.
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(ptr": "dereference of modified dynptr_ptr) -> __failure {
    __failure __msg("dereference of modified dynptr_ptr ptr")
#[no_mangle]
pub unsafe extern "C" fn user_ringbuf_callback_bad_access2(ctx: *mut c_void) -> c_int {
    int user_ringbuf_callback_bad_access2(void *ctx)
    {
    bpf_user_ringbuf_drain(&user_ringbuf, bad_access2, core::ptr::null_mut(), 0);
    return 0;
    }
    static long
    write_forbidden(struct bpf_dynptr *dynptr, void *context)
    {
// ((long *)dynptr) = 0;
    return 0;
    }
// A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
// not be able to write to that pointer.
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('dynptr_ptr'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'dynptr_ptr'")
#[no_mangle]
pub unsafe extern "C" fn user_ringbuf_callback_write_forbidden(ctx: *mut c_void) -> c_int {
    int user_ringbuf_callback_write_forbidden(void *ctx)
    {
    bpf_user_ringbuf_drain(&user_ringbuf, write_forbidden, core::ptr::null_mut(), 0);
    return 0;
    }
    static long
    null_context_write(struct bpf_dynptr *dynptr, void *context)
    {
// ((__u64 *)context) = 0;
    return 0;
    }
// A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
// not be able to write to that pointer.
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn user_ringbuf_callback_null_context_write(ctx: *mut c_void) -> c_int {
    int user_ringbuf_callback_null_context_write(void *ctx)
    {
    bpf_user_ringbuf_drain(&user_ringbuf, null_context_write, core::ptr::null_mut(), 0);
    return 0;
    }
    static long
    null_context_read(struct bpf_dynptr *dynptr, void *context)
    {
    let mut id: __u64 = *((__u64 *)context);
    bpf_printk("Read id %lu\n", id);
    return 0;
    }
// A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
// not be able to write to that pointer.
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn user_ringbuf_callback_null_context_read(ctx: *mut c_void) -> c_int {
    int user_ringbuf_callback_null_context_read(void *ctx)
    {
    bpf_user_ringbuf_drain(&user_ringbuf, null_context_read, core::ptr::null_mut(), 0);
    return 0;
    }
    static long
    try_discard_dynptr(struct bpf_dynptr *dynptr, void *context)
    {
    bpf_ringbuf_discard_dynptr(dynptr, 0);
    return 0;
    }
// A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
// not be able to read past the end of the pointer.
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(released": "CONST_PTR_TO_DYNPTR cannot be) -> __failure {
    __failure __msg("CONST_PTR_TO_DYNPTR cannot be released")
#[no_mangle]
pub unsafe extern "C" fn user_ringbuf_callback_discard_dynptr(ctx: *mut c_void) -> c_int {
    int user_ringbuf_callback_discard_dynptr(void *ctx)
    {
    bpf_user_ringbuf_drain(&user_ringbuf, try_discard_dynptr, core::ptr::null_mut(), 0);
    return 0;
    }
    static long
    try_submit_dynptr(struct bpf_dynptr *dynptr, void *context)
    {
    bpf_ringbuf_submit_dynptr(dynptr, 0);
    return 0;
    }
// A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
// not be able to read past the end of the pointer.
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(released": "CONST_PTR_TO_DYNPTR cannot be) -> __failure {
    __failure __msg("CONST_PTR_TO_DYNPTR cannot be released")
#[no_mangle]
pub unsafe extern "C" fn user_ringbuf_callback_submit_dynptr(ctx: *mut c_void) -> c_int {
    int user_ringbuf_callback_submit_dynptr(void *ctx)
    {
    bpf_user_ringbuf_drain(&user_ringbuf, try_submit_dynptr, core::ptr::null_mut(), 0);
    return 0;
    }
    static long
    invalid_drain_callback_return(struct bpf_dynptr *dynptr, void *context)
    {
    return 2;
    }
// A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
// not be able to write to that pointer.
//
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(": "At callback return the register R0 has) -> __failure {
    __failure __msg("At callback return the register R0 has ")
#[no_mangle]
pub unsafe extern "C" fn user_ringbuf_callback_invalid_return(ctx: *mut c_void) -> c_int {
    int user_ringbuf_callback_invalid_return(void *ctx)
    {
    bpf_user_ringbuf_drain(&user_ringbuf, invalid_drain_callback_return, core::ptr::null_mut(), 0);
    return 0;
    }
    static long
    try_reinit_dynptr_mem(struct bpf_dynptr *dynptr, void *context)
    {
    bpf_dynptr_from_mem(&map_value, 4, 0, dynptr);
    return 0;
    }
    static long
    try_reinit_dynptr_ringbuf(struct bpf_dynptr *dynptr, void *context)
    {
    bpf_ringbuf_reserve_dynptr(&ringbuf, 8, 0, dynptr);
    return 0;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(dynptr": "Dynptr has to be an uninitialized) -> __failure {
    __failure __msg("Dynptr has to be an uninitialized dynptr")
#[no_mangle]
pub unsafe extern "C" fn user_ringbuf_callback_reinit_dynptr_mem(ctx: *mut c_void) -> c_int {
    int user_ringbuf_callback_reinit_dynptr_mem(void *ctx)
    {
    bpf_user_ringbuf_drain(&user_ringbuf, try_reinit_dynptr_mem, core::ptr::null_mut(), 0);
    return 0;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(dynptr": "Dynptr has to be an uninitialized) -> __failure {
    __failure __msg("Dynptr has to be an uninitialized dynptr")
#[no_mangle]
pub unsafe extern "C" fn user_ringbuf_callback_reinit_dynptr_ringbuf(ctx: *mut c_void) -> c_int {
    int user_ringbuf_callback_reinit_dynptr_ringbuf(void *ctx)
    {
    bpf_user_ringbuf_drain(&user_ringbuf, try_reinit_dynptr_ringbuf, core::ptr::null_mut(), 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn global_call_bpf_dynptr_data(dynptr: *mut bpf_dynptr) -> __noinline long {
    __noinline long global_call_bpf_dynptr_data(struct bpf_dynptr *dynptr)
    {
    bpf_dynptr_data(dynptr, 0xA, 0xA);
    return 0;
    }
    static long callback_adjust_bpf_dynptr_reg_off(struct bpf_dynptr *dynptr,
    void *ctx)
    {
    global_call_bpf_dynptr_data(dynptr += 1024);
    return 0;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(disallowed": "dereference of modified dynptr_ptr ptr R1 off=16384) -> __failure {
    __failure __msg("dereference of modified dynptr_ptr ptr R1 off=16384 disallowed")
#[no_mangle]
pub unsafe extern "C" fn user_ringbuf_callback_const_ptr_to_dynptr_reg_off(ctx: *mut c_void) -> c_int {
    int user_ringbuf_callback_const_ptr_to_dynptr_reg_off(void *ctx)
    {
    bpf_user_ringbuf_drain(&user_ringbuf,
    callback_adjust_bpf_dynptr_reg_off, core::ptr::null_mut(), 0);
    return 0;
    }
