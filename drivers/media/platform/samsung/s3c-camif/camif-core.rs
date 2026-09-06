//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s3c-camif/camif-core.h
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
// s3c24xx/s3c64xx SoC series Camera Interface (CAMIF) driver
//
// Copyright (C) 2012 Sylwester Nawrocki <sylvester.nawrocki@gmail.com>
// Copyright (C) 2012 Tomasz Figa <tomasz.figa@gmail.com>
//

pub const CAMIF_REQ_BUFS_MIN: c_int = 3;
pub const CAMIF_MAX_OUT_BUFS: c_int = 4;
pub const CAMIF_MAX_PIX_WIDTH: c_int = 4096;
pub const CAMIF_MAX_PIX_HEIGHT: c_int = 4096;
pub const SCALER_MAX_RATIO: c_int = 64;
pub const CAMIF_DEF_WIDTH: c_int = 640;
pub const CAMIF_DEF_HEIGHT: c_int = 480;

pub const S3C244X_CAMIF_IP_REV: c_uint = 0x20 /* 2.0 */;
pub const S3C2450_CAMIF_IP_REV: c_uint = 0x30 /* 3.0 - not implemented, not tested */;
pub const S3C6400_CAMIF_IP_REV: c_uint = 0x31 /* 3.1 - not implemented, not tested */;
pub const S3C6410_CAMIF_IP_REV: c_uint = 0x32 /* 3.2 */;
// struct camif_vp::state

pub const CAMIF_SD_PAD_SINK: c_int = 0;
pub const CAMIF_SD_PAD_SOURCE_C: c_int = 1;
pub const CAMIF_SD_PAD_SOURCE_P: c_int = 2;
pub const CAMIF_SD_PADS_NUM: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum img_fmt {
    IMG_FMT_RGB565 = 0x0010,
    IMG_FMT_RGB666,
    IMG_FMT_XRGB8888,
    IMG_FMT_YCBCR420 = 0x0020,
    IMG_FMT_YCRCB420,
    IMG_FMT_YCBCR422P,
    IMG_FMT_YCBYCR422 = 0x0040,
    IMG_FMT_YCRYCB422,
    IMG_FMT_CBYCRY422,
    IMG_FMT_CRYCBY422,
}

// Possible values for struct camif_fmt::flags

//
// struct camif_fmt - pixel format description
// @fourcc:    fourcc code for this format, 0 if not applicable
// @color:     a corresponding enum img_fmt
// @colplanes: number of physically contiguous data planes
// @flags:     indicate for which SoCs revisions this format is valid
// @depth:     bits per pixel (total)
// @ybpp:      number of luminance bytes per pixel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct camif_fmt {
    pub fourcc: u32,
    pub color: u32,
    pub colplanes: u16,
    pub flags: u16,
    pub depth: u8,
    pub ybpp: u8,
}

//
// struct camif_dma_offset - pixel offset information for DMA
// @initial: offset (in pixels) to first pixel
// @line: offset (in pixels) from end of line to start of next line
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct camif_dma_offset {
    pub initial: c_int,
    pub line: c_int,
}

//
// struct camif_frame - source/target frame properties
// @f_width: full pixel width
// @f_height: full pixel height
// @rect: crop/composition rectangle
// @dma_offset: DMA offset configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct camif_frame {
    pub f_width: u16,
    pub f_height: u16,
    pub rect: v4l2_rect,
    pub dma_offset: camif_dma_offset,
}

// CAMIF clocks enumeration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp_pix_limits {
    pub max_out_width: u16,
    pub max_sc_out_width: u16,
    pub out_width_align: u16,
    pub max_height: u16,
    pub min_out_width: u8,
    pub out_hor_offset_align: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct camif_pix_limits {
    pub win_hor_offset_align: u16,
}

//
// struct s3c_camif_variant - CAMIF variant structure
// @vp_pix_limits:    pixel limits for the codec and preview paths
// @pix_limits:       pixel limits for the camera input interface
// @ip_revision:      the CAMIF IP revision: 0x20 for s3c244x, 0x32 for s3c6410
// @has_img_effect:   supports image effects
// @vp_offset:        register offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3c_camif_variant {
    pub vp_pix_limits: [vp_pix_limits; 2],
    pub pix_limits: camif_pix_limits,
    pub ip_revision: u8,
    pub has_img_effect: u8,
    pub vp_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3c_camif_drvdata {
    pub variant: *const s3c_camif_variant,
    pub bus_clk_freq: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct camif_scaler {
    pub scaleup_h: u8,
    pub scaleup_v: u8,
    pub copy: u8,
    pub enable: u8,
    pub h_shift: u32,
    pub v_shift: u32,
    pub pre_h_ratio: u32,
    pub pre_v_ratio: u32,
    pub pre_dst_width: u32,
    pub pre_dst_height: u32,
    pub main_h_ratio: u32,
    pub main_v_ratio: u32,
}

//
// struct camif_vp - CAMIF data processing path structure (codec/preview)
// @irq_queue:	    interrupt handling waitqueue
// @irq:	    interrupt number for this data path
// @camif:	    pointer to the camif structure
// @pad:	    media pad for the video node
// @vdev:           video device
// @ctrl_handler:   video node controls handler
// @owner:	    file handle that own the streaming
// @vb_queue:       vb2 buffer queue
// @pending_buf_q:  pending (empty) buffers queue head
// @active_buf_q:   active (being written) buffers queue head
// @active_buffers: counter of buffer set up at the DMA engine
// @buf_index:	    identifier of a last empty buffer set up in H/W
// @frame_sequence: image frame sequence counter
// @reqbufs_count:  the number of buffers requested
// @scaler:	    the scaler structure
// @out_fmt:	    pixel format at this video path output
// @payload:	    the output data frame payload size
// @out_frame:	    the output pixel resolution
// @state:	    the video path's state
// @fmt_flags:	    flags determining supported pixel formats
// @id:		    CAMIF id, 0 - codec, 1 - preview
// @rotation:	    current image rotation value
// @hflip:	    apply horizontal flip if set
// @vflip:	    apply vertical flip if set
// @offset:	    register offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct camif_vp {
    pub irq_queue: wait_queue_head_t,
    pub irq: c_int,
    pub camif: *mut camif_dev,
    pub pad: media_pad,
    pub vdev: video_device,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub owner: *mut v4l2_fh,
    pub vb_queue: vb2_queue,
    pub pending_buf_q: list_head,
    pub active_buf_q: list_head,
    pub active_buffers: c_uint,
    pub buf_index: c_uint,
    pub frame_sequence: c_uint,
    pub reqbufs_count: c_uint,
    pub scaler: camif_scaler,
    pub out_fmt: *const camif_fmt,
    pub payload: c_uint,
    pub out_frame: camif_frame,
    pub state: c_uint,
    pub fmt_flags: u16,
    pub id: u8,
    pub rotation: u16,
    pub hflip: u8,
    pub vflip: u8,
    pub offset: c_uint,
}

// Video processing path enumeration
pub const VP_CODEC: c_int = 0;
pub const VP_PREVIEW: c_int = 1;
pub const CAMIF_VP_NUM: c_int = 2;
//
// struct camif_dev - the CAMIF driver private data structure
// @media_dev:    top-level media device structure
// @v4l2_dev:	  root v4l2_device
// @subdev:       camera interface ("catchcam") subdev
// @mbus_fmt:	  camera input media bus format
// @camif_crop:   camera input interface crop rectangle
// @pads:	  the camif subdev's media pads
// @stream_count: the camera interface streaming reference counter
// @sensor:       image sensor data structure
// @m_pipeline:	  video entity pipeline description
// @ctrl_handler: v4l2 control handler (owned by @subdev)
// @ctrl_test_pattern: V4L2_CID_TEST_PATTERN control
// @ctrl_colorfx: V4L2_CID_COLORFX control
// @ctrl_colorfx_cbcr:  V4L2_CID_COLORFX_CBCR control
// @test_pattern: test pattern
// @colorfx:	  color effect
// @colorfx_cb:   Cb value for V4L2_COLORFX_SET_CBCR
// @colorfx_cr:   Cr value for V4L2_COLORFX_SET_CBCR
// @vp:           video path (DMA) description (codec/preview)
// @variant:      variant information for this device
// @dev:	  pointer to the CAMIF device struct
// @pdata:	  a copy of the driver's platform data
// @clock:	  clocks required for the CAMIF operation
// @lock:	  mutex protecting this data structure
// @slock:	  spinlock protecting CAMIF registers
// @io_base:	  start address of the mmapped CAMIF registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct camif_dev {
    pub media_dev: media_device,
    pub v4l2_dev: v4l2_device,
    pub subdev: v4l2_subdev,
    pub mbus_fmt: v4l2_mbus_framefmt,
    pub camif_crop: v4l2_rect,
    pub pads: [media_pad; CAMIF_SD_PADS_NUM],
    pub stream_count: c_int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cam_sensor {
    pub sd: *mut v4l2_subdev,
    pub power_count: c_short,
    pub stream_count: c_short,
    pub sensor: },
    pub m_pipeline: *mut media_pipeline,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub ctrl_test_pattern: *mut v4l2_ctrl,
    pub ctrl_colorfx: *mut v4l2_ctrl,
    pub ctrl_colorfx_cbcr: *mut v4l2_ctrl,
}

//
// struct camif_addr - Y/Cb/Cr DMA start address structure
// @y:	 luminance plane dma address
// @cb:	 Cb plane dma address
// @cr:	 Cr plane dma address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct camif_addr {
    pub y: dma_addr_t,
    pub cb: dma_addr_t,
    pub cr: dma_addr_t,
}

//
// struct camif_buffer - the camif video buffer structure
// @vb:    vb2 buffer
// @list:  list head for the buffers queue
// @paddr: DMA start addresses
// @index: an identifier of this buffer at the DMA engine
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct camif_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub paddr: camif_addr,
    pub index: c_uint,
}

extern "C" {
    pub fn s3c_camif_register_video_node(camif: *mut camif_dev, idx: c_int) -> c_int;
}
extern "C" {
    pub fn s3c_camif_unregister_video_node(camif: *mut camif_dev, idx: c_int);
}
extern "C" {
    pub fn s3c_camif_irq_handler(irq: c_int, priv: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn s3c_camif_create_subdev(camif: *mut camif_dev) -> c_int;
}
extern "C" {
    pub fn s3c_camif_unregister_subdev(camif: *mut camif_dev);
}
extern "C" {
    pub fn s3c_camif_set_defaults(camif: *mut camif_dev) -> c_int;
}
