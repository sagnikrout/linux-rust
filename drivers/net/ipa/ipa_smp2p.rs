//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/ipa_smp2p.h
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
// Copyright (C) 2019-2022 Linaro Ltd.
//

//
// ipa_smp2p_init() - Initialize the IPA SMP2P subsystem
// @ipa:	IPA pointer
// @pdev:	Platform device pointer
// @modem_init:	Whether the modem is responsible for GSI initialization
//
// Return:	0 if successful, or a negative error code
//
// ipa_smp2p_exit() - Inverse of ipa_smp2p_init()
// @ipa:	IPA pointer
//
extern "C" {
    pub fn ipa_smp2p_exit(ipa: *mut ipa);
}
//
// ipa_smp2p_irq_disable_setup() - Disable the "setup ready" interrupt
// @ipa:	IPA pointer
//
// Disable the "ipa-setup-ready" interrupt from the modem.
//
extern "C" {
    pub fn ipa_smp2p_irq_disable_setup(ipa: *mut ipa);
}
//
// ipa_smp2p_notify_reset() - Reset modem notification state
// @ipa:	IPA pointer
//
// If the modem crashes it queries the IPA power state.  In cleaning
// up after such a crash this is used to reset some state maintained
// for managing this notification.
//
extern "C" {
    pub fn ipa_smp2p_notify_reset(ipa: *mut ipa);
}
