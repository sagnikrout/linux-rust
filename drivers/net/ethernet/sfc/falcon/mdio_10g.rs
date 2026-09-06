//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/falcon/mdio_10g.h
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
// Copyright 2006-2011 Solarflare Communications Inc.
//

//
// Helper functions for doing 10G MDIO as specified in IEEE 802.3 clause 45.
//

extern "C" {
    pub fn ef4_mdio_id_oui(id: u32) -> unsigned;
}
//
// Reset a specific MMD and wait for reset to clear.
// Return number of spins left (>0) on success, -%ETIMEDOUT on failure.
//
// This function will sleep
//
extern "C" {
    pub fn ef4_mdio_reset_mmd(efx: *mut ef4_nic, mmd: c_int, spins: c_int, spintime: c_int) -> c_int;
}
// As ef4_mdio_check_mmd but for multiple MMDs
extern "C" {
    pub fn ef4_mdio_check_mmds(efx: *mut ef4_nic, mmd_mask: c_uint) -> c_int;
}
// Check the link status of specified mmds in bit mask
extern "C" {
    pub fn ef4_mdio_links_ok(efx: *mut ef4_nic, mmd_mask: c_uint) -> bool;
}
// Generic transmit disable support though PMAPMD
extern "C" {
    pub fn ef4_mdio_transmit_disable(efx: *mut ef4_nic);
}
// Generic part of reconfigure: set/clear loopback bits
extern "C" {
    pub fn ef4_mdio_phy_reconfigure(efx: *mut ef4_nic);
}
// Set the power state of the specified MMDs
// Set (some of) the PHY settings over MDIO
// Push advertising flags and restart autonegotiation
extern "C" {
    pub fn ef4_mdio_an_reconfigure(efx: *mut ef4_nic);
}
// Get pause parameters from AN if available (otherwise return
// requested pause parameters)
//
extern "C" {
    pub fn ef4_mdio_get_pause(efx: *mut ef4_nic) -> u8;
}
// Wait for specified MMDs to exit reset within a timeout
extern "C" {
    pub fn ef4_mdio_wait_reset_mmds(efx: *mut ef4_nic, mmd_mask: c_uint) -> c_int;
}
// Set or clear flag, debouncing
// Liveness self-test for MDIO PHYs
extern "C" {
    pub fn ef4_mdio_test_alive(efx: *mut ef4_nic) -> c_int;
}
