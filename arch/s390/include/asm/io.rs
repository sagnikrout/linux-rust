//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/io.h
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
// S390 version
// Copyright IBM Corp. 1999
// Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
//
// Derived from "include/asm-i386/io.h"
//

extern "C" {
    pub fn unxlate_dev_mem_ptr(phys: phys_addr_t, addr: *mut c_void);
}
pub const IO_SPACE_LIMIT: c_int = 0;
//
// I/O memory mapping functions.
//

//
// s390 needs a private implementation of pci_iomap since ioremap with its
// offset parameter isn't sufficient. That's because BAR spaces are not
// disjunctive on s390 so we need the bar parameter of pci_iomap to find
// the corresponding device and create the mapping cookie.
//

// combine single writes by using store-block insn

