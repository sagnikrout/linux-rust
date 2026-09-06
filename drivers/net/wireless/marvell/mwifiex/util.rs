//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/mwifiex/util.h
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
// NXP Wireless LAN device driver: utility functions
//
// Copyright 2011-2020 NXP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_dma_mapping {
    pub addr: dma_addr_t,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_cb {
    pub dma_mapping: mwifiex_dma_mapping,
    pub rx_info: mwifiex_rxinfo,
    pub tx_info: mwifiex_txinfo,
}

// size/addr for mwifiex_debug_info

// size/addr for struct mwifiex_adapter

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_debug_data {
    pub /: *mut *mut char name[32]; / variable/array name,
    pub /: *mut *mut u32 size; / size of the variable/array,
    pub /: *mut *mut size_t addr; / address of the variable/array,
    pub /: *mut *mut int num; / number of variables in an array,
}
