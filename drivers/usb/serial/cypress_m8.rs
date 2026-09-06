//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/cypress_m8.h
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
// definitions and function prototypes used for the cypress USB to Serial
// controller
//
// For sending our feature buffer - controlling serial communication states.
// Linux HID has no support for serial devices so we do this through the driver
//
pub const HID_REQ_GET_REPORT: c_uint = 0x01;
pub const HID_REQ_SET_REPORT: c_uint = 0x09;
// List other cypress USB to Serial devices here, and add them to the id_table
// DeLorme Earthmate USB - a GPS device
pub const VENDOR_ID_DELORME: c_uint = 0x1163;
pub const PRODUCT_ID_EARTHMATEUSB: c_uint = 0x0100;
pub const PRODUCT_ID_EARTHMATEUSB_LT20: c_uint = 0x0200;
// Cypress HID->COM RS232 Adapter
pub const VENDOR_ID_CYPRESS: c_uint = 0x04b4;
pub const PRODUCT_ID_CYPHIDCOM: c_uint = 0x5500;
// Simply Automated HID->COM UPB PIM (using Cypress PID 0x5500)
pub const VENDOR_ID_SAI: c_uint = 0x17dd;
// FRWD Dongle - a GPS sports watch
pub const VENDOR_ID_FRWD: c_uint = 0x6737;
pub const PRODUCT_ID_CYPHIDCOM_FRWD: c_uint = 0x0001;
// Powercom UPS, chip CY7C63723
pub const VENDOR_ID_POWERCOM: c_uint = 0x0d9f;
pub const PRODUCT_ID_UPS: c_uint = 0x0002;
// Nokia CA-42 USB to serial cable
pub const VENDOR_ID_DAZZLE: c_uint = 0x07d0;
pub const PRODUCT_ID_CA42: c_uint = 0x4101;
// End of device listing
// Used for setting / requesting serial line settings
pub const CYPRESS_SET_CONFIG: c_uint = 0x01;
pub const CYPRESS_GET_CONFIG: c_uint = 0x02;
// Used for throttle control
pub const THROTTLED: c_uint = 0x1;
pub const ACTUALLY_THROTTLED: c_uint = 0x2;
//
// chiptypes - used in case firmware differs from the generic form ... offering
// different baud speeds/etc.
//
pub const CT_EARTHMATE: c_uint = 0x01;
pub const CT_CYPHIDCOM: c_uint = 0x02;
pub const CT_CA42V2: c_uint = 0x03;
pub const CT_GENERIC: c_uint = 0x0F;
// End of chiptype definitions
//
// RS-232 serial data communication protocol definitions.
//
// These are sent / read at byte 0 of the input/output hid reports.
// You can find these values defined in the CY4601 USB to Serial design notes.
//
pub const CONTROL_DTR: c_uint = 0x20	/* data terminal ready */;
pub const CONTROL_RTS: c_uint = 0x10	/* request to send */;
pub const CONTROL_RESET: c_uint = 0x08	/* sent with output report */;
pub const UART_MSR_MASK: c_uint = 0xf0;
pub const UART_RI: c_uint = 0x80	/* ring indicator */;
pub const UART_CD: c_uint = 0x40	/* carrier detect */;
pub const UART_DSR: c_uint = 0x20	/* data set ready */;
pub const UART_CTS: c_uint = 0x10	/* clear to send */;
pub const CYP_ERROR: c_uint = 0x08	/* received from input report */;
// End of RS-232 protocol definitions
