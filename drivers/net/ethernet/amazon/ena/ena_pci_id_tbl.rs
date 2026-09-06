//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amazon/ena/ena_pci_id_tbl.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright 2015-2020 Amazon.com, Inc. or its affiliates. All rights reserved.
//

pub const PCI_VENDOR_ID_AMAZON: c_uint = 0x1d0f;

pub const PCI_DEV_ID_ENA_PF: c_uint = 0x0ec2;

pub const PCI_DEV_ID_ENA_LLQ_PF: c_uint = 0x1ec2;

pub const PCI_DEV_ID_ENA_VF: c_uint = 0xec20;

pub const PCI_DEV_ID_ENA_LLQ_VF: c_uint = 0xec21;

pub const PCI_DEV_ID_ENA_RESRV0: c_uint = 0x0051;

