//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/omap3isp/ispccdc.h
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
// ispccdc.h
//
// TI OMAP3 ISP - CCDC module
//
// Copyright (C) 2009-2010 Nokia Corporation
// Copyright (C) 2009 Texas Instruments, Inc.
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ccdc_input_entity {
    CCDC_INPUT_NONE,
    CCDC_INPUT_PARALLEL,
    CCDC_INPUT_CSI2A,
    CCDC_INPUT_CCP2B,
    CCDC_INPUT_CSI2C
}

pub const OMAP3ISP_CCDC_NEVENTS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ispccdc_fpc {
    pub addr: *mut c_void,
    pub dma: dma_addr_t,
    pub fpnum: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ispccdc_lsc_state {
    LSC_STATE_STOPPED = 0,
    LSC_STATE_STOPPING = 1,
    LSC_STATE_RUNNING = 2,
    LSC_STATE_RECONFIG = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ispccdc_lsc_config_req {
    pub list: list_head,
    pub config: omap3isp_ccdc_lsc_config,
    pub enable: c_uchar,
    pub addr: *mut c_void,
    pub dma: dma_addr_t,
    pub sgt: sg_table,
    pub table: },
}

//
// ispccdc_lsc - CCDC LSC parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ispccdc_lsc {
    pub state: ispccdc_lsc_state,
    pub table_work: work_struct,
// LSC queue of configurations
    pub req_lock: spinlock_t,
    pub /: *mut *mut *mut ispccdc_lsc_config_req request; / requested configuration,
    pub /: *mut *mut *mut ispccdc_lsc_config_req active; / active configuration,
    pub /: *mut *mut list_head free_queue; / configurations for freeing,
}

pub const CCDC_STOP_NOT_REQUESTED: c_uint = 0x00;
pub const CCDC_STOP_REQUEST: c_uint = 0x01;

pub const CCDC_STOP_CCDC_FINISHED: c_uint = 0x04;
pub const CCDC_STOP_LSC_FINISHED: c_uint = 0x08;

pub const CCDC_EVENT_VD1: c_uint = 0x10;
pub const CCDC_EVENT_VD0: c_uint = 0x20;
pub const CCDC_EVENT_LSC_DONE: c_uint = 0x40;
// Sink and source CCDC pads
pub const CCDC_PAD_SINK: c_int = 0;
pub const CCDC_PAD_SOURCE_OF: c_int = 1;
pub const CCDC_PAD_SOURCE_VP: c_int = 2;
pub const CCDC_PADS_NUM: c_int = 3;
pub const CCDC_FIELD_TOP: c_int = 1;
pub const CCDC_FIELD_BOTTOM: c_int = 2;
pub const CCDC_FIELD_BOTH: c_int = 3;
//
// struct isp_ccdc_device - Structure for the CCDC module to store its own
// information
// @subdev: V4L2 subdevice
// @pads: Sink and source media entity pads
// @formats: Active video formats
// @crop: Active crop rectangle on the OF source pad
// @input: Active input
// @output: Active outputs
// @video_out: Output video node
// @alaw: A-law compression enabled (1) or disabled (0)
// @lpf: Low pass filter enabled (1) or disabled (0)
// @obclamp: Optical-black clamp enabled (1) or disabled (0)
// @fpc_en: Faulty pixels correction enabled (1) or disabled (0)
// @blcomp: Black level compensation configuration
// @clamp: Optical-black or digital clamp configuration
// @fpc: Faulty pixels correction configuration
// @lsc: Lens shading compensation configuration
// @update: Bitmask of controls to update during the next interrupt
// @shadow_update: Controls update in progress by userspace
// @bt656: Whether the input interface uses BT.656 synchronization
// @fields: The fields (CCDC_FIELD_*) stored in the current buffer
// @underrun: A buffer underrun occurred and a new buffer has been queued
// @state: Streaming state
// @lock: Serializes shadow_update with interrupt handler
// @wait: Wait queue used to stop the module
// @stopping: Stopping state
// @running: Is the CCDC hardware running
// @ioctl_lock: Serializes ioctl calls and LSC requests freeing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_ccdc_device {
    pub subdev: v4l2_subdev,
    pub pads: [media_pad; CCDC_PADS_NUM],
    pub formats: [v4l2_mbus_framefmt; CCDC_PADS_NUM],
    pub crop: v4l2_rect,
    pub input: ccdc_input_entity,
    pub output: c_uint,
    pub video_out: isp_video,
    pub blcomp: omap3isp_ccdc_blcomp,
    pub clamp: omap3isp_ccdc_bclamp,
    pub fpc: ispccdc_fpc,
    pub lsc: ispccdc_lsc,
    pub update: c_uint,
    pub shadow_update: c_uint,
    pub bt656: bool,
    pub fields: c_uint,
    pub underrun:1: c_uint,
    pub state: isp_pipeline_stream_state,
    pub lock: spinlock_t,
    pub wait: wait_queue_head_t,
    pub stopping: c_uint,
    pub running: bool,
    pub ioctl_lock: mutex,
}

extern "C" {
    pub fn omap3isp_ccdc_init(isp: *mut isp_device) -> c_int;
}
extern "C" {
    pub fn omap3isp_ccdc_cleanup(isp: *mut isp_device);
}
extern "C" {
    pub fn omap3isp_ccdc_unregister_entities(ccdc: *mut isp_ccdc_device);
}
extern "C" {
    pub fn omap3isp_ccdc_busy(isp_ccdc: *mut isp_ccdc_device) -> c_int;
}
extern "C" {
    pub fn omap3isp_ccdc_isr(isp_ccdc: *mut isp_ccdc_device, events: u32) -> c_int;
}
extern "C" {
    pub fn omap3isp_ccdc_restore_context(isp: *mut isp_device);
}
