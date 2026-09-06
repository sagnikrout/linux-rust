//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/exynos-gsc/gsc-core.h
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
// Copyright (c) 2011 - 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// header file for Samsung EXYNOS5 SoC series G-Scaler driver
//

pub const GSC_MAX_DEVS: c_int = 4;
pub const GSC_MAX_CLOCKS: c_int = 4;
pub const GSC_M2M_BUF_NUM: c_int = 0;
pub const GSC_MAX_CTRL_NUM: c_int = 10;
pub const GSC_SC_ALIGN_4: c_int = 4;
pub const GSC_SC_ALIGN_2: c_int = 2;
pub const DEFAULT_CSC_EQ: c_int = 1;
pub const DEFAULT_CSC_RANGE: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gsc_dev_flags {
// for m2m node
    ST_M2M_OPEN,
    ST_M2M_RUN,
    ST_M2M_PEND,
    ST_M2M_SUSPENDED,
    ST_M2M_SUSPENDING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gsc_irq {
    GSC_IRQ_DONE,
    GSC_IRQ_OVERRUN
}

//
// enum gsc_datapath - the path of data used for G-Scaler
// @GSC_CAMERA: from camera
// @GSC_DMA: from/to DMA
// @GSC_WRITEBACK: from FIMD
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gsc_datapath {
    GSC_CAMERA = 0x1,
    GSC_DMA,
    GSC_WRITEBACK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gsc_color_fmt {
    GSC_RGB = 0x1,
    GSC_YUV420 = 0x2,
    GSC_YUV422 = 0x4,
    GSC_YUV444 = 0x8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gsc_yuv_fmt {
    GSC_LSB_Y = 0x10,
    GSC_LSB_C,
    GSC_CBCR = 0x20,
    GSC_CRCB,
}

//
// struct gsc_fmt - the driver's internal color format data
// @mbus_code: Media Bus pixel code, -1 if not applicable
// @pixelformat: the fourcc code for this format, 0 if not applicable
// @color: color encoding
// @yorder: Y/C order
// @corder: Chrominance order control
// @num_planes: number of physically non-contiguous data planes
// @num_comp: number of physically contiguous data planes
// @depth: per plane driver's private 'number of bits per pixel'
// @flags: flags indicating which operation mode format applies to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_fmt {
    pub mbus_code: u32,
    pub pixelformat: u32,
    pub color: u32,
    pub yorder: u32,
    pub corder: u32,
    pub num_planes: u16,
    pub num_comp: u16,
    pub depth: [u8; VIDEO_MAX_PLANES],
    pub flags: u32,
}

//
// struct gsc_input_buf - the driver's video buffer
// @vb:	videobuf2 buffer
// @list : linked list structure for buffer queue
// @idx : index of G-Scaler input buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_input_buf {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub idx: c_int,
}

//
// struct gsc_addr - the G-Scaler physical address set
// @y:	 luminance plane address
// @cb:	 Cb plane address
// @cr:	 Cr plane address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_addr {
    pub y: dma_addr_t,
    pub cb: dma_addr_t,
    pub cr: dma_addr_t,
}

// struct gsc_ctrls - the G-Scaler control set
// @rotate: rotation degree
// @hflip: horizontal flip
// @vflip: vertical flip
// @global_alpha: the alpha value of current frame
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_ctrls {
    pub rotate: *mut v4l2_ctrl,
    pub hflip: *mut v4l2_ctrl,
    pub vflip: *mut v4l2_ctrl,
    pub global_alpha: *mut v4l2_ctrl,
}

//
// struct gsc_scaler - the configuration data for G-Scaler inetrnal scaler
// @pre_shfactor:	pre sclaer shift factor
// @pre_hratio:		horizontal ratio of the prescaler
// @pre_vratio:		vertical ratio of the prescaler
// @main_hratio:	the main scaler's horizontal ratio
// @main_vratio:	the main scaler's vertical ratio
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_scaler {
    pub pre_shfactor: u32,
    pub pre_hratio: u32,
    pub pre_vratio: u32,
    pub main_hratio: u32,
    pub main_vratio: u32,
}

//
// struct gsc_frame - source/target frame properties
// @f_width:	SRC : SRCIMG_WIDTH, DST : OUTPUTDMA_WHOLE_IMG_WIDTH
// @f_height:	SRC : SRCIMG_HEIGHT, DST : OUTPUTDMA_WHOLE_IMG_HEIGHT
// @crop:	cropped(source)/scaled(destination) size
// @payload:	image size in bytes (w x h x bpp)
// @addr:	image frame buffer physical addresses
// @fmt:	G-Scaler color format pointer
// @colorspace: value indicating v4l2_colorspace
// @alpha:	frame's alpha value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_frame {
    pub f_width: u32,
    pub f_height: u32,
    pub crop: v4l2_rect,
    pub payload: [c_ulong; VIDEO_MAX_PLANES],
    pub addr: gsc_addr,
    pub fmt: *const gsc_fmt,
    pub colorspace: u32,
    pub alpha: u8,
}

//
// struct gsc_m2m_device - v4l2 memory-to-memory device data
// @vfd: the video device node for v4l2 m2m mode
// @m2m_dev: v4l2 memory-to-memory device data
// @ctx: hardware context data
// @refcnt: the reference counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_m2m_device {
    pub vfd: *mut video_device,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub ctx: *mut gsc_ctx,
    pub refcnt: c_int,
}

//
// struct gsc_pix_max - image pixel size limits in various IP configurations
//
// @org_scaler_bypass_w: max pixel width when the scaler is disabled
// @org_scaler_bypass_h: max pixel height when the scaler is disabled
// @org_scaler_input_w: max pixel width when the scaler is enabled
// @org_scaler_input_h: max pixel height when the scaler is enabled
// @real_rot_dis_w: max pixel src cropped height with the rotator is off
// @real_rot_dis_h: max pixel src cropped width with the rotator is off
// @real_rot_en_w: max pixel src cropped width with the rotator is on
// @real_rot_en_h: max pixel src cropped height with the rotator is on
// @target_rot_dis_w: max pixel dst scaled width with the rotator is off
// @target_rot_dis_h: max pixel dst scaled height with the rotator is off
// @target_rot_en_w: max pixel dst scaled width with the rotator is on
// @target_rot_en_h: max pixel dst scaled height with the rotator is on
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_pix_max {
    pub org_scaler_bypass_w: u16,
    pub org_scaler_bypass_h: u16,
    pub org_scaler_input_w: u16,
    pub org_scaler_input_h: u16,
    pub real_rot_dis_w: u16,
    pub real_rot_dis_h: u16,
    pub real_rot_en_w: u16,
    pub real_rot_en_h: u16,
    pub target_rot_dis_w: u16,
    pub target_rot_dis_h: u16,
    pub target_rot_en_w: u16,
    pub target_rot_en_h: u16,
}

//
// struct gsc_pix_min - image pixel size limits in various IP configurations
//
// @org_w: minimum source pixel width
// @org_h: minimum source pixel height
// @real_w: minimum input crop pixel width
// @real_h: minimum input crop pixel height
// @target_rot_dis_w: minimum output scaled pixel height when rotator is off
// @target_rot_dis_h: minimum output scaled pixel height when rotator is off
// @target_rot_en_w: minimum output scaled pixel height when rotator is on
// @target_rot_en_h: minimum output scaled pixel height when rotator is on
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_pix_min {
    pub org_w: u16,
    pub org_h: u16,
    pub real_w: u16,
    pub real_h: u16,
    pub target_rot_dis_w: u16,
    pub target_rot_dis_h: u16,
    pub target_rot_en_w: u16,
    pub target_rot_en_h: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_pix_align {
    pub org_h: u16,
    pub org_w: u16,
    pub offset_h: u16,
    pub real_w: u16,
    pub real_h: u16,
    pub target_w: u16,
    pub target_h: u16,
}

//
// struct gsc_variant - G-Scaler variant information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_variant {
    pub pix_max: *mut gsc_pix_max,
    pub pix_min: *mut gsc_pix_min,
    pub pix_align: *mut gsc_pix_align,
    pub in_buf_cnt: u16,
    pub out_buf_cnt: u16,
    pub sc_up_max: u16,
    pub sc_down_max: u16,
    pub poly_sc_down_max: u16,
    pub pre_sc_down_max: u16,
    pub local_sc_down: u16,
}

//
// struct gsc_driverdata - per device type driver data for init time.
//
// @variant: the variant information for this driver.
// @num_entities: the number of g-scalers
// @clk_names: clock names
// @num_clocks: the number of clocks in @clk_names
// @num_entities: the number of g-scalers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_driverdata {
    pub variant: [*mut gsc_variant; GSC_MAX_DEVS],
    pub clk_names: [*const c_char; GSC_MAX_CLOCKS],
    pub num_clocks: c_int,
    pub num_entities: c_int,
}

//
// struct gsc_dev - abstraction for G-Scaler entity
// @slock:	the spinlock protecting this data structure
// @lock:	the mutex protecting this data structure
// @pdev:	pointer to the G-Scaler platform device
// @variant:	the IP variant information
// @id:		G-Scaler device index (0..GSC_MAX_DEVS)
// @num_clocks:	number of clocks required for G-Scaler operation
// @clock:	clocks required for G-Scaler operation
// @regs:	the mapped hardware registers
// @irq_queue:	interrupt handler waitqueue
// @m2m:	memory-to-memory V4L2 device information
// @state:	flags used to synchronize m2m and capture mode operation
// @vdev:	video device for G-Scaler instance
// @v4l2_dev:	v4l2_device for G-Scaler instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_dev {
    pub slock: spinlock_t,
    pub lock: mutex,
    pub pdev: *mut platform_device,
    pub variant: *mut gsc_variant,
    pub id: u16,
    pub num_clocks: c_int,
    pub clock: [*mut clk; GSC_MAX_CLOCKS],
    pub regs: *mut void __iomem,
    pub irq_queue: wait_queue_head_t,
    pub m2m: gsc_m2m_device,
    pub state: c_ulong,
    pub vdev: video_device,
    pub v4l2_dev: v4l2_device,
}

//
// struct gsc_ctx - the device context data
// @s_frame:		source frame properties
// @d_frame:		destination frame properties
// @in_path:		input mode (DMA or camera)
// @out_path:		output mode (DMA or FIFO)
// @scaler:		image scaler properties
// @flags:		additional flags for image conversion
// @state:		flags to keep track of user configuration
// @rotation:		rotation
// @hflip:		horizontal flip
// @vflip:		vertical flip
// @gsc_dev:		the G-Scaler device this context applies to
// @m2m_ctx:		memory-to-memory device context
// @fh:                 v4l2 file handle
// @ctrl_handler:       v4l2 controls handler
// @gsc_ctrls:		G-Scaler control set
// @ctrls_rdy:          true if the control handler is initialized
// @out_colorspace:     the colorspace of the OUTPUT queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_ctx {
    pub s_frame: gsc_frame,
    pub d_frame: gsc_frame,
    pub in_path: gsc_datapath,
    pub out_path: gsc_datapath,
    pub scaler: gsc_scaler,
    pub flags: u32,
    pub state: u32,
    pub rotation: c_int,
    pub hflip:1: c_uint,
    pub vflip:1: c_uint,
    pub gsc_dev: *mut gsc_dev,
    pub m2m_ctx: *mut v4l2_m2m_ctx,
    pub fh: v4l2_fh,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub gsc_ctrls: gsc_ctrls,
    pub ctrls_rdy: bool,
    pub out_colorspace: v4l2_colorspace,
}

extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), gsc_ctx: struct, _arg: fh) -> return;
}
extern "C" {
    pub fn gsc_set_prefbuf(gsc: *mut gsc_dev, frm: *mut gsc_frame);
}
extern "C" {
    pub fn gsc_register_m2m_device(gsc: *mut gsc_dev) -> c_int;
}
extern "C" {
    pub fn gsc_unregister_m2m_device(gsc: *mut gsc_dev);
}
extern "C" {
    pub fn gsc_m2m_job_finish(ctx: *mut gsc_ctx, vb_state: c_int);
}
extern "C" {
    pub fn get_plane_size(fr: *mut gsc_frame, plane: c_uint) -> u32;
}
extern "C" {
    pub fn gsc_enum_fmt(f: *mut v4l2_fmtdesc) -> c_int;
}
extern "C" {
    pub fn gsc_try_fmt_mplane(ctx: *mut gsc_ctx, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn gsc_set_frame_size(frame: *mut gsc_frame, width: c_int, height: c_int);
}
extern "C" {
    pub fn gsc_g_fmt_mplane(ctx: *mut gsc_ctx, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn gsc_check_crop_change(tmp_w: u32, tmp_h: u32, w: *mut u32, h: *mut u32);
}
extern "C" {
    pub fn gsc_try_selection(ctx: *mut gsc_ctx, s: *mut v4l2_selection) -> c_int;
}
extern "C" {
    pub fn gsc_get_prescaler_shfactor(hratio: u32, vratio: u32, sh: *mut u32);
}
extern "C" {
    pub fn gsc_set_scaler_info(ctx: *mut gsc_ctx) -> c_int;
}
extern "C" {
    pub fn gsc_ctrls_create(ctx: *mut gsc_ctx) -> c_int;
}
extern "C" {
    pub fn gsc_ctrls_delete(ctx: *mut gsc_ctx);
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn gsc_hw_set_sw_reset(dev: *mut gsc_dev);
}
extern "C" {
    pub fn gsc_wait_reset(dev: *mut gsc_dev) -> c_int;
}
extern "C" {
    pub fn gsc_hw_set_frm_done_irq_mask(dev: *mut gsc_dev, mask: bool);
}
extern "C" {
    pub fn gsc_hw_set_gsc_irq_enable(dev: *mut gsc_dev, mask: bool);
}
extern "C" {
    pub fn gsc_hw_set_input_buf_masking(dev: *mut gsc_dev, shift: u32, enable: bool);
}
extern "C" {
    pub fn gsc_hw_set_output_buf_masking(dev: *mut gsc_dev, shift: u32, enable: bool);
}
extern "C" {
    pub fn gsc_hw_set_input_path(ctx: *mut gsc_ctx);
}
extern "C" {
    pub fn gsc_hw_set_in_size(ctx: *mut gsc_ctx);
}
extern "C" {
    pub fn gsc_hw_set_in_image_rgb(ctx: *mut gsc_ctx);
}
extern "C" {
    pub fn gsc_hw_set_in_image_format(ctx: *mut gsc_ctx);
}
extern "C" {
    pub fn gsc_hw_set_output_path(ctx: *mut gsc_ctx);
}
extern "C" {
    pub fn gsc_hw_set_out_size(ctx: *mut gsc_ctx);
}
extern "C" {
    pub fn gsc_hw_set_out_image_rgb(ctx: *mut gsc_ctx);
}
extern "C" {
    pub fn gsc_hw_set_out_image_format(ctx: *mut gsc_ctx);
}
extern "C" {
    pub fn gsc_hw_set_prescaler(ctx: *mut gsc_ctx);
}
extern "C" {
    pub fn gsc_hw_set_mainscaler(ctx: *mut gsc_ctx);
}
extern "C" {
    pub fn gsc_hw_set_rotation(ctx: *mut gsc_ctx);
}
extern "C" {
    pub fn gsc_hw_set_global_alpha(ctx: *mut gsc_ctx);
}
extern "C" {
    pub fn gsc_hw_set_sfr_update(ctx: *mut gsc_ctx);
}
