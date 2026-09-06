//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/hw_ip/pci/pci_general.h
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
// Copyright 2016-2019 HabanaLabs, Ltd.
// All Rights Reserved.
//
// PCI CONFIGURATION SPACE
pub const mmPCI_CONFIG_ELBI_ADDR: c_uint = 0xFF0;
pub const mmPCI_CONFIG_ELBI_DATA: c_uint = 0xFF4;
pub const mmPCI_CONFIG_ELBI_CTRL: c_uint = 0xFF8;

pub const mmPCI_CONFIG_ELBI_STS: c_uint = 0xFFC;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_revision_id {
// PCI revision ID 0 is not legal
    REV_ID_INVALID				= 0x00,
    REV_ID_A				= 0x01,
    REV_ID_B				= 0x02,
    REV_ID_C				= 0x03,
    REV_ID_D				= 0x04
}
