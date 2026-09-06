//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/rc/img-ir/img-ir-raw.h
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
// ImgTec IR Raw Decoder found in PowerDown Controller.
//
// Copyright 2010-2014 Imagination Technologies Ltd.
//

//
// struct img_ir_priv_raw - Private driver data for raw decoder.
// @rdev:		Raw remote control device
// @timer:		Timer to echo samples to keep soft decoders happy.
// @last_status:	Last raw status bits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ir_priv_raw {
    pub rdev: *mut rc_dev,
    pub timer: timer_list,
    pub last_status: u32,
}

extern "C" {
    pub fn img_ir_isr_raw(priv: *mut img_ir_priv, irq_status: u32);
}
extern "C" {
    pub fn img_ir_setup_raw(priv: *mut img_ir_priv);
}
extern "C" {
    pub fn img_ir_probe_raw(priv: *mut img_ir_priv) -> c_int;
}
extern "C" {
    pub fn img_ir_remove_raw(priv: *mut img_ir_priv);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ir_priv_raw {
}

