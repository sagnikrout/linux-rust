//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/ipa_table.h
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
// Copyright (C) 2019-2024 Linaro Ltd.
//

//
// ipa_filtered_valid() - Validate a filter table endpoint bitmap
// @ipa:	IPA pointer
// @filtered:	Filter table endpoint bitmap to check
//
// Return:	true if all regions are valid, false otherwise
//
extern "C" {
    pub fn ipa_filtered_valid(ipa: *mut ipa, filtered: u64) -> bool;
}
//
// ipa_table_hash_support() - Return true if hashed tables are supported
// @ipa:	IPA pointer
//
extern "C" {
    pub fn ipa_table_hash_support(ipa: *mut ipa) -> bool;
}
//
// ipa_table_reset() - Reset filter and route tables entries to "none"
// @ipa:	IPA pointer
// @modem:	Whether to reset modem or AP entries
//
extern "C" {
    pub fn ipa_table_reset(ipa: *mut ipa, modem: bool);
}
//
// ipa_table_hash_flush() - Synchronize hashed filter and route updates
// @ipa:	IPA pointer
//
extern "C" {
    pub fn ipa_table_hash_flush(ipa: *mut ipa) -> c_int;
}
//
// ipa_table_setup() - Set up filter and route tables
// @ipa:	IPA pointer
//
// There is no need for a matching ipa_table_teardown() function.
//
extern "C" {
    pub fn ipa_table_setup(ipa: *mut ipa) -> c_int;
}
//
// ipa_table_config() - Configure filter and route tables
// @ipa:	IPA pointer
//
// There is no need for a matching ipa_table_deconfig() function.
//
extern "C" {
    pub fn ipa_table_config(ipa: *mut ipa);
}
//
// ipa_table_init() - Do early initialization of filter and route tables
// @ipa:	IPA pointer
//
extern "C" {
    pub fn ipa_table_init(ipa: *mut ipa) -> c_int;
}
//
// ipa_table_exit() - Inverse of ipa_table_init()
// @ipa:	IPA pointer
//
extern "C" {
    pub fn ipa_table_exit(ipa: *mut ipa);
}
//
// ipa_table_mem_valid() - Validate sizes of table memory regions
// @ipa:	IPA pointer
// @filter:	Whether to check filter or routing tables
//
extern "C" {
    pub fn ipa_table_mem_valid(ipa: *mut ipa, filter: bool) -> bool;
}
