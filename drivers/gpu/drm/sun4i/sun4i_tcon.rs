//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sun4i/sun4i_tcon.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2015 Free Electrons
// Copyright (C) 2015 NextThing Co
//
// Boris Brezillon <boris.brezillon@free-electrons.com>
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

pub const SUN4I_TCON_GCTL_REG: c_uint = 0x0;

pub const SUN4I_TCON_GINT0_REG: c_uint = 0x4;

pub const SUN4I_TCON_GINT1_REG: c_uint = 0x8;
pub const SUN4I_TCON_FRM_CTL_REG: c_uint = 0x10;

pub const SUN4I_TCON0_FRM_SEED_PR_REG: c_uint = 0x14;
pub const SUN4I_TCON0_FRM_SEED_PG_REG: c_uint = 0x18;
pub const SUN4I_TCON0_FRM_SEED_PB_REG: c_uint = 0x1c;
pub const SUN4I_TCON0_FRM_SEED_LR_REG: c_uint = 0x20;
pub const SUN4I_TCON0_FRM_SEED_LG_REG: c_uint = 0x24;
pub const SUN4I_TCON0_FRM_SEED_LB_REG: c_uint = 0x28;
pub const SUN4I_TCON0_FRM_TBL0_REG: c_uint = 0x2c;
pub const SUN4I_TCON0_FRM_TBL1_REG: c_uint = 0x30;
pub const SUN4I_TCON0_FRM_TBL2_REG: c_uint = 0x34;
pub const SUN4I_TCON0_FRM_TBL3_REG: c_uint = 0x38;
pub const SUN4I_TCON0_CTL_REG: c_uint = 0x40;

pub const SUN4I_TCON0_DCLK_REG: c_uint = 0x44;

pub const SUN4I_TCON0_BASIC0_REG: c_uint = 0x48;

pub const SUN4I_TCON0_BASIC1_REG: c_uint = 0x4c;

pub const SUN4I_TCON0_BASIC2_REG: c_uint = 0x50;

pub const SUN4I_TCON0_BASIC3_REG: c_uint = 0x54;

pub const SUN4I_TCON0_HV_IF_REG: c_uint = 0x58;
pub const SUN4I_TCON0_CPU_IF_REG: c_uint = 0x60;

pub const SUN4I_TCON0_CPU_WR_REG: c_uint = 0x64;
pub const SUN4I_TCON0_CPU_RD0_REG: c_uint = 0x68;
pub const SUN4I_TCON0_CPU_RDA_REG: c_uint = 0x6c;
pub const SUN4I_TCON0_TTL0_REG: c_uint = 0x70;
pub const SUN4I_TCON0_TTL1_REG: c_uint = 0x74;
pub const SUN4I_TCON0_TTL2_REG: c_uint = 0x78;
pub const SUN4I_TCON0_TTL3_REG: c_uint = 0x7c;
pub const SUN4I_TCON0_TTL4_REG: c_uint = 0x80;
pub const SUN4I_TCON0_LVDS_IF_REG: c_uint = 0x84;

pub const SUN4I_TCON0_IO_POL_REG: c_uint = 0x88;

pub const SUN4I_TCON0_IO_TRI_REG: c_uint = 0x8c;

pub const SUN4I_TCON1_CTL_REG: c_uint = 0x90;

pub const SUN4I_TCON1_BASIC0_REG: c_uint = 0x94;

pub const SUN4I_TCON1_BASIC1_REG: c_uint = 0x98;

pub const SUN4I_TCON1_BASIC2_REG: c_uint = 0x9c;

pub const SUN4I_TCON1_BASIC3_REG: c_uint = 0xa0;

pub const SUN4I_TCON1_BASIC4_REG: c_uint = 0xa4;

pub const SUN4I_TCON1_BASIC5_REG: c_uint = 0xa8;

pub const SUN4I_TCON1_IO_POL_REG: c_uint = 0xf0;
// there is no documentation about this bit

pub const SUN4I_TCON1_IO_TRI_REG: c_uint = 0xf4;
pub const SUN4I_TCON_ECC_FIFO_REG: c_uint = 0xf8;

pub const SUN4I_TCON_CEU_CTL_REG: c_uint = 0x100;
pub const SUN4I_TCON_CEU_MUL_RR_REG: c_uint = 0x110;
pub const SUN4I_TCON_CEU_MUL_RG_REG: c_uint = 0x114;
pub const SUN4I_TCON_CEU_MUL_RB_REG: c_uint = 0x118;
pub const SUN4I_TCON_CEU_ADD_RC_REG: c_uint = 0x11c;
pub const SUN4I_TCON_CEU_MUL_GR_REG: c_uint = 0x120;
pub const SUN4I_TCON_CEU_MUL_GG_REG: c_uint = 0x124;
pub const SUN4I_TCON_CEU_MUL_GB_REG: c_uint = 0x128;
pub const SUN4I_TCON_CEU_ADD_GC_REG: c_uint = 0x12c;
pub const SUN4I_TCON_CEU_MUL_BR_REG: c_uint = 0x130;
pub const SUN4I_TCON_CEU_MUL_BG_REG: c_uint = 0x134;
pub const SUN4I_TCON_CEU_MUL_BB_REG: c_uint = 0x138;
pub const SUN4I_TCON_CEU_ADD_BC_REG: c_uint = 0x13c;
pub const SUN4I_TCON_CEU_RANGE_R_REG: c_uint = 0x140;
pub const SUN4I_TCON_CEU_RANGE_G_REG: c_uint = 0x144;
pub const SUN4I_TCON_CEU_RANGE_B_REG: c_uint = 0x148;
pub const SUN4I_TCON0_CPU_TRI0_REG: c_uint = 0x160;

pub const SUN4I_TCON0_CPU_TRI1_REG: c_uint = 0x164;

pub const SUN4I_TCON0_CPU_TRI2_REG: c_uint = 0x168;

pub const SUN4I_TCON_SAFE_PERIOD_REG: c_uint = 0x1f0;

pub const SUN4I_TCON_MUX_CTRL_REG: c_uint = 0x200;
pub const SUN4I_TCON0_LVDS_ANA0_REG: c_uint = 0x220;

pub const SUN4I_TCON0_LVDS_ANA1_REG: c_uint = 0x224;

pub const SUN4I_TCON1_FILL_CTL_REG: c_uint = 0x300;
pub const SUN4I_TCON1_FILL_BEG0_REG: c_uint = 0x304;
pub const SUN4I_TCON1_FILL_END0_REG: c_uint = 0x308;
pub const SUN4I_TCON1_FILL_DATA0_REG: c_uint = 0x30c;
pub const SUN4I_TCON1_FILL_BEG1_REG: c_uint = 0x310;
pub const SUN4I_TCON1_FILL_END1_REG: c_uint = 0x314;
pub const SUN4I_TCON1_FILL_DATA1_REG: c_uint = 0x318;
pub const SUN4I_TCON1_FILL_BEG2_REG: c_uint = 0x31c;
pub const SUN4I_TCON1_FILL_END2_REG: c_uint = 0x320;
pub const SUN4I_TCON1_FILL_DATA2_REG: c_uint = 0x324;
pub const SUN4I_TCON1_GAMMA_TABLE_REG: c_uint = 0x400;
pub const SUN4I_TCON_MAX_CHANNELS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_tcon_quirks {
    pub /: *mut *mut bool has_channel_0; / a83t does not have channel 0 on second TCON,
    pub /: *mut *mut bool has_channel_1; / a33 does not have channel 1,
    pub /: *mut *mut bool has_lvds_alt; / Does the LVDS clock have a parent other than the TCON clock?,
    pub /: *mut *mut bool needs_de_be_mux; / sun6i needs mux to select backend,
    pub /: *mut *mut bool needs_edp_reset; / a80 edp reset needed for tcon0 access,
    pub /: *mut *mut bool supports_lvds; / Does the TCON support an LVDS output?,
    pub /: *mut *mut bool polarity_in_ch0; / some tcon1 channels have polarity bits in tcon0 pol register,
    pub /: *mut *mut u8 dclk_min_div; / minimum divider for TCON0 DCLK,
// callback to handle tcon muxing options
    pub ): *const *const *const int (set_mux)(struct sun4i_tcon , struct drm_encoder,
// handler for LVDS setup routine
    pub encoder): *const drm_encoder,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_tcon {
    pub dev: *mut device,
    pub drm: *mut drm_device,
    pub regs: *mut regmap,
// Main bus clock
    pub clk: *mut clk,
// Clocks for the TCON channels
    pub sclk0: *mut clk,
    pub sclk1: *mut clk,
// Possible mux for the LVDS clock
    pub lvds_pll: *mut clk,
// Pixel clock
    pub dclk: *mut clk,
    pub dclk_max_div: u8,
    pub dclk_min_div: u8,
// Reset control
    pub lcd_rst: *mut reset_control,
    pub lvds_rst: *mut reset_control,
// Platform adjustments
    pub quirks: *const sun4i_tcon_quirks,
// Associated crtc
    pub crtc: *mut sun4i_crtc,
    pub id: c_int,
// TCON list management
    pub list: list_head,
}

extern "C" {
    pub fn sun4i_tcon_enable_vblank(tcon: *mut sun4i_tcon, enable: bool);
}
