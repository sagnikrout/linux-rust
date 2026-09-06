//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/gaudi2_async_ids_map_extended.h
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
// Copyright 2018-2022 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum event_reset_type {
    EVENT_RESET_TYPE_NONE,
    EVENT_RESET_TYPE_COMPUTE,
    EVENT_RESET_TYPE_HARD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gaudi2_async_events_ids_map {
    pub fc_id: c_int,
    pub cpu_id: c_int,
    pub valid: c_int,
    pub msg: c_int,
    pub reset: c_int,
    pub name: [c_char; 64],
}
