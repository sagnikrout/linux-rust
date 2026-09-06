//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mailbox/riscv-rpmi-message.h
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
// Copyright (C) 2025 Ventana Micro Systems Inc.

// RPMI version encode/decode macros

// RPMI message header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmi_message_header {
    pub servicegroup_id: __le16,
    pub service_id: u8,
    pub flags: u8,
    pub datalen: __le16,
    pub token: __le16,
}

// RPMI message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmi_message {
    pub header: rpmi_message_header,
    pub data: [u8; ],
}

// RPMI notification event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmi_notification_event {
    pub event_datalen: __le16,
    pub event_id: u8,
    pub reserved: u8,
    pub event_data: [u8; ],
}

// RPMI error codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpmi_error_codes {
    RPMI_SUCCESS			= 0,
    RPMI_ERR_FAILED			= -1,
    RPMI_ERR_NOTSUPP		= -2,
    RPMI_ERR_INVALID_PARAM		= -3,
    RPMI_ERR_DENIED			= -4,
    RPMI_ERR_INVALID_ADDR		= -5,
    RPMI_ERR_ALREADY		= -6,
    RPMI_ERR_EXTENSION		= -7,
    RPMI_ERR_HW_FAULT		= -8,
    RPMI_ERR_BUSY			= -9,
    RPMI_ERR_INVALID_STATE		= -10,
    RPMI_ERR_BAD_RANGE		= -11,
    RPMI_ERR_TIMEOUT		= -12,
    RPMI_ERR_IO			= -13,
    RPMI_ERR_NO_DATA		= -14,
    RPMI_ERR_RESERVED_START		= -15,
    RPMI_ERR_RESERVED_END		= -127,
    RPMI_ERR_VENDOR_START		= -128,
}

// RPMI service group IDs
pub const RPMI_SRVGRP_SYSTEM_MSI: c_uint = 0x00002;
pub const RPMI_SRVGRP_CLOCK: c_uint = 0x00008;
// RPMI clock service IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpmi_clock_service_id {
    RPMI_CLK_SRV_ENABLE_NOTIFICATION = 0x01,
    RPMI_CLK_SRV_GET_NUM_CLOCKS = 0x02,
    RPMI_CLK_SRV_GET_ATTRIBUTES = 0x03,
    RPMI_CLK_SRV_GET_SUPPORTED_RATES = 0x04,
    RPMI_CLK_SRV_SET_CONFIG = 0x05,
    RPMI_CLK_SRV_GET_CONFIG = 0x06,
    RPMI_CLK_SRV_SET_RATE = 0x07,
    RPMI_CLK_SRV_GET_RATE = 0x08,
    RPMI_CLK_SRV_ID_MAX_COUNT
}

// RPMI system MSI service IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpmi_sysmsi_service_id {
    RPMI_SYSMSI_SRV_ENABLE_NOTIFICATION = 0x01,
    RPMI_SYSMSI_SRV_GET_ATTRIBUTES = 0x02,
    RPMI_SYSMSI_SRV_GET_MSI_ATTRIBUTES = 0x03,
    RPMI_SYSMSI_SRV_SET_MSI_STATE = 0x04,
    RPMI_SYSMSI_SRV_GET_MSI_STATE = 0x05,
    RPMI_SYSMSI_SRV_SET_MSI_TARGET = 0x06,
    RPMI_SYSMSI_SRV_GET_MSI_TARGET = 0x07,
    RPMI_SYSMSI_SRV_ID_MAX_COUNT
}

// RPMI Linux mailbox attribute IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpmi_mbox_attribute_id {
    RPMI_MBOX_ATTR_SPEC_VERSION,
    RPMI_MBOX_ATTR_MAX_MSG_DATA_SIZE,
    RPMI_MBOX_ATTR_SERVICEGROUP_ID,
    RPMI_MBOX_ATTR_SERVICEGROUP_VERSION,
    RPMI_MBOX_ATTR_IMPL_ID,
    RPMI_MBOX_ATTR_IMPL_VERSION,
    RPMI_MBOX_ATTR_MAX_ID
}

// RPMI Linux mailbox message types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpmi_mbox_message_type {
    RPMI_MBOX_MSG_TYPE_GET_ATTRIBUTE,
    RPMI_MBOX_MSG_TYPE_SET_ATTRIBUTE,
    RPMI_MBOX_MSG_TYPE_SEND_WITH_RESPONSE,
    RPMI_MBOX_MSG_TYPE_SEND_WITHOUT_RESPONSE,
    RPMI_MBOX_MSG_TYPE_NOTIFICATION_EVENT,
    RPMI_MBOX_MSG_MAX_TYPE
}

// RPMI Linux mailbox message instance
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmi_mbox_message {
    pub type: rpmi_mbox_message_type,
    pub id: rpmi_mbox_attribute_id,
    pub value: u32,
    pub attr: },
    pub service_id: u32,
    pub request: *mut c_void,
    pub request_len: c_ulong,
    pub response: *mut c_void,
    pub max_response_len: c_ulong,
    pub out_response_len: c_ulong,
    pub data: },
    pub event_datalen: u16,
    pub event_id: u8,
    pub event_data: *mut u8,
    pub notif: },
}

// RPMI Linux mailbox message helper routines
// Send message for the underlying mailbox channel
// Explicitly signal txdone for mailbox channel
