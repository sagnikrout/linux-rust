//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/u_tcm.h
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
// u_tcm.h
//
// Utility definitions for the tcm function
//
// Copyright (c) 2015 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Andrzej Pietrasiewicz <andrzej.p@xxxxxxxxxxx>
//

//
// @dependent: optional dependent module. Meant for legacy gadget.
// If non-null its refcount will be increased when a tpg is created and
// decreased when tpg is dropped.
// @dep_lock: lock for dependent module operations.
// @ready: true if the dependent module information is set.
// @can_attach: true a function can be bound to gadget
// @has_dep: true if there is a dependent module
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_tcm_opts {
    pub func_inst: usb_function_instance,
    pub dependent: *mut module,
    pub dep_lock: mutex,
    pub ready: bool,
    pub can_attach: bool,
    pub has_dep: bool,
//
// Callbacks to be removed when legacy tcm gadget disappears.
//
// If you use the new function registration interface
// programmatically, you MUST set these callbacks to
// something sensible (e.g. probe/remove the composite).
//
    pub ): *mut *mut int (tcm_register_callback)(struct usb_function_instance,
    pub ): *mut *mut void (tcm_unregister_callback)(struct usb_function_instance,
}
