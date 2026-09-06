//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/uapi/asm/sigcontext.h
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

// FP context was used

// Address error was due to memory load

// Address error was due to memory store

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigcontext {
    pub sc_pc: __u64,
    pub sc_regs: [__u64; 32],
    pub sc_flags: __u32,
    pub __attribute__((__aligned__(16))): __u64 sc_extcontext[0],
}

pub const CONTEXT_INFO_ALIGN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctx_info {
    pub magic: __u32,
    pub size: __u32,
    pub /: *mut *mut __u64 padding; / padding to 16 bytes,
}

// FPU context
pub const FPU_CTX_MAGIC: c_uint = 0x46505501;
pub const FPU_CTX_ALIGN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpu_context {
    pub regs: [__u64; 32],
    pub fcc: __u64,
    pub fcsr: __u32,
}

// LSX context
pub const LSX_CTX_MAGIC: c_uint = 0x53580001;
pub const LSX_CTX_ALIGN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsx_context {
    pub regs: [*mut __u64; 2*32],
    pub fcc: __u64,
    pub fcsr: __u32,
}

// LASX context
pub const LASX_CTX_MAGIC: c_uint = 0x41535801;
pub const LASX_CTX_ALIGN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lasx_context {
    pub regs: [*mut __u64; 4*32],
    pub fcc: __u64,
    pub fcsr: __u32,
}

// LBT context
pub const LBT_CTX_MAGIC: c_uint = 0x42540001;
pub const LBT_CTX_ALIGN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lbt_context {
    pub regs: [__u64; 4],
    pub eflags: __u32,
    pub ftop: __u32,
}
