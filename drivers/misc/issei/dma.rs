//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/issei/dma.h
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
// Copyright (C) 2023-2026 Intel Corporation

//
// struct issei_dma_length - sizes of DMA memory portions
// @h2f: host to firmware buffer size
// @f2h: firmware to host buffer size
// @ctl: control buffer size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct issei_dma_length {
    pub h2f: usize,
    pub f2h: usize,
    pub ctl: usize,
}

//
// struct issei_dma - DMA memory structure
// @vaddr: virtual address
// @daddr: physical address
// @length: memory sizes structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct issei_dma {
    pub vaddr: *mut c_void,
    pub daddr: dma_addr_t,
    pub length: issei_dma_length,
}

// Operation statuses
pub const HAMS_SUCCESS: c_uint = 0x00;
pub const HAMS_PROTOCOL_NOT_SUPPORTED: c_uint = 0x01;
pub const HAMS_DEPRECATED_BUS_MSG: c_uint = 0x02;
pub const HAMS_CLIENT_NOT_EXISTS: c_uint = 0x03;
pub const HAMS_MSG_TOO_BIG: c_uint = 0x04;
pub const HAMS_MSG_NOT_CONSUMED: c_uint = 0x05;
pub const HAMS_CORRUPTED_BUS_MSG: c_uint = 0x06;
pub const HAMS_CORRUPTED_HEADER: c_uint = 0x07;
pub const HAMS_INVALID_LENGTH: c_uint = 0x08;
pub const HAMS_SHARED_MEMORY_SIZE_UNSUPPORTED: c_uint = 0x09;
pub const HAMS_GENERAL_FATAL_ERROR: c_uint = 0xff;
//
// struct issei_dma_data - data passed through channel
// @fw_id: firmware client id
// @host_id: host client id
// @flags: flags bitmap
// @status: operation status
// @length: data length
// @buf: pointer to data buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct issei_dma_data {
    pub fw_id: u16,
    pub host_id: u16,
    pub flags: u32,
    pub status: u32,
    pub length: u32,
    pub buf: *mut c_void,
}

extern "C" {
    pub fn issei_dmam_setup(idev: *mut issei_device) -> c_int;
}
extern "C" {
    pub fn issei_dma_write(idev: *mut issei_device, data: *const issei_dma_data) -> c_int;
}
extern "C" {
    pub fn issei_dma_read(idev: *mut issei_device, data: *mut issei_dma_data) -> c_int;
}
