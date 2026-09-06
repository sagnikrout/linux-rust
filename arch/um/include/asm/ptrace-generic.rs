//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/asm/ptrace-generic.h
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
//
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_regs {
    pub regs: uml_pt_regs,
}

pub const PTRACE_OLDSETOPTIONS: c_int = 21;
extern "C" {
    pub fn getreg(child: *mut task_struct, regno: c_int) -> c_ulong;
}
extern "C" {
    pub fn putreg(child: *mut task_struct, regno: c_int, value: c_ulong) -> c_int;
}
extern "C" {
    pub fn poke_user(child: *mut task_struct, addr: c_long, data: c_long) -> c_int;
}
extern "C" {
    pub fn peek_user(child: *mut task_struct, addr: c_long, data: c_long) -> c_int;
}
extern "C" {
    pub fn arch_set_tls(new: *mut task_struct, tls: c_ulong) -> c_int;
}
extern "C" {
    pub fn clear_flushed_tls(task: *mut task_struct);
}
extern "C" {
    pub fn syscall_trace_enter(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn syscall_trace_leave(regs: *mut pt_regs);
}

