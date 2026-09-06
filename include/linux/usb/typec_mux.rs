//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/typec_mux.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_switch_desc {
    pub fwnode: *mut fwnode_handle,
    pub set: typec_switch_set_fn_t,
    pub name: *const c_char,
    pub drvdata: *mut c_void,
}

extern "C" {
    pub fn typec_switch_put(sw: *mut typec_switch);
}
extern "C" {
    pub fn typec_switch_unregister(sw: *mut typec_switch_dev);
}
extern "C" {
    pub fn typec_switch_set_drvdata(sw: *mut typec_switch_dev, data: *mut c_void);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

extern "C" {
    pub fn fwnode_typec_switch_get(_arg: dev_fwnode(dev)) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_mux_state {
    pub alt: *mut typec_altmode,
    pub mode: c_ulong,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_mux_desc {
    pub fwnode: *mut fwnode_handle,
    pub set: typec_mux_set_fn_t,
    pub name: *const c_char,
    pub drvdata: *mut c_void,
}

extern "C" {
    pub fn typec_mux_put(mux: *mut typec_mux);
}
extern "C" {
    pub fn typec_mux_set(mux: *mut typec_mux, state: *mut typec_mux_state) -> c_int;
}
extern "C" {
    pub fn typec_mux_unregister(mux: *mut typec_mux_dev);
}
extern "C" {
    pub fn typec_mux_set_drvdata(mux: *mut typec_mux_dev, data: *mut c_void);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

extern "C" {
    pub fn fwnode_typec_mux_get(_arg: dev_fwnode(dev)) -> return;
}
