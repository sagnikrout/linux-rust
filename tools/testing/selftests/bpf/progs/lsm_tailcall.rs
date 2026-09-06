//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/lsm_tailcall.c
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
// Copyright (c) 2024 Huawei Technologies Co., Ltd

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 1);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
    } jmp_table SEC(".maps");
    SEC("lsm/file_permission")
#[no_mangle]
pub unsafe extern "C" fn lsm_file_permission_prog(ctx: *mut c_void) -> c_int {
    int lsm_file_permission_prog(void *ctx)
    {
    return 0;
    }
    SEC("lsm/kernfs_init_security")
#[no_mangle]
pub unsafe extern "C" fn lsm_kernfs_init_security_prog(ctx: *mut c_void) -> c_int {
    int lsm_kernfs_init_security_prog(void *ctx)
    {
    return 0;
    }
    SEC("lsm/kernfs_init_security")
#[no_mangle]
pub unsafe extern "C" fn lsm_kernfs_init_security_entry(ctx: *mut c_void) -> c_int {
    int lsm_kernfs_init_security_entry(void *ctx)
    {
    bpf_tail_call_static(ctx, &jmp_table, 0);
    return 0;
    }
