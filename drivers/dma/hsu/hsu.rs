//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/hsu/hsu.h
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
//
// Driver for the High Speed UART DMA
//
// Copyright (C) 2015 Intel Corporation
//
// Partially based on the bits found in drivers/tty/serial/mfd.c.
//

pub const HSU_CH_SR: c_uint = 0x00			/* channel status */;
pub const HSU_CH_CR: c_uint = 0x04			/* channel control */;
pub const HSU_CH_DCR: c_uint = 0x08			/* descriptor control */;
pub const HSU_CH_BSR: c_uint = 0x10			/* FIFO buffer size */;
pub const HSU_CH_MTSR: c_uint = 0x14			/* minimum transfer size */;

pub const HSU_CH_D0SAR: c_uint = 0x20			/* desc 0 start addr */;
pub const HSU_CH_D0TSR: c_uint = 0x24			/* desc 0 transfer size */;
pub const HSU_CH_D1SAR: c_uint = 0x28;
pub const HSU_CH_D1TSR: c_uint = 0x2c;
pub const HSU_CH_D2SAR: c_uint = 0x30;
pub const HSU_CH_D2TSR: c_uint = 0x34;
pub const HSU_CH_D3SAR: c_uint = 0x38;
pub const HSU_CH_D3TSR: c_uint = 0x3c;
pub const HSU_DMA_CHAN_NR_DESC: c_int = 4;
pub const HSU_DMA_CHAN_LENGTH: c_uint = 0x40;
// Bits in HSU_CH_SR

// Bits in HSU_CH_CR

// Bits in HSU_CH_DCR

// Bits in HSU_CH_DxTSR

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsu_dma_sg {
    pub addr: dma_addr_t,
    pub len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsu_dma_desc {
    pub vdesc: virt_dma_desc,
    pub direction: dma_transfer_direction,
    pub sg: *mut hsu_dma_sg,
    pub nents: c_uint,
    pub length: usize,
    pub active: c_uint,
    pub status: dma_status,
}

extern "C" {
    pub fn container_of(_arg: vdesc, hsu_dma_desc: struct, _arg: vdesc) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsu_dma_chan {
    pub vchan: virt_dma_chan,
    pub reg: *mut void __iomem,
// hardware configuration
    pub direction: dma_transfer_direction,
    pub config: dma_slave_config,
    pub desc: *mut hsu_dma_desc,
}

extern "C" {
    pub fn container_of(_arg: chan, hsu_dma_chan: struct, _arg: vchan.chan) -> return;
}
extern "C" {
    pub fn readl(offset: hsuc->reg +) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsu_dma {
    pub dma: dma_device,
// channels
    pub chan: *mut hsu_dma_chan,
    pub nr_channels: c_ushort,
}

extern "C" {
    pub fn container_of(_arg: ddev, hsu_dma: struct, _arg: dma) -> return;
}
