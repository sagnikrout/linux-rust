//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/cfi.h
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
// Clang Control Flow Integrity (CFI) support.
//
// Copyright (C) 2022 Google LLC
//

//
// An overview of the various calling conventions...
//
// Traditional:
//
// foo:
// ... code here ...
// ret
//
// direct caller:
// call foo
//
// indirect caller:
// lea foo(%rip), %r11
// ...
// call *%r11
//
// IBT:
//
// foo:
// endbr64
// ... code here ...
// ret
//
// direct caller:
// call foo / call foo+4
//
// indirect caller:
// lea foo(%rip), %r11
// ...
// call *%r11
//
// kCFI:
//
// __cfi_foo:
// movl $0x12345678, %eax
// # 11 nops when CONFIG_CALL_PADDING
// foo:
// endbr64			# when IBT
// ... code here ...
// ret
//
// direct call:
// call foo			# / call foo+4 when IBT
//
// indirect call:
// lea foo(%rip), %r11
// ...
// movl $(-0x12345678), %r10d
// addl -4(%r11), %r10d	# -15 when CONFIG_CALL_PADDING
// jz   1f
// ud2
// 1:call *%r11
//
// FineIBT (builds as kCFI + CALL_PADDING + IBT + RETPOLINE and runtime patches into):
//
// __cfi_foo:
// endbr64
// subl 0x12345678, %eax
// jne.32,pn foo+3
// foo:
// nopl -42(%rax)		# was endbr64
// ... code here ...
// ret
//
// direct caller:
// call foo / call foo+4
//
// indirect caller:
// lea foo(%rip), %r11
// ...
// movl $0x12345678, %eax
// lea  -0x10(%r11), %r11
// nop5
// call *%r11
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cfi_mode {
    CFI_AUTO,	/* FineIBT if hardware has IBT, otherwise kCFI */
    CFI_OFF,	/* Taditional / IBT depending on .config */
    CFI_KCFI,	/* Optionally CALL_PADDING, IBT, RETPOLINE */
    CFI_FINEIBT,	/* see arch/x86/kernel/alternative.c */
}

pub const CFI_OFFSET: c_int = 5;

extern "C" {
    pub fn handle_cfi_failure(regs: *mut pt_regs) -> bug_trap_type;
}
// Macro flag: #define __bpfcall

extern "C" {
    pub fn cfi_get_func_hash(func: *mut c_void) -> u32;
}

extern "C" {
    pub fn cfi_get_func_arity(func: *mut c_void) -> c_int;
}

extern "C" {
    pub fn decode_fineibt_insn(regs: *mut pt_regs, target: *mut c_ulong, type: *mut u32) -> bool;
}

