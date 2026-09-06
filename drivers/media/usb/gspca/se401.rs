//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/se401.h
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
// GSPCA Endpoints (formerly known as AOX) se401 USB Camera sub Driver
//
// Copyright (C) 2011 Hans de Goede <hdegoede@redhat.com>
//
// Based on the v4l1 se401 driver which is:
//
// Copyright (c) 2000 Jeroen B. Vreeken (pe1rxq@amsat.org)
//
pub const SE401_REQ_GET_CAMERA_DESCRIPTOR: c_uint = 0x06;
pub const SE401_REQ_START_CONTINUOUS_CAPTURE: c_uint = 0x41;
pub const SE401_REQ_STOP_CONTINUOUS_CAPTURE: c_uint = 0x42;
pub const SE401_REQ_CAPTURE_FRAME: c_uint = 0x43;
pub const SE401_REQ_GET_BRT: c_uint = 0x44;
pub const SE401_REQ_SET_BRT: c_uint = 0x45;
pub const SE401_REQ_GET_WIDTH: c_uint = 0x4c;
pub const SE401_REQ_SET_WIDTH: c_uint = 0x4d;
pub const SE401_REQ_GET_HEIGHT: c_uint = 0x4e;
pub const SE401_REQ_SET_HEIGHT: c_uint = 0x4f;
pub const SE401_REQ_GET_OUTPUT_MODE: c_uint = 0x50;
pub const SE401_REQ_SET_OUTPUT_MODE: c_uint = 0x51;
pub const SE401_REQ_GET_EXT_FEATURE: c_uint = 0x52;
pub const SE401_REQ_SET_EXT_FEATURE: c_uint = 0x53;
pub const SE401_REQ_CAMERA_POWER: c_uint = 0x56;
pub const SE401_REQ_LED_CONTROL: c_uint = 0x57;
pub const SE401_REQ_BIOS: c_uint = 0xff;
pub const SE401_BIOS_READ: c_uint = 0x07;
pub const SE401_FORMAT_BAYER: c_uint = 0x40;
// Hyundai hv7131b registers
// Mode registers:
pub const HV7131_REG_MODE_A: c_uint = 0x00;
pub const HV7131_REG_MODE_B: c_uint = 0x01;
pub const HV7131_REG_MODE_C: c_uint = 0x02;
// Frame registers:
pub const HV7131_REG_FRSU: c_uint = 0x10;
pub const HV7131_REG_FRSL: c_uint = 0x11;
pub const HV7131_REG_FCSU: c_uint = 0x12;
pub const HV7131_REG_FCSL: c_uint = 0x13;
pub const HV7131_REG_FWHU: c_uint = 0x14;
pub const HV7131_REG_FWHL: c_uint = 0x15;
pub const HV7131_REG_FWWU: c_uint = 0x16;
pub const HV7131_REG_FWWL: c_uint = 0x17;
// Timing registers:
pub const HV7131_REG_THBU: c_uint = 0x20;
pub const HV7131_REG_THBL: c_uint = 0x21;
pub const HV7131_REG_TVBU: c_uint = 0x22;
pub const HV7131_REG_TVBL: c_uint = 0x23;
pub const HV7131_REG_TITU: c_uint = 0x25;
pub const HV7131_REG_TITM: c_uint = 0x26;
pub const HV7131_REG_TITL: c_uint = 0x27;
pub const HV7131_REG_TMCD: c_uint = 0x28;
// Adjust Registers:
pub const HV7131_REG_ARLV: c_uint = 0x30;
pub const HV7131_REG_ARCG: c_uint = 0x31;
pub const HV7131_REG_AGCG: c_uint = 0x32;
pub const HV7131_REG_ABCG: c_uint = 0x33;
pub const HV7131_REG_APBV: c_uint = 0x34;
pub const HV7131_REG_ASLP: c_uint = 0x54;
// Offset Registers:
pub const HV7131_REG_OFSR: c_uint = 0x50;
pub const HV7131_REG_OFSG: c_uint = 0x51;
pub const HV7131_REG_OFSB: c_uint = 0x52;
// REset level statistics registers:
pub const HV7131_REG_LOREFNOH: c_uint = 0x57;
pub const HV7131_REG_LOREFNOL: c_uint = 0x58;
pub const HV7131_REG_HIREFNOH: c_uint = 0x59;
pub const HV7131_REG_HIREFNOL: c_uint = 0x5a;
// se401 registers
pub const SE401_OPERATINGMODE: c_uint = 0x2000;
