//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/nxp/imx8-isi/imx8-isi-core.h
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
// V4L2 Capture ISI subdev for i.MX8QXP/QM platform
//
// ISI is a Image Sensor Interface of i.MX8QXP/QM platform, which
// used to process image from camera sensor to memory or DC
// Copyright 2019-2020 NXP
//

// Pipeline pads
pub const MXC_ISI_PIPE_PAD_SINK: c_int = 0;
pub const MXC_ISI_PIPE_PAD_SOURCE: c_int = 1;
pub const MXC_ISI_PIPE_PADS_NUM: c_int = 2;

pub const MXC_MAX_PLANES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxc_isi_buf_id {
    MXC_ISI_BUF1 = 0x0,
    MXC_ISI_BUF2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxc_isi_encoding {
    MXC_ISI_ENC_RAW,
    MXC_ISI_ENC_RGB,
    MXC_ISI_ENC_YUV,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxc_isi_input_id {
// Inputs from the crossbar switch range from 0 to 15
    MXC_ISI_INPUT_MEM = 16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxc_isi_video_type {
    MXC_ISI_VIDEO_CAP = BIT(0),
    MXC_ISI_VIDEO_M2M_OUT = BIT(1),
    MXC_ISI_VIDEO_M2M_CAP = BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_isi_format_info {
    pub mbus_code: u32,
    pub fourcc: u32,
    pub type: mxc_isi_video_type,
    pub isi_in_format: u32,
    pub isi_out_format: u32,
    pub mem_planes: u8,
    pub color_planes: u8,
    pub depth: [u8; MXC_MAX_PLANES],
    pub hsub: u8,
    pub vsub: u8,
    pub encoding: mxc_isi_encoding,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_isi_bus_format_info {
    pub mbus_code: u32,
    pub output: u32,
    pub pads: u32,
    pub encoding: mxc_isi_encoding,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_isi_buffer {
    pub v4l2_buf: vb2_v4l2_buffer,
    pub list: list_head,
    pub dma_addrs: [dma_addr_t; 3],
    pub id: mxc_isi_buf_id,
    pub discard: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_isi_reg {
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_isi_ier_reg {
// Overflow Y/U/V trigger enable
    pub oflw_y_buf_en: mxc_isi_reg,
    pub oflw_u_buf_en: mxc_isi_reg,
    pub oflw_v_buf_en: mxc_isi_reg,
// Excess overflow Y/U/V trigger enable
    pub excs_oflw_y_buf_en: mxc_isi_reg,
    pub excs_oflw_u_buf_en: mxc_isi_reg,
    pub excs_oflw_v_buf_en: mxc_isi_reg,
// Panic Y/U/V trigger enable
    pub panic_y_buf_en: mxc_isi_reg,
    pub panic_v_buf_en: mxc_isi_reg,
    pub panic_u_buf_en: mxc_isi_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_isi_panic_thd {
    pub mask: u32,
    pub offset: u32,
    pub threshold: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_isi_set_thd {
    pub panic_set_thd_y: mxc_isi_panic_thd,
    pub panic_set_thd_u: mxc_isi_panic_thd,
    pub panic_set_thd_v: mxc_isi_panic_thd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_gasket_ops {
    pub port): c_uint,
    pub port): *const *const *const void (disable)(struct mxc_isi_dev isi, unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum model {
    MXC_ISI_IMX8MN,
    MXC_ISI_IMX8MP,
    MXC_ISI_IMX8QM,
    MXC_ISI_IMX8QXP,
    MXC_ISI_IMX8ULP,
    MXC_ISI_IMX91,
    MXC_ISI_IMX93,
    MXC_ISI_IMX95,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_isi_plat_data {
    pub model: model,
    pub num_ports: c_uint,
    pub num_channels: c_uint,
    pub /: *mut *mut unsigned int num_vc; / Number of VCs, 0 = no VC support,
    pub reg_offset: c_uint,
    pub ier_reg: *const mxc_isi_ier_reg,
    pub set_thd: *const mxc_isi_set_thd,
    pub gasket_ops: *const mxc_gasket_ops,
    pub buf_active_reverse: bool,
    pub has_36bit_dma: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_isi_dma_buffer {
    pub size: usize,
    pub addr: *mut c_void,
    pub dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_isi_input {
    pub enabled_streams: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_isi_crossbar {
    pub isi: *mut mxc_isi_dev,
    pub num_sinks: c_uint,
    pub num_sources: c_uint,
    pub inputs: *mut mxc_isi_input,
    pub sd: v4l2_subdev,
    pub pads: *mut media_pad,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_isi_video {
    pub pipe: *mut mxc_isi_pipe,
    pub vdev: video_device,
    pub pad: media_pad,
// Protects the vdev and vb2_q operations
    pub lock: mutex,
    pub pix: v4l2_pix_format_mplane,
    pub fmtinfo: *const mxc_isi_format_info,
    pub handler: v4l2_ctrl_handler,
    pub alpha: c_uint,
    pub hflip: bool,
    pub vflip: bool,
    pub ctrls: },
    pub vb2_q: vb2_queue,
    pub buf_discard: [mxc_isi_buffer; 3],
    pub out_pending: list_head,
    pub out_active: list_head,
    pub out_discard: list_head,
    pub frame_count: u32,
// Protects out_pending, out_active, out_discard and frame_count
    pub buf_lock: spinlock_t,
    pub discard_buffer: [mxc_isi_dma_buffer; MXC_MAX_PLANES],
}

extern "C" {
    pub fn void(: *mut *mut mxc_isi_pipe_irq_t)(struct mxc_isi_pipe, _arg: u32) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_isi_pipe {
    pub isi: *mut mxc_isi_dev,
    pub id: u32,
    pub regs: *mut void __iomem,
    pub pipe: media_pipeline,
    pub sd: v4l2_subdev,
    pub pads: [media_pad; MXC_ISI_PIPE_PADS_NUM],
    pub video: mxc_isi_video,
//
// Protects use_count, irq_handler, res_available, res_acquired,
// chained_res, and the CHNL_CTRL register.
//
    pub lock: mutex,
    pub use_count: c_uint,
    pub irq_handler: mxc_isi_pipe_irq_t,

    pub available_res: u8,
    pub acquired_res: u8,
    pub chained_res: u8,
    pub chained: bool,
    pub input: c_uint,
//
// Stream on the connected crossbar input, expressed as a bitmask. Zero
// when the pipeline is disabled, a single bit set when the pipeline is
// enabled (as each pipeline processes a single stream).
//
    pub input_stream: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_isi_m2m {
    pub isi: *mut mxc_isi_dev,
    pub pipe: *mut mxc_isi_pipe,
    pub pad: media_pad,
    pub vdev: video_device,
    pub intf: *mut media_intf_devnode,
    pub m2m_dev: *mut v4l2_m2m_dev,
// Protects last_ctx, usage_count and chained_count
    pub lock: mutex,
    pub last_ctx: *mut mxc_isi_m2m_ctx,
    pub usage_count: c_int,
    pub chained_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_isi_dev {
    pub dev: *mut device,
    pub pdata: *const mxc_isi_plat_data,
    pub regs: *mut void __iomem,
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub gasket: *mut regmap,
    pub crossbar: mxc_isi_crossbar,
    pub pipes: *mut mxc_isi_pipe,
    pub m2m: mxc_isi_m2m,
    pub media_dev: media_device,
    pub v4l2_dev: v4l2_device,
    pub notifier: v4l2_async_notifier,
    pub debugfs_root: *mut dentry,
}

extern "C" {
    pub fn mxc_isi_crossbar_init(isi: *mut mxc_isi_dev) -> c_int;
}
extern "C" {
    pub fn mxc_isi_crossbar_cleanup(xbar: *mut mxc_isi_crossbar);
}
extern "C" {
    pub fn mxc_isi_crossbar_register(xbar: *mut mxc_isi_crossbar) -> c_int;
}
extern "C" {
    pub fn mxc_isi_crossbar_unregister(xbar: *mut mxc_isi_crossbar);
}
extern "C" {
    pub fn mxc_isi_pipe_init(isi: *mut mxc_isi_dev, id: c_uint) -> c_int;
}
extern "C" {
    pub fn mxc_isi_pipe_cleanup(pipe: *mut mxc_isi_pipe);
}
extern "C" {
    pub fn mxc_isi_pipe_release(pipe: *mut mxc_isi_pipe);
}
extern "C" {
    pub fn mxc_isi_pipe_enable(pipe: *mut mxc_isi_pipe) -> c_int;
}
extern "C" {
    pub fn mxc_isi_pipe_disable(pipe: *mut mxc_isi_pipe);
}
extern "C" {
    pub fn mxc_isi_video_unregister(pipe: *mut mxc_isi_pipe);
}
extern "C" {
    pub fn mxc_isi_video_suspend(pipe: *mut mxc_isi_pipe);
}
extern "C" {
    pub fn mxc_isi_video_resume(pipe: *mut mxc_isi_pipe) -> c_int;
}

extern "C" {
    pub fn mxc_isi_m2m_register(isi: *mut mxc_isi_dev, v4l2_dev: *mut v4l2_device) -> c_int;
}
extern "C" {
    pub fn mxc_isi_m2m_unregister(isi: *mut mxc_isi_dev) -> c_int;
}
extern "C" {
    pub fn mxc_isi_m2m_suspend(m2m: *mut mxc_isi_m2m);
}
extern "C" {
    pub fn mxc_isi_m2m_resume(m2m: *mut mxc_isi_m2m) -> c_int;
}

extern "C" {
    pub fn mxc_isi_channel_release(pipe: *mut mxc_isi_pipe);
}
extern "C" {
    pub fn mxc_isi_channel_get(pipe: *mut mxc_isi_pipe);
}
extern "C" {
    pub fn mxc_isi_channel_put(pipe: *mut mxc_isi_pipe);
}
extern "C" {
    pub fn mxc_isi_channel_enable(pipe: *mut mxc_isi_pipe);
}
extern "C" {
    pub fn mxc_isi_channel_disable(pipe: *mut mxc_isi_pipe);
}
extern "C" {
    pub fn mxc_isi_channel_chain(pipe: *mut mxc_isi_pipe) -> c_int;
}
extern "C" {
    pub fn mxc_isi_channel_unchain(pipe: *mut mxc_isi_pipe);
}
extern "C" {
    pub fn mxc_isi_channel_m2m_start(pipe: *mut mxc_isi_pipe);
}
extern "C" {
    pub fn mxc_isi_channel_set_alpha(pipe: *mut mxc_isi_pipe, alpha: u8);
}
extern "C" {
    pub fn mxc_isi_channel_set_flip(pipe: *mut mxc_isi_pipe, hflip: bool, vflip: bool);
}
extern "C" {
    pub fn mxc_isi_channel_set_inbuf(pipe: *mut mxc_isi_pipe, dma_addr: dma_addr_t);
}
extern "C" {
    pub fn mxc_isi_channel_irq_status(pipe: *mut mxc_isi_pipe, clear: bool) -> u32;
}
extern "C" {
    pub fn mxc_isi_channel_irq_clear(pipe: *mut mxc_isi_pipe);
}

extern "C" {
    pub fn mxc_isi_debug_init(isi: *mut mxc_isi_dev);
}
extern "C" {
    pub fn mxc_isi_debug_cleanup(isi: *mut mxc_isi_dev);
}

//
// ISI scaling engine works in two parts: it performs pre-decimation of
// the image followed by bilinear filtering to achieve the desired
// downscaling factor.
//
// The decimation filter provides a maximum downscaling factor of 8, and
// the subsequent bilinear filter provides a maximum downscaling factor
// of 2. Combined, the maximum scaling factor can be up to 16.
//
extern "C" {
    pub fn clamp(_arg: val, _arg: max(1U, _arg: DIV_ROUND_UP(max_val, _arg: 16)), _arg: max_val) -> return;
}
