//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/omap3isp/ispstat.h
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
// ispstat.h
//
// TI OMAP3 ISP - Statistics core
//
// Copyright (C) 2010 Nokia Corporation
// Copyright (C) 2009 Texas Instruments, Inc
//
// Contacts: David Cohen <dacohen@gmail.com>
// Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//

pub const STAT_MAX_BUFS: c_int = 5;
pub const STAT_NEVENTS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ispstat_buffer {
    pub sgt: sg_table,
    pub virt_addr: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub ts: timespec64,
    pub buf_size: u32,
    pub frame_number: u32,
    pub config_counter: u16,
    pub empty: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ispstat_ops {
//
// Validate new params configuration.
// new_conf->buf_size value must be changed to the exact buffer size
// necessary for the new configuration if it's smaller.
//
    pub new_conf): *mut *mut *mut int (validate_params)(struct ispstat stat, void,
//
// Save new params configuration.
// stat->priv->buf_size value must be set to the exact buffer size for
// the new configuration.
// stat->update is set to 1 if new configuration is different than
// current one.
//
    pub new_conf): *mut *mut *mut void (set_params)(struct ispstat stat, void,
// Apply stored configuration.
    pub priv): *mut *mut *mut void (setup_regs)(struct ispstat stat, void,
// Enable/Disable module.
    pub enable): *mut *mut *mut void (enable)(struct ispstat stat, int,
// Verify is module is busy.
    pub stat): *mut *mut int (busy)(struct ispstat,
// Used for specific operations during generic buf process task.
    pub stat): *mut *mut int (buf_process)(struct ispstat,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ispstat_state_t {
    ISPSTAT_DISABLED = 0,
    ISPSTAT_DISABLING,
    ISPSTAT_ENABLED,
    ISPSTAT_ENABLING,
    ISPSTAT_SUSPENDED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ispstat {
    pub subdev: v4l2_subdev,
    pub /: *mut *mut media_pad pad; / sink pad,
// Control
    pub configured:1: unsigned,
    pub update:1: unsigned,
    pub buf_processing:1: unsigned,
    pub sbl_ovl_recover:1: unsigned,
    pub inc_config: u8,
    pub buf_err: core::sync::atomic::AtomicI32,
    pub /: *mut *mut ispstat_state_t state; / enabling/disabling state,
    pub isp: *mut isp_device,
    pub /: *mut *mut *mut void priv; / pointer to priv config struct,
    pub /: *mut *mut *mut void recover_priv; / pointer to recover priv configuration,
    pub /: *mut *mut mutex ioctl_lock; / serialize private ioctl,
    pub ops: *const ispstat_ops,
// Buffer
    pub wait_acc_frames: u8,
    pub config_counter: u16,
    pub frame_number: u32,
    pub buf_size: u32,
    pub buf_alloc_size: u32,
    pub dma_ch: *mut dma_chan,
    pub event_type: c_ulong,
    pub buf: *mut ispstat_buffer,
    pub active_buf: *mut ispstat_buffer,
    pub locked_buf: *mut ispstat_buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ispstat_generic_config {
//
// Fields must be in the same order as in:
// - omap3isp_h3a_aewb_config
// - omap3isp_h3a_af_config
// - omap3isp_hist_config
//
    pub buf_size: u32,
    pub config_counter: u16,
}

extern "C" {
    pub fn omap3isp_stat_config(stat: *mut ispstat, new_conf: *mut c_void) -> c_int;
}
extern "C" {
    pub fn omap3isp_stat_cleanup(stat: *mut ispstat);
}
extern "C" {
    pub fn omap3isp_stat_s_stream(subdev: *mut v4l2_subdev, enable: c_int) -> c_int;
}
extern "C" {
    pub fn omap3isp_stat_busy(stat: *mut ispstat) -> c_int;
}
extern "C" {
    pub fn omap3isp_stat_pcr_busy(stat: *mut ispstat) -> c_int;
}
extern "C" {
    pub fn omap3isp_stat_suspend(stat: *mut ispstat);
}
extern "C" {
    pub fn omap3isp_stat_resume(stat: *mut ispstat);
}
extern "C" {
    pub fn omap3isp_stat_enable(stat: *mut ispstat, enable: u8) -> c_int;
}
extern "C" {
    pub fn omap3isp_stat_sbl_overflow(stat: *mut ispstat);
}
extern "C" {
    pub fn omap3isp_stat_isr(stat: *mut ispstat);
}
extern "C" {
    pub fn omap3isp_stat_isr_frame_sync(stat: *mut ispstat);
}
extern "C" {
    pub fn omap3isp_stat_dma_isr(stat: *mut ispstat);
}
extern "C" {
    pub fn omap3isp_stat_unregister_entities(stat: *mut ispstat);
}
