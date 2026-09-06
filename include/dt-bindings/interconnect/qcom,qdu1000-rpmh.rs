//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interconnect/qcom,qdu1000-rpmh.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause
//
// Copyright (c) 2022, Qualcomm Innovation Center, Inc. All rights reserved.
//
pub const MASTER_QUP_CORE_0: c_int = 0;
pub const MASTER_QUP_CORE_1: c_int = 1;
pub const SLAVE_QUP_CORE_0: c_int = 2;
pub const SLAVE_QUP_CORE_1: c_int = 3;
pub const MASTER_SYS_TCU: c_int = 0;
pub const MASTER_APPSS_PROC: c_int = 1;
pub const MASTER_GEMNOC_ECPRI_DMA: c_int = 2;
pub const MASTER_FEC_2_GEMNOC: c_int = 3;
pub const MASTER_ANOC_PCIE_GEM_NOC: c_int = 4;
pub const MASTER_SNOC_GC_MEM_NOC: c_int = 5;
pub const MASTER_SNOC_SF_MEM_NOC: c_int = 6;
pub const MASTER_MSS_PROC: c_int = 7;
pub const SLAVE_GEM_NOC_CNOC: c_int = 8;
pub const SLAVE_LLCC: c_int = 9;
pub const SLAVE_GEMNOC_MODEM_CNOC: c_int = 10;
pub const SLAVE_MEM_NOC_PCIE_SNOC: c_int = 11;
pub const MASTER_LLCC: c_int = 0;
pub const SLAVE_EBI1: c_int = 1;
pub const MASTER_GIC_AHB: c_int = 0;
pub const MASTER_QDSS_BAM: c_int = 1;
pub const MASTER_QPIC: c_int = 2;
pub const MASTER_QSPI_0: c_int = 3;
pub const MASTER_QUP_0: c_int = 4;
pub const MASTER_QUP_1: c_int = 5;
pub const MASTER_SNOC_CFG: c_int = 6;
pub const MASTER_ANOC_SNOC: c_int = 7;
pub const MASTER_ANOC_GSI: c_int = 8;
pub const MASTER_GEM_NOC_CNOC: c_int = 9;
pub const MASTER_GEMNOC_MODEM_CNOC: c_int = 10;
pub const MASTER_GEM_NOC_PCIE_SNOC: c_int = 11;
pub const MASTER_CRYPTO: c_int = 12;
pub const MASTER_ECPRI_GSI: c_int = 13;
pub const MASTER_PIMEM: c_int = 14;
pub const MASTER_SNOC_ECPRI_DMA: c_int = 15;
pub const MASTER_GIC: c_int = 16;
pub const MASTER_PCIE: c_int = 17;
pub const MASTER_QDSS_ETR: c_int = 18;
pub const MASTER_QDSS_ETR_1: c_int = 19;
pub const MASTER_SDCC_1: c_int = 20;
pub const MASTER_USB3: c_int = 21;
pub const SLAVE_AHB2PHY_SOUTH: c_int = 22;
pub const SLAVE_AHB2PHY_NORTH: c_int = 23;
pub const SLAVE_AHB2PHY_EAST: c_int = 24;
pub const SLAVE_AOSS: c_int = 25;
pub const SLAVE_CLK_CTL: c_int = 26;
pub const SLAVE_RBCPR_CX_CFG: c_int = 27;
pub const SLAVE_RBCPR_MX_CFG: c_int = 28;
pub const SLAVE_CRYPTO_0_CFG: c_int = 29;
pub const SLAVE_ECPRI_CFG: c_int = 30;
pub const SLAVE_IMEM_CFG: c_int = 31;
pub const SLAVE_IPC_ROUTER_CFG: c_int = 32;
pub const SLAVE_CNOC_MSS: c_int = 33;
pub const SLAVE_PCIE_CFG: c_int = 34;
pub const SLAVE_PDM: c_int = 35;
pub const SLAVE_PIMEM_CFG: c_int = 36;
pub const SLAVE_PRNG: c_int = 37;
pub const SLAVE_QDSS_CFG: c_int = 38;
pub const SLAVE_QPIC: c_int = 40;
pub const SLAVE_QSPI_0: c_int = 41;
pub const SLAVE_QUP_0: c_int = 42;
pub const SLAVE_QUP_1: c_int = 43;
pub const SLAVE_SDCC_2: c_int = 44;
pub const SLAVE_SMBUS_CFG: c_int = 45;
pub const SLAVE_SNOC_CFG: c_int = 46;
pub const SLAVE_TCSR: c_int = 47;
pub const SLAVE_TLMM: c_int = 48;
pub const SLAVE_TME_CFG: c_int = 49;
pub const SLAVE_TSC_CFG: c_int = 50;
pub const SLAVE_USB3_0: c_int = 51;
pub const SLAVE_VSENSE_CTRL_CFG: c_int = 52;
pub const SLAVE_A1NOC_SNOC: c_int = 53;
pub const SLAVE_ANOC_SNOC_GSI: c_int = 54;
pub const SLAVE_DDRSS_CFG: c_int = 55;
pub const SLAVE_ECPRI_GEMNOC: c_int = 56;
pub const SLAVE_SNOC_GEM_NOC_GC: c_int = 57;
pub const SLAVE_SNOC_GEM_NOC_SF: c_int = 58;
pub const SLAVE_MODEM_OFFLINE: c_int = 59;
pub const SLAVE_ANOC_PCIE_GEM_NOC: c_int = 60;
pub const SLAVE_IMEM: c_int = 61;
pub const SLAVE_PIMEM: c_int = 62;
pub const SLAVE_SERVICE_SNOC: c_int = 63;
pub const SLAVE_ETHERNET_SS: c_int = 64;
pub const SLAVE_PCIE_0: c_int = 65;
pub const SLAVE_QDSS_STM: c_int = 66;
pub const SLAVE_TCU: c_int = 67;
