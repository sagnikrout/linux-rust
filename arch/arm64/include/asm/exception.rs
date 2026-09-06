//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/exception.h
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
// Based on arch/arm/include/asm/exception.h
//
// Copyright (C) 2012 ARM Ltd.
//

extern "C" {
    pub fn handle_bad_stack(regs: *mut pt_regs) -> asmlinkage void __noreturn;
}
extern "C" {
    pub fn el1t_64_sync_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn el1t_64_irq_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn el1t_64_fiq_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn el1t_64_error_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn el1h_64_sync_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn el1h_64_irq_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn el1h_64_fiq_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn el1h_64_error_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn el0t_64_sync_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn el0t_64_irq_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn el0t_64_fiq_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn el0t_64_error_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn el0t_32_sync_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn el0t_32_irq_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn el0t_32_fiq_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn el0t_32_error_handler(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn asm_exit_to_user_mode(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn do_mem_abort(far: c_ulong, esr: c_ulong, regs: *mut pt_regs);
}
extern "C" {
    pub fn do_el0_undef(regs: *mut pt_regs, esr: c_ulong);
}
extern "C" {
    pub fn do_el1_undef(regs: *mut pt_regs, esr: c_ulong);
}
extern "C" {
    pub fn do_el0_bti(regs: *mut pt_regs);
}
extern "C" {
    pub fn do_el1_bti(regs: *mut pt_regs, esr: c_ulong);
}
extern "C" {
    pub fn do_el0_gcs(regs: *mut pt_regs, esr: c_ulong);
}
extern "C" {
    pub fn do_el1_gcs(regs: *mut pt_regs, esr: c_ulong);
}

extern "C" {
    pub fn do_breakpoint(esr: c_ulong, regs: *mut pt_regs);
}

extern "C" {
    pub fn do_el0_softstep(esr: c_ulong, regs: *mut pt_regs);
}
extern "C" {
    pub fn do_el1_softstep(esr: c_ulong, regs: *mut pt_regs);
}
extern "C" {
    pub fn do_el0_brk64(esr: c_ulong, regs: *mut pt_regs);
}
extern "C" {
    pub fn do_el1_brk64(esr: c_ulong, regs: *mut pt_regs);
}
extern "C" {
    pub fn do_bkpt32(esr: c_ulong, regs: *mut pt_regs);
}
extern "C" {
    pub fn do_fpsimd_acc(esr: c_ulong, regs: *mut pt_regs);
}
extern "C" {
    pub fn do_sve_acc(esr: c_ulong, regs: *mut pt_regs);
}
extern "C" {
    pub fn do_sme_acc(esr: c_ulong, regs: *mut pt_regs);
}
extern "C" {
    pub fn do_fpsimd_exc(esr: c_ulong, regs: *mut pt_regs);
}
extern "C" {
    pub fn do_el0_sys(esr: c_ulong, regs: *mut pt_regs);
}
extern "C" {
    pub fn do_sp_pc_abort(addr: c_ulong, esr: c_ulong, regs: *mut pt_regs);
}
extern "C" {
    pub fn bad_el0_sync(regs: *mut pt_regs, reason: c_int, esr: c_ulong);
}
extern "C" {
    pub fn do_el0_cp15(esr: c_ulong, regs: *mut pt_regs);
}
extern "C" {
    pub fn do_compat_alignment_fixup(addr: c_ulong, regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn do_el0_svc(regs: *mut pt_regs);
}
extern "C" {
    pub fn do_el0_svc_compat(regs: *mut pt_regs);
}
extern "C" {
    pub fn do_el0_fpac(regs: *mut pt_regs, esr: c_ulong);
}
extern "C" {
    pub fn do_el1_fpac(regs: *mut pt_regs, esr: c_ulong);
}
extern "C" {
    pub fn do_el0_mops(regs: *mut pt_regs, esr: c_ulong);
}
extern "C" {
    pub fn do_el1_mops(regs: *mut pt_regs, esr: c_ulong);
}
extern "C" {
    pub fn do_serror(regs: *mut pt_regs, esr: c_ulong);
}
extern "C" {
    pub fn panic_bad_stack(regs: *mut pt_regs, esr: c_ulong, far: c_ulong) -> void __noreturn;
}
