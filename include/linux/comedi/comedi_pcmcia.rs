//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/comedi/comedi_pcmcia.h
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
// comedi_pcmcia.h
// header file for Comedi PCMCIA drivers
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 1997-2000 David A. Schleef <ds@schleef.org>
//

extern "C" {
    pub fn comedi_pcmcia_disable(dev: *mut comedi_device);
}
extern "C" {
    pub fn comedi_pcmcia_auto_unconfig(link: *mut pcmcia_device);
}
//
// module_comedi_pcmcia_driver() - Helper macro for registering a comedi
// PCMCIA driver
// @__comedi_driver: comedi_driver struct
// @__pcmcia_driver: pcmcia_driver struct
//
// Helper macro for comedi PCMCIA drivers which do not do anything special
// in module init/exit. This eliminates a lot of boilerplate. Each
// module may only use this macro once, and calling it replaces
// module_init() and module_exit()
//

