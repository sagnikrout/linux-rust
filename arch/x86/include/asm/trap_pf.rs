//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/trap_pf.h
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
// Page fault error code bits:
//
// bit 0 ==	 0: no page found	1: protection fault
// bit 1 ==	 0: read access		1: write access
// bit 2 ==	 0: kernel-mode access	1: user-mode access
// bit 3 ==				1: use of reserved bit detected
// bit 4 ==				1: fault was an instruction fetch
// bit 5 ==				1: protection keys block access
// bit 6 ==				1: shadow stack access fault
// bit 15 ==				1: SGX MMU page-fault
// bit 31 ==				1: fault was due to RMP violation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum x86_pf_error_code {
    X86_PF_PROT	=		BIT(0),
    X86_PF_WRITE	=		BIT(1),
    X86_PF_USER	=		BIT(2),
    X86_PF_RSVD	=		BIT(3),
    X86_PF_INSTR	=		BIT(4),
    X86_PF_PK	=		BIT(5),
    X86_PF_SHSTK	=		BIT(6),
    X86_PF_SGX	=		BIT(15),
    X86_PF_RMP	=		BIT(31),
}
