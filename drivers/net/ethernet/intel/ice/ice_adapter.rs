//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_adapter.h
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
// SPDX-FileCopyrightText: Copyright Red Hat

//
// struct ice_port_list - data used to store the list of adapter ports
//
// This structure contains data used to maintain a list of adapter ports
//
// @ports: list of ports
// @lock: protect access to the ports list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_port_list {
    pub ports: list_head,
// To synchronize the ports list operations
    pub lock: mutex,
}

//
// struct ice_adapter - PCI adapter resources shared across PFs
// @refcount: Reference count. struct ice_pf objects hold the references.
// @ptp_gltsyn_time_lock: Spinlock protecting access to the GLTSYN_TIME
// register of the PTP clock.
// @txq_ctx_lock: Spinlock protecting access to the GLCOMM_QTX_CNTX_CTL register
// @cpi_phy_lock: Per-PHY mutex serializing CPI REQ/ACK transactions.
// Index 0 = PHY0, index 1 = PHY1. Used on E825C devices.
// @ctrl_pf: Control PF of the adapter
// @ports: Ports list
// @index: 64-bit index cached for collision detection on 32bit systems
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_adapter {
    pub refcount: refcount_t,
// For access to the GLTSYN_TIME register
    pub ptp_gltsyn_time_lock: spinlock_t,
// For access to GLCOMM_QTX_CNTX_CTL register
    pub txq_ctx_lock: spinlock_t,
// Serialize CPI REQ/ACK transactions per PHY (E825C only)
    pub cpi_phy_lock: [mutex; ICE_E825_MAX_PHYS],
    pub ctrl_pf: *mut ice_pf,
    pub ports: ice_port_list,
    pub index: u64,
}

extern "C" {
    pub fn ice_adapter_put(pdev: *mut pci_dev);
}
