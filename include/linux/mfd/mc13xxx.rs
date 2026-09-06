//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/mc13xxx.h
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
// Copyright 2009-2010 Pengutronix
// Uwe Kleine-Koenig <u.kleine-koenig@pengutronix.de>
//

extern "C" {
    pub fn mc13xxx_lock(mc13xxx: *mut mc13xxx);
}
extern "C" {
    pub fn mc13xxx_unlock(mc13xxx: *mut mc13xxx);
}
extern "C" {
    pub fn mc13xxx_reg_read(mc13xxx: *mut mc13xxx, offset: c_uint, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn mc13xxx_reg_write(mc13xxx: *mut mc13xxx, offset: c_uint, val: u32) -> c_int;
}
extern "C" {
    pub fn mc13xxx_irq_free(mc13xxx: *mut mc13xxx, irq: c_int, dev: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mc13xxx_get_flags(mc13xxx: *mut mc13xxx) -> c_int;
}
extern "C" {
    pub fn mc13xxx_irq_request(_arg: mc13xxx, _arg: irq, _arg: handler, _arg: name, _arg: dev) -> return;
}
extern "C" {
    pub fn mc13xxx_irq_mask(mc13xxx: *mut mc13xxx, irq: c_int) -> c_int;
}
extern "C" {
    pub fn mc13xxx_irq_unmask(mc13xxx: *mut mc13xxx, irq: c_int) -> c_int;
}
pub const MC13783_AUDIO_RX0: c_int = 36;
pub const MC13783_AUDIO_RX1: c_int = 37;
pub const MC13783_AUDIO_TX: c_int = 38;
pub const MC13783_SSI_NETWORK: c_int = 39;
pub const MC13783_AUDIO_CODEC: c_int = 40;
pub const MC13783_AUDIO_DAC: c_int = 41;
pub const MC13XXX_IRQ_ADCDONE: c_int = 0;
pub const MC13XXX_IRQ_ADCBISDONE: c_int = 1;
pub const MC13XXX_IRQ_TS: c_int = 2;
pub const MC13XXX_IRQ_CHGDET: c_int = 6;
pub const MC13XXX_IRQ_CHGREV: c_int = 8;
pub const MC13XXX_IRQ_CHGSHORT: c_int = 9;
pub const MC13XXX_IRQ_CCCV: c_int = 10;
pub const MC13XXX_IRQ_CHGCURR: c_int = 11;
pub const MC13XXX_IRQ_BPON: c_int = 12;
pub const MC13XXX_IRQ_LOBATL: c_int = 13;
pub const MC13XXX_IRQ_LOBATH: c_int = 14;
pub const MC13XXX_IRQ_1HZ: c_int = 24;
pub const MC13XXX_IRQ_TODA: c_int = 25;
pub const MC13XXX_IRQ_SYSRST: c_int = 30;
pub const MC13XXX_IRQ_RTCRST: c_int = 31;
pub const MC13XXX_IRQ_PC: c_int = 32;
pub const MC13XXX_IRQ_WARM: c_int = 33;
pub const MC13XXX_IRQ_MEMHLD: c_int = 34;
pub const MC13XXX_IRQ_THWARNL: c_int = 36;
pub const MC13XXX_IRQ_THWARNH: c_int = 37;
pub const MC13XXX_IRQ_CLK: c_int = 38;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc13xxx_regulator_init_data {
    pub id: c_int,
    pub init_data: *mut regulator_init_data,
    pub node: *mut device_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc13xxx_regulator_platform_data {
    pub num_regulators: c_int,
    pub regulators: *mut mc13xxx_regulator_init_data,
}

// MC13783 LED IDs
// MC13892 LED IDs
// MC34708 LED IDs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc13xxx_led_platform_data {
    pub id: c_int,
    pub name: *const c_char,
    pub default_trigger: *const c_char,
}

pub const MAX_LED_CONTROL_REGS: c_int = 6;
// MC13783 LED Control 0

// MC13783 LED Control 1

// MC13783 LED Control 2

// MC13783 LED Control 3

// MC13783 LED Control 4

// MC13783 LED Control 5

// MC13892 LED Control 0

// MC13892 LED Control 1

// MC13892 LED Control 2

// MC13892 LED Control 3

// MC34708 LED Control 0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc13xxx_leds_platform_data {
    pub led: *mut mc13xxx_led_platform_data,
    pub num_leds: c_int,
    pub led_control: [u32; MAX_LED_CONTROL_REGS],
}

pub const MC13783_BUTTON_DBNC_0MS: c_int = 0;
pub const MC13783_BUTTON_DBNC_30MS: c_int = 1;
pub const MC13783_BUTTON_DBNC_150MS: c_int = 2;
pub const MC13783_BUTTON_DBNC_750MS: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc13xxx_buttons_platform_data {
    pub b1on_flags: c_int,
    pub b1on_key: c_ushort,
    pub b2on_flags: c_int,
    pub b2on_key: c_ushort,
    pub b3on_flags: c_int,
    pub b3on_key: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc13xxx_ts_platform_data {
// Delay between Touchscreen polarization and ADC Conversion.
// Given in clock ticks of a 32 kHz clock which gives a granularity of
// about 30.5ms
    pub ato: u8,
// Use the ATO delay only for the first conversion or for each one
    pub atox: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mc13783_ssi_port {
    MC13783_SSI1_PORT,
    MC13783_SSI2_PORT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc13xxx_codec_platform_data {
    pub adc_ssi_port: mc13783_ssi_port,
    pub dac_ssi_port: mc13783_ssi_port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc13xxx_platform_data {
    pub flags: c_uint,
    pub regulators: mc13xxx_regulator_platform_data,
    pub leds: *mut mc13xxx_leds_platform_data,
    pub buttons: *mut mc13xxx_buttons_platform_data,
    pub touch: mc13xxx_ts_platform_data,
    pub codec: *mut mc13xxx_codec_platform_data,
}

pub const MC13XXX_ADC_MODE_TS: c_int = 1;
pub const MC13XXX_ADC_MODE_SINGLE_CHAN: c_int = 2;
pub const MC13XXX_ADC_MODE_MULT_CHAN: c_int = 3;
pub const MC13XXX_ADC0: c_int = 43;

