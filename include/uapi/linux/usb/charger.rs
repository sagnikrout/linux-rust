//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/usb/charger.h
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


//
// This file defines the USB charger type and state that are needed for
// USB device APIs.
//
// USB charger type:
// SDP (Standard Downstream Port)
// DCP (Dedicated Charging Port)
// CDP (Charging Downstream Port)
// ACA (Accessory Charger Adapters)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_charger_type {
    UNKNOWN_TYPE = 0,
    SDP_TYPE = 1,
    DCP_TYPE = 2,
    CDP_TYPE = 3,
    ACA_TYPE = 4,
}

// USB charger state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_charger_state {
    USB_CHARGER_DEFAULT = 0,
    USB_CHARGER_PRESENT = 1,
    USB_CHARGER_ABSENT = 2,
}
