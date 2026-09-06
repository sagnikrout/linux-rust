//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/verisilicon/hantro.h
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
// Hantro VPU codec driver
//
// Copyright 2018 Google LLC.
// Tomasz Figa <tfiga@chromium.org>
//
// Based on s5p-mfc driver by Samsung Electronics Co., Ltd.
// Copyright (C) 2011 Samsung Electronics Co., Ltd.
//

pub const HANTRO_ENCODERS: c_uint = 0x0000ffff;

pub const HANTRO_DECODERS: c_uint = 0xffff0000;
//
// struct hantro_irq - irq handler and name
//
// @name:			irq name for device tree lookup
// @handler:			interrupt handler
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_irq {
    pub name: *const c_char,
    pub priv): *mut *mut irqreturn_t (handler)(int irq, void,
}

//
// struct hantro_variant - information about VPU hardware variant
//
// @enc_offset:			Offset from VPU base to encoder registers.
// @dec_offset:			Offset from VPU base to decoder registers.
// @enc_fmts:			Encoder formats.
// @num_enc_fmts:		Number of encoder formats.
// @dec_fmts:			Decoder formats.
// @num_dec_fmts:		Number of decoder formats.
// @postproc_fmts:		Post-processor formats.
// @num_postproc_fmts:		Number of post-processor formats.
// @postproc_ops:		Post-processor ops.
// @codec:			Supported codecs
// @codec_ops:			Codec ops.
// @init:			Initialize hardware, optional.
// @runtime_resume:		reenable hardware after power gating, optional.
// @irqs:			array of irq names and interrupt handlers
// @num_irqs:			number of irqs in the array
// @clk_names:			array of clock names
// @num_clocks:			number of clocks in the array
// @reg_names:			array of register range names
// @num_regs:			number of register range names in the array
// @double_buffer:		core needs double buffering
// @legacy_regs:		core uses legacy register set
// @late_postproc:		postproc must be set up at the end of the job
// @shared_devices:		an array of device ids that cannot run concurrently
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_variant {
    pub enc_offset: c_uint,
    pub dec_offset: c_uint,
    pub enc_fmts: *const hantro_fmt,
    pub num_enc_fmts: c_uint,
    pub dec_fmts: *const hantro_fmt,
    pub num_dec_fmts: c_uint,
    pub postproc_fmts: *const hantro_fmt,
    pub num_postproc_fmts: c_uint,
    pub postproc_ops: *const hantro_postproc_ops,
    pub codec: c_uint,
    pub codec_ops: *const hantro_codec_ops,
    pub vpu): *mut *mut int (init)(struct hantro_dev,
    pub vpu): *mut *mut int (runtime_resume)(struct hantro_dev,
    pub irqs: *const hantro_irq,
    pub num_irqs: c_int,
    pub clk_names: *const *const c_char,
    pub num_clocks: c_int,
    pub reg_names: *const *const c_char,
    pub num_regs: c_int,
    pub 1: unsigned int double_buffer :,
    pub 1: unsigned int legacy_regs :,
    pub 1: unsigned int late_postproc :,
    pub shared_devices: *const of_device_id,
}

//
// enum hantro_codec_mode - codec operating mode.
// @HANTRO_MODE_NONE:  No operating mode. Used for RAW video formats.
// @HANTRO_MODE_JPEG_ENC: JPEG encoder.
// @HANTRO_MODE_H264_DEC: H264 decoder.
// @HANTRO_MODE_MPEG2_DEC: MPEG-2 decoder.
// @HANTRO_MODE_VP8_DEC: VP8 decoder.
// @HANTRO_MODE_HEVC_DEC: HEVC decoder.
// @HANTRO_MODE_VP9_DEC: VP9 decoder.
// @HANTRO_MODE_AV1_DEC: AV1 decoder
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hantro_codec_mode {
    HANTRO_MODE_NONE = -1,
    HANTRO_MODE_JPEG_ENC,
    HANTRO_MODE_H264_DEC,
    HANTRO_MODE_MPEG2_DEC,
    HANTRO_MODE_VP8_DEC,
    HANTRO_MODE_HEVC_DEC,
    HANTRO_MODE_VP9_DEC,
    HANTRO_MODE_AV1_DEC,
}

//
// struct hantro_ctrl - helper type to declare supported controls
// @codec:	codec id this control belong to (HANTRO_JPEG_ENCODER, etc.)
// @cfg:	control configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_ctrl {
    pub codec: c_uint,
    pub cfg: v4l2_ctrl_config,
}

//
// struct hantro_func - Hantro VPU functionality
//
// @id:			processing functionality ID (can be
// %MEDIA_ENT_F_PROC_VIDEO_ENCODER or
// %MEDIA_ENT_F_PROC_VIDEO_DECODER)
// @vdev:		&struct video_device that exposes the encoder or
// decoder functionality
// @source_pad:		&struct media_pad with the source pad.
// @sink:		&struct media_entity pointer with the sink entity
// @sink_pad:		&struct media_pad with the sink pad.
// @proc:		&struct media_entity pointer with the M2M device itself.
// @proc_pads:		&struct media_pad with the @proc pads.
// @intf_devnode:	&struct media_intf devnode pointer with the interface
// with controls the M2M device.
//
// Contains everything needed to attach the video device to the media device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_func {
    pub id: c_uint,
    pub vdev: video_device,
    pub source_pad: media_pad,
    pub sink: media_entity,
    pub sink_pad: media_pad,
    pub proc: media_entity,
    pub proc_pads: [media_pad; 2],
    pub intf_devnode: *mut media_intf_devnode,
}

extern "C" {
    pub fn container_of(_arg: vdev, hantro_func: struct, _arg: vdev) -> return;
}
//
// struct hantro_dev - driver data
// @v4l2_dev:		V4L2 device to register video devices for.
// @m2m_dev:		mem2mem device associated to this device.
// @mdev:		media device associated to this device.
// @encoder:		encoder functionality.
// @decoder:		decoder functionality.
// @pdev:		Pointer to VPU platform device.
// @dev:		Pointer to device for convenient logging using
// dev_ macros.
// @clocks:		Array of clock handles.
// @resets:		Array of reset handles.
// @reg_bases:		Mapped addresses of VPU registers.
// @enc_base:		Mapped address of VPU encoder register for convenience.
// @dec_base:		Mapped address of VPU decoder register for convenience.
// @ctrl_base:		Mapped address of VPU control block.
// @vpu_mutex:		Mutex to synchronize V4L2 calls.
// @irqlock:		Spinlock to synchronize access to data structures
// shared with interrupt handlers.
// @variant:		Hardware variant-specific parameters.
// @watchdog_work:	Delayed work for hardware timeout handling.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_dev {
    pub v4l2_dev: v4l2_device,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub mdev: media_device,
    pub encoder: *mut hantro_func,
    pub decoder: *mut hantro_func,
    pub pdev: *mut platform_device,
    pub dev: *mut device,
    pub clocks: *mut clk_bulk_data,
    pub resets: *mut reset_control,
    pub reg_bases: *mut void __iomem,
    pub enc_base: *mut void __iomem,
    pub dec_base: *mut void __iomem,
    pub ctrl_base: *mut void __iomem,
    pub /: *mut *mut mutex vpu_mutex; / video_device lock,
    pub irqlock: spinlock_t,
    pub variant: *const hantro_variant,
    pub watchdog_work: delayed_work,
}

//
// struct hantro_ctx - Context (instance) private data.
//
// @dev:		VPU driver data to which the context belongs.
// @fh:			V4L2 file handler.
// @is_encoder:		Decoder or encoder context?
//
// @sequence_cap:       Sequence counter for capture queue
// @sequence_out:       Sequence counter for output queue
//
// @vpu_src_fmt:	Descriptor of active source format.
// @src_fmt:		V4L2 pixel format of active source format.
// @vpu_dst_fmt:	Descriptor of active destination format.
// @dst_fmt:		V4L2 pixel format of active destination format.
// @ref_fmt:		V4L2 pixel format of the reference frames format.
//
// @ctrl_handler:	Control handler used to register controls.
// @jpeg_quality:	User-specified JPEG compression quality.
// @bit_depth:		Bit depth of current frame
// @need_postproc:	Set to true if the bitstream features require to
// use the post-processor.
//
// @codec_ops:		Set of operations related to codec mode.
// @postproc:		Post-processing context.
// @h264_dec:		H.264-decoding context.
// @mpeg2_dec:		MPEG-2-decoding context.
// @vp8_dec:		VP8-decoding context.
// @hevc_dec:		HEVC-decoding context.
// @vp9_dec:		VP9-decoding context.
// @av1_dec:		AV1-decoding context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_ctx {
    pub dev: *mut hantro_dev,
    pub fh: v4l2_fh,
    pub is_encoder: bool,
    pub sequence_cap: u32,
    pub sequence_out: u32,
    pub vpu_src_fmt: *const hantro_fmt,
    pub src_fmt: v4l2_pix_format_mplane,
    pub vpu_dst_fmt: *const hantro_fmt,
    pub dst_fmt: v4l2_pix_format_mplane,
    pub ref_fmt: v4l2_pix_format_mplane,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub jpeg_quality: c_int,
    pub bit_depth: c_int,
    pub codec_ops: *const hantro_codec_ops,
    pub postproc: hantro_postproc_ctx,
    pub need_postproc: bool,
// Specific for particular codec modes.
    pub h264_dec: hantro_h264_dec_hw_ctx,
    pub mpeg2_dec: hantro_mpeg2_dec_hw_ctx,
    pub vp8_dec: hantro_vp8_dec_hw_ctx,
    pub hevc_dec: hantro_hevc_dec_hw_ctx,
    pub vp9_dec: hantro_vp9_dec_hw_ctx,
    pub av1_dec: hantro_av1_dec_hw_ctx,
}

//
// struct hantro_fmt - information about supported video formats.
// @name:	Human readable name of the format.
// @fourcc:	FourCC code of the format. See V4L2_PIX_FMT_*.
// @codec_mode:	Codec mode related to this format. See
// enum hantro_codec_mode.
// @header_size: Optional header size. Currently used by JPEG encoder.
// @max_depth:	Maximum depth, for bitstream formats
// @enc_fmt:	Format identifier for encoder registers.
// @frmsize:	Supported range of frame sizes (only for bitstream formats).
// @postprocessed: Indicates if this format needs the post-processor.
// @match_depth: Indicates if format bit depth must match video bit depth
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_fmt {
    pub name: *mut c_char,
    pub fourcc: u32,
    pub codec_mode: hantro_codec_mode,
    pub header_size: c_int,
    pub max_depth: c_int,
    pub enc_fmt: hantro_enc_fmt,
    pub frmsize: v4l2_frmsize_stepwise,
    pub postprocessed: bool,
    pub match_depth: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_reg {
    pub base: u32,
    pub shift: u32,
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_postproc_regs {
    pub pipeline_en: hantro_reg,
    pub max_burst: hantro_reg,
    pub clk_gate: hantro_reg,
    pub out_swap32: hantro_reg,
    pub out_endian: hantro_reg,
    pub out_luma_base: hantro_reg,
    pub input_width: hantro_reg,
    pub input_height: hantro_reg,
    pub output_width: hantro_reg,
    pub output_height: hantro_reg,
    pub input_fmt: hantro_reg,
    pub output_fmt: hantro_reg,
    pub orig_width: hantro_reg,
    pub display_width: hantro_reg,
    pub input_width_ext: hantro_reg,
    pub input_height_ext: hantro_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_vp9_decoded_buffer_info {
// Info needed when the decoded frame serves as a reference frame.
    pub width: c_ushort,
    pub height: c_ushort,
    pub chroma_offset: usize,
    pub mv_offset: usize,
    pub 4: u32 bit_depth :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_av1_decoded_buffer_info {
// Info needed when the decoded frame serves as a reference frame.
    pub chroma_offset: usize,
    pub mv_offset: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_decoded_buffer {
// Must be the first field in this struct.
    pub base: v4l2_m2m_buffer,
    pub vp9: hantro_vp9_decoded_buffer_info,
    pub av1: hantro_av1_decoded_buffer_info,
}

// Logging helpers
//
// DOC: hantro_debug: Module parameter to control level of debugging messages.
//
// Level of debugging messages can be controlled by bits of
// module parameter called "debug". Meaning of particular
// bits is as follows:
//
// bit 0 - global information: mode, size, init, release
// bit 1 - each run start/result information
// bit 2 - contents of small controls from userspace
// bit 3 - contents of big controls from userspace
// bit 4 - detail fmt, ctrl, buffer q/dq information
// bit 5 - detail function enter/leave trace information
// bit 6 - register write/read information
//

// Structure access helpers.
extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), hantro_ctx: struct, _arg: fh) -> return;
}
// Register accessors.
extern "C" {
    pub fn hantro_get_ref(ctx: *mut hantro_ctx, ts: u64) -> dma_addr_t;
}
extern "C" {
    pub fn v4l2_m2m_next_src_buf(_arg: ctx->fh.m2m_ctx) -> return;
}
extern "C" {
    pub fn v4l2_m2m_next_dst_buf(_arg: ctx->fh.m2m_ctx) -> return;
}
extern "C" {
    pub fn hantro_postproc_get_dec_buf_addr(_arg: ctx, _arg: vb->index) -> return;
}
extern "C" {
    pub fn vb2_dma_contig_plane_dma_addr(_arg: vb, _arg: 0) -> return;
}
extern "C" {
    pub fn container_of(_arg: buf, hantro_decoded_buffer: struct, _arg: base.vb.vb2_buf) -> return;
}
extern "C" {
    pub fn hantro_postproc_disable(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn hantro_postproc_enable(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn hantro_postproc_init(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn hantro_postproc_free(ctx: *mut hantro_ctx);
}
