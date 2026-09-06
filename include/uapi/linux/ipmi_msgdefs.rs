//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ipmi_msgdefs.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// ipmi_smi.h
//
// MontaVista IPMI system management interface
//
// Author: MontaVista Software, Inc.
// Corey Minyard <minyard@mvista.com>
// source@mvista.com
//
// Copyright 2002 MontaVista Software Inc.
//
// Various definitions for IPMI messages used by almost everything in
// NetFNs and commands used inside the IPMI stack.
pub const IPMI_NETFN_SENSOR_EVENT_REQUEST: c_uint = 0x04;
pub const IPMI_NETFN_SENSOR_EVENT_RESPONSE: c_uint = 0x05;
pub const IPMI_GET_EVENT_RECEIVER_CMD: c_uint = 0x01;
pub const IPMI_NETFN_APP_REQUEST: c_uint = 0x06;
pub const IPMI_NETFN_APP_RESPONSE: c_uint = 0x07;
pub const IPMI_GET_DEVICE_ID_CMD: c_uint = 0x01;
pub const IPMI_COLD_RESET_CMD: c_uint = 0x02;
pub const IPMI_WARM_RESET_CMD: c_uint = 0x03;
pub const IPMI_CLEAR_MSG_FLAGS_CMD: c_uint = 0x30;
pub const IPMI_GET_DEVICE_GUID_CMD: c_uint = 0x08;
pub const IPMI_GET_MSG_FLAGS_CMD: c_uint = 0x31;
pub const IPMI_SEND_MSG_CMD: c_uint = 0x34;
pub const IPMI_GET_MSG_CMD: c_uint = 0x33;
pub const IPMI_SET_BMC_GLOBAL_ENABLES_CMD: c_uint = 0x2e;
pub const IPMI_GET_BMC_GLOBAL_ENABLES_CMD: c_uint = 0x2f;
pub const IPMI_READ_EVENT_MSG_BUFFER_CMD: c_uint = 0x35;
pub const IPMI_GET_CHANNEL_INFO_CMD: c_uint = 0x42;
// Bit for BMC global enables.
pub const IPMI_BMC_RCV_MSG_INTR: c_uint = 0x01;
pub const IPMI_BMC_EVT_MSG_INTR: c_uint = 0x02;
pub const IPMI_BMC_EVT_MSG_BUFF: c_uint = 0x04;
pub const IPMI_BMC_SYS_LOG: c_uint = 0x08;
pub const IPMI_NETFN_STORAGE_REQUEST: c_uint = 0x0a;
pub const IPMI_NETFN_STORAGE_RESPONSE: c_uint = 0x0b;
pub const IPMI_ADD_SEL_ENTRY_CMD: c_uint = 0x44;
pub const IPMI_NETFN_FIRMWARE_REQUEST: c_uint = 0x08;
pub const IPMI_NETFN_FIRMWARE_RESPONSE: c_uint = 0x09;
// The default slave address
pub const IPMI_BMC_SLAVE_ADDR: c_uint = 0x20;
// The BT interface on high-end HP systems supports up to 255 bytes in
// one transfer.  Its "virtual" BMC supports some commands that are longer
// than 128 bytes.  Use the full 256, plus NetFn/LUN, Cmd, cCode, plus
// some overhead; it's not worth the effort to dynamically size this based
// on the results of the "Get BT Capabilities" command.

pub const IPMI_CC_NO_ERROR: c_uint = 0x00;
pub const IPMI_NODE_BUSY_ERR: c_uint = 0xc0;
pub const IPMI_INVALID_COMMAND_ERR: c_uint = 0xc1;
pub const IPMI_TIMEOUT_ERR: c_uint = 0xc3;
pub const IPMI_ERR_MSG_TRUNCATED: c_uint = 0xc6;
pub const IPMI_REQ_LEN_INVALID_ERR: c_uint = 0xc7;
pub const IPMI_REQ_LEN_EXCEEDED_ERR: c_uint = 0xc8;
pub const IPMI_DEVICE_IN_FW_UPDATE_ERR: c_uint = 0xd1;
pub const IPMI_DEVICE_IN_INIT_ERR: c_uint = 0xd2;
pub const IPMI_NOT_IN_MY_STATE_ERR: c_uint = 0xd5	/* IPMI 2.0 */;
pub const IPMI_LOST_ARBITRATION_ERR: c_uint = 0x81;
pub const IPMI_BUS_ERR: c_uint = 0x82;
pub const IPMI_NAK_ON_WRITE_ERR: c_uint = 0x83;
pub const IPMI_ERR_UNSPECIFIED: c_uint = 0xff;
pub const IPMI_CHANNEL_PROTOCOL_IPMB: c_int = 1;
pub const IPMI_CHANNEL_PROTOCOL_ICMB: c_int = 2;
pub const IPMI_CHANNEL_PROTOCOL_SMBUS: c_int = 4;
pub const IPMI_CHANNEL_PROTOCOL_KCS: c_int = 5;
pub const IPMI_CHANNEL_PROTOCOL_SMIC: c_int = 6;
pub const IPMI_CHANNEL_PROTOCOL_BT10: c_int = 7;
pub const IPMI_CHANNEL_PROTOCOL_BT15: c_int = 8;
pub const IPMI_CHANNEL_PROTOCOL_TMODE: c_int = 9;
pub const IPMI_CHANNEL_MEDIUM_IPMB: c_int = 1;
pub const IPMI_CHANNEL_MEDIUM_ICMB10: c_int = 2;
pub const IPMI_CHANNEL_MEDIUM_ICMB09: c_int = 3;
pub const IPMI_CHANNEL_MEDIUM_8023LAN: c_int = 4;
pub const IPMI_CHANNEL_MEDIUM_ASYNC: c_int = 5;
pub const IPMI_CHANNEL_MEDIUM_OTHER_LAN: c_int = 6;
pub const IPMI_CHANNEL_MEDIUM_PCI_SMBUS: c_int = 7;
pub const IPMI_CHANNEL_MEDIUM_SMBUS1: c_int = 8;
pub const IPMI_CHANNEL_MEDIUM_SMBUS2: c_int = 9;
pub const IPMI_CHANNEL_MEDIUM_USB1: c_int = 10;
pub const IPMI_CHANNEL_MEDIUM_USB2: c_int = 11;
pub const IPMI_CHANNEL_MEDIUM_SYSINTF: c_int = 12;
pub const IPMI_CHANNEL_MEDIUM_OEM_MIN: c_uint = 0x60;
pub const IPMI_CHANNEL_MEDIUM_OEM_MAX: c_uint = 0x7f;
