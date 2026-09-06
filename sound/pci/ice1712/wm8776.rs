//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ice1712/wm8776.h
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
// ALSA driver for ICEnsemble VT17xx
//
// Lowlevel functions for WM8776 codec
//
// Copyright (c) 2012 Ondrej Zary <linux@rainbow-software.org>
//
pub const WM8776_REG_HPLVOL: c_uint = 0x00;
pub const WM8776_REG_HPRVOL: c_uint = 0x01;
pub const WM8776_REG_HPMASTER: c_uint = 0x02;
pub const WM8776_HPVOL_MASK: c_uint = 0x17f		/* incl. update bit */;

pub const WM8776_REG_DACLVOL: c_uint = 0x03;
pub const WM8776_REG_DACRVOL: c_uint = 0x04;
pub const WM8776_REG_DACMASTER: c_uint = 0x05;
pub const WM8776_DACVOL_MASK: c_uint = 0x1ff		/* incl. update bit */;
pub const WM8776_REG_PHASESWAP: c_uint = 0x06;

pub const WM8776_REG_DACCTRL1: c_uint = 0x07;

pub const WM8776_DAC_PL_MASK: c_uint = 0xf0;

pub const WM8776_REG_DACMUTE: c_uint = 0x08;

pub const WM8776_REG_DACCTRL2: c_uint = 0x09;

pub const WM8776_REG_DACIFCTRL: c_uint = 0x0a;

pub const WM8776_REG_ADCIFCTRL: c_uint = 0x0b;

pub const WM8776_REG_MSTRCTRL: c_uint = 0x0c;

pub const WM8776_REG_PWRDOWN: c_uint = 0x0d;

pub const WM8776_REG_ADCLVOL: c_uint = 0x0e;
pub const WM8776_REG_ADCRVOL: c_uint = 0x0f;
pub const WM8776_ADC_GAIN_MASK: c_uint = 0xff;

pub const WM8776_REG_ALCCTRL1: c_uint = 0x10;
pub const WM8776_ALC1_LCT_MASK: c_uint = 0x0f	/* 0=-16dB, 1=-15dB..15=-1dB */;
pub const WM8776_ALC1_MAXGAIN_MASK: c_uint = 0x70	/* 0,1=0dB, 2=+4dB...7=+24dB */;
pub const WM8776_ALC1_LCSEL_MASK: c_uint = 0x180;

pub const WM8776_REG_ALCCTRL2: c_uint = 0x11;
pub const WM8776_ALC2_HOLD_MASK: c_uint = 0x0f	/*0=0ms, 1=2.67ms, 2=5.33ms.. */;

pub const WM8776_REG_ALCCTRL3: c_uint = 0x12;
pub const WM8776_ALC3_ATK_MASK: c_uint = 0x0f;
pub const WM8776_ALC3_DCY_MASK: c_uint = 0xf0;

pub const WM8776_REG_NOISEGATE: c_uint = 0x13;

pub const WM8776_NGAT_THR_MASK: c_uint = 0x1c	/*0=-78dB, 1=-72dB...7=-36dB */;
pub const WM8776_REG_LIMITER: c_uint = 0x14;
pub const WM8776_LIM_MAXATTEN_MASK: c_uint = 0x0f;
pub const WM8776_LIM_TRANWIN_MASK: c_uint = 0x70	/*0=0us, 1=62.5us, 2=125us.. */;
pub const WM8776_REG_ADCMUX: c_uint = 0x15;

pub const WM8776_REG_OUTMUX: c_uint = 0x16;

pub const WM8776_REG_RESET: c_uint = 0x17;
pub const WM8776_REG_COUNT: c_uint = 0x17	/* don't cache the RESET register */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_wm8776_ops {
    pub data): *mut *mut *mut void (write)(struct snd_wm8776 wm, u8 addr, u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_wm8776_ctl_id {
    WM8776_CTL_DAC_VOL,
    WM8776_CTL_DAC_SW,
    WM8776_CTL_DAC_ZC_SW,
    WM8776_CTL_HP_VOL,
    WM8776_CTL_HP_SW,
    WM8776_CTL_HP_ZC_SW,
    WM8776_CTL_AUX_SW,
    WM8776_CTL_BYPASS_SW,
    WM8776_CTL_DAC_IZD_SW,
    WM8776_CTL_PHASE_SW,
    WM8776_CTL_DEEMPH_SW,
    WM8776_CTL_ADC_VOL,
    WM8776_CTL_ADC_SW,
    WM8776_CTL_INPUT1_SW,
    WM8776_CTL_INPUT2_SW,
    WM8776_CTL_INPUT3_SW,
    WM8776_CTL_INPUT4_SW,
    WM8776_CTL_INPUT5_SW,
    WM8776_CTL_AGC_SEL,
    WM8776_CTL_LIM_THR,
    WM8776_CTL_LIM_ATK,
    WM8776_CTL_LIM_DCY,
    WM8776_CTL_LIM_TRANWIN,
    WM8776_CTL_LIM_MAXATTN,
    WM8776_CTL_ALC_TGT,
    WM8776_CTL_ALC_ATK,
    WM8776_CTL_ALC_DCY,
    WM8776_CTL_ALC_MAXGAIN,
    WM8776_CTL_ALC_MAXATTN,
    WM8776_CTL_ALC_HLD,
    WM8776_CTL_NGT_SW,
    WM8776_CTL_NGT_THR,

    WM8776_CTL_COUNT,
}

pub const WM8776_ENUM_MAX: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_wm8776_ctl {
    pub name: *const c_char,
    pub type: snd_ctl_elem_type_t,
    pub enum_names: [*const *const c_char; WM8776_ENUM_MAX],
    pub tlv: *const c_uint,
    pub flags: u16 reg1, reg2, mask1, mask2, min, max,,
    pub ch2): *mut *mut *mut void (set)(struct snd_wm8776 wm, u16 ch1, u16,
    pub ch2): *mut *mut *mut *mut void (get)(struct snd_wm8776 wm, u16 ch1, u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_wm8776_agc_mode {
    WM8776_AGC_OFF,
    WM8776_AGC_LIM,
    WM8776_AGC_ALC_R,
    WM8776_AGC_ALC_L,
    WM8776_AGC_ALC_STEREO
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_wm8776 {
    pub card: *mut snd_card,
    pub ctl: [snd_wm8776_ctl; WM8776_CTL_COUNT],
    pub agc_mode: snd_wm8776_agc_mode,
    pub ops: snd_wm8776_ops,
    pub /: *mut *mut u16 regs[WM8776_REG_COUNT]; / 9-bit registers,
}

extern "C" {
    pub fn snd_wm8776_init(wm: *mut snd_wm8776);
}
extern "C" {
    pub fn snd_wm8776_resume(wm: *mut snd_wm8776);
}
extern "C" {
    pub fn snd_wm8776_set_power(wm: *mut snd_wm8776, power: u16);
}
extern "C" {
    pub fn snd_wm8776_volume_restore(wm: *mut snd_wm8776);
}
extern "C" {
    pub fn snd_wm8776_build_controls(wm: *mut snd_wm8776) -> c_int;
}
