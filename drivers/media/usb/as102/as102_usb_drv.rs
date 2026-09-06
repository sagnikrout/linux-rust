//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/as102/as102_usb_drv.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Abilis Systems Single DVB-T Receiver
// Copyright (C) 2008 Pierrick Hascoet <pierrick.hascoet@abilis.com>
// Copyright (C) 2010 Devin Heitmueller <dheitmueller@kernellabs.com>
//
pub const AS102_USB_DEVICE_TX_CTRL_CMD: c_uint = 0xF1;
pub const AS102_USB_DEVICE_RX_CTRL_CMD: c_uint = 0xF2;
// define these values to match the supported devices
// Abilis system: "TITAN"

pub const AS102_USB_DEVICE_VENDOR_ID: c_uint = 0x1BA6;
pub const AS102_USB_DEVICE_PID_0001: c_uint = 0x0001;
// PCTV Systems: PCTV picoStick (74e)

pub const PCTV_74E_USB_VID: c_uint = 0x2013;
pub const PCTV_74E_USB_PID: c_uint = 0x0246;
// Elgato: EyeTV DTT Deluxe

pub const ELGATO_EYETV_DTT_USB_VID: c_uint = 0x0fd9;
pub const ELGATO_EYETV_DTT_USB_PID: c_uint = 0x002c;
// nBox: nBox DVB-T Dongle

pub const NBOX_DVBT_DONGLE_USB_VID: c_uint = 0x0b89;
pub const NBOX_DVBT_DONGLE_USB_PID: c_uint = 0x0007;
// Sky Italia: Digital Key (green led)

pub const SKY_IT_DIGITAL_KEY_USB_VID: c_uint = 0x2137;
pub const SKY_IT_DIGITAL_KEY_USB_PID: c_uint = 0x0001;
extern "C" {
    pub fn as102_urb_stream_irq(urb: *mut urb);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as10x_usb_token_cmd_t {
// token cmd
    pub c: as10x_cmd_t,
// token response
    pub r: as10x_cmd_t,
}
