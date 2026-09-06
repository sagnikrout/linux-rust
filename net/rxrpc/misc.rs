//! Automatically rewritten from C to Rust
//! Source: net/rxrpc/misc.c
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
// Miscellaneous bits
//
// Copyright (C) 2016 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// The maximum listening backlog queue size that may be set on a socket by
// listen().
//
    let mut __read_mostly: unsigned int rxrpc_max_backlog = 10;
//
// How long to wait before scheduling an ACK with subtype DELAY (in ms).
//
// We use this when we've received new data packets.  If those packets aren't
// all consumed within this time we will send a DELAY ACK if an ACK was not
// requested to let the sender know it doesn't need to resend.
//
    let mut rxrpc_soft_ack_delay: c_ulong = 1000;
//
// How long to wait before scheduling an ACK with subtype IDLE (in ms).
//
// We use this when we've consumed some previously soft-ACK'd packets when
// further packets aren't immediately received to decide when to send an IDLE
// ACK let the other end know that it can free up its Tx buffer space.
//
    let mut rxrpc_idle_ack_delay: c_ulong = 500;
//
// Receive window size in packets.  This indicates the maximum number of
// unconsumed received packets we're willing to retain in memory.  Once this
// limit is hit, we should generate an EXCEEDS_WINDOW ACK and discard further
// packets.
//
    let mut rxrpc_rx_window_size: c_uint = 255;
//
// Maximum Rx MTU size.  This indicates to the sender the size of jumbo packet
// made by gluing normal packets together that we're willing to handle.
//
    let mut rxrpc_rx_mtu: c_uint = RXRPC_JUMBO(46);
//
// The maximum number of fragments in a received jumbo packet that we tell the
// sender that we're willing to handle.
//
    let mut rxrpc_rx_jumbo_max: c_uint = 46;

//
// The delay to inject into packet reception.
//
    unsigned long rxrpc_inject_rx_delay;
