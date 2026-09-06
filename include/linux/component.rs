//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/component.h
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
// struct component_ops - callbacks for component drivers
//
// Components are registered with component_add() and unregistered with
// component_del().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct component_ops {
//
// @bind:
//
// Called through component_bind_all() when the aggregate driver is
// ready to bind the overall driver.
//
    pub master_data): *mut c_void,
//
// @unbind:
//
// Called through component_unbind_all() when the aggregate driver is
// ready to bind the overall driver, or when component_bind_all() fails
// part-ways through and needs to unbind some already bound components.
//
    pub master_data): *mut c_void,
}

extern "C" {
    pub fn component_add(: *mut device, : *const component_ops) -> c_int;
}
extern "C" {
    pub fn component_del(: *mut device, : *const component_ops);
}
extern "C" {
    pub fn component_bind_all(parent: *mut device, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn component_unbind_all(parent: *mut device, data: *mut c_void);
}
//
// struct component_master_ops - callback for the aggregate driver
//
// Aggregate drivers are registered with component_master_add_with_match() and
// unregistered with component_master_del().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct component_master_ops {
//
// @bind:
//
// Called when all components or the aggregate driver, as specified in
// the match list passed to component_master_add_with_match(), are
// ready. Usually there are 3 steps to bind an aggregate driver:
//
// 1. Allocate a structure for the aggregate driver.
//
// 2. Bind all components to the aggregate driver by calling
// component_bind_all() with the aggregate driver structure as opaque
// pointer data.
//
// 3. Register the aggregate driver with the subsystem to publish its
// interfaces.
//
// Note that the lifetime of the aggregate driver does not align with
// any of the underlying &struct device instances. Therefore devm cannot
// be used and all resources acquired or allocated in this callback must
// be explicitly released in the @unbind callback.
//
    pub master): *mut *mut int (bind)(struct device,
//
// @unbind:
//
// Called when either the aggregate driver, using
// component_master_del(), or one of its components, using
// component_del(), is unregistered.
//
    pub master): *mut *mut void (unbind)(struct device,
}

// A set helper functions for component compare/release
extern "C" {
    pub fn component_compare_of(dev: *mut device, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn component_release_of(dev: *mut device, data: *mut c_void);
}
extern "C" {
    pub fn component_compare_dev(dev: *mut device, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn component_compare_dev_name(dev: *mut device, data: *mut c_void) -> c_int;
}
//
// component_match_add - add a component match entry
// @parent: device with the aggregate driver
// @matchptr: pointer to the list of component matches
// @compare: compare function to match against all components
// @compare_data: opaque pointer passed to the @compare function
//
// Adds a new component match to the list stored in @matchptr, which the @parent
// aggregate driver needs to function. The list of component matches pointed to
// by @matchptr must be initialized to NULL before adding the first match. This
// only matches against components added with component_add().
//
// The allocated match list in @matchptr is automatically released using devm
// actions.
//
// See also component_match_add_release() and component_match_add_typed().
//
