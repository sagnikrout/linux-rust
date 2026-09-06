//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/google/gve/gve_register.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// Google virtual Ethernet (gve) driver
//
// Copyright (C) 2015-2019 Google, Inc.
//
// Fixed Configuration Registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_registers {
    pub device_status: __be32,
    pub driver_status: __be32,
    pub max_tx_queues: __be32,
    pub max_rx_queues: __be32,
    pub adminq_pfn: __be32,
    pub adminq_doorbell: __be32,
    pub adminq_event_counter: __be32,
    pub reserved: [u8; 3],
    pub driver_version: u8,
    pub adminq_base_address_hi: __be32,
    pub adminq_base_address_lo: __be32,
    pub adminq_length: __be16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_device_status_flags {
    GVE_DEVICE_STATUS_RESET_MASK		= BIT(1),
    GVE_DEVICE_STATUS_LINK_STATUS_MASK	= BIT(2),
    GVE_DEVICE_STATUS_REPORT_STATS_MASK	= BIT(3),
    GVE_DEVICE_STATUS_DEVICE_IS_RESET	= BIT(4),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_driver_status_flags {
    GVE_DRIVER_STATUS_RUN_MASK		= BIT(0),
    GVE_DRIVER_STATUS_RESET_MASK		= BIT(1),
}
