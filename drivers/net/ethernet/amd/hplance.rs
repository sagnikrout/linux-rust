//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amd/hplance.h
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
// Random defines and structures for the HP Lance driver.
// Copyright (C) 05/1998 Peter Maydell <pmaydell@chiark.greenend.org.uk>
// Based on the Sun Lance driver and the NetBSD HP Lance driver
//
// Registers
pub const HPLANCE_ID: c_uint = 0x01		/* DIO register: ID byte */;
pub const HPLANCE_STATUS: c_uint = 0x03		/* DIO register: interrupt enable/status */;
// Control and status bits for the status register
pub const LE_IE: c_uint = 0x80                                /* interrupt enable */;
pub const LE_IR: c_uint = 0x40                                /* interrupt requested */;
pub const LE_LOCK: c_uint = 0x08                              /* lock status register */;
pub const LE_ACK: c_uint = 0x04                               /* ack of lock */;
pub const LE_JAB: c_uint = 0x02                               /* loss of tx clock (???) */;
// We can also extract the IPL from the status register with the standard
// DIO_IPL(hplance) macro, or using dio_scodetoipl()
//
// These are the offsets for the DIO regs (hplance_reg), lance_ioreg,
// memory and NVRAM:
//

pub const HPLANCE_REGOFF: c_uint = 0x4000                     /* lance registers */;
pub const HPLANCE_MEMOFF: c_uint = 0x8000                     /* struct lance_init_block */;
pub const HPLANCE_NVRAMOFF: c_uint = 0xC008                   /* etheraddress as one *nibble* per byte */;
