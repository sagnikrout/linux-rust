//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/pcic.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2019-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const ATH11K_PCI_IRQ_CE0_OFFSET: c_int = 3;
pub const ATH11K_PCI_IRQ_DP_OFFSET: c_int = 14;
pub const ATH11K_PCI_CE_WAKE_IRQ: c_int = 2;
pub const ATH11K_PCI_WINDOW_ENABLE_BIT: c_uint = 0x40000000;
pub const ATH11K_PCI_WINDOW_REG_ADDRESS: c_uint = 0x310c;

pub const ATH11K_PCI_WINDOW_START: c_uint = 0x80000;

// BAR0 + 4k is always accessible, and no
// need to force wakeup.
// 4K - 32 = 0xFE0
//
pub const ATH11K_PCI_ACCESS_ALWAYS_OFF: c_uint = 0xFE0;
extern "C" {
    pub fn ath11k_pcic_write32(ab: *mut ath11k_base, offset: u32, value: u32);
}
extern "C" {
    pub fn ath11k_pcic_read32(ab: *mut ath11k_base, offset: u32) -> u32;
}
extern "C" {
    pub fn ath11k_pcic_get_ce_msi_idx(ab: *mut ath11k_base, ce_id: u32, msi_idx: *mut u32);
}
extern "C" {
    pub fn ath11k_pcic_free_irq(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_pcic_config_irq(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_pcic_ext_irq_enable(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_pcic_ext_irq_disable(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_pcic_stop(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_pcic_start(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_pcic_ce_irqs_enable(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_pcic_ce_irq_disable_sync(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_pcic_init_msi_config(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_pcic_read(ab: *mut ath11k_base, buf: *mut c_void, start: u32, end: u32) -> c_int;
}
extern "C" {
    pub fn ath11k_pci_enable_ce_irqs_except_wake_irq(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_pci_disable_ce_irqs_except_wake_irq(ab: *mut ath11k_base);
}
