//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/peci.h
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
// Copyright (c) 2018-2021 Intel Corporation

//
// Currently we don't support any PECI command over 32 bytes.
//
pub const PECI_REQUEST_MAX_BUF_SIZE: c_int = 32;
//
// struct peci_controller_ops - PECI controller specific methods
// @xfer: PECI transfer function
//
// PECI controllers may have different hardware interfaces - the drivers
// implementing PECI controllers can use this structure to abstract away those
// differences by exposing a common interface for PECI core.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct peci_controller_ops {
    pub req): *mut *mut *mut int (xfer)(struct peci_controller controller, u8 addr, struct peci_request,
}

//
// struct peci_controller - PECI controller
// @dev: device object to register PECI controller to the device model
// @ops: pointer to device specific controller operations
// @bus_lock: lock used to protect multiple callers
// @id: PECI controller ID
//
// PECI controllers usually connect to their drivers using non-PECI bus,
// such as the platform bus.
// Each PECI controller can communicate with one or more PECI devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct peci_controller {
    pub dev: device,
    pub ops: *const peci_controller_ops,
    pub /: *mut *mut mutex bus_lock; / held for the duration of xfer,
    pub id: u8,
}

extern "C" {
    pub fn container_of(_arg: d, peci_controller: struct, _arg: dev) -> return;
}
//
// struct peci_device - PECI device
// @dev: device object to register PECI device to the device model
// @info: PECI device characteristics
// @info.x86_vfm: device vendor-family-model
// @info.peci_revision: PECI revision supported by the PECI device
// @info.socket_id: the socket ID represented by the PECI device
// @addr: address used on the PECI bus connected to the parent controller
// @deleted: indicates that PECI device was already deleted
//
// A peci_device identifies a single device (i.e. CPU) connected to a PECI bus.
// The behaviour exposed to the rest of the system is defined by the PECI driver
// managing the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct peci_device {
    pub dev: device,
    pub x86_vfm: u32,
    pub peci_revision: u8,
    pub socket_id: u8,
    pub info: },
    pub addr: u8,
    pub deleted: bool,
}

extern "C" {
    pub fn container_of(_arg: d, peci_device: struct, _arg: dev) -> return;
}
//
// struct peci_request - PECI request
// @device: PECI device to which the request is sent
// @tx: TX buffer specific data
// @tx.buf: TX buffer
// @tx.len: transfer data length in bytes
// @rx: RX buffer specific data
// @rx.buf: RX buffer
// @rx.len: received data length in bytes
//
// A peci_request represents a request issued by PECI originator (TX) and
// a response received from PECI responder (RX).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct peci_request {
    pub device: *mut peci_device,
    pub buf: [u8; PECI_REQUEST_MAX_BUF_SIZE],
    pub len: u8,
    pub tx: } rx,,
}
