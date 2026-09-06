//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tifm.h
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
// tifm.h - TI FlashMedia driver
//
// Copyright (C) 2006 Alex Dubov <oakad@yahoo.com>
//

// Host registers (relative to pci base address):
// Socket registers (relative to socket base address):
pub const TIFM_CTRL_LED: c_uint = 0x00000040;
pub const TIFM_CTRL_FAST_CLK: c_uint = 0x00000100;
pub const TIFM_CTRL_POWER_MASK: c_uint = 0x00000007;
pub const TIFM_SOCK_STATE_OCCUPIED: c_uint = 0x00000008;
pub const TIFM_SOCK_STATE_POWERED: c_uint = 0x00000080;
pub const TIFM_FIFO_ENABLE: c_uint = 0x00000001;
pub const TIFM_FIFO_READY: c_uint = 0x00000001;
pub const TIFM_FIFO_MORE: c_uint = 0x00000008;
pub const TIFM_FIFO_INT_SETALL: c_uint = 0x0000ffff;
pub const TIFM_FIFO_INTMASK: c_uint = 0x00000005;
pub const TIFM_DMA_RESET: c_uint = 0x00000002;
pub const TIFM_DMA_TX: c_uint = 0x00008000;
pub const TIFM_DMA_EN: c_uint = 0x00000001;
pub const TIFM_DMA_TSIZE: c_uint = 0x0000007f;
pub const TIFM_TYPE_XD: c_int = 1;
pub const TIFM_TYPE_MS: c_int = 2;
pub const TIFM_TYPE_SD: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tifm_device_id {
    pub type: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tifm_dev {
    pub addr: *mut char __iomem,
    pub lock: spinlock_t,
    pub type: c_uchar,
    pub socket_id: c_uint,
    pub sock): *mut *mut void (card_event)(struct tifm_dev,
    pub sock): *mut *mut void (data_event)(struct tifm_dev,
    pub dev: device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tifm_driver {
    pub id_table: *const tifm_device_id,
    pub dev): *mut *mut int (probe)(struct tifm_dev,
    pub dev): *mut *mut void (remove)(struct tifm_dev,
    pub state): pm_message_t,
    pub dev): *mut *mut int (resume)(struct tifm_dev,
    pub driver: device_driver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tifm_adapter {
    pub addr: *mut char __iomem,
    pub lock: spinlock_t,
    pub irq_status: c_uint,
    pub socket_change_set: c_uint,
    pub id: c_uint,
    pub num_sockets: c_uint,
    pub finish_me: *mut completion,
    pub media_switcher: work_struct,
    pub dev: device,
    pub sock): *mut tifm_dev,
    pub sock): *mut tifm_dev,
    pub sockets: [*mut tifm_dev; ],
}

extern "C" {
    pub fn tifm_add_adapter(fm: *mut tifm_adapter) -> c_int;
}
extern "C" {
    pub fn tifm_remove_adapter(fm: *mut tifm_adapter);
}
extern "C" {
    pub fn tifm_free_adapter(fm: *mut tifm_adapter);
}
extern "C" {
    pub fn tifm_free_device(dev: *mut device);
}
extern "C" {
    pub fn tifm_register_driver(drv: *mut tifm_driver) -> c_int;
}
extern "C" {
    pub fn tifm_unregister_driver(drv: *mut tifm_driver);
}
extern "C" {
    pub fn tifm_eject(sock: *mut tifm_dev);
}
extern "C" {
    pub fn tifm_has_ms_pif(sock: *mut tifm_dev) -> c_int;
}
extern "C" {
    pub fn tifm_queue_work(work: *mut work_struct);
}
extern "C" {
    pub fn dev_get_drvdata(_arg: &dev->dev) -> return;
}
