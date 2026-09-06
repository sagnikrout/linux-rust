//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/backlight/ili9320.h
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
// drivers/video/backlight/ili9320.h
//
// ILI9320 LCD controller driver core.
//
// Copyright 2007 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
//
// http://armlinux.simtec.co.uk
//
// Holder for register and value pairs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ili9320_reg {
    pub address: c_ushort,
    pub value: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ili9320_client {
    pub name: *const c_char,
    pub cfg): *mut *mut *mut int (init)(struct ili9320 ili, struct ili9320_platdata,
}

// Device attached via an SPI bus.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ili9320_spi {
    pub dev: *mut spi_device,
    pub message: spi_message,
    pub xfer: [spi_transfer; 2],
    pub id: c_uchar,
    pub buffer_addr: [c_uchar; 4],
    pub buffer_data: [c_uchar; 4],
}

// ILI9320 device state.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ili9320 {
    pub /: *mut *mut ili9320_spi spi; / SPI attachged device.,
    pub /: *mut *mut } access; / Register access method.,
    pub dev: *mut device,
    pub /: *mut *mut *mut lcd_device lcd; / LCD device we created.,
    pub client: *mut ili9320_client,
    pub platdata: *mut ili9320_platdata,
    pub /: *mut *mut int power; / current power state.,
    pub initialised: c_int,
    pub display1: c_ushort,
    pub power1: c_ushort,
    pub val): *mut *mut *mut int (write)(struct ili9320 ili, unsigned int reg, unsigned int,
}

// ILI9320 register access routines
// Device probe
extern "C" {
    pub fn ili9320_remove(lcd: *mut ili9320);
}
extern "C" {
    pub fn ili9320_shutdown(lcd: *mut ili9320);
}
// PM
extern "C" {
    pub fn ili9320_suspend(lcd: *mut ili9320) -> c_int;
}
extern "C" {
    pub fn ili9320_resume(lcd: *mut ili9320) -> c_int;
}
