//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/dt3155/dt3155.h
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
// Copyright (C) 2006-2010 by Marin Mitov
// mitov@issp.bas.bg
//
// DT3155 header file

pub const DT3155_VER_MAJ: c_int = 2;
pub const DT3155_VER_MIN: c_int = 0;
pub const DT3155_VER_EXT: c_int = 0;

// DT3155 Base Register offsets (memory mapped)
pub const EVEN_DMA_START: c_uint = 0x00;
pub const ODD_DMA_START: c_uint = 0x0C;
pub const EVEN_DMA_STRIDE: c_uint = 0x18;
pub const ODD_DMA_STRIDE: c_uint = 0x24;
pub const EVEN_PIXEL_FMT: c_uint = 0x30;
pub const ODD_PIXEL_FMT: c_uint = 0x34;
pub const FIFO_TRIGGER: c_uint = 0x38;
pub const XFER_MODE: c_uint = 0x3C;
pub const CSR1: c_uint = 0x40;
pub const RETRY_WAIT_CNT: c_uint = 0x44;
pub const INT_CSR: c_uint = 0x48;
pub const EVEN_FLD_MASK: c_uint = 0x4C;
pub const ODD_FLD_MASK: c_uint = 0x50;
pub const MASK_LENGTH: c_uint = 0x54;
pub const FIFO_FLAG_CNT: c_uint = 0x58;
pub const IIC_CLK_DUR: c_uint = 0x5C;
pub const IIC_CSR1: c_uint = 0x60;
pub const IIC_CSR2: c_uint = 0x64;
// DT3155 Internal Registers indexes (i2c/IIC mapped)
pub const CSR2: c_uint = 0x10;
pub const EVEN_CSR: c_uint = 0x11;
pub const ODD_CSR: c_uint = 0x12;
pub const CONFIG: c_uint = 0x13;
pub const DT_ID: c_uint = 0x1F;
pub const X_CLIP_START: c_uint = 0x20;
pub const Y_CLIP_START: c_uint = 0x22;
pub const X_CLIP_END: c_uint = 0x24;
pub const Y_CLIP_END: c_uint = 0x26;
pub const AD_ADDR: c_uint = 0x30;
pub const AD_LUT: c_uint = 0x31;
pub const AD_CMD: c_uint = 0x32;
pub const DIG_OUT: c_uint = 0x40;
pub const PM_LUT_ADDR: c_uint = 0x50;
pub const PM_LUT_DATA: c_uint = 0x51;
// AD command register values
pub const AD_CMD_REG: c_uint = 0x00;
pub const AD_POS_REF: c_uint = 0x01;
pub const AD_NEG_REF: c_uint = 0x02;
// CSR1 bit masks
pub const RANGE_EN: c_uint = 0x00008000;
pub const CRPT_DIS: c_uint = 0x00004000;
pub const ADDR_ERR_ODD: c_uint = 0x00000800;
pub const ADDR_ERR_EVEN: c_uint = 0x00000400;
pub const FLD_CRPT_ODD: c_uint = 0x00000200;
pub const FLD_CRPT_EVEN: c_uint = 0x00000100;
pub const FIFO_EN: c_uint = 0x00000080;
pub const SRST: c_uint = 0x00000040;
pub const FLD_DN_ODD: c_uint = 0x00000020;
pub const FLD_DN_EVEN: c_uint = 0x00000010;
// These should not be used.
// Use CAP_CONT_ODD/EVEN instead
pub const CAP_SNGL_ODD: c_uint = 0x00000008;
pub const CAP_SNGL_EVEN: c_uint = 0x00000004;
//
pub const CAP_CONT_ODD: c_uint = 0x00000002;
pub const CAP_CONT_EVEN: c_uint = 0x00000001;
// INT_CSR bit masks
pub const FLD_START_EN: c_uint = 0x00000400;
pub const FLD_END_ODD_EN: c_uint = 0x00000200;
pub const FLD_END_EVEN_EN: c_uint = 0x00000100;
pub const FLD_START: c_uint = 0x00000004;
pub const FLD_END_ODD: c_uint = 0x00000002;
pub const FLD_END_EVEN: c_uint = 0x00000001;
// IIC_CSR1 bit masks
pub const DIRECT_ABORT: c_uint = 0x00000200;
// IIC_CSR2 bit masks
pub const NEW_CYCLE: c_uint = 0x01000000;
pub const DIR_RD: c_uint = 0x00010000;
pub const IIC_READ: c_uint = 0x01010000;
pub const IIC_WRITE: c_uint = 0x01000000;
// CSR2 bit masks
pub const DISP_PASS: c_uint = 0x40;
pub const BUSY_ODD: c_uint = 0x20;
pub const BUSY_EVEN: c_uint = 0x10;
pub const SYNC_PRESENT: c_uint = 0x08;
pub const VT_50HZ: c_uint = 0x04;
pub const SYNC_SNTL: c_uint = 0x02;
pub const CHROM_FILT: c_uint = 0x01;
pub const VT_60HZ: c_uint = 0x00;
// CSR_EVEN/ODD bit masks
pub const CSR_ERROR: c_uint = 0x04;
pub const CSR_SNGL: c_uint = 0x02;
pub const CSR_DONE: c_uint = 0x01;
// CONFIG bit masks
pub const PM_LUT_PGM: c_uint = 0x80;
pub const PM_LUT_SEL: c_uint = 0x40;
pub const CLIP_EN: c_uint = 0x20;
pub const HSCALE_EN: c_uint = 0x10;
pub const EXT_TRIG_UP: c_uint = 0x0C;
pub const EXT_TRIG_DOWN: c_uint = 0x04;
pub const ACQ_MODE_NEXT: c_uint = 0x02;
pub const ACQ_MODE_ODD: c_uint = 0x01;
pub const ACQ_MODE_EVEN: c_uint = 0x00;
// AD_CMD bit masks
pub const VIDEO_CNL_1: c_uint = 0x00;
pub const VIDEO_CNL_2: c_uint = 0x40;
pub const VIDEO_CNL_3: c_uint = 0x80;
pub const VIDEO_CNL_4: c_uint = 0xC0;
pub const SYNC_CNL_1: c_uint = 0x00;
pub const SYNC_CNL_2: c_uint = 0x10;
pub const SYNC_CNL_3: c_uint = 0x20;
pub const SYNC_CNL_4: c_uint = 0x30;
pub const SYNC_LVL_1: c_uint = 0x00;
pub const SYNC_LVL_2: c_uint = 0x04;
pub const SYNC_LVL_3: c_uint = 0x08;
pub const SYNC_LVL_4: c_uint = 0x0C;
// DT3155 identificator
pub const DT3155_ID: c_uint = 0x20;
// per board private data structure
//
// struct dt3155_priv - private data structure
//
// @v4l2_dev:		v4l2_device structure
// @vdev:		video_device structure
// @pdev:		pointer to pci_dev structure
// @vidq:		vb2_queue structure
// @curr_buf:		pointer to curren buffer
// @mux:		mutex to protect the instance
// @dmaq:		queue for dma buffers
// @lock:		spinlock for dma queue
// @std:		input standard
// @width:		frame width
// @height:		frame height
// @input:		current input
// @sequence:		frame counter
// @regs:		local copy of mmio base register
// @csr2:		local copy of csr2 register
// @config:		local copy of config register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dt3155_priv {
    pub v4l2_dev: v4l2_device,
    pub vdev: video_device,
    pub pdev: *mut pci_dev,
    pub vidq: vb2_queue,
    pub curr_buf: *mut vb2_v4l2_buffer,
    pub mux: mutex,
    pub dmaq: list_head,
    pub lock: spinlock_t,
    pub std: v4l2_std_id,
    pub height: unsigned width,,
    pub input: unsigned,
    pub sequence: c_uint,
    pub regs: *mut void __iomem,
    pub config: u8 csr2,,
}
