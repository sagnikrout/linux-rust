//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/char/sclp_sdias.h
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
// SCLP "store data in absolute storage"
//
// Copyright IBM Corp. 2003, 2013
//

pub const SDIAS_EQ_STORE_DATA: c_uint = 0x0;
pub const SDIAS_EQ_SIZE: c_uint = 0x1;
pub const SDIAS_DI_FCP_DUMP: c_uint = 0x0;
pub const SDIAS_ASA_SIZE_32: c_uint = 0x0;
pub const SDIAS_ASA_SIZE_64: c_uint = 0x1;
pub const SDIAS_EVSTATE_ALL_STORED: c_uint = 0x0;
pub const SDIAS_EVSTATE_NO_DATA: c_uint = 0x3;
pub const SDIAS_EVSTATE_PART_STORED: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdias_evbuf {
    pub hdr: evbuf_header,
    pub event_qual: u8,
    pub data_id: u8,
    pub reserved2: u64,
    pub event_id: u32,
    pub reserved3: u16,
    pub asa_size: u8,
    pub event_status: u8,
    pub reserved4: u32,
    pub blk_cnt: u32,
    pub asa: u64,
    pub reserved5: u32,
    pub fbn: u32,
    pub reserved6: u32,
    pub lbn: u32,
    pub reserved7: u16,
    pub dbs: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdias_sccb {
    pub hdr: sccb_header,
    pub evbuf: sdias_evbuf,
    pub __packed: },
