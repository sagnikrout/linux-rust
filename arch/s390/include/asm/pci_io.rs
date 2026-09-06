//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/pci_io.h
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

// I/O size constraints
pub const ZPCI_MAX_READ_SIZE: c_int = 8;
pub const ZPCI_MAX_WRITE_SIZE: c_int = 128;

// I/O Map
pub const ZPCI_IOMAP_SHIFT: c_int = 48;
pub const ZPCI_IOMAP_ADDR_SHIFT: c_int = 62;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_iomap_entry {
    pub fh: u32,
    pub bar: u8,
    pub count: u16,
}

extern "C" {
    pub fn zpci_store(_arg: dst, _arg: val, _arg: len) -> return;
}
// ((u8 *) dst) = (u8) data;
// ((u16 *) dst) = (u16) data;
// ((u32 *) dst) = (u32) data;
// ((u64 *) dst) = (u64) data;
extern "C" {
    pub fn rounddown_pow_of_two(_arg: size) -> return;
}

