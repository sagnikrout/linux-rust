//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/vpe/vip.h
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
// TI VIP capture driver
//
// Copyright (C) 2025 Texas Instruments Incorpated - http://www.ti.com
// David Griego, <dagriego@biglakesoftware.com>
// Dale Farnsworth, <dale@farnsworth.org>
// Yemike Abhilash Chandra, <y-abhilashchandra@ti.com>
//

pub const VIP_INSTANCE1: c_int = 1;
pub const VIP_INSTANCE2: c_int = 2;
pub const VIP_INSTANCE3: c_int = 3;
pub const VIP_SLICE1: c_int = 0;
pub const VIP_SLICE2: c_int = 1;
pub const VIP_NUM_SLICES: c_int = 2;
//
// Additional client identifiers used for VPDMA configuration descriptors
//
pub const VIP_SLICE1_CFD_SC_CLIENT: c_int = 7;
pub const VIP_SLICE2_CFD_SC_CLIENT: c_int = 8;
pub const VIP_PORTA: c_int = 0;
pub const VIP_PORTB: c_int = 1;
pub const VIP_NUM_PORTS: c_int = 2;
pub const VIP_MAX_PLANES: c_int = 2;
pub const VIP_LUMA: c_int = 0;
pub const VIP_CHROMA: c_int = 1;
pub const VIP_CAP_STREAMS_PER_PORT: c_int = 16;
pub const VIP_VBI_STREAMS_PER_PORT: c_int = 16;
pub const VIP_MAX_SUBDEV: c_int = 5;

//
// This value needs to be at least as large as the number of entry in
// vip_formats[].
// When vip_formats[] is modified make sure to adjust this value also.
//
pub const VIP_MAX_ACTIVE_FMT: c_int = 16;
//
// Colorspace conversion unit can be in one of 3 modes:
// NA  - Not Available on this port
// Y2R - Needed for YUV to RGB on this port
// R2Y - Needed for RGB to YUV on this port
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vip_csc_state {
    VIP_CSC_NA = 0,
    VIP_CSC_Y2R,
    VIP_CSC_R2Y,
}

// buffer for one video frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vip_buffer {
// common v4l buffer stuff
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub drop: bool,
}

//
// struct vip_fmt - VIP media bus format information
// @fourcc: V4L2 pixel format FCC identifier
// @code: V4L2 media bus format code
// @colorspace: V4L2 colorspace identifier
// @coplanar: 1 if unpacked Luma and Chroma, 0 otherwise (packed/interleaved)
// @vpdma_fmt: VPDMA data format per plane.
// @finfo: Cache v4l2_format_info for associated fourcc
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vip_fmt {
    pub fourcc: u32,
    pub code: u32,
    pub colorspace: u32,
    pub coplanar: u8,
    pub vpdma_fmt: [*const vpdma_data_format; VIP_MAX_PLANES],
    pub finfo: *const v4l2_format_info,
}

//
// The vip_parser_data structures contains the memory mapped
// info to access the parser registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vip_parser_data {
    pub base: *mut void __iomem,
    pub pdev: *mut platform_device,
}

//
// The vip_shared structure contains data that is shared by both
// the VIP1 and VIP2 slices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vip_shared {
    pub list: list_head,
    pub base: *mut void __iomem,
    pub vpdma_data: vpdma_data,
    pub vpdma: *mut vpdma_data,
    pub v4l2_dev: v4l2_device,
    pub devs: [*mut vip_dev; VIP_NUM_SLICES],
    pub ctrl_handler: v4l2_ctrl_handler,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vip_ctrl_module {
    pub syscon_pol: *mut regmap,
    pub syscon_offset: u32,
    pub syscon_bit_field: [u32; 4],
}

//
// There are two vip_dev structure, one for each vip slice: VIP1 & VIP2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vip_dev {
    pub v4l2_dev: v4l2_device,
    pub pdev: *mut platform_device,
    pub shared: *mut vip_shared,
    pub syscon: *mut vip_ctrl_module,
    pub instance_id: c_int,
    pub slice_id: c_int,
    pub /: *mut *mut int num_ports; / count of open ports,
    pub mutex: mutex,
// protects access to stream buffer queues
    pub slock: spinlock_t,
    pub irq: c_int,
    pub base: *mut void __iomem,
    pub ports: [*mut vip_port; VIP_NUM_PORTS],
    pub name: [c_char; 16],
// parser data handle
    pub parser: *mut vip_parser_data,
// scaler data handle
    pub sc: *mut sc_data,
// scaler port assignation
    pub sc_assigned: c_int,
// csc data handle
    pub csc: *mut csc_data,
// csc port assignation
    pub csc_assigned: c_int,
}

//
// There are two vip_port structures for each vip_dev, one for port A
// and one for port B.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vip_port {
    pub dev: *mut vip_dev,
    pub port_id: c_int,
    pub flags: c_uint,
    pub /: *mut *mut v4l2_rect c_rect; / crop rectangle,
    pub mbus_framefmt: v4l2_mbus_framefmt,
    pub try_mbus_framefmt: v4l2_mbus_framefmt,
    pub name: [c_char; 16],
    pub /: *mut *mut *mut vip_fmt fmt; / current format info,
// Number of channels/streams configured
    pub num_streams_configured: c_int,
    pub /: *mut *mut int num_streams; / count of open streams,
    pub cap_streams: [*mut vip_stream; VIP_CAP_STREAMS_PER_PORT],
    pub notifier: v4l2_async_notifier,
    pub subdev: *mut v4l2_subdev,
    pub endpoint: v4l2_fwnode_endpoint,
    pub active_fmt: [*mut vip_fmt; VIP_MAX_ACTIVE_FMT],
    pub num_active_fmt: c_int,
// have new shadow reg values
    pub load_mmrs: bool,
// shadow reg addr/data block
    pub mmr_adb: vpdma_buf,
// h coeff buffer
    pub sc_coeff_h: vpdma_buf,
// v coeff buffer
    pub sc_coeff_v: vpdma_buf,
// Show if scaler resource is available on this port
    pub scaler: bool,
// Show the csc resource state on this port
    pub csc: vip_csc_state,
}

//
// When handling multiplexed video, there can be multiple streams for each
// port.  The vip_stream structure holds per-stream data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vip_stream {
    pub vfd: *mut video_device,
    pub port: *mut vip_port,
    pub stream_id: c_int,
    pub list_num: c_int,
    pub vfl_type: c_int,
    pub name: [c_char; 16],
    pub recovery_work: work_struct,
    pub num_recovery: c_int,
    pub /: *mut *mut v4l2_field field; / current field,
    pub /: *mut *mut unsigned int sequence; / current frame/field seq,
    pub /: *mut *mut v4l2_field sup_field; / supported field value,
    pub /: *mut *mut unsigned int width; / frame width,
    pub /: *mut *mut unsigned int height; / frame height,
    pub /: *mut *mut unsigned int bytesperline; / bytes per line in memory,
    pub /: *mut *mut unsigned int sizeimage; / image size in memory,
    pub /: *mut *mut list_head vidq; / incoming vip_bufs queue,
    pub /: *mut *mut list_head dropq; / drop vip_bufs queue,
    pub /: *mut *mut list_head post_bufs; / vip_bufs to be DMAed,
// Maintain a list of used channels - Needed for VPDMA cleanup
    pub vpdma_channels: [c_int; VPDMA_MAX_CHANNELS],
    pub vpdma_channels_to_abort: [c_int; VPDMA_MAX_CHANNELS],
    pub /: *mut *mut vpdma_desc_list desc_list; / DMA descriptor list,
    pub write_desc: *mut vpdma_dtd,
// next unused desc_list addr
    pub desc_next: *mut c_void,
    pub vb_vidq: vb2_queue,
}

//
// VIP Enumerations
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum data_path_select {
    ALL_FIELDS_DATA_SELECT = 0,
    VIP_CSC_SRC_DATA_SELECT,
    VIP_SC_SRC_DATA_SELECT,
    VIP_RGB_SRC_DATA_SELECT,
    VIP_RGB_OUT_LO_DATA_SELECT,
    VIP_RGB_OUT_HI_DATA_SELECT,
    VIP_CHR_DS_1_SRC_DATA_SELECT,
    VIP_CHR_DS_2_SRC_DATA_SELECT,
    VIP_MULTI_CHANNEL_DATA_SELECT,
    VIP_CHR_DS_1_DATA_BYPASS,
    VIP_CHR_DS_2_DATA_BYPASS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum data_interface_modes {
    SINGLE_24B_INTERFACE = 0,
    SINGLE_16B_INTERFACE = 1,
    DUAL_8B_INTERFACE = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sync_types {
    EMBEDDED_SYNC_SINGLE_YUV422 = 0,
    EMBEDDED_SYNC_2X_MULTIPLEXED_YUV422 = 1,
    EMBEDDED_SYNC_4X_MULTIPLEXED_YUV422 = 2,
    EMBEDDED_SYNC_LINE_MULTIPLEXED_YUV422 = 3,
    DISCRETE_SYNC_SINGLE_YUV422 = 4,
    EMBEDDED_SYNC_SINGLE_RGB_OR_YUV444 = 5,
    DISCRETE_SYNC_SINGLE_RGB_24B = 10,
}

//
// Register offsets and field selectors
//
pub const VIP_PID_FUNC: c_uint = 0xf02;
pub const VIP_PID: c_uint = 0x0000;
pub const VIP_PID_MINOR_MASK: c_uint = 0x3f;
pub const VIP_PID_MINOR_SHIFT: c_int = 0;
pub const VIP_PID_CUSTOM_MASK: c_uint = 0x03;
pub const VIP_PID_CUSTOM_SHIFT: c_int = 6;
pub const VIP_PID_MAJOR_MASK: c_uint = 0x07;
pub const VIP_PID_MAJOR_SHIFT: c_int = 8;
pub const VIP_PID_RTL_MASK: c_uint = 0x1f;
pub const VIP_PID_RTL_SHIFT: c_int = 11;
pub const VIP_PID_FUNC_MASK: c_uint = 0xfff;
pub const VIP_PID_FUNC_SHIFT: c_int = 16;
pub const VIP_PID_SCHEME_MASK: c_uint = 0x03;
pub const VIP_PID_SCHEME_SHIFT: c_int = 30;
pub const VIP_SYSCONFIG: c_uint = 0x0010;
pub const VIP_SYSCONFIG_IDLE_MASK: c_uint = 0x03;
pub const VIP_SYSCONFIG_IDLE_SHIFT: c_int = 2;
pub const VIP_SYSCONFIG_STANDBY_MASK: c_uint = 0x03;
pub const VIP_SYSCONFIG_STANDBY_SHIFT: c_int = 4;
pub const VIP_FORCE_IDLE_MODE: c_int = 0;
pub const VIP_NO_IDLE_MODE: c_int = 1;
pub const VIP_SMART_IDLE_MODE: c_int = 2;
pub const VIP_SMART_IDLE_WAKEUP_MODE: c_int = 3;
pub const VIP_FORCE_STANDBY_MODE: c_int = 0;
pub const VIP_NO_STANDBY_MODE: c_int = 1;
pub const VIP_SMART_STANDBY_MODE: c_int = 2;
pub const VIP_SMART_STANDBY_WAKEUP_MODE: c_int = 3;
pub const VIP_INTC_INTX_OFFSET: c_uint = 0x0020;
pub const VIP_INT0_STATUS0_RAW_SET: c_uint = 0x0020;

pub const VIP_INT0_STATUS0_CLR: c_uint = 0x0028;

pub const VIP_INT0_ENABLE0_SET: c_uint = 0x0030;

pub const VIP_INT0_ENABLE0_CLR: c_uint = 0x0038;

pub const VIP_INT0_STATUS1_RAW_SET: c_uint = 0x0024;

pub const VIP_INT0_STATUS1_CLR: c_uint = 0x002c;

pub const VIP_INT0_ENABLE1_SET: c_uint = 0x0034;

pub const VIP_INT0_ENABLE1_CLR: c_uint = 0x003c;
pub const VIP_INT0_ENABLE1_STAT: c_uint = 0x004c;

pub const VIP_INTC_E0I: c_uint = 0x00a0;
pub const VIP_CLK_ENABLE: c_uint = 0x0100;

pub const VIP_CLK_RESET: c_uint = 0x0104;

pub const VIP_VPDMA_CLK_RESET_MASK: c_uint = 0x1;
pub const VIP_VPDMA_CLK_RESET_SHIFT: c_int = 0;
pub const VIP_DATA_PATH_CLK_RESET_MASK: c_uint = 0x1;
pub const VIP_VIP1_DATA_PATH_RESET_SHIFT: c_int = 16;
pub const VIP_VIP2_DATA_PATH_RESET_SHIFT: c_int = 17;

pub const VIP_VIP1_DATA_PATH_SELECT: c_uint = 0x010c;
pub const VIP_VIP2_DATA_PATH_SELECT: c_uint = 0x0110;
pub const VIP_CSC_SRC_SELECT_MASK: c_uint = 0x07;
pub const VIP_CSC_SRC_SELECT_SHFT: c_int = 0;
pub const VIP_SC_SRC_SELECT_MASK: c_uint = 0x07;
pub const VIP_SC_SRC_SELECT_SHFT: c_int = 3;

pub const VIP_DS1_SRC_SELECT_MASK: c_uint = 0x07;
pub const VIP_DS1_SRC_SELECT_SHFT: c_int = 9;
pub const VIP_DS2_SRC_SELECT_MASK: c_uint = 0x07;
pub const VIP_DS2_SRC_SELECT_SHFT: c_int = 12;

pub const VIP_DATAPATH_SELECT_MASK: c_uint = 0x0f;
pub const VIP_DATAPATH_SELECT_SHFT: c_int = 28;
pub const VIP_PARSER_MAIN_CFG: c_uint = 0x0000;
pub const VIP_DATA_INTERFACE_MODE_MASK: c_uint = 0x03;
pub const VIP_DATA_INTERFACE_MODE_SHFT: c_int = 0;

pub const VIP_SLICE0_PARSER: c_uint = 0x5500;
pub const VIP_SLICE1_PARSER: c_uint = 0x5a00;
pub const VIP_PARSER_PORTA_0: c_uint = 0x0004;
pub const VIP_PARSER_PORTB_0: c_uint = 0x000c;
pub const VIP_SYNC_TYPE_MASK: c_uint = 0x0f;
pub const VIP_SYNC_TYPE_SHFT: c_int = 0;
pub const VIP_CTRL_CHANNEL_SEL_MASK: c_uint = 0x03;
pub const VIP_CTRL_CHANNEL_SEL_SHFT: c_int = 4;

pub const VIP_FID_SKEW_PRECOUNT_MASK: c_uint = 0x3f;
pub const VIP_FID_SKEW_PRECOUNT_SHFT: c_int = 16;

pub const VIP_FID_SKEW_POSTCOUNT_MASK: c_uint = 0x3f;
pub const VIP_FID_SKEW_POSTCOUNT_SHFT: c_int = 24;

pub const VIP_PARSER_PORTA_1: c_uint = 0x0008;
pub const VIP_PARSER_PORTB_1: c_uint = 0x0010;
pub const VIP_SRC0_NUMLINES_MASK: c_uint = 0x0fff;
pub const VIP_SRC0_NUMLINES_SHFT: c_int = 0;
pub const VIP_ANC_CHAN_SEL_8B_MASK: c_uint = 0x03;
pub const VIP_ANC_CHAN_SEL_8B_SHFT: c_int = 13;
pub const VIP_SRC0_NUMPIX_MASK: c_uint = 0x0fff;
pub const VIP_SRC0_NUMPIX_SHFT: c_int = 16;
pub const VIP_REPACK_SEL_MASK: c_uint = 0x07;
pub const VIP_REPACK_SEL_SHFT: c_int = 28;
pub const VIP_PARSER_FIQ_MASK: c_uint = 0x0014;
pub const VIP_PARSER_FIQ_CLR: c_uint = 0x0018;
pub const VIP_PARSER_FIQ_STATUS: c_uint = 0x001c;

pub const VIP_PARSER_PORTA_SOURCE_FID: c_uint = 0x0020;
pub const VIP_PARSER_PORTA_ENCODER_FID: c_uint = 0x0024;
pub const VIP_PARSER_PORTB_SOURCE_FID: c_uint = 0x0028;
pub const VIP_PARSER_PORTB_ENCODER_FID: c_uint = 0x002c;
pub const VIP_PARSER_PORTA_SRC0_SIZE: c_uint = 0x0030;
pub const VIP_PARSER_PORTB_SRC0_SIZE: c_uint = 0x0070;
pub const VIP_SOURCE_HEIGHT_MASK: c_uint = 0x0fff;
pub const VIP_SOURCE_HEIGHT_SHFT: c_int = 0;
pub const VIP_SOURCE_WIDTH_MASK: c_uint = 0x0fff;
pub const VIP_SOURCE_WIDTH_SHFT: c_int = 16;
pub const VIP_PARSER_PORTA_VDET_VEC: c_uint = 0x00b0;
pub const VIP_PARSER_PORTB_VDET_VEC: c_uint = 0x00b4;
pub const VIP_PARSER_PORTA_EXTRA2: c_uint = 0x00b8;
pub const VIP_PARSER_PORTB_EXTRA2: c_uint = 0x00c8;
pub const VIP_ANC_SKIP_NUMPIX_MASK: c_uint = 0x0fff;
pub const VIP_ANC_SKIP_NUMPIX_SHFT: c_int = 0;

pub const VIP_ANC_USE_NUMPIX_MASK: c_uint = 0x0fff;
pub const VIP_ANC_USE_NUMPIX_SHFT: c_int = 16;
pub const VIP_ANC_TARGET_SRCNUM_MASK: c_uint = 0x0f;
pub const VIP_ANC_TARGET_SRCNUM_SHFT: c_int = 28;
pub const VIP_PARSER_PORTA_EXTRA3: c_uint = 0x00bc;
pub const VIP_PARSER_PORTB_EXTRA3: c_uint = 0x00cc;
pub const VIP_ANC_SKIP_NUMLINES_MASK: c_uint = 0x0fff;
pub const VIP_ANC_SKIP_NUMLINES_SHFT: c_int = 0;
pub const VIP_ANC_USE_NUMLINES_MASK: c_uint = 0x0fff;
pub const VIP_ANC_USE_NUMLINES_SHFT: c_int = 16;
pub const VIP_PARSER_PORTA_EXTRA4: c_uint = 0x00c0;
pub const VIP_PARSER_PORTB_EXTRA4: c_uint = 0x00d0;
pub const VIP_ACT_SKIP_NUMPIX_MASK: c_uint = 0x0fff;
pub const VIP_ACT_SKIP_NUMPIX_SHFT: c_int = 0;

pub const VIP_ACT_USE_NUMPIX_MASK: c_uint = 0x0fff;
pub const VIP_ACT_USE_NUMPIX_SHFT: c_int = 16;
pub const VIP_ACT_TARGET_SRCNUM_MASK: c_uint = 0x0f;
pub const VIP_ACT_TARGET_SRCNUM_SHFT: c_int = 28;
pub const VIP_PARSER_PORTA_EXTRA5: c_uint = 0x00c4;
pub const VIP_PARSER_PORTB_EXTRA5: c_uint = 0x00d4;
pub const VIP_ACT_SKIP_NUMLINES_MASK: c_uint = 0x0fff;
pub const VIP_ACT_SKIP_NUMLINES_SHFT: c_int = 0;
pub const VIP_ACT_USE_NUMLINES_MASK: c_uint = 0x0fff;
pub const VIP_ACT_USE_NUMLINES_SHFT: c_int = 16;
pub const VIP_PARSER_PORTA_EXTRA6: c_uint = 0x00d8;
pub const VIP_PARSER_PORTB_EXTRA6: c_uint = 0x00dc;
pub const VIP_ANC_SRCNUM_STOP_IMM_SHFT: c_int = 0;
pub const VIP_YUV_SRCNUM_STOP_IMM_SHFT: c_int = 16;
pub const VIP_SLICE0_CSC: c_uint = 0x5700;
pub const VIP_SLICE1_CSC: c_uint = 0x5c00;
pub const VIP_CSC_CSC00: c_uint = 0x0200;
pub const VIP_CSC_A0_MASK: c_uint = 0x1fff;
pub const VIP_CSC_A0_SHFT: c_int = 0;
pub const VIP_CSC_B0_MASK: c_uint = 0x1fff;
pub const VIP_CSC_B0_SHFT: c_int = 16;
pub const VIP_CSC_CSC01: c_uint = 0x0204;
pub const VIP_CSC_C0_MASK: c_uint = 0x1fff;
pub const VIP_CSC_C0_SHFT: c_int = 0;
pub const VIP_CSC_A1_MASK: c_uint = 0x1fff;
pub const VIP_CSC_A1_SHFT: c_int = 16;
pub const VIP_CSC_CSC02: c_uint = 0x0208;
pub const VIP_CSC_B1_MASK: c_uint = 0x1fff;
pub const VIP_CSC_B1_SHFT: c_int = 0;
pub const VIP_CSC_C1_MASK: c_uint = 0x1fff;
pub const VIP_CSC_C1_SHFT: c_int = 16;
pub const VIP_CSC_CSC03: c_uint = 0x020c;
pub const VIP_CSC_A2_MASK: c_uint = 0x1fff;
pub const VIP_CSC_A2_SHFT: c_int = 0;
pub const VIP_CSC_B2_MASK: c_uint = 0x1fff;
pub const VIP_CSC_B2_SHFT: c_int = 16;
pub const VIP_CSC_CSC04: c_uint = 0x0210;
pub const VIP_CSC_C2_MASK: c_uint = 0x1fff;
pub const VIP_CSC_C2_SHFT: c_int = 0;
pub const VIP_CSC_D0_MASK: c_uint = 0x0fff;
pub const VIP_CSC_D0_SHFT: c_int = 16;
pub const VIP_CSC_CSC05: c_uint = 0x0214;
pub const VIP_CSC_D1_MASK: c_uint = 0x0fff;
pub const VIP_CSC_D1_SHFT: c_int = 0;
pub const VIP_CSC_D2_MASK: c_uint = 0x0fff;
pub const VIP_CSC_D2_SHFT: c_int = 16;

pub const VIP_SLICE0_SC: c_uint = 0x5800;
pub const VIP_SLICE1_SC: c_uint = 0x5d00;
pub const VIP_SC_MP_SC0: c_uint = 0x0300;

pub const VIP_SC_MP_SC1: c_uint = 0x0304;
pub const VIP_ROW_ACC_INC_MASK: c_uint = 0x07ffffff;
pub const VIP_ROW_ACC_INC_SHFT: c_int = 0;
pub const VIP_SC_MP_SC2: c_uint = 0x0308;
pub const VIP_ROW_ACC_OFFSET_MASK: c_uint = 0x0fffffff;
pub const VIP_ROW_ACC_OFFSET_SHFT: c_int = 0;
pub const VIP_SC_MP_SC3: c_uint = 0x030c;
pub const VIP_ROW_ACC_OFFSET_B_MASK: c_uint = 0x0fffffff;
pub const VIP_ROW_ACC_OFFSET_B_SHFT: c_int = 0;
pub const VIP_SC_MP_SC4: c_uint = 0x0310;
pub const VIP_TAR_H_MASK: c_uint = 0x07ff;
pub const VIP_TAR_H_SHFT: c_int = 0;
pub const VIP_TAR_W_MASK: c_uint = 0x07ff;
pub const VIP_TAR_W_SHFT: c_int = 12;
pub const VIP_LIN_ACC_INC_U_MASK: c_uint = 0x07;
pub const VIP_LIN_ACC_INC_U_SHFT: c_int = 24;
pub const VIP_NLIN_ACC_INIT_U_MASK: c_uint = 0x07;
pub const VIP_NLIN_ACC_INIT_U_SHFT: c_int = 28;
pub const VIP_SC_MP_SC5: c_uint = 0x0314;
pub const VIP_SRC_H_MASK: c_uint = 0x03ff;
pub const VIP_SRC_H_SHFT: c_int = 0;
pub const VIP_SRC_W_MASK: c_uint = 0x07ff;
pub const VIP_SRC_W_SHFT: c_int = 12;
pub const VIP_NLIN_ACC_INC_U_MASK: c_uint = 0x07;
pub const VIP_NLIN_ACC_INC_U_SHFT: c_int = 24;
pub const VIP_SC_MP_SC6: c_uint = 0x0318;
pub const VIP_ROW_ACC_INIT_RAV_MASK: c_uint = 0x03ff;
pub const VIP_ROW_ACC_INIT_RAV_SHFT: c_int = 0;
pub const VIP_ROW_ACC_INIT_RAV_B_MASK: c_uint = 0x03ff;
pub const VIP_ROW_ACC_INIT_RAV_B_SHFT: c_int = 10;
pub const VIP_SC_MP_SC8: c_uint = 0x0320;
pub const VIP_NLIN_LEFT_MASK: c_uint = 0x07ff;
pub const VIP_NLIN_LEFT_SHFT: c_int = 0;
pub const VIP_NLIN_RIGHT_MASK: c_uint = 0x07ff;
pub const VIP_NLIN_RIGHT_SHFT: c_int = 12;
pub const VIP_SC_MP_SC9: c_uint = 0x0324;

pub const VIP_SC_MP_SC10: c_uint = 0x0328;

pub const VIP_SC_MP_SC11: c_uint = 0x032c;

pub const VIP_SC_MP_SC12: c_uint = 0x0330;
pub const VIP_COL_ACC_OFFSET_MASK: c_uint = 0x01ffffff;
pub const VIP_COL_ACC_OFFSET_SHFT: c_int = 0;
pub const VIP_SC_MP_SC13: c_uint = 0x0334;
pub const VIP_SC_FACTOR_RAV_MASK: c_uint = 0x03ff;
pub const VIP_SC_FACTOR_RAV_SHFT: c_int = 0;
pub const VIP_CHROMA_INTP_THR_MASK: c_uint = 0x03ff;
pub const VIP_CHROMA_INTP_THR_SHFT: c_int = 12;
pub const VIP_DELTA_CHROMA_THR_MASK: c_uint = 0x0f;
pub const VIP_DELTA_CHROMA_THR_SHFT: c_int = 24;
pub const VIP_SC_MP_SC17: c_uint = 0x0344;
pub const VIP_EV_THR_MASK: c_uint = 0x03ff;
pub const VIP_EV_THR_SHFT: c_int = 12;
pub const VIP_DELTA_LUMA_THR_MASK: c_uint = 0x0f;
pub const VIP_DELTA_LUMA_THR_SHFT: c_int = 24;
pub const VIP_DELTA_EV_THR_MASK: c_uint = 0x0f;
pub const VIP_DELTA_EV_THR_SHFT: c_int = 28;
pub const VIP_SC_MP_SC18: c_uint = 0x0348;
pub const VIP_HS_FACTOR_MASK: c_uint = 0x03ff;
pub const VIP_HS_FACTOR_SHFT: c_int = 0;
pub const VIP_CONF_DEFAULT_MASK: c_uint = 0x01ff;
pub const VIP_CONF_DEFAULT_SHFT: c_int = 16;
pub const VIP_SC_MP_SC19: c_uint = 0x034c;
pub const VIP_HPF_COEFF0_MASK: c_uint = 0xff;
pub const VIP_HPF_COEFF0_SHFT: c_int = 0;
pub const VIP_HPF_COEFF1_MASK: c_uint = 0xff;
pub const VIP_HPF_COEFF1_SHFT: c_int = 8;
pub const VIP_HPF_COEFF2_MASK: c_uint = 0xff;
pub const VIP_HPF_COEFF2_SHFT: c_int = 16;
pub const VIP_HPF_COEFF3_MASK: c_uint = 0xff;
pub const VIP_HPF_COEFF3_SHFT: c_int = 23;
pub const VIP_SC_MP_SC20: c_uint = 0x0350;
pub const VIP_HPF_COEFF4_MASK: c_uint = 0xff;
pub const VIP_HPF_COEFF4_SHFT: c_int = 0;
pub const VIP_HPF_COEFF5_MASK: c_uint = 0xff;
pub const VIP_HPF_COEFF5_SHFT: c_int = 8;
pub const VIP_HPF_NORM_SHFT_MASK: c_uint = 0x07;
pub const VIP_HPF_NORM_SHFT_SHFT: c_int = 16;
pub const VIP_NL_LIMIT_MASK: c_uint = 0x1ff;
pub const VIP_NL_LIMIT_SHFT: c_int = 20;
pub const VIP_SC_MP_SC21: c_uint = 0x0354;
pub const VIP_NL_LO_THR_MASK: c_uint = 0x01ff;
pub const VIP_NL_LO_THR_SHFT: c_int = 0;
pub const VIP_NL_LO_SLOPE_MASK: c_uint = 0xff;
pub const VIP_NL_LO_SLOPE_SHFT: c_int = 16;
pub const VIP_SC_MP_SC22: c_uint = 0x0358;
pub const VIP_NL_HI_THR_MASK: c_uint = 0x01ff;
pub const VIP_NL_HI_THR_SHFT: c_int = 0;
pub const VIP_NL_HI_SLOPE_SH_MASK: c_uint = 0x07;
pub const VIP_NL_HI_SLOPE_SH_SHFT: c_int = 16;
pub const VIP_SC_MP_SC23: c_uint = 0x035c;
pub const VIP_GRADIENT_THR_MASK: c_uint = 0x07ff;
pub const VIP_GRADIENT_THR_SHFT: c_int = 0;
pub const VIP_GRADIENT_THR_RANGE_MASK: c_uint = 0x0f;
pub const VIP_GRADIENT_THR_RANGE_SHFT: c_int = 12;
pub const VIP_MIN_GY_THR_MASK: c_uint = 0xff;
pub const VIP_MIN_GY_THR_SHFT: c_int = 16;
pub const VIP_MIN_GY_THR_RANGE_MASK: c_uint = 0x0f;
pub const VIP_MIN_GY_THR_RANGE_SHFT: c_int = 28;
pub const VIP_SC_MP_SC24: c_uint = 0x0360;
pub const VIP_ORG_H_MASK: c_uint = 0x07ff;
pub const VIP_ORG_H_SHFT: c_int = 0;
pub const VIP_ORG_W_MASK: c_uint = 0x07ff;
pub const VIP_ORG_W_SHFT: c_int = 16;
pub const VIP_SC_MP_SC25: c_uint = 0x0364;
pub const VIP_OFF_H_MASK: c_uint = 0x07ff;
pub const VIP_OFF_H_SHFT: c_int = 0;
pub const VIP_OFF_W_MASK: c_uint = 0x07ff;
pub const VIP_OFF_W_SHFT: c_int = 16;
pub const VIP_VPDMA_BASE: c_uint = 0xd000;
