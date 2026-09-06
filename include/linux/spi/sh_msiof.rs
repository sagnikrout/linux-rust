//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/sh_msiof.h
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

pub const SITMDR1: c_uint = 0x00	/* Transmit Mode Register 1 */;
pub const SITMDR2: c_uint = 0x04	/* Transmit Mode Register 2 */;
pub const SITMDR3: c_uint = 0x08	/* Transmit Mode Register 3 */;
pub const SIRMDR1: c_uint = 0x10	/* Receive Mode Register 1 */;
pub const SIRMDR2: c_uint = 0x14	/* Receive Mode Register 2 */;
pub const SIRMDR3: c_uint = 0x18	/* Receive Mode Register 3 */;
pub const SITSCR: c_uint = 0x20	/* Transmit Clock Select Register */;
pub const SIRSCR: c_uint = 0x22	/* Receive Clock Select Register (SH, A1, APE6) */;
pub const SICTR: c_uint = 0x28	/* Control Register */;
pub const SIFCTR: c_uint = 0x30	/* FIFO Control Register */;
pub const SISTR: c_uint = 0x40	/* Status Register */;
pub const SIIER: c_uint = 0x44	/* Interrupt Enable Register */;
pub const SITDR1: c_uint = 0x48	/* Transmit Control Data Register 1 (SH, A1) */;
pub const SITDR2: c_uint = 0x4c	/* Transmit Control Data Register 2 (SH, A1) */;
pub const SITFDR: c_uint = 0x50	/* Transmit FIFO Data Register */;
pub const SIRDR1: c_uint = 0x58	/* Receive Control Data Register 1 (SH, A1) */;
pub const SIRDR2: c_uint = 0x5c	/* Receive Control Data Register 2 (SH, A1) */;
pub const SIRFDR: c_uint = 0x60	/* Receive FIFO Data Register */;
// SITMDR1 and SIRMDR1

// SITMDR1

// 0=MSIOF_SYNC, 1=MSIOF_SS1, 2=MSIOF_SS2
// SITMDR2 and SIRMDR2

// SITMDR3 and SIRMDR3

// SITSCR and SIRSCR

// SICTR

// SIFCTR

// SISTR

// SIIER

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_msiof_spi_info {
    pub tx_fifo_override: c_int,
    pub rx_fifo_override: c_int,
    pub num_chipselect: u16,
    pub mode: c_int,
    pub dma_tx_id: c_uint,
    pub dma_rx_id: c_uint,
    pub dtdl: u32,
    pub syncdl: u32,
}
