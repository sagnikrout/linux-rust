//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vboxvideo/hgsmi_channels.h
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


// SPDX-License-Identifier: MIT
// Copyright (C) 2006-2017 Oracle Corporation
//
// Each channel has an 8 bit identifier. There are a number of predefined
// (hardcoded) channels.
//
// HGSMI_CH_HGSMI channel can be used to map a string channel identifier
// to a free 16 bit numerical value. values are allocated in range
// [HGSMI_CH_STRING_FIRST;HGSMI_CH_STRING_LAST].
//
// A reserved channel value
pub const HGSMI_CH_RESERVED: c_uint = 0x00;
// HGCMI: setup and configuration
pub const HGSMI_CH_HGSMI: c_uint = 0x01;
// Graphics: VBVA
pub const HGSMI_CH_VBVA: c_uint = 0x02;
// Graphics: Seamless with a single guest region
pub const HGSMI_CH_SEAMLESS: c_uint = 0x03;
// Graphics: Seamless with separate host windows
pub const HGSMI_CH_SEAMLESS2: c_uint = 0x04;
// Graphics: OpenGL HW acceleration
pub const HGSMI_CH_OPENGL: c_uint = 0x05;
// The first channel index to be used for string mappings (inclusive)
pub const HGSMI_CH_STRING_FIRST: c_uint = 0x20;
// The last channel index for string mappings (inclusive)
pub const HGSMI_CH_STRING_LAST: c_uint = 0xff;
