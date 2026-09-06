//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/asihpi/hpi6000.h
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
pub const HPI_NMIXER_CONTROLS: c_int = 200;
//
// Control caching is always supported in the HPI code.
// The DSP should make sure that dwControlCacheSizeInBytes is initialized to 0
// during boot to make it in-active.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_hif_6000 {
    pub host_cmd: u32,
    pub dsp_ack: u32,
    pub address: u32,
    pub length: u32,
    pub message_buffer_address: u32,
    pub response_buffer_address: u32,
    pub dsp_number: u32,
    pub adapter_info: u32,
    pub control_cache_is_dirty: u32,
    pub control_cache_address: u32,
    pub control_cache_size_in_bytes: u32,
    pub control_cache_count: u32,
}

// Command/status exchanged between host and DSP
pub const HPI_HIF_IDLE: c_int = 0;
pub const HPI_HIF_SEND_MSG: c_int = 1;
pub const HPI_HIF_GET_RESP: c_int = 2;
pub const HPI_HIF_DATA_MASK: c_uint = 0x10;
pub const HPI_HIF_SEND_DATA: c_uint = 0x13;
pub const HPI_HIF_GET_DATA: c_uint = 0x14;
pub const HPI_HIF_SEND_DONE: c_int = 5;
pub const HPI_HIF_RESET: c_int = 9;
