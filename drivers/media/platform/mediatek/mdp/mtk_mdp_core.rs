//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp/mtk_mdp_core.h
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
// Copyright (c) 2015-2016 MediaTek Inc.
// Author: Houlong Wei <houlong.wei@mediatek.com>
// Ming Hsiu Tsai <minghsiu.tsai@mediatek.com>
//

pub const MTK_MDP_MAX_CTRL_NUM: c_int = 10;

//
// struct mtk_mdp_pix_align - alignment of image
// @org_w: source alignment of width
// @org_h: source alignment of height
// @target_w: dst alignment of width
// @target_h: dst alignment of height
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mdp_pix_align {
    pub org_w: u16,
    pub org_h: u16,
    pub target_w: u16,
    pub target_h: u16,
}

//
// struct mtk_mdp_fmt - the driver's internal color format data
// @pixelformat: the fourcc code for this format, 0 if not applicable
// @num_planes: number of physically non-contiguous data planes
// @num_comp: number of logical data planes
// @depth: per plane driver's private 'number of bits per pixel'
// @row_depth: per plane driver's private 'number of bits per pixel per row'
// @flags: flags indicating which operation mode format applies to
// MTK_MDP_FMT_FLAG_OUTPUT is used in OUTPUT stream
// MTK_MDP_FMT_FLAG_CAPTURE is used in CAPTURE stream
// @align: pointer to a pixel alignment struct, NULL if using default value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mdp_fmt {
    pub pixelformat: u32,
    pub num_planes: u16,
    pub num_comp: u16,
    pub depth: [u8; VIDEO_MAX_PLANES],
    pub row_depth: [u8; VIDEO_MAX_PLANES],
    pub flags: u32,
    pub align: *mut mtk_mdp_pix_align,
}

//
// struct mtk_mdp_addr - the image processor physical address set
// @addr:	address of planes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mdp_addr {
    pub addr: [dma_addr_t; MTK_MDP_MAX_NUM_PLANE],
}

// struct mtk_mdp_ctrls - the image processor control set
// @rotate: rotation degree
// @hflip: horizontal flip
// @vflip: vertical flip
// @global_alpha: the alpha value of current frame
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mdp_ctrls {
    pub rotate: *mut v4l2_ctrl,
    pub hflip: *mut v4l2_ctrl,
    pub vflip: *mut v4l2_ctrl,
    pub global_alpha: *mut v4l2_ctrl,
}

//
// struct mtk_mdp_frame - source/target frame properties
// @width:	SRC : SRCIMG_WIDTH, DST : OUTPUTDMA_WHOLE_IMG_WIDTH
// @height:	SRC : SRCIMG_HEIGHT, DST : OUTPUTDMA_WHOLE_IMG_HEIGHT
// @crop:	cropped(source)/scaled(destination) size
// @payload:	image size in bytes (w x h x bpp)
// @pitch:	bytes per line of image in memory
// @addr:	image frame buffer physical addresses
// @fmt:	color format pointer
// @alpha:	frame's alpha value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mdp_frame {
    pub width: u32,
    pub height: u32,
    pub crop: v4l2_rect,
    pub payload: [c_ulong; VIDEO_MAX_PLANES],
    pub pitch: [c_uint; VIDEO_MAX_PLANES],
    pub addr: mtk_mdp_addr,
    pub fmt: *const mtk_mdp_fmt,
    pub alpha: u8,
}

//
// struct mtk_mdp_variant - image processor variant information
// @pix_max:		maximum limit of image size
// @pix_min:		minimum limit of image size
// @pix_align:		alignment of image
// @h_scale_up_max:	maximum scale-up in horizontal
// @v_scale_up_max:	maximum scale-up in vertical
// @h_scale_down_max:	maximum scale-down in horizontal
// @v_scale_down_max:	maximum scale-down in vertical
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mdp_variant {
    pub pix_max: *mut mtk_mdp_pix_limit,
    pub pix_min: *mut mtk_mdp_pix_limit,
    pub pix_align: *mut mtk_mdp_pix_align,
    pub h_scale_up_max: u16,
    pub v_scale_up_max: u16,
    pub h_scale_down_max: u16,
    pub v_scale_down_max: u16,
}

//
// struct mtk_mdp_dev - abstraction for image processor entity
// @lock:	the mutex protecting this data structure
// @vpulock:	the mutex protecting the communication with VPU
// @pdev:	pointer to the image processor platform device
// @variant:	the IP variant information
// @id:		image processor device index (0..MTK_MDP_MAX_DEVS)
// @comp_list:	list of MDP function components
// @m2m_dev:	v4l2 memory-to-memory device data
// @ctx_list:	list of struct mtk_mdp_ctx
// @vdev:	video device for image processor driver
// @v4l2_dev:	V4L2 device to register video devices for.
// @job_wq:	processor work queue
// @vpu_dev:	VPU platform device
// @ctx_num:	counter of active MTK MDP context
// @id_counter:	An integer id given to the next opened context
// @wdt_wq:	work queue for VPU watchdog
// @wdt_work:	worker for VPU watchdog
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mdp_dev {
    pub lock: mutex,
    pub vpulock: mutex,
    pub pdev: *mut platform_device,
    pub variant: *mut mtk_mdp_variant,
    pub id: u16,
    pub comp_list: list_head,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub ctx_list: list_head,
    pub vdev: *mut video_device,
    pub v4l2_dev: v4l2_device,
    pub job_wq: *mut workqueue_struct,
    pub vpu_dev: *mut platform_device,
    pub ctx_num: c_int,
    pub id_counter: c_ulong,
    pub wdt_wq: *mut workqueue_struct,
    pub wdt_work: work_struct,
}

//
// struct mtk_mdp_ctx - the device context data
// @list:		link to ctx_list of mtk_mdp_dev
// @s_frame:		source frame properties
// @d_frame:		destination frame properties
// @id:			index of the context that this structure describes
// @flags:		additional flags for image conversion
// @state:		flags to keep track of user configuration
// Protected by slock
// @rotation:		rotates the image by specified angle
// @hflip:		mirror the picture horizontally
// @vflip:		mirror the picture vertically
// @mdp_dev:		the image processor device this context applies to
// @m2m_ctx:		memory-to-memory device context
// @fh:			v4l2 file handle
// @ctrl_handler:	v4l2 controls handler
// @ctrls:		image processor control set
// @ctrls_rdy:		true if the control handler is initialized
// @colorspace:		enum v4l2_colorspace; supplemental to pixelformat
// @ycbcr_enc:		enum v4l2_ycbcr_encoding, Y'CbCr encoding
// @xfer_func:		enum v4l2_xfer_func, colorspace transfer function
// @quant:		enum v4l2_quantization, colorspace quantization
// @vpu:		VPU instance
// @slock:		the mutex protecting mtp_mdp_ctx.state
// @work:		worker for image processing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mdp_ctx {
    pub list: list_head,
    pub s_frame: mtk_mdp_frame,
    pub d_frame: mtk_mdp_frame,
    pub flags: u32,
    pub state: u32,
    pub id: c_int,
    pub rotation: c_int,
    pub hflip:1: u32,
    pub vflip:1: u32,
    pub mdp_dev: *mut mtk_mdp_dev,
    pub m2m_ctx: *mut v4l2_m2m_ctx,
    pub fh: v4l2_fh,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub ctrls: mtk_mdp_ctrls,
    pub ctrls_rdy: bool,
    pub colorspace: v4l2_colorspace,
    pub ycbcr_enc: v4l2_ycbcr_encoding,
    pub xfer_func: v4l2_xfer_func,
    pub quant: v4l2_quantization,
    pub vpu: mtk_mdp_vpu,
    pub slock: mutex,
    pub work: work_struct,
}

// Macro flag: #define mtk_mdp_dbg_enter()
// Macro flag: #define mtk_mdp_dbg_leave()

