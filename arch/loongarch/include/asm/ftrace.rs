//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/ftrace.h
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
// Copyright (C) 2022 Loongson Technology Corporation Limited
//
pub const FTRACE_PLT_IDX: c_int = 0;
pub const FTRACE_REGS_PLT_IDX: c_int = 1;
pub const NR_FTRACE_PLTS: c_int = 2;

extern "C" {
    pub fn _mcount();
}
extern "C" {
    pub fn prepare_ftrace_return(self_addr: c_ulong, callsite_sp: c_ulong, old: c_ulong);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dyn_arch_ftrace {
pub const ARCH_SUPPORTS_FTRACE_OPS: c_int = 1;

    pub rec): *mut *mut int ftrace_init_nop(struct module mod, struct dyn_ftrace,
    pub addr: return,
    pub parent): *mut void prepare_ftrace_return(unsigned long self_addr, unsigned long,

    pub ftrace_ops: struct,

    pub &arch_ftrace_regs(fregs)->regs: return,
    pub ip): instruction_pointer_set(&arch_ftrace_regs(fregs)->regs,,

    pub )(arch_ftrace_regs(fregs)->regs.regs[1]): *mut *mut return (unsigned long,

    pub fregs): *mut *mut ftrace_ops op, ftrace_regs,

    pub /: *mut *mut regs->regs[13] = addr; / t1,

