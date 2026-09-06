//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vboxvideo/vboxvideo_vbe.h
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


// SPDX-License-Identifier: MIT
// Copyright (C) 2006-2016 Oracle Corporation
// GUEST <-> HOST Communication API
pub const VBE_DISPI_BANK_ADDRESS: c_uint = 0xA0000;
pub const VBE_DISPI_BANK_SIZE_KB: c_int = 64;
pub const VBE_DISPI_MAX_XRES: c_int = 16384;
pub const VBE_DISPI_MAX_YRES: c_int = 16384;
pub const VBE_DISPI_MAX_BPP: c_int = 32;
pub const VBE_DISPI_IOPORT_INDEX: c_uint = 0x01CE;
pub const VBE_DISPI_IOPORT_DATA: c_uint = 0x01CF;
pub const VBE_DISPI_IOPORT_DAC_WRITE_INDEX: c_uint = 0x03C8;
pub const VBE_DISPI_IOPORT_DAC_DATA: c_uint = 0x03C9;
pub const VBE_DISPI_INDEX_ID: c_uint = 0x0;
pub const VBE_DISPI_INDEX_XRES: c_uint = 0x1;
pub const VBE_DISPI_INDEX_YRES: c_uint = 0x2;
pub const VBE_DISPI_INDEX_BPP: c_uint = 0x3;
pub const VBE_DISPI_INDEX_ENABLE: c_uint = 0x4;
pub const VBE_DISPI_INDEX_BANK: c_uint = 0x5;
pub const VBE_DISPI_INDEX_VIRT_WIDTH: c_uint = 0x6;
pub const VBE_DISPI_INDEX_VIRT_HEIGHT: c_uint = 0x7;
pub const VBE_DISPI_INDEX_X_OFFSET: c_uint = 0x8;
pub const VBE_DISPI_INDEX_Y_OFFSET: c_uint = 0x9;
pub const VBE_DISPI_INDEX_VBOX_VIDEO: c_uint = 0xa;
pub const VBE_DISPI_INDEX_FB_BASE_HI: c_uint = 0xb;
pub const VBE_DISPI_ID0: c_uint = 0xB0C0;
pub const VBE_DISPI_ID1: c_uint = 0xB0C1;
pub const VBE_DISPI_ID2: c_uint = 0xB0C2;
pub const VBE_DISPI_ID3: c_uint = 0xB0C3;
pub const VBE_DISPI_ID4: c_uint = 0xB0C4;
pub const VBE_DISPI_ID_VBOX_VIDEO: c_uint = 0xBE00;
// The VBOX interface id. Indicates support for VBVA shared memory interface.
pub const VBE_DISPI_ID_HGSMI: c_uint = 0xBE01;
pub const VBE_DISPI_ID_ANYX: c_uint = 0xBE02;
pub const VBE_DISPI_DISABLED: c_uint = 0x00;
pub const VBE_DISPI_ENABLED: c_uint = 0x01;
pub const VBE_DISPI_GETCAPS: c_uint = 0x02;
pub const VBE_DISPI_8BIT_DAC: c_uint = 0x20;
pub const VGA_PORT_HGSMI_HOST: c_uint = 0x3b0;
pub const VGA_PORT_HGSMI_GUEST: c_uint = 0x3d0;
