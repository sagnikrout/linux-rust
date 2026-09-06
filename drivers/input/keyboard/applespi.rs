//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/keyboard/applespi.h
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
// MacBook (Pro) SPI keyboard and touchpad driver
//
// Copyright (c) 2015-2019 Federico Lorenzi
// Copyright (c) 2017-2019 Ronald Tschalär
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum applespi_evt_type {
    ET_CMD_TP_INI = BIT(0),
    ET_CMD_BL = BIT(1),
    ET_CMD_CL = BIT(2),
    ET_RD_KEYB = BIT(8),
    ET_RD_TPAD = BIT(9),
    ET_RD_UNKN = BIT(10),
    ET_RD_IRQ = BIT(11),
    ET_RD_CRC = BIT(12),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum applespi_pkt_type {
    PT_READ,
    PT_WRITE,
    PT_STATUS,
}
