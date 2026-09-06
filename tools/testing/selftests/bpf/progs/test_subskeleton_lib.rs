//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_subskeleton_lib.c
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
// Copyright (c) Meta Platforms, Inc. and affiliates.

// volatile to force a read
    const volatile int var1;
    let mut var2: volatile int = 1;
    struct {
    int var3_1;
    __s64 var3_2;
    } var3;
    int libout1;
    extern volatile bool CONFIG_BPF_SYSCALL __kconfig;
    int var4[4];
    __weak int var5 SEC(".data");
// Fully contained within library extern-and-definition
    extern int var6;
    int var7 SEC(".data.custom");
    int (*fn_ptr)(void);
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, __u32);
    __type(value, __u32);
    __uint(max_entries, 16);
    } map1 SEC(".maps");
    extern struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, __u32);
    __type(value, __u32);
    __uint(max_entries, 16);
    } map2 SEC(".maps");
#[no_mangle]
pub unsafe extern "C" fn lib_routine() -> c_int {
    int lib_routine(void)
    {
    let mut key: __u32 = 1, value = 2;
    (void) CONFIG_BPF_SYSCALL;
    bpf_map_update_elem(&map2, &key, &value, BPF_ANY);
    libout1 = var1 + var2 + var3.var3_1 + var3.var3_2 + var5 + var6;
    return libout1;
    }
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn lib_perf_handler(ctx: *mut pt_regs) -> c_int {
    int lib_perf_handler(struct pt_regs *ctx)
    {
    return 0;
    }
    char LICENSE[] SEC("license") = "GPL";
