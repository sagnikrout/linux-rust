//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/microchip/microchip-isc.h
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
// Microchip Image Sensor Controller (ISC) driver header file
//
// Copyright (C) 2016-2019 Microchip Technology, Inc.
//
// Author: Songjun Wu
// Author: Eugen Hristev <eugen.hristev@microchip.com>
//

pub const ISC_CLK_MAX_DIV: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isc_clk_id {
    ISC_ISPCK = 0,
    ISC_MCK = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isc_clk {
    pub hw: clk_hw,
    pub clk: *mut clk,
    pub regmap: *mut regmap,
    pub /: *mut *mut spinlock_t lock; / serialize access to clock registers,
    pub id: u8,
    pub parent_id: u8,
    pub div: u32,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isc_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isc_subdev_entity {
    pub sd: *mut v4l2_subdev,
    pub asd: *mut v4l2_async_connection,
    pub epn: *mut device_node,
    pub notifier: v4l2_async_notifier,
    pub pfe_cfg0: u32,
    pub list: list_head,
}

//
// struct isc_format - ISC media bus format information
// @fourcc:		Fourcc code for this format
// @mbus_code:		V4L2 media bus format code.
// @cfa_baycfg:		If this format is RAW BAYER, indicate the type of bayer.
// @pfe_cfg0_bps:	Number of hardware data lines connected to the ISC
// @raw:		If the format is raw bayer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isc_format {
    pub fourcc: u32,
    pub mbus_code: u32,
    pub cfa_baycfg: u32,
    pub pfe_cfg0_bps: u32,
    pub raw: bool,
}

// Pipeline bitmap

//
// struct fmt_config - ISC format configuration and internal pipeline
// @sd_format:		Pointer to an isc_format struct that holds the sensor
// @fourcc:		Fourcc code for this format.
// @bpp:		Bytes per pixel in the current format.
// @bpp_v4l2:		Bytes per pixel in the current format, for v4l2.
// @rlp_cfg_mode:	Configuration of the RLP (rounding, limiting packaging)
// @dcfg_imode:		Configuration of the input of the DMA module
// @dctrl_dview:	Configuration of the output of the DMA module
// @bits_pipeline:	Configuration of the pipeline, which modules are enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fmt_config {
    pub sd_format: *mut isc_format,
    pub fourcc: u32,
    pub bpp: u8,
    pub bpp_v4l2: u8,
    pub rlp_cfg_mode: u32,
    pub dcfg_imode: u32,
    pub dctrl_dview: u32,
    pub bits_pipeline: u32,
}

pub const HIST_ENTRIES: c_int = 512;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isc_ctrls {
    pub handler: v4l2_ctrl_handler,
    pub brightness: u32,
    pub contrast: u32,
    pub gamma_index: u8,
pub const ISC_WB_NONE: c_int = 0;
pub const ISC_WB_AUTO: c_int = 1;
pub const ISC_WB_ONETIME: c_int = 2;
    pub awb: u8,
// one for each component : GR, R, GB, B
    pub gain: [u32; HIST_BAYER],
    pub offset: [i32; HIST_BAYER],
    pub hist_entry: [u32; HIST_ENTRIES],
    pub hist_count: [u32; HIST_BAYER],
    pub hist_id: u8,
    pub hist_stat: u8,
pub const HIST_MIN_INDEX: c_int = 0;
pub const HIST_MAX_INDEX: c_int = 1;
    pub hist_minmax: [u32; HIST_BAYER][2],
}

pub const ISC_PIPE_LINE_NODE_NUM: c_int = 15;
//
// struct isc_reg_offsets - ISC device register offsets
// @csc:		Offset for the CSC register
// @cbc:		Offset for the CBC register
// @sub422:		Offset for the SUB422 register
// @sub420:		Offset for the SUB420 register
// @rlp:		Offset for the RLP register
// @his:		Offset for the HIS related registers
// @dma:		Offset for the DMA related registers
// @version:		Offset for the version register
// @his_entry:		Offset for the HIS entries registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isc_reg_offsets {
    pub csc: u32,
    pub cbc: u32,
    pub sub422: u32,
    pub sub420: u32,
    pub rlp: u32,
    pub his: u32,
    pub dma: u32,
    pub version: u32,
    pub his_entry: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isc_mc_pads {
    ISC_PAD_SINK	= 0,
    ISC_PADS_NUM	= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isc_scaler_pads {
    ISC_SCALER_PAD_SINK	= 0,
    ISC_SCALER_PAD_SOURCE	= 1,
    ISC_SCALER_PADS_NUM	= 2,
}

//
// struct isc_device - ISC device driver data/config struct
// @regmap:		Register map
// @hclock:		Hclock clock input (refer datasheet)
// @ispck:		iscpck clock (refer datasheet)
// @isc_clks:		ISC clocks
// @ispck_required:	ISC requires ISP Clock initialization
// @dcfg:		DMA master configuration, architecture dependent
//
// @dev:		Registered device driver
// @v4l2_dev:		v4l2 registered device
// @video_dev:		registered video device
//
// @vb2_vidq:		video buffer 2 video queue
// @dma_queue_lock:	lock to serialize the dma buffer queue
// @dma_queue:		the queue for dma buffers
// @cur_frm:		current isc frame/buffer
// @sequence:		current frame number
// @stop:		true if isc is not streaming, false if streaming
// @comp:		completion reference that signals frame completion
//
// @fmt:		current v42l format
// @try_fmt:		current v4l2 try format
//
// @config:		current ISC format configuration
// @try_config:		the current ISC try format , not yet activated
//
// @ctrls:		holds information about ISC controls
// @do_wb_ctrl:		control regarding the DO_WHITE_BALANCE button
// @awb_work:		workqueue reference for autowhitebalance histogram
// analysis
//
// @lock:		lock for serializing userspace file operations
// with ISC operations
// @awb_mutex:		serialize access to streaming status from awb work queue
// @awb_lock:		lock for serializing awb work queue operations
// with DMA/buffer operations
//
// @pipeline:		configuration of the ISC pipeline
//
// @current_subdev:	current subdevice: the sensor
// @subdev_entities:	list of subdevice entitites
//
// @gamma_table:	pointer to the table with gamma values, has
// gamma_max sets of GAMMA_ENTRIES entries each
// @gamma_max:		maximum number of sets of inside the gamma_table
//
// @max_width:		maximum frame width, dependent on the internal RAM
// @max_height:		maximum frame height, dependent on the internal RAM
//
// @config_dpc:		pointer to a function that initializes product
// specific DPC module
// @config_csc:		pointer to a function that initializes product
// specific CSC module
// @config_cbc:		pointer to a function that initializes product
// specific CBC module
// @config_cc:		pointer to a function that initializes product
// specific CC module
// @config_gam:		pointer to a function that initializes product
// specific GAMMA module
// @config_rlp:		pointer to a function that initializes product
// specific RLP module
// @config_ctrls:	pointer to a functoin that initializes product
// specific v4l2 controls.
//
// @adapt_pipeline:	pointer to a function that adapts the pipeline bits
// to the product specific pipeline
//
// @offsets:		struct holding the product specific register offsets
// @controller_formats:	pointer to the array of possible formats that the
// controller can output
// @formats_list:	pointer to the array of possible formats that can
// be used as an input to the controller
// @controller_formats_size:	size of controller_formats array
// @formats_list_size:	size of formats_list array
// @pads:		media controller pads for isc video entity
// @mdev:		media device that is registered by the isc
// @mpipe:		media device pipeline used by the isc
// @remote_pad:		remote pad on the connected subdevice
// @scaler_sd:		subdevice for the scaler that isc registers
// @scaler_pads:	media controller pads for the scaler subdevice
// @scaler_format:	current format for the scaler subdevice
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isc_device {
    pub regmap: *mut regmap,
    pub hclock: *mut clk,
    pub ispck: *mut clk,
    pub isc_clks: [isc_clk; 2],
    pub ispck_required: bool,
    pub dcfg: u32,
    pub dev: *mut device,
    pub v4l2_dev: v4l2_device,
    pub video_dev: video_device,
    pub vb2_vidq: vb2_queue,
    pub dma_queue_lock: spinlock_t,
    pub dma_queue: list_head,
    pub cur_frm: *mut isc_buffer,
    pub sequence: c_uint,
    pub stop: bool,
    pub comp: completion,
    pub fmt: v4l2_format,
    pub try_fmt: v4l2_format,
    pub config: fmt_config,
    pub try_config: fmt_config,
    pub ctrls: isc_ctrls,
    pub awb_work: work_struct,
    pub lock: mutex,
    pub awb_mutex: mutex,
    pub awb_lock: spinlock_t,
    pub pipeline: [*mut regmap_field; ISC_PIPE_LINE_NODE_NUM],
    pub current_subdev: *mut isc_subdev_entity,
    pub subdev_entities: list_head,
pub const ISC_CTRL_DO_WB: c_int = 1;
pub const ISC_CTRL_R_GAIN: c_int = 2;
pub const ISC_CTRL_B_GAIN: c_int = 3;
pub const ISC_CTRL_GR_GAIN: c_int = 4;
pub const ISC_CTRL_GB_GAIN: c_int = 5;
pub const ISC_CTRL_R_OFF: c_int = 6;
pub const ISC_CTRL_B_OFF: c_int = 7;
pub const ISC_CTRL_GR_OFF: c_int = 8;
pub const ISC_CTRL_GB_OFF: c_int = 9;
    pub awb_ctrl: *mut v4l2_ctrl,
    pub do_wb_ctrl: *mut v4l2_ctrl,
    pub r_gain_ctrl: *mut v4l2_ctrl,
    pub b_gain_ctrl: *mut v4l2_ctrl,
    pub gr_gain_ctrl: *mut v4l2_ctrl,
    pub gb_gain_ctrl: *mut v4l2_ctrl,
    pub r_off_ctrl: *mut v4l2_ctrl,
    pub b_off_ctrl: *mut v4l2_ctrl,
    pub gr_off_ctrl: *mut v4l2_ctrl,
    pub gb_off_ctrl: *mut v4l2_ctrl,
}

pub const GAMMA_ENTRIES: c_int = 64;
// pointer to the defined gamma table
extern "C" {
    pub fn microchip_isc_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn microchip_isc_pipeline_init(isc: *mut isc_device) -> c_int;
}
extern "C" {
    pub fn microchip_isc_clk_init(isc: *mut isc_device) -> c_int;
}
extern "C" {
    pub fn microchip_isc_subdev_cleanup(isc: *mut isc_device);
}
extern "C" {
    pub fn microchip_isc_clk_cleanup(isc: *mut isc_device);
}
extern "C" {
    pub fn isc_scaler_link(isc: *mut isc_device) -> c_int;
}
extern "C" {
    pub fn isc_scaler_init(isc: *mut isc_device) -> c_int;
}
extern "C" {
    pub fn isc_mc_init(isc: *mut isc_device, ver: u32) -> c_int;
}
extern "C" {
    pub fn isc_mc_cleanup(isc: *mut isc_device);
}
