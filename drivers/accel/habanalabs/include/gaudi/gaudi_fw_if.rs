//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi/gaudi_fw_if.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2019-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
pub const GAUDI_EVENT_QUEUE_MSI_IDX: c_int = 8;
pub const GAUDI_NIC_PORT1_MSI_IDX: c_int = 10;
pub const GAUDI_NIC_PORT3_MSI_IDX: c_int = 12;
pub const GAUDI_NIC_PORT5_MSI_IDX: c_int = 14;
pub const GAUDI_NIC_PORT7_MSI_IDX: c_int = 16;
pub const GAUDI_NIC_PORT9_MSI_IDX: c_int = 18;
pub const UBOOT_FW_OFFSET: c_uint = 0x100000	/* 1MB in SRAM */;
pub const LINUX_FW_OFFSET: c_uint = 0x800000	/* 8MB in HBM */;
// HBM thermal delta in [Deg] added to composite (CTemp)
pub const HBM_TEMP_ADJUST_COEFF: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi_nic_axi_error {
    RXB,
    RXE,
    TXS,
    TXE,
    QPC_RESP,
    NON_AXI_ERR,
    TMR,
}

//
// struct eq_nic_sei_event - describes an AXI error cause.
// @axi_error_cause: one of the events defined in enum gaudi_nic_axi_error.
// @id: can be either 0 or 1, to further describe unit with interrupt cause
// (i.e. TXE0 or TXE1).
// @pad[6]: padding structure to 64bit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eq_nic_sei_event {
    pub axi_error_cause: __u8,
    pub id: __u8,
    pub pad: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gaudi_cold_rst_data {
    pub 1: u32 spsram_init_done :,
    pub 31: u32 reserved :,
}

