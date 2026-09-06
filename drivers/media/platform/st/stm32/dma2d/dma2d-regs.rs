//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/st/stm32/dma2d/dma2d-regs.h
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
// ST stm32 Chrom-Art - 2D Graphics Accelerator Driver
//
// Copyright (c) 2021 Dillon Min
// Dillon Min, <dillon.minfei@gmail.com>
//
// based on s5p-g2d
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
// Kamil Debski, <k.debski@samsung.com>
//
pub const DMA2D_CR_REG: c_uint = 0x0000;

pub const CR_MODE_SHIFT: c_int = 16;
pub const CR_M2M: c_uint = 0x0000;

pub const DMA2D_ISR_REG: c_uint = 0x0004;

pub const DMA2D_IFCR_REG: c_uint = 0x0008;

pub const DMA2D_FGMAR_REG: c_uint = 0x000c;
pub const DMA2D_FGOR_REG: c_uint = 0x0010;

pub const DMA2D_BGMAR_REG: c_uint = 0x0014;
pub const DMA2D_BGOR_REG: c_uint = 0x0018;

pub const DMA2D_FGPFCCR_REG: c_uint = 0x001c;

pub const DMA2D_FGCOLR_REG: c_uint = 0x0020;

pub const DMA2D_BGPFCCR_REG: c_uint = 0x0024;

pub const DMA2D_BGCOLR_REG: c_uint = 0x0028;

pub const DMA2D_OPFCCR_REG: c_uint = 0x0034;

pub const DMA2D_OCOLR_REG: c_uint = 0x0038;

pub const DMA2D_OMAR_REG: c_uint = 0x003c;
pub const DMA2D_OOR_REG: c_uint = 0x0040;

pub const DMA2D_NLR_REG: c_uint = 0x0044;

// Hardware limits
pub const MAX_WIDTH: c_int = 2592;
pub const MAX_HEIGHT: c_int = 2592;
pub const DEFAULT_WIDTH: c_int = 240;
pub const DEFAULT_HEIGHT: c_int = 320;
pub const DEFAULT_SIZE: c_int = 307200;
pub const CM_MODE_ARGB8888: c_uint = 0x00;
pub const CM_MODE_ARGB4444: c_uint = 0x04;
pub const CM_MODE_A4: c_uint = 0x0a;
