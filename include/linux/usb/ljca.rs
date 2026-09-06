//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/ljca.h
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
// Copyright (c) 2023, Intel Corporation. All rights reserved.
//

pub const LJCA_MAX_GPIO_NUM: c_int = 64;

//
// typedef ljca_event_cb_t - event callback function signature
//
// @context: the execution context of who registered this callback
// @cmd: the command from device for this event
// @evt_data: the event data payload
// @len: the event data payload length
//
// The callback function is called in interrupt context and the data payload is
// only valid during the call. If the user needs later access of the data, it
// must copy it.
//
extern "C" {
    pub fn void(context: *mut *mut ljca_event_cb_t)(void, cmd: u8, evt_data: *const c_void, len: c_int) -> typedef;
}
//
// struct ljca_client - represent a ljca client device
//
// @type: ljca client type
// @id: ljca client id within same client type
// @link: ljca client on the same ljca adapter
// @auxdev: auxiliary device object
// @adapter: ljca adapter the ljca client sit on
// @context: the execution context of the event callback
// @event_cb: ljca client driver register this callback to get
// firmware asynchronous rx buffer pending notifications
// @event_cb_lock: spinlock to protect event callback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ljca_client {
    pub type: u8,
    pub id: u8,
    pub link: list_head,
    pub auxdev: auxiliary_device,
    pub adapter: *mut ljca_adapter,
    pub context: *mut c_void,
    pub event_cb: ljca_event_cb_t,
// lock to protect event_cb
    pub event_cb_lock: spinlock_t,
}

//
// struct ljca_gpio_info - ljca gpio client device info
//
// @num: ljca gpio client device pin number
// @valid_pin_map: ljca gpio client device valid pin mapping
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ljca_gpio_info {
    pub num: c_uint,
    pub LJCA_MAX_GPIO_NUM): DECLARE_BITMAP(valid_pin_map,,
}

//
// struct ljca_i2c_info - ljca i2c client device info
//
// @id: ljca i2c client device identification number
// @capacity: ljca i2c client device capacity
// @intr_pin: ljca i2c client device interrupt pin number if exists
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ljca_i2c_info {
    pub id: u8,
    pub capacity: u8,
    pub intr_pin: u8,
}

//
// struct ljca_spi_info - ljca spi client device info
//
// @id: ljca spi client device identification number
// @capacity: ljca spi client device capacity
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ljca_spi_info {
    pub id: u8,
    pub capacity: u8,
}

//
// ljca_register_event_cb - register a callback function to receive events
//
// @client: ljca client device
// @event_cb: callback function
// @context: execution context of event callback
//
// Return: 0 in case of success, negative value in case of error
//
extern "C" {
    pub fn ljca_register_event_cb(client: *mut ljca_client, event_cb: ljca_event_cb_t, context: *mut c_void) -> c_int;
}
//
// ljca_unregister_event_cb - unregister the callback function for an event
//
// @client: ljca client device
//
extern "C" {
    pub fn ljca_unregister_event_cb(client: *mut ljca_client);
}
//
// ljca_transfer - issue a LJCA command and wait for a response
//
// @client: ljca client device
// @cmd: the command to be sent to the device
// @obuf: the buffer to be sent to the device; it can be NULL if the user
// doesn't need to transmit data with this command
// @obuf_len: the size of the buffer to be sent to the device; it should
// be 0 when obuf is NULL
// @ibuf: any data associated with the response will be copied here; it can be
// NULL if the user doesn't need the response data
// @ibuf_len: must be initialized to the input buffer size
//
// Return: the actual length of response data for success, negative value for errors
//
// ljca_transfer_noack - issue a LJCA command without a response
//
// @client: ljca client device
// @cmd: the command to be sent to the device
// @obuf: the buffer to be sent to the device; it can be NULL if the user
// doesn't need to transmit data with this command
// @obuf_len: the size of the buffer to be sent to the device
//
// Return: 0 for success, negative value for errors
//
