//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fdomain.h
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
pub const FDOMAIN_REGION_SIZE: c_uint = 0x10;
pub const FDOMAIN_BIOS_SIZE: c_uint = 0x2000;
// (@) = not present on TMC1800, (#) = not present on TMC1800 and TMC18C50

pub const ICTL_FIFO_MASK: c_uint = 0x0f	 /* FIFO threshold, 1/16 FIFO size */;

pub const MCTL_ACK_MASK: c_uint = 0x0f	 /* Acknowledge period */;

pub const ASTAT3_IRQMASK: c_uint = 0xf0	 /* Enabled interrupts mask */;

pub const CFG1_IRQ_MASK: c_uint = 0x0e	 /* IRQ jumpers */;
pub const CFG1_IO_MASK: c_uint = 0x30	 /* I/O base jumpers */;
pub const CFG1_BIOS_MASK: c_uint = 0xc0	 /* BIOS base jumpers */;

extern "C" {
    pub fn fdomain_destroy(sh: *mut Scsi_Host) -> c_int;
}
