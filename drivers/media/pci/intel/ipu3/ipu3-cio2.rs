//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu3/ipu3-cio2.h
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
// Copyright (C) 2017 Intel Corporation

pub const CIO2_PCI_ID: c_uint = 0x9d32;
pub const CIO2_PCI_BAR: c_int = 0;

// 32MB = 8xFBPT_entry
pub const CIO2_MAX_LOPS: c_int = 8;

// 1 for each sensor

// Register and bit field definitions

pub const CIO2_REG_CSIRX_BASE: c_uint = 0x000;
pub const CIO2_REG_MIPIBE_BASE: c_uint = 0x100;
pub const CIO2_REG_PIXELGEN_BAS: c_uint = 0x200;
pub const CIO2_REG_IRQCTRL_BASE: c_uint = 0x300;
pub const CIO2_REG_GPREG_BASE: c_uint = 0x1000;
// base register: CIO2_REG_PIPE_BASE(pipe) * CIO2_REG_CSIRX_BASE

pub const CIO2_CSIRX_IF_CONFIG_FILTEROUT: c_uint = 0x00;
pub const CIO2_CSIRX_IF_CONFIG_FILTEROUT_VC_INACTIVE: c_uint = 0x01;
pub const CIO2_CSIRX_IF_CONFIG_PASS: c_uint = 0x02;

pub const CIO2_CSIRX_STATUS_DLANE_HS_MASK: c_uint = 0xff;

pub const CIO2_CSIRX_STATUS_DLANE_LP_MASK: c_uint = 0xffffff;
// Termination enable and settle in 0.0625ns units, lane=0..3 or -1 for clock

// base register: CIO2_REG_PIPE_BASE(pipe) * CIO2_REG_MIPIBE_BASE

// base register: CIO2_REG_PIPE_BASE(pipe) * CIO2_REG_IRQCTRL_BASE
// IRQ registers are 18-bit wide, see cio2_irq_error for bit definitions

pub const CIO2_GPREG_SRST_ALL: c_uint = 0xffff	/* Reset all */;

pub const CIO2_REG_CGC: c_uint = 0x1400;

pub const CIO2_REG_D0I3C: c_uint = 0x1408;

pub const CIO2_REG_SWRESET: c_uint = 0x140c;

pub const CIO2_REG_SENSOR_ACTIVE: c_uint = 0x1410;
pub const CIO2_REG_INT_STS: c_uint = 0x1414;
pub const CIO2_REG_INT_STS_EXT_OE: c_uint = 0x1418;

pub const CIO2_INT_EXT_OE_DMAOE_MASK: c_uint = 0x7ffff;

pub const CIO2_REG_INT_EN: c_uint = 0x1420;

//
// Interrupt on completion bit, Eg. DMA 0-3 maps to bit 0-3,
// DMA4 & DMA5 map to bit 4 ... DMA18 & DMA19 map to bit 11 Et cetera
//

pub const CIO2_INT_IOC_SHIFT: c_int = 0;

pub const CIO2_INT_IOS_IOLN_SHIFT: c_int = 12;

pub const CIO2_REG_INT_EN_EXT_OE: c_uint = 0x1424;
pub const CIO2_REG_DMA_DBG: c_uint = 0x1448;

pub const CIO2_REG_PBM_ARB_CTRL: c_uint = 0x1460;

pub const CIO2_REG_PBM_WMCTRL1: c_uint = 0x1464;

pub const CIO2_REG_PBM_WMCTRL2: c_uint = 0x1468;

pub const CIO2_REG_PBM_TS_COUNT: c_uint = 0x146c;
pub const CIO2_REG_PBM_FOPN_ABORT: c_uint = 0x1474;
// below n = 0..3

pub const CIO2_REG_LTRCTRL: c_uint = 0x1480;

pub const CIO2_LTRCTRL_LTRSTABLETIME_MASK: c_uint = 0xff;

pub const CIO2_REG_LTRVAL23: c_uint = 0x1484;
pub const CIO2_REG_LTRVAL01: c_uint = 0x1488;

// Value times 1024 ns

pub const CIO2_CDMARI_FBPT_RP_MASK: c_uint = 0xff;

// n = 0..3

pub const CIO2_REG_INT_STS_EXT_IE: c_uint = 0x17e4;
pub const CIO2_REG_INT_EN_EXT_IE: c_uint = 0x17e8;

pub const CIO2_FB_HPLL_FREQ: c_uint = 0x2;
pub const CIO2_ISCLK_RATIO: c_uint = 0xc;
pub const CIO2_IRQCTRL_MASK: c_uint = 0x3ffff;
pub const CIO2_INT_EN_EXT_OE_MASK: c_uint = 0x8f0fffff;

pub const CIO2_PXM_FRF_CFG_CRC_TH: c_int = 16;
pub const CIO2_INT_EN_EXT_IE_MASK: c_uint = 0xffffffff;

pub const CIO2_CSIRX_DLY_CNT_TERMEN_CLANE_A: c_int = 0;
pub const CIO2_CSIRX_DLY_CNT_TERMEN_CLANE_B: c_int = 0;
pub const CIO2_CSIRX_DLY_CNT_SETTLE_CLANE_A: c_int = 95;

pub const CIO2_CSIRX_DLY_CNT_TERMEN_DLANE_A: c_int = 0;
pub const CIO2_CSIRX_DLY_CNT_TERMEN_DLANE_B: c_int = 0;
pub const CIO2_CSIRX_DLY_CNT_SETTLE_DLANE_A: c_int = 85;

pub const CIO2_CSIRX_DLY_CNT_TERMEN_DEFAULT: c_uint = 0x4;
pub const CIO2_CSIRX_DLY_CNT_SETTLE_DEFAULT: c_uint = 0x570;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cio2_csi2_timing {
    pub clk_termen: i32,
    pub clk_settle: i32,
    pub dat_termen: i32,
    pub dat_settle: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cio2_buffer {
    pub vbb: vb2_v4l2_buffer,
    pub lop: [*mut u32; CIO2_MAX_LOPS],
    pub lop_bus_addr: [dma_addr_t; CIO2_MAX_LOPS],
    pub offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csi2_bus_info {
    pub port: u32,
    pub lanes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cio2_queue {
// mutex to be used by vb2_queue
    pub lock: mutex,
    pub pipe: media_pipeline,
    pub csi2: csi2_bus_info,
    pub sensor: *mut v4l2_subdev,
    pub csi_rx_base: *mut void __iomem,
// Subdev, /dev/v4l-subdevX
    pub subdev: v4l2_subdev,
    pub subdev_pads: [media_pad; CIO2_PADS],
    pub frame_sequence: core::sync::atomic::AtomicI32,
// Video device, /dev/videoX
    pub vdev: video_device,
    pub vdev_pad: media_pad,
    pub format: v4l2_pix_format_mplane,
    pub vbq: vb2_queue,
// Buffer queue handling
    pub /: *mut *mut *mut cio2_fbpt_entry fbpt; / Frame buffer pointer table,
    pub fbpt_bus_addr: dma_addr_t,
    pub bufs: [*mut cio2_buffer; CIO2_MAX_BUFFERS],
    pub /: *mut *mut unsigned int bufs_first; / Index of the first used entry,
    pub /: *mut *mut unsigned int bufs_next; / Index of the first unused entry,
    pub bufs_queued: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cio2_device {
    pub pci_dev: *mut pci_dev,
    pub base: *mut void __iomem,
    pub v4l2_dev: v4l2_device,
    pub queue: [cio2_queue; CIO2_QUEUES],
    pub cur_queue: *mut cio2_queue,
// mutex to be used by video_device
    pub lock: mutex,
    pub streaming: bool,
    pub notifier: v4l2_async_notifier,
    pub media_dev: media_device,
//
// Safety net to catch DMA fetch ahead
// when reaching the end of LOP
//
    pub dummy_page: *mut c_void,
// DMA handle of dummy_page
    pub dummy_page_bus_addr: dma_addr_t,
// single List of Pointers (LOP) page
    pub dummy_lop: *mut u32,
// DMA handle of dummy_lop
    pub dummy_lop_bus_addr: dma_addr_t,
}

// Virtual channel
//
// This should come from sensor driver. No
// driver interface nor requirement yet.
//
pub const SENSOR_VIR_CH_DFLT: c_int = 0;
// FBPT operations

pub const CIO2_FBPT_SUBENTRY_UNIT: c_int = 4;
// cio2 fbpt first_entry ctrl status

pub const CIO2_FBPT_CTRL_CMPLCODE_SHIFT: c_int = 4;
//
// Frame Buffer Pointer Table(FBPT) entry
// each entry describe an output buffer and consists of
// several sub-entries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub /: *mut *mut u32 ctrl; / status ctrl,
    pub /: *mut *mut u16 cur_line_num; / current line # written to DDR,
    pub /: *mut *mut u16 frame_num; / updated by DMA upon FE,
    pub /: *mut *mut u32 first_page_offset; / offset for 1st page in LOP,
    pub first_entry: },
// Second entry per buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub timestamp: u32,
    pub num_of_bytes: u32,
// the number of bytes for write on last page
    pub last_page_available_bytes: u16,
// the number of pages allocated for this buf
    pub num_of_pages: u16,
    pub second_entry: },
}

extern "C" {
    pub fn container_of(_arg: video_devdata(file), cio2_queue: struct, _arg: vdev) -> return;
}
extern "C" {
    pub fn container_of(_arg: vq, cio2_queue: struct, _arg: vbq) -> return;
}
