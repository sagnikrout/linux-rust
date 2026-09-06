//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-acorn.c
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
// ARM IOC/IOMD i2c driver.
//
// Copyright (C) 2000 Russell King
//
// On Acorn machines, the following i2c devices are on the bus:
// - PCF8583 real time clock & static RAM
//

pub const FORCE_ONES: c_uint = 0xdc;
pub const SCL: c_uint = 0x02;
pub const SDA: c_uint = 0x01;
//
// We must preserve all non-i2c output bits in IOC_CONTROL.
// Note also that we need to preserve the value of SCL and
// SDA outputs as well (which may be different from the
// values read back from IOC_CONTROL).
//
    static u_int force_ones;
#[no_mangle]
unsafe extern "C" fn ioc_setscl(data: *mut c_void, state: c_int) {
    static void ioc_setscl(void *data, int state)
    {
    let mut ioc_control: u_int = ioc_readb(IOC_CONTROL) & ~(SCL | SDA);
    let mut ones: u_int = force_ones;
    if (state)
    ones |= SCL;
    else
    ones &= ~SCL;
    force_ones = ones;
    ioc_writeb(ioc_control | ones, IOC_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn ioc_setsda(data: *mut c_void, state: c_int) {
    static void ioc_setsda(void *data, int state)
    {
    let mut ioc_control: u_int = ioc_readb(IOC_CONTROL) & ~(SCL | SDA);
    let mut ones: u_int = force_ones;
    if (state)
    ones |= SDA;
    else
    ones &= ~SDA;
    force_ones = ones;
    ioc_writeb(ioc_control | ones, IOC_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn ioc_getscl(data: *mut c_void) -> c_int {
    static int ioc_getscl(void *data)
    {
    return (ioc_readb(IOC_CONTROL) & SCL) != 0;
    }
#[no_mangle]
unsafe extern "C" fn ioc_getsda(data: *mut c_void) -> c_int {
    static int ioc_getsda(void *data)
    {
    return (ioc_readb(IOC_CONTROL) & SDA) != 0;
    }
    static struct i2c_algo_bit_data ioc_data = {
    .setsda		= ioc_setsda,
    .setscl		= ioc_setscl,
    .getsda		= ioc_getsda,
    .getscl		= ioc_getscl,
    .udelay		= 80,
    .timeout	= HZ,
    };
    static struct i2c_adapter ioc_ops = {
    .nr			= 0,
    .name			= "ioc",
    .algo_data		= &ioc_data,
    };
#[no_mangle]
unsafe extern "C" fn i2c_ioc_init() -> int __init {
    static int __init i2c_ioc_init(void)
    {
    force_ones = FORCE_ONES | SCL | SDA;
    return i2c_bit_add_numbered_bus(&ioc_ops);
    }
    module_init(i2c_ioc_init);
    MODULE_AUTHOR("Russell King <linux@armlinux.org.uk>");
    MODULE_DESCRIPTION("ARM IOC/IOMD i2c driver");
    MODULE_LICENSE("GPL v2");
