//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/of_pdt.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Definitions for building a device tree by calling into the
// Open Firmware PROM.
//
// Copyright (C) 2010  Andres Salomon <dilinger@queued.net>
//
// overridable operations for calling into the PROM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_pdt_ops {
//
// buf should be 32 bytes; return 0 on success.
// If prev is NULL, the first property will be returned.
//
    pub buf): *mut *mut *mut int (nextprop)(phandle node, char prev, char,
// for both functions, return proplen on success; -1 on error
    pub prop): *const *const int (getproplen)(phandle node, char,
    pub bufsize): c_int,
// phandles are 0 if no child or sibling exists
    pub parent): *mut *mut phandle (getchild)(phandle,
    pub node): *mut *mut phandle (getsibling)(phandle,
// return 0 on success; fill in 'len' with number of bytes in path
    pub len): *const *const *const int (pkg2path)(phandle node, char buf, int buflen, int,
}

// for building the device tree
extern "C" {
    pub fn of_pdt_build_devicetree(root_node: phandle, ops: *mut of_pdt_ops);
}
