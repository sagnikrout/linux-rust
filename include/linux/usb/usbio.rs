//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/usbio.h
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
// Copyright (c) 2025 Intel Corporation.
//

//
// USBIO Clients Names
//

//
// USBIO quirks
//

//
// USBIO Type Definitions
//
// USBIO Packet Type
pub const USBIO_PKTTYPE_CTRL: c_int = 1;
pub const USBIO_PKTTYPE_DBG: c_int = 2;
pub const USBIO_PKTTYPE_GPIO: c_int = 3;
pub const USBIO_PKTTYPE_I2C: c_int = 4;
// USBIO Packet Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_packet_header {
    pub type: u8,
    pub cmd: u8,
    pub flags: u8,
    pub __packed: },
// USBIO Control Transfer Packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_ctrl_packet {
    pub header: usbio_packet_header,
    pub len: u8,
    pub __counted_by(len): u8 data[],
    pub __packed: },
// USBIO Bulk Transfer Packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_bulk_packet {
    pub header: usbio_packet_header,
    pub len: __le16,
    pub __counted_by(len): u8 data[],
    pub __packed: },
// USBIO GPIO commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usbio_gpio_cmd {
    USBIO_GPIOCMD_DEINIT,
    USBIO_GPIOCMD_INIT,
    USBIO_GPIOCMD_READ,
    USBIO_GPIOCMD_WRITE,
    USBIO_GPIOCMD_END
}

// USBIO GPIO config
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usbio_gpio_pincfg {
    USBIO_GPIO_PINCFG_DEFAULT,
    USBIO_GPIO_PINCFG_PULLUP,
    USBIO_GPIO_PINCFG_PULLDOWN,
    USBIO_GPIO_PINCFG_PUSHPULL
}

pub const USBIO_GPIO_PINCFG_SHIFT: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usbio_gpio_pinmode {
    USBIO_GPIO_PINMOD_INVAL,
    USBIO_GPIO_PINMOD_INPUT,
    USBIO_GPIO_PINMOD_OUTPUT,
    USBIO_GPIO_PINMOD_MAXVAL
}

pub const USBIO_GPIO_PINMOD_MASK: c_uint = 0x3;

//
// USBIO GPIO Controller
//
pub const USBIO_MAX_GPIOBANKS: c_int = 5;
pub const USBIO_GPIOSPERBANK: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_gpio_bank_desc {
    pub id: u8,
    pub pins: u8,
    pub bmap: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_gpio_init {
    pub bankid: u8,
    pub config: u8,
    pub pincount: u8,
    pub pin: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_gpio_rw {
    pub bankid: u8,
    pub pincount: u8,
    pub pin: u8,
    pub value: __le32,
    pub __packed: },
// USBIO I2C commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usbio_i2c_cmd {
    USBIO_I2CCMD_UNINIT,
    USBIO_I2CCMD_INIT,
    USBIO_I2CCMD_READ,
    USBIO_I2CCMD_WRITE,
    USBIO_I2CCMD_END
}

//
// USBIO I2C Controller
//
pub const USBIO_MAX_I2CBUSES: c_int = 5;

pub const USBIO_I2C_BUS_MODE_CAP_MASK: c_uint = 0x3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_i2c_bus_desc {
    pub id: u8,
    pub caps: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_i2c_uninit {
    pub busid: u8,
    pub config: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_i2c_init {
    pub busid: u8,
    pub config: __le16,
    pub speed: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_i2c_rw {
    pub busid: u8,
    pub config: __le16,
    pub size: __le16,
    pub __counted_by(size): u8 data[],
    pub __packed: },
    pub ibuf_len): *const *const *const void obuf, u16 obuf_len, void ibuf, u16,
    pub ibuf_len): *const *const *const void obuf, u16 obuf_len, void ibuf, u16,
    pub adev): *mut int usbio_acquire(struct auxiliary_device,
    pub adev): *mut void usbio_release(struct auxiliary_device,
    pub rxbuf_len): *mut *mut *mut void usbio_get_txrxbuf_len(struct auxiliary_device adev, u16 txbuf_len, u16,
    pub adev): *mut unsigned long usbio_get_quirks(struct auxiliary_device,
    pub hids): *const *const void usbio_acpi_bind(struct auxiliary_device adev, struct acpi_device_id,
