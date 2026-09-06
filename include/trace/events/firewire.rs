//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/firewire.h
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
// Copyright (c) 2024 Takashi Sakamoto

// Some macros are defined in 'drivers/firewire/packet-header-definitions.h'.
// The content of TP_printk field is preprocessed, then put to the module binary.

pub const QUADLET_SIZE: c_int = 4;
// This format is for the request subaction.
// The value of status is one of ack codes and rcodes specific to Linux FireWire subsystem.
// This format is for the response subaction.

// Some macros are defined in 'drivers/firewire/phy-packet-definitions.h'.
// The content of TP_printk field is preprocessed, then put to the module binary.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_iso_context_completions_cause {
    FW_ISO_CONTEXT_COMPLETIONS_CAUSE_FLUSH = 0,
    FW_ISO_CONTEXT_COMPLETIONS_CAUSE_INTERRUPT,
    FW_ISO_CONTEXT_COMPLETIONS_CAUSE_HEADER_OVERFLOW,
}

