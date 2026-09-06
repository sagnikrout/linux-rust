//! Automatically rewritten from C to Rust
//! Source: drivers/misc/c2port/c2port-duramar2150.c
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
// Silicon Labs C2 port Linux support for Eurotech Duramar 2150
//
// Copyright (c) 2008 Rodolfo Giometti <giometti@linux.it>
// Copyright (c) 2008 Eurotech S.p.A. <info@eurotech.it>
//

pub const DATA_PORT: c_uint = 0x325;
pub const DIR_PORT: c_uint = 0x326;

    static DEFINE_MUTEX(update_lock);
//
// C2 port operations
//
#[no_mangle]
unsafe extern "C" fn duramar2150_c2port_access(dev: *mut c2port_device, status: c_int) {
    static void duramar2150_c2port_access(struct c2port_device *dev, int status)
    {
    u8 v;
    mutex_lock(&update_lock);
    v = inb(DIR_PORT);
// 0 = input, 1 = output
    if (status)
    outb(v | (C2D | C2CK), DIR_PORT);
    else
// When access is "off" is important that both lines are set
// as inputs or hi-impedance
    outb(v & ~(C2D | C2CK), DIR_PORT);
    mutex_unlock(&update_lock);
    }
#[no_mangle]
unsafe extern "C" fn duramar2150_c2port_c2d_dir(dev: *mut c2port_device, dir: c_int) {
    static void duramar2150_c2port_c2d_dir(struct c2port_device *dev, int dir)
    {
    u8 v;
    mutex_lock(&update_lock);
    v = inb(DIR_PORT);
    if (dir)
    outb(v & ~C2D, DIR_PORT);
    else
    outb(v | C2D, DIR_PORT);
    mutex_unlock(&update_lock);
    }
#[no_mangle]
unsafe extern "C" fn duramar2150_c2port_c2d_get(dev: *mut c2port_device) -> c_int {
    static int duramar2150_c2port_c2d_get(struct c2port_device *dev)
    {
    return inb(DATA_PORT) & C2D;
    }
#[no_mangle]
unsafe extern "C" fn duramar2150_c2port_c2d_set(dev: *mut c2port_device, status: c_int) {
    static void duramar2150_c2port_c2d_set(struct c2port_device *dev, int status)
    {
    u8 v;
    mutex_lock(&update_lock);
    v = inb(DATA_PORT);
    if (status)
    outb(v | C2D, DATA_PORT);
    else
    outb(v & ~C2D, DATA_PORT);
    mutex_unlock(&update_lock);
    }
#[no_mangle]
unsafe extern "C" fn duramar2150_c2port_c2ck_set(dev: *mut c2port_device, status: c_int) {
    static void duramar2150_c2port_c2ck_set(struct c2port_device *dev, int status)
    {
    u8 v;
    mutex_lock(&update_lock);
    v = inb(DATA_PORT);
    if (status)
    outb(v | C2CK, DATA_PORT);
    else
    outb(v & ~C2CK, DATA_PORT);
    mutex_unlock(&update_lock);
    }
    static struct c2port_ops duramar2150_c2port_ops = {
    .block_size	= 512,	/* bytes */
    .blocks_num	= 30,	/* total flash size: 15360 bytes */
    .access		= duramar2150_c2port_access,
    .c2d_dir	= duramar2150_c2port_c2d_dir,
    .c2d_get	= duramar2150_c2port_c2d_get,
    .c2d_set	= duramar2150_c2port_c2d_set,
    .c2ck_set	= duramar2150_c2port_c2ck_set,
    };
    static struct c2port_device *duramar2150_c2port_dev;
//
// Module stuff
//
#[no_mangle]
unsafe extern "C" fn duramar2150_c2port_init() -> int __init {
    static int __init duramar2150_c2port_init(void)
    {
    struct resource *res;
    let mut ret: c_int = 0;
    res = request_region(0x325, 2, "c2port");
    if (!res)
    return -EBUSY;
    duramar2150_c2port_dev = c2port_device_register("uc",
    &duramar2150_c2port_ops, core::ptr::null_mut());
    if (IS_ERR(duramar2150_c2port_dev)) {
    ret = PTR_ERR(duramar2150_c2port_dev);
    goto free_region;
    }
    return 0;
    free_region:
    release_region(0x325, 2);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn duramar2150_c2port_exit() -> void __exit {
    static void __exit duramar2150_c2port_exit(void)
    {
// Setup the GPIOs as input by default (access = 0)
    duramar2150_c2port_access(duramar2150_c2port_dev, 0);
    c2port_device_unregister(duramar2150_c2port_dev);
    release_region(0x325, 2);
    }
    module_init(duramar2150_c2port_init);
    module_exit(duramar2150_c2port_exit);
    MODULE_AUTHOR("Rodolfo Giometti <giometti@linux.it>");
    MODULE_DESCRIPTION("Silicon Labs C2 port Linux support for Duramar 2150");
    MODULE_LICENSE("GPL");
