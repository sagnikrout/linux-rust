//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/madera/core.h
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
// MFD internals for Cirrus Logic Madera codecs
//
// Copyright (C) 2015-2018 Cirrus Logic
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum madera_type {
// 0 is reserved for indicating failure to identify
    CS47L35 = 1,
    CS47L85 = 2,
    CS47L90 = 3,
    CS47L91 = 4,
    CS47L92 = 5,
    CS47L93 = 6,
    WM1840 = 7,
    CS47L15 = 8,
    CS42L92 = 9,
}

pub const MADERA_MAX_CORE_SUPPLIES: c_int = 2;
pub const MADERA_MAX_GPIOS: c_int = 40;
pub const CS47L15_NUM_GPIOS: c_int = 15;
pub const CS47L35_NUM_GPIOS: c_int = 16;
pub const CS47L85_NUM_GPIOS: c_int = 40;
pub const CS47L90_NUM_GPIOS: c_int = 38;
pub const CS47L92_NUM_GPIOS: c_int = 16;
pub const MADERA_MAX_MICBIAS: c_int = 4;
pub const MADERA_MAX_HP_OUTPUT: c_int = 3;
// Notifier events
pub const MADERA_NOTIFY_VOICE_TRIGGER: c_uint = 0x1;
pub const MADERA_NOTIFY_HPDET: c_uint = 0x2;
pub const MADERA_NOTIFY_MICDET: c_uint = 0x4;
// GPIO Function Definitions
pub const MADERA_GP_FN_ALTERNATE: c_uint = 0x00;
pub const MADERA_GP_FN_GPIO: c_uint = 0x01;
pub const MADERA_GP_FN_DSP_GPIO: c_uint = 0x02;
pub const MADERA_GP_FN_IRQ1: c_uint = 0x03;
pub const MADERA_GP_FN_IRQ2: c_uint = 0x04;
pub const MADERA_GP_FN_FLL1_CLOCK: c_uint = 0x10;
pub const MADERA_GP_FN_FLL2_CLOCK: c_uint = 0x11;
pub const MADERA_GP_FN_FLL3_CLOCK: c_uint = 0x12;
pub const MADERA_GP_FN_FLLAO_CLOCK: c_uint = 0x13;
pub const MADERA_GP_FN_FLL1_LOCK: c_uint = 0x18;
pub const MADERA_GP_FN_FLL2_LOCK: c_uint = 0x19;
pub const MADERA_GP_FN_FLL3_LOCK: c_uint = 0x1A;
pub const MADERA_GP_FN_FLLAO_LOCK: c_uint = 0x1B;
pub const MADERA_GP_FN_OPCLK_OUT: c_uint = 0x40;
pub const MADERA_GP_FN_OPCLK_ASYNC_OUT: c_uint = 0x41;
pub const MADERA_GP_FN_PWM1: c_uint = 0x48;
pub const MADERA_GP_FN_PWM2: c_uint = 0x49;
pub const MADERA_GP_FN_SPDIF_OUT: c_uint = 0x4C;
pub const MADERA_GP_FN_HEADPHONE_DET: c_uint = 0x50;
pub const MADERA_GP_FN_MIC_DET: c_uint = 0x58;
pub const MADERA_GP_FN_DRC1_SIGNAL_DETECT: c_uint = 0x80;
pub const MADERA_GP_FN_DRC2_SIGNAL_DETECT: c_uint = 0x81;
pub const MADERA_GP_FN_ASRC1_IN1_LOCK: c_uint = 0x88;
pub const MADERA_GP_FN_ASRC1_IN2_LOCK: c_uint = 0x89;
pub const MADERA_GP_FN_ASRC2_IN1_LOCK: c_uint = 0x8A;
pub const MADERA_GP_FN_ASRC2_IN2_LOCK: c_uint = 0x8B;
pub const MADERA_GP_FN_DSP_IRQ1: c_uint = 0xA0;
pub const MADERA_GP_FN_DSP_IRQ2: c_uint = 0xA1;
pub const MADERA_GP_FN_DSP_IRQ3: c_uint = 0xA2;
pub const MADERA_GP_FN_DSP_IRQ4: c_uint = 0xA3;
pub const MADERA_GP_FN_DSP_IRQ5: c_uint = 0xA4;
pub const MADERA_GP_FN_DSP_IRQ6: c_uint = 0xA5;
pub const MADERA_GP_FN_DSP_IRQ7: c_uint = 0xA6;
pub const MADERA_GP_FN_DSP_IRQ8: c_uint = 0xA7;
pub const MADERA_GP_FN_DSP_IRQ9: c_uint = 0xA8;
pub const MADERA_GP_FN_DSP_IRQ10: c_uint = 0xA9;
pub const MADERA_GP_FN_DSP_IRQ11: c_uint = 0xAA;
pub const MADERA_GP_FN_DSP_IRQ12: c_uint = 0xAB;
pub const MADERA_GP_FN_DSP_IRQ13: c_uint = 0xAC;
pub const MADERA_GP_FN_DSP_IRQ14: c_uint = 0xAD;
pub const MADERA_GP_FN_DSP_IRQ15: c_uint = 0xAE;
pub const MADERA_GP_FN_DSP_IRQ16: c_uint = 0xAF;
pub const MADERA_GP_FN_HPOUT1L_SC: c_uint = 0xB0;
pub const MADERA_GP_FN_HPOUT1R_SC: c_uint = 0xB1;
pub const MADERA_GP_FN_HPOUT2L_SC: c_uint = 0xB2;
pub const MADERA_GP_FN_HPOUT2R_SC: c_uint = 0xB3;
pub const MADERA_GP_FN_HPOUT3L_SC: c_uint = 0xB4;
pub const MADERA_GP_FN_HPOUT4R_SC: c_uint = 0xB5;
pub const MADERA_GP_FN_SPKOUTL_SC: c_uint = 0xB6;
pub const MADERA_GP_FN_SPKOUTR_SC: c_uint = 0xB7;
pub const MADERA_GP_FN_HPOUT1L_ENA: c_uint = 0xC0;
pub const MADERA_GP_FN_HPOUT1R_ENA: c_uint = 0xC1;
pub const MADERA_GP_FN_HPOUT2L_ENA: c_uint = 0xC2;
pub const MADERA_GP_FN_HPOUT2R_ENA: c_uint = 0xC3;
pub const MADERA_GP_FN_HPOUT3L_ENA: c_uint = 0xC4;
pub const MADERA_GP_FN_HPOUT4R_ENA: c_uint = 0xC5;
pub const MADERA_GP_FN_SPKOUTL_ENA: c_uint = 0xC6;
pub const MADERA_GP_FN_SPKOUTR_ENA: c_uint = 0xC7;
pub const MADERA_GP_FN_HPOUT1L_DIS: c_uint = 0xD0;
pub const MADERA_GP_FN_HPOUT1R_DIS: c_uint = 0xD1;
pub const MADERA_GP_FN_HPOUT2L_DIS: c_uint = 0xD2;
pub const MADERA_GP_FN_HPOUT2R_DIS: c_uint = 0xD3;
pub const MADERA_GP_FN_HPOUT3L_DIS: c_uint = 0xD4;
pub const MADERA_GP_FN_HPOUT4R_DIS: c_uint = 0xD5;
pub const MADERA_GP_FN_SPKOUTL_DIS: c_uint = 0xD6;
pub const MADERA_GP_FN_SPKOUTR_DIS: c_uint = 0xD7;
pub const MADERA_GP_FN_SPK_SHUTDOWN: c_uint = 0xE0;
pub const MADERA_GP_FN_SPK_OVH_SHUTDOWN: c_uint = 0xE1;
pub const MADERA_GP_FN_SPK_OVH_WARN: c_uint = 0xE2;
pub const MADERA_GP_FN_TIMER1_STATUS: c_uint = 0x140;
pub const MADERA_GP_FN_TIMER2_STATUS: c_uint = 0x141;
pub const MADERA_GP_FN_TIMER3_STATUS: c_uint = 0x142;
pub const MADERA_GP_FN_TIMER4_STATUS: c_uint = 0x143;
pub const MADERA_GP_FN_TIMER5_STATUS: c_uint = 0x144;
pub const MADERA_GP_FN_TIMER6_STATUS: c_uint = 0x145;
pub const MADERA_GP_FN_TIMER7_STATUS: c_uint = 0x146;
pub const MADERA_GP_FN_TIMER8_STATUS: c_uint = 0x147;
pub const MADERA_GP_FN_EVENTLOG1_FIFO_STS: c_uint = 0x150;
pub const MADERA_GP_FN_EVENTLOG2_FIFO_STS: c_uint = 0x151;
pub const MADERA_GP_FN_EVENTLOG3_FIFO_STS: c_uint = 0x152;
pub const MADERA_GP_FN_EVENTLOG4_FIFO_STS: c_uint = 0x153;
pub const MADERA_GP_FN_EVENTLOG5_FIFO_STS: c_uint = 0x154;
pub const MADERA_GP_FN_EVENTLOG6_FIFO_STS: c_uint = 0x155;
pub const MADERA_GP_FN_EVENTLOG7_FIFO_STS: c_uint = 0x156;
pub const MADERA_GP_FN_EVENTLOG8_FIFO_STS: c_uint = 0x157;
//
// struct madera - internal data shared by the set of Madera drivers
//
// This should not be used by anything except child drivers of the Madera MFD
//
// @regmap:		pointer to the regmap instance for 16-bit registers
// @regmap_32bit:	pointer to the regmap instance for 32-bit registers
// @dev:		pointer to the MFD device
// @type:		type of codec
// @rev:		silicon revision
// @type_name:		display name of this codec
// @num_core_supplies:	number of core supply regulators
// @core_supplies:	list of core supplies that are always required
// @dcvdd:		pointer to DCVDD regulator
// @internal_dcvdd:	true if DCVDD is supplied from the internal LDO1
// @pdata:		our pdata
// @irq_dev:		the irqchip child driver device
// @irq_data:		pointer to irqchip data for the child irqchip driver
// @irq:		host irq number from SPI or I2C configuration
// @mclk:		Structure holding clock supplies
// @out_clamp:		indicates output clamp state for each analogue output
// @out_shorted:	indicates short circuit state for each analogue output
// @hp_ena:		bitflags of enable state for the headphone outputs
// @num_micbias:	number of MICBIAS outputs
// @num_childbias:	number of child biases for each MICBIAS
// @dapm:		pointer to codec driver DAPM context
// @notifier:		notifier for signalling events to ASoC machine driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct madera {
    pub regmap: *mut regmap,
    pub regmap_32bit: *mut regmap,
    pub dev: *mut device,
    pub type: madera_type,
    pub rev: c_uint,
    pub type_name: *const c_char,
    pub num_core_supplies: c_int,
    pub core_supplies: [regulator_bulk_data; MADERA_MAX_CORE_SUPPLIES],
    pub dcvdd: *mut regulator,
    pub internal_dcvdd: bool,
    pub reset_errata: bool,
    pub pdata: madera_pdata,
    pub irq_dev: *mut device,
    pub irq_data: *mut regmap_irq_chip_data,
    pub irq: c_int,
    pub mclk: [clk_bulk_data; MADERA_NUM_MCLK],
    pub num_micbias: c_uint,
    pub num_childbias: [c_uint; MADERA_MAX_MICBIAS],
    pub dapm: *mut snd_soc_dapm_context,
    pub dapm_ptr_lock: mutex,
    pub hp_ena: c_uint,
    pub out_clamp: [bool; MADERA_MAX_HP_OUTPUT],
    pub out_shorted: [bool; MADERA_MAX_HP_OUTPUT],
    pub notifier: blocking_notifier_head,
}
