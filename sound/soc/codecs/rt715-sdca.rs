//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt715-sdca.h
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
// rt715-sdca.h -- RT715 ALSA SoC audio driver header
//
// Copyright(c) 2020 Realtek Semiconductor Corp.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt715_sdca_priv {
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub codec: *mut snd_soc_codec,
    pub slave: *mut sdw_slave,
    pub adc_mute_work: delayed_work,
    pub dbg_nid: c_int,
    pub dbg_vid: c_int,
    pub dbg_payload: c_int,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub l_is_unmute: c_int,
    pub r_is_unmute: c_int,
    pub hw_sdw_ver: c_int,
    pub kctl_switch_orig: [c_int; 4],
    pub kctl_2ch_orig: [c_int; 2],
    pub kctl_4ch_orig: [c_int; 4],
    pub kctl_8ch_orig: [c_int; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt715_sdca_kcontrol_private {
    pub reg_base: c_uint,
    pub count: c_uint,
    pub max: c_uint,
    pub shift: c_uint,
    pub invert: c_uint,
}

// MIPI Register
pub const RT715_INT_CTRL: c_uint = 0x005a;
pub const RT715_INT_MASK: c_uint = 0x005e;
// NID
pub const RT715_AUDIO_FUNCTION_GROUP: c_uint = 0x01;
pub const RT715_MIC_ADC: c_uint = 0x07;
pub const RT715_LINE_ADC: c_uint = 0x08;
pub const RT715_MIX_ADC: c_uint = 0x09;
pub const RT715_DMIC1: c_uint = 0x12;
pub const RT715_DMIC2: c_uint = 0x13;
pub const RT715_MIC1: c_uint = 0x18;
pub const RT715_MIC2: c_uint = 0x19;
pub const RT715_LINE1: c_uint = 0x1a;
pub const RT715_LINE2: c_uint = 0x1b;
pub const RT715_DMIC3: c_uint = 0x1d;
pub const RT715_DMIC4: c_uint = 0x29;
pub const RT715_VENDOR_REG: c_uint = 0x20;
pub const RT715_MUX_IN1: c_uint = 0x22;
pub const RT715_MUX_IN2: c_uint = 0x23;
pub const RT715_MUX_IN3: c_uint = 0x24;
pub const RT715_MUX_IN4: c_uint = 0x25;
pub const RT715_MIX_ADC2: c_uint = 0x27;
pub const RT715_INLINE_CMD: c_uint = 0x55;
pub const RT715_VENDOR_HDA_CTL: c_uint = 0x61;
// Index (NID:20h)
pub const RT715_PRODUCT_NUM: c_uint = 0x0;
pub const RT715_IRQ_CTRL: c_uint = 0x2b;
pub const RT715_AD_FUNC_EN: c_uint = 0x36;
pub const RT715_REV_1: c_uint = 0x37;
pub const RT715_SDW_INPUT_SEL: c_uint = 0x39;
pub const RT715_DFLL_VAD: c_uint = 0x44;
pub const RT715_EXT_DMIC_CLK_CTRL2: c_uint = 0x54;
// Index (NID:61h)
pub const RT715_HDA_LEGACY_MUX_CTL1: c_uint = 0x00;
// SDCA (Function)
pub const FUN_JACK_CODEC: c_uint = 0x01;
pub const FUN_MIC_ARRAY: c_uint = 0x02;
pub const FUN_HID: c_uint = 0x03;
// SDCA (Entity)
pub const RT715_SDCA_ST_EN: c_uint = 0x00;
pub const RT715_SDCA_CS_FREQ_IND_EN: c_uint = 0x01;
pub const RT715_SDCA_FU_ADC8_9_VOL: c_uint = 0x02;
pub const RT715_SDCA_SMPU_TRIG_ST_EN: c_uint = 0x05;
pub const RT715_SDCA_FU_ADC10_11_VOL: c_uint = 0x06;
pub const RT715_SDCA_FU_ADC7_27_VOL: c_uint = 0x0a;
pub const RT715_SDCA_FU_AMIC_GAIN_EN: c_uint = 0x0c;
pub const RT715_SDCA_FU_DMIC_GAIN_EN: c_uint = 0x0e;
pub const RT715_SDCA_CX_CLK_SEL_EN: c_uint = 0x10;
pub const RT715_SDCA_CREQ_POW_EN: c_uint = 0x18;
// SDCA (Control)
pub const RT715_SDCA_ST_CTRL: c_uint = 0x00;
pub const RT715_SDCA_CX_CLK_SEL_CTRL: c_uint = 0x01;
pub const RT715_SDCA_REQ_POW_CTRL: c_uint = 0x01;
pub const RT715_SDCA_FU_MUTE_CTRL: c_uint = 0x01;
pub const RT715_SDCA_FU_VOL_CTRL: c_uint = 0x02;
pub const RT715_SDCA_FU_DMIC_GAIN_CTRL: c_uint = 0x0b;
pub const RT715_SDCA_FREQ_IND_CTRL: c_uint = 0x10;
pub const RT715_SDCA_SMPU_TRIG_EN_CTRL: c_uint = 0x10;
pub const RT715_SDCA_SMPU_TRIG_ST_CTRL: c_uint = 0x11;
// SDCA (Channel)
pub const CH_00: c_uint = 0x00;
pub const CH_01: c_uint = 0x01;
pub const CH_02: c_uint = 0x02;
pub const CH_03: c_uint = 0x03;
pub const CH_04: c_uint = 0x04;
pub const CH_05: c_uint = 0x05;
pub const CH_06: c_uint = 0x06;
pub const CH_07: c_uint = 0x07;
pub const CH_08: c_uint = 0x08;
pub const RT715_SDCA_DB_STEP: c_int = 375;
extern "C" {
    pub fn rt715_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int;
}
