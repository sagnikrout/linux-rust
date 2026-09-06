//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/altera/altera_sgdma.h
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
// Altera TSE SGDMA and MSGDMA Linux driver
// Copyright (C) 2014 Altera Corporation. All rights reserved
//
extern "C" {
    pub fn sgdma_reset(: *mut altera_tse_private);
}
extern "C" {
    pub fn sgdma_enable_txirq(: *mut altera_tse_private);
}
extern "C" {
    pub fn sgdma_enable_rxirq(: *mut altera_tse_private);
}
extern "C" {
    pub fn sgdma_disable_rxirq(: *mut altera_tse_private);
}
extern "C" {
    pub fn sgdma_disable_txirq(: *mut altera_tse_private);
}
extern "C" {
    pub fn sgdma_clear_rxirq(: *mut altera_tse_private);
}
extern "C" {
    pub fn sgdma_clear_txirq(: *mut altera_tse_private);
}
extern "C" {
    pub fn sgdma_tx_buffer(priv: *mut altera_tse_private, : *mut tse_buffer) -> c_int;
}
extern "C" {
    pub fn sgdma_tx_completions(: *mut altera_tse_private) -> u32;
}
extern "C" {
    pub fn sgdma_add_rx_desc(priv: *mut altera_tse_private, : *mut tse_buffer);
}
extern "C" {
    pub fn sgdma_status(: *mut altera_tse_private);
}
extern "C" {
    pub fn sgdma_rx_status(: *mut altera_tse_private) -> u32;
}
extern "C" {
    pub fn sgdma_initialize(: *mut altera_tse_private) -> c_int;
}
extern "C" {
    pub fn sgdma_uninitialize(: *mut altera_tse_private);
}
extern "C" {
    pub fn sgdma_start_rxdma(: *mut altera_tse_private);
}
