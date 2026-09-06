//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/gma500/psb_intel_drv.h
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
// Copyright (c) 2009-2011, Intel Corporation.
//

//
// Display related stuff
//
// maximum connectors per crtcs in the mode set
pub const INTELFB_CONN_LIMIT: c_int = 4;
// Intel Pipe Clone Bit
pub const INTEL_HDMIB_CLONE_BIT: c_int = 1;
pub const INTEL_HDMIC_CLONE_BIT: c_int = 2;
pub const INTEL_HDMID_CLONE_BIT: c_int = 3;
pub const INTEL_HDMIE_CLONE_BIT: c_int = 4;
pub const INTEL_HDMIF_CLONE_BIT: c_int = 5;
pub const INTEL_SDVO_NON_TV_CLONE_BIT: c_int = 6;
pub const INTEL_SDVO_TV_CLONE_BIT: c_int = 7;
pub const INTEL_SDVO_LVDS_CLONE_BIT: c_int = 8;
pub const INTEL_ANALOG_CLONE_BIT: c_int = 9;
pub const INTEL_TV_CLONE_BIT: c_int = 10;
pub const INTEL_DP_B_CLONE_BIT: c_int = 11;
pub const INTEL_DP_C_CLONE_BIT: c_int = 12;
pub const INTEL_DP_D_CLONE_BIT: c_int = 13;
pub const INTEL_LVDS_CLONE_BIT: c_int = 14;
pub const INTEL_DVO_TMDS_CLONE_BIT: c_int = 15;
pub const INTEL_DVO_LVDS_CLONE_BIT: c_int = 16;
pub const INTEL_EDP_CLONE_BIT: c_int = 17;
// these are outputs from the chip - integrated only
// external chips are via DVO or SDVO output
pub const INTEL_OUTPUT_UNUSED: c_int = 0;
pub const INTEL_OUTPUT_ANALOG: c_int = 1;
pub const INTEL_OUTPUT_DVO: c_int = 2;
pub const INTEL_OUTPUT_SDVO: c_int = 3;
pub const INTEL_OUTPUT_LVDS: c_int = 4;
pub const INTEL_OUTPUT_TVOUT: c_int = 5;
pub const INTEL_OUTPUT_HDMI: c_int = 6;
pub const INTEL_OUTPUT_MIPI: c_int = 7;
pub const INTEL_OUTPUT_MIPI2: c_int = 8;
pub const INTEL_OUTPUT_DISPLAYPORT: c_int = 9;
pub const INTEL_OUTPUT_EDP: c_int = 10;
//
// Hold information useally put on the device driver privates here,
// since it needs to be shared across multiple of devices drivers privates.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psb_intel_mode_device {
//
// Abstracted memory manager operations
//
    pub bo): *mut *mut *mut size_t(bo_offset) (struct drm_device dev, void,
//
// LVDS info
//
    pub /: *mut *mut int backlight_duty_cycle; / restore backlight to this value,
    pub panel_wants_dither: bool,
    pub panel_fixed_mode: *mut drm_display_mode,
    pub panel_fixed_mode2: *mut drm_display_mode,
    pub /: *mut *mut *mut drm_display_mode vbt_mode; / if any,
    pub saveBLC_PWM_CTL: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gma_i2c_chan {
    pub base: i2c_adapter,
    pub algo: i2c_algo_bit_data,
    pub target_addr: u8,
// for getting at dev. private (mmio etc.)
    pub drm_dev: *mut drm_device,
    pub /: *mut *mut u32 reg; / GPIO reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gma_encoder {
    pub base: drm_encoder,
    pub type: c_int,
    pub needs_tv_clock: bool,
    pub ): *mut *mut void (hot_plug)(struct gma_encoder,
    pub crtc_mask: c_int,
    pub clone_mask: c_int,
    pub /: *mut *mut u32 ddi_select; / Channel info,
pub const DDI0_SELECT: c_uint = 0x01;
pub const DDI1_SELECT: c_uint = 0x02;
pub const DP_MASK: c_uint = 0x8000;
pub const DDI_MASK: c_uint = 0x03;
    pub /: *mut *mut *mut void dev_priv; / For sdvo_priv, lvds_priv, etc...,
// FIXME: Either make SDVO and LVDS store it's i2c here or give CDV it's
    pub i2c_bus: *mut gma_i2c_chan,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gma_connector {
    pub base: drm_connector,
    pub encoder: *mut gma_encoder,
    pub connector): *mut *mut void (save)(struct drm_connector,
    pub connector): *mut *mut void (restore)(struct drm_connector,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psb_intel_crtc_state {
    pub saveDSPCNTR: u32,
    pub savePIPECONF: u32,
    pub savePIPESRC: u32,
    pub saveDPLL: u32,
    pub saveFP0: u32,
    pub saveFP1: u32,
    pub saveHTOTAL: u32,
    pub saveHBLANK: u32,
    pub saveHSYNC: u32,
    pub saveVTOTAL: u32,
    pub saveVBLANK: u32,
    pub saveVSYNC: u32,
    pub saveDSPSTRIDE: u32,
    pub saveDSPSIZE: u32,
    pub saveDSPPOS: u32,
    pub saveDSPBASE: u32,
    pub savePalette: [u32; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gma_crtc {
    pub base: drm_crtc,
    pub pipe: c_int,
    pub plane: c_int,
    pub cursor_addr: u32,
    pub cursor_pobj: *mut psb_gem_object,
    pub lut_adj: [u8; 256],
    pub fbdev_fb: *mut psb_intel_framebuffer,
// a mode_set for fbdev users on this crtc
    pub mode_set: drm_mode_set,
// GEM object that holds our cursor
    pub cursor_obj: *mut drm_gem_object,
    pub saved_mode: drm_display_mode,
    pub saved_adjusted_mode: drm_display_mode,
    pub mode_dev: *mut psb_intel_mode_device,
// crtc mode setting flags
    pub mode_flags: u32,
    pub active: bool,
// Saved Crtc HW states
    pub crtc_state: *mut psb_intel_crtc_state,
    pub clock_funcs: *const gma_clock_funcs,
    pub page_flip_event: *mut drm_pending_vblank_event,
}

extern "C" {
    pub fn gma_i2c_destroy(chan: *mut gma_i2c_chan);
}
extern "C" {
    pub fn psb_intel_sdvo_init(dev: *mut drm_device, output_device: c_int) -> bool;
}
extern "C" {
    pub fn psb_intel_lvds_set_brightness(dev: *mut drm_device, level: c_int);
}
extern "C" {
    pub fn psb_intel_lvds_destroy(connector: *mut drm_connector);
}
// intel_gmbus.c
extern "C" {
    pub fn gma_intel_i2c_reset(dev: *mut drm_device);
}
extern "C" {
    pub fn gma_intel_setup_gmbus(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn gma_intel_gmbus_set_speed(adapter: *mut i2c_adapter, speed: c_int);
}
extern "C" {
    pub fn gma_intel_gmbus_force_bit(adapter: *mut i2c_adapter, force_bit: bool);
}
extern "C" {
    pub fn gma_intel_teardown_gmbus(dev: *mut drm_device);
}
// DP support
extern "C" {
    pub fn cdv_intel_dp_init(dev: *mut drm_device, mode_dev: *mut psb_intel_mode_device, output_reg: c_int);
}
extern "C" {
    pub fn cdv_sb_read(dev: *mut drm_device, reg: u32, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn cdv_sb_write(dev: *mut drm_device, reg: u32, val: u32) -> c_int;
}
extern "C" {
    pub fn cdv_sb_reset(dev: *mut drm_device);
}
extern "C" {
    pub fn cdv_intel_attach_force_audio_property(connector: *mut drm_connector);
}
extern "C" {
    pub fn cdv_intel_attach_broadcast_rgb_property(connector: *mut drm_connector);
}
