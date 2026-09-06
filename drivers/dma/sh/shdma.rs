//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/sh/shdma.h
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
// Renesas SuperH DMA Engine support
//
// Copyright (C) 2009 Nobuhiro Iwamatsu <iwamatsu.nobuhiro@renesas.com>
// Copyright (C) 2009 Renesas Solutions, Inc. All rights reserved.
//

pub const SH_DMAE_MAX_CHANNELS: c_int = 20;
pub const SH_DMAE_TCR_MAX: c_uint = 0x00FFFFFF	/* 16MB */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_dmae_chan {
    pub shdma_chan: shdma_chan,
    pub /: *const *const *const sh_dmae_slave_config config; / Slave DMA configuration,
    pub /: *mut *mut int xmit_shift; / log_2(bytes_per_xfer),
    pub base: *mut void __iomem,
    pub /: *mut *mut char dev_id[32]; / unique name per DMAC of channel,
    pub pm_error: c_int,
    pub slave_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_dmae_device {
    pub shdma_dev: shdma_dev,
    pub chan: [*mut sh_dmae_chan; SH_DMAE_MAX_CHANNELS],
    pub pdata: *const sh_dmae_pdata,
    pub node: list_head,
    pub chan_reg: *mut void __iomem,
    pub dmars: *mut void __iomem,
    pub chcr_offset: c_uint,
    pub chcr_ie_bit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_dmae_regs {
    pub /: *mut *mut u32 sar; / SAR / source address,
    pub /: *mut *mut u32 dar; / DAR / destination address,
    pub /: *mut *mut u32 tcr; / TCR / transfer count,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_dmae_desc {
    pub hw: sh_dmae_regs,
    pub shdma_desc: shdma_desc,
}

