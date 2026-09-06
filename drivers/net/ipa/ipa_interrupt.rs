//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/ipa_interrupt.h
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
// Copyright (C) 2018-2024 Linaro Ltd.
//

//
// ipa_interrupt_suspend_enable - Enable TX_SUSPEND for an endpoint
// @interrupt:		IPA interrupt structure
// @endpoint_id:	Endpoint whose interrupt should be enabled
//
// Note:  The "TX" in the name is from the perspective of the IPA hardware.
// A TX_SUSPEND interrupt arrives on an AP RX enpoint when packet data can't
// be delivered to the endpoint because it is suspended (or its underlying
// channel is stopped).
//
// ipa_interrupt_suspend_disable - Disable TX_SUSPEND for an endpoint
// @interrupt:		IPA interrupt structure
// @endpoint_id:	Endpoint whose interrupt should be disabled
//
// ipa_interrupt_simulate_suspend() - Simulate TX_SUSPEND IPA interrupt
// @interrupt:	IPA interrupt structure
//
// This calls the TX_SUSPEND interrupt handler, as if such an interrupt
// had been signaled.  This is needed to work around a hardware quirk
// that occurs if aggregation is active on an endpoint when its underlying
// channel is suspended.
//
extern "C" {
    pub fn ipa_interrupt_simulate_suspend(interrupt: *mut ipa_interrupt);
}
//
// ipa_interrupt_enable() - Enable an IPA interrupt type
// @ipa:	IPA pointer
// @ipa_irq:	IPA interrupt ID
//
extern "C" {
    pub fn ipa_interrupt_enable(ipa: *mut ipa, ipa_irq: ipa_irq_id);
}
//
// ipa_interrupt_disable() - Disable an IPA interrupt type
// @ipa:	IPA pointer
// @ipa_irq:	IPA interrupt ID
//
extern "C" {
    pub fn ipa_interrupt_disable(ipa: *mut ipa, ipa_irq: ipa_irq_id);
}
//
// ipa_interrupt_irq_enable() - Enable IPA interrupts
// @ipa:	IPA pointer
//
// This enables the IPA interrupt line
//
extern "C" {
    pub fn ipa_interrupt_irq_enable(ipa: *mut ipa);
}
//
// ipa_interrupt_irq_disable() - Disable IPA interrupts
// @ipa:	IPA pointer
//
// This disables the IPA interrupt line
//
extern "C" {
    pub fn ipa_interrupt_irq_disable(ipa: *mut ipa);
}
//
// ipa_interrupt_config() - Configure IPA interrupts
// @ipa:	IPA pointer
//
// Return:	0 if successful, or a negative error code
//
extern "C" {
    pub fn ipa_interrupt_config(ipa: *mut ipa) -> c_int;
}
//
// ipa_interrupt_deconfig() - Inverse of ipa_interrupt_config()
// @ipa:	IPA pointer
//
extern "C" {
    pub fn ipa_interrupt_deconfig(ipa: *mut ipa);
}
//
// ipa_interrupt_init() - Initialize the IPA interrupt structure
// @pdev:	IPA platform device pointer
//
// Return:	Pointer to an IPA interrupt structure, or a pointer-coded error
//
// ipa_interrupt_exit() - Inverse of ipa_interrupt_init()
// @interrupt:	IPA interrupt structure
//
extern "C" {
    pub fn ipa_interrupt_exit(interrupt: *mut ipa_interrupt);
}
