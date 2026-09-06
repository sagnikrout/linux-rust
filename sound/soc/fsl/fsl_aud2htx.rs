//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/fsl_aud2htx.h
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
// Copyright 2020 NXP
//

// AUD2HTX Register Map
pub const AUD2HTX_CTRL: c_uint = 0x0   /* AUD2HTX Control Register */;
pub const AUD2HTX_CTRL_EXT: c_uint = 0x4   /* AUD2HTX Control Extended Register */;
pub const AUD2HTX_WR: c_uint = 0x8   /* AUD2HTX Write Register */;
pub const AUD2HTX_STATUS: c_uint = 0xC   /* AUD2HTX Status Register */;
pub const AUD2HTX_IRQ_NOMASK: c_uint = 0x10  /* AUD2HTX Nonmasked Interrupt Flags Register */;
pub const AUD2HTX_IRQ_MASKED: c_uint = 0x14  /* AUD2HTX Masked Interrupt Flags Register */;
pub const AUD2HTX_IRQ_MASK: c_uint = 0x18  /* AUD2HTX IRQ Masks Register */;
// AUD2HTX Control Register

// AUD2HTX Control Extended Register

pub const AUD2HTX_CTRE_DT_SHIFT: c_uint = 0x1;
pub const AUD2HTX_CTRE_DT_WIDTH: c_uint = 0x2;

pub const AUD2HTX_CTRE_WL_SHIFT: c_int = 16;
pub const AUD2HTX_CTRE_WL_WIDTH: c_int = 5;

pub const AUD2HTX_CTRE_WH_SHIFT: c_int = 24;
pub const AUD2HTX_CTRE_WH_WIDTH: c_int = 5;

// AUD2HTX IRQ Masks Register

pub const AUD2HTX_FIFO_DEPTH: c_uint = 0x20;
pub const AUD2HTX_WTMK_LOW: c_uint = 0x10;
pub const AUD2HTX_WTMK_HIGH: c_uint = 0x10;
pub const AUD2HTX_MAXBURST: c_uint = 0x10;
//
// fsl_aud2htx: AUD2HTX private data
//
// @pdev: platform device pointer
// @regmap: regmap handler
// @bus_clk: clock source to access register
// @dma_params_rx: DMA parameters for receive channel
// @dma_params_tx: DMA parameters for transmit channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_aud2htx {
    pub pdev: *mut platform_device,
    pub regmap: *mut regmap,
    pub bus_clk: *mut clk,
    pub dma_params_rx: snd_dmaengine_dai_dma_data,
    pub dma_params_tx: snd_dmaengine_dai_dma_data,
}
