//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/si2157_priv.h
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
// Silicon Labs Si2146/2147/2148/2157/2158 silicon tuner driver
//
// Copyright (C) 2014 Antti Palosaari <crope@iki.fi>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si2157_pads {
    SI2157_PAD_RF_INPUT,
    SI2157_PAD_VID_OUT,
    SI2157_PAD_AUD_OUT,
    SI2157_NUM_PADS
}

// state struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si2157_dev {
    pub i2c_mutex: mutex,
    pub fe: *mut dvb_frontend,
    pub active:1: c_uint,
    pub inversion:1: c_uint,
    pub dont_load_firmware:1: c_uint,
    pub part_id: u8,
    pub if_port: u8,
    pub if_frequency: u32,
    pub bandwidth: u32,
    pub frequency: u32,
    pub stat_work: delayed_work,

    pub mdev: *mut media_device,
    pub ent: media_entity,
    pub pad: [media_pad; SI2157_NUM_PADS],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si2157_part_id {
    SI2141 = 41,
    SI2146 = 46,
    SI2147 = 47,
    SI2148 = 48,
    SI2157 = 57,
    SI2158 = 58,
    SI2177 = 77,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si2157_tuner_info {
    pub part_id: si2157_part_id,
    pub rom_id: c_uchar,
    pub required: bool,
    pub fw_alt_name: *const *const char fw_name,,
}

// firmware command struct
pub const SI2157_ARGLEN: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si2157_cmd {
    pub args: [u8; SI2157_ARGLEN],
    pub wlen: unsigned,
    pub rlen: unsigned,
}

// Old firmware namespace

// New firmware namespace

