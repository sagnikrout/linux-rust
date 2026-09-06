//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/item.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2015-2018 Mellanox Technologies. All rights reserved

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_item {
    pub /: *mut *mut unsigned short offset; / bytes in container,
    pub /: *mut *mut short step; / step in bytes for indexed items,
    pub /: *mut *mut unsigned short in_step_offset; / offset within one step,
    pub /: *mut *mut unsigned char shift; / shift in bits,
    pub /: *mut *mut unsigned char element_size; / size of element in bit array,
    pub no_real_shift: bool,
    pub bits: c_uchar,
    pub bytes: c_ushort,
    pub size: },
    pub name: *const c_char,
}

// shift = in_byte_index * item->element_size;

// _type: cmd_mbox, reg, etc.
// _cname: containter name (e.g. command name, register name)
// _iname: item name within the container
//

pub const LOCAL_PORT_LSB_SIZE: c_int = 8;
pub const LOCAL_PORT_MSB_SIZE: c_int = 2;

