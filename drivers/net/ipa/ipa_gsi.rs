//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/ipa_gsi.h
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
// Copyright (c) 2012-2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2019-2020 Linaro Ltd.
//

//
// ipa_gsi_trans_complete() - GSI transaction completion callback
// @trans:	Transaction that has completed
//
// This called from the GSI layer to notify the IPA layer that a
// transaction has completed.
//
extern "C" {
    pub fn ipa_gsi_trans_complete(trans: *mut gsi_trans);
}
//
// ipa_gsi_trans_release() - GSI transaction release callback
// @trans:	Transaction whose resources should be freed
//
// This called from the GSI layer to notify the IPA layer that a
// transaction is about to be freed, so any resources associated
// with it should be released.
//
extern "C" {
    pub fn ipa_gsi_trans_release(trans: *mut gsi_trans);
}
//
// ipa_gsi_channel_tx_queued() - GSI queued to hardware notification
// @gsi:	GSI pointer
// @channel_id:	Channel number
// @count:	Number of transactions queued
// @byte_count:	Number of bytes to transfer represented by transactions
//
// This called from the GSI layer to notify the IPA layer that some
// number of transactions have been queued to hardware for execution.
//
// ipa_gsi_channel_tx_completed() - GSI transaction completion callback
// @gsi:	GSI pointer
// @channel_id:	Channel number
// @count:	Number of transactions completed since last report
// @byte_count:	Number of bytes transferred represented by transactions
//
// This called from the GSI layer to notify the IPA layer that the hardware
// has reported the completion of some number of transactions.
//
// ipa_gsi_endpoint_data_empty() - Empty endpoint config data test
// @data:	endpoint configuration data
//
// Determines whether an endpoint configuration data entry is empty,
// meaning it contains no valid configuration information and should
// be ignored.
//
// Return:	true if empty; false otherwise
//
extern "C" {
    pub fn ipa_gsi_endpoint_data_empty(data: *const ipa_gsi_endpoint_data) -> bool;
}
