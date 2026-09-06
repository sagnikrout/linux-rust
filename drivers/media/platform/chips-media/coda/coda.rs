//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/chips-media/coda/coda.h
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
// Coda multi-standard codec IP
//
// Copyright (C) 2012 Vista Silicon S.L.
// Javier Martin, <javier.martin@vista-silicon.com>
// Xavier Duret
// Copyright (C) 2012-2014 Philipp Zabel, Pengutronix
//

pub const CODA_MAX_FRAMEBUFFERS: c_int = 19;

//
// This control allows applications to read the per-stream
// (i.e. per-context) Macroblocks Error Count. This value
// is CODA specific.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coda_inst_type {
    CODA_INST_ENCODER,
    CODA_INST_DECODER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coda_product {
    CODA_DX6 = 0xf001,
    CODA_HX4 = 0xf00a,
    CODA_7541 = 0xf012,
    CODA_960 = 0xf020,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_devtype {
    pub firmware: [*mut c_char; 3],
    pub product: coda_product,
    pub codecs: *const coda_codec,
    pub num_codecs: c_uint,
    pub vdevs: *const coda_video_device,
    pub num_vdevs: c_uint,
    pub workbuf_size: usize,
    pub tempbuf_size: usize,
    pub iram_size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_aux_buf {
    pub vaddr: *mut c_void,
    pub paddr: dma_addr_t,
    pub size: u32,
    pub blob: debugfs_blob_wrapper,
    pub dentry: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_dev {
    pub v4l2_dev: v4l2_device,
    pub vfd: [video_device; 6],
    pub dev: *mut device,
    pub devtype: *const coda_devtype,
    pub firmware: c_int,
    pub vdoa: *mut vdoa_data,
    pub regs_base: *mut void __iomem,
    pub clk_per: *mut clk,
    pub clk_ahb: *mut clk,
    pub rstc: *mut reset_control,
    pub codebuf: coda_aux_buf,
    pub tempbuf: coda_aux_buf,
    pub workbuf: coda_aux_buf,
    pub iram_pool: *mut gen_pool,
    pub iram: coda_aux_buf,
    pub dev_mutex: mutex,
    pub coda_mutex: mutex,
    pub workqueue: *mut workqueue_struct,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub ida: ida,
    pub debugfs_root: *mut dentry,
    pub mb_err_rs: ratelimit_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_codec {
    pub mode: u32,
    pub src_fourcc: u32,
    pub dst_fourcc: u32,
    pub max_w: u32,
    pub max_h: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_params {
    pub rot_mode: u8,
    pub h264_intra_qp: u8,
    pub h264_inter_qp: u8,
    pub h264_min_qp: u8,
    pub h264_max_qp: u8,
    pub h264_disable_deblocking_filter_idc: u8,
    pub h264_slice_alpha_c0_offset_div2: i8,
    pub h264_slice_beta_offset_div2: i8,
    pub h264_constrained_intra_pred_flag: bool,
    pub h264_chroma_qp_index_offset: i8,
    pub h264_profile_idc: u8,
    pub h264_level_idc: u8,
    pub mpeg2_profile_idc: u8,
    pub mpeg2_level_idc: u8,
    pub mpeg4_intra_qp: u8,
    pub mpeg4_inter_qp: u8,
    pub gop_size: u8,
    pub intra_refresh: c_int,
    pub jpeg_chroma_subsampling: v4l2_jpeg_chroma_subsampling,
    pub jpeg_quality: u8,
    pub jpeg_restart_interval: u8,
    pub jpeg_qmat_tab: [*mut u8; 3],
    pub jpeg_qmat_index: [c_int; 3],
    pub jpeg_huff_dc_index: [c_int; 3],
    pub jpeg_huff_ac_index: [c_int; 3],
    pub jpeg_huff_data: *mut u32,
    pub jpeg_huff_tab: *mut coda_huff_tab,
    pub codec_mode: c_int,
    pub codec_mode_aux: c_int,
    pub slice_mode: v4l2_mpeg_video_multi_slice_mode,
    pub framerate: u32,
    pub bitrate: u16,
    pub vbv_delay: u16,
    pub vbv_size: u32,
    pub slice_max_bits: u32,
    pub slice_max_mb: u32,
    pub force_ipicture: bool,
    pub gop_size_changed: bool,
    pub bitrate_changed: bool,
    pub framerate_changed: bool,
    pub h264_intra_qp_changed: bool,
    pub intra_refresh_changed: bool,
    pub slice_mode_changed: bool,
    pub frame_rc_enable: bool,
    pub mb_rc_enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_buffer_meta {
    pub list: list_head,
    pub sequence: u32,
    pub timecode: v4l2_timecode,
    pub timestamp: u64,
    pub start: c_uint,
    pub end: c_uint,
    pub last: bool,
}

// Per-queue, driver-specific private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_q_data {
    pub width: c_uint,
    pub height: c_uint,
    pub bytesperline: c_uint,
    pub sizeimage: c_uint,
    pub fourcc: c_uint,
    pub rect: v4l2_rect,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_iram_info {
    pub axi_sram_use: u32,
    pub buf_bit_use: phys_addr_t,
    pub buf_ip_ac_dc_use: phys_addr_t,
    pub buf_dbk_y_use: phys_addr_t,
    pub buf_dbk_c_use: phys_addr_t,
    pub buf_ovl_use: phys_addr_t,
    pub buf_btp_use: phys_addr_t,
    pub search_ram_paddr: phys_addr_t,
    pub search_ram_size: c_int,
    pub remaining: c_int,
    pub next_paddr: phys_addr_t,
}

pub const GDI_LINEAR_FRAME_MAP: c_int = 0;
pub const GDI_TILED_FRAME_MB_RASTER_MAP: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_context_ops {
    pub dst_vq): *mut vb2_queue,
    pub rb): *mut *mut *mut int (reqbufs)(struct coda_ctx ctx, struct v4l2_requestbuffers,
    pub ctx): *mut *mut int (start_streaming)(struct coda_ctx,
    pub ctx): *mut *mut int (prepare_run)(struct coda_ctx,
    pub ctx): *mut *mut void (finish_run)(struct coda_ctx,
    pub ctx): *mut *mut void (run_timeout)(struct coda_ctx,
    pub work): *mut *mut void (seq_init_work)(struct work_struct,
    pub work): *mut *mut void (seq_end_work)(struct work_struct,
    pub ctx): *mut *mut void (release)(struct coda_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_internal_frame {
    pub buf: coda_aux_buf,
    pub meta: coda_buffer_meta,
    pub type: u32,
    pub error: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_ctx {
    pub dev: *mut coda_dev,
    pub buffer_mutex: mutex,
    pub pic_run_work: work_struct,
    pub seq_init_work: work_struct,
    pub seq_end_work: work_struct,
    pub completion: completion,
    pub cvd: *const coda_video_device,
    pub ops: *const coda_context_ops,
    pub aborting: c_int,
    pub initialized: c_int,
    pub streamon_out: c_int,
    pub streamon_cap: c_int,
    pub qsequence: u32,
    pub osequence: u32,
    pub sequence_offset: u32,
    pub q_data: [coda_q_data; 2],
    pub inst_type: coda_inst_type,
    pub codec: *const coda_codec,
    pub colorspace: v4l2_colorspace,
    pub xfer_func: v4l2_xfer_func,
    pub ycbcr_enc: v4l2_ycbcr_encoding,
    pub quantization: v4l2_quantization,
    pub params: coda_params,
    pub ctrls: v4l2_ctrl_handler,
    pub h264_profile_ctrl: *mut v4l2_ctrl,
    pub h264_level_ctrl: *mut v4l2_ctrl,
    pub mpeg2_profile_ctrl: *mut v4l2_ctrl,
    pub mpeg2_level_ctrl: *mut v4l2_ctrl,
    pub mpeg4_profile_ctrl: *mut v4l2_ctrl,
    pub mpeg4_level_ctrl: *mut v4l2_ctrl,
    pub mb_err_cnt_ctrl: *mut v4l2_ctrl,
    pub fh: v4l2_fh,
    pub gopcounter: c_int,
    pub runcounter: c_int,
    pub jpeg_ecs_offset: c_int,
    pub vpu_header: [c_char; 3][64],
    pub vpu_header_size: [c_int; 3],
    pub bitstream_fifo: kfifo,
    pub bitstream_mutex: mutex,
    pub bitstream: coda_aux_buf,
    pub hold: bool,
    pub parabuf: coda_aux_buf,
    pub psbuf: coda_aux_buf,
    pub slicebuf: coda_aux_buf,
    pub internal_frames: [coda_internal_frame; CODA_MAX_FRAMEBUFFERS],
    pub buffer_meta_list: list_head,
    pub buffer_meta_lock: spinlock_t,
    pub num_metas: c_int,
    pub first_frame_sequence: c_uint,
    pub workbuf: coda_aux_buf,
    pub num_internal_frames: c_int,
    pub idx: c_int,
    pub reg_idx: c_int,
    pub iram_info: coda_iram_info,
    pub tiled_map_type: c_int,
    pub bit_stream_param: u32,
    pub frm_dis_flg: u32,
    pub frame_mem_ctrl: u32,
    pub para_change: u32,
    pub display_idx: c_int,
    pub debugfs_entry: *mut dentry,
    pub use_bit: bool,
    pub use_vdoa: bool,
    pub vdoa: *mut vdoa_ctx,
//
// wakeup mutex used to serialize encoder stop command and finish_run,
// ensures that finish_run always either flags the last returned buffer
// or wakes up the capture queue to signal EOS afterwards.
//
    pub wakeup_mutex: mutex,
}

extern "C" {
    pub fn coda_write(dev: *mut coda_dev, data: u32, reg: u32);
}
extern "C" {
    pub fn coda_read(dev: *mut coda_dev, reg: u32) -> c_uint;
}
extern "C" {
    pub fn coda_free_aux_buf(dev: *mut coda_dev, buf: *mut coda_aux_buf);
}
extern "C" {
    pub fn coda_hw_reset(ctx: *mut coda_ctx) -> c_int;
}
extern "C" {
    pub fn coda_fill_bitstream(ctx: *mut coda_ctx, buffer_list: *mut list_head);
}
extern "C" {
    pub fn coda_set_gdi_regs(ctx: *mut coda_ctx);
}
extern "C" {
    pub fn coda_check_firmware(dev: *mut coda_dev) -> c_int;
}
extern "C" {
    pub fn kfifo_len(_arg: &ctx->bitstream_fifo) -> return;
}
//
// The bitstream prefetcher needs to read at least 2 256 byte periods past
// the desired bitstream position for all data to reach the decoder.
//
extern "C" {
    pub fn coda_bitstream_can_fetch_past(ctx: *mut coda_ctx, pos: c_uint) -> bool;
}
extern "C" {
    pub fn coda_bitstream_flush(ctx: *mut coda_ctx) -> c_int;
}
extern "C" {
    pub fn coda_bit_stream_end_flag(ctx: *mut coda_ctx);
}
extern "C" {
    pub fn coda_h264_filler_nal(size: c_int, p: *mut c_char) -> c_int;
}
extern "C" {
    pub fn coda_h264_padding(size: c_int, p: *mut c_char) -> c_int;
}
extern "C" {
    pub fn coda_h264_profile(profile_idc: c_int) -> c_int;
}
extern "C" {
    pub fn coda_h264_level(level_idc: c_int) -> c_int;
}
extern "C" {
    pub fn coda_sps_parse_profile(ctx: *mut coda_ctx, vb: *mut vb2_buffer) -> c_int;
}
extern "C" {
    pub fn coda_mpeg2_profile(profile_idc: c_int) -> c_int;
}
extern "C" {
    pub fn coda_mpeg2_level(level_idc: c_int) -> c_int;
}
extern "C" {
    pub fn coda_mpeg2_parse_headers(ctx: *mut coda_ctx, buf: *mut u8, size: u32) -> u32;
}
extern "C" {
    pub fn coda_mpeg4_profile(profile_idc: c_int) -> c_int;
}
extern "C" {
    pub fn coda_mpeg4_level(level_idc: c_int) -> c_int;
}
extern "C" {
    pub fn coda_mpeg4_parse_headers(ctx: *mut coda_ctx, buf: *mut u8, size: u32) -> u32;
}
extern "C" {
    pub fn coda_jpeg_check_buffer(ctx: *mut coda_ctx, vb: *mut vb2_buffer) -> bool;
}
extern "C" {
    pub fn coda_jpeg_decode_header(ctx: *mut coda_ctx, vb: *mut vb2_buffer) -> c_int;
}
extern "C" {
    pub fn coda_jpeg_write_tables(ctx: *mut coda_ctx) -> c_int;
}
extern "C" {
    pub fn coda_set_jpeg_compression_quality(ctx: *mut coda_ctx, quality: c_int);
}
extern "C" {
    pub fn coda_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn coda9_jpeg_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
