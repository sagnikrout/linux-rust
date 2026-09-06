//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/image.h
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

pub const ARM64_IMAGE_FLAG_BE_SHIFT: c_int = 0;

pub const ARM64_IMAGE_FLAG_BE_MASK: c_uint = 0x1;
pub const ARM64_IMAGE_FLAG_PAGE_SIZE_MASK: c_uint = 0x3;
pub const ARM64_IMAGE_FLAG_PHYS_BASE_MASK: c_uint = 0x1;
pub const ARM64_IMAGE_FLAG_LE: c_int = 0;
pub const ARM64_IMAGE_FLAG_BE: c_int = 1;
pub const ARM64_IMAGE_FLAG_PAGE_SIZE_4K: c_int = 1;
pub const ARM64_IMAGE_FLAG_PAGE_SIZE_16K: c_int = 2;
pub const ARM64_IMAGE_FLAG_PAGE_SIZE_64K: c_int = 3;
pub const ARM64_IMAGE_FLAG_PHYS_BASE: c_int = 1;

//
// struct arm64_image_header - arm64 kernel image header
// See Documentation/arch/arm64/booting.rst for details
//
// @code0:		Executable code, or
// @mz_header		  alternatively used for part of MZ header
// @code1:		Executable code
// @text_offset:	Image load offset
// @image_size:		Effective Image size
// @flags:		kernel flags
// @reserved:		reserved
// @magic:		Magic number
// @reserved5:		reserved, or
// @pe_header:	  alternatively used for PE COFF offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm64_image_header {
    pub code0: __le32,
    pub code1: __le32,
    pub text_offset: __le64,
    pub image_size: __le64,
    pub flags: __le64,
    pub res2: __le64,
    pub res3: __le64,
    pub res4: __le64,
    pub magic: __le32,
    pub res5: __le32,
}

