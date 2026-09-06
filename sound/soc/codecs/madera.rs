//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/madera.h
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
// Cirrus Logic Madera class codecs common support
//
// Copyright (C) 2015-2018 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

pub const MADERA_FLL1_REFCLK: c_int = 1;
pub const MADERA_FLL2_REFCLK: c_int = 2;
pub const MADERA_FLL3_REFCLK: c_int = 3;
pub const MADERA_FLLAO_REFCLK: c_int = 4;
pub const MADERA_FLL1_SYNCCLK: c_int = 5;
pub const MADERA_FLL2_SYNCCLK: c_int = 6;
pub const MADERA_FLL3_SYNCCLK: c_int = 7;
pub const MADERA_FLLAO_SYNCCLK: c_int = 8;

pub const MADERA_FLL_SRC_MCLK1: c_int = 0;
pub const MADERA_FLL_SRC_MCLK2: c_int = 1;
pub const MADERA_FLL_SRC_MCLK3: c_int = 2;
pub const MADERA_FLL_SRC_SLIMCLK: c_int = 3;
pub const MADERA_FLL_SRC_FLL1: c_int = 4;
pub const MADERA_FLL_SRC_FLL2: c_int = 5;
pub const MADERA_FLL_SRC_AIF1BCLK: c_int = 8;
pub const MADERA_FLL_SRC_AIF2BCLK: c_int = 9;
pub const MADERA_FLL_SRC_AIF3BCLK: c_int = 10;
pub const MADERA_FLL_SRC_AIF4BCLK: c_int = 11;
pub const MADERA_FLL_SRC_AIF1LRCLK: c_int = 12;
pub const MADERA_FLL_SRC_AIF2LRCLK: c_int = 13;
pub const MADERA_FLL_SRC_AIF3LRCLK: c_int = 14;
pub const MADERA_FLL_SRC_AIF4LRCLK: c_int = 15;
pub const MADERA_CLK_SYSCLK_1: c_int = 1;
pub const MADERA_CLK_ASYNCCLK_1: c_int = 2;
pub const MADERA_CLK_OPCLK: c_int = 3;
pub const MADERA_CLK_ASYNC_OPCLK: c_int = 4;
pub const MADERA_CLK_SYSCLK_2: c_int = 5;
pub const MADERA_CLK_SYSCLK_3: c_int = 6;
pub const MADERA_CLK_ASYNCCLK_2: c_int = 7;
pub const MADERA_CLK_DSPCLK: c_int = 8;
pub const MADERA_CLK_OUTCLK: c_int = 9;
pub const MADERA_CLK_SRC_MCLK1: c_uint = 0x0;
pub const MADERA_CLK_SRC_MCLK2: c_uint = 0x1;
pub const MADERA_CLK_SRC_MCLK3: c_uint = 0x2;
pub const MADERA_CLK_SRC_FLL1: c_uint = 0x4;
pub const MADERA_CLK_SRC_FLL2: c_uint = 0x5;
pub const MADERA_CLK_SRC_FLL3: c_uint = 0x6;
pub const MADERA_CLK_SRC_FLLAO_HI: c_uint = 0x7;
pub const MADERA_CLK_SRC_FLL1_DIV6: c_uint = 0x7;
pub const MADERA_CLK_SRC_AIF1BCLK: c_uint = 0x8;
pub const MADERA_CLK_SRC_AIF2BCLK: c_uint = 0x9;
pub const MADERA_CLK_SRC_AIF3BCLK: c_uint = 0xA;
pub const MADERA_CLK_SRC_AIF4BCLK: c_uint = 0xB;
pub const MADERA_CLK_SRC_FLLAO: c_uint = 0xF;
pub const MADERA_OUTCLK_SYSCLK: c_int = 0;
pub const MADERA_OUTCLK_ASYNCCLK: c_int = 1;
pub const MADERA_OUTCLK_MCLK1: c_int = 4;
pub const MADERA_OUTCLK_MCLK2: c_int = 5;
pub const MADERA_OUTCLK_MCLK3: c_int = 6;
pub const MADERA_MIXER_VOL_MASK: c_uint = 0x00FE;
pub const MADERA_MIXER_VOL_SHIFT: c_int = 1;
pub const MADERA_MIXER_VOL_WIDTH: c_int = 7;
pub const MADERA_DOM_GRP_FX: c_int = 0;
pub const MADERA_DOM_GRP_ASRC1: c_int = 1;
pub const MADERA_DOM_GRP_ASRC2: c_int = 2;
pub const MADERA_DOM_GRP_ISRC1: c_int = 3;
pub const MADERA_DOM_GRP_ISRC2: c_int = 4;
pub const MADERA_DOM_GRP_ISRC3: c_int = 5;
pub const MADERA_DOM_GRP_ISRC4: c_int = 6;
pub const MADERA_DOM_GRP_OUT: c_int = 7;
pub const MADERA_DOM_GRP_SPD: c_int = 8;
pub const MADERA_DOM_GRP_DSP1: c_int = 9;
pub const MADERA_DOM_GRP_DSP2: c_int = 10;
pub const MADERA_DOM_GRP_DSP3: c_int = 11;
pub const MADERA_DOM_GRP_DSP4: c_int = 12;
pub const MADERA_DOM_GRP_DSP5: c_int = 13;
pub const MADERA_DOM_GRP_DSP6: c_int = 14;
pub const MADERA_DOM_GRP_DSP7: c_int = 15;
pub const MADERA_DOM_GRP_AIF1: c_int = 16;
pub const MADERA_DOM_GRP_AIF2: c_int = 17;
pub const MADERA_DOM_GRP_AIF3: c_int = 18;
pub const MADERA_DOM_GRP_AIF4: c_int = 19;
pub const MADERA_DOM_GRP_SLIMBUS: c_int = 20;
pub const MADERA_DOM_GRP_PWM: c_int = 21;
pub const MADERA_DOM_GRP_DFC: c_int = 22;
pub const MADERA_N_DOM_GRPS: c_int = 23;
pub const MADERA_MAX_DAI: c_int = 11;
pub const MADERA_MAX_ADSP: c_int = 7;
pub const MADERA_NUM_MIXER_INPUTS: c_int = 148;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct madera_voice_trigger_info {
// Which core triggered, 1-based (1 = DSP1, ...)
    pub core_num: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct madera_dai_priv {
    pub clk: c_int,
    pub constraint: snd_pcm_hw_constraint_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct madera_priv {
    pub adsp: [wm_adsp; MADERA_MAX_ADSP],
    pub madera: *mut madera,
    pub dev: *mut device,
    pub sysclk: c_int,
    pub asyncclk: c_int,
    pub dspclk: c_int,
    pub dai: [madera_dai_priv; MADERA_MAX_DAI],
    pub num_inputs: c_int,
    pub in_pending: c_uint,
    pub out_up_pending: c_uint,
    pub out_up_delay: c_uint,
    pub out_down_pending: c_uint,
    pub out_down_delay: c_uint,
    pub adsp_rate_cache: [c_uint; MADERA_MAX_ADSP],
    pub rate_lock: mutex,
    pub tdm_width: [c_int; MADERA_MAX_AIF],
    pub tdm_slots: [c_int; MADERA_MAX_AIF],
    pub domain_group_ref: [c_int; MADERA_N_DOM_GRPS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct madera_fll_cfg {
    pub n: c_int,
    pub theta: c_uint,
    pub lambda: c_uint,
    pub refdiv: c_int,
    pub fratio: c_int,
    pub gain: c_int,
    pub alt_gain: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct madera_fll {
    pub madera: *mut madera,
    pub id: c_int,
    pub base: c_uint,
    pub fout: c_uint,
    pub sync_src: c_int,
    pub sync_freq: c_uint,
    pub ref_src: c_int,
    pub ref_freq: c_uint,
    pub ref_cfg: madera_fll_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct madera_enum {
    pub mixer_enum: soc_enum,
    pub val: c_int,
}

pub const MADERA_OSR_ENUM_SIZE: c_int = 5;
pub const MADERA_SYNC_RATE_ENUM_SIZE: c_int = 3;
pub const MADERA_ASYNC_RATE_ENUM_SIZE: c_int = 2;

pub const MADERA_SAMPLE_RATE_ENUM_SIZE: c_int = 16;
pub const MADERA_DFC_TYPE_ENUM_SIZE: c_int = 5;
pub const MADERA_DFC_WIDTH_ENUM_SIZE: c_int = 5;
extern "C" {
    pub fn madera_core_init(priv: *mut madera_priv) -> c_int;
}
extern "C" {
    pub fn madera_core_free(priv: *mut madera_priv) -> c_int;
}
extern "C" {
    pub fn madera_init_overheat(priv: *mut madera_priv) -> c_int;
}
extern "C" {
    pub fn madera_free_overheat(priv: *mut madera_priv) -> c_int;
}
extern "C" {
    pub fn madera_init_inputs(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn madera_free_bus_error_irq(priv: *mut madera_priv, dsp_num: c_int);
}
extern "C" {
    pub fn madera_init_dai(priv: *mut madera_priv, id: c_int) -> c_int;
}
// Following functions are for use by machine drivers
extern "C" {
    pub fn blocking_notifier_chain_register(_arg: &madera->notifier, _arg: nb) -> return;
}
extern "C" {
    pub fn blocking_notifier_chain_unregister(_arg: &madera->notifier, _arg: nb) -> return;
}
