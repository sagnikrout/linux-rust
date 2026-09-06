//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/asihpi/hpidspcd.h
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

// Header structure for dsp firmware file
//
// #ifndef DISABLE_PRAGMA_PACK1
// #pragma pack(push, 1)
// #endif
#[repr(C)]
#[derive(Copy, Clone)]
pub struct code_header {
// Size in bytes including header
    pub size: u32,
// File type tag "CODE" == 0x45444F43
    pub type: u32,
// Adapter model number
    pub adapter: u32,
// Firmware version
    pub version: u32,
// Data checksum
    pub checksum: u32,
}

// #ifndef DISABLE_PRAGMA_PACK1
// #pragma pack(pop)
// #endif
// ? Don't need the pragmas?
// Descriptor for dspcode from firmware loader
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_code {
// copy of  file header
    pub header: code_header,
// Expected number of words in the whole dsp code,INCL header
    pub block_length: u32,
// Number of words read so far
    pub word_count: u32,
// internal state of DSP code reader
    pub pvt: *mut dsp_code_private,
}

// Prepare *psDspCode to refer to the requested adapter's firmware.
//
// Code identifier, usually adapter family
// Pointer to DSP code control structure
// Pointer to dword to receive OS specific error code
// Close the DSP code file
extern "C" {
    pub fn hpi_dsp_code_close(ps_dsp_code: *mut dsp_code);
}
// Rewind to the beginning of the DSP code file (for verify)
extern "C" {
    pub fn hpi_dsp_code_rewind(ps_dsp_code: *mut dsp_code);
}
// Read one word from the dsp code file
//
// < DSP code descriptor
// Get a block of dsp code into an internal buffer, and provide a pointer to
//
// Pointer to store (Pointer to code buffer)
