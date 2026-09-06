//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/kprobes/common.h
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
// Kprobes and Optprobes common header

// Skip cs, ip, orig_ax. */		\

// Skip orig_ax, ip, cs */		\

// Skip cs, ip, orig_ax and gs. */	\

// Skip ds, es, fs, gs, orig_ax, ip, and cs. */\

// Ensure if the instruction can be boostable
extern "C" {
    pub fn can_boost(insn: *mut insn, orig_addr: *mut c_void) -> bool;
}
// Recover instruction if given address is probed
//
// Copy an instruction and adjust the displacement if the instruction
// uses the %rip-relative addressing mode.
//
extern "C" {
    pub fn __copy_instruction(dest: *mut u8, src: *mut u8, real: *mut u8, insn: *mut insn) -> c_int;
}
// Generate a relative-jump/call instruction
extern "C" {
    pub fn synthesize_reljump(dest: *mut c_void, from: *mut c_void, to: *mut c_void);
}
extern "C" {
    pub fn synthesize_relcall(dest: *mut c_void, from: *mut c_void, to: *mut c_void);
}

extern "C" {
    pub fn setup_detour_execution(p: *mut kprobe, regs: *mut pt_regs, reenter: c_int) -> c_int;
}
extern "C" {
    pub fn __recover_optprobed_insn(buf: *mut kprobe_opcode_t, addr: c_ulong) -> c_ulong;
}

