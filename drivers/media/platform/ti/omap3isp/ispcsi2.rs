//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/omap3isp/ispcsi2.h
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
// ispcsi2.h
//
// TI OMAP3 ISP - CSI2 module
//
// Copyright (C) 2010 Nokia Corporation
// Copyright (C) 2009 Texas Instruments, Inc.
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//

// This is not an exhaustive list
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_csi2_pix_formats {
    CSI2_PIX_FMT_OTHERS = 0,
    CSI2_PIX_FMT_YUV422_8BIT = 0x1e,
    CSI2_PIX_FMT_YUV422_8BIT_VP = 0x9e,
    CSI2_PIX_FMT_RAW10_EXP16 = 0xab,
    CSI2_PIX_FMT_RAW10_EXP16_VP = 0x12f,
    CSI2_PIX_FMT_RAW8 = 0x2a,
    CSI2_PIX_FMT_RAW8_DPCM10_EXP16 = 0x2aa,
    CSI2_PIX_FMT_RAW8_DPCM10_VP = 0x32a,
    CSI2_PIX_FMT_RAW8_VP = 0x12a,
    CSI2_USERDEF_8BIT_DATA1_DPCM10_VP = 0x340,
    CSI2_USERDEF_8BIT_DATA1_DPCM10 = 0x2c0,
    CSI2_USERDEF_8BIT_DATA1 = 0x40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_csi2_irqevents {
    OCP_ERR_IRQ = 0x4000,
    SHORT_PACKET_IRQ = 0x2000,
    ECC_CORRECTION_IRQ = 0x1000,
    ECC_NO_CORRECTION_IRQ = 0x800,
    COMPLEXIO2_ERR_IRQ = 0x400,
    COMPLEXIO1_ERR_IRQ = 0x200,
    FIFO_OVF_IRQ = 0x100,
    CONTEXT7 = 0x80,
    CONTEXT6 = 0x40,
    CONTEXT5 = 0x20,
    CONTEXT4 = 0x10,
    CONTEXT3 = 0x8,
    CONTEXT2 = 0x4,
    CONTEXT1 = 0x2,
    CONTEXT0 = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_csi2_ctx_irqevents {
    CTX_ECC_CORRECTION = 0x100,
    CTX_LINE_NUMBER = 0x80,
    CTX_FRAME_NUMBER = 0x40,
    CTX_CS = 0x20,
    CTX_LE = 0x8,
    CTX_LS = 0x4,
    CTX_FE = 0x2,
    CTX_FS = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_csi2_frame_mode {
    ISP_CSI2_FRAME_IMMEDIATE,
    ISP_CSI2_FRAME_AFTERFEC,
}

pub const ISP_CSI2_MAX_CTX_NUM: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_csi2_ctx_cfg {
    pub /: *mut *mut u8 ctxnum; / context number 0 - 7,
    pub dpcm_decompress: u8,
// Fields in CSI2_CTx_CTRL2 - locked by CSI2_CTx_CTRL1.CTX_EN
    pub virtual_id: u8,
    pub /: *mut *mut u16 format_id; / as in CSI2_CTx_CTRL2[9:0],
    pub /: *mut *mut u8 dpcm_predictor; / 1: simple, 0: advanced,
// Fields in CSI2_CTx_CTRL1/3 - Shadowed
    pub alpha: u16,
    pub data_offset: u16,
    pub ping_addr: u32,
    pub pong_addr: u32,
    pub eof_enabled: u8,
    pub eol_enabled: u8,
    pub checksum_enabled: u8,
    pub enabled: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_csi2_timing_cfg {
    pub /: *mut *mut u8 ionum; / IO1 or IO2 as in CSI2_TIMING,
    pub force_rx_mode:1: unsigned,
    pub stop_state_16x:1: unsigned,
    pub stop_state_4x:1: unsigned,
    pub stop_state_counter: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_csi2_ctrl_cfg {
    pub vp_clk_enable: bool,
    pub vp_only_enable: bool,
    pub vp_out_ctrl: u8,
    pub frame_mode: isp_csi2_frame_mode,
    pub ecc_enable: bool,
    pub if_enable: bool,
}

pub const CSI2_PAD_SINK: c_int = 0;
pub const CSI2_PAD_SOURCE: c_int = 1;
pub const CSI2_PADS_NUM: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_csi2_device {
    pub subdev: v4l2_subdev,
    pub pads: [media_pad; CSI2_PADS_NUM],
    pub formats: [v4l2_mbus_framefmt; CSI2_PADS_NUM],
    pub video_out: isp_video,
    pub isp: *mut isp_device,
    pub /: *mut *mut u8 available; / Is the IP present on the silicon?,
// mem resources - enums as defined in enum isp_mem_resources
    pub regs1: u8,
    pub regs2: u8,
    pub /: *mut *mut u32 output; / output to CCDC, memory or both?,
    pub dpcm_decompress: bool,
    pub frame_skip: c_uint,
    pub phy: *mut isp_csiphy,
    pub 1]: isp_csi2_ctx_cfg contexts[ISP_CSI2_MAX_CTX_NUM +,
    pub timing: [isp_csi2_timing_cfg; 2],
    pub ctrl: isp_csi2_ctrl_cfg,
    pub state: isp_pipeline_stream_state,
    pub wait: wait_queue_head_t,
    pub stopping: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn omap3isp_csi2_isr(csi2: *mut isp_csi2_device);
}
extern "C" {
    pub fn omap3isp_csi2_reset(csi2: *mut isp_csi2_device) -> c_int;
}
extern "C" {
    pub fn omap3isp_csi2_init(isp: *mut isp_device) -> c_int;
}
extern "C" {
    pub fn omap3isp_csi2_cleanup(isp: *mut isp_device);
}
extern "C" {
    pub fn omap3isp_csi2_unregister_entities(csi2: *mut isp_csi2_device);
}
