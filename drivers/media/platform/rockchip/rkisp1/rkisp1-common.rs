//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/rockchip/rkisp1/rkisp1-common.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR MIT)
//
// Rockchip ISP1 Driver - Common definitions
//
// Copyright (C) 2019 Collabora, Ltd.
//
// Based on Rockchip ISP1 driver by Rockchip Electronics Co., Ltd.
// Copyright (C) 2017 Rockchip Electronics Co., Ltd.
//

//
// flags on the 'direction' field in struct rkisp1_mbus_info' that indicate
// on which pad the media bus format is supported
//

//
// Minimum values for the width and height of entities. The maximum values are
// model-specific and stored in the rkisp1_info structure.
//
pub const RKISP1_ISP_MIN_WIDTH: c_int = 32;
pub const RKISP1_ISP_MIN_HEIGHT: c_int = 32;
pub const RKISP1_RSZ_MP_SRC_MAX_WIDTH: c_int = 4416;
pub const RKISP1_RSZ_MP_SRC_MAX_HEIGHT: c_int = 3312;
pub const RKISP1_RSZ_SP_SRC_MAX_WIDTH: c_int = 1920;
pub const RKISP1_RSZ_SP_SRC_MAX_HEIGHT: c_int = 1920;
pub const RKISP1_RSZ_SRC_MIN_WIDTH: c_int = 32;
pub const RKISP1_RSZ_SRC_MIN_HEIGHT: c_int = 16;
// the default width and height of all the entities
pub const RKISP1_DEFAULT_WIDTH: c_int = 800;
pub const RKISP1_DEFAULT_HEIGHT: c_int = 600;

// maximum number of clocks
pub const RKISP1_MAX_BUS_CLK: c_int = 4;
// a bitmask of the ready stats

// IRQ lines
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_irq_line {
    RKISP1_IRQ_ISP = 0,
    RKISP1_IRQ_MI,
    RKISP1_IRQ_MIPI,
    RKISP1_NUM_IRQS,
}

// enum for the resizer pads
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_rsz_pad {
    RKISP1_RSZ_PAD_SINK,
    RKISP1_RSZ_PAD_SRC,
    RKISP1_RSZ_PAD_MAX
}

// enum for the csi receiver pads
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_csi_pad {
    RKISP1_CSI_PAD_SINK,
    RKISP1_CSI_PAD_SRC,
    RKISP1_CSI_PAD_NUM
}

// enum for the capture id
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_stream_id {
    RKISP1_MAINPATH,
    RKISP1_SELFPATH,
}

// bayer patterns
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_fmt_raw_pat_type {
    RKISP1_RAW_RGGB = 0,
    RKISP1_RAW_GRBG,
    RKISP1_RAW_GBRG,
    RKISP1_RAW_BGGR,
}

// enum for the isp pads
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_isp_pad {
    RKISP1_ISP_PAD_SINK_VIDEO,
    RKISP1_ISP_PAD_SINK_PARAMS,
    RKISP1_ISP_PAD_SOURCE_VIDEO,
    RKISP1_ISP_PAD_SOURCE_STATS,
    RKISP1_ISP_PAD_MAX
}

//
// enum rkisp1_feature - ISP features
//
// @RKISP1_FEATURE_MIPI_CSI2: The ISP has an internal MIPI CSI-2 receiver
// @RKISP1_FEATURE_MAIN_STRIDE: The ISP supports configurable stride on the main path
// @RKISP1_FEATURE_SELF_PATH: The ISP has a self path
// @RKISP1_FEATURE_DUAL_CROP: The ISP has the dual crop block at the resizer input
// @RKISP1_FEATURE_DMA_34BIT: The ISP uses 34-bit DMA addresses
// @RKISP1_FEATURE_BLS: The ISP has a dedicated BLS block
// @RKISP1_FEATURE_COMPAND: The ISP has a companding block
//
// The ISP features are stored in a bitmask in &rkisp1_info.features and allow
// the driver to implement support for features present in some ISP versions
// only.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkisp1_feature {
    RKISP1_FEATURE_MIPI_CSI2 = BIT(0),
    RKISP1_FEATURE_MAIN_STRIDE = BIT(1),
    RKISP1_FEATURE_SELF_PATH = BIT(2),
    RKISP1_FEATURE_DUAL_CROP = BIT(3),
    RKISP1_FEATURE_DMA_34BIT = BIT(4),
    RKISP1_FEATURE_BLS = BIT(5),
    RKISP1_FEATURE_COMPAND = BIT(6),
}

//
// struct rkisp1_info - Model-specific ISP Information
//
// @num_clocks: number of clocks
// @isrs: array of ISP interrupt descriptors
// @isr_size: number of entries in the @isrs array
// @isp_ver: ISP version
// @features: bitmask of rkisp1_feature features implemented by the ISP
// @max_width: maximum input frame width
// @max_height: maximum input frame height
// @pm_domains.names: name of the power domains
// @pm_domains.count: number of power domains
//
// This structure contains information about the ISP specific to a particular
// ISP model, version, or integration in a particular SoC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_info {
    pub num_clocks: c_uint,
    pub isrs: *const rkisp1_isr_data,
    pub isr_size: c_uint,
    pub isp_ver: rkisp1_cif_isp_version,
    pub features: c_uint,
    pub max_width: c_uint,
    pub max_height: c_uint,
    pub names: *const *const c_char,
    pub count: c_uint,
    pub pm_domains: },
}

//
// struct rkisp1_sensor_async - A container for the v4l2_async_subdev to add to the notifier
// of the v4l2-async API
//
// @asd:		async_subdev variable for the sensor
// @index:		index of the sensor (counting sensor found in DT)
// @source_ep:		fwnode for the sensor source endpoint
// @lanes:		number of lanes
// @mbus_type:		type of bus (currently only CSI2 is supported)
// @mbus_flags:		media bus (V4L2_MBUS_*) flags
// @sd:			a pointer to v4l2_subdev struct of the sensor
// @pixel_rate_ctrl:	pixel rate of the sensor, used to initialize the phy
// @port:		port number (0: MIPI, 1: Parallel)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_sensor_async {
    pub asd: v4l2_async_connection,
    pub index: c_uint,
    pub source_ep: *mut fwnode_handle,
    pub lanes: c_uint,
    pub mbus_type: v4l2_mbus_type,
    pub mbus_flags: c_uint,
    pub sd: *mut v4l2_subdev,
    pub pixel_rate_ctrl: *mut v4l2_ctrl,
    pub port: c_uint,
}

//
// struct rkisp1_csi - CSI receiver subdev
//
// @rkisp1: pointer to the rkisp1 device
// @dphy: a pointer to the phy
// @is_dphy_errctrl_disabled: if dphy errctrl is disabled (avoid endless interrupt)
// @sd: v4l2_subdev variable
// @pads: media pads
// @source: source in-use, set when starting streaming
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_csi {
    pub rkisp1: *mut rkisp1_device,
    pub dphy: *mut phy,
    pub is_dphy_errctrl_disabled: bool,
    pub sd: v4l2_subdev,
    pub pads: [media_pad; RKISP1_CSI_PAD_NUM],
    pub source: *mut v4l2_subdev,
}

//
// struct rkisp1_isp - ISP subdev entity
//
// @sd:				v4l2_subdev variable
// @rkisp1:			pointer to rkisp1_device
// @pads:			media pads
// @sink_fmt:			input format
// @frame_sequence:		used to synchronize frame_id between video devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_isp {
    pub sd: v4l2_subdev,
    pub rkisp1: *mut rkisp1_device,
    pub pads: [media_pad; RKISP1_ISP_PAD_MAX],
    pub sink_fmt: *const rkisp1_mbus_info,
    pub frame_sequence: __u32,
    pub frame_active: bool,
}

//
// struct rkisp1_vdev_node - Container for the video nodes: params, stats, mainpath, selfpath
//
// @buf_queue:	queue of buffers
// @vlock:	lock of the video node
// @vdev:	video node
// @pad:	media pad
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_vdev_node {
    pub buf_queue: vb2_queue,
    pub /: *mut *mut mutex vlock; / ioctl serialization mutex,
    pub vdev: video_device,
    pub pad: media_pad,
}

//
// struct rkisp1_buffer - A container for the vb2 buffers used by the video devices:
// stats, mainpath, selfpath
//
// @vb:		vb2 buffer
// @queue:	entry of the buffer in the queue
// @buff_addr:	dma addresses of each plane, used only by the capture devices: selfpath, mainpath
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_buffer {
    pub vb: vb2_v4l2_buffer,
    pub queue: list_head,
    pub buff_addr: [dma_addr_t; VIDEO_MAX_PLANES],
}

//
// struct rkisp1_params_buffer - A container for the vb2 buffers used by the
// params video device
//
// @vb:		vb2 buffer
// @queue:	entry of the buffer in the queue
// @cfg:	scratch buffer used for caching the ISP configuration parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_params_buffer {
    pub vb: vb2_v4l2_buffer,
    pub queue: list_head,
    pub cfg: *mut c_void,
}

extern "C" {
    pub fn container_of(_arg: vbuf, rkisp1_params_buffer: struct, _arg: vb) -> return;
}
//
// struct rkisp1_dummy_buffer - A buffer to write the next frame to in case
// there are no vb2 buffers available.
//
// @vaddr:	return value of call to dma_alloc_attrs.
// @dma_addr:	dma address of the buffer.
// @size:	size of the buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_dummy_buffer {
    pub vaddr: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub size: u32,
}

//
// struct rkisp1_capture - ISP capture video device
//
// @vnode:	  video node
// @rkisp1:	  pointer to rkisp1_device
// @id:		  id of the capture, one of RKISP1_SELFPATH, RKISP1_MAINPATH
// @ops:	  list of callbacks to configure the capture device.
// @config:	  a pointer to the list of registers to configure the capture format.
// @is_streaming: device is streaming
// @is_stopping:  stop_streaming callback was called and the device is in the process of
// stopping the streaming.
// @done:	  when stop_streaming callback is called, the device waits for the next irq
// handler to stop the streaming by waiting on the 'done' wait queue.
// If the irq handler is not called, the stream is stopped by the callback
// after timeout.
// @stride:       the line stride for the first plane, in pixel units
// @buf.lock:	  lock to protect buf.queue
// @buf.queue:	  queued buffer list
// @buf.dummy:	  dummy space to store dropped data
//
// rkisp1 uses shadow registers, so it needs two buffers at a time
// @buf.curr:	  the buffer used for current frame
// @buf.next:	  the buffer used for next frame
// @pix.cfg:	  pixel configuration
// @pix.info:	  a pointer to the v4l2_format_info of the pixel format
// @pix.fmt:	  buffer format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_capture {
    pub vnode: rkisp1_vdev_node,
    pub rkisp1: *mut rkisp1_device,
    pub id: rkisp1_stream_id,
    pub ops: *const rkisp1_capture_ops,
    pub config: *const rkisp1_capture_config,
    pub is_streaming: bool,
    pub is_stopping: bool,
    pub done: wait_queue_head_t,
    pub stride: c_uint,
// protects queue, curr and next
    pub lock: spinlock_t,
    pub queue: list_head,
    pub dummy: rkisp1_dummy_buffer,
    pub curr: *mut rkisp1_buffer,
    pub next: *mut rkisp1_buffer,
    pub buf: },
    pub cfg: *const rkisp1_capture_fmt_cfg,
    pub info: *const v4l2_format_info,
    pub fmt: v4l2_pix_format_mplane,
    pub pix: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_stats_ops {
    pub pbuf): *mut rkisp1_stat_buffer,
    pub pbuf): *mut rkisp1_stat_buffer,
    pub pbuf): *mut rkisp1_stat_buffer,
}

//
// struct rkisp1_stats - ISP Statistics device
//
// @vnode:	  video node
// @rkisp1:	  pointer to the rkisp1 device
// @lock:	  locks the buffer list 'stat'
// @stat:	  queue of rkisp1_buffer
// @vdev_fmt:	  v4l2_format of the metadata format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_stats {
    pub vnode: rkisp1_vdev_node,
    pub rkisp1: *mut rkisp1_device,
    pub ops: *const rkisp1_stats_ops,
    pub /: *mut *mut spinlock_t lock; / locks the buffers list 'stats',
    pub stat: list_head,
    pub vdev_fmt: v4l2_format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_params_ops {
    pub pconfig): *const rkisp1_cif_isp_lsc_config,
    pub arg): *const rkisp1_cif_isp_goc_config,
    pub arg): *const rkisp1_cif_isp_awb_meas_config,
    pub en): bool,
    pub arg): *const rkisp1_cif_isp_awb_gain_config,
    pub arg): *const rkisp1_cif_isp_aec_config,
    pub arg): *const rkisp1_cif_isp_hst_config,
    pub en): *const *const rkisp1_cif_isp_hst_config arg, bool,
    pub arg): *const rkisp1_cif_isp_afc_config,
}

//
// struct rkisp1_params - ISP input parameters device
//
// @vnode:		video node
// @rkisp1:		pointer to the rkisp1 device
// @ops:		pointer to the variant-specific operations
// @config_lock:	locks the buffer list 'params'
// @params:		queue of rkisp1_buffer
// @metafmt		the currently enabled metadata format
// @quantization:	the quantization configured on the isp's src pad
// @ycbcr_encoding	the YCbCr encoding
// @raw_type:		the bayer pattern on the isp video sink pad
// @enabled_blocks:	bitmask of enabled ISP blocks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_params {
    pub vnode: rkisp1_vdev_node,
    pub rkisp1: *mut rkisp1_device,
    pub ops: *const rkisp1_params_ops,
    pub /: *mut *mut spinlock_t config_lock; / locks the buffers list 'params',
    pub params: list_head,
    pub ctrls: v4l2_ctrl_handler,
    pub metafmt: *const v4l2_meta_format,
    pub quantization: v4l2_quantization,
    pub ycbcr_encoding: v4l2_ycbcr_encoding,
    pub raw_type: rkisp1_fmt_raw_pat_type,
    pub enabled_blocks: u32,
}

//
// struct rkisp1_resizer - Resizer subdev
//
// @sd:	       v4l2_subdev variable
// @regs_base: base register address offset
// @id:	       id of the resizer, one of RKISP1_SELFPATH, RKISP1_MAINPATH
// @rkisp1:    pointer to the rkisp1 device
// @pads:      media pads
// @config:    the set of registers to configure the resizer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_resizer {
    pub sd: v4l2_subdev,
    pub regs_base: u32,
    pub id: rkisp1_stream_id,
    pub rkisp1: *mut rkisp1_device,
    pub pads: [media_pad; RKISP1_RSZ_PAD_MAX],
    pub config: *const rkisp1_rsz_config,
}

//
// struct rkisp1_debug - Values to be exposed on debugfs.
// The parameters are counters of the number of times the
// event occurred since the driver was loaded.
//
// @data_loss:			  loss of data occurred within a line, processing failure
// @outform_size_error:		  size error is generated in outmux submodule
// @img_stabilization_size_error: size error is generated in image stabilization submodule
// @inform_size_err:		  size error is generated in inform submodule
// @mipi_error:			  mipi error occurred
// @stats_error:		  writing to the 'Interrupt clear register' did not clear
// it in the register 'Masked interrupt status'
// @stop_timeout:		  upon stream stop, the capture waits 1 second for the isr to stop
// the stream. This param is incremented in case of timeout.
// @frame_drop:			  a frame was ready but the buffer queue was empty so the frame
// was not sent to userspace
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_debug {
    pub debugfs_dir: *mut dentry,
    pub data_loss: c_ulong,
    pub outform_size_error: c_ulong,
    pub img_stabilization_size_error: c_ulong,
    pub inform_size_error: c_ulong,
    pub irq_delay: c_ulong,
    pub mipi_error: c_ulong,
    pub stats_error: c_ulong,
    pub stop_timeout: [c_ulong; 2],
    pub frame_drop: [c_ulong; 2],
    pub complete_frames: c_ulong,
}

//
// struct rkisp1_device - ISP platform device
//
// @base_addr:	   base register address
// @dev:	   a pointer to the struct device
// @clk_size:	   number of clocks
// @clks:	   array of clocks
// @pm_domains:    power domains
// @gasket:	   the gasket - i.MX8MP only
// @gasket_id:	   the gasket ID (0 or 1) - i.MX8MP only
// @v4l2_dev:	   v4l2_device variable
// @media_dev:	   media_device variable
// @notifier:	   a notifier to register on the v4l2-async API to be notified on the sensor
// @source:        source subdev in-use, set when starting streaming
// @csi:	   internal CSI-2 receiver
// @isp:	   ISP sub-device
// @resizer_devs:  resizer sub-devices
// @capture_devs:  capture devices
// @stats:	   ISP statistics metadata capture device
// @params:	   ISP parameters metadata output device
// @pipe:	   media pipeline
// @stream_lock:   serializes {start/stop}_streaming callbacks between the capture devices.
// @debug:	   debug params to be exposed on debugfs
// @info:	   version-specific ISP information
// @irqs:          IRQ line numbers
// @irqs_enabled:  the hardware is enabled and can cause interrupts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_device {
    pub base_addr: *mut void __iomem,
    pub dev: *mut device,
    pub clk_size: c_uint,
    pub clks: [clk_bulk_data; RKISP1_MAX_BUS_CLK],
    pub pm_domains: *mut dev_pm_domain_list,
    pub gasket: *mut regmap,
    pub gasket_id: c_uint,
    pub v4l2_dev: v4l2_device,
    pub media_dev: media_device,
    pub notifier: v4l2_async_notifier,
    pub source: *mut v4l2_subdev,
    pub csi: rkisp1_csi,
    pub isp: rkisp1_isp,
    pub resizer_devs: [rkisp1_resizer; 2],
    pub capture_devs: [rkisp1_capture; 2],
    pub stats: rkisp1_stats,
    pub params: rkisp1_params,
    pub pipe: media_pipeline,
    pub /: *mut *mut mutex stream_lock; / serialize {start/stop}_streaming cb between capture devices,
    pub debug: rkisp1_debug,
    pub info: *const rkisp1_info,
    pub irqs: [c_int; RKISP1_NUM_IRQS],
    pub irqs_enabled: bool,
}

//
// struct rkisp1_mbus_info - ISP media bus info, Translates media bus code to hardware
// format values
//
// @mbus_code: media bus code
// @pixel_enc: pixel encoding
// @mipi_dt:   mipi data type
// @yuv_seq:   the order of the Y, Cb, Cr values
// @bus_width: bus width
// @bayer_pat: bayer pattern
// @direction: a bitmask of the flags indicating on which pad the format is supported on
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkisp1_mbus_info {
    pub mbus_code: u32,
    pub pixel_enc: v4l2_pixel_encoding,
    pub mipi_dt: u32,
    pub yuv_seq: u32,
    pub bus_width: u8,
    pub bayer_pat: rkisp1_fmt_raw_pat_type,
    pub direction: c_uint,
}

extern "C" {
    pub fn readl(addr: rkisp1->base_addr +) -> return;
}
//
// rkisp1_cap_enum_mbus_codes - A helper function that return the i'th supported mbus code
// of the capture entity. This is used to enumerate the supported
// mbus codes on the source pad of the resizer.
//
// @cap:  the capture entity
// @code: the mbus code, the function reads the code->index and fills the code->code
//
// rkisp1_mbus_info_get_by_index - Retrieve the ith supported mbus info
//
// @index: index of the mbus info to fetch
//
// rkisp1_path_count - Return the number of paths supported by the device
//
// Some devices only have a main path, while other device have both a main path
// and a self path. This function returns the number of paths that this device
// has, based on the feature flags. It should be used insted of checking
// ARRAY_SIZE of capture_devs/resizer_devs.
//
// rkisp1_sd_adjust_crop_rect - adjust a rectangle to fit into another rectangle.
//
// @crop:   rectangle to adjust.
// @bounds: rectangle used as bounds.
//
// rkisp1_sd_adjust_crop - adjust a rectangle to fit into media bus format
//
// @crop:   rectangle to adjust.
// @bounds: media bus format used as bounds.
//
// rkisp1_mbus_info_get_by_code - get the isp info of the media bus code
//
// @mbus_code: the media bus code
//
// rkisp1_params_pre_configure - Configure the params before stream start
//
// @params:	  pointer to rkisp1_params
// @bayer_pat:	  the bayer pattern on the isp video sink pad
// @quantization: the quantization configured on the isp's src pad
// @ycbcr_encoding: the ycbcr_encoding configured on the isp's src pad
//
// This function is called by the ISP entity just before the ISP gets started.
// It applies the initial ISP parameters from the first params buffer, but
// skips LSC as it needs to be configured after the ISP is started.
//
// rkisp1_params_post_configure - Configure the params after stream start
//
// @params:	  pointer to rkisp1_params
//
// This function is called by the ISP entity just after the ISP gets started.
// It applies the initial ISP LSC parameters from the first params buffer.
//
extern "C" {
    pub fn rkisp1_params_post_configure(params: *mut rkisp1_params);
}
// rkisp1_params_disable - disable all parameters.
// This function is called by the isp entity upon stream start
// when capturing bayer format.
//
// @params: pointer to rkisp1_params.
//
extern "C" {
    pub fn rkisp1_params_disable(params: *mut rkisp1_params);
}
// irq handlers
extern "C" {
    pub fn rkisp1_isp_isr(irq: c_int, ctx: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn rkisp1_csi_isr(irq: c_int, ctx: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn rkisp1_capture_isr(irq: c_int, ctx: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn rkisp1_stats_isr(stats: *mut rkisp1_stats, isp_ris: u32);
}
extern "C" {
    pub fn rkisp1_params_isr(rkisp1: *mut rkisp1_device);
}
// register/unregisters functions of the entities
extern "C" {
    pub fn rkisp1_capture_devs_register(rkisp1: *mut rkisp1_device) -> c_int;
}
extern "C" {
    pub fn rkisp1_capture_devs_unregister(rkisp1: *mut rkisp1_device);
}
extern "C" {
    pub fn rkisp1_isp_register(rkisp1: *mut rkisp1_device) -> c_int;
}
extern "C" {
    pub fn rkisp1_isp_unregister(rkisp1: *mut rkisp1_device);
}
extern "C" {
    pub fn rkisp1_resizer_devs_register(rkisp1: *mut rkisp1_device) -> c_int;
}
extern "C" {
    pub fn rkisp1_resizer_devs_unregister(rkisp1: *mut rkisp1_device);
}
extern "C" {
    pub fn rkisp1_stats_register(rkisp1: *mut rkisp1_device) -> c_int;
}
extern "C" {
    pub fn rkisp1_stats_unregister(rkisp1: *mut rkisp1_device);
}
extern "C" {
    pub fn rkisp1_params_register(rkisp1: *mut rkisp1_device) -> c_int;
}
extern "C" {
    pub fn rkisp1_params_unregister(rkisp1: *mut rkisp1_device);
}

extern "C" {
    pub fn rkisp1_debug_init(rkisp1: *mut rkisp1_device);
}
extern "C" {
    pub fn rkisp1_debug_cleanup(rkisp1: *mut rkisp1_device);
}

