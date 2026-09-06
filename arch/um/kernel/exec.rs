//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/exec.c
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

#[no_mangle]
pub unsafe extern "C" fn flush_thread() {
    void flush_thread(void)
    {
    arch_flush_thread(&current.thread.arch);
    get_safe_registers(current_pt_regs().regs.gp,
    current_pt_regs().regs.fp);
    }
#[no_mangle]
pub unsafe extern "C" fn start_thread(regs: *mut pt_regs, eip: c_ulong, esp: c_ulong) {
    void start_thread(struct pt_regs *regs, unsigned long eip, unsigned long esp)
    {
    PT_REGS_IP(regs) = eip;
    PT_REGS_SP(regs) = esp;
    clear_thread_flag(TIF_SINGLESTEP);
    }
    EXPORT_SYMBOL(start_thread);
