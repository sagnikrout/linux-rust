//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/cell-regs.h
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
// cbe_regs.h
//
// This file is intended to hold the various register definitions for CBE
// on-chip system devices (memory controller, IO controller, etc...)
//
// (C) Copyright IBM Corporation 2001,2006
//
// Authors: Maximino Aguilar (maguilar@us.ibm.com)
// David J. Erb (djerb@us.ibm.com)
//
// (c) 2006 Benjamin Herrenschmidt <benh@kernel.crashing.org>, IBM Corp.
//

// Cell page table entries
pub const CBE_IOPTE_PP_W: c_uint = 0x8000000000000000ul /* protection: write */;
pub const CBE_IOPTE_PP_R: c_uint = 0x4000000000000000ul /* protection: read */;
pub const CBE_IOPTE_M: c_uint = 0x2000000000000000ul /* coherency required */;
pub const CBE_IOPTE_SO_R: c_uint = 0x1000000000000000ul /* ordering: writes */;
pub const CBE_IOPTE_SO_RW: c_uint = 0x1800000000000000ul /* ordering: r & w */;
pub const CBE_IOPTE_RPN_Mask: c_uint = 0x07fffffffffff000ul /* RPN */;
pub const CBE_IOPTE_H: c_uint = 0x0000000000000800ul /* cache hint */;
pub const CBE_IOPTE_IOID_Mask: c_uint = 0x00000000000007fful /* ioid */;
