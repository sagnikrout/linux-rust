//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/samsung/sxgbe/sxgbe_dma.h
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
// 10G controller driver for Samsung SoCs
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Siva Reddy Kallam <siva.kallam@samsung.com>
//
// forward declaration
pub const SXGBE_DMA_BLENMAP_LSHIFT: c_int = 1;
pub const SXGBE_DMA_TXPBL_LSHIFT: c_int = 16;
pub const SXGBE_DMA_RXPBL_LSHIFT: c_int = 16;
pub const DEFAULT_DMA_PBL: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_dma_ops {
// DMA core initialization
    pub burst_map): *mut *mut *mut int (init)(void __iomem ioaddr, int fix_burst, int,
    pub r_rsize): int t_rzie, int,
    pub dma_cnum): *mut *mut *mut void (enable_dma_transmission)(void __iomem ioaddr, int,
    pub dma_cnum): *mut *mut *mut void (enable_dma_irq)(void __iomem ioaddr, int,
    pub dma_cnum): *mut *mut *mut void (disable_dma_irq)(void __iomem ioaddr, int,
    pub tchannels): *mut *mut *mut void (start_tx)(void __iomem ioaddr, int,
    pub dma_cnum): *mut *mut *mut void (start_tx_queue)(void __iomem ioaddr, int,
    pub tchannels): *mut *mut *mut void (stop_tx)(void __iomem ioaddr, int,
    pub dma_cnum): *mut *mut *mut void (stop_tx_queue)(void __iomem ioaddr, int,
    pub rchannels): *mut *mut *mut void (start_rx)(void __iomem ioaddr, int,
    pub rchannels): *mut *mut *mut void (stop_rx)(void __iomem ioaddr, int,
    pub x): *mut sxgbe_extra_stats,
    pub x): *mut sxgbe_extra_stats,
// Program the HW RX Watchdog
    pub riwt): *mut *mut *mut void (rx_watchdog)(void __iomem ioaddr, u32,
// Enable TSO for each DMA channel
    pub chan_num): *mut *mut *mut void (enable_tso)(void __iomem ioaddr, u8,
}
