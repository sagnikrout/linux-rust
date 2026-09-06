//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/block/dasd_diag.h
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
// Author(s)......: Holger Smolinski <Holger.Smolinski@de.ibm.com>
// Based on.......: linux/drivers/s390/block/mdisk.h
// ...............: by Hartmunt Penner <hpenner@de.ibm.com>
// Bugreports.to..: <Linux390@de.ibm.com>
// Copyright IBM Corp. 1999, 2000
//
pub const MDSK_WRITE_REQ: c_uint = 0x01;
pub const MDSK_READ_REQ: c_uint = 0x02;
pub const INIT_BIO: c_uint = 0x00;
pub const RW_BIO: c_uint = 0x01;
pub const TERM_BIO: c_uint = 0x02;
pub const DEV_CLASS_FBA: c_uint = 0x01;
pub const DEV_CLASS_ECKD: c_uint = 0x04;
pub const DASD_DIAG_CODE_31BIT: c_uint = 0x03;
pub const DASD_DIAG_CODE_64BIT: c_uint = 0x07;
pub const DASD_DIAG_RWFLAG_ASYNC: c_uint = 0x02;
pub const DASD_DIAG_RWFLAG_NOCACHE: c_uint = 0x01;
pub const DASD_DIAG_FLAGA_FORMAT_64BIT: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_diag_characteristics {
    pub dev_nr: u16,
    pub rdc_len: u16,
    pub vdev_class: u8,
    pub vdev_type: u8,
    pub vdev_status: u8,
    pub vdev_flags: u8,
    pub rdev_class: u8,
    pub rdev_type: u8,
    pub rdev_model: u8,
    pub rdev_features: u8,
// C attribute field omitted

pub type blocknum_t = u64;
pub type sblocknum_t = i64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_diag_bio {
    pub type: u8,
    pub status: u8,
    pub spare1: [u8; 2],
    pub alet: u32,
    pub block_number: blocknum_t,
    pub buffer: *mut c_void,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_diag_init_io {
    pub dev_nr: u16,
    pub flaga: u8,
    pub spare1: [u8; 21],
    pub block_size: u32,
    pub spare2: [u8; 4],
    pub offset: blocknum_t,
    pub start_block: sblocknum_t,
    pub end_block: blocknum_t,
    pub spare3: [u8; 8],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_diag_rw_io {
    pub dev_nr: u16,
    pub flaga: u8,
    pub spare1: [u8; 21],
    pub key: u8,
    pub flags: u8,
    pub spare2: [u8; 2],
    pub block_count: u32,
    pub alet: u32,
    pub spare3: [u8; 4],
    pub interrupt_params: u64,
    pub bio_list: *mut dasd_diag_bio,
    pub spare4: [u8; 8],
// C attribute field omitted
