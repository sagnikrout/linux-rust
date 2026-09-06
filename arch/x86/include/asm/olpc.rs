//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/olpc.h
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
// OLPC machine specific definitions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct olpc_platform_t {
    pub flags: c_int,
    pub boardrev: u32,
}

pub const OLPC_F_PRESENT: c_uint = 0x01;
pub const OLPC_F_DCON: c_uint = 0x02;

//
// OLPC board IDs contain the major build number within the mask 0x0ff0,
// and the minor build number within 0x000f.  Pre-builds have a minor
// number less than 8, and normal builds start at 8.  For example, 0x0B10
// is a PreB1, and 0x0C18 is a C1.
//
// The DCON is OLPC's Display Controller.  It has a number of unique
// features that we might want to take advantage of..
//
// The "Mass Production" version of OLPC's XO is identified as being model
// C2.  During the prototype phase, the following models (in chronological
// order) were created: A1, B1, B2, B3, B4, C1.  The A1 through B2 models
// were based on Geode GX CPUs, and models after that were based upon
// Geode LX CPUs.  There were also some hand-assembled models floating
// around, referred to as PreB1, PreB2, etc.
//

extern "C" {
    pub fn do_olpc_suspend_lowlevel();
}
extern "C" {
    pub fn olpc_xo1_pm_wakeup_set(value: u16);
}
extern "C" {
    pub fn olpc_xo1_pm_wakeup_clear(value: u16);
}

extern "C" {
    pub fn pci_olpc_init() -> c_int;
}
// GPIO assignments
pub const OLPC_GPIO_MIC_AC: c_int = 1;
pub const OLPC_GPIO_DCON_STAT0: c_int = 5;
pub const OLPC_GPIO_DCON_STAT1: c_int = 6;
pub const OLPC_GPIO_DCON_IRQ: c_int = 7;

pub const OLPC_GPIO_DCON_LOAD: c_int = 11;
pub const OLPC_GPIO_DCON_BLANK: c_int = 12;
pub const OLPC_GPIO_SMB_CLK: c_int = 14;
pub const OLPC_GPIO_SMB_DATA: c_int = 15;

pub const OLPC_GPIO_LID: c_int = 26;
pub const OLPC_GPIO_ECSCI: c_int = 27;
