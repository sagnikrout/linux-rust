//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/shdma-base.h
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
// Dmaengine driver base library for DMA controllers, found on SH-based SoCs
//
// extracted from shdma.c and headers
//
// Copyright (C) 2011-2012 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
// Copyright (C) 2009 Nobuhiro Iwamatsu <iwamatsu.nobuhiro@renesas.com>
// Copyright (C) 2009 Renesas Solutions, Inc. All rights reserved.
// Copyright (C) 2007 Freescale Semiconductor, Inc. All rights reserved.
//

//
// enum shdma_pm_state - DMA channel PM state
// @SHDMA_PM_ESTABLISHED:	either idle or during data transfer
// @SHDMA_PM_BUSY:		during the transfer preparation, when we have to
// drop the lock temporarily
// @SHDMA_PM_PENDING:	transfers pending
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum shdma_pm_state {
    SHDMA_PM_ESTABLISHED,
    SHDMA_PM_BUSY,
    SHDMA_PM_PENDING,
}

//
// Drivers, using this library are expected to embed struct shdma_dev,
// struct shdma_chan, struct shdma_desc, and struct shdma_slave
// in their respective device, channel, descriptor and slave objects.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shdma_slave {
    pub slave_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shdma_desc {
    pub node: list_head,
    pub async_tx: dma_async_tx_descriptor,
    pub direction: dma_transfer_direction,
    pub partial: usize,
    pub cookie: dma_cookie_t,
    pub chunks: c_int,
    pub mark: c_int,
    pub /: *mut *mut bool cyclic; / used as cyclic transfer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shdma_chan {
    pub /: *mut *mut spinlock_t chan_lock; / Channel operation lock,
    pub /: *mut *mut list_head ld_queue; / Link descriptors queue,
    pub /: *mut *mut list_head ld_free; / Free link descriptors,
    pub /: *mut *mut dma_chan dma_chan; / DMA channel,
    pub /: *mut *mut *mut device dev; / Channel device,
    pub /: *mut *mut *mut void desc; / buffer for descriptor array,
    pub /: *mut *mut int desc_num; / desc count,
    pub /: *mut *mut size_t max_xfer_len; / max transfer length,
    pub /: *mut *mut int id; / Raw id of this channel,
    pub /: *mut *mut int irq; / Channel IRQ,
    pub /: *mut *mut int slave_id; / Client ID for slave DMA,
    pub /: *mut *mut int real_slave_id; / argument passed to filter function,
    pub same: *mut *mut int hw_req; / DMA request line for slave DMA -,
// as MID/RID, used with DT
    pub pm_state: shdma_pm_state,
}

//
// struct shdma_ops - simple DMA driver operations
// @desc_completed:	return true, if this is the descriptor, that just has
// completed (atomic)
// @halt_channel:	stop DMA channel operation (atomic)
// @channel_busy:	return true, if the channel is busy (atomic)
// @slave_addr:		return slave DMA address
// @desc_setup:		set up the hardware specific descriptor portion (atomic)
// @set_slave:		bind channel to a slave
// @setup_xfer:		configure channel hardware for operation (atomic)
// @start_xfer:		start the DMA transfer (atomic)
// @embedded_desc:	return Nth struct shdma_desc pointer from the
// descriptor array
// @chan_irq:		process channel IRQ, return true if a transfer has
// completed (atomic)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shdma_ops {
    pub ): *mut *mut *mut bool (desc_completed)(struct shdma_chan , struct shdma_desc,
    pub ): *mut *mut void (halt_channel)(struct shdma_chan,
    pub ): *mut *mut bool (channel_busy)(struct shdma_chan,
    pub ): *mut *mut dma_addr_t (slave_addr)(struct shdma_chan,
    pub ): *mut dma_addr_t, dma_addr_t, size_t,
    pub bool): *mut *mut *mut int (set_slave)(struct shdma_chan , int, dma_addr_t,,
    pub int): *mut *mut *mut int (setup_xfer)(struct shdma_chan ,,
    pub ): *mut *mut *mut void (start_xfer)(struct shdma_chan , struct shdma_desc,
    pub int): *mut *mut *mut *mut shdma_desc (embedded_desc)(void ,,
    pub int): *mut *mut *mut bool (chan_irq)(struct shdma_chan ,,
    pub ): *mut *mut *mut size_t (get_partial)(struct shdma_chan , struct shdma_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shdma_dev {
    pub dma_dev: dma_device,
    pub schan: *mut shdma_chan,
    pub ops: *const shdma_ops,
    pub desc_size: usize,
}

extern "C" {
    pub fn shdma_reset(sdev: *mut shdma_dev) -> bool;
}
extern "C" {
    pub fn shdma_chan_remove(schan: *mut shdma_chan);
}
extern "C" {
    pub fn shdma_cleanup(sdev: *mut shdma_dev);
}

extern "C" {
    pub fn shdma_chan_filter(chan: *mut dma_chan, arg: *mut c_void) -> bool;
}

