//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/nct6694.h
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
// Copyright (C) 2025 Nuvoton Technology Corp.
//
// Nuvoton NCT6694 USB transaction and data structure.
//
pub const NCT6694_VENDOR_ID: c_uint = 0x0416;
pub const NCT6694_PRODUCT_ID: c_uint = 0x200B;
pub const NCT6694_INT_IN_EP: c_uint = 0x81;
pub const NCT6694_BULK_IN_EP: c_uint = 0x02;
pub const NCT6694_BULK_OUT_EP: c_uint = 0x03;
pub const NCT6694_HCTRL_SET: c_uint = 0x40;
pub const NCT6694_HCTRL_GET: c_uint = 0x80;
pub const NCT6694_URB_TIMEOUT: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nct6694_irq_id {
    NCT6694_IRQ_GPIO0 = 0,
    NCT6694_IRQ_GPIO1,
    NCT6694_IRQ_GPIO2,
    NCT6694_IRQ_GPIO3,
    NCT6694_IRQ_GPIO4,
    NCT6694_IRQ_GPIO5,
    NCT6694_IRQ_GPIO6,
    NCT6694_IRQ_GPIO7,
    NCT6694_IRQ_GPIO8,
    NCT6694_IRQ_GPIO9,
    NCT6694_IRQ_GPIOA,
    NCT6694_IRQ_GPIOB,
    NCT6694_IRQ_GPIOC,
    NCT6694_IRQ_GPIOD,
    NCT6694_IRQ_GPIOE,
    NCT6694_IRQ_GPIOF,
    NCT6694_IRQ_CAN0,
    NCT6694_IRQ_CAN1,
    NCT6694_IRQ_RTC,
    NCT6694_NR_IRQS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nct6694_response_err_status {
    NCT6694_NO_ERROR = 0,
    NCT6694_FORMAT_ERROR,
    NCT6694_RESERVED1,
    NCT6694_RESERVED2,
    NCT6694_NOT_SUPPORT_ERROR,
    NCT6694_NO_RESPONSE_ERROR,
    NCT6694_TIMEOUT_ERROR,
    NCT6694_PENDING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union __packed {
    pub offset: __le16,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub cmd: u8,
    pub sel: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nct6694 {
    pub dev: *mut device,
    pub gpio_ida: ida,
    pub i2c_ida: ida,
    pub canfd_ida: ida,
    pub wdt_ida: ida,
    pub domain: *mut irq_domain,
    pub access_lock: mutex,
    pub irq_lock: spinlock_t,
    pub int_in_urb: *mut urb,
    pub udev: *mut usb_device,
    pub usb_msg: *mut nct6694_usb_msg,
    pub int_buffer: *mut __le32,
    pub irq_enable: c_uint,
}

extern "C" {
    pub fn nct6694_read_msg(nct6694: *mut nct6694, cmd_hd: *const nct6694_cmd_header, buf: *mut c_void) -> c_int;
}
extern "C" {
    pub fn nct6694_write_msg(nct6694: *mut nct6694, cmd_hd: *const nct6694_cmd_header, buf: *mut c_void) -> c_int;
}
