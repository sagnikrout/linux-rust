//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/image.h
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

pub const RISCV_IMAGE_FLAG_BE_SHIFT: c_int = 0;
pub const RISCV_IMAGE_FLAG_BE_MASK: c_uint = 0x1;
pub const RISCV_IMAGE_FLAG_LE: c_int = 0;
pub const RISCV_IMAGE_FLAG_BE: c_int = 1;

pub const RISCV_HEADER_VERSION_MAJOR: c_int = 0;
pub const RISCV_HEADER_VERSION_MINOR: c_int = 2;

//
// struct riscv_image_header - riscv kernel image header
// @code0:		Executable code
// @code1:		Executable code
// @text_offset:	Image load offset (little endian)
// @image_size:		Effective Image size (little endian)
// @flags:		kernel flags (little endian)
// @version:		version
// @res1:		reserved
// @res2:		reserved
// @magic:		Magic number (RISC-V specific; deprecated)
// @magic2:		Magic number 2 (to match the ARM64 'magic' field pos)
// @res3:		reserved (will be used for PE COFF offset)
//
// The intention is for this header format to be shared between multiple
// architectures to avoid a proliferation of image header formats.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_image_header {
    pub code0: u32,
    pub code1: u32,
    pub text_offset: u64,
    pub image_size: u64,
    pub flags: u64,
    pub version: u32,
    pub res1: u32,
    pub res2: u64,
    pub magic: u64,
    pub magic2: u32,
    pub res3: u32,
}

