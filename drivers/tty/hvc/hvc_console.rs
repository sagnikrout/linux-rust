//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/hvc/hvc_console.h
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
// hvc_console.h
// Copyright (C) 2005 IBM Corporation
//
// Author(s):
// Ryan S. Arnold <rsa@us.ibm.com>
//
// hvc_console header information:
// moved here from arch/powerpc/include/asm/hvconsole.h
// and drivers/char/hvc_console.c
//

//
// This is the max number of console adapters that can/will be found as
// console devices on first stage console init.  Any number beyond this range
// can't be used as a console device but is still a valid tty device.
//
pub const MAX_NR_HVC_CONSOLES: c_int = 16;
//
// The Linux TTY code does not support dynamic addition of tty derived devices
// so we need to know how many tty devices we might need when space is allocated
// for the tty device.  Since this driver supports hotplug of vty adapters we
// need to make sure we have enough allocated.
//
pub const HVC_ALLOC_TTY_ADAPTERS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvc_struct {
    pub port: tty_port,
    pub lock: spinlock_t,
    pub index: c_int,
    pub do_wakeup: c_int,
    pub outbuf_size: c_int,
    pub n_outbuf: c_int,
    pub vtermno: u32,
    pub ops: *const hv_ops,
    pub irq_requested: c_int,
    pub data: c_int,
    pub ws: winsize,
    pub tty_resize: work_struct,
    pub next: list_head,
    pub flags: c_ulong,
    pub __aligned(sizeof(long)): u8 outbuf[],
}

// implemented by a low level driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_ops {
    pub count): *mut *mut *mut ssize_t (get_chars)(uint32_t vtermno, u8 buf, size_t,
    pub count): *const *const *const ssize_t (put_chars)(uint32_t vtermno, u8 buf, size_t,
    pub wait): *mut *mut int (flush)(uint32_t vtermno, bool,
// Callbacks for notification. Called in open, close and hangup
    pub irq): *mut *mut *mut int (notifier_add)(struct hvc_struct hp, int,
    pub irq): *mut *mut *mut void (notifier_del)(struct hvc_struct hp, int,
    pub irq): *mut *mut *mut void (notifier_hangup)(struct hvc_struct hp, int,
// tiocmget/set implementation
    pub hp): *mut *mut int (tiocmget)(struct hvc_struct,
    pub clear): *mut *mut *mut int (tiocmset)(struct hvc_struct hp, unsigned int set, unsigned int,
// Callbacks to handle tty ports
    pub active): *mut *mut *mut void (dtr_rts)(struct hvc_struct hp, bool,
}

// Register a vterm and a slot index for use as a console (console_init)
// register a vterm for hvc tty operation (module_init or hotplug add)
// remove a vterm from hvc tty operation (module_exit or hotplug remove)
extern "C" {
    pub fn hvc_remove(hp: *mut hvc_struct);
}
// data available
extern "C" {
    pub fn hvc_poll(hp: *mut hvc_struct) -> c_int;
}
extern "C" {
    pub fn hvc_kick();
}
// Resize hvc tty terminal window
extern "C" {
    pub fn __hvc_resize(hp: *mut hvc_struct, ws: winsize);
}
// default notifier for irq based notification
extern "C" {
    pub fn notifier_add_irq(hp: *mut hvc_struct, data: c_int) -> c_int;
}
extern "C" {
    pub fn notifier_del_irq(hp: *mut hvc_struct, data: c_int);
}
extern "C" {
    pub fn notifier_hangup_irq(hp: *mut hvc_struct, data: c_int);
}

