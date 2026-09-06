//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/soc-jack.h
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


// SPDX-License-Identifier: GPL-2.0
//
// soc-jack.h
//
// Copyright (C) 2019 Renesas Electronics Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//
// struct snd_soc_jack_pin - Describes a pin to update based on jack detection
//
// @pin:    name of the pin to update
// @mask:   bits to check for in reported jack status
// @invert: if non-zero then pin is enabled when status is not reported
// @list:   internal list entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_jack_pin {
    pub list: list_head,
    pub pin: *const c_char,
    pub mask: c_int,
    pub invert: bool,
}

//
// struct snd_soc_jack_zone - Describes voltage zones of jack detection
//
// @min_mv: start voltage in mv
// @max_mv: end voltage in mv
// @jack_type: type of jack that is expected for this voltage
// @debounce_time: debounce_time for jack, codec driver should wait for this
// duration before reading the adc for voltages
// @list:   internal list entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_jack_zone {
    pub min_mv: c_uint,
    pub max_mv: c_uint,
    pub jack_type: c_uint,
    pub debounce_time: c_uint,
    pub list: list_head,
}

//
// struct snd_soc_jack_gpio - Describes a gpio pin for jack detection
//
// @idx:          gpio descriptor index within the function of the GPIO
// consumer device
// @gpiod_dev:    GPIO consumer device
// @name:         gpio name. Also as connection ID for the GPIO consumer
// device function name lookup
// @report:       value to report when jack detected
// @invert:       report presence in low state
// @debounce_time: debounce time in ms
// @wake:	  enable as wake source
// @jack_status_check: callback function which overrides the detection
// to provide more complex checks (eg, reading an
// ADC).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_jack_gpio {
    pub idx: c_uint,
    pub gpiod_dev: *mut device,
    pub name: *const c_char,
    pub report: c_int,
    pub invert: c_int,
    pub debounce_time: c_int,
    pub wake: bool,
// private:
    pub jack: *mut snd_soc_jack,
    pub work: delayed_work,
    pub pm_notifier: notifier_block,
    pub desc: *mut gpio_desc,
    pub data: *mut c_void,
// public:
    pub data): *mut *mut int (jack_status_check)(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_jack {
    pub mutex: mutex,
    pub jack: *mut snd_jack,
    pub card: *mut snd_soc_card,
    pub pins: list_head,
    pub status: c_int,
    pub notifier: blocking_notifier_head,
    pub jack_zones: list_head,
}

// Jack reporting
extern "C" {
    pub fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
}
extern "C" {
    pub fn snd_soc_jack_get_type(jack: *mut snd_soc_jack, micbias_voltage: c_int) -> c_int;
}

