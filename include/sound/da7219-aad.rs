//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/da7219-aad.h
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
// da7219-aad.h - DA7322 ASoC Codec AAD Driver Platform Data
//
// Copyright (c) 2015 Dialog Semiconductor Ltd.
//
// Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_aad_micbias_pulse_lvl {
    DA7219_AAD_MICBIAS_PULSE_LVL_OFF = 0,
    DA7219_AAD_MICBIAS_PULSE_LVL_2_8V = 6,
    DA7219_AAD_MICBIAS_PULSE_LVL_2_9V,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_aad_btn_cfg {
    DA7219_AAD_BTN_CFG_2MS = 1,
    DA7219_AAD_BTN_CFG_5MS,
    DA7219_AAD_BTN_CFG_10MS,
    DA7219_AAD_BTN_CFG_50MS,
    DA7219_AAD_BTN_CFG_100MS,
    DA7219_AAD_BTN_CFG_200MS,
    DA7219_AAD_BTN_CFG_500MS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_aad_mic_det_thr {
    DA7219_AAD_MIC_DET_THR_200_OHMS = 0,
    DA7219_AAD_MIC_DET_THR_500_OHMS,
    DA7219_AAD_MIC_DET_THR_750_OHMS,
    DA7219_AAD_MIC_DET_THR_1000_OHMS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_aad_jack_ins_deb {
    DA7219_AAD_JACK_INS_DEB_5MS = 0,
    DA7219_AAD_JACK_INS_DEB_10MS,
    DA7219_AAD_JACK_INS_DEB_20MS,
    DA7219_AAD_JACK_INS_DEB_50MS,
    DA7219_AAD_JACK_INS_DEB_100MS,
    DA7219_AAD_JACK_INS_DEB_200MS,
    DA7219_AAD_JACK_INS_DEB_500MS,
    DA7219_AAD_JACK_INS_DEB_1S,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_aad_jack_ins_det_pty {
    DA7219_AAD_JACK_INS_DET_PTY_LOW = 0,
    DA7219_AAD_JACK_INS_DET_PTY_HIGH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_aad_jack_det_rate {
    DA7219_AAD_JACK_DET_RATE_32_64MS = 0,
    DA7219_AAD_JACK_DET_RATE_64_128MS,
    DA7219_AAD_JACK_DET_RATE_128_256MS,
    DA7219_AAD_JACK_DET_RATE_256_512MS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_aad_jack_rem_deb {
    DA7219_AAD_JACK_REM_DEB_1MS = 0,
    DA7219_AAD_JACK_REM_DEB_5MS,
    DA7219_AAD_JACK_REM_DEB_10MS,
    DA7219_AAD_JACK_REM_DEB_20MS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_aad_btn_avg {
    DA7219_AAD_BTN_AVG_1 = 0,
    DA7219_AAD_BTN_AVG_2,
    DA7219_AAD_BTN_AVG_4,
    DA7219_AAD_BTN_AVG_8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_aad_adc_1bit_rpt {
    DA7219_AAD_ADC_1BIT_RPT_1 = 0,
    DA7219_AAD_ADC_1BIT_RPT_2,
    DA7219_AAD_ADC_1BIT_RPT_4,
    DA7219_AAD_ADC_1BIT_RPT_8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da7219_aad_pdata {
    pub irq: c_int,
    pub micbias_pulse_lvl: da7219_aad_micbias_pulse_lvl,
    pub micbias_pulse_time: u32,
    pub btn_cfg: da7219_aad_btn_cfg,
    pub mic_det_thr: da7219_aad_mic_det_thr,
    pub jack_ins_deb: da7219_aad_jack_ins_deb,
    pub jack_ins_det_pty: da7219_aad_jack_ins_det_pty,
    pub jack_det_rate: da7219_aad_jack_det_rate,
    pub jack_rem_deb: da7219_aad_jack_rem_deb,
    pub a_d_btn_thr: u8,
    pub d_b_btn_thr: u8,
    pub b_c_btn_thr: u8,
    pub c_mic_btn_thr: u8,
    pub btn_avg: da7219_aad_btn_avg,
    pub adc_1bit_rpt: da7219_aad_adc_1bit_rpt,
}
