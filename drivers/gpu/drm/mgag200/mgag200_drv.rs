//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mgag200/mgag200_drv.h
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
// Copyright 2010 Matt Turner.
// Copyright 2012 Red Hat
//
// Authors: Matthew Garrett
// Matt Turner
// Dave Airlie
//

pub const DRIVER_MAJOR: c_int = 1;
pub const DRIVER_MINOR: c_int = 0;
pub const DRIVER_PATCHLEVEL: c_int = 0;

pub const MGA_BIOS_OFFSET: c_uint = 0x7ffc;
pub const ATTR_INDEX: c_uint = 0x1fc0;
pub const ATTR_DATA: c_uint = 0x1fc1;

pub const GFX_INDEX: c_uint = 0x1fce;
pub const GFX_DATA: c_uint = 0x1fcf;

pub const DAC_INDEX: c_uint = 0x3c00;
pub const DAC_DATA: c_uint = 0x3c0a;

pub const MGA_MISC_OUT: c_uint = 0x1fc2;
pub const MGA_MISC_IN: c_uint = 0x1fcc;
//
// TODO: This is a pretty large set of default values for all kinds of
// settings. It should be split and set in the various DRM helpers,
// such as the CRTC reset or atomic_enable helpers. The PLL values
// probably belong to each model's PLL code.
//

// 0x00: */        0,    0,    0,    0,    0,    0, 0x00,    0,				\
// 0x08: */        0,    0,    0,    0,    0,    0,    0,    0,				\
// 0x10: */        0,    0,    0,    0,    0,    0,    0,    0,				\
// 0x18: */     (xvrefctrl),								\
// 0x19: */        0,									\
// 0x1a: */     (xpixclkctrl),								\
// 0x1b: */     0xff, 0xbf, 0x20,							\
// 0x1e: */	(xmiscctrl),								\
// 0x1f: */	0x20,									\
// 0x20: */     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,				\
// 0x28: */     0x00, 0x00, 0x00, 0x00,							\
// 0x2c: */     (xsyspllm),								\
// 0x2d: */     (xsysplln),								\
// 0x2e: */     (xsyspllp),								\
// 0x2f: */     0x40,									\
// 0x30: */     0x00, 0xb0, 0x00, 0xc2, 0x34, 0x14, 0x02, 0x83,				\
// 0x38: */     0x00, 0x93, 0x00, 0x77, 0x00, 0x00, 0x00, 0x3a,				\
// 0x40: */        0,    0,    0,    0,    0,    0,    0,    0,				\
// 0x48: */        0,    0,    0,    0,    0,    0,    0,    0				\
pub const MGAG200_LUT_SIZE: c_int = 256;
pub const MGAG200_MAX_FB_HEIGHT: c_int = 4096;
pub const MGAG200_MAX_FB_WIDTH: c_int = 4096;
//
// Stores parameters for programming the PLLs
//
// Fref: reference frequency (A: 25.175 Mhz, B: 28.361, C: XX Mhz)
// Fo: output frequency
// Fvco = Fref * (N / M)
// Fo = Fvco / P
//
// S = [0..3]
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgag200_pll_values {
    pub m: c_uint,
    pub n: c_uint,
    pub p: c_uint,
    pub s: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgag200_crtc_state {
    pub base: drm_crtc_state,
// Primary-plane format; required for modesetting and color mgmt.
    pub format: *const drm_format_info,
    pub pixpllc: mgag200_pll_values,
    pub set_vidrst: bool,
}

extern "C" {
    pub fn container_of(_arg: base, mgag200_crtc_state: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mga_type {
    G200_PCI,
    G200_AGP,
    G200_SE_A,
    G200_SE_B,
    G200_WB,
    G200_EV,
    G200_EH,
    G200_EH3,
    G200_EH5,
    G200_ER,
    G200_EW3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgag200_device_info {
    pub max_hdisplay: u16,
    pub max_vdisplay: u16,
//
// Maximum memory bandwidth (MiB/sec). Setting this to zero disables
// the rsp test during mode validation.
//
    pub max_mem_bandwidth: c_ulong,
// Synchronize scanout with BMC
    pub sync_bmc:1: bool,
    pub data_bit:3: unsigned,
    pub clock_bit:3: unsigned,
    pub i2c: },
//
// HW does not handle 'startadd' register correctly. Always set
// it's value to 0.
//
    pub bug_no_startadd:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgag200_device_funcs {
//
// Validate that the given state can be programmed into PIXPLLC. On
// success, the calculated parameters should be stored in the CRTC's
// state in struct @mgag200_crtc_state.pixpllc.
//
    pub new_state): *mut *mut *mut int (pixpllc_atomic_check)(struct drm_crtc crtc, struct drm_atomic_commit,
//
// Program PIXPLLC from the CRTC state. The parameters should have been
// stored in struct @mgag200_crtc_state.pixpllc by the corresponding
// implementation of @pixpllc_atomic_check.
//
    pub old_state): *mut *mut *mut void (pixpllc_atomic_update)(struct drm_crtc crtc, struct drm_atomic_commit,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mga_device {
    pub base: drm_device,
    pub info: *const mgag200_device_info,
    pub funcs: *const mgag200_device_funcs,
    pub rmmio_res: *mut resource,
    pub rmmio: *mut void __iomem,
    pub /: *mut *mut mutex rmmio_lock; / Protects access to rmmio,
    pub vram_res: *mut resource,
    pub vram: *mut void __iomem,
    pub vram_available: resource_size_t,
    pub primary_plane: drm_plane,
    pub crtc: drm_crtc,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
    pub vga: },
    pub output: },
}

extern "C" {
    pub fn container_of(_arg: dev, mga_device: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgag200_g200_device {
    pub base: mga_device,
// PLL constants
    pub ref_clk: c_long,
    pub pclk_min: c_long,
    pub pclk_max: c_long,
}

extern "C" {
    pub fn container_of(_arg: to_mga_device(dev), mgag200_g200_device: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgag200_g200se_device {
    pub base: mga_device,
// SE model number stored in reg 0x1e24
    pub unique_rev_id: u32,
}

extern "C" {
    pub fn container_of(_arg: to_mga_device(dev), mgag200_g200se_device: struct, _arg: base) -> return;
}
// mgag200_drv.c
extern "C" {
    pub fn mgag200_init_pci_options(pdev: *mut pci_dev, option: u32, option2: u32) -> c_int;
}
extern "C" {
    pub fn mgag200_probe_vram(mem: *mut void __iomem, size: resource_size_t) -> resource_size_t;
}
extern "C" {
    pub fn mgag200_device_probe_vram(mdev: *mut mga_device) -> resource_size_t;
}
extern "C" {
    pub fn mgag200_device_preinit(mdev: *mut mga_device) -> c_int;
}
// mgag200_<device type>.c
extern "C" {
    pub fn mgag200_g200wb_init_registers(mdev: *mut mga_device);
}
extern "C" {
    pub fn mgag200_g200wb_pixpllc_atomic_update(crtc: *mut drm_crtc, old_state: *mut drm_atomic_commit);
}
extern "C" {
    pub fn mgag200_g200eh_init_registers(mdev: *mut mga_device);
}
extern "C" {
    pub fn mgag200_g200eh_pixpllc_atomic_update(crtc: *mut drm_crtc, old_state: *mut drm_atomic_commit);
}
//
// mgag200_mode.c
//

extern "C" {
    pub fn mgag200_crtc_fill_gamma(mdev: *mut mga_device, format: *const drm_format_info);
}
extern "C" {
    pub fn mgag200_crtc_helper_atomic_check(crtc: *mut drm_crtc, new_state: *mut drm_atomic_commit) -> c_int;
}
extern "C" {
    pub fn mgag200_crtc_helper_atomic_flush(crtc: *mut drm_crtc, old_state: *mut drm_atomic_commit);
}
extern "C" {
    pub fn mgag200_crtc_helper_atomic_enable(crtc: *mut drm_crtc, old_state: *mut drm_atomic_commit);
}
extern "C" {
    pub fn mgag200_crtc_helper_atomic_disable(crtc: *mut drm_crtc, old_state: *mut drm_atomic_commit);
}

extern "C" {
    pub fn mgag200_crtc_reset(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn mgag200_crtc_atomic_destroy_state(crtc: *mut drm_crtc, crtc_state: *mut drm_crtc_state);
}

extern "C" {
    pub fn mgag200_set_format_regs(mdev: *mut mga_device, format: *const drm_format_info);
}
extern "C" {
    pub fn mgag200_enable_display(mdev: *mut mga_device);
}
extern "C" {
    pub fn mgag200_init_registers(mdev: *mut mga_device);
}
extern "C" {
    pub fn mgag200_mode_config_init(mdev: *mut mga_device, vram_available: resource_size_t) -> c_int;
}
// mgag200_vga_bmc.c
extern "C" {
    pub fn mgag200_vga_bmc_output_init(mdev: *mut mga_device) -> c_int;
}
// mgag200_vga.c
extern "C" {
    pub fn mgag200_vga_output_init(mdev: *mut mga_device) -> c_int;
}
// mgag200_bmc.c
extern "C" {
    pub fn mgag200_bmc_stop_scanout(mdev: *mut mga_device);
}
extern "C" {
    pub fn mgag200_bmc_start_scanout(mdev: *mut mga_device);
}
