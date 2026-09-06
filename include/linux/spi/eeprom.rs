//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/eeprom.h
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
// Put one of these structures in platform_data for SPI EEPROMS handled
// by the "at25" driver.  On SPI, most EEPROMS understand the same core
// command set.  If you need to support EEPROMs that don't yet fit, add
// flags to support those protocol options.  These values all come from
// the chip datasheets.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_eeprom {
    pub byte_len: u32,
    pub name: [c_char; 10],
    pub /: *mut *mut u32 page_size; / for writes,
    pub flags: u16,
pub const EE_ADDR1: c_uint = 0x0001			/*  8 bit addrs */;
pub const EE_ADDR2: c_uint = 0x0002			/* 16 bit addrs */;
pub const EE_ADDR3: c_uint = 0x0004			/* 24 bit addrs */;
pub const EE_READONLY: c_uint = 0x0008			/* disallow writes */;
//
// Certain EEPROMS have a size that is larger than the number of address
// bytes would allow (e.g. like M95040 from ST that has 512 Byte size
// but uses only one address byte (A0 to A7) for addressing.) For
// the extra address bit (A8, A16 or A24) bit 3 of the instruction byte
// is used. This instruction bit is normally defined as don't care for
// other AT25 like chips.
//
pub const EE_INSTR_BIT3_IS_ADDR: c_uint = 0x0010;
    pub context: *mut c_void,
}
