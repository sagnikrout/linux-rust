//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/ix2505v.h
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
// Driver for Sharp IX2505V (marked B0017) DVB-S silicon tuner
//
// Copyright (C) 2010 Malcolm Priestley
//

//
// struct ix2505v_config - ix2505 attachment configuration
//
// @tuner_address: tuner address
// @tuner_gain: Baseband AMP gain control 0/1=0dB(default) 2=-2bB 3=-4dB
// @tuner_chargepump: Charge pump output +/- 0=120 1=260 2=555 3=1200(default)
// @min_delay_ms: delay after tune
// @tuner_write_only: disables reads
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ix2505v_config {
    pub tuner_address: u8,
    pub tuner_gain: u8,
    pub tuner_chargepump: u8,
    pub min_delay_ms: c_int,
    pub tuner_write_only: u8,
}

//
// ix2505v_attach - Attach a ix2505v tuner to the supplied frontend structure.
//
// @fe: Frontend to attach to.
// @config: pointer to &struct ix2505v_config
// @i2c: pointer to &struct i2c_adapter.
//
// return: FE pointer on success, NULL on failure.
//

