//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/misc/keba.h
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
// Copyright (C) 2024, KEBA Industrial Automation Gmbh

//
// struct keba_i2c_auxdev - KEBA I2C auxiliary device
// @auxdev: auxiliary device object
// @io: address range of I2C controller IO memory
// @info_size: number of I2C devices to be probed
// @info: I2C devices to be probed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct keba_i2c_auxdev {
    pub auxdev: auxiliary_device,
    pub io: resource,
    pub info_size: c_int,
    pub info: *mut i2c_board_info,
}

//
// struct keba_spi_auxdev - KEBA SPI auxiliary device
// @auxdev: auxiliary device object
// @io: address range of SPI controller IO memory
// @info_size: number of SPI devices to be probed
// @info: SPI devices to be probed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct keba_spi_auxdev {
    pub auxdev: auxiliary_device,
    pub io: resource,
    pub info_size: c_int,
    pub info: *mut spi_board_info,
}

//
// struct keba_fan_auxdev - KEBA fan auxiliary device
// @auxdev: auxiliary device object
// @io: address range of fan controller IO memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct keba_fan_auxdev {
    pub auxdev: auxiliary_device,
    pub io: resource,
}

//
// struct keba_batt_auxdev - KEBA battery auxiliary device
// @auxdev: auxiliary device object
// @io: address range of battery controller IO memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct keba_batt_auxdev {
    pub auxdev: auxiliary_device,
    pub io: resource,
}

//
// struct keba_uart_auxdev - KEBA UART auxiliary device
// @auxdev: auxiliary device object
// @io: address range of UART controller IO memory
// @irq: number of UART controller interrupt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct keba_uart_auxdev {
    pub auxdev: auxiliary_device,
    pub io: resource,
    pub irq: c_uint,
}
