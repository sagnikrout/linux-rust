//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vboxvideo/hgsmi_ch_setup.h
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
// Tell the host the location of hgsmi_host_flags structure, where the host
// can write information about pending buffers, etc, and which can be quickly
// polled by the guest without a need to port IO.
//
pub const HGSMI_CC_HOST_FLAGS_LOCATION: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hgsmi_buffer_location {
    pub buf_location: u32,
    pub buf_len: u32,
    pub __packed: },
// HGSMI setup and configuration data structures.
pub const HGSMIHOSTFLAGS_COMMANDS_PENDING: c_uint = 0x01u;
pub const HGSMIHOSTFLAGS_IRQ: c_uint = 0x02u;
pub const HGSMIHOSTFLAGS_VSYNC: c_uint = 0x10u;
pub const HGSMIHOSTFLAGS_HOTPLUG: c_uint = 0x20u;
pub const HGSMIHOSTFLAGS_CURSOR_CAPABILITIES: c_uint = 0x40u;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hgsmi_host_flags {
    pub host_flags: u32,
    pub reserved: [u32; 3],
    pub __packed: },
