//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/char/hmcdrv_ftp.h
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
// SE/HMC Drive FTP Services
//
// Copyright IBM Corp. 2013
// Author(s): Ralf Hoppe (rhoppe@de.ibm.com)
//

//
// HMC drive FTP Service max. length of path (w/ EOS)
//
pub const HMCDRV_FTP_FIDENT_MAX: c_int = 192;
//
// enum hmcdrv_ftp_cmdid - HMC drive FTP commands
// @HMCDRV_FTP_NOOP: do nothing (only for probing)
// @HMCDRV_FTP_GET: read a file
// @HMCDRV_FTP_PUT: (over-) write a file
// @HMCDRV_FTP_APPEND: append to a file
// @HMCDRV_FTP_DIR: list directory long (ls -l)
// @HMCDRV_FTP_NLIST: list files, no directories (name list)
// @HMCDRV_FTP_DELETE: delete a file
// @HMCDRV_FTP_CANCEL: cancel operation (SCLP/LPAR only)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hmcdrv_ftp_cmdid {
    HMCDRV_FTP_NOOP = 0,
    HMCDRV_FTP_GET = 1,
    HMCDRV_FTP_PUT = 2,
    HMCDRV_FTP_APPEND = 3,
    HMCDRV_FTP_DIR = 4,
    HMCDRV_FTP_NLIST = 5,
    HMCDRV_FTP_DELETE = 6,
    HMCDRV_FTP_CANCEL = 7
}

//
// struct hmcdrv_ftp_cmdspec - FTP command specification
// @id: FTP command ID
// @ofs: offset in file
// @fname: filename (ASCII), null-terminated
// @buf: kernel-space transfer data buffer, 4k aligned
// @len: (max) number of bytes to transfer from/to @buf
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmcdrv_ftp_cmdspec {
    pub id: hmcdrv_ftp_cmdid,
    pub ofs: loff_t,
    pub fname: *const c_char,
    pub buf: *mut void __kernel,
    pub len: usize,
}

extern "C" {
    pub fn hmcdrv_ftp_startup() -> c_int;
}
extern "C" {
    pub fn hmcdrv_ftp_shutdown();
}
extern "C" {
    pub fn hmcdrv_ftp_probe() -> c_int;
}
extern "C" {
    pub fn hmcdrv_ftp_do(ftp: *const hmcdrv_ftp_cmdspec) -> isize;
}
