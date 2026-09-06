//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/usb/ch11.h
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
// This file holds Hub protocol constants and data structures that are
// defined in chapter 11 (Hub Specification) of the USB 2.0 specification.
//
// It is used/shared between the USB core, the HCDs and couple of other USB
// drivers.
//

// This is arbitrary.
// From USB 2.0 spec Table 11-13, offset 7, a hub can
// have up to 255 ports. The most yet reported is 10.
// Upcoming hardware might raise that limit.
// Because the arrays need to add a bit for hub status data, we
// use 31, so plus one evens out to four bytes.
//
pub const USB_MAXCHILDREN: c_int = 31;
// See USB 3.1 spec Table 10-5
pub const USB_SS_MAXPORTS: c_int = 15;
//
// Hub request types
//

//
// Port status type for GetPortStatus requests added in USB 3.1
// See USB 3.1 spec Table 10-12
//
pub const HUB_PORT_STATUS: c_int = 0;
pub const HUB_PORT_PD_STATUS: c_int = 1;
pub const HUB_EXT_PORT_STATUS: c_int = 2;
//
// Hub class requests
// See USB 2.0 spec Table 11-16
//
pub const HUB_CLEAR_TT_BUFFER: c_int = 8;
pub const HUB_RESET_TT: c_int = 9;
pub const HUB_GET_TT_STATE: c_int = 10;
pub const HUB_STOP_TT: c_int = 11;
//
// Hub class additional requests defined by USB 3.0 spec
// See USB 3.0 spec Table 10-6
//
pub const HUB_SET_DEPTH: c_int = 12;
pub const HUB_GET_PORT_ERR_COUNT: c_int = 13;
//
// Hub Class feature numbers
// See USB 2.0 spec Table 11-17
//
pub const C_HUB_LOCAL_POWER: c_int = 0;
pub const C_HUB_OVER_CURRENT: c_int = 1;
//
// Port feature numbers
// See USB 2.0 spec Table 11-17
//
pub const USB_PORT_FEAT_CONNECTION: c_int = 0;
pub const USB_PORT_FEAT_ENABLE: c_int = 1;

pub const USB_PORT_FEAT_OVER_CURRENT: c_int = 3;
pub const USB_PORT_FEAT_RESET: c_int = 4;

pub const USB_PORT_FEAT_POWER: c_int = 8;

pub const USB_PORT_FEAT_C_CONNECTION: c_int = 16;
pub const USB_PORT_FEAT_C_ENABLE: c_int = 17;
pub const USB_PORT_FEAT_C_SUSPEND: c_int = 18;
pub const USB_PORT_FEAT_C_OVER_CURRENT: c_int = 19;
pub const USB_PORT_FEAT_C_RESET: c_int = 20;
pub const USB_PORT_FEAT_TEST: c_int = 21;
pub const USB_PORT_FEAT_INDICATOR: c_int = 22;
pub const USB_PORT_FEAT_C_PORT_L1: c_int = 23;
//
// Port feature selectors added by USB 3.0 spec.
// See USB 3.0 spec Table 10-7
//
pub const USB_PORT_FEAT_LINK_STATE: c_int = 5;
pub const USB_PORT_FEAT_U1_TIMEOUT: c_int = 23;
pub const USB_PORT_FEAT_U2_TIMEOUT: c_int = 24;
pub const USB_PORT_FEAT_C_PORT_LINK_STATE: c_int = 25;
pub const USB_PORT_FEAT_C_PORT_CONFIG_ERROR: c_int = 26;
pub const USB_PORT_FEAT_REMOTE_WAKE_MASK: c_int = 27;
pub const USB_PORT_FEAT_BH_PORT_RESET: c_int = 28;
pub const USB_PORT_FEAT_C_BH_PORT_RESET: c_int = 29;
pub const USB_PORT_FEAT_FORCE_LINKPM_ACCEPT: c_int = 30;

// USB 3.0 hub remote wake mask bits, see table 10-14

//
// Hub Status and Hub Change results
// See USB 2.0 spec Table 11-19 and Table 11-20
// USB 3.1 extends the port status request and may return 4 additional bytes.
// See USB 3.1 spec section 10.16.2.6 Table 10-12 and 10-15
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_port_status {
    pub wPortStatus: __le16,
    pub wPortChange: __le16,
    pub dwExtPortStatus: __le32,
// C attribute field omitted
//
// wPortStatus bit field
// See USB 2.0 spec Table 11-21
//
pub const USB_PORT_STAT_CONNECTION: c_uint = 0x0001;
pub const USB_PORT_STAT_ENABLE: c_uint = 0x0002;
pub const USB_PORT_STAT_SUSPEND: c_uint = 0x0004;
pub const USB_PORT_STAT_OVERCURRENT: c_uint = 0x0008;
pub const USB_PORT_STAT_RESET: c_uint = 0x0010;
pub const USB_PORT_STAT_L1: c_uint = 0x0020;
// bits 6 to 7 are reserved
pub const USB_PORT_STAT_POWER: c_uint = 0x0100;
pub const USB_PORT_STAT_LOW_SPEED: c_uint = 0x0200;
pub const USB_PORT_STAT_HIGH_SPEED: c_uint = 0x0400;
pub const USB_PORT_STAT_TEST: c_uint = 0x0800;
pub const USB_PORT_STAT_INDICATOR: c_uint = 0x1000;
// bits 13 to 15 are reserved
//
// Additions to wPortStatus bit field from USB 3.0
// See USB 3.0 spec Table 10-10
//
pub const USB_PORT_STAT_LINK_STATE: c_uint = 0x01e0;
pub const USB_SS_PORT_STAT_POWER: c_uint = 0x0200;
pub const USB_SS_PORT_STAT_SPEED: c_uint = 0x1c00;
pub const USB_PORT_STAT_SPEED_5GBPS: c_uint = 0x0000;
// Valid only if port is enabled
// Bits that are the same from USB 2.0

//
// Definitions for PORT_LINK_STATE values
// (bits 5-8) in wPortStatus
//
pub const USB_SS_PORT_LS_U0: c_uint = 0x0000;
pub const USB_SS_PORT_LS_U1: c_uint = 0x0020;
pub const USB_SS_PORT_LS_U2: c_uint = 0x0040;
pub const USB_SS_PORT_LS_U3: c_uint = 0x0060;
pub const USB_SS_PORT_LS_SS_DISABLED: c_uint = 0x0080;
pub const USB_SS_PORT_LS_RX_DETECT: c_uint = 0x00a0;
pub const USB_SS_PORT_LS_SS_INACTIVE: c_uint = 0x00c0;
pub const USB_SS_PORT_LS_POLLING: c_uint = 0x00e0;
pub const USB_SS_PORT_LS_RECOVERY: c_uint = 0x0100;
pub const USB_SS_PORT_LS_HOT_RESET: c_uint = 0x0120;
pub const USB_SS_PORT_LS_COMP_MOD: c_uint = 0x0140;
pub const USB_SS_PORT_LS_LOOPBACK: c_uint = 0x0160;
//
// wPortChange bit field
// See USB 2.0 spec Table 11-22 and USB 2.0 LPM ECN Table-4.10
// Bits 0 to 5 shown, bits 6 to 15 are reserved
//
pub const USB_PORT_STAT_C_CONNECTION: c_uint = 0x0001;
pub const USB_PORT_STAT_C_ENABLE: c_uint = 0x0002;
pub const USB_PORT_STAT_C_SUSPEND: c_uint = 0x0004;
pub const USB_PORT_STAT_C_OVERCURRENT: c_uint = 0x0008;
pub const USB_PORT_STAT_C_RESET: c_uint = 0x0010;
pub const USB_PORT_STAT_C_L1: c_uint = 0x0020;
//
// USB 3.0 wPortChange bit fields
// See USB 3.0 spec Table 10-11
//
pub const USB_PORT_STAT_C_BH_RESET: c_uint = 0x0020;
pub const USB_PORT_STAT_C_LINK_STATE: c_uint = 0x0040;
pub const USB_PORT_STAT_C_CONFIG_ERROR: c_uint = 0x0080;
//
// USB 3.1 dwExtPortStatus field masks
// See USB 3.1 spec 10.16.2.6.3 Table 10-15
//
pub const USB_EXT_PORT_STAT_RX_SPEED_ID: c_uint = 0x0000000f;
pub const USB_EXT_PORT_STAT_TX_SPEED_ID: c_uint = 0x000000f0;
pub const USB_EXT_PORT_STAT_RX_LANES: c_uint = 0x00000f00;
pub const USB_EXT_PORT_STAT_TX_LANES: c_uint = 0x0000f000;

//
// wHubCharacteristics (masks)
// See USB 2.0 spec Table 11-13, offset 3
//
pub const HUB_CHAR_LPSM: c_uint = 0x0003 /* Logical Power Switching Mode mask */;
pub const HUB_CHAR_COMMON_LPSM: c_uint = 0x0000 /* All ports power control at once */;
pub const HUB_CHAR_INDV_PORT_LPSM: c_uint = 0x0001 /* per-port power control */;
pub const HUB_CHAR_NO_LPSM: c_uint = 0x0002 /* no power switching */;
pub const HUB_CHAR_COMPOUND: c_uint = 0x0004 /* hub is part of a compound device */;
pub const HUB_CHAR_OCPM: c_uint = 0x0018 /* Over-Current Protection Mode mask */;
pub const HUB_CHAR_COMMON_OCPM: c_uint = 0x0000 /* All ports Over-Current reporting */;
pub const HUB_CHAR_INDV_PORT_OCPM: c_uint = 0x0008 /* per-port Over-current reporting */;
pub const HUB_CHAR_NO_OCPM: c_uint = 0x0010 /* No Over-current Protection support */;
pub const HUB_CHAR_TTTT: c_uint = 0x0060 /* TT Think Time mask */;
pub const HUB_CHAR_PORTIND: c_uint = 0x0080 /* per-port indicators (LEDs) */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_hub_status {
    pub wHubStatus: __le16,
    pub wHubChange: __le16,
// C attribute field omitted
//
// Hub Status & Hub Change bit masks
// See USB 2.0 spec Table 11-19 and Table 11-20
// Bits 0 and 1 for wHubStatus and wHubChange
// Bits 2 to 15 are reserved for both
//
pub const HUB_STATUS_LOCAL_POWER: c_uint = 0x0001;
pub const HUB_STATUS_OVERCURRENT: c_uint = 0x0002;
pub const HUB_CHANGE_LOCAL_POWER: c_uint = 0x0001;
pub const HUB_CHANGE_OVERCURRENT: c_uint = 0x0002;
//
// Hub descriptor
// See USB 2.0 spec Table 11-13
//

pub const USB_DT_HUB_NONVAR_SIZE: c_int = 7;
pub const USB_DT_SS_HUB_SIZE: c_int = 12;
//
// Hub Device descriptor
// USB Hub class device protocols
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_hub_descriptor {
    pub bDescLength: __u8,
    pub bDescriptorType: __u8,
    pub bNbrPorts: __u8,
    pub wHubCharacteristics: __le16,
    pub bPwrOn2PwrGood: __u8,
    pub bHubContrCurrent: __u8,
// 2.0 and 3.0 hubs differ here
// add 1 bit for hub status change; round to bytes
    pub 8]: __u8 DeviceRemovable[(USB_MAXCHILDREN + 1 + 7) /,
    pub 8]: __u8 PortPwrCtrlMask[(USB_MAXCHILDREN + 1 + 7) /,
// C attribute field omitted
    pub bHubHdrDecLat: __u8,
    pub wHubDelay: __le16,
    pub DeviceRemovable: __le16,
// C attribute field omitted
    pub u: },
// C attribute field omitted
// port indicator status selectors, tables 11-7 and 11-25
pub const HUB_LED_AUTO: c_int = 0;
pub const HUB_LED_AMBER: c_int = 1;
pub const HUB_LED_GREEN: c_int = 2;
pub const HUB_LED_OFF: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hub_led_mode {
    INDICATOR_AUTO = 0,
    INDICATOR_CYCLE,
// software blinks for attention:  software, hardware, reserved
    INDICATOR_GREEN_BLINK, INDICATOR_GREEN_BLINK_OFF,
    INDICATOR_AMBER_BLINK, INDICATOR_AMBER_BLINK_OFF,
    INDICATOR_ALT_BLINK, INDICATOR_ALT_BLINK_OFF
    } __attribute__ ((packed));

// Transaction Translator Think Times, in bits
pub const HUB_TTTT_8_BITS: c_uint = 0x00;
pub const HUB_TTTT_16_BITS: c_uint = 0x20;
pub const HUB_TTTT_24_BITS: c_uint = 0x40;
pub const HUB_TTTT_32_BITS: c_uint = 0x60;
