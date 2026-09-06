//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nvram.h
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
// struct nvram_ops - NVRAM functionality made available to drivers
// @read: validate checksum (if any) then load a range of bytes from NVRAM
// @write: store a range of bytes to NVRAM then update checksum (if any)
// @read_byte: load a single byte from NVRAM
// @write_byte: store a single byte to NVRAM
// @get_size: return the fixed number of bytes in the NVRAM
//
// Architectures which provide an nvram ops struct need not implement all
// of these methods. If the NVRAM hardware can be accessed only one byte
// at a time then it may be sufficient to provide .read_byte and .write_byte.
// If the NVRAM has a checksum (and it is to be checked) the .read and
// .write methods can be used to implement that efficiently.
//
// Portable drivers may use the wrapper functions defined here.
// The nvram_read() and nvram_write() functions call the .read and .write
// methods when available and fall back on the .read_byte and .write_byte
// methods otherwise.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvram_ops {
    pub (*get_size)(void): *mut isize,
    pub (*read_byte)(int): *mut c_uchar,
    pub int): *mut *mut void (write_byte)(unsigned char,,
    pub ): *mut *mut *mut ssize_t (read)(char , size_t, loff_t,
    pub ): *mut *mut *mut ssize_t (write)(char , size_t, loff_t,

    pub (*initialize)(void): *mut c_long,
    pub (*set_checksum)(void): *mut c_long,

}

// p = nvram_read_byte(i);
// ppos = i;

extern "C" {
    pub fn nvram_read_bytes(_arg: buf, _arg: count, _arg: ppos) -> return;
}

extern "C" {
    pub fn nvram_write_bytes(_arg: buf, _arg: count, _arg: ppos) -> return;
}
