//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/traps.h
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
// Based on arch/arm/include/asm/traps.h
//
// Copyright (C) 2012 ARM Ltd.
//

extern "C" {
    pub fn try_emulate_armv8_deprecated(regs: *mut pt_regs, insn: u32) -> bool;
}

extern "C" {
    pub fn force_signal_inject(signal: c_int, code: c_int, address: c_ulong, err: c_ulong);
}
extern "C" {
    pub fn arm64_notify_segfault(addr: c_ulong);
}
extern "C" {
    pub fn arm64_force_sig_fault(signo: c_int, code: c_int, far: c_ulong, str: *const c_char);
}
extern "C" {
    pub fn arm64_force_sig_fault_pkey(far: c_ulong, str: *const c_char, pkey: c_int);
}
extern "C" {
    pub fn arm64_force_sig_mceerr(code: c_int, far: c_ulong, lsb: c_short, str: *const c_char);
}
extern "C" {
    pub fn arm64_force_sig_ptrace_errno_trap(errno: c_int, far: c_ulong, str: *const c_char);
}
extern "C" {
    pub fn bug_brk_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int;
}
extern "C" {
    pub fn cfi_brk_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int;
}
extern "C" {
    pub fn reserved_fault_brk_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int;
}
extern "C" {
    pub fn kasan_brk_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int;
}
extern "C" {
    pub fn ubsan_brk_handler(regs: *mut pt_regs, esr: c_ulong) -> c_int;
}
extern "C" {
    pub fn early_brk64(addr: c_ulong, esr: c_ulong, regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn dump_kernel_instr(kaddr: c_ulong);
}
//
// Move regs->pc to next instruction and do necessary setup before it
// is executed.
//
extern "C" {
    pub fn arm64_skip_faulting_instruction(regs: *mut pt_regs, size: c_ulong);
}
//
// CPUs with the RAS extensions have an Implementation-Defined-Syndrome bit
// to indicate whether this ESR has a RAS encoding. CPUs without this feature
// have a ISS-Valid bit in the same position.
// If this bit is set, we know its not a RAS SError.
// If its clear, we need to know if the CPU supports RAS. Uncategorized RAS
// errors share the same encoding as an all-zeros encoding from a CPU that
// doesn't support RAS.
//
// Return the AET bits from a RAS SError's ESR.
//
// It is implementation defined whether Uncategorized errors are containable.
// We treat them as Uncontainable.
// Non-RAS SError's are reported as Uncontained/Uncategorized.
//
// Not a RAS error, we can't interpret the ESR.
//
// AET is RES0 if 'the value returned in the DFSC field is not
// [ESR_ELx_FSC_SERROR]'
//
// No severity information : Uncategorized
extern "C" {
    pub fn arm64_is_fatal_ras_serror(regs: *mut pt_regs, esr: c_ulong) -> bool;
}
extern "C" {
    pub fn arm64_serror_panic(regs: *mut pt_regs, esr: c_ulong) -> void __noreturn;
}
//
// Put the registers back in the original format suitable for a
// prologue instruction, using the generic return routine from the
// Arm ARM (DDI 0487I.a) rules CNTMJ and MWFQH.
//
// SET* instruction
// Format is from Option A; forward set
// CPY* instruction
// Format is from Option B
// Backward copy
// Format is from Option A
// Forward copy
