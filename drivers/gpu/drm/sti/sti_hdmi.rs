//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sti/sti_hdmi.h
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
// Copyright (C) STMicroelectronics SA 2014
// Author: Vincent Abriou <vincent.abriou@st.com> for STMicroelectronics.
//

pub const HDMI_STA: c_uint = 0x0010;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_phy_ops {
    pub hdmi): *mut *mut bool (start)(struct sti_hdmi,
    pub hdmi): *mut *mut void (stop)(struct sti_hdmi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_audio_params {
    pub enabled: bool,
    pub sample_width: c_uint,
    pub sample_rate: c_uint,
    pub cea: hdmi_audio_infoframe,
}

//
// STI hdmi structure
//
// @dev: driver device
// @drm_dev: pointer to drm device
// @mode: current display mode selected
// @regs: hdmi register
// @syscfg: syscfg register for pll rejection configuration
// @clk_pix: hdmi pixel clock
// @clk_tmds: hdmi tmds clock
// @clk_phy: hdmi phy clock
// @clk_audio: hdmi audio clock
// @irq: hdmi interrupt number
// @irq_status: interrupt status register
// @phy_ops: phy start/stop operations
// @enabled: true if hdmi is enabled else false
// @hpd: hot plug detect status
// @wait_event: wait event
// @event_received: wait event status
// @reset: reset control of the hdmi phy
// @ddc_adapt: i2c ddc adapter
// @colorspace: current colorspace selected
// @audio_pdev: ASoC hdmi-codec platform device
// @audio: hdmi audio parameters.
// @drm_connector: hdmi connector
// @notifier: hotplug detect notifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_hdmi {
    pub dev: device,
    pub drm_dev: *mut drm_device,
    pub mode: drm_display_mode,
    pub regs: *mut void __iomem,
    pub syscfg: *mut void __iomem,
    pub clk_pix: *mut clk,
    pub clk_tmds: *mut clk,
    pub clk_phy: *mut clk,
    pub clk_audio: *mut clk,
    pub irq: c_int,
    pub irq_status: u32,
    pub phy_ops: *mut hdmi_phy_ops,
    pub enabled: bool,
    pub hpd: bool,
    pub wait_event: wait_queue_head_t,
    pub event_received: bool,
    pub reset: *mut reset_control,
    pub ddc_adapt: *mut i2c_adapter,
    pub colorspace: hdmi_colorspace,
    pub audio_pdev: *mut platform_device,
    pub audio: hdmi_audio_params,
    pub drm_connector: *mut drm_connector,
    pub notifier: *mut cec_notifier,
    pub bridge: drm_bridge,
}

extern "C" {
    pub fn hdmi_read(hdmi: *mut sti_hdmi, offset: c_int) -> u32;
}
extern "C" {
    pub fn hdmi_write(hdmi: *mut sti_hdmi, val: u32, offset: c_int);
}
//
// hdmi phy config structure
//
// A pointer to an array of these structures is passed to a TMDS (HDMI) output
// via the control interface to provide board and SoC specific
// configurations of the HDMI PHY. Each entry in the array specifies a hardware
// specific configuration for a given TMDS clock frequency range.
//
// @min_tmds_freq: Lower bound of TMDS clock frequency this entry applies to
// @max_tmds_freq: Upper bound of TMDS clock frequency this entry applies to
// @config: SoC specific register configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_phy_config {
    pub min_tmds_freq: u32,
    pub max_tmds_freq: u32,
    pub config: [u32; 4],
}
