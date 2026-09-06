//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/if0011.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_conn_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_conn_v0 {
    pub version: __u8,
    pub /: *mut *mut __u8 id; / DCB connector table index.,
    pub pad02: [__u8; 6],
pub const NVIF_CONN_V0_VGA: c_uint = 0x00;
pub const NVIF_CONN_V0_TV: c_uint = 0x01;
pub const NVIF_CONN_V0_DVI_I: c_uint = 0x02;
pub const NVIF_CONN_V0_DVI_D: c_uint = 0x03;
pub const NVIF_CONN_V0_LVDS: c_uint = 0x04;
pub const NVIF_CONN_V0_LVDS_SPWG: c_uint = 0x05;
pub const NVIF_CONN_V0_HDMI: c_uint = 0x06;
pub const NVIF_CONN_V0_DP: c_uint = 0x07;
pub const NVIF_CONN_V0_EDP: c_uint = 0x08;
    pub type: __u8,
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_conn_event_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_conn_event_v0 {
    pub version: __u8,
pub const NVIF_CONN_EVENT_V0_PLUG: c_uint = 0x01;
pub const NVIF_CONN_EVENT_V0_UNPLUG: c_uint = 0x02;
pub const NVIF_CONN_EVENT_V0_IRQ: c_uint = 0x04;
    pub types: __u8,
    pub pad02: [__u8; 6],
    pub v0: },
}
