//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/hws/hws_reg.h
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


// SPDX-License-Identifier: GPL-2.0-only

pub const PCI_BUS_ACCESS_BASE: c_uint = 0x00000000U;

pub const HWS_INT_EN_MASK: c_uint = 0x0003FFFFU;
pub const PCIEBAR_AXI_BASE: c_uint = 0x20000000U;
pub const CTL_REG_ACC_BASE: c_uint = 0x0;

pub const CVBS_IN_BASE: c_uint = 0x00004000U;

// 2 Mib
pub const MAX_L_VIDEO_SIZE: c_uint = 0x200000U;
pub const PCI_E_BAR_PAGE_SIZE: c_uint = 0x20000000;
pub const PCI_E_BAR_ADD_MASK: c_uint = 0xE0000000;
pub const PCI_E_BAR_ADD_LOWMASK: c_uint = 0x1FFFFFFF;
pub const MAX_VID_CHANNELS: c_int = 4;

pub const MAX_VIDEO_HW_W: c_int = 1920;
pub const MAX_VIDEO_HW_H: c_int = 1080;

pub const MIN_VAMP_BRIGHTNESS_UNITS: c_int = 0;
pub const MAX_VAMP_BRIGHTNESS_UNITS: c_uint = 0xff;
pub const MIN_VAMP_CONTRAST_UNITS: c_int = 0;
pub const MAX_VAMP_CONTRAST_UNITS: c_uint = 0xff;
pub const MIN_VAMP_SATURATION_UNITS: c_int = 0;
pub const MAX_VAMP_SATURATION_UNITS: c_uint = 0xff;
pub const MIN_VAMP_HUE_UNITS: c_int = 0;
pub const MAX_VAMP_HUE_UNITS: c_uint = 0xff;
pub const HWS_BRIGHTNESS_DEFAULT: c_uint = 0x80;
pub const HWS_CONTRAST_DEFAULT: c_uint = 0x80;
pub const HWS_SATURATION_DEFAULT: c_uint = 0x80;
pub const HWS_HUE_DEFAULT: c_uint = 0x00;
// Core/global status.

// bit3: DMA busy, bit2: int, ...

// Main control register

// Write 0x00 to fully reset decoder,
// set bit 31=1 to "start run",
// low byte=0x13 selects YUYV/BT.709/etc,
// in ReadChipId() we also write 0x00 and 0x10 here for chip-ID sequencing.
//
// Per-channel done flags.

// Capture enable switches.
// bit0-3: CH0-CH3 video enable

// bits0-3: signal present, bits8-11: interlace

// bits0-3: HDCP detected

// Buffer addresses (written once during init/reset).
// Base of host-visible buffer.

// Per-channel DMA address.

// Per-channel live buffer toggles (read-only).

//
// Returns 0 or 1 = which half of the video ring the DMA engine is
// currently filling for channel *ch* (0-3).
//
// Per-interrupt bits (video 0-3).

// 16-bit W | 16-bit H.

// B|C|H|S packed bytes.

// Input fps.

// Programmed out W|H.

// Programmed out fps.

// Device version/port ID/subversion register.

//
// Reading this 32-bit word returns:
// bits 7:0   = "device version"
// bits 15:8  = "device sub-version"
// bits 23:24 = "HW key / port ID" etc.
// bits 31:28 = "support YV12" flags
//
// Convenience aliases for individual channels.

