//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/kprobes.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copied from arch/arm64/include/asm/kprobes.h
//
// Copyright (C) 2013 Linaro Limited
// Copyright (C) 2017 SiFive
//

pub const MAX_INSN_SIZE: c_int = 2;

pub const kretprobe_blacklist_size: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prev_kprobe {
    pub kp: *mut kprobe,
    pub status: c_uint,
}

// per-cpu kprobe control block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kprobe_ctlblk {
    pub kprobe_status: c_uint,
    pub saved_status: c_ulong,
    pub prev_kprobe: prev_kprobe,
}

extern "C" {
    pub fn arch_remove_kprobe(p: *mut kprobe);
}
extern "C" {
    pub fn kprobe_fault_handler(regs: *mut pt_regs, trapnr: c_uint) -> c_int;
}
extern "C" {
    pub fn kprobe_breakpoint_handler(regs: *mut pt_regs) -> bool;
}
extern "C" {
    pub fn kprobe_single_step_handler(regs: *mut pt_regs) -> bool;
}

