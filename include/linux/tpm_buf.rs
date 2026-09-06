//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tpm_buf.h
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
// Following copyright information was take from the original file
// <include/linux/tpm.h> where the definitions were moved from:
//
// Copyright (C) 2004,2007,2008 IBM Corporation
//
// Authors:
// Leendert van Doorn <leendert@watson.ibm.com>
// Dave Safford <safford@watson.ibm.com>
// Reiner Sailer <sailer@watson.ibm.com>
// Kylene Hall <kjhall@us.ibm.com>
// Debora Velarde <dvelarde@us.ibm.com>
//
// Maintained by: <tpmdd_devel@lists.sourceforge.net>
//
// Device driver for TCG/TCPA TPM (trusted platform module).
// Specifications at www.trustedcomputinggroup.org
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpm_buf_flags {
// TPM2B format:
    TPM_BUF_TPM2B		= BIT(0),
// The buffer is in invalid and unusable state:
    TPM_BUF_INVALID		= BIT(1),
}

//
// A buffer for constructing and parsing TPM commands, responses and sized
// (TPM2B) buffers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_buf {
    pub flags: u8,
    pub handles: u8,
    pub length: u16,
    pub capacity: u16,
    pub data: [u8; ],
}

extern "C" {
    pub fn tpm_buf_init(buf: *mut tpm_buf, buf_size: u16);
}
extern "C" {
    pub fn tpm_buf_init_sized(buf: *mut tpm_buf, buf_size: u16);
}
extern "C" {
    pub fn tpm_buf_reset(buf: *mut tpm_buf, tag: u16, ordinal: u32);
}
extern "C" {
    pub fn tpm_buf_reset_sized(buf: *mut tpm_buf);
}
extern "C" {
    pub fn tpm_buf_length(buf: *mut tpm_buf) -> u16;
}
extern "C" {
    pub fn tpm_buf_append(buf: *mut tpm_buf, new_data: *const u8, new_length: u16);
}
extern "C" {
    pub fn tpm_buf_append_u8(buf: *mut tpm_buf, value: u8);
}
extern "C" {
    pub fn tpm_buf_append_u16(buf: *mut tpm_buf, value: u16);
}
extern "C" {
    pub fn tpm_buf_append_u32(buf: *mut tpm_buf, value: u32);
}
extern "C" {
    pub fn tpm_buf_read_u8(buf: *mut tpm_buf, offset: *mut off_t) -> u8;
}
extern "C" {
    pub fn tpm_buf_read_u16(buf: *mut tpm_buf, offset: *mut off_t) -> u16;
}
extern "C" {
    pub fn tpm_buf_read_u32(buf: *mut tpm_buf, offset: *mut off_t) -> u32;
}
extern "C" {
    pub fn tpm_buf_append_handle(buf: *mut tpm_buf, handle: u32);
}
