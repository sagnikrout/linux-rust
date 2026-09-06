//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/tw686x/tw686x-regs.h
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
// DMA controller registers

pub const INT_STATUS: c_uint = 0x00;
pub const PB_STATUS: c_uint = 0x01;
pub const DMA_CMD: c_uint = 0x02;
pub const VIDEO_FIFO_STATUS: c_uint = 0x03;
pub const VIDEO_CHANNEL_ID: c_uint = 0x04;
pub const VIDEO_PARSER_STATUS: c_uint = 0x05;
pub const SYS_SOFT_RST: c_uint = 0x06;

pub const DMA_CHANNEL_ENABLE: c_uint = 0x0a;
pub const DMA_CONFIG: c_uint = 0x0b;
pub const DMA_TIMER_INTERVAL: c_uint = 0x0c;
pub const DMA_CHANNEL_TIMEOUT: c_uint = 0x0d;

pub const DMA10_P_ADDR: c_uint = 0x28;
pub const DMA10_B_ADDR: c_uint = 0x29;
pub const VIDEO_CONTROL1: c_uint = 0x2a;
pub const VIDEO_CONTROL2: c_uint = 0x2b;
pub const AUDIO_CONTROL1: c_uint = 0x2c;
pub const AUDIO_CONTROL2: c_uint = 0x2d;
pub const PHASE_REF: c_uint = 0x2e;
pub const GPIO_REG: c_uint = 0x2f;

pub const AUDIO_CONTROL3: c_uint = 0x38;

pub const EP_REG_ADDR: c_uint = 0xfe;
pub const EP_REG_DATA: c_uint = 0xff;
// Video decoder registers

pub const SYS_MODE_DMA_SHIFT: c_int = 13;
pub const AUDIO_DMA_SIZE_SHIFT: c_int = 19;

pub const TW686X_STD_NTSC_M: c_int = 0;
pub const TW686X_STD_PAL: c_int = 1;
pub const TW686X_STD_SECAM: c_int = 2;
pub const TW686X_STD_NTSC_443: c_int = 3;
pub const TW686X_STD_PAL_M: c_int = 4;
pub const TW686X_STD_PAL_CN: c_int = 5;
pub const TW686X_STD_PAL_60: c_int = 6;
pub const TW686X_FIELD_MODE: c_uint = 0x3;
pub const TW686X_FRAME_MODE: c_uint = 0x2;
// 0x1 is reserved
pub const TW686X_SG_MODE: c_uint = 0x0;
