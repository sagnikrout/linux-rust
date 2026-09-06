//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/tw9910.h
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
// tw9910 Driver header
//
// Copyright (C) 2008 Renesas Solutions Corp.
// Kuninori Morimoto <morimoto.kuninori@renesas.com>
//
// Based on ov772x.h
//
// Copyright (C) Kuninori Morimoto <morimoto.kuninori@renesas.com>
//
// MPOUT (multi-purpose output) pin functions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tw9910_mpout_pin {
    TW9910_MPO_VLOSS,
    TW9910_MPO_HLOCK,
    TW9910_MPO_SLOCK,
    TW9910_MPO_VLOCK,
    TW9910_MPO_MONO,
    TW9910_MPO_DET50,
    TW9910_MPO_FIELD,
    TW9910_MPO_RTCO,
}

//
// struct tw9910_video_info - tw9910 driver interface structure
// @buswidth:		Parallel data bus width (8 or 16).
// @mpout:		Selected function of MPOUT (multi-purpose output) pin.
// See enum tw9910_mpout_pin
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw9910_video_info {
    pub buswidth: c_ulong,
    pub mpout: tw9910_mpout_pin,
}
