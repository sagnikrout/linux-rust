//! Automatically rewritten from C Header to Rust Module
//! Source: sound/hda/codecs/side-codecs/cs35l41_hda.h
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
// CS35L41 ALSA HDA audio driver
//
// Copyright 2021 Cirrus Logic, Inc.
//
// Author: Lucas Tanure <tanureal@opensource.cirrus.com>
//

pub const CS35L41_MAX_ACCEPTABLE_SPI_SPEED_HZ: c_int = 1000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l41_amp_cal_data {
    pub calTarget: [u32; 2],
    pub calTime: [u32; 2],
    pub calAmbient: i8,
    pub calStatus: u8,
    pub calR: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l41_amp_efi_data {
    pub size: u32,
    pub count: u32,
    pub data: [cs35l41_amp_cal_data; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs35l41_hda_spk_pos {
    CS35L41_LEFT,
    CS35L41_RIGHT,
    CS35L41_CENTER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs35l41_hda_gpio_function {
    CS35L41_NOT_USED,
    CS35l41_VSPK_SWITCH,
    CS35L41_INTERRUPT,
    CS35l41_SYNC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum control_bus {
    I2C,
    SPI
}

    pub snd_kcontrol: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l41_hda {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub cs_gpio: *mut gpio_desc,
    pub hw_cfg: cs35l41_hw_cfg,
    pub codec: *mut hda_codec,
    pub irq: c_int,
    pub index: c_int,
    pub channel_index: c_int,
    pub irq_errors: unsigned volatile long,
    pub amp_name: *const c_char,
    pub acpi_subsystem_id: *const c_char,
    pub firmware_type: c_int,
    pub speaker_id: c_int,
    pub fw_mutex: mutex,
    pub fw_load_work: work_struct,
    pub fw_type_ctl: *mut snd_kcontrol,
    pub fw_load_ctl: *mut snd_kcontrol,
    pub mute_override_ctl: *mut snd_kcontrol,
    pub irq_data: *mut regmap_irq_chip_data,
    pub firmware_running: bool,
    pub request_fw_load: bool,
    pub fw_request_ongoing: bool,
    pub halo_initialized: bool,
    pub playback_started: bool,
    pub cs_dsp: cs_dsp,
    pub dacpi: *mut acpi_device,
    pub mute_override: bool,
    pub control_bus: control_bus,
    pub bypass_fw: bool,
    pub tuning_gain: c_uint,
    pub cal_data: cirrus_amp_cal_data,
    pub cal_data_valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum halo_state {
    HALO_STATE_CODE_INIT_DOWNLOAD = 0,
    HALO_STATE_CODE_START,
    HALO_STATE_CODE_RUN
}

extern "C" {
    pub fn cs35l41_hda_remove(dev: *mut device);
}
extern "C" {
    pub fn cs35l41_get_speaker_id(dev: *mut device, amp_index: c_int, num_amps: c_int, fixed_gpio_id: c_int) -> c_int;
}
extern "C" {
    pub fn cs35l41_hda_parse_acpi(cs35l41: *mut cs35l41_hda, physdev: *mut device, id: c_int) -> c_int;
}
