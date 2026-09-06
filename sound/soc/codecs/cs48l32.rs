//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs48l32.h
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
// Cirrus Logic CS48L32 audio DSP.
//
// Copyright (C) 2016-2018, 2020, 2022, 2025 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

pub const CS48L32_SILICON_ID: c_uint = 0x48a32;
pub const CS48L32_32K_MCLK1: c_int = 0;
pub const CS48L32_SFT_RESET_MAGIC: c_uint = 0x5a000000;
pub const CS48L32_SOFT_RESET_US: c_int = 2000;
pub const CS48L32_HARD_RESET_MIN_US: c_int = 1000;

pub const CS48L32_BOOT_TIMEOUT_US: c_int = 25000;
pub const CS48L32_ASP_ENABLES1: c_uint = 0x00;
pub const CS48L32_ASP_CONTROL1: c_uint = 0x04;
pub const CS48L32_ASP_CONTROL2: c_uint = 0x08;
pub const CS48L32_ASP_CONTROL3: c_uint = 0x0c;
pub const CS48L32_ASP_FRAME_CONTROL1: c_uint = 0x10;
pub const CS48L32_ASP_FRAME_CONTROL2: c_uint = 0x14;
pub const CS48L32_ASP_FRAME_CONTROL5: c_uint = 0x20;
pub const CS48L32_ASP_FRAME_CONTROL6: c_uint = 0x24;
pub const CS48L32_ASP_DATA_CONTROL1: c_uint = 0x30;
pub const CS48L32_ASP_DATA_CONTROL5: c_uint = 0x40;
pub const CS48L32_SYSCLK_RATE_6MHZ: c_int = 0;
pub const CS48L32_SYSCLK_RATE_12MHZ: c_int = 1;
pub const CS48L32_SYSCLK_RATE_24MHZ: c_int = 2;
pub const CS48L32_SYSCLK_RATE_49MHZ: c_int = 3;
pub const CS48L32_SYSCLK_RATE_98MHZ: c_int = 4;
pub const CS48L32_FLLHJ_INT_MAX_N: c_int = 1023;
pub const CS48L32_FLLHJ_INT_MIN_N: c_int = 1;
pub const CS48L32_FLLHJ_FRAC_MAX_N: c_int = 255;
pub const CS48L32_FLLHJ_FRAC_MIN_N: c_int = 2;
pub const CS48L32_FLLHJ_LP_INT_MODE_THRESH: c_int = 100000;
pub const CS48L32_FLLHJ_LOW_THRESH: c_int = 192000;
pub const CS48L32_FLLHJ_MID_THRESH: c_int = 1152000;
pub const CS48L32_FLLHJ_MAX_THRESH: c_int = 13000000;
pub const CS48L32_FLLHJ_LOW_GAINS: c_uint = 0x23f0;
pub const CS48L32_FLLHJ_MID_GAINS: c_uint = 0x22f2;
pub const CS48L32_FLLHJ_HIGH_GAINS: c_uint = 0x21f0;
pub const CS48L32_FLL_MAX_FOUT: c_int = 50000000;
pub const CS48L32_FLL_MAX_REFDIV: c_int = 8;
pub const CS48L32_FLL_CONTROL1_OFFS: c_uint = 0x00;
pub const CS48L32_FLL_CONTROL2_OFFS: c_uint = 0x04;
pub const CS48L32_FLL_CONTROL3_OFFS: c_uint = 0x08;
pub const CS48L32_FLL_CONTROL4_OFFS: c_uint = 0x0c;
pub const CS48L32_FLL_CONTROL5_OFFS: c_uint = 0x10;
pub const CS48L32_FLL_CONTROL6_OFFS: c_uint = 0x14;
pub const CS48L32_FLL_DIGITAL_TEST2_OFFS: c_uint = 0x34;
pub const CS48L32_FLL_GPIO_CLOCK_OFFS: c_uint = 0xa0;
pub const CS48L32_DSP_CLOCK_FREQ_OFFS: c_uint = 0x00000;
pub const CS48L32_ASP_FMT_DSP_MODE_A: c_int = 0;
pub const CS48L32_ASP_FMT_DSP_MODE_B: c_int = 1;
pub const CS48L32_ASP_FMT_I2S_MODE: c_int = 2;
pub const CS48L32_ASP_FMT_LEFT_JUSTIFIED_MODE: c_int = 3;
pub const CS48L32_HALO_SAMPLE_RATE_RX1: c_uint = 0x00080;
pub const CS48L32_HALO_SAMPLE_RATE_TX1: c_uint = 0x00280;
pub const CS48L32_HALO_DSP_RATE_MASK: c_uint = 0x1f;
pub const CS48L32_PDMCLK_SRC_IN1_PDMCLK: c_uint = 0x0;
pub const CS48L32_PDMCLK_SRC_IN2_PDMCLK: c_uint = 0x1;
pub const CS48L32_PDMCLK_SRC_IN3_PDMCLK: c_uint = 0x2;
pub const CS48L32_PDMCLK_SRC_IN4_PDMCLK: c_uint = 0x3;
pub const CS48L32_PDMCLK_SRC_AUXPDM1_CLK: c_uint = 0x8;
pub const CS48L32_PDMCLK_SRC_AUXPDM2_CLK: c_uint = 0x9;
pub const CS48L32_MAX_DAI: c_int = 6;
pub const CS48L32_MAX_INPUT: c_int = 4;
pub const CS48L32_MAX_ANALOG_INPUT: c_int = 2;
pub const CS48L32_MAX_IN_MUX_WAYS: c_int = 2;
pub const CS48L32_MAX_ASP: c_int = 2;
pub const CS48L32_EQ_BLOCK_SZ: c_int = 60;
pub const CS48L32_N_EQ_BLOCKS: c_int = 4;
pub const CS48L32_DSP_N_RX_CHANNELS: c_int = 8;
pub const CS48L32_DSP_N_TX_CHANNELS: c_int = 8;
pub const CS48L32_LHPF_MAX_COEFF: c_int = 4095;
pub const CS48L32_EQ_MAX_COEFF: c_int = 4095;

// these have a subseq number so they run after SYSCLK and DSPCLK widgets

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs48l32_enum {
    pub mixer_enum: soc_enum,
    pub val: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs48l32_eq_control {
    pub reg: c_uint,
    pub shift: c_uint,
    pub block_base: c_uint,
    pub max: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs48l32_dai_priv {
    pub clk: c_int,
    pub constraint: snd_pcm_hw_constraint_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs48l32_dsp_power_reg_block {
    pub start: c_uint,
    pub end: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs48l32_dsp_power_regs {
    pub pwd: *const c_uint,
    pub n_pwd: c_uint,
    pub ext: *const cs48l32_dsp_power_reg_block,
    pub n_ext: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs48l32_fll_cfg {
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
pub struct cs48l32_fll {
    pub codec: *mut cs48l32_codec,
    pub id: c_int,
    pub base: c_uint,
    pub sts_addr: c_uint,
    pub sts_mask: c_uint,
    pub fout: c_uint,
    pub ref_src: c_int,
    pub ref_freq: c_uint,
    pub ref_cfg: cs48l32_fll_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs48l32_codec {
    pub /: *mut *mut wm_adsp dsp; / must be first,
    pub core: cs48l32,
    pub sysclk: c_int,
    pub dspclk: c_int,
    pub dai: [cs48l32_dai_priv; CS48L32_MAX_DAI],
    pub fll: cs48l32_fll,
    pub in_up_pending: c_uint,
    pub in_vu_reg: c_uint,
    pub rate_lock: mutex,
    pub CS48L32_DSP_N_TX_CHANNELS]: u8 dsp_dma_rates[CS48L32_DSP_N_RX_CHANNELS +,
    pub in_type: [u8; CS48L32_MAX_ANALOG_INPUT][CS48L32_MAX_IN_MUX_WAYS],
    pub pdm_sup: [u8; CS48L32_MAX_ANALOG_INPUT],
    pub tdm_width: [u8; CS48L32_MAX_ASP],
    pub tdm_slots: [u8; CS48L32_MAX_ASP],
    pub eq_mode: [c_uint; CS48L32_N_EQ_BLOCKS],
    pub 2]: __be16 eq_coefficients[CS48L32_N_EQ_BLOCKS][CS48L32_EQ_BLOCK_SZ /,
    pub dsp_power_regs: *const cs48l32_dsp_power_regs,
}

extern "C" {
    pub fn cs48l32_apply_patch(cs48l32: *mut cs48l32) -> c_int;
}
extern "C" {
    pub fn cs48l32_create_regmap(spi: *mut spi_device, cs48l32: *mut cs48l32) -> c_int;
}
extern "C" {
    pub fn cs48l32_enable_asp1_pins(cs48l32_codec: *mut cs48l32_codec) -> c_int;
}
extern "C" {
    pub fn cs48l32_enable_asp2_pins(cs48l32_codec: *mut cs48l32_codec) -> c_int;
}
extern "C" {
    pub fn cs48l32_micvdd_voltage_index(voltage: u32) -> c_int;
}
extern "C" {
    pub fn cs48l32_micbias1_voltage_index(voltage: u32) -> c_int;
}
