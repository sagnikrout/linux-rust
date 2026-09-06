//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/cn66xx_device.h
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


//
// Author: Cavium, Inc.
//
// Contact: support@cavium.com
// Please include "LiquidIO" in the subject.
//
// Copyright (c) 2003-2016 Cavium, Inc.
//
// This file is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License, Version 2, as
// published by the Free Software Foundation.
//
// This file is distributed in the hope that it will be useful, but
// AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
// NONINFRINGEMENT.  See the GNU General Public License for more details.
//
// ! \file  cn66xx_device.h
// \brief Host Driver: Routines that perform CN66XX specific operations.
//
// Register address and configuration for a CN6XXX devices.
// If device specific changes need to be made then add a struct to include
// device specific fields as shown in the commented section
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_cn6xxx {
// PCI interrupt summary register
    pub intr_sum_reg64: *mut u8 __iomem,
// PCI interrupt enable register
    pub intr_enb_reg64: *mut u8 __iomem,
// The PCI interrupt mask used by interrupt handler
    pub intr_mask64: u64,
    pub conf: *mut octeon_config,
// Example additional fields - not used currently
// struct {
// }cn6xyz;
//
// For the purpose of atomic access to interrupt enable reg
    pub lock_for_droq_int_enb_reg: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octeon_pcie_mps {
    PCIE_MPS_DEFAULT = -1,	/* Use the default setup by BIOS */
    PCIE_MPS_128B = 0,
    PCIE_MPS_256B = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octeon_pcie_mrrs {
    PCIE_MRRS_DEFAULT = -1,	/* Use the default setup by BIOS */
    PCIE_MRRS_128B = 0,
    PCIE_MRRS_256B = 1,
    PCIE_MRRS_512B = 2,
    PCIE_MRRS_1024B = 3,
    PCIE_MRRS_2048B = 4,
    PCIE_MRRS_4096B = 5
}

// Common functions for 66xx and 68xx
extern "C" {
    pub fn lio_cn6xxx_soft_reset(oct: *mut octeon_device) -> c_int;
}
extern "C" {
    pub fn lio_cn6xxx_enable_error_reporting(oct: *mut octeon_device);
}
extern "C" {
    pub fn lio_cn6xxx_setup_global_input_regs(oct: *mut octeon_device);
}
extern "C" {
    pub fn lio_cn6xxx_setup_global_output_regs(oct: *mut octeon_device);
}
extern "C" {
    pub fn lio_cn6xxx_setup_iq_regs(oct: *mut octeon_device, iq_no: u32);
}
extern "C" {
    pub fn lio_cn6xxx_setup_oq_regs(oct: *mut octeon_device, oq_no: u32);
}
extern "C" {
    pub fn lio_cn6xxx_enable_io_queues(oct: *mut octeon_device) -> c_int;
}
extern "C" {
    pub fn lio_cn6xxx_disable_io_queues(oct: *mut octeon_device);
}
extern "C" {
    pub fn lio_cn6xxx_process_interrupt_regs(dev: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn lio_cn6xxx_bar1_idx_write(oct: *mut octeon_device, idx: u32, mask: u32);
}
extern "C" {
    pub fn lio_cn6xxx_bar1_idx_read(oct: *mut octeon_device, idx: u32) -> u32;
}
extern "C" {
    pub fn lio_cn6xxx_enable_interrupt(oct: *mut octeon_device, unused: u8);
}
extern "C" {
    pub fn lio_cn6xxx_disable_interrupt(oct: *mut octeon_device, unused: u8);
}
extern "C" {
    pub fn lio_cn6xxx_coprocessor_clock(oct: *mut octeon_device) -> u32;
}
extern "C" {
    pub fn lio_cn6xxx_get_oq_ticks(oct: *mut octeon_device, time_intr_in_us: u32) -> u32;
}
extern "C" {
    pub fn lio_setup_cn66xx_octeon_device(oct: *mut octeon_device) -> c_int;
}
