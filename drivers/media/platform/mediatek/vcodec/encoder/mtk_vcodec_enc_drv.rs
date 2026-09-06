//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/encoder/mtk_vcodec_enc_drv.h
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
// Copyright (c) 2023 MediaTek Inc.
// Author: Yunfei Dong <yunfei.dong@mediatek.com>
//

//
// struct mtk_vcodec_enc_pdata - compatible data for each IC
//
// @uses_ext: whether the encoder uses the extended firmware messaging format
// @min_bitrate: minimum supported encoding bitrate
// @max_bitrate: maximum supported encoding bitrate
// @capture_formats: array of supported capture formats
// @num_capture_formats: number of entries in capture_formats
// @output_formats: array of supported output formats
// @num_output_formats: number of entries in output_formats
// @core_id: stand for h264 or vp8 encode index
// @uses_34bit: whether the encoder uses 34-bit iova
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_enc_pdata {
    pub uses_ext: bool,
    pub min_bitrate: u64,
    pub max_bitrate: u64,
    pub capture_formats: *const mtk_video_fmt,
    pub num_capture_formats: usize,
    pub output_formats: *const mtk_video_fmt,
    pub num_output_formats: usize,
    pub core_id: u8,
    pub uses_34bit: bool,
}

//
// enum mtk_encode_param - General encoding parameters type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_encode_param {
    MTK_ENCODE_PARAM_NONE = 0,
    MTK_ENCODE_PARAM_BITRATE = (1 << 0),
    MTK_ENCODE_PARAM_FRAMERATE = (1 << 1),
    MTK_ENCODE_PARAM_INTRA_PERIOD = (1 << 2),
    MTK_ENCODE_PARAM_FORCE_INTRA = (1 << 3),
    MTK_ENCODE_PARAM_GOP_SIZE = (1 << 4),
}

//
// struct mtk_enc_params - General encoding parameters
// @bitrate: target bitrate in bits per second
// @num_b_frame: number of b frames between p-frame
// @rc_frame: frame based rate control
// @rc_mb: macroblock based rate control
// @seq_hdr_mode: H.264 sequence header is encoded separately or joined
// with the first frame
// @intra_period: I frame period
// @gop_size: group of picture size, it's used as the intra frame period
// @framerate_num: frame rate numerator. ex: framerate_num=30 and
// framerate_denom=1 means FPS is 30
// @framerate_denom: frame rate denominator. ex: framerate_num=30 and
// framerate_denom=1 means FPS is 30
// @h264_max_qp: Max value for H.264 quantization parameter
// @h264_profile: V4L2 defined H.264 profile
// @h264_level: V4L2 defined H.264 level
// @force_intra: force/insert intra frame
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_enc_params {
    pub bitrate: c_uint,
    pub num_b_frame: c_uint,
    pub rc_frame: c_uint,
    pub rc_mb: c_uint,
    pub seq_hdr_mode: c_uint,
    pub intra_period: c_uint,
    pub gop_size: c_uint,
    pub framerate_num: c_uint,
    pub framerate_denom: c_uint,
    pub h264_max_qp: c_uint,
    pub h264_profile: c_uint,
    pub h264_level: c_uint,
    pub force_intra: c_uint,
}

//
// struct mtk_vcodec_enc_ctx - Context (instance) private data.
//
// @type: type of encoder instance
// @dev: pointer to the mtk_vcodec_enc_dev of the device
// @list: link to ctx_list of mtk_vcodec_enc_dev
//
// @fh: struct v4l2_fh
// @m2m_ctx: pointer to the v4l2_m2m_ctx of the context
// @q_data: store information of input and output queue of the context
// @id: index of the context that this structure describes
// @state: state of the context
// @param_change: indicate encode parameter type
// @enc_params: encoding parameters
//
// @enc_if: hooked encoder driver interface
// @drv_handle: driver handle for specific decode/encode instance
//
// @int_cond: variable used by the waitqueue
// @int_type: type of the last interrupt
// @queue: waitqueue that can be used to wait for this context to finish
// @irq_status: irq status
//
// @ctrl_hdl: handler for v4l2 framework
// @encode_work: worker for the encoding
// @empty_flush_buf: a fake size-0 capture buffer that indicates flush. Used for encoder.
// @is_flushing: set to true if flushing is in progress.
//
// @colorspace: enum v4l2_colorspace; supplemental to pixelformat
// @ycbcr_enc: enum v4l2_ycbcr_encoding, Y'CbCr encoding
// @quantization: enum v4l2_quantization, colorspace quantization
// @xfer_func: enum v4l2_xfer_func, colorspace transfer function
//
// @q_mutex: vb2_queue mutex.
// @vpu_inst: vpu instance pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_enc_ctx {
    pub type: mtk_instance_type,
    pub dev: *mut mtk_vcodec_enc_dev,
    pub list: list_head,
    pub fh: v4l2_fh,
    pub m2m_ctx: *mut v4l2_m2m_ctx,
    pub q_data: [mtk_q_data; 2],
    pub id: c_int,
    pub state: mtk_instance_state,
    pub param_change: mtk_encode_param,
    pub enc_params: mtk_enc_params,
    pub enc_if: *const venc_common_if,
    pub drv_handle: *mut c_void,
    pub int_cond: [c_int; MTK_VDEC_HW_MAX],
    pub int_type: [c_int; MTK_VDEC_HW_MAX],
    pub queue: [wait_queue_head_t; MTK_VDEC_HW_MAX],
    pub irq_status: c_uint,
    pub ctrl_hdl: v4l2_ctrl_handler,
    pub encode_work: work_struct,
    pub empty_flush_buf: v4l2_m2m_buffer,
    pub is_flushing: bool,
    pub colorspace: v4l2_colorspace,
    pub ycbcr_enc: v4l2_ycbcr_encoding,
    pub quantization: v4l2_quantization,
    pub xfer_func: v4l2_xfer_func,
    pub q_mutex: mutex,
    pub vpu_inst: *mut c_void,
}

//
// struct mtk_vcodec_enc_dev - driver data
// @v4l2_dev: V4L2 device to register video devices for.
// @vfd_enc: Video device for encoder.
//
// @m2m_dev_enc: m2m device for encoder.
// @plat_dev: platform device
// @ctx_list: list of struct mtk_vcodec_ctx
// @curr_ctx: The context that is waiting for codec hardware
//
// @reg_base: Mapped address of MTK Vcodec registers.
// @venc_pdata: encoder IC-specific data
//
// @fw_handler: used to communicate with the firmware.
// @id_counter: used to identify current opened instance
//
// @enc_mutex: encoder hardware lock.
// @dev_mutex: video_device lock
// @dev_ctx_lock: the lock of context list
// @encode_workqueue: encode work queue
//
// @enc_irq: h264 encoder irq resource
// @irqlock: protect data access by irq handler and work thread
//
// @pm: power management control
// @enc_capability: used to identify encode capability
// @dbgfs: debug log related information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_enc_dev {
    pub v4l2_dev: v4l2_device,
    pub vfd_enc: *mut video_device,
    pub m2m_dev_enc: *mut v4l2_m2m_dev,
    pub plat_dev: *mut platform_device,
    pub ctx_list: list_head,
    pub curr_ctx: *mut mtk_vcodec_enc_ctx,
    pub reg_base: [*mut void __iomem; NUM_MAX_VCODEC_REG_BASE],
    pub venc_pdata: *const mtk_vcodec_enc_pdata,
    pub fw_handler: *mut mtk_vcodec_fw,
    pub id_counter: u64,
// encoder hardware mutex lock
    pub enc_mutex: mutex,
    pub dev_mutex: mutex,
    pub dev_ctx_lock: spinlock_t,
    pub encode_workqueue: *mut workqueue_struct,
    pub enc_irq: c_int,
    pub irqlock: spinlock_t,
    pub pm: mtk_vcodec_pm,
    pub enc_capability: c_uint,
    pub dbgfs: mtk_vcodec_dbgfs,
}

extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), mtk_vcodec_enc_ctx: struct, _arg: fh) -> return;
}
extern "C" {
    pub fn container_of(_arg: ctrl->handler, mtk_vcodec_enc_ctx: struct, _arg: ctrl_hdl) -> return;
}
// Wake up context wait_queue

