//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/as102_fe.h
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
// Abilis Systems Single DVB-T Receiver
// Copyright (C) 2014 Mauro Carvalho Chehab <mchehab+samsung@kernel.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct as102_fe_ops {
    pub tune_args): *mut *mut *mut int (set_tune)(void priv, struct as10x_tune_args,
    pub tps): *mut *mut *mut int (get_tps)(void priv, struct as10x_tps,
    pub tstate): *mut *mut *mut int (get_status)(void priv, struct as10x_tune_status,
    pub demod_stats): *mut *mut *mut int (get_stats)(void priv, struct as10x_demod_stats,
    pub elna_cfg): *mut *mut *mut int (stream_ctrl)(void priv, int acquire, uint32_t,
}
