//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/uprobes.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// User-space Probes (UProbes) for x86
//
// Copyright (C) IBM Corporation, 2008-2011
// Authors:
// Srikar Dronamraju
// Jim Keniston
//

pub type uprobe_opcode_t = u8;
pub const MAX_UINSN_BYTES: c_int = 16;

pub const UPROBE_SWBP_INSN: c_uint = 0xcc;
pub const UPROBE_SWBP_INSN_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_uprobe {
    pub insn: [u8; MAX_UINSN_BYTES],
    pub ixol: [u8; MAX_UINSN_BYTES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_uprobe_task {

    pub saved_scratch_register: c_ulong,

    pub saved_trap_nr: c_uint,
    pub saved_tf: c_uint,
}

extern "C" {
    pub fn is_uprobe_at_func_entry(regs: *mut pt_regs) -> bool;
}

