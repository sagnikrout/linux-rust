//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/efx_channels.h
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
// Copyright 2018 Solarflare Communications Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//
extern "C" {
    pub fn efx_probe_interrupts(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_remove_interrupts(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_soft_enable_interrupts(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_soft_disable_interrupts(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_enable_interrupts(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_disable_interrupts(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_set_interrupt_affinity(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_clear_interrupt_affinity(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_probe_eventq(channel: *mut efx_channel) -> c_int;
}
extern "C" {
    pub fn efx_init_eventq(channel: *mut efx_channel) -> c_int;
}
extern "C" {
    pub fn efx_start_eventq(channel: *mut efx_channel);
}
extern "C" {
    pub fn efx_stop_eventq(channel: *mut efx_channel);
}
extern "C" {
    pub fn efx_fini_eventq(channel: *mut efx_channel);
}
extern "C" {
    pub fn efx_remove_eventq(channel: *mut efx_channel);
}
extern "C" {
    pub fn efx_realloc_channels(efx: *mut efx_nic, rxq_entries: u32, txq_entries: u32) -> c_int;
}
extern "C" {
    pub fn efx_set_channel_names(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_init_channels(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_probe_channels(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_set_channels(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_remove_channel(channel: *mut efx_channel);
}
extern "C" {
    pub fn efx_remove_channels(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_fini_channels(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_start_channels(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_stop_channels(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_init_napi_channel(channel: *mut efx_channel);
}
extern "C" {
    pub fn efx_init_napi(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_fini_napi_channel(channel: *mut efx_channel);
}
extern "C" {
    pub fn efx_fini_napi(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_channel_dummy_op_void(channel: *mut efx_channel);
}
