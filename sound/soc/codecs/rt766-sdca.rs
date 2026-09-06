//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt766-sdca.h
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
// rt766-sdca.h -- RT766 SDCA ALSA SoC audio driver header
//
// Copyright(c) 2026 Realtek Semiconductor Corp.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt766_sdca_priv {
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub slave: *mut sdw_slave,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub hs_jack: *mut snd_soc_jack,
    pub /: *mut *mut mutex disable_irq_lock; / SDCA irq lock protection,
    pub disable_irq: bool,
    pub jack_type: c_int,
    pub fu41_dapm_mute: bool,
    pub fu41_mixer_l_mute: bool,
    pub fu41_mixer_r_mute: bool,
    pub fu113_dapm_mute: bool,
    pub fu113_mixer_mute: [bool; 4],
    pub fu21_dapm_mute: bool,
    pub fu21_mixer_l_mute: bool,
    pub fu21_mixer_r_mute: bool,
    pub fu36_dapm_mute: bool,
    pub fu36_mixer_l_mute: bool,
    pub fu36_mixer_r_mute: bool,
    pub uaj_func_data: *mut sdca_function_data,
    pub sm_func_data: *mut sdca_function_data,
    pub sa_func_data: *mut sdca_function_data,
    pub hid_func_data: *mut sdca_function_data,
    pub irq_info: *mut sdca_interrupt_info,
    pub hid: *mut hid_device,
}

// vendor registers
pub const RT766_VERSION_ID: c_uint = 0xc404;
pub const RT766_DEV_ID1: c_uint = 0xc405;
pub const RT766_DEV_ID0: c_uint = 0xc406;
pub const RT766_BOND_LATCH_ID: c_uint = 0xc407;
pub const RT766_HP_POWER_STATE: c_uint = 0x1000004;
pub const RT766_HP_FSM_CTL2_1: c_uint = 0x100000d;
// MCU Patch address
pub const RT766_MCU_PATCH_ADDR1_START: c_uint = 0x10010000;
pub const RT766_MCU_PATCH_ADDR1_END: c_uint = 0x10011fff;
pub const RT766_MCU_PATCH_ADDR2_START: c_uint = 0x10020000;
pub const RT766_MCU_PATCH_ADDR2_END: c_uint = 0x10023fff;
// Buffer address for HID
pub const RT766_BUF_ADDR_HID1: c_uint = 0x44030000;
pub const RT766_BUF_ADDR_HID2: c_uint = 0x44030020;
// SDCA (Channel)
pub const RT766_CH_1: c_uint = 0x01;
pub const RT766_CH_2: c_uint = 0x02;
pub const RT766_CH_3: c_uint = 0x03;
pub const RT766_CH_4: c_uint = 0x04;
// RT766 SDCA Control - function number
pub const RT766_FUNC_NUM_UAJ: c_uint = 0x01;
pub const RT766_FUNC_NUM_MIC: c_uint = 0x02;
pub const RT766_FUNC_NUM_HID: c_uint = 0x03;
pub const RT766_FUNC_NUM_AMP: c_uint = 0x04;
// RT766 SDCA entity
pub const RT766_SDCA_ENT_0: c_uint = 0x00;
pub const RT766_SDCA_ENT_HID101: c_uint = 0x01;
pub const RT766_SDCA_ENT_GE49: c_uint = 0x49;
pub const RT766_SDCA_ENT_USER_FU41: c_uint = 0x05;
pub const RT766_SDCA_ENT_USER_FU36: c_uint = 0x0f;
pub const RT766_SDCA_ENT_USER_FU21: c_uint = 0x03;
pub const RT766_SDCA_ENT_USER_FU113: c_uint = 0x30;
pub const RT766_SDCA_ENT_PDE23: c_uint = 0x33;
pub const RT766_SDCA_ENT_PDE47: c_uint = 0x28;
pub const RT766_SDCA_ENT_PDE11: c_uint = 0x2a;
pub const RT766_SDCA_ENT_PDE34: c_uint = 0x29;
pub const RT766_SDCA_ENT_CS41: c_uint = 0x01;
pub const RT766_SDCA_ENT_CS36: c_uint = 0x11;
pub const RT766_SDCA_ENT_CS113: c_uint = 0x12;
pub const RT766_SDCA_ENT_CS21: c_uint = 0x21;
pub const RT766_SDCA_ENT_PLATFORM_FU33: c_uint = 0x44;
pub const RT766_SDCA_ENT_PPU21: c_uint = 0x04;
// sample frequency index
pub const RT766_SDCA_RATE_44100HZ: c_uint = 0x08;
pub const RT766_SDCA_RATE_48000HZ: c_uint = 0x09;
pub const RT766_SDCA_RATE_96000HZ: c_uint = 0x0b;
pub const RT766_SDCA_RATE_192000HZ: c_uint = 0x0d;
// SDCA Register macros

extern "C" {
    pub fn rt766_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int;
}
extern "C" {
    pub fn rt766_sdca_init(dev: *mut device, regmap: *mut regmap, slave: *mut sdw_slave) -> c_int;
}
