//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwspinlock/hwspinlock_internal.h
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
// Hardware spinlocks internal header
//
// Copyright (C) 2010 Texas Instruments Incorporated - http://www.ti.com
//
// Contact: Ohad Ben-Cohen <ohad@wizery.com>
//

//
// struct hwspinlock_ops - platform-specific hwspinlock handlers
//
// @trylock: make a single attempt to take the lock. returns 0 on
// failure and true on success. may _not_ sleep.
// @unlock:  release the lock. always succeed. may _not_ sleep.
// @bust:    optional, platform-specific bust handler, called by hwspinlock
// core to bust a specific lock.
// @relax:   optional, platform-specific relax handler, called by hwspinlock
// core while spinning on a lock, between two successive
// invocations of @trylock. may _not_ sleep.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwspinlock_ops {
    pub lock): *mut *mut int (trylock)(struct hwspinlock,
    pub lock): *mut *mut void (unlock)(struct hwspinlock,
    pub id): *mut *mut *mut int (bust)(struct hwspinlock lock, unsigned int,
    pub lock): *mut *mut void (relax)(struct hwspinlock,
}

//
// struct hwspinlock - this struct represents a single hwspinlock instance
// @bank: the hwspinlock_device structure which owns this lock
// @lock: initialized and used by hwspinlock core
// @priv: private data, owned by the underlying platform-specific hwspinlock drv
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwspinlock {
    pub bank: *mut hwspinlock_device,
    pub lock: spinlock_t,
    pub priv: *mut c_void,
}

//
// struct hwspinlock_device - a device which usually spans numerous hwspinlocks
// @dev: underlying device, will be used to invoke runtime PM api
// @ops: platform-specific hwspinlock handlers
// @base_id: id index of the first lock in this device
// @num_locks: number of locks in this device
// @lock: dynamically allocated array of 'struct hwspinlock'
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwspinlock_device {
    pub dev: *mut device,
    pub ops: *const hwspinlock_ops,
    pub base_id: c_int,
    pub num_locks: c_int,
    pub lock: [hwspinlock; ],
}
