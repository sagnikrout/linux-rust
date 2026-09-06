//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/uapi/asm/ptrace.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Author: Hanlu Li <lihanlu@loongson.cn>
// Huacai Chen <chenhuacai@loongson.cn>
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

//
// For PTRACE_{POKE,PEEK}USR. 0 - 31 are GPRs,
// 32 is syscall's original ARG0, 33 is PC, 34 is BADVADDR.
//
pub const GPR_BASE: c_int = 0;
pub const GPR_NUM: c_int = 32;

pub const NUM_FPU_REGS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_pt_regs {
// Main processor registers.
    pub regs: [c_ulong; 32],
// Original syscall arg0.
    pub orig_a0: c_ulong,
// Special CSR registers.
    pub csr_era: c_ulong,
    pub csr_badv: c_ulong,
    pub reserved: [c_ulong; 10],
    pub __attribute__((aligned(8))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_fp_state {
    pub fpr: [__u64; 32],
    pub fcc: __u64,
    pub fcsr: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_lsx_state {
// 32 registers, 128 bits width per register.
    pub vregs: [*mut __u64; 32*2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_lasx_state {
// 32 registers, 256 bits width per register.
    pub vregs: [*mut __u64; 32*4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_lbt_state {
    pub scr: [__u64; 4],
    pub eflags: __u32,
    pub ftop: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_watch_state {
    pub dbg_info: __u64,

    pub addr: __u32,
    pub mask: __u32,

    pub addr: __u64,
    pub mask: __u64,

    pub ctrl: __u32,
    pub pad: __u32,
    pub dbg_regs: [}; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_watch_state_v2 {
    pub dbg_info: __u64,

    pub addr: __u32,
    pub mask: __u32,

    pub addr: __u64,
    pub mask: __u64,

    pub ctrl: __u32,
    pub pad: __u32,
    pub dbg_regs: [}; 14],
}

pub const PTRACE_SYSEMU: c_uint = 0x1f;
pub const PTRACE_SYSEMU_SINGLESTEP: c_uint = 0x20;
