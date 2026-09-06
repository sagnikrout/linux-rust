//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx18/cx18-irq.h
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
// cx18 interrupt handling
//
// Copyright (C) 2007  Hans Verkuil <hverkuil@kernel.org>
// Copyright (C) 2008  Andy Walls <awalls@md.metrocast.net>
//

pub const HW2_INT_CLR_STATUS: c_uint = 0xc730c4;
pub const HW2_INT_MASK5_PCI: c_uint = 0xc730e4;
pub const SW1_INT_SET: c_uint = 0xc73100;
pub const SW1_INT_STATUS: c_uint = 0xc73104;
pub const SW1_INT_ENABLE_PCI: c_uint = 0xc7311c;
pub const SW2_INT_SET: c_uint = 0xc73140;
pub const SW2_INT_STATUS: c_uint = 0xc73144;
pub const SW2_INT_ENABLE_CPU: c_uint = 0xc73158;
pub const SW2_INT_ENABLE_PCI: c_uint = 0xc7315c;
extern "C" {
    pub fn cx18_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
