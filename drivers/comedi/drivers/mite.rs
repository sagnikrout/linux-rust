//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/mite.h
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
// module/mite.h
// Hardware driver for NI Mite PCI interface chip
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 1999 David A. Schleef <ds@schleef.org>
//

pub const MAX_MITE_DMA_CHANNELS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mite_dma_desc {
    pub count: __le32,
    pub addr: __le32,
    pub next: __le32,
    pub dar: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mite_ring {
    pub hw_dev: *mut device,
    pub n_links: c_uint,
    pub descs: *mut mite_dma_desc,
    pub dma_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mite_channel {
    pub mite: *mut mite,
    pub channel: c_uint,
    pub dir: c_int,
    pub done: c_int,
    pub ring: *mut mite_ring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mite {
    pub pcidev: *mut pci_dev,
    pub mmio: *mut void __iomem,
    pub channels: [mite_channel; MAX_MITE_DMA_CHANNELS],
    pub num_channels: c_int,
    pub fifo_size: c_uint,
// protects mite_channel from being released by the driver
    pub lock: spinlock_t,
}

extern "C" {
    pub fn mite_bytes_in_transit(mite_chan: *mut mite_channel) -> u32;
}
extern "C" {
    pub fn mite_sync_dma(mite_chan: *mut mite_channel, s: *mut comedi_subdevice);
}
extern "C" {
    pub fn mite_done(mite_chan: *mut mite_channel) -> c_int;
}
extern "C" {
    pub fn mite_dma_arm(mite_chan: *mut mite_channel);
}
extern "C" {
    pub fn mite_dma_disarm(mite_chan: *mut mite_channel);
}
extern "C" {
    pub fn mite_release_channel(mite_chan: *mut mite_channel);
}
extern "C" {
    pub fn mite_buf_change(ring: *mut mite_ring, s: *mut comedi_subdevice) -> c_int;
}
extern "C" {
    pub fn mite_free_ring(ring: *mut mite_ring);
}
extern "C" {
    pub fn mite_detach(mite: *mut mite);
}
//
// Mite registers (used outside of the mite driver)
//
pub const MITE_IODWBSR: c_uint = 0xc0	/* IO Device Window Base Size */;
pub const MITE_IODWBSR_1: c_uint = 0xc4	/* IO Device Window1 Base Size */;

pub const MITE_IODWCR_1: c_uint = 0xf4;
