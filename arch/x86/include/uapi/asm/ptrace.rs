//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/ptrace.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// this struct defines the way the registers are stored on the
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_regs {
    pub ebx: c_long,
    pub ecx: c_long,
    pub edx: c_long,
    pub esi: c_long,
    pub edi: c_long,
    pub ebp: c_long,
    pub eax: c_long,
    pub xds: c_int,
    pub xes: c_int,
    pub xfs: c_int,
    pub xgs: c_int,
    pub orig_eax: c_long,
    pub eip: c_long,
    pub xcs: c_int,
    pub eflags: c_long,
    pub esp: c_long,
    pub xss: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_regs {
//
// C ABI says these regs are callee-preserved. They aren't saved on kernel entry
// unless syscall needs a complete, fully filled "struct pt_regs".
//
    pub r15: c_ulong,
    pub r14: c_ulong,
    pub r13: c_ulong,
    pub r12: c_ulong,
    pub rbp: c_ulong,
    pub rbx: c_ulong,
// These regs are callee-clobbered. Always saved on kernel entry.
    pub r11: c_ulong,
    pub r10: c_ulong,
    pub r9: c_ulong,
    pub r8: c_ulong,
    pub rax: c_ulong,
    pub rcx: c_ulong,
    pub rdx: c_ulong,
    pub rsi: c_ulong,
    pub rdi: c_ulong,
//
// On syscall entry, this is syscall#. On CPU exception, this is error code.
// On hw interrupt, it's IRQ number:
//
    pub orig_rax: c_ulong,
// Return frame for iretq
    pub rip: c_ulong,
    pub cs: c_ulong,
    pub eflags: c_ulong,
    pub rsp: c_ulong,
    pub ss: c_ulong,
// top of stack page
}

