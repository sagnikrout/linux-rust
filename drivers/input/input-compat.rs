//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/input-compat.h
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
// 32bit compatibility wrappers for the input subsystem.
//
// Very heavily based on evdev.c - Copyright (c) 1999-2002 Vojtech Pavlik
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_event_compat {
    pub sec: compat_ulong_t,
    pub usec: compat_ulong_t,
    pub type: __u16,
    pub code: __u16,
    pub value: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ff_periodic_effect_compat {
    pub waveform: __u16,
    pub period: __u16,
    pub magnitude: __s16,
    pub offset: __s16,
    pub phase: __u16,
    pub envelope: ff_envelope,
    pub custom_len: __u32,
    pub custom_data: compat_uptr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ff_effect_compat {
    pub type: __u16,
    pub id: __s16,
    pub direction: __u16,
    pub trigger: ff_trigger,
    pub replay: ff_replay,
    pub constant: ff_constant_effect,
    pub ramp: ff_ramp_effect,
    pub periodic: ff_periodic_effect_compat,
    pub /: *mut *mut ff_condition_effect condition[2]; / One for each axis,
    pub rumble: ff_rumble_effect,
    pub u: },
}

extern "C" {
    pub fn sizeof(input_event: struct) -> return;
}

