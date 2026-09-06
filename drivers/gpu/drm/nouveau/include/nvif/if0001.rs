//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/if0001.h
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
pub const NVIF_CONTROL_PSTATE_INFO: c_uint = 0x00;
pub const NVIF_CONTROL_PSTATE_ATTR: c_uint = 0x01;
pub const NVIF_CONTROL_PSTATE_USER: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_control_pstate_info_v0 {
    pub version: __u8,
    pub /: *mut *mut __u8 count; / out: number of power states,

    pub /: *mut *mut __s8 ustate_ac; / out: target pstate index,
    pub /: *mut *mut __s8 ustate_dc; / out: target pstate index,
    pub /: *mut *mut __s8 pwrsrc; / out: current power source,

    pub /: *mut *mut __s8 pstate; / out: current pstate index,
    pub pad06: [__u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_control_pstate_attr_v0 {
    pub version: __u8,

    pub query: *mut *mut __s8 state; / in: index of pstate to,
// out: pstate identifier
//
    pub query: *mut *mut __u8 index; / in: index of attribute to,
// out: index of next attribute, or 0 if no more
//
    pub pad03: [__u8; 5],
    pub min: __u32,
    pub max: __u32,
    pub name: [c_char; 32],
    pub unit: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_control_pstate_user_v0 {
    pub version: __u8,

    pub /: *mut *mut __s8 ustate; / in: pstate identifier,
    pub /: *mut *mut __s8 pwrsrc; / in: target power source,
    pub pad03: [__u8; 5],
}
