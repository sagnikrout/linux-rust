//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/tw5864/tw5864.h
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
// TW5864 driver  - common header file
//
// Copyright (C) 2016 Bluecherry, LLC <maintainers@bluecherrydvr.com>
//

pub const PCI_DEVICE_ID_TECHWELL_5864: c_uint = 0x5864;

// -----------------------------------------------------------
// card configuration
pub const TW5864_INPUTS: c_int = 4;
// The TW5864 uses 192 (16x12) detection cells in full screen for motion
// detection. Each detection cell is composed of 44 pixels and 20 lines for
// NTSC and 24 lines for PAL.
//
pub const MD_CELLS_HOR: c_int = 16;
pub const MD_CELLS_VERT: c_int = 12;

pub const H264_VLC_BUF_SIZE: c_uint = 0x80000;
pub const H264_MV_BUF_SIZE: c_uint = 0x2000 /* device writes 5396 bytes */;
pub const QP_VALUE: c_int = 28;
pub const MAX_GOP_SIZE: c_int = 255;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum resolution {
    D1 = 1,
    HD1 = 2, /* half d1 - 360x(240|288) */
    CIF = 3,
    QCIF = 4,
}

// -----------------------------------------------------------
// device / file handle status
// buffer for one video/vbi/ts frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw5864_buf {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw5864_dma_buf {
    pub addr: *mut c_void,
    pub dma_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tw5864_vid_std {
    STD_NTSC = 0, /* NTSC (M) */
    STD_PAL = 1, /* PAL (B, D, G, H, I) */
    STD_SECAM = 2, /* SECAM */
    STD_NTSC443 = 3, /* NTSC4.43 */
    STD_PAL_M = 4, /* PAL (M) */
    STD_PAL_CN = 5, /* PAL (CN) */
    STD_PAL_60 = 6, /* PAL 60 */
    STD_INVALID = 7,
    STD_AUTO = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw5864_input {
    pub /: *mut *mut int nr; / input number,
    pub root: *mut tw5864_dev,
    pub /: *mut *mut mutex lock; / used for vidq and vdev,
    pub /: *mut *mut spinlock_t slock; / used for sync between ISR, bh_work & V4L2 API,
    pub vdev: video_device,
    pub hdl: v4l2_ctrl_handler,
    pub vidq: vb2_queue,
    pub active: list_head,
    pub resolution: resolution,
    pub height: unsigned int width,,
    pub frame_seqno: c_uint,
    pub frame_gop_seqno: c_uint,
    pub h264_idr_pic_id: c_uint,
    pub enabled: c_int,
    pub std: tw5864_vid_std,
    pub v4l2_std: v4l2_std_id,
    pub tail_nb_bits: c_int,
    pub tail: u8,
    pub buf_cur_ptr: *mut u8,
    pub buf_cur_space_left: c_int,
    pub reg_interlacing: u32,
    pub reg_vlc: u32,
    pub reg_dsp_codec: u32,
    pub reg_dsp: u32,
    pub reg_emu: u32,
    pub reg_dsp_qp: u32,
    pub reg_dsp_ref_mvp_lambda: u32,
    pub reg_dsp_i4x4_weight: u32,
    pub buf_id: u32,
    pub vb: *mut tw5864_buf,
    pub md_threshold_grid_ctrl: *mut v4l2_ctrl,
    pub 16]: *mut *mut u16 md_threshold_grid_values[12,
    pub qp: c_int,
    pub gop: c_int,
//
// In (1/MAX_FPS) units.
// For max FPS (default), set to 1.
// For 1 FPS, set to e.g. 32.
//
    pub frame_interval: c_int,
    pub new_frame_deadline: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw5864_h264_frame {
    pub vlc: tw5864_dma_buf,
    pub mv: tw5864_dma_buf,
    pub vlc_len: c_int,
    pub checksum: u32,
    pub input: *mut tw5864_input,
    pub timestamp: u64,
    pub seqno: c_uint,
    pub gop_seqno: c_uint,
}

// global device status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw5864_dev {
    pub /: *mut *mut spinlock_t slock; / used for sync between ISR, bh_work & V4L2 API,
    pub v4l2_dev: v4l2_device,
    pub inputs: [tw5864_input; TW5864_INPUTS],
pub const H264_BUF_CNT: c_int = 4;
    pub h264_buf: [tw5864_h264_frame; H264_BUF_CNT],
    pub h264_buf_r_index: c_int,
    pub h264_buf_w_index: c_int,
    pub bh_work: work_struct,
    pub encoder_busy: c_int,
// Input number to check next for ready raw picture (in RR fashion)
    pub next_input: c_int,
// pci i/o
    pub name: [c_char; 64],
    pub pci: *mut pci_dev,
    pub mmio: *mut void __iomem,
    pub irqmask: u32,
}

extern "C" {
    pub fn tw5864_indir_readb(dev: *mut tw5864_dev, addr: u16) -> u8;
}

extern "C" {
    pub fn tw5864_indir_writeb(dev: *mut tw5864_dev, addr: u16, data: u8);
}

extern "C" {
    pub fn tw5864_irqmask_apply(dev: *mut tw5864_dev);
}
extern "C" {
    pub fn tw5864_video_init(dev: *mut tw5864_dev, video_nr: *mut c_int) -> c_int;
}
extern "C" {
    pub fn tw5864_video_fini(dev: *mut tw5864_dev);
}
extern "C" {
    pub fn tw5864_prepare_frame_headers(input: *mut tw5864_input);
}
extern "C" {
    pub fn tw5864_request_encoded_frame(input: *mut tw5864_input);
}
