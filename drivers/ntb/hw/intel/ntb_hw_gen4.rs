//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ntb/hw/intel/ntb_hw_gen4.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
// Copyright(c) 2020 Intel Corporation. All rights reserved.

// Supported PCI device revision range for ICX
pub const PCI_DEVICE_REVISION_ICX_MIN: c_uint = 0x2;
pub const PCI_DEVICE_REVISION_ICX_MAX: c_uint = 0xF;
// Intel Gen4 NTB hardware
// PCIe config space
pub const GEN4_IMBAR23SZ_OFFSET: c_uint = 0x00c4;
pub const GEN4_IMBAR45SZ_OFFSET: c_uint = 0x00c5;
pub const GEN4_EMBAR23SZ_OFFSET: c_uint = 0x00c6;
pub const GEN4_EMBAR45SZ_OFFSET: c_uint = 0x00c7;
pub const GEN4_DEVCTRL_OFFSET: c_uint = 0x0048;
pub const GEN4_DEVSTS_OFFSET: c_uint = 0x004a;
pub const GEN4_UNCERRSTS_OFFSET: c_uint = 0x0104;
pub const GEN4_CORERRSTS_OFFSET: c_uint = 0x0110;
// BAR0 MMIO
pub const GEN4_NTBCNTL_OFFSET: c_uint = 0x0000;
pub const GEN4_IM23XBASE_OFFSET: c_uint = 0x0010	/* IMBAR1XBASE */;
pub const GEN4_IM23XLMT_OFFSET: c_uint = 0x0018  /* IMBAR1XLMT */;
pub const GEN4_IM45XBASE_OFFSET: c_uint = 0x0020	/* IMBAR2XBASE */;
pub const GEN4_IM45XLMT_OFFSET: c_uint = 0x0028  /* IMBAR2XLMT */;
pub const GEN4_IM_INT_STATUS_OFFSET: c_uint = 0x0040;
pub const GEN4_IM_INT_DISABLE_OFFSET: c_uint = 0x0048;
pub const GEN4_INTVEC_OFFSET: c_uint = 0x0050  /* 0-32 vecs */;
pub const GEN4_IM23XBASEIDX_OFFSET: c_uint = 0x0074;
pub const GEN4_IM45XBASEIDX_OFFSET: c_uint = 0x0076;
pub const GEN4_IM_SPAD_OFFSET: c_uint = 0x0080  /* 0-15 SPADs */;
pub const GEN4_IM_SPAD_SEM_OFFSET: c_uint = 0x00c0	/* SPAD hw semaphore */;
pub const GEN4_IM_SPAD_STICKY_OFFSET: c_uint = 0x00c4  /* sticky SPAD */;
pub const GEN4_IM_DOORBELL_OFFSET: c_uint = 0x0100  /* 0-31 doorbells */;
pub const GEN4_LTR_SWSEL_OFFSET: c_uint = 0x30ec;
pub const GEN4_LTR_ACTIVE_OFFSET: c_uint = 0x30f0;
pub const GEN4_LTR_IDLE_OFFSET: c_uint = 0x30f4;
pub const GEN4_EM_SPAD_OFFSET: c_uint = 0x8080;
// note, link status is now in MMIO and not config space for NTB
pub const GEN4_LINK_CTRL_OFFSET: c_uint = 0xb050;
pub const GEN4_LINK_STATUS_OFFSET: c_uint = 0xb052;
pub const GEN4_PPD0_OFFSET: c_uint = 0xb0d4;
pub const GEN4_PPD1_OFFSET: c_uint = 0xb4c0;
pub const GEN4_LTSSMSTATEJMP: c_uint = 0xf040;
pub const GEN4_PPD_CLEAR_TRN: c_uint = 0x0001;
pub const GEN4_PPD_LINKTRN: c_uint = 0x0008;
pub const GEN4_PPD_CONN_MASK: c_uint = 0x0300;
pub const SPR_PPD_CONN_MASK: c_uint = 0x0700;
pub const GEN4_PPD_CONN_B2B: c_uint = 0x0200;
pub const GEN4_PPD_DEV_MASK: c_uint = 0x1000;
pub const GEN4_PPD_DEV_DSD: c_uint = 0x1000;
pub const GEN4_PPD_DEV_USD: c_uint = 0x0000;
pub const SPR_PPD_DEV_MASK: c_uint = 0x4000;
pub const SPR_PPD_DEV_DSD: c_uint = 0x4000;
pub const SPR_PPD_DEV_USD: c_uint = 0x0000;
pub const GEN4_LINK_CTRL_LINK_DISABLE: c_uint = 0x0010;
pub const GEN4_SLOTSTS: c_uint = 0xb05a;
pub const GEN4_SLOTSTS_DLLSCS: c_uint = 0x100;

pub const GEN4_DB_COUNT: c_int = 32;
pub const GEN4_DB_LINK: c_int = 32;

pub const GEN4_DB_MSIX_VECTOR_COUNT: c_int = 33;
pub const GEN4_DB_MSIX_VECTOR_SHIFT: c_int = 1;
pub const GEN4_DB_TOTAL_SHIFT: c_int = 33;
pub const GEN4_SPAD_COUNT: c_int = 16;
pub const NTB_CTL_E2I_BAR23_SNOOP: c_uint = 0x000004;
pub const NTB_CTL_E2I_BAR23_NOSNOOP: c_uint = 0x000008;
pub const NTB_CTL_I2E_BAR23_SNOOP: c_uint = 0x000010;
pub const NTB_CTL_I2E_BAR23_NOSNOOP: c_uint = 0x000020;
pub const NTB_CTL_E2I_BAR45_SNOOP: c_uint = 0x000040;
pub const NTB_CTL_E2I_BAR45_NOSNOO: c_uint = 0x000080;
pub const NTB_CTL_I2E_BAR45_SNOOP: c_uint = 0x000100;
pub const NTB_CTL_I2E_BAR45_NOSNOOP: c_uint = 0x000200;
pub const NTB_CTL_BUSNO_DIS_INC: c_uint = 0x000400;
pub const NTB_CTL_LINK_DOWN: c_uint = 0x010000;
pub const NTB_SJC_FORCEDETECT: c_uint = 0x000004;
pub const NTB_LTR_SWSEL_ACTIVE: c_uint = 0x0;
pub const NTB_LTR_SWSEL_IDLE: c_uint = 0x1;
pub const NTB_LTR_NS_SHIFT: c_int = 16;
pub const NTB_LTR_ACTIVE_VAL: c_uint = 0x0000  /* 0 us */;
pub const NTB_LTR_ACTIVE_LATSCALE: c_uint = 0x0800  /* 1us scale */;
pub const NTB_LTR_ACTIVE_REQMNT: c_uint = 0x8000  /* snoop req enable */;
pub const NTB_LTR_IDLE_VAL: c_uint = 0x0258  /* 600 us */;
pub const NTB_LTR_IDLE_LATSCALE: c_uint = 0x0800  /* 1us scale */;
pub const NTB_LTR_IDLE_REQMNT: c_uint = 0x8000  /* snoop req enable */;
pub const GEN6_PPD0_OFFSET: c_uint = 0xf0d4;
extern "C" {
    pub fn gen4_init_dev(ndev: *mut intel_ntb_dev) -> c_int;
}
