//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/edd.h
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
// linux/include/linux/edd.h
// Copyright (C) 2002, 2003, 2004 Dell Inc.
// by Matt Domsch <Matt_Domsch@dell.com>
//
// structures and definitions for the int 13h, ax={41,48}h
// BIOS Enhanced Disk Drive Services
// This is based on the T13 group document D1572 Revision 0 (August 14 2002)
// available at http://www.t13.org/docs2002/d1572r0.pdf.  It is
// very similar to D1484 Revision 3 http://www.t13.org/docs2002/d1484r3.pdf
//
// In a nutshell, arch/{i386,x86_64}/boot/setup.S populates a scratch
// table in the boot_params that contains a list of BIOS-enumerated
// boot devices.
// In arch/{i386,x86_64}/kernel/setup.c, this information is
// transferred into the edd structure, and in drivers/firmware/edd.c, that
// information is used to identify BIOS boot disk.  The code in setup.S
// is very sensitive to the size of these structures.
//

