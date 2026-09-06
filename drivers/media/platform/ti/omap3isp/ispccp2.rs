//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/omap3isp/ispccp2.h
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
// ispccp2.h
//
// TI OMAP3 ISP - CCP2 module
//
// Copyright (C) 2010 Nokia Corporation
// Copyright (C) 2010 Texas Instruments, Inc.
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//

// Sink and source ccp2 pads
pub const CCP2_PAD_SINK: c_int = 0;
pub const CCP2_PAD_SOURCE: c_int = 1;
pub const CCP2_PADS_NUM: c_int = 2;
// CCP2 input media entity
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ccp2_input_entity {
    CCP2_INPUT_NONE,
    CCP2_INPUT_SENSOR,
    CCP2_INPUT_MEMORY,
}

// CCP2 output media entity
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ccp2_output_entity {
    CCP2_OUTPUT_NONE,
    CCP2_OUTPUT_CCDC,
    CCP2_OUTPUT_MEMORY,
}

// Logical channel configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_interface_lcx_config {
    pub crc: c_int,
    pub data_start: u32,
    pub data_size: u32,
    pub format: u32,
}

// Memory channel configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_interface_mem_config {
    pub dst_port: u32,
    pub vsize_count: u32,
    pub hsize_count: u32,
    pub src_ofst: u32,
    pub dst_ofst: u32,
}

// CCP2 device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_ccp2_device {
    pub subdev: v4l2_subdev,
    pub formats: [v4l2_mbus_framefmt; CCP2_PADS_NUM],
    pub pads: [media_pad; CCP2_PADS_NUM],
    pub input: ccp2_input_entity,
    pub output: ccp2_output_entity,
    pub if_cfg: isp_interface_lcx_config,
    pub mem_cfg: isp_interface_mem_config,
    pub video_in: isp_video,
    pub phy: *mut isp_csiphy,
    pub vdds_csib: *mut regulator,
    pub state: isp_pipeline_stream_state,
    pub wait: wait_queue_head_t,
    pub stopping: core::sync::atomic::AtomicI32,
}

// Function declarations
extern "C" {
    pub fn omap3isp_ccp2_init(isp: *mut isp_device) -> c_int;
}
extern "C" {
    pub fn omap3isp_ccp2_cleanup(isp: *mut isp_device);
}
extern "C" {
    pub fn omap3isp_ccp2_unregister_entities(ccp2: *mut isp_ccp2_device);
}
extern "C" {
    pub fn omap3isp_ccp2_isr(ccp2: *mut isp_ccp2_device);
}
