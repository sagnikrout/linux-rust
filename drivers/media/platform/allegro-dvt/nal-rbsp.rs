//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/allegro-dvt/nal-rbsp.h
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
// Copyright (C) 2019-2020 Pengutronix, Michael Tretter <kernel@pengutronix.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nal_rbsp_ops {
    pub val): *mut *mut *mut int (rbsp_bit)(struct rbsp rbsp, int,
    pub val): *mut *mut *mut int (rbsp_bits)(struct rbsp rbsp, int n, unsigned int,
    pub val): *mut *mut *mut int (rbsp_uev)(struct rbsp rbsp, unsigned int,
    pub val): *mut *mut *mut int (rbsp_sev)(struct rbsp rbsp, int,
}

//
// struct rbsp - State object for handling a raw byte sequence payload
// @data: pointer to the data of the rbsp
// @size: maximum size of the data of the rbsp
// @pos: current bit position inside the rbsp
// @num_consecutive_zeros: number of zeros before @pos
// @ops: per datatype functions for interacting with the rbsp
// @error: an error occurred while handling the rbsp
//
// This struct is passed around the various parsing functions and tracks the
// current position within the raw byte sequence payload.
//
// The @ops field allows to separate the operation, i.e., reading/writing a
// value from/to that rbsp, from the structure of the NAL unit. This allows to
// have a single function for iterating the NAL unit, while @ops has function
// pointers for handling each type in the rbsp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rbsp {
    pub data: *mut u8,
    pub size: usize,
    pub pos: c_uint,
    pub num_consecutive_zeros: c_uint,
    pub ops: *mut nal_rbsp_ops,
    pub error: c_int,
}

extern "C" {
    pub fn rbsp_unsupported(rbsp: *mut rbsp);
}
extern "C" {
    pub fn rbsp_bit(rbsp: *mut rbsp, value: *mut c_int);
}
extern "C" {
    pub fn rbsp_bits(rbsp: *mut rbsp, n: c_int, value: *mut c_int);
}
extern "C" {
    pub fn rbsp_uev(rbsp: *mut rbsp, value: *mut c_uint);
}
extern "C" {
    pub fn rbsp_sev(rbsp: *mut rbsp, value: *mut c_int);
}
extern "C" {
    pub fn rbsp_trailing_bits(rbsp: *mut rbsp);
}
