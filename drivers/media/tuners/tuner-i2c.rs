//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/tuner-i2c.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tuner_i2c_props {
    pub addr: u8,
    pub adap: *mut i2c_adapter,
// used for tuner instance management
    pub count: c_int,
    pub name: *mut c_char,
}

// Callers must declare as a global for the module:
//
// static LIST_HEAD(hybrid_tuner_instance_list);
//
// hybrid_tuner_instance_list should be the third argument
// passed into hybrid_tuner_request_state().
//
// state structure must contain the following:
//
// struct list_head	hybrid_tuner_instance_list;
// struct tuner_i2c_props	i2c_props;
//
// hybrid_tuner_instance_list (both within state structure and globally)
// is only required if the driver is using hybrid_tuner_request_state
// and hybrid_tuner_release_state to manage state sharing between
// multiple instances of hybrid tuners.
//

// TO DO: convert all callers of these macros to pass in
// struct tuner_i2c_props, then remove the macro wrappers

//
// The return value of hybrid_tuner_request_state indicates the number of
// instances using this tuner object.
//
// 0 - no instances, indicates an error - kzalloc must have failed
//
// 1 - one instance, indicates that the tuner object was created successfully
//
// 2 (or more) instances, indicates that an existing tuner object was found
//

