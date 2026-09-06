//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intersil/p54/p54spi_eeprom.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2003 Conexant Americas Inc. All Rights Reserved.
// Copyright (C) 2004, 2005, 2006 Nokia Corporation
// Copyright 2008 Johannes Berg <johannes@sipsolutions.net>
// Copyright 2008 Christian Lamparter <chunkeey@web.de>
//
// based on:
// - cx3110x's pda.h from Nokia
// - cx3110-transfer.log by Johannes Berg
//
// struct eeprom_pda_wrap
// bogus MAC address
// struct bootrec_exp_if
// struct pda_country[6]
// struct pda_country
// struct pda_custom_wrapper
// 2412 MHz
// 2417 MHz
// 2422 MHz
// 2427 MHz
// 2432 MHz
// 2437 MHz
// 2442 MHz
// 2447 MHz
// 2452 MHz
// 2557 MHz
// 2562 MHz
// 2572 MHz
//
// Not really sure if this is actually the power_limit database,
// it looks a bit "related" to PDR_PRISM_ZIF_TX_IQ_CALIBRATION
//
// struct pda_custom_wrapper
// 2412 MHz
// 2417 MHz
// 2422 MHz
// 2427 MHz
// 2432 MHz
// 2437 MHz
// 2442 MHz
// 2447 MHz
// 2452 MHz
// 2457 MHz
// 2462 MHz
// 2467 MHz
// 2472 MHz
// struct pda_iq_autocal_entry[13]
// 2412 MHz
// 2417 MHz
// 2422 MHz
// 2427 MHz
// 2432 MHz
// 2437 MHz
// 2442 MHz
// 2447 MHz
// 2452 MHz
// 2457 MHz
// 2462 MHz
// 2467 MHz
// 2472 MHz
