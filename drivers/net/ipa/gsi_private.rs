//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/gsi_private.h
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
// Copyright (c) 2015-2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2018-2024 Linaro Ltd.
//
// === Only "gsi.c" and "gsi_trans.c" should include this file ===

//
// gsi_trans_move_complete() - Mark a GSI transaction completed
// @trans:	Transaction whose state is to be updated
//
extern "C" {
    pub fn gsi_trans_move_complete(trans: *mut gsi_trans);
}
//
// gsi_trans_move_polled() - Mark a transaction polled
// @trans:	Transaction whose state is to be updated
//
extern "C" {
    pub fn gsi_trans_move_polled(trans: *mut gsi_trans);
}
//
// gsi_trans_complete() - Complete a GSI transaction
// @trans:	Transaction to complete
//
// Marks a transaction complete (including freeing it).
//
extern "C" {
    pub fn gsi_trans_complete(trans: *mut gsi_trans);
}
//
// gsi_channel_trans_mapped() - Return a transaction mapped to a TRE index
// @channel:	Channel associated with the transaction
// @index:	Index of the TRE having a transaction
//
// Return:	The GSI transaction pointer associated with the TRE index
//
// gsi_channel_trans_complete() - Return a channel's next completed transaction
// @channel:	Channel whose next transaction is to be returned
//
// Return:	The next completed transaction, or NULL if nothing new
//
// gsi_channel_trans_cancel_pending() - Cancel pending transactions
// @channel:	Channel whose pending transactions should be cancelled
//
// Cancel all pending transactions on a channel.  These are transactions
// that have been committed but not yet completed.  This is required when
// the channel gets reset.  At that time all pending transactions will be
// marked as cancelled.
//
// NOTE:  Transactions already complete at the time of this call are
// unaffected.
//
extern "C" {
    pub fn gsi_channel_trans_cancel_pending(channel: *mut gsi_channel);
}
//
// gsi_channel_trans_init() - Initialize a channel's GSI transaction info
// @gsi:	GSI pointer
// @channel_id:	Channel number
//
// Return:	0 if successful, or -ENOMEM on allocation failure
//
// Creates and sets up information for managing transactions on a channel
//
extern "C" {
    pub fn gsi_channel_trans_init(gsi: *mut gsi, channel_id: u32) -> c_int;
}
//
// gsi_channel_trans_exit() - Inverse of gsi_channel_trans_init()
// @channel:	Channel whose transaction information is to be cleaned up
//
extern "C" {
    pub fn gsi_channel_trans_exit(channel: *mut gsi_channel);
}
//
// gsi_channel_doorbell() - Ring a channel's doorbell
// @channel:	Channel whose doorbell should be rung
//
// Rings a channel's doorbell to inform the GSI hardware that new
// transactions (TREs, really) are available for it to process.
//
extern "C" {
    pub fn gsi_channel_doorbell(channel: *mut gsi_channel);
}
// gsi_channel_update() - Update knowledge of channel hardware state
// @channel:	Channel to be updated
//
// Consult hardware, change the state of any newly-completed transactions
// on a channel.
//
extern "C" {
    pub fn gsi_channel_update(channel: *mut gsi_channel);
}
//
// gsi_ring_virt() - Return virtual address for a ring entry
// @ring:	Ring whose address is to be translated
// @index:	Index (slot number) of entry
//
// gsi_trans_tx_committed() - Record bytes committed for transmit
// @trans:	TX endpoint transaction being committed
//
// Report that a TX transaction has been committed.  It updates some
// statistics used to manage transmit rates.
//
extern "C" {
    pub fn gsi_trans_tx_committed(trans: *mut gsi_trans);
}
//
// gsi_trans_tx_queued() - Report a queued TX channel transaction
// @trans:	Transaction being passed to hardware
//
// Report to the network stack that a TX transaction is being supplied
// to the hardware.
//
extern "C" {
    pub fn gsi_trans_tx_queued(trans: *mut gsi_trans);
}
