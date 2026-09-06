//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/extcon.h
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
// External Connector (extcon) framework
// - linux/include/linux/extcon.h for extcon consumer device driver.
//
// Copyright (C) 2015 Samsung Electronics
// Author: Chanwoo Choi <cw00.choi@samsung.com>
//
// Copyright (C) 2012 Samsung Electronics
// Author: Donggeun Kim <dg77.kim@samsung.com>
// Author: MyungJoo Ham <myungjoo.ham@samsung.com>
//
// based on switch class driver
// Copyright (C) 2008 Google, Inc.
// Author: Mike Lockwood <lockwood@android.com>
//

//
// Define the type of supported external connectors
//

//
// Define the unique id of supported external connectors
//
pub const EXTCON_NONE: c_int = 0;
// USB external connector
pub const EXTCON_USB: c_int = 1;
pub const EXTCON_USB_HOST: c_int = 2;
//
// Charging external connector
//
// When one SDP charger connector was reported, we should also report
// the USB connector, which means EXTCON_CHG_USB_SDP should always
// appear together with EXTCON_USB. The same as ACA charger connector,
// EXTCON_CHG_USB_ACA would normally appear with EXTCON_USB_HOST.
//
// The EXTCON_CHG_USB_SLOW connector can provide at least 500mA of
// current at 5V. The EXTCON_CHG_USB_FAST connector can provide at
// least 1A of current at 5V.
//

pub const EXTCON_CHG_USB_FAST: c_int = 9;
pub const EXTCON_CHG_USB_SLOW: c_int = 10;

// Jack external connector
pub const EXTCON_JACK_MICROPHONE: c_int = 20;
pub const EXTCON_JACK_HEADPHONE: c_int = 21;
pub const EXTCON_JACK_LINE_IN: c_int = 22;
pub const EXTCON_JACK_LINE_OUT: c_int = 23;
pub const EXTCON_JACK_VIDEO_IN: c_int = 24;
pub const EXTCON_JACK_VIDEO_OUT: c_int = 25;

pub const EXTCON_JACK_SPDIF_OUT: c_int = 27;
// Display external connector

// Miscellaneous external connector
pub const EXTCON_DOCK: c_int = 60;
pub const EXTCON_JIG: c_int = 61;
pub const EXTCON_MECHANICAL: c_int = 62;
pub const EXTCON_NUM: c_int = 63;
//
// Define the properties of supported external connectors.
//
// When adding the new extcon property, they *must* have
// the type/value/default information. Also, you *have to
// modify the EXTCON_PROP_[type]_START/END definitions
// which mean the range of the supported properties
// for each extcon type.
//
// The naming style of property
// : EXTCON_PROP_[type]_[property name]
//
// EXTCON_PROP_USB_[property name]	: USB property
// EXTCON_PROP_CHG_[property name]	: Charger property
// EXTCON_PROP_JACK_[property name]	: Jack property
// EXTCON_PROP_DISP_[property name]	: Display property
//
// Properties of EXTCON_TYPE_USB.
//
// - EXTCON_PROP_USB_VBUS
// @type:	integer (intval)
// @value:	0 (low) or 1 (high)
// @default:	0 (low)
// - EXTCON_PROP_USB_TYPEC_POLARITY
// @type:	integer (intval)
// @value:	0 (normal) or 1 (flip)
// @default:	0 (normal)
// - EXTCON_PROP_USB_SS (SuperSpeed)
// @type:       integer (intval)
// @value:      0 (USB/USB2) or 1 (USB3)
// @default:    0 (USB/USB2)
//
pub const EXTCON_PROP_USB_VBUS: c_int = 0;
pub const EXTCON_PROP_USB_TYPEC_POLARITY: c_int = 1;
pub const EXTCON_PROP_USB_SS: c_int = 2;
pub const EXTCON_PROP_USB_MIN: c_int = 0;
pub const EXTCON_PROP_USB_MAX: c_int = 2;

// Properties of EXTCON_TYPE_CHG.
pub const EXTCON_PROP_CHG_MIN: c_int = 50;
pub const EXTCON_PROP_CHG_MAX: c_int = 50;

// Properties of EXTCON_TYPE_JACK.
pub const EXTCON_PROP_JACK_MIN: c_int = 100;
pub const EXTCON_PROP_JACK_MAX: c_int = 100;

//
// Properties of EXTCON_TYPE_DISP.
//
// - EXTCON_PROP_DISP_HPD (Hot Plug Detect)
// @type:       integer (intval)
// @value:      0 (no hpd) or 1 (hpd)
// @default:    0 (no hpd)
//
pub const EXTCON_PROP_DISP_HPD: c_int = 150;
// Properties of EXTCON_TYPE_DISP.
pub const EXTCON_PROP_DISP_MIN: c_int = 150;
pub const EXTCON_PROP_DISP_MAX: c_int = 151;

//
// Define the type of property's value.
//
// Define the property's value as union type. Because each property
// would need the different data type to store it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union extcon_property_value {
    pub /: *mut *mut int intval; / type : integer (intval),
}

//
// Following APIs get the connected state of each external connector.
// The 'id' argument indicates the defined external connector.
//
extern "C" {
    pub fn extcon_get_state(edev: *mut extcon_dev, id: c_uint) -> c_int;
}
//
// Following APIs get the property of each external connector.
// The 'id' argument indicates the defined external connector
// and the 'prop' indicates the extcon property.
//
// And extcon_get_property_capability() get the capability of the property
// for each external connector. They are used to get the capability of the
// property of each external connector based on the id and property.
//
// Following APIs register the notifier block in order to detect
// the change of both state and property value for each external connector.
//
// extcon_register_notifier(*edev, id, *nb) : Register a notifier block
// for specific external connector of the extcon.
// extcon_register_notifier_all(*edev, *nb) : Register a notifier block
// for all supported external connectors of the extcon.
//
// Following APIs get the extcon_dev from devicetree or by through extcon name.
//
// Following API get the name of extcon device.

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

//
// Following structure and API are deprecated. EXTCON remains the function
// definition to prevent the build break.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extcon_specific_cable_nb {
    pub user_nb: *mut notifier_block,
    pub cable_index: c_int,
    pub edev: *mut extcon_dev,
    pub previous_value: c_ulong,
}
