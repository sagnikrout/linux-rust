//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/pmmu_pif_regs.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// PMMU_PIF
// (Prototype: PIF)
//
pub const mmPMMU_PIF_WR_CORE_CREDITS_THRESHOLD: c_uint = 0x4D03000;
pub const mmPMMU_PIF_RD_CORE_CREDITS_THRESHOLD: c_uint = 0x4D03004;
pub const mmPMMU_PIF_CORE_CREDITS_THRESHOLD: c_uint = 0x4D03008;
pub const mmPMMU_PIF_CORE_SEPARATION_DISABLE: c_uint = 0x4D0300C;
pub const mmPMMU_PIF_DISABLE_E2E_CREDITS: c_uint = 0x4D03010;
pub const mmPMMU_PIF_RATE_LIMITER_ENABLE: c_uint = 0x4D03014;
pub const mmPMMU_PIF_RATE_LIMITER_TOKEN_RESET: c_uint = 0x4D03018;
pub const mmPMMU_PIF_RATE_LIMITER_SATURATION: c_uint = 0x4D0301C;
pub const mmPMMU_PIF_RATE_LIMITER_TIMEOUT_LSB: c_uint = 0x4D03020;
pub const mmPMMU_PIF_RATE_LIMITER_TIMEOUT_MSB: c_uint = 0x4D03024;
pub const mmPMMU_PIF_ARB_TYPE: c_uint = 0x4D03028;
pub const mmPMMU_PIF_CLOCK_GATE_CONFIG: c_uint = 0x4D0302C;
pub const mmPMMU_PIF_CLOCK_GATE_ACTIVE: c_uint = 0x4D03030;
pub const mmPMMU_PIF_SPI_INTERRUPT_CAUSE: c_uint = 0x4D03034;
pub const mmPMMU_PIF_SPI_INTERRUPT_CAUSE_MASK: c_uint = 0x4D03038;
pub const mmPMMU_PIF_SPI_INTERRUPT_REG: c_uint = 0x4D0303C;
pub const mmPMMU_PIF_SPI_INTERRUPT_MASK: c_uint = 0x4D03040;
pub const mmPMMU_PIF_SEI_INTERRUPT_CAUSE: c_uint = 0x4D03044;
pub const mmPMMU_PIF_SEI_INTERRUPT_CAUSE_MASK: c_uint = 0x4D03048;
pub const mmPMMU_PIF_SEI_INTERRUPT_REG: c_uint = 0x4D0304C;
pub const mmPMMU_PIF_SEI_INTERRUPT_MASK: c_uint = 0x4D03050;
pub const mmPMMU_PIF_DEBUG_BUFFER_CNT_CTRL: c_uint = 0x4D03054;
pub const mmPMMU_PIF_DEBUG_WR_BUF_CNT: c_uint = 0x4D03058;
pub const mmPMMU_PIF_DEBUG_RD_BUF_CNT: c_uint = 0x4D0305C;
pub const mmPMMU_PIF_DEBUG_WR_CORE_BUF_CNT: c_uint = 0x4D03060;
pub const mmPMMU_PIF_DEBUG_RD_CORE_BUF_CNT: c_uint = 0x4D03070;
pub const mmPMMU_PIF_DEBUG_WR_BUF_FULL: c_uint = 0x4D03080;
pub const mmPMMU_PIF_DEBUG_RD_BUF_FULL: c_uint = 0x4D03084;
pub const mmPMMU_PIF_E2E_ROUTING_CFG: c_uint = 0x4D03090;
pub const mmPMMU_PIF_E2E_ROUTING_CFG2: c_uint = 0x4D03094;
pub const mmPMMU_PIF_SPI_INTERRUPT_CLEAR: c_uint = 0x4D03100;
pub const mmPMMU_PIF_SEI_INTERRUPT_CLEAR: c_uint = 0x4D03104;
pub const mmPMMU_PIF_BASE_ADDR_PMMU: c_uint = 0x4D03200;
pub const mmPMMU_PIF_ADDR_MASK_PMMU: c_uint = 0x4D03204;
pub const mmPMMU_PIF_BASE_ADDR_PCI0: c_uint = 0x4D03208;
pub const mmPMMU_PIF_ADDR_MASK_PCI0: c_uint = 0x4D0320C;
pub const mmPMMU_PIF_BASE_ADDR_PCI2: c_uint = 0x4D03210;
pub const mmPMMU_PIF_ADDR_MASK_PCI1: c_uint = 0x4D03214;
pub const mmPMMU_PIF_BASE_ADDR_PCI1: c_uint = 0x4D03218;
pub const mmPMMU_PIF_ADDR_MASK_PCI2: c_uint = 0x4D0321C;
pub const mmPMMU_PIF_BASE_ADDR_TPC: c_uint = 0x4D03220;
pub const mmPMMU_PIF_ADDR_MASK_TPC: c_uint = 0x4D03224;
pub const mmPMMU_PIF_BASE_ADDR_DEC0: c_uint = 0x4D03228;
pub const mmPMMU_PIF_ADDR_MASK_DEC0: c_uint = 0x4D0322C;
pub const mmPMMU_PIF_BASE_ADDR_DEC1: c_uint = 0x4D03230;
pub const mmPMMU_PIF_ADDR_MASK_DEC1: c_uint = 0x4D03234;
pub const mmPMMU_PIF_PMMU_DBG_BASE_ADDR: c_uint = 0x4D03300;
pub const mmPMMU_PIF_PMMU_DBG_ADDR_MASK: c_uint = 0x4D03304;
pub const mmPMMU_PIF_PCI_DBG_BASE_ADDR: c_uint = 0x4D03308;
pub const mmPMMU_PIF_PCI_DBG_ADDR_MASK: c_uint = 0x4D0330C;
pub const mmPMMU_PIF_DEC0_DBG_BASE_ADDR: c_uint = 0x4D03310;
pub const mmPMMU_PIF_DEC0_DBG_ADDR_MASK: c_uint = 0x4D03314;
pub const mmPMMU_PIF_DEC1_DBG_BASE_ADDR: c_uint = 0x4D03318;
pub const mmPMMU_PIF_DEC1_DBG_ADDR_MASK: c_uint = 0x4D0331C;
pub const mmPMMU_PIF_TPC_DBG_BASE_ADDR: c_uint = 0x4D03320;
pub const mmPMMU_PIF_TPC_DBG_ADDR_MASK: c_uint = 0x4D03324;
