//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/kempld.h
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
// Kontron PLD driver definitions
//
// Copyright (c) 2010-2012 Kontron Europe GmbH
// Author: Michael Brunner <michael.brunner@kontron.com>
//
// kempld register definitions
pub const KEMPLD_IOINDEX: c_uint = 0xa80;
pub const KEMPLD_IODATA: c_uint = 0xa81;
pub const KEMPLD_MUTEX_KEY: c_uint = 0x80;
pub const KEMPLD_VERSION: c_uint = 0x00;
pub const KEMPLD_VERSION_LSB: c_uint = 0x00;
pub const KEMPLD_VERSION_MSB: c_uint = 0x01;

pub const KEMPLD_BUILDNR: c_uint = 0x02;
pub const KEMPLD_BUILDNR_LSB: c_uint = 0x02;
pub const KEMPLD_BUILDNR_MSB: c_uint = 0x03;
pub const KEMPLD_FEATURE: c_uint = 0x04;
pub const KEMPLD_FEATURE_LSB: c_uint = 0x04;
pub const KEMPLD_FEATURE_MSB: c_uint = 0x05;

pub const KEMPLD_SPEC: c_uint = 0x06;

pub const KEMPLD_IRQ_GPIO: c_uint = 0x35;
pub const KEMPLD_IRQ_GPIO_MASK: c_uint = 0x0f;
pub const KEMPLD_IRQ_I2C: c_uint = 0x36;
pub const KEMPLD_CFG: c_uint = 0x37;

pub const KEMPLD_CLK: c_int = 33333333;
pub const KEMPLD_TYPE_RELEASE: c_uint = 0x0;
pub const KEMPLD_TYPE_DEBUG: c_uint = 0x1;
pub const KEMPLD_TYPE_CUSTOM: c_uint = 0x2;
pub const KEMPLD_VERSION_LEN: c_int = 10;
//
// struct kempld_info - PLD device information structure
// @major:	PLD major revision
// @minor:	PLD minor revision
// @buildnr:	PLD build number
// @number:	PLD board specific index
// @type:	PLD type
// @spec_major:	PLD FW specification major revision
// @spec_minor:	PLD FW specification minor revision
// @version:	PLD version string
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kempld_info {
    pub major: c_uint,
    pub minor: c_uint,
    pub buildnr: c_uint,
    pub number: c_uint,
    pub type: c_uint,
    pub spec_major: c_uint,
    pub spec_minor: c_uint,
    pub version: [c_char; KEMPLD_VERSION_LEN],
}

//
// struct kempld_device_data - Internal representation of the PLD device
// @io_base:		Pointer to the IO memory
// @io_index:		Pointer to the IO index register
// @io_data:		Pointer to the IO data register
// @pld_clock:		PLD clock frequency
// @feature_mask:	PLD feature mask
// @dev:		Pointer to kernel device structure
// @info:		KEMPLD info structure
// @lock:		PLD mutex
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kempld_device_data {
    pub io_base: *mut void __iomem,
    pub io_index: *mut void __iomem,
    pub io_data: *mut void __iomem,
    pub pld_clock: u32,
    pub feature_mask: u32,
    pub dev: *mut device,
    pub info: kempld_info,
    pub lock: mutex,
}

//
// struct kempld_platform_data - PLD hardware configuration structure
// @pld_clock:			PLD clock frequency
// @gpio_base:			GPIO base pin number
// @ioresource:			IO addresses of the PLD
// @get_hardware_mutex:		PLD specific get_mutex callback
// @release_hardware_mutex:	PLD specific release_mutex callback
// @get_info:			PLD specific get_info callback
// @register_cells:		PLD specific register_cells callback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kempld_platform_data {
    pub pld_clock: u32,
    pub gpio_base: c_int,
    pub ioresource: *mut resource,
    pub ): *mut *mut void (get_hardware_mutex) (struct kempld_device_data,
    pub ): *mut *mut void (release_hardware_mutex) (struct kempld_device_data,
    pub ): *mut *mut int (get_info) (struct kempld_device_data,
    pub ): *mut *mut int (register_cells) (struct kempld_device_data,
}

extern "C" {
    pub fn kempld_get_mutex(pld: *mut kempld_device_data);
}
extern "C" {
    pub fn kempld_release_mutex(pld: *mut kempld_device_data);
}
extern "C" {
    pub fn kempld_read8(pld: *mut kempld_device_data, index: u8) -> u8;
}
extern "C" {
    pub fn kempld_write8(pld: *mut kempld_device_data, index: u8, data: u8);
}
extern "C" {
    pub fn kempld_read16(pld: *mut kempld_device_data, index: u8) -> u16;
}
extern "C" {
    pub fn kempld_write16(pld: *mut kempld_device_data, index: u8, data: u16);
}
extern "C" {
    pub fn kempld_read32(pld: *mut kempld_device_data, index: u8) -> u32;
}
extern "C" {
    pub fn kempld_write32(pld: *mut kempld_device_data, index: u8, data: u32);
}
