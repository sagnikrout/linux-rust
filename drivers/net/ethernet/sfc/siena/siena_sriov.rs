//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/siena/siena_sriov.h
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
// Driver for Solarflare network controllers and boards
// Copyright 2015 Solarflare Communications Inc.
//

// On the SFC9000 family each port is associated with 1 PCI physical
// function (PF) handled by sfc and a configurable number of virtual
// functions (VFs) that may be handled by some other driver, often in
// a VM guest.  The queue pointer registers are mapped in both PF and
// VF BARs such that an 8K region provides access to a single RX, TX
// and event queue (collectively a Virtual Interface, VI or VNIC).
//
// The PF has access to all 1024 VIs while VFs are mapped to VIs
// according to VI_BASE and VI_SCALE: VF i has access to VIs numbered
// in range [VI_BASE + i << VI_SCALE, VI_BASE + i + 1 << VI_SCALE).
// The number of VIs and the VI_SCALE value are configurable but must
// be established at boot time by firmware.
//
// Maximum VI_SCALE parameter supported by Siena
pub const EFX_VI_SCALE_MAX: c_int = 6;
// Base VI to use for SR-IOV. Must be aligned to (1 << EFX_VI_SCALE_MAX),
// so this is the smallest allowed value.
//

// Maximum number of VFs allowed
pub const EFX_VF_COUNT_MAX: c_int = 127;
// Limit EVQs on VFs to be only 8k to reduce buffer table reservation

// The number of buffer table entries reserved for each VI on a VF

extern "C" {
    pub fn efx_siena_sriov_configure(efx: *mut efx_nic, num_vfs: c_int) -> c_int;
}
extern "C" {
    pub fn efx_siena_sriov_init(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_siena_sriov_fini(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_siena_sriov_mac_address_changed(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_siena_sriov_wanted(efx: *mut efx_nic) -> bool;
}
extern "C" {
    pub fn efx_siena_sriov_reset(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_siena_sriov_flr(efx: *mut efx_nic, flr: unsigned);
}
extern "C" {
    pub fn efx_siena_sriov_set_vf_mac(efx: *mut efx_nic, vf: c_int, mac: *const u8) -> c_int;
}

extern "C" {
    pub fn efx_init_sriov() -> c_int;
}
extern "C" {
    pub fn efx_fini_sriov();
}

extern "C" {
    pub fn efx_siena_sriov_probe(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_siena_sriov_tx_flush_done(efx: *mut efx_nic, event: *mut efx_qword_t);
}
extern "C" {
    pub fn efx_siena_sriov_rx_flush_done(efx: *mut efx_nic, event: *mut efx_qword_t);
}
extern "C" {
    pub fn efx_siena_sriov_event(channel: *mut efx_channel, event: *mut efx_qword_t);
}
extern "C" {
    pub fn efx_siena_sriov_desc_fetch_err(efx: *mut efx_nic, dmaq: unsigned);
}
