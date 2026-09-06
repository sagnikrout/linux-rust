//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/htab_mem_bench.c
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
// Copyright (C) 2023. Huawei Technologies Co., Ltd

pub const OP_BATCH: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_ctx {
    pub from: c_uint,
    pub step: c_uint,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, 4);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    } htab SEC(".maps");
    char _license[] SEC("license") = "GPL";
    unsigned char zeroed_value[4096];
    let mut nr_thread: c_uint = 0;
    let mut op_cnt: c_long = 0;
#[no_mangle]
unsafe extern "C" fn write_htab(i: c_uint, ctx: *mut update_ctx, flags: c_uint) -> c_int {
    static int write_htab(unsigned int i, struct update_ctx *ctx, unsigned int flags)
    {
    bpf_map_update_elem(&htab, &ctx.from, zeroed_value, flags);
    ctx.from += ctx.step;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn overwrite_htab(i: c_uint, ctx: *mut update_ctx) -> c_int {
    static int overwrite_htab(unsigned int i, struct update_ctx *ctx)
    {
    return write_htab(i, ctx, 0);
    }
#[no_mangle]
unsafe extern "C" fn newwrite_htab(i: c_uint, ctx: *mut update_ctx) -> c_int {
    static int newwrite_htab(unsigned int i, struct update_ctx *ctx)
    {
    return write_htab(i, ctx, BPF_NOEXIST);
    }
#[no_mangle]
unsafe extern "C" fn del_htab(i: c_uint, ctx: *mut update_ctx) -> c_int {
    static int del_htab(unsigned int i, struct update_ctx *ctx)
    {
    bpf_map_delete_elem(&htab, &ctx.from);
    ctx.from += ctx.step;
    return 0;
    }
    SEC("?tp/syscalls/sys_enter_getpgid")
#[no_mangle]
pub unsafe extern "C" fn overwrite(ctx: *mut c_void) -> c_int {
    int overwrite(void *ctx)
    {
    struct update_ctx update;
    update.from = bpf_get_smp_processor_id();
    update.step = nr_thread;
    bpf_loop(OP_BATCH, overwrite_htab, &update, 0);
    __sync_fetch_and_add(&op_cnt, 1);
    return 0;
    }
    SEC("?tp/syscalls/sys_enter_getpgid")
#[no_mangle]
pub unsafe extern "C" fn batch_add_batch_del(ctx: *mut c_void) -> c_int {
    int batch_add_batch_del(void *ctx)
    {
    struct update_ctx update;
    update.from = bpf_get_smp_processor_id();
    update.step = nr_thread;
    bpf_loop(OP_BATCH, overwrite_htab, &update, 0);
    update.from = bpf_get_smp_processor_id();
    bpf_loop(OP_BATCH, del_htab, &update, 0);
    __sync_fetch_and_add(&op_cnt, 2);
    return 0;
    }
    SEC("?tp/syscalls/sys_enter_getpgid")
#[no_mangle]
pub unsafe extern "C" fn add_only(ctx: *mut c_void) -> c_int {
    int add_only(void *ctx)
    {
    struct update_ctx update;
    update.from = bpf_get_smp_processor_id() / 2;
    update.step = nr_thread / 2;
    bpf_loop(OP_BATCH, newwrite_htab, &update, 0);
    __sync_fetch_and_add(&op_cnt, 1);
    return 0;
    }
    SEC("?tp/syscalls/sys_enter_getppid")
#[no_mangle]
pub unsafe extern "C" fn del_only(ctx: *mut c_void) -> c_int {
    int del_only(void *ctx)
    {
    struct update_ctx update;
    update.from = bpf_get_smp_processor_id() / 2;
    update.step = nr_thread / 2;
    bpf_loop(OP_BATCH, del_htab, &update, 0);
    __sync_fetch_and_add(&op_cnt, 1);
    return 0;
    }
