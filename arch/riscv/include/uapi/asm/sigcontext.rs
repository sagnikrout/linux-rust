//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/uapi/asm/sigcontext.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// Copyright (C) 2012 Regents of the University of California
//

// The Magic number for signal context frame header.
pub const RISCV_V_MAGIC: c_uint = 0x53465457;
pub const RISCV_ZICFISS_MAGIC: c_uint = 0x9487;
pub const END_MAGIC: c_uint = 0x0;
// The size of END signal context header.
pub const END_HDR_SIZE: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sc_riscv_v_state {
    pub v_state: __riscv_v_ext_state,
    pub __attribute__((aligned(16))): },
//
// Signal context structure
//
// This contains the context saved before a signal handler is invoked;
// it is restored by sys_rt_sigreturn.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigcontext {
    pub sc_regs: user_regs_struct,
    pub sc_fpregs: __riscv_fp_state,
    pub sc_extdesc: __riscv_extra_ext_header,
}

