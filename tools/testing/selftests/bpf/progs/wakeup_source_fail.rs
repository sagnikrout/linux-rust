//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/wakeup_source_fail.c
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
// Copyright 2026 Google LLC

    struct bpf_ws_lock;
    struct bpf_ws_lock *bpf_wakeup_sources_read_lock(void) __ksym;
    void bpf_wakeup_sources_read_unlock(struct bpf_ws_lock *lock) __ksym;
    void *bpf_wakeup_sources_get_head(void) __ksym;
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(leak": "BPF_EXIT instruction in main prog would lead to reference) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog would lead to reference leak")
#[no_mangle]
pub unsafe extern "C" fn wakeup_source_lock_no_unlock(ctx: *mut c_void) -> c_int {
    int wakeup_source_lock_no_unlock(void *ctx)
    {
    struct bpf_ws_lock *lock;
    lock = bpf_wakeup_sources_read_lock();
    if (!lock)
    return 0;
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(struct": "access beyond) -> __failure {
    __failure __msg("access beyond struct")
#[no_mangle]
pub unsafe extern "C" fn wakeup_source_access_lock_fields(ctx: *mut c_void) -> c_int {
    int wakeup_source_access_lock_fields(void *ctx)
    {
    struct bpf_ws_lock *lock;
    int val;
    lock = bpf_wakeup_sources_read_lock();
    if (!lock)
    return 0;
    val = *(int *)lock;
    bpf_wakeup_sources_read_unlock(lock);
    return val;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "release kfunc bpf_wakeup_sources_read_unlock expects referenced PTR_TO_BTF_ID passed to) -> __failure {
    __failure __msg("release kfunc bpf_wakeup_sources_read_unlock expects referenced PTR_TO_BTF_ID passed to R1")
#[no_mangle]
pub unsafe extern "C" fn wakeup_source_unlock_no_lock(ctx: *mut c_void) -> c_int {
    int wakeup_source_unlock_no_lock(void *ctx)
    {
    struct bpf_ws_lock *lock = (void *)0x1;
    bpf_wakeup_sources_read_unlock(lock);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(trusted": "Possibly NULL pointer passed to) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted")
#[no_mangle]
pub unsafe extern "C" fn wakeup_source_unlock_null(ctx: *mut c_void) -> c_int {
    int wakeup_source_unlock_null(void *ctx)
    {
    bpf_wakeup_sources_read_unlock(core::ptr::null_mut());
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "R0 invalid mem access) -> __failure {
    __failure __msg("R0 invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn wakeup_source_unsafe_dereference(ctx: *mut c_void) -> c_int {
    int wakeup_source_unsafe_dereference(void *ctx)
    {
    struct list_head *head = bpf_wakeup_sources_get_head();
    if (head.next)
    return 1;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
