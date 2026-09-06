//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/atmel/atmel-pcm.h
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
// at91-pcm.h - ALSA PCM interface for the Atmel AT91 SoC.
//
// Copyright (C) 2005 SAN People
// Copyright (C) 2008 Atmel
//
// Authors: Sedji Gaouaou <sedji.gaouaou@atmel.com>
//
// Based on at91-pcm. by:
// Frank Mandarino <fmandarino@endrelia.com>
// Copyright 2006 Endrelia Technologies Inc.
//
// Based on pxa2xx-pcm.c by:
//
// Author:	Nicolas Pitre
// Created:	Nov 30, 2004
// Copyright:	(C) 2004 MontaVista Software, Inc.
//

//
// Registers and status bits that are required by the PCM driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_pdc_regs {
    pub /: *mut *mut unsigned int xpr; / PDC recv/trans pointer,
    pub /: *mut *mut unsigned int xcr; / PDC recv/trans counter,
    pub /: *mut *mut unsigned int xnpr; / PDC next recv/trans pointer,
    pub /: *mut *mut unsigned int xncr; / PDC next recv/trans counter,
    pub /: *mut *mut unsigned int ptcr; / PDC transfer control,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_ssc_mask {
    pub /: *mut *mut u32 ssc_enable; / SSC recv/trans enable,
    pub /: *mut *mut u32 ssc_disable; / SSC recv/trans disable,
    pub /: *mut *mut u32 ssc_error; / SSC error conditions,
    pub /: *mut *mut u32 ssc_endx; / SSC ENDTX or ENDRX,
    pub /: *mut *mut u32 ssc_endbuf; / SSC TXBUFE or RXBUFF,
    pub /: *mut *mut u32 pdc_enable; / PDC recv/trans enable,
    pub /: *mut *mut u32 pdc_disable; / PDC recv/trans disable,
}

//
// This structure, shared between the PCM driver and the interface,
// contains all information required by the PCM driver to perform the
// PDC DMA operation.  All fields except dma_intr_handler() are initialized
// by the interface.  The dma_intr_handler() pointer is set by the PCM
// driver and called by the interface SSC interrupt handler if it is
// non-NULL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_pcm_dma_params {
    pub /: *mut *mut *mut char name; / stream identifier,
    pub /: *mut *mut int pdc_xfer_size; / PDC counter increment in bytes,
    pub /: *mut *mut *mut ssc_device ssc; / SSC device for stream,
    pub /: *mut *mut *mut atmel_pdc_regs pdc; / PDC receive or transmit registers,
    pub /: *mut *mut *mut atmel_ssc_mask mask; / SSC & PDC status bits,
    pub substream: *mut snd_pcm_substream,
    pub ): *mut *mut void (dma_intr_handler)(u32, struct snd_pcm_substream,
}

//
// SSC register access (since ssc_writel() / ssc_readl() require literal name)
//

extern "C" {
    pub fn atmel_pcm_pdc_platform_register(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn atmel_pcm_dma_platform_register(dev: *mut device) -> c_int;
}

