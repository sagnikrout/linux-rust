//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_reloc_bitfields_probed.c
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

    char _license[] SEC("license") = "GPL";
    struct {
    char in[256];
    char out[256];
    } data = {};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_bitfields {
// unsigned bitfields
    pub 1: uint8_t ub1:,
    pub 2: uint8_t ub2:,
    pub 7: uint32_t ub7:,
// signed bitfields
    pub 4: int8_t sb4:,
    pub 20: int32_t sb20:,
// non-bitfields
    pub u32: u32,
    pub s32: i32,
}

// bitfield read results, all as plain integers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_bitfields_output {
    pub ub1: i64,
    pub ub2: i64,
    pub ub7: i64,
    pub sb4: i64,
    pub sb20: i64,
    pub u32: i64,
    pub s32: i64,
}

    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn test_core_bitfields(ctx: *mut c_void) -> c_int {
    int test_core_bitfields(void *ctx)
    {
    struct core_reloc_bitfields *in = (void *)&data.in;
    struct core_reloc_bitfields_output *out = (void *)&data.out;
    out.ub1 = BPF_CORE_READ_BITFIELD_PROBED(in, ub1);
    out.ub2 = BPF_CORE_READ_BITFIELD_PROBED(in, ub2);
    out.ub7 = BPF_CORE_READ_BITFIELD_PROBED(in, ub7);
    out.sb4 = BPF_CORE_READ_BITFIELD_PROBED(in, sb4);
    out.sb20 = BPF_CORE_READ_BITFIELD_PROBED(in, sb20);
    out.u32 = BPF_CORE_READ_BITFIELD_PROBED(in, u32);
    out.s32 = BPF_CORE_READ_BITFIELD_PROBED(in, s32);
    return 0;
    }
