//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ice1712/wm8766.h
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
// Lowlevel functions for WM8766 codec
//
// Copyright (c) 2012 Ondrej Zary <linux@rainbow-software.org>
//
pub const WM8766_REG_DACL1: c_uint = 0x00;
pub const WM8766_REG_DACR1: c_uint = 0x01;
pub const WM8766_VOL_MASK: c_uint = 0x1ff		/* incl. update bit */;

pub const WM8766_REG_DACCTRL1: c_uint = 0x02;

pub const WM8766_DAC_PL_MASK: c_uint = 0x1e0;

pub const WM8766_REG_IFCTRL: c_uint = 0x03;

pub const WM8766_IF_MASK: c_uint = 0x3f;

pub const WM8766_REG_DACL2: c_uint = 0x04;
pub const WM8766_REG_DACR2: c_uint = 0x05;
pub const WM8766_REG_DACL3: c_uint = 0x06;
pub const WM8766_REG_DACR3: c_uint = 0x07;
pub const WM8766_REG_MASTDA: c_uint = 0x08;
pub const WM8766_REG_DACCTRL2: c_uint = 0x09;

pub const WM8766_REG_DACCTRL3: c_uint = 0x0a;

pub const WM8766_DAC3_POWER_MASK: c_uint = 0x1e;

pub const WM8766_DAC3_MSTR_MASK: c_uint = 0x1e0;
pub const WM8766_REG_MUTE1: c_uint = 0x0c;

pub const WM8766_REG_MUTE2: c_uint = 0x0f;

pub const WM8766_REG_RESET: c_uint = 0x1f;
pub const WM8766_REG_COUNT: c_uint = 0x10	/* don't cache the RESET register */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_wm8766_ops {
    pub data): *mut *mut *mut void (write)(struct snd_wm8766 wm, u16 addr, u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_wm8766_ctl_id {
    WM8766_CTL_CH1_VOL,
    WM8766_CTL_CH2_VOL,
    WM8766_CTL_CH3_VOL,
    WM8766_CTL_CH1_SW,
    WM8766_CTL_CH2_SW,
    WM8766_CTL_CH3_SW,
    WM8766_CTL_PHASE1_SW,
    WM8766_CTL_PHASE2_SW,
    WM8766_CTL_PHASE3_SW,
    WM8766_CTL_DEEMPH1_SW,
    WM8766_CTL_DEEMPH2_SW,
    WM8766_CTL_DEEMPH3_SW,
    WM8766_CTL_IZD_SW,
    WM8766_CTL_ZC_SW,

    WM8766_CTL_COUNT,
}

pub const WM8766_ENUM_MAX: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_wm8766_ctl {
    pub kctl: *mut snd_kcontrol,
    pub name: *const c_char,
    pub type: snd_ctl_elem_type_t,
    pub enum_names: [*const *const c_char; WM8766_ENUM_MAX],
    pub tlv: *const c_uint,
    pub flags: u16 reg1, reg2, mask1, mask2, min, max,,
    pub ch2): *mut *mut *mut void (set)(struct snd_wm8766 wm, u16 ch1, u16,
    pub ch2): *mut *mut *mut *mut void (get)(struct snd_wm8766 wm, u16 ch1, u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_wm8766_agc_mode {

    struct snd_wm8766 {
    struct snd_card *card;
    struct snd_wm8766_ctl ctl[WM8766_CTL_COUNT];
    enum snd_wm8766_agc_mode agc_mode;
    struct snd_wm8766_ops ops;
    u16 regs[WM8766_REG_COUNT];	/* 9-bit registers */
}

extern "C" {
    pub fn snd_wm8766_init(wm: *mut snd_wm8766);
}
extern "C" {
    pub fn snd_wm8766_resume(wm: *mut snd_wm8766);
}
extern "C" {
    pub fn snd_wm8766_set_if(wm: *mut snd_wm8766, dac: u16);
}
extern "C" {
    pub fn snd_wm8766_volume_restore(wm: *mut snd_wm8766);
}
extern "C" {
    pub fn snd_wm8766_build_controls(wm: *mut snd_wm8766) -> c_int;
}
