//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/exynos4-is/fimc-core.h
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
// Copyright (C) 2010 - 2012 Samsung Electronics Co., Ltd.
//
// #define DEBUG

// Time to wait for next frame VSYNC interrupt while stopping operation.

pub const MAX_FIMC_CLOCKS: c_int = 2;

pub const FIMC_MAX_DEVS: c_int = 4;
pub const FIMC_MAX_OUT_BUFS: c_int = 4;
pub const SCALER_MAX_HRATIO: c_int = 64;
pub const SCALER_MAX_VRATIO: c_int = 64;
pub const DMA_MIN_SIZE: c_int = 8;
pub const FIMC_CAMIF_MAX_HEIGHT: c_uint = 0x2000;

pub const FIMC_MAX_PLANES: c_int = 3;
pub const FIMC_PIX_LIMITS_MAX: c_int = 4;
pub const FIMC_DEF_MIN_SIZE: c_int = 16;
pub const FIMC_DEF_HEIGHT_ALIGN: c_int = 2;
pub const FIMC_DEF_HOR_OFFS_ALIGN: c_int = 1;
pub const FIMC_DEFAULT_WIDTH: c_int = 640;
pub const FIMC_DEFAULT_HEIGHT: c_int = 480;
// indices to the clocks array
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fimc_dev_flags {
    ST_LPM,
// m2m node
    ST_M2M_RUN,
    ST_M2M_PEND,
    ST_M2M_SUSPENDING,
    ST_M2M_SUSPENDED,
// capture node
    ST_CAPT_PEND,
    ST_CAPT_RUN,
    ST_CAPT_STREAM,
    ST_CAPT_ISP_STREAM,
    ST_CAPT_SUSPENDED,
    ST_CAPT_SHUT,
    ST_CAPT_BUSY,
    ST_CAPT_APPLY_CFG,
    ST_CAPT_JPEG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fimc_datapath {
    FIMC_IO_NONE,
    FIMC_IO_CAMERA,
    FIMC_IO_DMA,
    FIMC_IO_LCDFIFO,
    FIMC_IO_WRITEBACK,
    FIMC_IO_ISP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fimc_color_fmt {
    FIMC_FMT_RGB444	= 0x10,
    FIMC_FMT_RGB555,
    FIMC_FMT_RGB565,
    FIMC_FMT_RGB666,
    FIMC_FMT_RGB888,
    FIMC_FMT_RGB30_LOCAL,
    FIMC_FMT_YCBCR420 = 0x20,
    FIMC_FMT_YCBYCR422,
    FIMC_FMT_YCRYCB422,
    FIMC_FMT_CBYCRY422,
    FIMC_FMT_CRYCBY422,
    FIMC_FMT_YCBCR444_LOCAL,
    FIMC_FMT_RAW8 = 0x40,
    FIMC_FMT_RAW10,
    FIMC_FMT_RAW12,
    FIMC_FMT_JPEG = 0x80,
    FIMC_FMT_YUYV_JPEG = 0x100,
}

// The hardware context state.

// Image conversion flags

//
// YCbCr data dynamic range for RGB-YUV color conversion.
// Y/Cb/Cr: (0 ~ 255)

// Y (16 ~ 235), Cb/Cr (16 ~ 240)

//
// struct fimc_dma_offset - pixel offset information for DMA
// @y_h:	y value horizontal offset
// @y_v:	y value vertical offset
// @cb_h:	cb value horizontal offset
// @cb_v:	cb value vertical offset
// @cr_h:	cr value horizontal offset
// @cr_v:	cr value vertical offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_dma_offset {
    pub y_h: c_int,
    pub y_v: c_int,
    pub cb_h: c_int,
    pub cb_v: c_int,
    pub cr_h: c_int,
    pub cr_v: c_int,
}

//
// struct fimc_effect - color effect information
// @type:	effect type
// @pat_cb:	cr value when type is "arbitrary"
// @pat_cr:	cr value when type is "arbitrary"
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_effect {
    pub type: u32,
    pub pat_cb: u8,
    pub pat_cr: u8,
}

//
// struct fimc_scaler - the configuration data for FIMC inetrnal scaler
// @scaleup_h:		flag indicating scaling up horizontally
// @scaleup_v:		flag indicating scaling up vertically
// @copy_mode:		flag indicating transparent DMA transfer (no scaling
// and color format conversion)
// @enabled:		flag indicating if the scaler is used
// @hfactor:		horizontal shift factor
// @vfactor:		vertical shift factor
// @pre_hratio:		horizontal ratio of the prescaler
// @pre_vratio:		vertical ratio of the prescaler
// @pre_dst_width:	the prescaler's destination width
// @pre_dst_height:	the prescaler's destination height
// @main_hratio:	the main scaler's horizontal ratio
// @main_vratio:	the main scaler's vertical ratio
// @real_width:		source pixel (width - offset)
// @real_height:	source pixel (height - offset)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_scaler {
    pub scaleup_h:1: c_uint,
    pub scaleup_v:1: c_uint,
    pub copy_mode:1: c_uint,
    pub enabled:1: c_uint,
    pub hfactor: u32,
    pub vfactor: u32,
    pub pre_hratio: u32,
    pub pre_vratio: u32,
    pub pre_dst_width: u32,
    pub pre_dst_height: u32,
    pub main_hratio: u32,
    pub main_vratio: u32,
    pub real_width: u32,
    pub real_height: u32,
}

//
// struct fimc_addr - the FIMC address set for DMA
// @y:	 luminance plane address
// @cb:	 Cb plane address
// @cr:	 Cr plane address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_addr {
    pub y: u32,
    pub cb: u32,
    pub cr: u32,
}

//
// struct fimc_vid_buffer - the driver's video buffer
// @vb:    v4l vb2 buffer
// @list:  linked list structure for buffer queue
// @addr: precalculated DMA address set
// @index: buffer index for the output DMA engine
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_vid_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub addr: fimc_addr,
    pub index: c_int,
}

//
// struct fimc_frame - source/target frame properties
// @f_width:	image full width (virtual screen size)
// @f_height:	image full height (virtual screen size)
// @o_width:	original image width as set by S_FMT
// @o_height:	original image height as set by S_FMT
// @offs_h:	image horizontal pixel offset
// @offs_v:	image vertical pixel offset
// @width:	image pixel width
// @height:	image pixel weight
// @payload:	image size in bytes (w x h x bpp)
// @bytesperline: bytesperline value for each plane
// @addr:	image frame buffer DMA addresses
// @dma_offset:	DMA offset in bytes
// @fmt:	fimc color format pointer
// @alpha:	alpha value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_frame {
    pub f_width: u32,
    pub f_height: u32,
    pub o_width: u32,
    pub o_height: u32,
    pub offs_h: u32,
    pub offs_v: u32,
    pub width: u32,
    pub height: u32,
    pub payload: [c_uint; VIDEO_MAX_PLANES],
    pub bytesperline: [c_uint; VIDEO_MAX_PLANES],
    pub addr: fimc_addr,
    pub dma_offset: fimc_dma_offset,
    pub fmt: *const fimc_fmt,
    pub alpha: u8,
}

//
// struct fimc_m2m_device - v4l2 memory-to-memory device data
// @vfd: the video device node for v4l2 m2m mode
// @m2m_dev: v4l2 memory-to-memory device data
// @ctx: hardware context data
// @refcnt: the reference counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_m2m_device {
    pub vfd: video_device,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub ctx: *mut fimc_ctx,
    pub refcnt: c_int,
}

pub const FIMC_SD_PAD_SINK_CAM: c_int = 0;
pub const FIMC_SD_PAD_SINK_FIFO: c_int = 1;
pub const FIMC_SD_PAD_SOURCE: c_int = 2;
pub const FIMC_SD_PADS_NUM: c_int = 3;
//
// struct fimc_vid_cap - camera capture device information
// @ctx: hardware context data
// @subdev: subdev exposing the FIMC processing block
// @ve: exynos video device entity structure
// @vd_pad: fimc video capture node pad
// @sd_pads: fimc video processing block pads
// @ci_fmt: image format at the FIMC camera input (and the scaler output)
// @wb_fmt: image format at the FIMC ISP Writeback input
// @source_config: external image source related configuration structure
// @pending_buf_q: the pending buffer queue head
// @active_buf_q: the queue head of buffers scheduled in hardware
// @vbq: the capture am video buffer queue
// @active_buf_cnt: number of video buffers scheduled in hardware
// @buf_index: index for managing the output DMA buffers
// @frame_count: the frame counter for statistics
// @reqbufs_count: the number of buffers requested in REQBUFS ioctl
// @streaming: is streaming in progress?
// @input: capture input type, grp_id of the attached subdev
// @user_subdev_api: true if subdevs are not configured by the host driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_vid_cap {
    pub ctx: *mut fimc_ctx,
    pub subdev: v4l2_subdev,
    pub ve: exynos_video_entity,
    pub vd_pad: media_pad,
    pub sd_pads: [media_pad; FIMC_SD_PADS_NUM],
    pub ci_fmt: v4l2_mbus_framefmt,
    pub wb_fmt: v4l2_mbus_framefmt,
    pub source_config: fimc_source_info,
    pub pending_buf_q: list_head,
    pub active_buf_q: list_head,
    pub vbq: vb2_queue,
    pub active_buf_cnt: c_int,
    pub buf_index: c_int,
    pub frame_count: c_uint,
    pub reqbufs_count: c_uint,
    pub streaming: bool,
    pub input: u32,
    pub user_subdev_api: bool,
}

//
// struct fimc_pix_limit - image pixel size limits in various IP configurations
//
// @scaler_en_w: max input pixel width when the scaler is enabled
// @scaler_dis_w: max input pixel width when the scaler is disabled
// @in_rot_en_h: max input width with the input rotator is on
// @in_rot_dis_w: max input width with the input rotator is off
// @out_rot_en_w: max output width with the output rotator on
// @out_rot_dis_w: max output width with the output rotator off
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_pix_limit {
    pub scaler_en_w: u16,
    pub scaler_dis_w: u16,
    pub in_rot_en_h: u16,
    pub in_rot_dis_w: u16,
    pub out_rot_en_w: u16,
    pub out_rot_dis_w: u16,
}

//
// struct fimc_variant - FIMC device variant information
// @has_inp_rot: set if has input rotator
// @has_out_rot: set if has output rotator
// @has_mainscaler_ext: 1 if extended mainscaler ratios in CIEXTEN register
// are present in this IP revision
// @has_cam_if: set if this instance has a camera input interface
// @has_isp_wb: set if this instance has ISP writeback input
// @pix_limit: pixel size constraints for the scaler
// @min_inp_pixsize: minimum input pixel size
// @min_out_pixsize: minimum output pixel size
// @hor_offs_align: horizontal pixel offset alignment
// @min_vsize_align: minimum vertical pixel size alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_variant {
    pub has_inp_rot:1: c_uint,
    pub has_out_rot:1: c_uint,
    pub has_mainscaler_ext:1: c_uint,
    pub has_cam_if:1: c_uint,
    pub has_isp_wb:1: c_uint,
    pub pix_limit: *const fimc_pix_limit,
    pub min_inp_pixsize: u16,
    pub min_out_pixsize: u16,
    pub hor_offs_align: u16,
    pub min_vsize_align: u16,
}

//
// struct fimc_drvdata - per device type driver data
// @variant: variant information for this device
// @num_entities: number of fimc instances available in a SoC
// @lclk_frequency: local bus clock frequency
// @cistatus2: 1 if the FIMC IPs have CISTATUS2 register
// @dma_pix_hoff: the horizontal DMA offset unit: 1 - pixels, 0 - bytes
// @alpha_color: 1 if alpha color component is supported
// @out_buf_count: maximum number of output DMA buffers supported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_drvdata {
    pub variant: [*const fimc_variant; FIMC_MAX_DEVS],
    pub num_entities: c_int,
    pub lclk_frequency: c_ulong,
// Fields common to all FIMC IP instances
    pub cistatus2: u8,
    pub dma_pix_hoff: u8,
    pub alpha_color: u8,
    pub out_buf_count: u8,
}

//
// struct fimc_dev - abstraction for FIMC entity
// @slock:	the spinlock protecting this data structure
// @lock:	the mutex protecting this data structure
// @pdev:	pointer to the FIMC platform device
// @pdata:	pointer to the device platform data
// @sysreg:	pointer to the SYSREG regmap
// @variant:	the IP variant information
// @drv_data:	driver data
// @id:		FIMC device index (0..FIMC_MAX_DEVS)
// @clock:	clocks required for FIMC operation
// @regs:	the mapped hardware registers
// @irq_queue:	interrupt handler waitqueue
// @v4l2_dev:	root v4l2_device
// @m2m:	memory-to-memory V4L2 device information
// @vid_cap:	camera capture device information
// @state:	flags used to synchronize m2m and capture mode operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_dev {
    pub slock: spinlock_t,
    pub lock: mutex,
    pub pdev: *mut platform_device,
    pub pdata: *mut s5p_platform_fimc,
    pub sysreg: *mut regmap,
    pub variant: *const fimc_variant,
    pub drv_data: *const fimc_drvdata,
    pub id: c_int,
    pub clock: [*mut clk; MAX_FIMC_CLOCKS],
    pub regs: *mut void __iomem,
    pub irq_queue: wait_queue_head_t,
    pub v4l2_dev: *mut v4l2_device,
    pub m2m: fimc_m2m_device,
    pub vid_cap: fimc_vid_cap,
    pub state: c_ulong,
}

//
// struct fimc_ctrls - v4l2 controls structure
// @handler: the control handler
// @colorfx: image effect control
// @colorfx_cbcr: Cb/Cr coefficients control
// @rotate: image rotation control
// @hflip: horizontal flip control
// @vflip: vertical flip control
// @alpha: RGB alpha control
// @ready: true if @handler is initialized
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_ctrls {
    pub handler: v4l2_ctrl_handler,
    pub colorfx: *mut v4l2_ctrl,
    pub colorfx_cbcr: *mut v4l2_ctrl,
}

//
// struct fimc_ctx - the device context data
// @s_frame:		source frame properties
// @d_frame:		destination frame properties
// @out_order_1p:	output 1-plane YCBCR order
// @out_order_2p:	output 2-plane YCBCR order
// @in_order_1p:	input 1-plane YCBCR order
// @in_order_2p:	input 2-plane YCBCR order
// @in_path:		input mode (DMA or camera)
// @out_path:		output mode (DMA or FIFO)
// @scaler:		image scaler properties
// @effect:		image effect
// @rotation:		image clockwise rotation in degrees
// @hflip:		indicates image horizontal flip if set
// @vflip:		indicates image vertical flip if set
// @flags:		additional flags for image conversion
// @state:		flags to keep track of user configuration
// @fimc_dev:		the FIMC device this context applies to
// @fh:			v4l2 file handle
// @ctrls:		v4l2 controls structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_ctx {
    pub s_frame: fimc_frame,
    pub d_frame: fimc_frame,
    pub out_order_1p: u32,
    pub out_order_2p: u32,
    pub in_order_1p: u32,
    pub in_order_2p: u32,
    pub in_path: fimc_datapath,
    pub out_path: fimc_datapath,
    pub scaler: fimc_scaler,
    pub effect: fimc_effect,
    pub rotation: c_int,
    pub hflip:1: c_uint,
    pub vflip:1: c_uint,
    pub flags: u32,
    pub state: u32,
    pub fimc_dev: *mut fimc_dev,
    pub fh: v4l2_fh,
    pub ctrls: fimc_ctrls,
}

extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), fimc_ctx: struct, _arg: fh) -> return;
}
// Return the alpha component bit mask
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
// -----------------------------------------------------
// fimc-core.c
extern "C" {
    pub fn fimc_ctrls_create(ctx: *mut fimc_ctx) -> c_int;
}
extern "C" {
    pub fn fimc_ctrls_delete(ctx: *mut fimc_ctx);
}
extern "C" {
    pub fn fimc_ctrls_activate(ctx: *mut fimc_ctx, active: bool);
}
extern "C" {
    pub fn fimc_alpha_ctrl_update(ctx: *mut fimc_ctx);
}
extern "C" {
    pub fn __fimc_get_format(frame: *const fimc_frame, f: *mut v4l2_format);
}
extern "C" {
    pub fn fimc_set_scaler_info(ctx: *mut fimc_ctx) -> c_int;
}
extern "C" {
    pub fn fimc_prepare_config(ctx: *mut fimc_ctx, flags: u32) -> c_int;
}
extern "C" {
    pub fn fimc_prepare_dma_offset(ctx: *mut fimc_ctx, f: *mut fimc_frame);
}
extern "C" {
    pub fn fimc_set_yuv_order(ctx: *mut fimc_ctx);
}
extern "C" {
    pub fn fimc_capture_irq_handler(fimc: *mut fimc_dev, deq_buf: c_int);
}
extern "C" {
    pub fn fimc_unregister_m2m_device(fimc: *mut fimc_dev);
}
extern "C" {
    pub fn fimc_register_driver() -> c_int;
}
extern "C" {
    pub fn fimc_unregister_driver();
}

extern "C" {
    pub fn syscon_regmap_lookup_by_phandle(_arg: node, _arg: "samsung, _arg: sysreg") -> return;
}

// -----------------------------------------------------
// fimc-m2m.c
extern "C" {
    pub fn fimc_m2m_job_finish(ctx: *mut fimc_ctx, vb_state: c_int);
}
// -----------------------------------------------------
// fimc-capture.c
extern "C" {
    pub fn fimc_initialize_capture_subdev(fimc: *mut fimc_dev) -> c_int;
}
extern "C" {
    pub fn fimc_unregister_capture_subdev(fimc: *mut fimc_dev);
}
extern "C" {
    pub fn fimc_capture_ctrls_create(fimc: *mut fimc_dev) -> c_int;
}
extern "C" {
    pub fn fimc_capture_suspend(fimc: *mut fimc_dev) -> c_int;
}
extern "C" {
    pub fn fimc_capture_resume(fimc: *mut fimc_dev) -> c_int;
}
//
// Buffer list manipulation functions. Must be called with fimc.slock held.
//
// fimc_active_queue_add - add buffer to the capture active buffers queue
// @vid_cap:	camera capture device information
// @buf: buffer to add to the active buffers list
//
// fimc_active_queue_pop - pop buffer from the capture active buffers queue
// @vid_cap:	camera capture device information
//
// The caller must assure the active_buf_q list is not empty.
//
// fimc_pending_queue_add - add buffer to the capture pending buffers queue
// @vid_cap:	camera capture device information
// @buf: buffer to add to the pending buffers list
//
// fimc_pending_queue_pop - pop buffer from the capture pending buffers queue
// @vid_cap:	camera capture device information
//
// The caller must assure the pending_buf_q list is not empty.
//
