//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ctxfi/ctimap.h
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
// Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
//
// @File	ctimap.h
//
// @Brief
// This file contains the definition of generic input mapper operations
// for input mapper management.
//
// @Author	Liu Chun
// @Date 	May 23 2008
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imapper {
    pub /: *mut *mut unsigned short slot; / the id of the slot containing input data,
    pub /: *mut *mut unsigned short user; / the id of the user resource consuming data,
    pub /: *mut *mut unsigned short addr; / the input mapper ram id,
    pub /: *mut *mut unsigned short next; / the next input mapper ram id,
    pub list: list_head,
}

extern "C" {
    pub fn free_input_mapper_list(mappers: *mut list_head);
}
