//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/belkin_sa.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Definitions for Belkin USB Serial Adapter Driver
//
// Copyright (C) 2000
// William Greathouse (wgreathouse@smva.com)
//
// This program is largely derived from work by the linux-usb group
// and associated source files.  Please see the usb/serial files for
// individual credits and copyrights.
//
// See Documentation/usb/usb-serial.rst for more information on using this
// driver
//
// 12-Mar-2001 gkh
// Added GoHubs GO-COM232 device id.
//
// 06-Nov-2000 gkh
// Added old Belkin and Peracom device ids, which this driver supports
//
// 12-Oct-2000 William Greathouse
// First cut at supporting Belkin USB Serial Adapter F5U103
// I did not have a copy of the original work to support this
// adapter, so pardon any stupid mistakes.  All of the information
// I am using to write this driver was acquired by using a modified
// UsbSnoop on Windows2000.
//
pub const BELKIN_DOCKSTATION_VID: c_uint = 0x050d	/* Vendor Id */;
pub const BELKIN_DOCKSTATION_PID: c_uint = 0x1203	/* Product Id */;
pub const BELKIN_SA_VID: c_uint = 0x050d	/* Vendor Id */;
pub const BELKIN_SA_PID: c_uint = 0x0103	/* Product Id */;
pub const BELKIN_OLD_VID: c_uint = 0x056c	/* Belkin's "old" vendor id */;
pub const BELKIN_OLD_PID: c_uint = 0x8007	/* Belkin's "old" single port serial converter's id */;
pub const PERACOM_VID: c_uint = 0x0565	/* Peracom's vendor id */;
pub const PERACOM_PID: c_uint = 0x0001	/* Peracom's single port serial converter's id */;
pub const GOHUBS_VID: c_uint = 0x0921	/* GoHubs vendor id */;
pub const GOHUBS_PID: c_uint = 0x1000	/* GoHubs single port serial converter's id (identical to the Peracom device) */;
pub const HANDYLINK_PID: c_uint = 0x1200	/* HandyLink USB's id (identical to the Peracom device) */;
// Vendor Request Interface

// (always in Wininit sequence before flow control)

pub const BELKIN_SA_SET_REQUEST_TYPE: c_uint = 0x40;

pub const BELKIN_SA_PARITY_NONE: c_int = 0;
pub const BELKIN_SA_PARITY_EVEN: c_int = 1;
pub const BELKIN_SA_PARITY_ODD: c_int = 2;
pub const BELKIN_SA_PARITY_MARK: c_int = 3;
pub const BELKIN_SA_PARITY_SPACE: c_int = 4;
pub const BELKIN_SA_FLOW_NONE: c_uint = 0x0000	/* No flow control */;
pub const BELKIN_SA_FLOW_OCTS: c_uint = 0x0001	/* use CTS input to throttle output */;
pub const BELKIN_SA_FLOW_ODSR: c_uint = 0x0002	/* use DSR input to throttle output */;
pub const BELKIN_SA_FLOW_IDSR: c_uint = 0x0004	/* use DSR input to enable receive */;
pub const BELKIN_SA_FLOW_IDTR: c_uint = 0x0008	/* use DTR output for input flow control */;
pub const BELKIN_SA_FLOW_IRTS: c_uint = 0x0010	/* use RTS output for input flow control */;
pub const BELKIN_SA_FLOW_ORTS: c_uint = 0x0020	/* use RTS to indicate data available to send */;
pub const BELKIN_SA_FLOW_ERRSUB: c_uint = 0x0040	/* ???? guess ???? substitute inline errors */;
pub const BELKIN_SA_FLOW_OXON: c_uint = 0x0080	/* use XON/XOFF for output flow control */;
pub const BELKIN_SA_FLOW_IXON: c_uint = 0x0100	/* use XON/XOFF for input flow control */;
//
// It seems that the interrupt pipe is closely modelled after the
// 16550 register layout.  This is probably because the adapter can
// be used in a "DOS" environment to simulate a standard hardware port.
//

pub const BELKIN_SA_LSR_RDR: c_uint = 0x01	/* receive data ready */;
pub const BELKIN_SA_LSR_OE: c_uint = 0x02	/* overrun error */;
pub const BELKIN_SA_LSR_PE: c_uint = 0x04	/* parity error */;
pub const BELKIN_SA_LSR_FE: c_uint = 0x08	/* framing error */;
pub const BELKIN_SA_LSR_BI: c_uint = 0x10	/* break indicator */;
pub const BELKIN_SA_LSR_THE: c_uint = 0x20	/* tx holding register empty */;
pub const BELKIN_SA_LSR_TE: c_uint = 0x40	/* transmit register empty */;
pub const BELKIN_SA_LSR_ERR: c_uint = 0x80	/* OE | PE | FE | BI */;

pub const BELKIN_SA_MSR_DCTS: c_uint = 0x01	/* Delta CTS */;
pub const BELKIN_SA_MSR_DDSR: c_uint = 0x02	/* Delta DSR */;
pub const BELKIN_SA_MSR_DRI: c_uint = 0x04	/* Delta RI */;
pub const BELKIN_SA_MSR_DCD: c_uint = 0x08	/* Delta CD */;
pub const BELKIN_SA_MSR_CTS: c_uint = 0x10	/* Current CTS */;
pub const BELKIN_SA_MSR_DSR: c_uint = 0x20	/* Current DSR */;
pub const BELKIN_SA_MSR_RI: c_uint = 0x40	/* Current RI */;
pub const BELKIN_SA_MSR_CD: c_uint = 0x80	/* Current CD */;
