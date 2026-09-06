//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/arizona/core.h
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
// Arizona MFD internals
//
// Copyright 2012 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

pub const ARIZONA_MAX_CORE_SUPPLIES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arizona_type {
    WM5102 = 1,
    WM5110 = 2,
    WM8997 = 3,
    WM8280 = 4,
    WM8998 = 5,
    WM1814 = 6,
    WM1831 = 7,
    CS47L24 = 8,
}

pub const ARIZONA_IRQ_GP1: c_int = 0;
pub const ARIZONA_IRQ_GP2: c_int = 1;
pub const ARIZONA_IRQ_GP3: c_int = 2;
pub const ARIZONA_IRQ_GP4: c_int = 3;
pub const ARIZONA_IRQ_GP5_FALL: c_int = 4;
pub const ARIZONA_IRQ_GP5_RISE: c_int = 5;
pub const ARIZONA_IRQ_JD_FALL: c_int = 6;
pub const ARIZONA_IRQ_JD_RISE: c_int = 7;
pub const ARIZONA_IRQ_DSP1_RAM_RDY: c_int = 8;
pub const ARIZONA_IRQ_DSP2_RAM_RDY: c_int = 9;
pub const ARIZONA_IRQ_DSP3_RAM_RDY: c_int = 10;
pub const ARIZONA_IRQ_DSP4_RAM_RDY: c_int = 11;
pub const ARIZONA_IRQ_DSP_IRQ1: c_int = 12;
pub const ARIZONA_IRQ_DSP_IRQ2: c_int = 13;
pub const ARIZONA_IRQ_DSP_IRQ3: c_int = 14;
pub const ARIZONA_IRQ_DSP_IRQ4: c_int = 15;
pub const ARIZONA_IRQ_DSP_IRQ5: c_int = 16;
pub const ARIZONA_IRQ_DSP_IRQ6: c_int = 17;
pub const ARIZONA_IRQ_DSP_IRQ7: c_int = 18;
pub const ARIZONA_IRQ_DSP_IRQ8: c_int = 19;
pub const ARIZONA_IRQ_SPK_OVERHEAT_WARN: c_int = 20;
pub const ARIZONA_IRQ_SPK_OVERHEAT: c_int = 21;
pub const ARIZONA_IRQ_MICDET: c_int = 22;
pub const ARIZONA_IRQ_HPDET: c_int = 23;
pub const ARIZONA_IRQ_WSEQ_DONE: c_int = 24;
pub const ARIZONA_IRQ_DRC2_SIG_DET: c_int = 25;
pub const ARIZONA_IRQ_DRC1_SIG_DET: c_int = 26;
pub const ARIZONA_IRQ_ASRC2_LOCK: c_int = 27;
pub const ARIZONA_IRQ_ASRC1_LOCK: c_int = 28;
pub const ARIZONA_IRQ_UNDERCLOCKED: c_int = 29;
pub const ARIZONA_IRQ_OVERCLOCKED: c_int = 30;
pub const ARIZONA_IRQ_FLL2_LOCK: c_int = 31;
pub const ARIZONA_IRQ_FLL1_LOCK: c_int = 32;
pub const ARIZONA_IRQ_CLKGEN_ERR: c_int = 33;
pub const ARIZONA_IRQ_CLKGEN_ERR_ASYNC: c_int = 34;
pub const ARIZONA_IRQ_ASRC_CFG_ERR: c_int = 35;
pub const ARIZONA_IRQ_AIF3_ERR: c_int = 36;
pub const ARIZONA_IRQ_AIF2_ERR: c_int = 37;
pub const ARIZONA_IRQ_AIF1_ERR: c_int = 38;
pub const ARIZONA_IRQ_CTRLIF_ERR: c_int = 39;
pub const ARIZONA_IRQ_MIXER_DROPPED_SAMPLES: c_int = 40;
pub const ARIZONA_IRQ_ASYNC_CLK_ENA_LOW: c_int = 41;
pub const ARIZONA_IRQ_SYSCLK_ENA_LOW: c_int = 42;
pub const ARIZONA_IRQ_ISRC1_CFG_ERR: c_int = 43;
pub const ARIZONA_IRQ_ISRC2_CFG_ERR: c_int = 44;
pub const ARIZONA_IRQ_BOOT_DONE: c_int = 45;
pub const ARIZONA_IRQ_DCS_DAC_DONE: c_int = 46;
pub const ARIZONA_IRQ_DCS_HP_DONE: c_int = 47;
pub const ARIZONA_IRQ_FLL2_CLOCK_OK: c_int = 48;
pub const ARIZONA_IRQ_FLL1_CLOCK_OK: c_int = 49;
pub const ARIZONA_IRQ_MICD_CLAMP_RISE: c_int = 50;
pub const ARIZONA_IRQ_MICD_CLAMP_FALL: c_int = 51;
pub const ARIZONA_IRQ_HP3R_DONE: c_int = 52;
pub const ARIZONA_IRQ_HP3L_DONE: c_int = 53;
pub const ARIZONA_IRQ_HP2R_DONE: c_int = 54;
pub const ARIZONA_IRQ_HP2L_DONE: c_int = 55;
pub const ARIZONA_IRQ_HP1R_DONE: c_int = 56;
pub const ARIZONA_IRQ_HP1L_DONE: c_int = 57;
pub const ARIZONA_IRQ_ISRC3_CFG_ERR: c_int = 58;
pub const ARIZONA_IRQ_DSP_SHARED_WR_COLL: c_int = 59;
pub const ARIZONA_IRQ_SPK_SHUTDOWN: c_int = 60;
pub const ARIZONA_IRQ_SPK1R_SHORT: c_int = 61;
pub const ARIZONA_IRQ_SPK1L_SHORT: c_int = 62;
pub const ARIZONA_IRQ_HP3R_SC_NEG: c_int = 63;
pub const ARIZONA_IRQ_HP3R_SC_POS: c_int = 64;
pub const ARIZONA_IRQ_HP3L_SC_NEG: c_int = 65;
pub const ARIZONA_IRQ_HP3L_SC_POS: c_int = 66;
pub const ARIZONA_IRQ_HP2R_SC_NEG: c_int = 67;
pub const ARIZONA_IRQ_HP2R_SC_POS: c_int = 68;
pub const ARIZONA_IRQ_HP2L_SC_NEG: c_int = 69;
pub const ARIZONA_IRQ_HP2L_SC_POS: c_int = 70;
pub const ARIZONA_IRQ_HP1R_SC_NEG: c_int = 71;
pub const ARIZONA_IRQ_HP1R_SC_POS: c_int = 72;
pub const ARIZONA_IRQ_HP1L_SC_NEG: c_int = 73;
pub const ARIZONA_IRQ_HP1L_SC_POS: c_int = 74;
pub const ARIZONA_NUM_IRQ: c_int = 75;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arizona {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub type: arizona_type,
    pub rev: c_uint,
    pub num_core_supplies: c_int,
    pub core_supplies: [regulator_bulk_data; ARIZONA_MAX_CORE_SUPPLIES],
    pub dcvdd: *mut regulator,
    pub has_fully_powered_off: bool,
    pub pdata: arizona_pdata,
    pub external_dcvdd:1: c_uint,
    pub irq: c_int,
    pub virq: *mut irq_domain,
    pub aod_irq_chip: *mut regmap_irq_chip_data,
    pub irq_chip: *mut regmap_irq_chip_data,
    pub hpdet_clamp: bool,
    pub hp_ena: c_uint,
    pub clk_lock: mutex,
    pub clk32k_ref: c_int,
    pub mclk: [*mut clk; ARIZONA_NUM_MCLK],
    pub ctrlif_error: bool,
    pub dapm: *mut snd_soc_dapm_context,
    pub tdm_width: [c_int; ARIZONA_MAX_AIF],
    pub tdm_slots: [c_int; ARIZONA_MAX_AIF],
    pub dac_comp_coeff: u16,
    pub dac_comp_enabled: u8,
    pub dac_comp_lock: mutex,
    pub notifier: blocking_notifier_head,
}

extern "C" {
    pub fn blocking_notifier_call_chain(_arg: &arizona->notifier, _arg: event, _arg: data) -> return;
}
extern "C" {
    pub fn arizona_clk32k_enable(arizona: *mut arizona) -> c_int;
}
extern "C" {
    pub fn arizona_clk32k_disable(arizona: *mut arizona) -> c_int;
}
extern "C" {
    pub fn arizona_free_irq(arizona: *mut arizona, irq: c_int, data: *mut c_void);
}
extern "C" {
    pub fn arizona_set_irq_wake(arizona: *mut arizona, irq: c_int, on: c_int) -> c_int;
}

extern "C" {
    pub fn wm5102_patch(arizona: *mut arizona) -> c_int;
}

extern "C" {
    pub fn wm5110_patch(arizona: *mut arizona) -> c_int;
}
extern "C" {
    pub fn cs47l24_patch(arizona: *mut arizona) -> c_int;
}
extern "C" {
    pub fn wm8997_patch(arizona: *mut arizona) -> c_int;
}
extern "C" {
    pub fn wm8998_patch(arizona: *mut arizona) -> c_int;
}
