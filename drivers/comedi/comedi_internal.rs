//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/comedi_internal.h
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
// various internal comedi stuff
//
extern "C" {
    pub fn comedi_release_hardware_device(hardware_device: *mut device);
}
extern "C" {
    pub fn comedi_alloc_subdevice_minor(s: *mut comedi_subdevice) -> c_int;
}
extern "C" {
    pub fn comedi_free_subdevice_minor(s: *mut comedi_subdevice);
}
extern "C" {
    pub fn comedi_buf_reset(s: *mut comedi_subdevice);
}
extern "C" {
    pub fn comedi_buf_is_mmapped(s: *mut comedi_subdevice) -> bool;
}
extern "C" {
    pub fn comedi_buf_map_get(bm: *mut comedi_buf_map);
}
extern "C" {
    pub fn comedi_buf_map_put(bm: *mut comedi_buf_map) -> c_int;
}
extern "C" {
    pub fn comedi_buf_write_n_available(s: *mut comedi_subdevice) -> c_uint;
}
extern "C" {
    pub fn comedi_buf_write_n_allocated(s: *mut comedi_subdevice) -> c_uint;
}
extern "C" {
    pub fn _comedi_buf_read_n_available(s: *mut comedi_subdevice) -> c_uint;
}
extern "C" {
    pub fn _comedi_event(dev: *mut comedi_device, s: *mut comedi_subdevice);
}
extern "C" {
    pub fn comedi_device_cancel_all(dev: *mut comedi_device);
}
extern "C" {
    pub fn comedi_can_auto_free_spriv(s: *mut comedi_subdevice) -> bool;
}
// drivers.c
extern "C" {
    pub fn comedi_device_detach_locked(dev: *mut comedi_device);
}
extern "C" {
    pub fn comedi_device_detach(dev: *mut comedi_device);
}

// proc.c
extern "C" {
    pub fn comedi_proc_init();
}
extern "C" {
    pub fn comedi_proc_cleanup();
}

