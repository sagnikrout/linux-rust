//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/marvell/mcam-core.h
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
// Marvell camera core structures.
//
// Copyright 2011 Jonathan Corbet corbet@lwn.net
//

//
// Create our own symbols for the supported buffer modes, but, for now,
// base them entirely on which videobuf2 options have been selected.
//

pub const MCAM_MODE_VMALLOC: c_int = 1;

pub const MCAM_MODE_DMA_CONTIG: c_int = 1;

pub const MCAM_MODE_DMA_SG: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcam_state {
    S_NOTREADY,	/* Not yet initialized */
    S_IDLE,		/* Just hanging around */
    S_FLAKED,	/* Some sort of problem */
    S_STREAMING,	/* Streaming data */
    S_BUFWAIT	/* streaming requested but no buffers yet */
}

pub const MAX_DMA_BUFS: c_int = 3;
//
// Different platforms work best with different buffer modes, so we
// let the platform pick.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcam_buffer_mode {
    B_vmalloc = 0,
    B_DMA_contig = 1,
    B_DMA_sg = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcam_chip_id {
    MCAM_CAFE,
    MCAM_ARMADA610,
}

//
// Is a given buffer mode supported by the current kernel configuration?
//

//
// Basic frame states
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcam_frame_state {
    pub frames: c_uint,
    pub singles: c_uint,
    pub delivered: c_uint,
}

pub const NR_MCAM_CLK: c_int = 3;
//
// A description of one of our devices.
// Locking: controlled by s_mutex.  Certain fields, however, require
// the dev_lock spinlock; they are marked as such by comments.
// dev_lock is also required for access to device registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcam_camera {
//
// These fields should be set by the platform code prior to
// calling mcam_register().
//
    pub regs: *mut unsigned char __iomem,
    pub /: *mut *mut unsigned regs_size; / size in bytes of the register space,
    pub dev_lock: spinlock_t,
    pub /: *mut *mut *mut device dev; / For messages, dma alloc,
    pub chip_id: mcam_chip_id,
    pub buffer_mode: mcam_buffer_mode,
    pub /: *mut *mut int mclk_src; / which clock source the mclk derives from,
    pub /: *mut *mut int mclk_div; / Clock Divider Value for MCLK,
    pub bus_type: v4l2_mbus_type,
// MIPI support
// The dphy config value, allocated in board file
// dphy[0]: DPHY3
// dphy[1]: DPHY5
// dphy[2]: DPHY6
//
    pub dphy: *mut c_int,
    pub /: *mut *mut bool mipi_enabled; / flag whether mipi is enabled already,
    pub /: *mut *mut int lane; / lane number,
// clock tree support
    pub clk: [*mut clk; NR_MCAM_CLK],
    pub mclk_hw: clk_hw,
    pub mclk: *mut clk,
//
// Callbacks from the core to the platform code.
//
    pub cam): *mut *mut int (plat_power_up) (struct mcam_camera,
    pub cam): *mut *mut void (plat_power_down) (struct mcam_camera,
    pub cam): *mut *mut void (calc_dphy) (struct mcam_camera,
//
// Everything below here is private to the mcam core and
// should not be touched by the platform code.
//
    pub v4l2_dev: v4l2_device,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub state: mcam_state,
    pub /: *mut *mut unsigned long flags; / Buffer status, mainly (dev_lock),
    pub /: *mut *mut mcam_frame_state frame_state; / Frame state counter,
//
// Subsystem structures.
//
    pub vdev: video_device,
    pub notifier: v4l2_async_notifier,
    pub sensor: *mut v4l2_subdev,
// Videobuf2 stuff
    pub vb_queue: vb2_queue,
    pub /: *mut *mut list_head buffers; / Available frames,
    pub /: *mut *mut unsigned int nbufs; / How many are alloc'd,
    pub /: *mut *mut int next_buf; / Next to consume (dev_lock),
    pub /: *mut *mut char bus_info[32]; / querycap bus_info,
// DMA buffers - vmalloc mode

    pub /: *mut *mut unsigned int dma_buf_size; / allocated size,
    pub /: *mut *mut *mut void dma_bufs[MAX_DMA_BUFS]; / Internal buffer addresses,
    pub /: *mut *mut dma_addr_t dma_handles[MAX_DMA_BUFS]; / Buffer bus addresses,
    pub s_bh_work: work_struct,

    pub /: *mut *mut unsigned int sequence; / Frame sequence number,
    pub /: *mut *mut unsigned int buf_seq[MAX_DMA_BUFS]; / Sequence for individual bufs,
// DMA buffers - DMA modes
    pub vb_bufs: [*mut mcam_vb_buffer; MAX_DMA_BUFS],
// Mode-specific ops, set at open time
    pub cam): *mut *mut void (dma_setup)(struct mcam_camera,
    pub frame): *mut *mut *mut void (frame_complete)(struct mcam_camera cam, int,
// Current operating parameters
    pub pix_format: v4l2_pix_format,
    pub mbus_code: u32,
// Locks
    pub /: *mut *mut mutex s_mutex; / Access to this structure,
}

//
// Register I/O functions.  These are here because the platform code
// may legitimately need to mess with the register space.
//
// Device register I/O
//
extern "C" {
    pub fn ioread32(reg: cam->regs +) -> return;
}
//
// Functions for use by platform code.
//
extern "C" {
    pub fn mccic_register(cam: *mut mcam_camera) -> c_int;
}
extern "C" {
    pub fn mccic_irq(cam: *mut mcam_camera, irqs: c_uint) -> c_int;
}
extern "C" {
    pub fn mccic_shutdown(cam: *mut mcam_camera);
}
extern "C" {
    pub fn mccic_suspend(cam: *mut mcam_camera);
}
extern "C" {
    pub fn mccic_resume(cam: *mut mcam_camera) -> c_int;
}
//
// Register definitions for the m88alp01 camera interface.  Offsets in bytes
// as given in the spec.
//
pub const REG_Y0BAR: c_uint = 0x00;
pub const REG_Y1BAR: c_uint = 0x04;
pub const REG_Y2BAR: c_uint = 0x08;
pub const REG_U0BAR: c_uint = 0x0c;
pub const REG_U1BAR: c_uint = 0x10;
pub const REG_U2BAR: c_uint = 0x14;
pub const REG_V0BAR: c_uint = 0x18;
pub const REG_V1BAR: c_uint = 0x1C;
pub const REG_V2BAR: c_uint = 0x20;
//
// register definitions for MIPI support
//
pub const REG_CSI2_CTRL0: c_uint = 0x100;

pub const REG_CSI2_DPHY3: c_uint = 0x12c;
pub const REG_CSI2_DPHY5: c_uint = 0x134;
pub const REG_CSI2_DPHY6: c_uint = 0x138;
// ...
pub const REG_IMGPITCH: c_uint = 0x24	/* Image pitch register */;

pub const IMGP_YP_MASK: c_uint = 0x00003ffc	/* Y pitch field */;

pub const IMGP_UVP_MASK: c_uint = 0x3ffc0000;
pub const REG_IRQSTATRAW: c_uint = 0x28	/* RAW IRQ Status */;
pub const IRQ_EOF0: c_uint = 0x00000001	/* End of frame 0 */;
pub const IRQ_EOF1: c_uint = 0x00000002	/* End of frame 1 */;
pub const IRQ_EOF2: c_uint = 0x00000004	/* End of frame 2 */;
pub const IRQ_SOF0: c_uint = 0x00000008	/* Start of frame 0 */;
pub const IRQ_SOF1: c_uint = 0x00000010	/* Start of frame 1 */;
pub const IRQ_SOF2: c_uint = 0x00000020	/* Start of frame 2 */;
pub const IRQ_OVERFLOW: c_uint = 0x00000040	/* FIFO overflow */;
pub const IRQ_TWSIW: c_uint = 0x00010000	/* TWSI (smbus) write */;
pub const IRQ_TWSIR: c_uint = 0x00020000	/* TWSI read */;
pub const IRQ_TWSIE: c_uint = 0x00040000	/* TWSI error */;

pub const REG_IRQMASK: c_uint = 0x2c	/* IRQ mask - same bits as IRQSTAT */;
pub const REG_IRQSTAT: c_uint = 0x30	/* IRQ status / clear */;
pub const REG_IMGSIZE: c_uint = 0x34	/* Image size */;
pub const IMGSZ_V_MASK: c_uint = 0x1fff0000;
pub const IMGSZ_V_SHIFT: c_int = 16;
pub const IMGSZ_H_MASK: c_uint = 0x00003fff;
pub const REG_IMGOFFSET: c_uint = 0x38	/* IMage offset */;
pub const REG_CTRL0: c_uint = 0x3c	/* Control 0 */;
pub const C0_ENABLE: c_uint = 0x00000001	/* Makes the whole thing go */;
// Mask for all the format bits
pub const C0_DF_MASK: c_uint = 0x00fffffc    /* Bits 2-23 */;
// RGB ordering
pub const C0_RGB4_RGBX: c_uint = 0x00000000;
pub const C0_RGB4_XRGB: c_uint = 0x00000004;
pub const C0_RGB4_BGRX: c_uint = 0x00000008;
pub const C0_RGB4_XBGR: c_uint = 0x0000000c;
pub const C0_RGB5_RGGB: c_uint = 0x00000000;
pub const C0_RGB5_GRBG: c_uint = 0x00000004;
pub const C0_RGB5_GBRG: c_uint = 0x00000008;
pub const C0_RGB5_BGGR: c_uint = 0x0000000c;
// Spec has two fields for DIN and DOUT, but they must match, so
pub const C0_DF_YUV: c_uint = 0x00000000	/* Data is YUV	    */;
pub const C0_DF_RGB: c_uint = 0x000000a0	/* ... RGB		    */;
pub const C0_DF_BAYER: c_uint = 0x00000140	/* ... Bayer		    */;
// 8-8-8 must be missing from the below - ask
pub const C0_RGBF_565: c_uint = 0x00000000;
pub const C0_RGBF_444: c_uint = 0x00000800;
pub const C0_RGB_BGR: c_uint = 0x00001000	/* Blue comes first */;
pub const C0_YUV_PLANAR: c_uint = 0x00000000	/* YUV 422 planar format */;
pub const C0_YUV_PACKED: c_uint = 0x00008000	/* YUV 422 packed	*/;
pub const C0_YUV_420PL: c_uint = 0x0000a000	/* YUV 420 planar	*/;
// Think that 420 packed must be 111 - ask
pub const C0_YUVE_YUYV: c_uint = 0x00000000	/* Y1CbY0Cr		*/;
pub const C0_YUVE_YVYU: c_uint = 0x00010000	/* Y1CrY0Cb		*/;
pub const C0_YUVE_VYUY: c_uint = 0x00020000	/* CrY1CbY0		*/;
pub const C0_YUVE_UYVY: c_uint = 0x00030000	/* CbY1CrY0		*/;
pub const C0_YUVE_NOSWAP: c_uint = 0x00000000	/* no bytes swapping	*/;
pub const C0_YUVE_SWAP13: c_uint = 0x00010000	/* swap byte 1 and 3	*/;
pub const C0_YUVE_SWAP24: c_uint = 0x00020000	/* swap byte 2 and 4	*/;
pub const C0_YUVE_SWAP1324: c_uint = 0x00030000	/* swap bytes 1&3 and 2&4 */;
// Bayer bits 18,19 if needed
pub const C0_EOF_VSYNC: c_uint = 0x00400000	/* Generate EOF by VSYNC */;
pub const C0_VEDGE_CTRL: c_uint = 0x00800000	/* Detect falling edge of VSYNC */;
pub const C0_HPOL_LOW: c_uint = 0x01000000	/* HSYNC polarity active low */;
pub const C0_VPOL_LOW: c_uint = 0x02000000	/* VSYNC polarity active low */;
pub const C0_VCLK_LOW: c_uint = 0x04000000	/* VCLK on falling edge */;
pub const C0_DOWNSCALE: c_uint = 0x08000000	/* Enable downscaler */;
// SIFMODE
pub const C0_SIF_HVSYNC: c_uint = 0x00000000	/* Use H/VSYNC */;
pub const C0_SOF_NOSYNC: c_uint = 0x40000000	/* Use inband active signaling */;
pub const C0_SIFM_MASK: c_uint = 0xc0000000	/* SIF mode bits */;
// Bits below C1_444ALPHA are not present in Cafe
pub const REG_CTRL1: c_uint = 0x40	/* Control 1 */;
pub const C1_CLKGATE: c_uint = 0x00000001	/* Sensor clock gate */;
pub const C1_DESC_ENA: c_uint = 0x00000100	/* DMA descriptor enable */;
pub const C1_DESC_3WORD: c_uint = 0x00000200	/* Three-word descriptors used */;
pub const C1_444ALPHA: c_uint = 0x00f00000	/* Alpha field in RGB444 */;
pub const C1_ALPHA_SHFT: c_int = 20;
pub const C1_DMAB32: c_uint = 0x00000000	/* 32-byte DMA burst */;
pub const C1_DMAB16: c_uint = 0x02000000	/* 16-byte DMA burst */;
pub const C1_DMAB64: c_uint = 0x04000000	/* 64-byte DMA burst */;
pub const C1_DMAB_MASK: c_uint = 0x06000000;
pub const C1_TWOBUFS: c_uint = 0x08000000	/* Use only two DMA buffers */;
pub const C1_PWRDWN: c_uint = 0x10000000	/* Power down */;
pub const REG_CLKCTRL: c_uint = 0x88	/* Clock control */;
pub const CLK_DIV_MASK: c_uint = 0x0000ffff	/* Upper bits RW "reserved" */;
// This appears to be a Cafe-only register
pub const REG_UBAR: c_uint = 0xc4	/* Upper base address register */;
// Armada 610 DMA descriptor registers
pub const REG_DMA_DESC_Y: c_uint = 0x200;
pub const REG_DMA_DESC_U: c_uint = 0x204;
pub const REG_DMA_DESC_V: c_uint = 0x208;
pub const REG_DESC_LEN_Y: c_uint = 0x20c	/* Lengths are in bytes */;
pub const REG_DESC_LEN_U: c_uint = 0x210;
pub const REG_DESC_LEN_V: c_uint = 0x214;
//
// Useful stuff that probably belongs somewhere global.
//
pub const VGA_WIDTH: c_int = 640;
pub const VGA_HEIGHT: c_int = 480;
