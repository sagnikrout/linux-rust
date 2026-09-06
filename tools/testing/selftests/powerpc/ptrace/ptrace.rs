//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/ptrace/ptrace.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Ptrace interface test helper functions
//
// Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
//

pub const TEST_PASS: c_int = 0;
pub const TEST_FAIL: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpr_regs {
    pub fpr: [__u64; 32],
    pub fpscr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tm_spr_regs {
    pub tm_tfhar: c_ulong,
    pub tm_texasr: c_ulong,
    pub tm_tfiar: c_ulong,
}

pub const NT_PPC_TAR: c_uint = 0x103;
pub const NT_PPC_PPR: c_uint = 0x104;
pub const NT_PPC_DSCR: c_uint = 0x105;
pub const NT_PPC_EBB: c_uint = 0x106;
pub const NT_PPC_PMU: c_uint = 0x107;
pub const NT_PPC_TM_CGPR: c_uint = 0x108;
pub const NT_PPC_TM_CFPR: c_uint = 0x109;
pub const NT_PPC_TM_CVMX: c_uint = 0x10a;
pub const NT_PPC_TM_CVSX: c_uint = 0x10b;
pub const NT_PPC_TM_SPR: c_uint = 0x10c;
pub const NT_PPC_TM_CTAR: c_uint = 0x10d;
pub const NT_PPC_TM_CPPR: c_uint = 0x10e;
pub const NT_PPC_TM_CDSCR: c_uint = 0x10f;

// Basic ptrace operations
// TAR, PPR, DSCR
// reg = tar;
// reg = ppr;
// reg = dscr;
// reg = tar;
// reg = ppr;
// reg = dscr;
// FPR
// GPR
extern "C" {
    pub fn syscall(_arg: __NR_ptrace, _arg: request, _arg: pid, )addr: *mut (void, _arg: data) -> return;
}
// 33 because of FPSCR

// VMX
// VSX
// TM SPR
// Analyse TEXASR after TM failure
extern "C" {
    pub fn mfspr(_arg: SPRN_TFIAR) -> return;
}
extern "C" {
    pub fn store_gpr(addr: *mut c_ulong);
}
