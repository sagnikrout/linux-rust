//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/if0020.h
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
pub union nvif_chan_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_chan_v0 {
    pub version: __u8,
    pub namelen: __u8,
    pub runlist: __u8,
    pub runq: __u8,
    pub priv: __u8,
    pub pad05: __u8,
    pub devm: __u16,
    pub vmm: __u64,
    pub ctxdma: __u64,
    pub offset: __u64,
    pub length: __u64,
    pub huserd: __u64,
    pub ouserd: __u64,
    pub token: __u32,
    pub chid: __u16,
    pub pad3e: __u8,
pub const NVIF_CHAN_V0_INST_APER_VRAM: c_int = 0;
pub const NVIF_CHAN_V0_INST_APER_HOST: c_int = 1;
pub const NVIF_CHAN_V0_INST_APER_NCOH: c_int = 2;
pub const NVIF_CHAN_V0_INST_APER_INST: c_uint = 0xff;
    pub aper: __u8,
    pub inst: __u64,
    pub name: [__u8; ],
    pub v0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvif_chan_event_args {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_chan_event_v0 {
    pub version: __u8,
pub const NVIF_CHAN_EVENT_V0_NON_STALL_INTR: c_uint = 0x00;
pub const NVIF_CHAN_EVENT_V0_KILLED: c_uint = 0x01;
    pub type: __u8,
    pub v0: },
}
