//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/da7219-aad.h
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
// da7219-aad.h - DA7322 ASoC AAD Driver
//
// Copyright (c) 2015 Dialog Semiconductor Ltd.
//
// Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
//

//
// Registers
//
pub const DA7219_ACCDET_STATUS_A: c_uint = 0xC0;
pub const DA7219_ACCDET_STATUS_B: c_uint = 0xC1;
pub const DA7219_ACCDET_IRQ_EVENT_A: c_uint = 0xC2;
pub const DA7219_ACCDET_IRQ_EVENT_B: c_uint = 0xC3;
pub const DA7219_ACCDET_IRQ_MASK_A: c_uint = 0xC4;
pub const DA7219_ACCDET_IRQ_MASK_B: c_uint = 0xC5;
pub const DA7219_ACCDET_CONFIG_1: c_uint = 0xC6;
pub const DA7219_ACCDET_CONFIG_2: c_uint = 0xC7;
pub const DA7219_ACCDET_CONFIG_3: c_uint = 0xC8;
pub const DA7219_ACCDET_CONFIG_4: c_uint = 0xC9;
pub const DA7219_ACCDET_CONFIG_5: c_uint = 0xCA;
pub const DA7219_ACCDET_CONFIG_6: c_uint = 0xCB;
pub const DA7219_ACCDET_CONFIG_7: c_uint = 0xCC;
pub const DA7219_ACCDET_CONFIG_8: c_uint = 0xCD;
//
// Bit Fields
//
// DA7219_ACCDET_STATUS_A = 0xC0
pub const DA7219_JACK_INSERTION_STS_SHIFT: c_int = 0;

pub const DA7219_JACK_TYPE_STS_SHIFT: c_int = 1;

pub const DA7219_JACK_PIN_ORDER_STS_SHIFT: c_int = 2;

pub const DA7219_MICBIAS_UP_STS_SHIFT: c_int = 3;

// DA7219_ACCDET_STATUS_B = 0xC1
pub const DA7219_BUTTON_TYPE_STS_SHIFT: c_int = 0;

// DA7219_ACCDET_IRQ_EVENT_A = 0xC2
pub const DA7219_E_JACK_INSERTED_SHIFT: c_int = 0;

pub const DA7219_E_JACK_REMOVED_SHIFT: c_int = 1;

pub const DA7219_E_JACK_DETECT_COMPLETE_SHIFT: c_int = 2;

// DA7219_ACCDET_IRQ_EVENT_B = 0xC3
pub const DA7219_E_BUTTON_A_PRESSED_SHIFT: c_int = 0;

pub const DA7219_E_BUTTON_B_PRESSED_SHIFT: c_int = 1;

pub const DA7219_E_BUTTON_C_PRESSED_SHIFT: c_int = 2;

pub const DA7219_E_BUTTON_D_PRESSED_SHIFT: c_int = 3;

pub const DA7219_E_BUTTON_D_RELEASED_SHIFT: c_int = 4;

pub const DA7219_E_BUTTON_C_RELEASED_SHIFT: c_int = 5;

pub const DA7219_E_BUTTON_B_RELEASED_SHIFT: c_int = 6;

pub const DA7219_E_BUTTON_A_RELEASED_SHIFT: c_int = 7;

// DA7219_ACCDET_IRQ_MASK_A = 0xC4
pub const DA7219_M_JACK_INSERTED_SHIFT: c_int = 0;

pub const DA7219_M_JACK_REMOVED_SHIFT: c_int = 1;

pub const DA7219_M_JACK_DETECT_COMPLETE_SHIFT: c_int = 2;

// DA7219_ACCDET_IRQ_MASK_B = 0xC5
pub const DA7219_M_BUTTON_A_PRESSED_SHIFT: c_int = 0;

pub const DA7219_M_BUTTON_B_PRESSED_SHIFT: c_int = 1;

pub const DA7219_M_BUTTON_C_PRESSED_SHIFT: c_int = 2;

pub const DA7219_M_BUTTON_D_PRESSED_SHIFT: c_int = 3;

pub const DA7219_M_BUTTON_D_RELEASED_SHIFT: c_int = 4;

pub const DA7219_M_BUTTON_C_RELEASED_SHIFT: c_int = 5;

pub const DA7219_M_BUTTON_B_RELEASED_SHIFT: c_int = 6;

pub const DA7219_M_BUTTON_A_RELEASED_SHIFT: c_int = 7;

// DA7219_ACCDET_CONFIG_1 = 0xC6
pub const DA7219_ACCDET_EN_SHIFT: c_int = 0;

pub const DA7219_BUTTON_CONFIG_SHIFT: c_int = 1;

pub const DA7219_MIC_DET_THRESH_SHIFT: c_int = 4;

pub const DA7219_JACK_TYPE_DET_EN_SHIFT: c_int = 6;

pub const DA7219_PIN_ORDER_DET_EN_SHIFT: c_int = 7;

// DA7219_ACCDET_CONFIG_2 = 0xC7
pub const DA7219_ACCDET_PAUSE_SHIFT: c_int = 0;

pub const DA7219_JACKDET_DEBOUNCE_SHIFT: c_int = 1;

pub const DA7219_JACK_DETECT_RATE_SHIFT: c_int = 4;

pub const DA7219_JACKDET_REM_DEB_SHIFT: c_int = 6;

// DA7219_ACCDET_CONFIG_3 = 0xC8
pub const DA7219_A_D_BUTTON_THRESH_SHIFT: c_int = 0;

// DA7219_ACCDET_CONFIG_4 = 0xC9
pub const DA7219_D_B_BUTTON_THRESH_SHIFT: c_int = 0;

// DA7219_ACCDET_CONFIG_5 = 0xCA
pub const DA7219_B_C_BUTTON_THRESH_SHIFT: c_int = 0;

// DA7219_ACCDET_CONFIG_6 = 0xCB
pub const DA7219_C_MIC_BUTTON_THRESH_SHIFT: c_int = 0;

// DA7219_ACCDET_CONFIG_7 = 0xCC
pub const DA7219_BUTTON_AVERAGE_SHIFT: c_int = 0;

pub const DA7219_ADC_1_BIT_REPEAT_SHIFT: c_int = 2;

pub const DA7219_PIN_ORDER_FORCE_SHIFT: c_int = 4;

pub const DA7219_JACK_TYPE_FORCE_SHIFT: c_int = 5;

// DA7219_ACCDET_CONFIG_8 = 0xCD
pub const DA7219_HPTEST_EN_SHIFT: c_int = 0;

pub const DA7219_HPTEST_RES_SEL_SHIFT: c_int = 1;

pub const DA7219_HPTEST_COMP_SHIFT: c_int = 4;

pub const DA7219_AAD_MAX_BUTTONS: c_int = 4;

pub const DA7219_AAD_MICBIAS_CHK_DELAY: c_int = 10;
pub const DA7219_AAD_MICBIAS_CHK_RETRIES: c_int = 5;
pub const DA7219_AAD_HPTEST_RAMP_FREQ: c_uint = 0x28;
pub const DA7219_AAD_HPTEST_RAMP_FREQ_INT_OSC: c_uint = 0x4D;
pub const DA7219_AAD_HPTEST_PERIOD: c_int = 65;
pub const DA7219_AAD_HPTEST_INT_OSC_PATH_DELAY: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_aad_event_regs {
    DA7219_AAD_IRQ_REG_A = 0,
    DA7219_AAD_IRQ_REG_B,
    DA7219_AAD_IRQ_REG_MAX,
}

// Private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da7219_aad_priv {
    pub component: *mut snd_soc_component,
    pub irq: c_int,
    pub gnd_switch_delay: c_int,
    pub micbias_pulse_lvl: u8,
    pub micbias_pulse_time: u32,
    pub btn_cfg: u8,
    pub btn_det_work: work_struct,
    pub hptest_work: work_struct,
    pub jack_det_work: delayed_work,
    pub aad_wq: *mut workqueue_struct,
    pub jack: *mut snd_soc_jack,
    pub micbias_resume_enable: bool,
    pub jack_inserted: bool,
}

// AAD control
extern "C" {
    pub fn da7219_aad_jack_det(component: *mut snd_soc_component, jack: *mut snd_soc_jack);
}
// Suspend/Resume

extern "C" {
    pub fn da7219_aad_suspend(component: *mut snd_soc_component);
}
extern "C" {
    pub fn da7219_aad_resume(component: *mut snd_soc_component);
}

// Init/Exit
extern "C" {
    pub fn da7219_aad_init(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn da7219_aad_exit(component: *mut snd_soc_component);
}
// I2C Probe
extern "C" {
    pub fn da7219_aad_probe(i2c: *mut i2c_client) -> c_int;
}
