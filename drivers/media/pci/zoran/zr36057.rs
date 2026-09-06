//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/zoran/zr36057.h
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
// zr36057.h - zr36057 register offsets
//
// Copyright (C) 1998 Dave Perks <dperks@ibm.net>
//
// Zoran ZR36057 registers
pub const ZR36057_VFEHCR: c_uint = 0x000	/* Video Front End, Horizontal Configuration Register */;

pub const ZR36057_VFEHCR_H_START: c_int = 10;
pub const ZR36057_VFEHCR_H_END: c_int = 0;
pub const ZR36057_VFEHCR_HMASK: c_uint = 0x3ff;
pub const ZR36057_VFEVCR: c_uint = 0x004	/* Video Front End, Vertical Configuration Register */;

pub const ZR36057_VFEVCR_V_START: c_int = 10;
pub const ZR36057_VFEVCR_V_END: c_int = 0;
pub const ZR36057_VFEVCR_VMASK: c_uint = 0x3ff;
pub const ZR36057_VFESPFR: c_uint = 0x008	/* Video Front End, Scaler and Pixel Format Register */;

pub const ZR36057_VFESPFR_H_FILTER: c_int = 21;
pub const ZR36057_VFESPFR_HOR_DCM: c_int = 14;
pub const ZR36057_VFESPFR_VER_DCM: c_int = 8;
pub const ZR36057_VFESPFR_DISP_MODE: c_int = 6;

pub const ZR36057_VDTR: c_uint = 0x00c	/* Video Display "Top" Register */;
pub const ZR36057_VDBR: c_uint = 0x010	/* Video Display "Bottom" Register */;
pub const ZR36057_VSSFGR: c_uint = 0x014	/* Video Stride, Status, and Frame Grab Register */;
pub const ZR36057_VSSFGR_DISP_STRIDE: c_int = 16;

pub const ZR36057_VDCR: c_uint = 0x018	/* Video Display Configuration Register */;

pub const ZR36057_VDCR_MIN_PIX: c_int = 24;

pub const ZR36057_VDCR_VID_WIN_HT: c_int = 12;
pub const ZR36057_VDCR_VID_WIN_WID: c_int = 0;
pub const ZR36057_MMTR: c_uint = 0x01c	/* Masking Map "Top" Register */;
pub const ZR36057_MMBR: c_uint = 0x020	/* Masking Map "Bottom" Register */;
pub const ZR36057_OCR: c_uint = 0x024	/* Overlay Control Register */;

pub const ZR36057_OCR_MASK_STRIDE: c_int = 0;
pub const ZR36057_SPGPPCR: c_uint = 0x028	/* System, PCI, and General Purpose Pins Control Register */;

pub const ZR36057_GPPGCR1: c_uint = 0x02c	/* General Purpose Pins and GuestBus Control Register (1) */;
pub const ZR36057_MCSAR: c_uint = 0x030	/* MPEG Code Source Address Register */;
pub const ZR36057_MCTCR: c_uint = 0x034	/* MPEG Code Transfer Control Register */;

pub const ZR36057_MCTCR_COD_GUEST_ID: c_int = 20;
pub const ZR36057_MCTCR_COD_GUEST_REG: c_int = 16;
pub const ZR36057_MCMPR: c_uint = 0x038	/* MPEG Code Memory Pointer Register */;
pub const ZR36057_ISR: c_uint = 0x03c	/* Interrupt Status Register */;

pub const ZR36057_ICR: c_uint = 0x040	/* Interrupt Control Register */;

pub const ZR36057_I2CBR: c_uint = 0x044	/* I2C Bus Register */;

pub const ZR36057_JMC: c_uint = 0x100	/* JPEG Mode and Control */;

pub const ZR36057_JPC: c_uint = 0x104	/* JPEG Process Control */;

pub const ZR36057_VSP: c_uint = 0x108	/* Vertical Sync Parameters */;
pub const ZR36057_VSP_VSYNC_SIZE: c_int = 16;
pub const ZR36057_VSP_FRM_TOT: c_int = 0;
pub const ZR36057_HSP: c_uint = 0x10c	/* Horizontal Sync Parameters */;
pub const ZR36057_HSP_HSYNC_START: c_int = 16;
pub const ZR36057_HSP_LINE_TOT: c_int = 0;
pub const ZR36057_FHAP: c_uint = 0x110	/* Field Horizontal Active Portion */;
pub const ZR36057_FHAP_NAX: c_int = 16;
pub const ZR36057_FHAP_PAX: c_int = 0;
pub const ZR36057_FVAP: c_uint = 0x114	/* Field Vertical Active Portion */;
pub const ZR36057_FVAP_NAY: c_int = 16;
pub const ZR36057_FVAP_PAY: c_int = 0;
pub const ZR36057_FPP: c_uint = 0x118	/* Field Process Parameters */;

pub const ZR36057_JCBA: c_uint = 0x11c	/* JPEG Code Base Address */;
pub const ZR36057_JCFT: c_uint = 0x120	/* JPEG Code FIFO Threshold */;
pub const ZR36057_JCGI: c_uint = 0x124	/* JPEG Codec Guest ID */;
pub const ZR36057_JCGI_JPE_GUEST_ID: c_int = 4;
pub const ZR36057_JCGI_JPE_GUEST_REG: c_int = 0;
pub const ZR36057_GCR2: c_uint = 0x12c	/* GuestBus Control Register (2) */;
pub const ZR36057_POR: c_uint = 0x200	/* Post Office Register */;

pub const ZR36057_STR: c_uint = 0x300	/* "Still" Transfer Register */;
