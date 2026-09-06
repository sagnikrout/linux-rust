//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cobalt/cobalt-driver.h
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
// cobalt driver internal defines and structures
//
// Derived from cx18-driver.h
//
// Copyright 2012-2015 Cisco Systems, Inc. and/or its affiliates.
// All rights reserved.
//

// System device ID
pub const PCI_DEVICE_ID_COBALT: c_uint = 0x2732;
// Number of cobalt device nodes.
pub const COBALT_NUM_INPUTS: c_int = 4;
pub const COBALT_NUM_NODES: c_int = 6;
// Number of cobalt device streams.
pub const COBALT_NUM_STREAMS: c_int = 12;
pub const COBALT_HSMA_IN_NODE: c_int = 4;
pub const COBALT_HSMA_OUT_NODE: c_int = 5;
// Cobalt audio streams
pub const COBALT_AUDIO_IN_STREAM: c_int = 6;
pub const COBALT_AUDIO_OUT_STREAM: c_int = 11;
// DMA stuff
pub const DMA_CHANNELS_MAX: c_int = 16;
// i2c stuff
pub const I2C_CLIENTS_MAX: c_int = 16;
pub const COBALT_NUM_ADAPTERS: c_int = 5;
pub const COBALT_CLK: c_int = 50000000;
// System status register

// Cobalt memory map
pub const COBALT_I2C_0_BASE: c_uint = 0x0;
pub const COBALT_I2C_1_BASE: c_uint = 0x080;
pub const COBALT_I2C_2_BASE: c_uint = 0x100;
pub const COBALT_I2C_3_BASE: c_uint = 0x180;
pub const COBALT_I2C_HSMA_BASE: c_uint = 0x200;
pub const COBALT_SYS_CTRL_BASE: c_uint = 0x400;
pub const COBALT_SYS_CTRL_HSMA_TX_ENABLE_BIT: c_int = 1;

pub const COBALT_SYS_CTRL_PWRDN0_TO_HSMA_TX_BIT: c_int = 24;
pub const COBALT_SYS_CTRL_VIDEO_TX_RESETN_BIT: c_int = 25;
pub const COBALT_SYS_CTRL_AUDIO_OPP_RESETN_BIT: c_int = 27;
pub const COBALT_SYS_STAT_BASE: c_uint = 0x500;

pub const COBALT_HDL_INFO_BASE: c_uint = 0x4800;
pub const COBALT_HDL_INFO_SIZE: c_uint = 0x200;
pub const COBALT_VID_BASE: c_uint = 0x10000;
pub const COBALT_VID_SIZE: c_uint = 0x1000;

pub const DMA_INTERRUPT_STATUS_REG: c_uint = 0x08;

// Cobalt CPU bus interface
pub const COBALT_BUS_BAR1_BASE: c_uint = 0x600;
pub const COBALT_BUS_SRAM_BASE: c_uint = 0x0;
pub const COBALT_BUS_CPLD_BASE: c_uint = 0x00600000;
pub const COBALT_BUS_FLASH_BASE: c_uint = 0x08000000;
// FDMA to PCIe packing
pub const COBALT_BYTES_PER_PIXEL_YUYV: c_int = 2;
pub const COBALT_BYTES_PER_PIXEL_RGB24: c_int = 3;
pub const COBALT_BYTES_PER_PIXEL_RGB32: c_int = 4;
// debugging

// Per I2C bus private algo callback data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cobalt_i2c_data {
    pub cobalt: *mut cobalt,
    pub regs: *mut cobalt_i2c_regs __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_consistent_buffer {
    pub virt: *mut c_void,
    pub bus: dma_addr_t,
    pub bytes: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_dma_desc_info {
    pub virt: *mut c_void,
    pub bus: dma_addr_t,
    pub size: unsigned,
    pub last_desc_virt: *mut c_void,
    pub dev: *mut device,
}

pub const COBALT_MAX_WIDTH: c_int = 1920;
pub const COBALT_MAX_HEIGHT: c_int = 1200;
pub const COBALT_MAX_BPP: c_int = 3;

pub const COBALT_STREAM_FL_DMA_IRQ: c_int = 0;
pub const COBALT_STREAM_FL_ADV_IRQ: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cobalt_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
}

extern "C" {
    pub fn container_of(_arg: vb2, cobalt_buffer: struct, _arg: vb) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cobalt_stream {
    pub vdev: video_device,
    pub q: vb2_queue,
    pub bufs: list_head,
    pub i2c_adap: *mut i2c_adapter,
    pub sd: *mut v4l2_subdev,
    pub lock: mutex,
    pub irqlock: spinlock_t,
    pub timings: v4l2_dv_timings,
    pub input: u32,
    pub pad_source: u32,
    pub bpp: u32 width, height,,
    pub stride: u32,
    pub pixfmt: u32,
    pub sequence: u32,
    pub colorspace: u32,
    pub xfer_func: u32,
    pub ycbcr_enc: u32,
    pub quantization: u32,
    pub dma_channel: u8,
    pub video_channel: c_int,
    pub dma_fifo_mask: unsigned,
    pub adv_irq_mask: unsigned,
    pub dma_desc_info: [sg_dma_desc_info; NR_BUFS],
    pub flags: c_ulong,
    pub unstable_frame: bool,
    pub enable_cvi: bool,
    pub enable_freewheel: bool,
    pub skip_first_frames: unsigned,
    pub is_output: bool,
    pub is_audio: bool,
    pub is_dummy: bool,
    pub cobalt: *mut cobalt,
    pub alsa: *mut snd_cobalt_card,
}

// Struct to hold info about cobalt cards
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cobalt {
    pub instance: c_int,
    pub pci_dev: *mut pci_dev,
    pub v4l2_dev: v4l2_device,
// serialize PCI access in cobalt_s_bit_sysctrl()
    pub pci_lock: mutex,
    pub bar1: *mut *mut void __iomem bar0,,
    pub card_rev: u8,
    pub device_id: u16,
// device nodes
    pub streams: [cobalt_stream; DMA_CHANNELS_MAX],
    pub i2c_adap: [i2c_adapter; COBALT_NUM_ADAPTERS],
    pub i2c_data: [cobalt_i2c_data; COBALT_NUM_ADAPTERS],
    pub have_hsma_rx: bool,
    pub have_hsma_tx: bool,
// irq
    pub irq_work_queues: *mut workqueue_struct,
    pub /: *mut *mut work_irq_work_queue; / work entry,
// irq counters
    pub irq_adv1: u32,
    pub irq_adv2: u32,
    pub irq_advout: u32,
    pub irq_dma_tot: u32,
    pub irq_dma: [u32; COBALT_NUM_STREAMS],
    pub irq_none: u32,
    pub irq_full_fifo: u32,
// omnitek dma
    pub dma_channels: c_int,
    pub first_fifo_channel: c_int,
    pub pci_32_bit: bool,
    pub hdl_info: [c_char; COBALT_HDL_INFO_SIZE],
// NOR flash
    pub mtd: *mut mtd_info,
}

extern "C" {
    pub fn container_of(_arg: v4l2_dev, cobalt: struct, _arg: v4l2_dev) -> return;
}
extern "C" {
    pub fn ioread32(reg: cobalt->bar0 +) -> return;
}
extern "C" {
    pub fn ioread32(reg: cobalt->bar1 +) -> return;
}
extern "C" {
    pub fn cobalt_read_bar1(_arg: cobalt, _arg: COBALT_SYS_CTRL_BASE) -> return;
}
extern "C" {
    pub fn cobalt_read_bar1(_arg: cobalt, _arg: COBALT_SYS_STAT_BASE) -> return;
}

extern "C" {
    pub fn ioread32(_arg: LOWER_DATA) -> return;
}
// ==============Prototypes==================
extern "C" {
    pub fn cobalt_pcie_status_show(cobalt: *mut cobalt);
}
