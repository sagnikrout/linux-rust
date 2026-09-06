//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/omap/omap_voutdef.h
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


//
// omap_voutdef.h
//
// Copyright (C) 2010 Texas Instruments.
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

pub const YUYV_BPP: c_int = 2;
pub const RGB565_BPP: c_int = 2;
pub const RGB24_BPP: c_int = 3;
pub const RGB32_BPP: c_int = 4;
pub const TILE_SIZE: c_int = 32;
pub const YUYV_VRFB_BPP: c_int = 2;
pub const RGB_VRFB_BPP: c_int = 1;
pub const MAX_CID: c_int = 3;
pub const MAC_VRFB_CTXS: c_int = 4;
pub const MAX_VOUT_DEV: c_int = 2;
pub const MAX_OVLS: c_int = 3;
pub const MAX_DISPLAYS: c_int = 10;
pub const MAX_MANAGERS: c_int = 3;
pub const QQVGA_WIDTH: c_int = 160;
pub const QQVGA_HEIGHT: c_int = 120;
// Max Resolution supported by the driver

// Minimum requirement is 2x2 for DSS
pub const VID_MIN_WIDTH: c_int = 2;
pub const VID_MIN_HEIGHT: c_int = 2;
// 2048 x 2048 is max res supported by OMAP display controller
pub const MAX_PIXELS_PER_LINE: c_int = 2048;
pub const VRFB_TX_TIMEOUT: c_int = 1000;
pub const VRFB_NUM_BUFS: c_int = 4;
// Max buffer size to be allocated during init

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_channel_state {
    DMA_CHAN_NOT_ALLOTED,
    DMA_CHAN_ALLOTED,
}

// Enum for Rotation
// DSS understands rotation in 0, 1, 2, 3 context
// while V4L2 driver understands it as 0, 90, 180, 270
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dss_rotation {
    dss_rotation_0_degree	= 0,
    dss_rotation_90_degree	= 1,
    dss_rotation_180_degree	= 2,
    dss_rotation_270_degree = 3,
}

// Enum for choosing rotation type for vout
// DSS2 doesn't understand no rotation as an
// option while V4L2 driver doesn't support
// rotation in the case where VRFB is not built in
// the kernel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vout_rotaion_type {
    VOUT_ROT_NONE	= 0,
    VOUT_ROT_VRFB	= 1,
}

//
// This structure is used to store the DMA transfer parameters
// for VRFB hidden buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vid_vrfb_dma {
    pub chan: *mut dma_chan,
    pub xt: *mut dma_interleaved_template,
    pub req_status: c_int,
    pub tx_status: c_int,
    pub wait: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapvideo_info {
    pub id: c_int,
    pub num_overlays: c_int,
    pub overlays: [*mut omap_overlay; MAX_OVLS],
    pub rotation_type: vout_rotaion_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap2video_device {
    pub mtx: mutex,
    pub state: c_int,
    pub v4l2_dev: v4l2_device,
    pub vouts: [*mut omap_vout_device; MAX_VOUT_DEV],
    pub num_displays: c_int,
    pub displays: [*mut omap_dss_device; MAX_DISPLAYS],
    pub num_overlays: c_int,
    pub overlays: [*mut omap_overlay; MAX_OVLS],
    pub num_managers: c_int,
    pub managers: [*mut omap_overlay_manager; MAX_MANAGERS],
}

// buffer for one video frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_vout_buffer {
// common v4l buffer stuff -- must be first
    pub vbuf: vb2_v4l2_buffer,
    pub queue: list_head,
}

extern "C" {
    pub fn container_of(_arg: vbuf, omap_vout_buffer: struct, _arg: vbuf) -> return;
}
// per-device data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_vout_device {
    pub vid_info: omapvideo_info,
    pub vfd: *mut video_device,
    pub vid_dev: *mut omap2video_device,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub vid: c_int,
// allow to reuse previously allocated buffer which is big enough
    pub buffer_size: c_int,
    pub dss_mode: omap_color_mode,
    pub sequence: u32,
    pub pix: v4l2_pix_format,
    pub crop: v4l2_rect,
    pub win: v4l2_window,
    pub fbuf: v4l2_framebuffer,
// Lock to protect the shared data structures in ioctl
    pub lock: mutex,
    pub rotation: dss_rotation,
    pub mirror: bool,
    pub flicker_filter: c_int,
    pub /: *mut *mut int bpp; / bytes per pixel,
    pub /: *mut *mut int vrfb_bpp; / bytes per pixel with respect to VRFB,
    pub vrfb_dma_tx: vid_vrfb_dma,
    pub smsshado_phy_addr: [c_uint; MAC_VRFB_CTXS],
    pub smsshado_virt_addr: [c_uint; MAC_VRFB_CTXS],
    pub vrfb_context: [vrfb; MAC_VRFB_CTXS],
    pub vrfb_static_allocation: bool,
    pub smsshado_size: c_uint,
    pub pos: c_uchar,
    pub field_id: int ps, vr_ps, line_length, first_int,,
    pub next_frm: *mut *mut omap_vout_buffer cur_frm,,
    pub /: *mut *mut spinlock_t vbq_lock; / spinlock for dma_queue,
    pub dma_queue: list_head,
    pub queued_buf_addr: [dma_addr_t; VIDEO_MAX_FRAME],
    pub cropped_offset: u32,
    pub tv_field1_offset: i32,
    pub isr_handle: *mut c_void,
    pub vq: vb2_queue,
}

//
// Return true if rotation is 90 or 270
//
// Return true if rotation is enabled
//
// Reverse the rotation degree if mirroring is enabled
//
extern "C" {
    pub fn omap_vout_free_buffers(vout: *mut omap_vout_device);
}
