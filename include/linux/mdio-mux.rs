//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mdio-mux.h
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


//
// MDIO bus multiplexer framwork.
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (C) 2011, 2012 Cavium, Inc.
//

// mdio_mux_init() - Initialize a MDIO mux
// @dev		The device owning the MDIO mux
// @mux_node	The device node of the MDIO mux
// @switch_fn	The function called for switching target MDIO child
// mux_handle	A pointer to a (void *) used internaly by mdio-mux
// @data	Private data used by switch_fn()
// @mux_bus	An optional parent bus (Other case are to use parent_bus property)
//
extern "C" {
    pub fn mdio_mux_uninit(mux_handle: *mut c_void);
}
