//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/exynos4-is/fimc-lite.h
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
// Copyright (C) 2012 Samsung Electronics Co., Ltd.
//

pub const FIMC_LITE_MAX_DEVS: c_int = 3;
pub const FLITE_REQ_BUFS_MIN: c_int = 2;
pub const FLITE_DEFAULT_WIDTH: c_int = 640;
pub const FLITE_DEFAULT_HEIGHT: c_int = 480;
// Bit index definitions for struct fimc_lite::state
pub const FLITE_SD_PAD_SINK: c_int = 0;
pub const FLITE_SD_PAD_SOURCE_DMA: c_int = 1;
pub const FLITE_SD_PAD_SOURCE_ISP: c_int = 2;
pub const FLITE_SD_PADS_NUM: c_int = 3;
//
// struct flite_drvdata - FIMC-LITE IP variant data structure
// @max_width: maximum camera interface input width in pixels
// @max_height: maximum camera interface input height in pixels
// @out_width_align: minimum output width alignment in pixels
// @win_hor_offs_align: minimum camera interface crop window horizontal
// offset alignment in pixels
// @out_hor_offs_align: minimum output DMA compose rectangle horizontal
// offset alignment in pixels
// @max_dma_bufs: number of output DMA buffer start address registers
// @num_instances: total number of FIMC-LITE IP instances available
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flite_drvdata {
    pub max_width: c_ushort,
    pub max_height: c_ushort,
    pub out_width_align: c_ushort,
    pub win_hor_offs_align: c_ushort,
    pub out_hor_offs_align: c_ushort,
    pub max_dma_bufs: c_ushort,
    pub num_instances: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_lite_events {
    pub data_overflow: c_uint,
}

pub const FLITE_MAX_PLANES: c_int = 1;
//
// struct flite_frame - source/target frame properties
// @f_width: full pixel width
// @f_height: full pixel height
// @rect: crop/composition rectangle
// @fmt: pointer to pixel format description data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flite_frame {
    pub f_width: u16,
    pub f_height: u16,
    pub rect: v4l2_rect,
    pub fmt: *const fimc_fmt,
}

//
// struct flite_buffer - video buffer structure
// @vb:    vb2 buffer
// @list:  list head for the buffers queue
// @addr: DMA buffer start address
// @index: DMA start address register's index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flite_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub addr: dma_addr_t,
    pub index: c_ushort,
}

//
// struct fimc_lite - fimc lite structure
// @pdev: pointer to FIMC-LITE platform device
// @dd: SoC specific driver data structure
// @ve: exynos video device entity structure
// @v4l2_dev: pointer to top the level v4l2_device
// @fh: v4l2 file handle
// @subdev: FIMC-LITE subdev
// @vd_pad: media (sink) pad for the capture video node
// @subdev_pads: the subdev media pads
// @sensor: sensor subdev attached to FIMC-LITE directly or through MIPI-CSIS
// @ctrl_handler: v4l2 control handler
// @test_pattern: test pattern controls
// @index: FIMC-LITE platform device index
// @slock: spinlock protecting this data structure and the hw registers
// @lock: mutex serializing video device and the subdev operations
// @clock: FIMC-LITE gate clock
// @regs: memory mapped io registers
// @irq_queue: interrupt handler waitqueue
// @payload: image size in bytes (w x h x bpp)
// @inp_frame: camera input frame structure
// @out_frame: DMA output frame structure
// @out_path: output data path (DMA or FIFO)
// @source_subdev_grp_id: source subdev group id
// @state: driver state flags
// @pending_buf_q: pending buffers queue head
// @active_buf_q: the queue head of buffers scheduled in hardware
// @vb_queue: vb2 buffers queue
// @buf_index: helps to keep track of the DMA start address register index
// @frame_count: the captured frames counter
// @reqbufs_count: the number of buffers requested with REQBUFS ioctl
// @events: event info
// @streaming: is streaming in progress?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_lite {
    pub pdev: *mut platform_device,
    pub dd: *mut flite_drvdata,
    pub ve: exynos_video_entity,
    pub v4l2_dev: *mut v4l2_device,
    pub fh: v4l2_fh,
    pub subdev: v4l2_subdev,
    pub vd_pad: media_pad,
    pub subdev_pads: [media_pad; FLITE_SD_PADS_NUM],
    pub sensor: *mut v4l2_subdev,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub test_pattern: *mut v4l2_ctrl,
    pub index: c_int,
    pub lock: mutex,
    pub slock: spinlock_t,
    pub clock: *mut clk,
    pub regs: *mut void __iomem,
    pub irq_queue: wait_queue_head_t,
    pub payload: [c_ulong; FLITE_MAX_PLANES],
    pub inp_frame: flite_frame,
    pub out_frame: flite_frame,
    pub out_path: core::sync::atomic::AtomicI32,
    pub source_subdev_grp_id: c_uint,
    pub state: c_ulong,
    pub pending_buf_q: list_head,
    pub active_buf_q: list_head,
    pub vb_queue: vb2_queue,
    pub buf_index: c_ushort,
    pub frame_count: c_uint,
    pub reqbufs_count: c_uint,
    pub events: fimc_lite_events,
    pub streaming: bool,
}
