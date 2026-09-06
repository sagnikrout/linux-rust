//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/fc001x-common.h
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
// Fitipower FC0012 & FC0013 tuner driver - common defines
//
// Copyright (C) 2012 Hans-Frieder Vogt <hfvogt@gmx.net>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc001x_xtal_freq {
    FC_XTAL_27_MHZ,		/* 27000000 */
    FC_XTAL_28_8_MHZ,	/* 28800000 */
    FC_XTAL_36_MHZ,		/* 36000000 */
}

//
// enum fc001x_fe_callback_commands - Frontend callbacks
//
// @FC_FE_CALLBACK_VHF_ENABLE: enable VHF or UHF
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc001x_fe_callback_commands {
    FC_FE_CALLBACK_VHF_ENABLE,
}
