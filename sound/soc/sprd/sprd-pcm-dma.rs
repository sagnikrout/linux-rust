//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sprd/sprd-pcm-dma.h
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

pub const SPRD_PCM_CHANNEL_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_pcm_dma_params {
    pub dev_phys: [dma_addr_t; SPRD_PCM_CHANNEL_MAX],
    pub datawidth: [u32; SPRD_PCM_CHANNEL_MAX],
    pub fragment_len: [u32; SPRD_PCM_CHANNEL_MAX],
    pub chan_name: [*const c_char; SPRD_PCM_CHANNEL_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_compr_playinfo {
    pub total_time: c_int,
    pub current_time: c_int,
    pub total_data_length: c_int,
    pub current_data_offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_compr_params {
    pub direction: u32,
    pub rate: u32,
    pub sample_rate: u32,
    pub channels: u32,
    pub format: u32,
    pub period: u32,
    pub periods: u32,
    pub info_phys: u32,
    pub info_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_compr_callback {
    pub data): *mut *mut void (drain_notify)(void,
    pub drain_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_compr_ops {
    pub cb): *mut *mut int (open)(int str_id, struct sprd_compr_callback,
    pub str_id): *mut *mut int (close)(int,
    pub str_id): *mut *mut int (start)(int,
    pub str_id): *mut *mut int (stop)(int,
    pub str_id): *mut *mut int (pause)(int,
    pub str_id): *mut *mut int (pause_release)(int,
    pub received_total): *mut *mut int (drain)(u64,
    pub params): *mut *mut int (set_params)(int str_id, struct sprd_compr_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_compr_data {
    pub ops: *mut sprd_compr_ops,
    pub dma_params: *mut sprd_pcm_dma_params,
}
