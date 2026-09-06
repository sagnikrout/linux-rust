//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/usb/tmc.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright (C) 2007 Stefan Kopp, Gechingen, Germany
// Copyright (C) 2008 Novell, Inc.
// Copyright (C) 2008 Greg Kroah-Hartman <gregkh@suse.de>
// Copyright (C) 2015 Dave Penkler <dpenkler@gmail.com>
// Copyright (C) 2018 IVI Foundation, Inc.
//
// This file holds USB constants defined by the USB Device Class
// and USB488 Subclass Definitions for Test and Measurement devices
// published by the USB-IF.
//
// It also has the ioctl and capability definitions for the
// usbtmc kernel driver that userspace needs to know about.
//

// USB TMC status values
pub const USBTMC_STATUS_SUCCESS: c_uint = 0x01;
pub const USBTMC_STATUS_PENDING: c_uint = 0x02;
pub const USBTMC_STATUS_FAILED: c_uint = 0x80;
pub const USBTMC_STATUS_TRANSFER_NOT_IN_PROGRESS: c_uint = 0x81;
pub const USBTMC_STATUS_SPLIT_NOT_IN_PROGRESS: c_uint = 0x82;
pub const USBTMC_STATUS_SPLIT_IN_PROGRESS: c_uint = 0x83;
// USB TMC requests values
pub const USBTMC_REQUEST_INITIATE_ABORT_BULK_OUT: c_int = 1;
pub const USBTMC_REQUEST_CHECK_ABORT_BULK_OUT_STATUS: c_int = 2;
pub const USBTMC_REQUEST_INITIATE_ABORT_BULK_IN: c_int = 3;
pub const USBTMC_REQUEST_CHECK_ABORT_BULK_IN_STATUS: c_int = 4;
pub const USBTMC_REQUEST_INITIATE_CLEAR: c_int = 5;
pub const USBTMC_REQUEST_CHECK_CLEAR_STATUS: c_int = 6;
pub const USBTMC_REQUEST_GET_CAPABILITIES: c_int = 7;
pub const USBTMC_REQUEST_INDICATOR_PULSE: c_int = 64;
pub const USBTMC488_REQUEST_READ_STATUS_BYTE: c_int = 128;
pub const USBTMC488_REQUEST_REN_CONTROL: c_int = 160;
pub const USBTMC488_REQUEST_GOTO_LOCAL: c_int = 161;
pub const USBTMC488_REQUEST_LOCAL_LOCKOUT: c_int = 162;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbtmc_request {
    pub bRequestType: __u8,
    pub bRequest: __u8,
    pub wValue: __u16,
    pub wIndex: __u16,
    pub wLength: __u16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbtmc_ctrlrequest {
    pub req: usbtmc_request,
    pub /: *mut *mut *mut void __user data; / pointer to user space,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbtmc_termchar {
    pub term_char: __u8,
    pub term_char_enabled: __u8,
// C attribute field omitted
//
// usbtmc_message->flags:
//
pub const USBTMC_FLAG_ASYNC: c_uint = 0x0001;
pub const USBTMC_FLAG_APPEND: c_uint = 0x0002;
pub const USBTMC_FLAG_IGNORE_TRAILER: c_uint = 0x0004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbtmc_message {
    pub /: *mut *mut __u32 transfer_size; / size of bytes to transfer,
    pub /: *mut *mut __u32 transferred; / size of received/written bytes,
    pub /: *mut *mut __u32 flags; / bit 0: 0 = synchronous; 1 = asynchronous,
    pub /: *mut *mut *mut void __user message; / pointer to header and data in user space,
// C attribute field omitted
// Request values for USBTMC driver's ioctl entry point
pub const USBTMC_IOC_NR: c_int = 91;

// Cancel and cleanup asynchronous calls

// Driver encoded usb488 capabilities
pub const USBTMC488_CAPABILITY_TRIGGER: c_int = 1;
pub const USBTMC488_CAPABILITY_SIMPLE: c_int = 2;
pub const USBTMC488_CAPABILITY_REN_CONTROL: c_int = 2;
pub const USBTMC488_CAPABILITY_GOTO_LOCAL: c_int = 2;
pub const USBTMC488_CAPABILITY_LOCAL_LOCKOUT: c_int = 2;
pub const USBTMC488_CAPABILITY_488_DOT_2: c_int = 4;
pub const USBTMC488_CAPABILITY_DT1: c_int = 16;
pub const USBTMC488_CAPABILITY_RL1: c_int = 32;
pub const USBTMC488_CAPABILITY_SR1: c_int = 64;
pub const USBTMC488_CAPABILITY_FULL_SCPI: c_int = 128;
