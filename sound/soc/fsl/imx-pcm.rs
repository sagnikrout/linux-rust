//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/imx-pcm.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2009 Sascha Hauer <s.hauer@pengutronix.de>
//
// This code is based on code copyrighted by Freescale,
// Liam Girdwood, Javier Martin and probably others.
//

//
// Do not change this as the FIQ handler depends on this size
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_pcm_fiq_params {
    pub irq: c_int,
    pub base: *mut void __iomem,
// Pointer to original ssi driver to setup tx rx sizes
    pub dma_params_rx: *mut snd_dmaengine_dai_dma_data,
    pub dma_params_tx: *mut snd_dmaengine_dai_dma_data,
}

extern "C" {
    pub fn imx_pcm_dma_init(pdev: *mut platform_device) -> c_int;
}

extern "C" {
    pub fn imx_pcm_fiq_exit(pdev: *mut platform_device);
}

