//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/pmag-ba-fb.h
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


//
// linux/include/video/pmag-ba-fb.h
//
// TURBOchannel PMAG-BA Color Frame Buffer (CFB) card support,
// Copyright (C) 1999, 2000, 2001 by
// Michael Engel <engel@unix-ag.org>,
// Karsten Merker <merker@linuxtag.org>
// Copyright (c) 2005  Maciej W. Rozycki
//
// This file is subject to the terms and conditions of the GNU General
// Public License.  See the file COPYING in the main directory of this
// archive for more details.
//
// IOmem resource offsets.
pub const PMAG_BA_FBMEM: c_uint = 0x000000	/* frame buffer */;
pub const PMAG_BA_BT459: c_uint = 0x200000	/* Bt459 RAMDAC */;
pub const PMAG_BA_IRQ: c_uint = 0x300000	/* IRQ acknowledge */;
pub const PMAG_BA_ROM: c_uint = 0x380000	/* REX option ROM */;
pub const PMAG_BA_BT438: c_uint = 0x380000	/* Bt438 clock chip reset */;
pub const PMAG_BA_SIZE: c_uint = 0x400000	/* address space size */;
// Bt459 register offsets, byte-wide registers.
pub const BT459_ADDR_LO: c_uint = 0x0		/* address low */;
pub const BT459_ADDR_HI: c_uint = 0x4		/* address high */;
pub const BT459_DATA: c_uint = 0x8		/* data window register */;
pub const BT459_CMAP: c_uint = 0xc		/* color map window register */;
