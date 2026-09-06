//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rave-sp.h
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
// Core definitions for RAVE SP MFD driver.
//
// Copyright (C) 2017 Zodiac Inflight Innovations
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rave_sp_command {
    RAVE_SP_CMD_GET_FIRMWARE_VERSION	= 0x20,
    RAVE_SP_CMD_GET_BOOTLOADER_VERSION	= 0x21,
    RAVE_SP_CMD_BOOT_SOURCE			= 0x26,
    RAVE_SP_CMD_GET_BOARD_COPPER_REV	= 0x2B,
    RAVE_SP_CMD_GET_GPIO_STATE		= 0x2F,

    RAVE_SP_CMD_STATUS			= 0xA0,
    RAVE_SP_CMD_SW_WDT			= 0xA1,
    RAVE_SP_CMD_PET_WDT			= 0xA2,
    RAVE_SP_CMD_RMB_EEPROM			= 0xA4,
    RAVE_SP_CMD_SET_BACKLIGHT		= 0xA6,
    RAVE_SP_CMD_RESET			= 0xA7,
    RAVE_SP_CMD_RESET_REASON		= 0xA8,

    RAVE_SP_CMD_REQ_COPPER_REV		= 0xB6,
    RAVE_SP_CMD_GET_I2C_DEVICE_STATUS	= 0xBA,
    RAVE_SP_CMD_GET_SP_SILICON_REV		= 0xB9,
    RAVE_SP_CMD_CONTROL_EVENTS		= 0xBB,

    RAVE_SP_EVNT_BASE			= 0xE0,
}
