//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/dma-dw.h
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
// Driver for the Synopsys DesignWare DMA Controller
//
// Copyright (C) 2007 Atmel Corporation
// Copyright (C) 2010-2011 ST Microelectronics
//

pub const DW_DMA_MAX_NR_MASTERS: c_int = 4;
pub const DW_DMA_MAX_NR_CHANNELS: c_int = 8;
pub const DW_DMA_MIN_BURST: c_int = 1;
pub const DW_DMA_MAX_BURST: c_int = 256;
//
// struct dw_dma_slave - Controller-specific information about a slave
//
// @dma_dev:	required DMA master device
// @src_id:	src request line
// @dst_id:	dst request line
// @m_master:	memory master for transfers on allocated channel
// @p_master:	peripheral master for transfers on allocated channel
// @channels:	mask of the channels permitted for allocation (zero value means any)
// @hs_polarity:set active low polarity of handshake interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_dma_slave {
    pub dma_dev: *mut device,
    pub src_id: u8,
    pub dst_id: u8,
    pub m_master: u8,
    pub p_master: u8,
    pub channels: u8,
    pub hs_polarity: bool,
}

//
// struct dw_dma_platform_data - Controller configuration parameters
// @nr_masters: Number of AHB masters supported by the controller
// @nr_channels: Number of channels supported by hardware (max 8)
// @chan_allocation_order: Allocate channels starting from 0 or 7
// @chan_priority: Set channel priority increasing from 0 to 7 or 7 to 0.
// @block_size: Maximum block size supported by the controller
// @data_width: Maximum data width supported by hardware per AHB master
// (in bytes, power of 2)
// @multi_block: Multi block transfers supported by hardware per channel.
// @max_burst: Maximum value of burst transaction size supported by hardware
// per channel (in units of CTL.SRC_TR_WIDTH/CTL.DST_TR_WIDTH).
// @protctl: Protection control signals setting per channel.
// @quirks: Optional platform quirks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_dma_platform_data {
    pub nr_masters: u32,
    pub nr_channels: u32,

    pub chan_allocation_order: u32,

    pub chan_priority: u32,
    pub block_size: u32,
    pub data_width: [u32; DW_DMA_MAX_NR_MASTERS],
    pub multi_block: [u32; DW_DMA_MAX_NR_CHANNELS],
    pub max_burst: [u32; DW_DMA_MAX_NR_CHANNELS],
    pub protctl: u32,

    pub quirks: u32,
}
