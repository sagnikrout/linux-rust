//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/pcie_vdec0_brdg_ctrl_axuser_msix_nrm_regs.h
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
// PCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM
// (Prototype: AXUSER)
//
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_HB_ASID: c_uint = 0x4F03A00;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_HB_MMU_BP: c_uint = 0x4F03A04;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_HB_STRONG_ORDER: c_uint = 0x4F03A08;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_HB_NO_SNOOP: c_uint = 0x4F03A0C;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_HB_WR_REDUCTION: c_uint = 0x4F03A10;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_HB_RD_ATOMIC: c_uint = 0x4F03A14;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_HB_QOS: c_uint = 0x4F03A18;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_HB_RSVD: c_uint = 0x4F03A1C;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_HB_EMEM_CPAGE: c_uint = 0x4F03A20;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_HB_CORE: c_uint = 0x4F03A24;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_E2E_COORD: c_uint = 0x4F03A28;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_HB_WR_OVRD_LO: c_uint = 0x4F03A30;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_HB_WR_OVRD_HI: c_uint = 0x4F03A34;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_HB_RD_OVRD_LO: c_uint = 0x4F03A38;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_HB_RD_OVRD_HI: c_uint = 0x4F03A3C;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_LB_COORD: c_uint = 0x4F03A40;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_LB_LOCK: c_uint = 0x4F03A44;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_LB_RSVD: c_uint = 0x4F03A48;
pub const mmPCIE_VDEC0_BRDG_CTRL_AXUSER_MSIX_NRM_LB_OVRD: c_uint = 0x4F03A4C;
