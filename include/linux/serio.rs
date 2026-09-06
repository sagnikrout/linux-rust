//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/serio.h
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
// Copyright (C) 1999-2002 Vojtech Pavlik
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct serio {
    pub port_data: *mut c_void,
    pub name: [c_char; 32],
    pub phys: [c_char; 32],
    pub firmware_id: [c_char; 128],
    pub manual_bind: bool,
    pub id: serio_device_id,
// Protects critical sections from port's interrupt handler
    pub lock: spinlock_t,
    pub char): *mut *mut *mut int (write)(struct serio , unsigned,
    pub ): *mut *mut int (open)(struct serio,
    pub ): *mut *mut void (close)(struct serio,
    pub ): *mut *mut int (start)(struct serio,
    pub ): *mut *mut void (stop)(struct serio,
    pub parent: *mut serio,
// Entry in parent->children list
    pub child_node: list_head,
    pub children: list_head,
// Level of nesting in serio hierarchy
    pub depth: c_uint,
//
// serio->drv is accessed from interrupt handlers; when modifying
// caller should acquire serio->drv_mutex and serio->lock.
//
    pub drv: *mut serio_driver,
// Protects serio->drv so attributes can pin current driver
    pub drv_mutex: mutex,
    pub dev: device,
    pub node: list_head,
//
// For use by PS/2 layer when several ports share hardware and
// may get indigestion when exposed to concurrent access (i8042).
//
    pub ps2_cmd_mutex: *mut mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct serio_driver {
    pub description: *const c_char,
    pub id_table: *const serio_device_id,
    pub manual_bind: bool,
    pub ): *mut *mut void (write_wakeup)(struct serio,
    pub int): *mut *mut *mut irqreturn_t (interrupt)(struct serio , unsigned char, unsigned,
    pub drv): *mut *mut *mut int (connect)(struct serio , struct serio_driver,
    pub ): *mut *mut int (reconnect)(struct serio,
    pub ): *mut *mut int (fast_reconnect)(struct serio,
    pub ): *mut *mut void (disconnect)(struct serio,
    pub ): *mut *mut void (cleanup)(struct serio,
    pub driver: device_driver,
}

extern "C" {
    pub fn serio_open(serio: *mut serio, drv: *mut serio_driver) -> c_int;
}
extern "C" {
    pub fn serio_close(serio: *mut serio);
}
extern "C" {
    pub fn serio_rescan(serio: *mut serio);
}
extern "C" {
    pub fn serio_reconnect(serio: *mut serio);
}
extern "C" {
    pub fn serio_interrupt(serio: *mut serio, data: c_uchar, flags: c_uint) -> irqreturn_t;
}
extern "C" {
    pub fn __serio_register_port(serio: *mut serio, owner: *mut module);
}
// use a define to avoid include chaining to get THIS_MODULE

extern "C" {
    pub fn serio_unregister_port(serio: *mut serio);
}
extern "C" {
    pub fn serio_unregister_child_port(serio: *mut serio);
}
// use a define to avoid include chaining to get THIS_MODULE & friends

extern "C" {
    pub fn serio_unregister_driver(drv: *mut serio_driver);
}
//
// module_serio_driver() - Helper macro for registering a serio driver
// @__serio_driver: serio_driver struct
//
// Helper macro for serio drivers which do not do anything special in
// module init/exit. This eliminates a lot of boilerplate. Each module
// may only use this macro once, and calling it replaces module_init()
// and module_exit().
//

//
// Use the following functions to manipulate serio's per-port
// driver-specific data.
//
extern "C" {
    pub fn dev_get_drvdata(_arg: &serio->dev) -> return;
}
//
// Use the following functions to protect critical sections in
// driver code from port's interrupt handler
//
