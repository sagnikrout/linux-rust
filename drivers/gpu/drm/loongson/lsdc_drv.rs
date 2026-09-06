//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/loongson/lsdc_drv.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2023 Loongson Technology Corporation Limited
//

// Currently, all Loongson display controllers have two display pipes.
pub const LSDC_NUM_CRTC: c_int = 2;
//
// LS7A1000/LS7A2000 chipsets function as the south & north bridges of the
// Loongson 3 series processors, they are equipped with on-board video RAM
// typically. While Loongson LS2K series are low cost SoCs which share the
// system RAM as video RAM, they don't has a dedicated VRAM.
//
// There is only a 1:1 mapping of crtcs, encoders and connectors for the DC
//
// display pipe 0 = crtc0 + dvo0 + encoder0 + connector0 + cursor0 + primary0
// display pipe 1 = crtc1 + dvo1 + encoder1 + connectro1 + cursor1 + primary1
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum loongson_chip_id {
    CHIP_LS7A1000 = 0,
    CHIP_LS7A2000 = 1,
    CHIP_LS_LAST,
}

// DC specific
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_desc {
    pub num_of_crtc: u32,
    pub max_pixel_clk: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub num_of_hw_cursor: u32,
    pub hw_cursor_w: u32,
    pub hw_cursor_h: u32,
    pub /: *mut *mut u32 pitch_align; / CRTC DMA alignment constraint,
    pub /: *mut *mut bool has_vblank_counter; / 32 bit hw vsync counter,
// device dependent ops, dc side
    pub funcs: *const lsdc_kms_funcs,
}

// GFX related resources wrangler
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson_gfx_desc {
    pub dc: lsdc_desc,
    pub conf_reg_base: u32,
// GFXPLL shared by the DC, GMC and GPU
    pub reg_offset: u32,
    pub reg_size: u32,
    pub gfxpll: },
// Pixel PLL, per display pipe
    pub reg_offset: u32,
    pub reg_size: u32,
    pub pixpll: [}; LSDC_NUM_CRTC],
    pub chip_id: loongson_chip_id,
    pub model: [c_char; 64],
}

extern "C" {
    pub fn container_of_const(_arg: dcp, loongson_gfx_desc: struct, _arg: dc) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_reg32 {
    pub name: *mut c_char,
    pub offset: u32,
}

// crtc hardware related ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_crtc_hw_ops {
    pub lcrtc): *mut *mut void (enable)(struct lsdc_crtc,
    pub lcrtc): *mut *mut void (disable)(struct lsdc_crtc,
    pub lcrtc): *mut *mut void (enable_vblank)(struct lsdc_crtc,
    pub lcrtc): *mut *mut void (disable_vblank)(struct lsdc_crtc,
    pub lcrtc): *mut *mut void (flip)(struct lsdc_crtc,
    pub lcrtc): *mut *mut void (clone)(struct lsdc_crtc,
    pub vpos): *mut *mut *mut *mut void (get_scan_pos)(struct lsdc_crtc lcrtc, int hpos, int,
    pub mode): *const *const *const void (set_mode)(struct lsdc_crtc lcrtc, struct drm_display_mode,
    pub lcrtc): *mut *mut void (soft_reset)(struct lsdc_crtc,
    pub lcrtc): *mut *mut void (reset)(struct lsdc_crtc,
    pub lcrtc): *mut *mut u32 (get_vblank_counter)(struct lsdc_crtc,
    pub step): *mut *mut *mut void (set_dma_step)(struct lsdc_crtc lcrtc, enum lsdc_dma_steps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_crtc {
    pub base: drm_crtc,
    pub pixpll: lsdc_pixpll,
    pub ldev: *mut lsdc_device,
    pub hw_ops: *const lsdc_crtc_hw_ops,
    pub preg: *const lsdc_reg32,
    pub nreg: c_uint,
    pub p_info_list: *mut drm_info_list,
    pub n_info_list: c_uint,
    pub has_vblank: bool,
}

// primary plane hardware related ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_primary_plane_ops {
    pub addr): *mut *mut *mut void (update_fb_addr)(struct lsdc_primary plane, u64,
    pub stride): *mut *mut *mut void (update_fb_stride)(struct lsdc_primary plane, u32,
    pub format): *const drm_format_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_primary {
    pub base: drm_plane,
    pub ops: *const lsdc_primary_plane_ops,
    pub ldev: *mut lsdc_device,
}

// cursor plane hardware related ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_cursor_plane_ops {
    pub addr): *mut *mut *mut void (update_bo_addr)(struct lsdc_cursor plane, u64,
    pub lsdc_cursor_format): enum,
    pub y): *mut *mut *mut void (update_position)(struct lsdc_cursor plane, int x, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_cursor {
    pub base: drm_plane,
    pub ops: *const lsdc_cursor_plane_ops,
    pub ldev: *mut lsdc_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_output {
    pub encoder: drm_encoder,
    pub connector: drm_connector,
}

extern "C" {
    pub fn container_of(_arg: connector, lsdc_output: struct, _arg: connector) -> return;
}
extern "C" {
    pub fn container_of(_arg: encoder, lsdc_output: struct, _arg: encoder) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_display_pipe {
    pub crtc: lsdc_crtc,
    pub primary: lsdc_primary,
    pub cursor: lsdc_cursor,
    pub output: lsdc_output,
    pub li2c: *mut lsdc_i2c,
    pub index: c_uint,
}

extern "C" {
    pub fn container_of(_arg: output, lsdc_display_pipe: struct, _arg: output) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_kms_funcs {
    pub arg): *mut *mut irqreturn_t (irq_handler)(int irq, void,
    pub index): c_uint,
    pub index): c_uint,
    pub index): c_uint,
    pub index): c_uint,
    pub has_vblank): bool,
}

extern "C" {
    pub fn container_of(_arg: crtc, lsdc_crtc: struct, _arg: base) -> return;
}
extern "C" {
    pub fn container_of(_arg: crtc, lsdc_display_pipe: struct, _arg: crtc.base) -> return;
}
extern "C" {
    pub fn container_of(_arg: plane, lsdc_primary: struct, _arg: base) -> return;
}
extern "C" {
    pub fn container_of(_arg: plane, lsdc_cursor: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_crtc_state {
    pub base: drm_crtc_state,
    pub pparms: lsdc_pixpll_parms,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_gem {
// @mutex: protect objects list
    pub mutex: mutex,
    pub objects: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_device {
    pub base: drm_device,
    pub bdev: ttm_device,
// @descp: features description of the DC variant
    pub descp: *const lsdc_desc,
    pub dc: *mut pci_dev,
    pub gpu: *mut pci_dev,
    pub gfxpll: *mut loongson_gfxpll,
// @reglock: protects concurrent access
    pub reglock: spinlock_t,
    pub reg_base: *mut void __iomem,
    pub vram_base: resource_size_t,
    pub vram_size: resource_size_t,
    pub gtt_base: resource_size_t,
    pub gtt_size: resource_size_t,
    pub dispipe: [lsdc_display_pipe; LSDC_NUM_CRTC],
    pub gem: lsdc_gem,
    pub irq_status: u32,
// tracking pinned memory
    pub vram_pinned_size: usize,
    pub gtt_pinned_size: usize,
// @num_output: count the number of active display pipe
    pub num_output: c_uint,
}

extern "C" {
    pub fn container_of(_arg: bdev, lsdc_device: struct, _arg: bdev) -> return;
}
extern "C" {
    pub fn container_of(_arg: ddev, lsdc_device: struct, _arg: base) -> return;
}
extern "C" {
    pub fn container_of(_arg: base, lsdc_crtc_state: struct, _arg: base) -> return;
}
extern "C" {
    pub fn lsdc_debugfs_init(minor: *mut drm_minor);
}
// Registers access helpers
extern "C" {
    pub fn readl(offset: ldev->reg_base +) -> return;
}
extern "C" {
    pub fn readl(CRTC_PIPE_OFFSET: *mut *mut ldev->reg_base + offset + pipe) -> return;
}
