//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/musb/musb_dma.h
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
// MUSB OTG driver DMA controller abstraction
//
// Copyright 2005 Mentor Graphics Corporation
// Copyright (C) 2005-2006 by Texas Instruments
// Copyright (C) 2006-2007 Nokia Corporation
//
// DMA Controller Abstraction
//
// DMA Controllers are abstracted to allow use of a variety of different
// implementations of DMA, as allowed by the Inventra USB cores.  On the
// host side, usbcore sets up the DMA mappings and flushes caches; on the
// peripheral side, the gadget controller driver does.  Responsibilities
// of a DMA controller driver include:
//
// - Handling the details of moving multiple USB packets
// in cooperation with the Inventra USB core, including especially
// the correct RX side treatment of short packets and buffer-full
// states (both of which terminate transfers).
//
// - Knowing the correlation between dma channels and the
// Inventra core's local endpoint resources and data direction.
//
// - Maintaining a list of allocated/available channels.
//
// - Updating channel status on interrupts,
// whether shared with the Inventra core or separate.
//
pub const MUSB_HSDMA_BASE: c_uint = 0x200;

pub const MUSB_HSDMA_CONTROL: c_uint = 0x4;
pub const MUSB_HSDMA_ADDRESS: c_uint = 0x8;
pub const MUSB_HSDMA_COUNT: c_uint = 0xc;

pub const musb_dma_ux500(musb): c_int = 0;

pub const musb_dma_cppi41(musb): c_int = 0;

pub const tusb_dma_omap(musb): c_int = 0;

pub const musb_dma_inventra(musb): c_int = 0;

pub const is_cppi_enabled(musb): c_int = 0;

//
// DMA channel status ... updated by the dma controller driver whenever that
// status changes, and protected by the overall controller spinlock.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_channel_status {
// unallocated
    MUSB_DMA_STATUS_UNKNOWN,
// allocated ... but not busy, no errors
    MUSB_DMA_STATUS_FREE,
// busy ... transactions are active
    MUSB_DMA_STATUS_BUSY,
// transaction(s) aborted due to ... dma or memory bus error
    MUSB_DMA_STATUS_BUS_ABORT,
// transaction(s) aborted due to ... core error or USB fault
    MUSB_DMA_STATUS_CORE_ABORT
}

//
// struct dma_channel - A DMA channel.
// @private_data: channel-private data
// @max_len: the maximum number of bytes the channel can move in one
// transaction (typically representing many USB maximum-sized packets)
// @actual_len: how many bytes have been transferred
// @status: current channel status (updated e.g. on interrupt)
// @desired_mode: true if mode 1 is desired; false if mode 0 is desired
//
// channels are associated with an endpoint for the duration of at least
// one usb transfer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_channel {
    pub private_data: *mut c_void,
// FIXME not void* private_data, but a dma_controller *
    pub max_len: usize,
    pub actual_len: usize,
    pub status: dma_channel_status,
    pub desired_mode: bool,
    pub rx_packet_done: bool,
}

//
// dma_channel_status - return status of dma channel
// @c: the channel
//
// Returns the software's view of the channel status.  If that status is BUSY
// then it's possible that the hardware has completed (or aborted) a transfer,
// so the driver needs to update that status.
//
// struct dma_controller - A DMA Controller.
// @musb: the usb controller
// @start: call this to start a DMA controller;
// return 0 on success, else negative errno
// @stop: call this to stop a DMA controller
// return 0 on success, else negative errno
// @channel_alloc: call this to allocate a DMA channel
// @channel_release: call this to release a DMA channel
// @channel_abort: call this to abort a pending DMA transaction,
// returning it to FREE (but allocated) state
// @dma_callback: invoked on DMA completion, useful to run platform
// code such IRQ acknowledgment.
//
// Controllers manage dma channels.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_controller {
    pub musb: *mut musb,
    pub is_tx): *mut *mut musb_hw_ep , u8,
    pub ): *mut *mut void (channel_release)(struct dma_channel,
    pub length): u32,
    pub ): *mut *mut int (channel_abort)(struct dma_channel,
    pub length): *mut *mut void buf, u32,
    pub ): *mut *mut void (dma_callback)(struct dma_controller,
}

// called after channel_program(), may indicate a fault
extern "C" {
    pub fn musb_dma_completion(musb: *mut musb, epnum: u8, transmit: u8);
}

extern "C" {
    pub fn void(: *mut *mut musb_dma_controller_destroy)(struct dma_controller) -> extern;
}

// Platform specific DMA functions
extern "C" {
    pub fn musbhs_dma_controller_destroy(c: *mut dma_controller);
}
extern "C" {
    pub fn dma_controller_irq(irq: c_int, private_data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn tusb_dma_controller_destroy(c: *mut dma_controller);
}
extern "C" {
    pub fn cppi41_dma_controller_destroy(c: *mut dma_controller);
}
extern "C" {
    pub fn ux500_dma_controller_destroy(c: *mut dma_controller);
}
