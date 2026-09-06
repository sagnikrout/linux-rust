//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/cudbg_lib_common.h
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
// Copyright (C) 2017 Chelsio Communications.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cudbg_dump_type {
    CUDBG_DUMP_TYPE_MINI = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cudbg_compression_type {
    CUDBG_COMPRESSION_NONE = 1,
    CUDBG_COMPRESSION_ZLIB,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_hdr {
    pub signature: u32,
    pub hdr_len: u32,
    pub major_ver: u16,
    pub minor_ver: u16,
    pub data_len: u32,
    pub hdr_flags: u32,
    pub max_entities: u16,
    pub chip_ver: u8,
    pub dump_type:3: u8,
    pub reserved1:1: u8,
    pub compress_type:4: u8,
    pub reserved: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_entity_hdr {
    pub entity_type: u32,
    pub start_offset: u32,
    pub size: u32,
    pub hdr_flags: c_int,
    pub sys_warn: u32,
    pub sys_err: u32,
    pub num_pad: u8,
    pub /: *mut *mut u8 flag; / bit 0 is used to indicate ext data,
    pub reserved1: [u8; 2],
    pub /: *mut *mut u32 next_ext_offset; / pointer to next extended entity meta data,
    pub reserved: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_ver_hdr {
    pub signature: u32,
    pub revision: u16,
    pub size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_buffer {
    pub size: u32,
    pub offset: u32,
    pub data: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_error {
    pub sys_err: c_int,
    pub sys_warn: c_int,
    pub app_err: c_int,
}

