//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/decoder/mtk_vcodec_dec_drv.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_vcodec_dec_chip_name {
    MTK_VDEC_INVAL = 0,
    MTK_VDEC_MT8173 = 8173,
    MTK_VDEC_MT8183 = 8183,
    MTK_VDEC_MT8186 = 8186,
    MTK_VDEC_MT8188 = 8188,
    MTK_VDEC_MT8192 = 8192,
    MTK_VDEC_MT8195 = 8195,
}

//
// enum mtk_vdec_format_types - Structure used to get supported
// format types according to decoder capability
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_vdec_format_types {
    MTK_VDEC_FORMAT_MM21 = 0x20,
    MTK_VDEC_FORMAT_MT21C = 0x40,
    MTK_VDEC_FORMAT_H264_SLICE = 0x100,
    MTK_VDEC_FORMAT_VP8_FRAME = 0x200,
    MTK_VDEC_FORMAT_VP9_FRAME = 0x400,
    MTK_VDEC_FORMAT_AV1_FRAME = 0x800,
    MTK_VDEC_FORMAT_HEVC_FRAME = 0x1000,
    MTK_VCODEC_INNER_RACING = 0x20000,
    MTK_VDEC_IS_SUPPORT_10BIT = 0x40000,
    MTK_VDEC_IS_SUPPORT_EXT = 0x80000,
}

//
// enum mtk_vdec_hw_count - Supported hardware count
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_vdec_hw_count {
    MTK_VDEC_NO_HW = 0,
    MTK_VDEC_ONE_CORE,
    MTK_VDEC_ONE_LAT_ONE_CORE,
    MTK_VDEC_MAX_HW_COUNT,
}

//
// enum mtk_vdec_hw_arch - Used to separate different hardware architecture
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_vdec_hw_arch {
    MTK_VDEC_PURE_SINGLE_CORE,
    MTK_VDEC_LAT_SINGLE_CORE,
}

//
// struct vdec_pic_info  - picture size information
// @pic_w: picture width
// @pic_h: picture height
// @buf_w: picture buffer width (64 aligned up from pic_w)
// @buf_h: picture buffer height (64 aligned up from pic_h)
// @fb_sz: bitstream size of each plane
// E.g. suppose picture size is 176x144,
// buffer size will be aligned to 176x160.
// @cap_fourcc: fourcc number(may change on a resolution change)
// @reserved: align struct to 64-bit in order to adjust 32-bit and 64-bit os.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_pic_info {
    pub pic_w: c_uint,
    pub pic_h: c_uint,
    pub buf_w: c_uint,
    pub buf_h: c_uint,
    pub fb_sz: [c_uint; VIDEO_MAX_PLANES],
    pub cap_fourcc: c_uint,
    pub reserved: c_uint,
}

//
// struct mtk_vcodec_dec_pdata - compatible data for each IC
// @init_vdec_params: init vdec params
// @ctrls_setup: init vcodec dec ctrls
// @worker: worker to start a decode job
// @flush_decoder: function that flushes the decoder
// @get_cap_buffer: get capture buffer from capture queue
// @cap_to_disp: put capture buffer to disp list for lat and core arch
// @vdec_vb2_ops: struct vb2_ops
//
// @vdec_formats: supported video decoder formats
// @num_formats: count of video decoder formats
// @default_out_fmt: default output buffer format
// @default_cap_fmt: default capture buffer format
//
// @hw_arch: hardware arch is used to separate pure_sin_core and lat_sin_core
//
// @is_subdev_supported: whether support parent-node architecture(subdev)
// @uses_stateless_api: whether the decoder uses the stateless API with requests
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_dec_pdata {
    pub ctx): *mut *mut void (init_vdec_params)(struct mtk_vcodec_dec_ctx,
    pub ctx): *mut *mut int (ctrls_setup)(struct mtk_vcodec_dec_ctx,
    pub work): *mut *mut void (worker)(struct work_struct,
    pub ctx): *mut *mut int (flush_decoder)(struct mtk_vcodec_dec_ctx,
    pub ctx): *mut *mut *mut vdec_fb (get_cap_buffer)(mtk_vcodec_dec_ctx,
    pub src_buf_req): *mut media_request,
    pub vdec_vb2_ops: *const vb2_ops,
    pub vdec_formats: *const mtk_video_fmt,
    pub num_formats: *const c_int,
    pub default_out_fmt: *const mtk_video_fmt,
    pub default_cap_fmt: *const mtk_video_fmt,
    pub hw_arch: mtk_vdec_hw_arch,
    pub is_subdev_supported: bool,
    pub uses_stateless_api: bool,
}

//
// struct mtk_vcodec_dec_request - Media request private data.
// @refcount: Used to ensure we don't complete the request too soon
// @req: Media Request structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_dec_request {
    pub refcount: kref,
    pub req: media_request,
}

//
// struct mtk_vcodec_dec_ctx - Context (instance) private data.
//
// @type: type of decoder instance
// @dev: pointer to the mtk_vcodec_dec_dev of the device
// @list: link to ctx_list of mtk_vcodec_dec_dev
//
// @fh: struct v4l2_fh
// @m2m_ctx: pointer to the v4l2_m2m_ctx of the context
// @q_data: store information of input and output queue of the context
// @id: index of the context that this structure describes
// @state: state of the context
//
// @dec_if: hooked decoder driver interface
// @drv_handle: driver handle for specific decode/encode instance
//
// @picinfo: store picture info after header parsing
// @dpb_size: store dpb count after header parsing
//
// @int_cond: variable used by the waitqueue
// @int_type: type of the last interrupt
// @queue: waitqueue that can be used to wait for this context to finish
// @irq_status: irq status
//
// @ctrl_hdl: handler for v4l2 framework
// @decode_work: worker for the decoding
// @last_decoded_picinfo: pic information get from latest decode
// @empty_flush_buf: a fake size-0 capture buffer that indicates flush. Used
// for stateful decoder.
// @is_flushing: set to true if flushing is in progress.
//
// @current_codec: current set input codec, in V4L2 pixel format
// @capture_fourcc: capture queue type in V4L2 pixel format
//
// @colorspace: enum v4l2_colorspace; supplemental to pixelformat
// @ycbcr_enc: enum v4l2_ycbcr_encoding, Y'CbCr encoding
// @quantization: enum v4l2_quantization, colorspace quantization
// @xfer_func: enum v4l2_xfer_func, colorspace transfer function
//
// @decoded_frame_cnt: number of decoded frames
// @lock: protect variables accessed by V4L2 threads and worker thread such as
// mtk_video_dec_buf.
// @hw_id: hardware index used to identify different hardware.
//
// @msg_queue: msg queue used to store lat buffer information.
// @vpu_inst: vpu instance pointer.
//
// @is_10bit_bitstream: set to true if it's 10bit bitstream
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_dec_ctx {
    pub type: mtk_instance_type,
    pub dev: *mut mtk_vcodec_dec_dev,
    pub list: list_head,
    pub fh: v4l2_fh,
    pub m2m_ctx: *mut v4l2_m2m_ctx,
    pub q_data: [mtk_q_data; 2],
    pub id: c_int,
    pub state: mtk_instance_state,
    pub dec_if: *const vdec_common_if,
    pub drv_handle: *mut c_void,
    pub picinfo: vdec_pic_info,
    pub dpb_size: c_int,
    pub int_cond: [c_int; MTK_VDEC_HW_MAX],
    pub int_type: [c_int; MTK_VDEC_HW_MAX],
    pub queue: [wait_queue_head_t; MTK_VDEC_HW_MAX],
    pub irq_status: c_uint,
    pub ctrl_hdl: v4l2_ctrl_handler,
    pub decode_work: work_struct,
    pub last_decoded_picinfo: vdec_pic_info,
    pub empty_flush_buf: v4l2_m2m_buffer,
    pub is_flushing: bool,
    pub current_codec: u32,
    pub capture_fourcc: u32,
    pub colorspace: v4l2_colorspace,
    pub ycbcr_enc: v4l2_ycbcr_encoding,
    pub quantization: v4l2_quantization,
    pub xfer_func: v4l2_xfer_func,
    pub decoded_frame_cnt: c_int,
    pub lock: mutex,
    pub hw_id: c_int,
    pub msg_queue: vdec_msg_queue,
    pub vpu_inst: *mut c_void,
    pub is_10bit_bitstream: bool,
}

//
// struct mtk_vcodec_dec_dev - driver data
// @v4l2_dev: V4L2 device to register video devices for.
// @vfd_dec: Video device for decoder
// @mdev_dec: Media device for decoder
//
// @m2m_dev_dec: m2m device for decoder
// @plat_dev: platform device
// @ctx_list: list of struct mtk_vcodec_ctx
// @curr_ctx: The context that is waiting for codec hardware
//
// @reg_base: Mapped address of MTK Vcodec registers.
// @vdec_pdata: decoder IC-specific data
// @vdecsys_regmap: VDEC_SYS register space passed through syscon
//
// @fw_handler: used to communicate with the firmware.
// @id_counter: used to identify current opened instance
//
// @dec_mutex: decoder hardware lock
// @dev_mutex: video_device lock
// @dev_ctx_lock: the lock of context list
// @decode_workqueue: decode work queue
//
// @irqlock: protect data access by irq handler and work thread
// @dec_irq: decoder irq resource
//
// @pm: power management control
// @dec_capability: used to identify decode capability, ex: 4k
//
// @core_workqueue: queue used for core hardware decode
//
// @subdev_dev: subdev hardware device
// @subdev_prob_done: check whether all used hw device is prob done
// @subdev_bitmap: used to record hardware is ready or not
//
// @dec_active_cnt: used to mark whether need to record register value
// @vdec_racing_info: record register value
// @dec_racing_info_mutex: mutex lock used for inner racing mode
// @dbgfs: debug log related information
//
// @chip_name: used to distinguish platforms and select the correct codec configuration values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_dec_dev {
    pub v4l2_dev: v4l2_device,
    pub vfd_dec: *mut video_device,
    pub mdev_dec: media_device,
    pub m2m_dev_dec: *mut v4l2_m2m_dev,
    pub plat_dev: *mut platform_device,
    pub ctx_list: list_head,
    pub curr_ctx: *mut mtk_vcodec_dec_ctx,
    pub reg_base: [*mut void __iomem; NUM_MAX_VCODEC_REG_BASE],
    pub vdec_pdata: *const mtk_vcodec_dec_pdata,
    pub vdecsys_regmap: *mut regmap,
    pub fw_handler: *mut mtk_vcodec_fw,
    pub id_counter: u64,
// decoder hardware mutex lock
    pub dec_mutex: [mutex; MTK_VDEC_HW_MAX],
    pub dev_mutex: mutex,
    pub dev_ctx_lock: spinlock_t,
    pub decode_workqueue: *mut workqueue_struct,
    pub irqlock: spinlock_t,
    pub dec_irq: c_int,
    pub pm: mtk_vcodec_pm,
    pub dec_capability: c_uint,
    pub core_workqueue: *mut workqueue_struct,
    pub subdev_dev: [*mut c_void; MTK_VDEC_HW_MAX],
    pub vdec_dev): *mut *mut int (subdev_prob_done)(struct mtk_vcodec_dec_dev,
    pub MTK_VDEC_HW_MAX): DECLARE_BITMAP(subdev_bitmap,,
    pub dec_active_cnt: core::sync::atomic::AtomicI32,
    pub vdec_racing_info: [u32; 132],
// Protects access to vdec_racing_info data
    pub dec_racing_info_mutex: mutex,
    pub dbgfs: mtk_vcodec_dbgfs,
    pub chip_name: mtk_vcodec_dec_chip_name,
}

extern "C" {
    pub fn container_of(_arg: fh, mtk_vcodec_dec_ctx: struct, _arg: fh) -> return;
}
extern "C" {
    pub fn fh_to_dec_ctx(_arg: file_to_v4l2_fh(filp)) -> return;
}
extern "C" {
    pub fn container_of(_arg: ctrl->handler, mtk_vcodec_dec_ctx: struct, _arg: ctrl_hdl) -> return;
}
extern "C" {
    pub fn container_of(_arg: req, mtk_vcodec_dec_request: struct, _arg: req) -> return;
}
// Wake up context wait_queue

