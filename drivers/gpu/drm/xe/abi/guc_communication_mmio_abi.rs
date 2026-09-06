//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/abi/guc_communication_mmio_abi.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2014-2021 Intel Corporation
//
// DOC: GuC MMIO based communication
//
// The MMIO based communication between Host and GuC relies on special
// hardware registers which format could be defined by the software
// (so called scratch registers).
//
// Each MMIO based message, both Host to GuC (H2G) and GuC to Host (G2H)
// messages, which maximum length depends on number of available scratch
// registers, is directly written into those scratch registers.
//
// For Gen9+, there are 16 software scratch registers 0xC180-0xC1B8,
// but no H2G command takes more than 4 parameters and the GuC firmware
// itself uses an 4-element array to store the H2G message.
//
// For Gen11+, there are additional 4 registers 0x190240-0x19024C, which
// are, regardless on lower count, preferred over legacy ones.
//
// The MMIO based communication is mainly used during driver initialization
// phase to setup the `CTB based communication`_ that will be used afterwards.
//
pub const GUC_MAX_MMIO_MSG_LEN: c_int = 4;
//
// DOC: MMIO HXG Message
//
// Format of the MMIO messages follows definitions of `HXG Message`_.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |  31:0 |                                                              |
// +---+-------+                                                              |
// |...|       | [Embedded `HXG Message`_]                                    |
// +---+-------+                                                              |
// | n |  31:0 |                                                              |
// +---+-------+--------------------------------------------------------------+
//
