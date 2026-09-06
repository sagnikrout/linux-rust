//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/char/sclp_diag.h
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
// Copyright IBM Corp. 2013
// Author(s): Ralf Hoppe (rhoppe@de.ibm.com)
//

// return codes for Diagnostic Test FTP Service, as indicated in member
// sclp_diag_ftp::ldflg
//
pub const SCLP_DIAG_FTP_OK: c_uint = 0x80U /* success */;
pub const SCLP_DIAG_FTP_LDFAIL: c_uint = 0x01U /* load failed */;
pub const SCLP_DIAG_FTP_LDNPERM: c_uint = 0x02U /* not allowed */;
pub const SCLP_DIAG_FTP_LDRUNS: c_uint = 0x03U /* LD runs */;
pub const SCLP_DIAG_FTP_LDNRUNS: c_uint = 0x04U /* LD does not run */;
pub const SCLP_DIAG_FTP_XPCX: c_uint = 0x80 /* PCX communication code */;

//
// length of Diagnostic Test FTP Service event buffer
//

//
// struct sclp_diag_ftp - Diagnostic Test FTP Service model-dependent data
// @pcx: code for PCX communication (should be 0x80)
// @ldflg: load flag (see defines above)
// @cmd: FTP command
// @pgsize: page size (0 = 4kB, 1 = large page size)
// @srcflg: source flag
// @spare: reserved (zeroes)
// @offset: file offset
// @fsize: file size
// @length: buffer size resp. bytes transferred
// @failaddr: failing address
// @bufaddr: buffer address, virtual
// @asce: region or segment table designation
// @fident: file name (ASCII, zero-terminated)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclp_diag_ftp {
    pub pcx: u8,
    pub ldflg: u8,
    pub cmd: u8,
    pub pgsize: u8,
    pub srcflg: u8,
    pub spare: u8,
    pub offset: u64,
    pub fsize: u64,
    pub length: u64,
    pub failaddr: u64,
    pub bufaddr: u64,
    pub asce: u64,
    pub fident: [u8; 256],
    pub __packed: },
//
// struct sclp_diag_evbuf - Diagnostic Test (ET7) Event Buffer
// @hdr: event buffer header
// @route: diagnostic route
// @mdd: model-dependent data (@route dependent)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclp_diag_evbuf {
    pub hdr: evbuf_header,
    pub route: u16,
    pub ftp: sclp_diag_ftp,
    pub mdd: },
    pub __packed: },
//
// struct sclp_diag_sccb - Diagnostic Test (ET7) SCCB
// @hdr: SCCB header
// @evbuf: event buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclp_diag_sccb {
    pub hdr: sccb_header,
    pub evbuf: sclp_diag_evbuf,
    pub __packed: },
