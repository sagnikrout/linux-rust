//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/hisilicon/kirin/kirin_ade_reg.h
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
// Copyright (c) 2016 Linaro Limited.
// Copyright (c) 2014-2016 HiSilicon Limited.
//
// ADE Registers
//

pub const ADE_CTRL: c_uint = 0x0004;
pub const FRM_END_START_OFST: c_int = 0;

pub const AUTO_CLK_GATE_EN_OFST: c_int = 0;

pub const ADE_DISP_SRC_CFG: c_uint = 0x0018;
pub const ADE_CTRL1: c_uint = 0x008C;
pub const ADE_EN: c_uint = 0x0100;
pub const ADE_DISABLE: c_int = 0;
pub const ADE_ENABLE: c_int = 1;
// reset and reload regs

pub const RDMA_OFST: c_int = 0;
pub const CLIP_OFST: c_int = 15;
pub const SCL_OFST: c_int = 21;
pub const CTRAN_OFST: c_int = 24;

// channel regs

// overlay regs
pub const ADE_OVLY1_TRANS_CFG: c_uint = 0x002C;
pub const ADE_OVLY_CTL: c_uint = 0x0098;

pub const OUTPUT_XSIZE_OFST: c_int = 16;

pub const CH_ALP_MODE_OFST: c_int = 0;
pub const CH_ALP_SEL_OFST: c_int = 2;
pub const CH_UNDER_ALP_SEL_OFST: c_int = 4;
pub const CH_EN_OFST: c_int = 6;
pub const CH_ALP_GBL_OFST: c_int = 15;
pub const CH_SEL_OFST: c_int = 28;
// ctran regs

pub const CTRAN_BYPASS_ON: c_int = 1;
pub const CTRAN_BYPASS_OFF: c_int = 0;

// clip regs

//
// LDI Registers
//
pub const LDI_HRZ_CTRL0: c_uint = 0x7400;
pub const HBP_OFST: c_int = 20;
pub const LDI_HRZ_CTRL1: c_uint = 0x7404;
pub const LDI_VRT_CTRL0: c_uint = 0x7408;
pub const VBP_OFST: c_int = 20;
pub const LDI_VRT_CTRL1: c_uint = 0x740C;
pub const LDI_PLR_CTRL: c_uint = 0x7410;

pub const LDI_DSP_SIZE: c_uint = 0x7414;
pub const VSIZE_OFST: c_int = 20;
pub const LDI_INT_EN: c_uint = 0x741C;
pub const FRAME_END_INT_EN_OFST: c_int = 1;
pub const LDI_CTRL: c_uint = 0x7420;
pub const BPP_OFST: c_int = 3;

pub const LDI_MSK_INT: c_uint = 0x7428;
pub const LDI_INT_CLR: c_uint = 0x742C;
pub const LDI_WORK_MODE: c_uint = 0x7430;
pub const LDI_HDMI_DSI_GT: c_uint = 0x7434;
//
// ADE media bus service regs
//
pub const ADE0_QOSGENERATOR_MODE: c_uint = 0x010C;

pub const ADE0_QOSGENERATOR_EXTCONTROL: c_uint = 0x0118;

pub const ADE1_QOSGENERATOR_MODE: c_uint = 0x020C;
pub const ADE1_QOSGENERATOR_EXTCONTROL: c_uint = 0x0218;
//
// ADE regs relevant enums
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum frame_end_start {
// regs take effect in every vsync
    REG_EFFECTIVE_IN_VSYNC = 0,
// regs take effect in fist ade en and every frame end
    REG_EFFECTIVE_IN_ADEEN_FRMEND,
// regs take effect in ade en immediately
    REG_EFFECTIVE_IN_ADEEN,
// regs take effect in first vsync and every frame end
    REG_EFFECTIVE_IN_VSYNC_FRMEND
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ade_fb_format {
    ADE_RGB_565 = 0,
    ADE_BGR_565,
    ADE_XRGB_8888,
    ADE_XBGR_8888,
    ADE_ARGB_8888,
    ADE_ABGR_8888,
    ADE_RGBA_8888,
    ADE_BGRA_8888,
    ADE_RGB_888,
    ADE_BGR_888 = 9,
    ADE_FORMAT_UNSUPPORT = 800
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ade_channel {
    ADE_CH1 = 0,	/* channel 1 for primary plane */
    ADE_CH_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ade_scale {
    ADE_SCL1 = 0,
    ADE_SCL2,
    ADE_SCL3,
    ADE_SCL_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ade_ctran {
    ADE_CTRAN1 = 0,
    ADE_CTRAN2,
    ADE_CTRAN3,
    ADE_CTRAN4,
    ADE_CTRAN5,
    ADE_CTRAN6,
    ADE_CTRAN_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ade_overlay {
    ADE_OVLY1 = 0,
    ADE_OVLY2,
    ADE_OVLY3,
    ADE_OVLY_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ade_alpha_mode {
    ADE_ALP_GLOBAL = 0,
    ADE_ALP_PIXEL,
    ADE_ALP_PIXEL_AND_GLB
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ade_alpha_blending_mode {
    ADE_ALP_MUL_COEFF_0 = 0,	/* alpha */
    ADE_ALP_MUL_COEFF_1,		/* 1-alpha */
    ADE_ALP_MUL_COEFF_2,		/* 0 */
    ADE_ALP_MUL_COEFF_3		/* 1 */
}

//
// LDI regs relevant enums
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsi_pclk_en {
    DSI_PCLK_ON = 0,
    DSI_PCLK_OFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ldi_output_format {
    LDI_OUT_RGB_565 = 0,
    LDI_OUT_RGB_666,
    LDI_OUT_RGB_888
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ldi_work_mode {
    TEST_MODE = 0,
    NORMAL_MODE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ldi_input_source {
    DISP_SRC_NONE = 0,
    DISP_SRC_OVLY2,
    DISP_SRC_DISP,
    DISP_SRC_ROT,
    DISP_SRC_SCL2
}

//
// ADE media bus service relevant enums
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qos_generator_mode {
    FIXED_MODE = 0,
    LIMITER_MODE,
    BYPASS_MODE,
    REGULATOR_MODE
}

//
// Register Write/Read Helper functions
//
