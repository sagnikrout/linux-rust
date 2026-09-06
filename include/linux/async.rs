//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/async.h
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
// async.h: Asynchronous function calls for boot performance
//
// (C) Copyright 2009 Intel Corporation
// Author: Arjan van de Ven <arjan@linux.intel.com>
//

pub type async_cookie_t = u64;
extern "C" {
    pub fn void(data: *mut *mut async_func_t) (void, cookie: async_cookie_t) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct async_domain {
    pub pending: list_head,
    pub registered:1: unsigned,
}

//
// domain participates in global async_synchronize_full
//

//
// domain is free to go out of scope as soon as all pending work is
// complete, this domain does not participate in async_synchronize_full
//

//
// async_schedule - schedule a function for asynchronous execution
// @func: function to execute asynchronously
// @data: data pointer to pass to the function
//
// Returns an async_cookie_t that may be used for checkpointing later.
// Note: This function may be called from atomic or non-atomic contexts.
//
extern "C" {
    pub fn async_schedule_node(_arg: func, _arg: data, _arg: NUMA_NO_NODE) -> return;
}
//
// async_schedule_domain - schedule a function for asynchronous execution within a certain domain
// @func: function to execute asynchronously
// @data: data pointer to pass to the function
// @domain: the domain
//
// Returns an async_cookie_t that may be used for checkpointing later.
// @domain may be used in the async_synchronize_*_domain() functions to
// wait within a certain synchronization domain rather than globally.
// Note: This function may be called from atomic or non-atomic contexts.
//
extern "C" {
    pub fn async_schedule_node_domain(_arg: func, _arg: data, _arg: NUMA_NO_NODE, _arg: domain) -> return;
}
//
// async_schedule_dev - A device specific version of async_schedule
// @func: function to execute asynchronously
// @dev: device argument to be passed to function
//
// Returns an async_cookie_t that may be used for checkpointing later.
// @dev is used as both the argument for the function and to provide NUMA
// context for where to run the function. By doing this we can try to
// provide for the best possible outcome by operating on the device on the
// CPUs closest to the device.
// Note: This function may be called from atomic or non-atomic contexts.
//
extern "C" {
    pub fn async_schedule_node(_arg: func, _arg: dev, _arg: dev_to_node(dev)) -> return;
}
extern "C" {
    pub fn async_schedule_dev_nocall(func: async_func_t, dev: *mut device) -> bool;
}
//
// async_schedule_dev_domain - A device specific version of async_schedule_domain
// @func: function to execute asynchronously
// @dev: device argument to be passed to function
// @domain: the domain
//
// Returns an async_cookie_t that may be used for checkpointing later.
// @dev is used as both the argument for the function and to provide NUMA
// context for where to run the function. By doing this we can try to
// provide for the best possible outcome by operating on the device on the
// CPUs closest to the device.
// @domain may be used in the async_synchronize_*_domain() functions to
// wait within a certain synchronization domain rather than globally.
// Note: This function may be called from atomic or non-atomic contexts.
//
extern "C" {
    pub fn async_schedule_node_domain(_arg: func, _arg: dev, _arg: dev_to_node(dev), _arg: domain) -> return;
}
extern "C" {
    pub fn async_synchronize_full();
}
extern "C" {
    pub fn async_synchronize_full_domain(domain: *mut async_domain);
}
extern "C" {
    pub fn async_synchronize_cookie(cookie: async_cookie_t);
}
extern "C" {
    pub fn current_is_async() -> bool;
}
extern "C" {
    pub fn async_init();
}
