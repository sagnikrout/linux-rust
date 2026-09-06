//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/gameport.h
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
// Copyright (c) 1999-2002 Vojtech Pavlik
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gameport {
    pub /: *mut *mut *mut void port_data; / Private pointer for gameport drivers,
    pub name: [c_char; 32],
    pub phys: [c_char; 32],
    pub io: c_int,
    pub speed: c_int,
    pub fuzz: c_int,
    pub ): *mut *mut void (trigger)(struct gameport,
    pub ): *mut *mut unsigned char (read)(struct gameport,
    pub ): *mut *mut *mut *mut int (cooked_read)(struct gameport , int , int,
    pub ): *mut *mut *mut *mut int (calibrate)(struct gameport , int , int,
    pub int): *mut *mut *mut int (open)(struct gameport ,,
    pub ): *mut *mut void (close)(struct gameport,
    pub poll_timer: timer_list,
    pub /: *mut *mut unsigned int poll_interval; / in msecs,
    pub timer_lock: spinlock_t,
    pub poll_cnt: c_uint,
    pub ): *mut *mut void (poll_handler)(struct gameport,
    pub child: *mut *mut gameport parent,,
    pub drv: *mut gameport_driver,
    pub /: *mut *mut mutex drv_mutex; / protects serio->drv so attributes can pin driver,
    pub dev: device,
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gameport_driver {
    pub description: *const c_char,
    pub drv): *mut *mut *mut int (connect)(struct gameport , struct gameport_driver,
    pub ): *mut *mut int (reconnect)(struct gameport,
    pub ): *mut *mut void (disconnect)(struct gameport,
    pub driver: device_driver,
    pub ignore: bool,
}

extern "C" {
    pub fn gameport_open(gameport: *mut gameport, drv: *mut gameport_driver, mode: c_int) -> c_int;
}
extern "C" {
    pub fn gameport_close(gameport: *mut gameport);
}

extern "C" {
    pub fn __gameport_register_port(gameport: *mut gameport, owner: *mut module);
}
// use a define to avoid include chaining to get THIS_MODULE

extern "C" {
    pub fn gameport_unregister_port(gameport: *mut gameport);
}
extern "C" {
    pub fn gameport_set_phys(gameport: *mut gameport, fmt: *const c_char, ...);
}

//
// Use the following functions to manipulate gameport's per-port
// driver-specific data.
//
extern "C" {
    pub fn dev_get_drvdata(_arg: &gameport->dev) -> return;
}
//
// Use the following functions to pin gameport's driver in process context
//
extern "C" {
    pub fn mutex_lock_interruptible(_arg: &gameport->drv_mutex) -> return;
}
// use a define to avoid include chaining to get THIS_MODULE & friends

extern "C" {
    pub fn gameport_unregister_driver(drv: *mut gameport_driver);
}
//
// module_gameport_driver() - Helper macro for registering a gameport driver
// @__gameport_driver: gameport_driver struct
//
// Helper macro for gameport drivers which do not do anything special in
// module init/exit. This eliminates a lot of boilerplate. Each module may
// only use this macro once, and calling it replaces module_init() and
// module_exit().
//

extern "C" {
    pub fn gameport_start_polling(gameport: *mut gameport);
}
extern "C" {
    pub fn gameport_stop_polling(gameport: *mut gameport);
}
