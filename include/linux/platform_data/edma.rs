//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/edma.h
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
// TI EDMA definitions
//
// Copyright (C) 2006-2013 Texas Instruments.
//
// This EDMA3 programming framework exposes two basic kinds of resource:
//
// Channel	Triggers transfers, usually from a hardware event but
// also manually or by "chaining" from DMA completions.
// Each channel is coupled to a Parameter RAM (PaRAM) slot.
//
// Slot	Each PaRAM slot holds a DMA transfer descriptor (PaRAM
// "set"), source and destination addresses, a link to a
// next PaRAM slot (if any), options for the transfer, and
// instructions for updating those addresses.  There are
// more than twice as many slots as event channels.
//
// Each PaRAM set describes a sequence of transfers, either for one large
// buffer or for several discontiguous smaller buffers.  An EDMA transfer
// is driven only from a channel, which performs the transfers specified
// in its PaRAM slot until there are no more transfers.  When that last
// transfer completes, the "link" field may be used to reload the channel's
// PaRAM slot with a new transfer descriptor.
//
// The EDMA Channel Controller (CC) maps requests from channels into physical
// Transfer Controller (TC) requests when the channel triggers (by hardware
// or software events, or by chaining).  The two physical DMA channels provided
// by the TCs are thus shared by many logical channels.
//
// DaVinci hardware also has a "QDMA" mechanism which is not currently
// supported through this interface.  (DSP firmware uses it though.)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_event_q {
    EVENTQ_0 = 0,
    EVENTQ_1 = 1,
    EVENTQ_2 = 2,
    EVENTQ_3 = 3,
    EVENTQ_DEFAULT = -1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct edma_rsv_info {
    pub (*rsv_chans)[2]: *const i16,
    pub (*rsv_slots)[2]: *const i16,
}

// platform_data for EDMA driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edma_soc_info {
//
// Default queue is expected to be a low-priority queue.
// This way, long transfers on the default queue started
// by the codec engine will not cause audio defects.
//
    pub default_queue: dma_event_q,
// Resource reservation for other cores
    pub rsv: *mut edma_rsv_info,
// List of channels allocated for memcpy, terminated with -1
    pub memcpy_channels: *mut i32,
    pub (*queue_priority_mapping)[2]: *mut i8,
    pub (*xbar_chans)[2]: *const i16,
    pub slave_map: *const dma_slave_map,
    pub slavecnt: c_int,
}
