//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/auxdisplay/line-display.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Character line display core support
//
// Copyright (C) 2016 Imagination Technologies
// Author: Paul Burton <paul.burton@mips.com>
//
// Copyright (C) 2021 Glider bv
// Copyright (C) 2025 Jean-François Lessard
//

//
// enum linedisp_map_type - type of the character mapping
// @LINEDISP_MAP_SEG7: Map characters to 7 segment display
// @LINEDISP_MAP_SEG14: Map characters to 14 segment display
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum linedisp_map_type {
    LINEDISP_MAP_SEG7,
    LINEDISP_MAP_SEG14,
}

//
// struct linedisp_map - character mapping
// @type: type of the character mapping
// @map: conversion character mapping
// @size: size of the @map
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct linedisp_map {
    pub type: linedisp_map_type,
    pub seg7: seg7_conversion_map,
    pub seg14: seg14_conversion_map,
    pub map: },
    pub size: c_uint,
}

//
// struct linedisp_ops - character line display operations
// @get_map_type: Function called to get the character mapping, if required
// @update: Function called to update the display. This must not sleep!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct linedisp_ops {
    pub linedisp): *mut *mut int (get_map_type)(struct linedisp,
    pub linedisp): *mut *mut void (update)(struct linedisp,
}

//
// struct linedisp - character line display private data structure
// @dev: the line display device
// @timer: timer used to implement scrolling
// @ops: character line display operations
// @buf: pointer to the buffer for the string currently displayed
// @message: the full message to display or scroll on the display
// @num_chars: the number of characters that can be displayed
// @message_len: the length of the @message string
// @scroll_pos: index of the first character of @message currently displayed
// @scroll_rate: scroll interval in jiffies
// @id: instance id of this display
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct linedisp {
    pub dev: device,
    pub timer: timer_list,
    pub ops: *const linedisp_ops,
    pub map: *mut linedisp_map,
    pub buf: *mut c_char,
    pub message: *mut c_char,
    pub num_chars: c_uint,
    pub message_len: c_uint,
    pub scroll_pos: c_uint,
    pub scroll_rate: c_uint,
    pub id: c_uint,
}

extern "C" {
    pub fn linedisp_detach(dev: *mut device);
}
extern "C" {
    pub fn linedisp_unregister(linedisp: *mut linedisp);
}
