//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mcde/mcde_drm.h
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
// Copyright (C) 2018 Linus Walleij <linus.walleij@linaro.org>
// Parts of this file were based on the MCDE driver by Marcus Lorentzon
// (C) ST-Ericsson SA 2013
//

// Shared basic registers
pub const MCDE_CR: c_uint = 0x00000000;
pub const MCDE_CR_IFIFOEMPTYLINECOUNT_V422_SHIFT: c_int = 0;
pub const MCDE_CR_IFIFOEMPTYLINECOUNT_V422_MASK: c_uint = 0x0000003F;

pub const MCDE_CONF0: c_uint = 0x00000004;

pub const MCDE_CONF0_IFIFOCTRLWTRMRKLVL_SHIFT: c_int = 12;
pub const MCDE_CONF0_IFIFOCTRLWTRMRKLVL_MASK: c_uint = 0x00007000;
pub const MCDE_CONF0_OUTMUX0_SHIFT: c_int = 16;
pub const MCDE_CONF0_OUTMUX0_MASK: c_uint = 0x00070000;
pub const MCDE_CONF0_OUTMUX1_SHIFT: c_int = 19;
pub const MCDE_CONF0_OUTMUX1_MASK: c_uint = 0x00380000;
pub const MCDE_CONF0_OUTMUX2_SHIFT: c_int = 22;
pub const MCDE_CONF0_OUTMUX2_MASK: c_uint = 0x01C00000;
pub const MCDE_CONF0_OUTMUX3_SHIFT: c_int = 25;
pub const MCDE_CONF0_OUTMUX3_MASK: c_uint = 0x0E000000;
pub const MCDE_CONF0_OUTMUX4_SHIFT: c_int = 28;
pub const MCDE_CONF0_OUTMUX4_MASK: c_uint = 0x70000000;
pub const MCDE_SSP: c_uint = 0x00000008;
pub const MCDE_AIS: c_uint = 0x00000100;
pub const MCDE_IMSCERR: c_uint = 0x00000110;
pub const MCDE_RISERR: c_uint = 0x00000120;
pub const MCDE_MISERR: c_uint = 0x00000130;
pub const MCDE_SISERR: c_uint = 0x00000140;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcde_flow_mode {
// One-shot mode: flow stops after one frame
    MCDE_COMMAND_ONESHOT_FLOW,
// Command mode with tearing effect (TE) IRQ sync
    MCDE_COMMAND_TE_FLOW,
//
// Command mode with bus turn-around (BTA) and tearing effect
// (TE) IRQ sync.
//
    MCDE_COMMAND_BTA_TE_FLOW,
// Video mode with tearing effect (TE) sync IRQ
    MCDE_VIDEO_TE_FLOW,
// Video mode with the formatter itself as sync source
    MCDE_VIDEO_FORMATTER_FLOW,
// DPI video with the formatter itsels as sync source
    MCDE_DPI_FORMATTER_FLOW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcde {
    pub drm: drm_device,
    pub dev: *mut device,
    pub panel: *mut drm_panel,
    pub bridge: *mut drm_bridge,
    pub connector: *mut drm_connector,
    pub pipe: drm_simple_display_pipe,
    pub mdsi: *mut mipi_dsi_device,
    pub dpi_output: bool,
    pub stride: i16,
    pub flow_mode: mcde_flow_mode,
    pub flow_active: c_uint,
    pub /: *mut *mut spinlock_t flow_lock; / Locks the channel flow control,
    pub regs: *mut void __iomem,
    pub mcde_clk: *mut clk,
    pub lcd_clk: *mut clk,
    pub hdmi_clk: *mut clk,
// Handles to the clock dividers for FIFO A and B
    pub fifoa_clk: *mut clk,
    pub fifob_clk: *mut clk,
// Locks the MCDE FIFO control register A and B
    pub fifo_crx1_lock: spinlock_t,
    pub epod: *mut regulator,
    pub vana: *mut regulator,
}

extern "C" {
    pub fn mcde_dsi_irq(mdsi: *mut mipi_dsi_device) -> bool;
}
extern "C" {
    pub fn mcde_dsi_te_request(mdsi: *mut mipi_dsi_device);
}
extern "C" {
    pub fn mcde_dsi_enable(bridge: *mut drm_bridge);
}
extern "C" {
    pub fn mcde_dsi_disable(bridge: *mut drm_bridge);
}
extern "C" {
    pub fn mcde_display_irq(mcde: *mut mcde);
}
extern "C" {
    pub fn mcde_display_disable_irqs(mcde: *mut mcde);
}
extern "C" {
    pub fn mcde_display_init(drm: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn mcde_init_clock_divider(mcde: *mut mcde) -> c_int;
}
