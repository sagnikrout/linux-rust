//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/ipa_uc.h
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
// ipa_uc_interrupt_handler() - Handler for microcontroller IPA interrupts
// @ipa:	IPA pointer
// @irq_id:	IPA interrupt ID
//
extern "C" {
    pub fn ipa_uc_interrupt_handler(ipa: *mut ipa, irq_id: ipa_irq_id);
}
//
// ipa_uc_config() - Configure the IPA microcontroller subsystem
// @ipa:	IPA pointer
//
extern "C" {
    pub fn ipa_uc_config(ipa: *mut ipa);
}
//
// ipa_uc_deconfig() - Inverse of ipa_uc_config()
// @ipa:	IPA pointer
//
extern "C" {
    pub fn ipa_uc_deconfig(ipa: *mut ipa);
}
//
// ipa_uc_power() - Take a proxy power reference for the microcontroller
// @ipa:	IPA pointer
//
// The first time the modem boots, it loads firmware for and starts the
// IPA-resident microcontroller.  The microcontroller signals that it
// has completed its initialization by sending an INIT_COMPLETED response
// message to the AP.  The AP must ensure the IPA is powered until
// it receives this message, and to do so we take a "proxy" clock
// reference on its behalf here.  Once we receive the INIT_COMPLETED
// message (in ipa_uc_response_hdlr()) we drop this power reference.
//
extern "C" {
    pub fn ipa_uc_power(ipa: *mut ipa);
}
//
// ipa_uc_panic_notifier()
// @ipa:	IPA pointer
//
// Notifier function called when the system crashes, to inform the
// microcontroller of the event.
//
extern "C" {
    pub fn ipa_uc_panic_notifier(ipa: *mut ipa);
}
