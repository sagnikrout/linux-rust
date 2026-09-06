//! Automatically rewritten from C to Rust
//! Source: drivers/hsi/hsi_boardinfo.c
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
// HSI clients registration interface
//
// Copyright (C) 2010 Nokia Corporation. All rights reserved.
//
// Contact: Carlos Chinea <carlos.chinea@nokia.com>
//

//
// hsi_board_list is only used internally by the HSI framework.
// No one else is allowed to make use of it.
//
    LIST_HEAD(hsi_board_list);
    EXPORT_SYMBOL_GPL(hsi_board_list);
//
// hsi_register_board_info - Register HSI clients information
// @info: Array of HSI clients on the board
// @len: Length of the array
//
// HSI clients are statically declared and registered on board files.
//
// HSI clients will be automatically registered to the HSI bus once the
// controller and the port where the clients wishes to attach are registered
// to it.
//
// Return -errno on failure, 0 on success.
//
    int __init hsi_register_board_info(struct hsi_board_info const *info,
    unsigned int len)
    {
    struct hsi_cl_info *cl_info;
    cl_info = kzalloc_objs(*cl_info, len);
    if (!cl_info)
    return -ENOMEM;
    for (; len; len--, info++, cl_info++) {
    cl_info.info = *info;
    list_add_tail(&cl_info.list, &hsi_board_list);
    }
    return 0;
    }
