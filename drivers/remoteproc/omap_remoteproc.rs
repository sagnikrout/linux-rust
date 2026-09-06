//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/remoteproc/omap_remoteproc.h
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Remote processor messaging
//
// Copyright (C) 2011-2020 Texas Instruments, Inc.
// Copyright (C) 2011 Google, Inc.
// All rights reserved.
//
// enum - Predefined Mailbox Messages
//
// @RP_MBOX_READY: informs the M3's that we're up and running. this is
// part of the init sequence sent that the M3 expects to see immediately
// after it is booted.
//
// @RP_MBOX_PENDING_MSG: informs the receiver that there is an inbound
// message waiting in its own receive-side vring. please note that currently
// this message is optional: alternatively, one can explicitly send the index
// of the triggered virtqueue itself. the preferred approach will be decided
// as we progress and experiment with those two different approaches.
//
// @RP_MBOX_CRASH: this message is sent if BIOS crashes
//
// @RP_MBOX_ECHO_REQUEST: a mailbox-level "ping" message.
//
// @RP_MBOX_ECHO_REPLY: a mailbox-level reply to a "ping"
//
// @RP_MBOX_ABORT_REQUEST: a "please crash" request, used for testing the
// recovery mechanism (to some extent).
//
// @RP_MBOX_SUSPEND_AUTO: auto suspend request for the remote processor
//
// @RP_MBOX_SUSPEND_SYSTEM: system suspend request for the remote processor
//
// @RP_MBOX_SUSPEND_ACK: successful response from remote processor for a
// suspend request
//
// @RP_MBOX_SUSPEND_CANCEL: a cancel suspend response from a remote processor
// on a suspend request
//
// Introduce new message definitions if any here.
//
// @RP_MBOX_END_MSG: Indicates end of known/defined messages from remote core
// This should be the last definition.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_rp_mbox_messages {
    RP_MBOX_READY		= 0xFFFFFF00,
    RP_MBOX_PENDING_MSG	= 0xFFFFFF01,
    RP_MBOX_CRASH		= 0xFFFFFF02,
    RP_MBOX_ECHO_REQUEST	= 0xFFFFFF03,
    RP_MBOX_ECHO_REPLY	= 0xFFFFFF04,
    RP_MBOX_ABORT_REQUEST	= 0xFFFFFF05,
    RP_MBOX_SUSPEND_AUTO	= 0xFFFFFF10,
    RP_MBOX_SUSPEND_SYSTEM	= 0xFFFFFF11,
    RP_MBOX_SUSPEND_ACK	= 0xFFFFFF12,
    RP_MBOX_SUSPEND_CANCEL	= 0xFFFFFF13,
    RP_MBOX_END_MSG		= 0xFFFFFF14,
}
