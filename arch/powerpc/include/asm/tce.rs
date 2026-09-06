//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/tce.h
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
// Copyright (C) 2001 Mike Corrigan & Dave Engebretsen, IBM Corporation
// Rewrite, cleanup:
// Copyright (C) 2004 Olof Johansson <olof@lixom.net>, IBM Corporation
//

//
// Tces come in two formats, one for the virtual bus and a different
// format for PCI.  PCI TCEs can have hardware or software maintianed
// coherency.
//
pub const TCE_VB: c_int = 0;
pub const TCE_PCI: c_int = 1;

pub const TCE_VALID: c_uint = 0x800		/* TCE valid */;
pub const TCE_ALLIO: c_uint = 0x400		/* TCE valid for all lpars */;
pub const TCE_PCI_WRITE: c_uint = 0x2		/* write from PCI allowed */;
pub const TCE_PCI_READ: c_uint = 0x1		/* read from PCI allowed */;
pub const TCE_VB_WRITE: c_uint = 0x1		/* write from VB allowed */;

