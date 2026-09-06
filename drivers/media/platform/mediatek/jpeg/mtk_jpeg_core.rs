//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/jpeg/mtk_jpeg_core.h
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
// Copyright (c) 2016 MediaTek Inc.
// Author: Ming Hsiu Tsai <minghsiu.tsai@mediatek.com>
// Rick Chang <rick.chang@mediatek.com>
// Xia Jiang <xia.jiang@mediatek.com>
//

pub const MTK_JPEG_HW_TIMEOUT_MSEC: c_int = 1000;

//
// enum mtk_jpeg_ctx_state - states of the context state machine
// @MTK_JPEG_INIT:		current state is initialized
// @MTK_JPEG_RUNNING:		current state is running
// @MTK_JPEG_SOURCE_CHANGE:	current state is source resolution change
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_jpeg_ctx_state {
    MTK_JPEG_INIT = 0,
    MTK_JPEG_RUNNING,
    MTK_JPEG_SOURCE_CHANGE,
}

//
// struct mtk_jpeg_variant - mtk jpeg driver variant
// @clks:			clock names
// @num_clks:			numbers of clock
// @formats:			jpeg driver's internal color format
// @num_formats:		number of formats
// @qops:			the callback of jpeg vb2_ops
// @irq_handler:		jpeg irq handler callback
// @hw_reset:			jpeg hardware reset callback
// @m2m_ops:			the callback of jpeg v4l2_m2m_ops
// @dev_name:			jpeg device name
// @ioctl_ops:			the callback of jpeg v4l2_ioctl_ops
// @out_q_default_fourcc:	output queue default fourcc
// @cap_q_default_fourcc:	capture queue default fourcc
// @multi_core:		mark jpeg hw is multi_core or not
// @jpeg_worker:		jpeg dec or enc worker
// @support_34bit:	flag to check support for 34-bit DMA address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_jpeg_variant {
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub formats: *mut mtk_jpeg_fmt,
    pub num_formats: c_int,
    pub qops: *const vb2_ops,
    pub priv): *mut *mut irqreturn_t (irq_handler)(int irq, void,
    pub base): *mut *mut void (hw_reset)(void __iomem,
    pub m2m_ops: *const v4l2_m2m_ops,
    pub dev_name: *const c_char,
    pub ioctl_ops: *const v4l2_ioctl_ops,
    pub out_q_default_fourcc: u32,
    pub cap_q_default_fourcc: u32,
    pub multi_core: bool,
    pub work): *mut *mut void (jpeg_worker)(struct work_struct,
    pub support_34bit: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_jpeg_src_buf {
    pub frame_num: u32,
    pub b: vb2_v4l2_buffer,
    pub list: list_head,
    pub bs_size: u32,
    pub dec_param: mtk_jpeg_dec_param,
    pub curr_ctx: *mut mtk_jpeg_ctx,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_jpeg_hw_state {
    MTK_JPEG_HW_IDLE = 0,
    MTK_JPEG_HW_BUSY = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_jpeg_hw_param {
    pub src_buffer: *mut vb2_v4l2_buffer,
    pub dst_buffer: *mut vb2_v4l2_buffer,
    pub curr_ctx: *mut mtk_jpeg_ctx,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_jpegenc_hw_id {
    MTK_JPEGENC_HW0,
    MTK_JPEGENC_HW1,
    MTK_JPEGENC_HW_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_jpegdec_hw_id {
    MTK_JPEGDEC_HW0,
    MTK_JPEGDEC_HW1,
    MTK_JPEGDEC_HW2,
    MTK_JPEGDEC_HW_MAX,
}

//
// struct mtk_jpegenc_clk - Structure used to store vcodec clock information
// @clks:		JPEG encode clock
// @clk_num:		JPEG encode clock numbers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_jpegenc_clk {
    pub clks: *mut clk_bulk_data,
    pub clk_num: c_int,
}

//
// struct mtk_jpegdec_clk - Structure used to store vcodec clock information
// @clks:		JPEG decode clock
// @clk_num:		JPEG decode clock numbers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_jpegdec_clk {
    pub clks: *mut clk_bulk_data,
    pub clk_num: c_int,
}

//
// struct mtk_jpegenc_comp_dev - JPEG COREX abstraction
// @dev:		JPEG device
// @plat_dev:		platform device data
// @reg_base:		JPEG registers mapping
// @master_dev:		mtk_jpeg_dev device
// @venc_clk:		jpeg encode clock
// @jpegenc_irq:	jpeg encode irq num
// @job_timeout_work:	encode timeout workqueue
// @hw_param:		jpeg encode hw parameters
// @hw_state:		record hw state
// @hw_lock:		spinlock protecting the hw device resource
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_jpegenc_comp_dev {
    pub dev: *mut device,
    pub plat_dev: *mut platform_device,
    pub reg_base: *mut void __iomem,
    pub master_dev: *mut mtk_jpeg_dev,
    pub venc_clk: mtk_jpegenc_clk,
    pub jpegenc_irq: c_int,
    pub job_timeout_work: delayed_work,
    pub hw_param: mtk_jpeg_hw_param,
    pub hw_state: mtk_jpeg_hw_state,
// spinlock protecting the hw device resource
    pub hw_lock: spinlock_t,
}

//
// struct mtk_jpegdec_comp_dev - JPEG COREX abstraction
// @dev:			JPEG device
// @plat_dev:			platform device data
// @reg_base:			JPEG registers mapping
// @master_dev:			mtk_jpeg_dev device
// @jdec_clk:			mtk_jpegdec_clk
// @jpegdec_irq:		jpeg decode irq num
// @job_timeout_work:		decode timeout workqueue
// @hw_param:			jpeg decode hw parameters
// @hw_state:			record hw state
// @hw_lock:			spinlock protecting hw
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_jpegdec_comp_dev {
    pub dev: *mut device,
    pub plat_dev: *mut platform_device,
    pub reg_base: *mut void __iomem,
    pub master_dev: *mut mtk_jpeg_dev,
    pub jdec_clk: mtk_jpegdec_clk,
    pub jpegdec_irq: c_int,
    pub job_timeout_work: delayed_work,
    pub hw_param: mtk_jpeg_hw_param,
    pub hw_state: mtk_jpeg_hw_state,
// spinlock protecting the hw device resource
    pub hw_lock: spinlock_t,
}

//
// struct mtk_jpeg_dev - JPEG IP abstraction
// @lock:		the mutex protecting this structure
// @hw_lock:		spinlock protecting the hw device resource
// @workqueue:		decode work queue
// @dev:		JPEG device
// @v4l2_dev:		v4l2 device for mem2mem mode
// @m2m_dev:		v4l2 mem2mem device data
// @alloc_ctx:		videobuf2 memory allocator's context
// @vdev:		video device node for jpeg mem2mem mode
// @reg_base:		JPEG registers mapping
// @job_timeout_work:	IRQ timeout structure
// @variant:		driver variant to be used
// @reg_encbase:	jpg encode register base addr
// @enc_hw_dev:	jpg encode hardware device
// @hw_wq:		jpg wait queue
// @hw_rdy:		jpg hw ready flag
// @reg_decbase:	jpg decode register base addr
// @dec_hw_dev:	jpg decode hardware device
// @hw_index:		jpg hw index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_jpeg_dev {
    pub lock: mutex,
    pub hw_lock: spinlock_t,
    pub workqueue: *mut workqueue_struct,
    pub dev: *mut device,
    pub v4l2_dev: v4l2_device,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub alloc_ctx: *mut c_void,
    pub vdev: *mut video_device,
    pub reg_base: *mut void __iomem,
    pub job_timeout_work: delayed_work,
    pub variant: *const mtk_jpeg_variant,
    pub reg_encbase: [*mut void __iomem; MTK_JPEGENC_HW_MAX],
    pub enc_hw_dev: [*mut mtk_jpegenc_comp_dev; MTK_JPEGENC_HW_MAX],
    pub hw_wq: wait_queue_head_t,
    pub hw_rdy: core::sync::atomic::AtomicI32,
    pub reg_decbase: [*mut void __iomem; MTK_JPEGDEC_HW_MAX],
    pub dec_hw_dev: [*mut mtk_jpegdec_comp_dev; MTK_JPEGDEC_HW_MAX],
    pub hw_index: core::sync::atomic::AtomicI32,
}

//
// struct mtk_jpeg_fmt - driver's internal color format data
// @fourcc:	the fourcc code, 0 if not applicable
// @hw_format:	hardware format value
// @h_sample:	horizontal sample count of plane in 4 * 4 pixel image
// @v_sample:	vertical sample count of plane in 4 * 4 pixel image
// @colplanes:	number of color planes (1 for packed formats)
// @h_align:	horizontal alignment order (align to 2^h_align)
// @v_align:	vertical alignment order (align to 2^v_align)
// @flags:	flags describing format applicability
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_jpeg_fmt {
    pub fourcc: u32,
    pub hw_format: u32,
    pub h_sample: [c_int; VIDEO_MAX_PLANES],
    pub v_sample: [c_int; VIDEO_MAX_PLANES],
    pub colplanes: c_int,
    pub h_align: c_int,
    pub v_align: c_int,
    pub flags: u32,
}

//
// struct mtk_jpeg_q_data - parameters of one queue
// @fmt:	  driver-specific format of this queue
// @pix_mp:	  multiplanar format
// @enc_crop_rect:	jpeg encoder crop information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_jpeg_q_data {
    pub fmt: *mut mtk_jpeg_fmt,
    pub pix_mp: v4l2_pix_format_mplane,
    pub enc_crop_rect: v4l2_rect,
}

//
// struct mtk_jpeg_ctx - the device context data
// @jpeg:			JPEG IP device for this context
// @out_q:			source (output) queue information
// @cap_q:			destination queue information
// @fh:				V4L2 file handle
// @state:			state of the context
// @enable_exif:		enable exif mode of jpeg encoder
// @enc_quality:		jpeg encoder quality
// @restart_interval:		jpeg encoder restart interval
// @ctrl_hdl:			controls handler
// @jpeg_work:			jpeg encoder workqueue
// @total_frame_num:		encoded frame number
// @dst_done_queue:		encoded frame buffer queue
// @done_queue_lock:		encoded frame operation spinlock
// @last_done_frame_num:	the last encoded frame number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_jpeg_ctx {
    pub jpeg: *mut mtk_jpeg_dev,
    pub out_q: mtk_jpeg_q_data,
    pub cap_q: mtk_jpeg_q_data,
    pub fh: v4l2_fh,
    pub state: mtk_jpeg_ctx_state,
    pub enable_exif: bool,
    pub enc_quality: u8,
    pub restart_interval: u8,
    pub ctrl_hdl: v4l2_ctrl_handler,
    pub jpeg_work: work_struct,
    pub total_frame_num: u32,
    pub dst_done_queue: list_head,
// spinlock protecting the encode done buffer
    pub done_queue_lock: spinlock_t,
    pub last_done_frame_num: u32,
}
