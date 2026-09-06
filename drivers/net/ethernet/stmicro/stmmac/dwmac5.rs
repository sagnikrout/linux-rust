//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac5.h
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
// Copyright (c) 2017 Synopsys, Inc. and/or its affiliates.
// stmmac Support for 5.xx Ethernet QoS cores
pub const MAC_DPP_FSM_INT_STATUS: c_uint = 0x00000140;
pub const MAC_AXI_SLV_DPE_ADDR_STATUS: c_uint = 0x00000144;
pub const MAC_FSM_CONTROL: c_uint = 0x00000148;

pub const MAC_PPS_CONTROL: c_uint = 0x00000b70;

pub const MTL_RXP_CONTROL_STATUS: c_uint = 0x00000ca0;

pub const MTL_RXP_IACC_CTRL_STATUS: c_uint = 0x00000cb0;

pub const MTL_RXP_IACC_DATA: c_uint = 0x00000cb4;
pub const MTL_ECC_CONTROL: c_uint = 0x00000cc0;

pub const MTL_SAFETY_INT_STATUS: c_uint = 0x00000cc4;

pub const MTL_ECC_INT_ENABLE: c_uint = 0x00000cc8;

pub const MTL_ECC_INT_STATUS: c_uint = 0x00000ccc;
pub const MTL_DPP_CONTROL: c_uint = 0x00000ce0;

pub const DMA_SAFETY_INT_STATUS: c_uint = 0x00001080;

pub const DMA_ECC_INT_ENABLE: c_uint = 0x00001084;

pub const DMA_ECC_INT_STATUS: c_uint = 0x00001088;
// EQoS version 5.xx VLAN Tag Filter Fail Packets Queuing
pub const GMAC_RXQ_CTRL4: c_uint = 0x00000094;

pub const GMAC_RXQCTRL_VFFQ_SHIFT: c_int = 17;

