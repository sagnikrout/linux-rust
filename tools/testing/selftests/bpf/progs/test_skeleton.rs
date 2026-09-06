//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_skeleton.c
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
// Copyright (c) 2019 Facebook

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s {
    pub a: c_int,
    pub b: c_longlong,
    pub __attribute__((packed)): },
// .data section
    pub -1: int in1 =,
    pub -1: long long in2 =,
// .bss section
    pub '\0': char in3 =,
// C attribute field omitted
    pub {}: s in5 =,
// .rodata section
    const volatile struct {
    pub in6: c_int,
    pub {}: } in =,
// .data section
    pub -1: int out1 =,
    pub -1: long long out2 =,
// .bss section
    pub 0: char out3 =,
    pub 0: long long out4 =,
    pub 0: int out6 =,
    pub __kconfig: extern bool CONFIG_BPF_SYSCALL,
    pub __kconfig: extern int LINUX_KERNEL_VERSION,
    pub 0: bool bpf_syscall =,
    pub 0: int kern_ver =,
    pub {}: s out5 =,
    pub SEC(".rodata.dyn"): volatile int in_dynarr_sz,
    pub }: volatile int in_dynarr[4] SEC(".rodata.dyn") = { -1, -2, -3, -4,
    pub }: int out_dynarr[4] SEC(".data.dyn") = { 1, 2, 3, 4,
    pub __read_mostly: int read_mostly_var,
    pub out_mostly_var: c_int,
    pub 1024]: *mut *mut *mut char huge_arr[16  1024,
// non-mmapable custom .data section
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_value {
    pub SEC(".data.non_mmapable"): __hidden int zero_key,
    pub SEC(".data.non_mmapable"): static struct my_value zero_value,
    struct {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub int): __type(key,,
    pub my_value): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } my_map,
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handler(ctx: *const c_void) -> c_int {
    int handler(const void *ctx)
    {
    pub i: c_int,
    pub in1: out1 =,
    pub in2: out2 =,
    pub in3: out3 =,
    pub in4: out4 =,
    pub in5: out5 =,
    pub in.in6: out6 =,
    pub CONFIG_BPF_SYSCALL: bpf_syscall =,
    pub LINUX_KERNEL_VERSION: kern_ver =,
    pub i++): for (i = 0; i < in_dynarr_sz;,
    pub in_dynarr: [out_dynarr[i] =; i],
    pub read_mostly_var: out_mostly_var =,
    pub 123: huge_arr[sizeof(huge_arr) - 1] =,
// make sure zero_key and zero_value are not optimized out
    pub BPF_ANY): bpf_map_update_elem(&my_map, &zero_key, &zero_value,,
    pub 0: return,
    }
    pub "GPL": char _license[] SEC("license") =,
