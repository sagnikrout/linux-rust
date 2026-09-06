//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/mt6359p/registers.h
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
// Copyright (c) 2021 MediaTek Inc.
//
pub const MT6359P_CHIP_VER: c_uint = 0x5930;
// PMIC Registers
pub const MT6359P_HWCID: c_uint = 0x8;
pub const MT6359P_TOP_TRAP: c_uint = 0x50;
pub const MT6359P_TOP_TMA_KEY: c_uint = 0x3a8;
pub const MT6359P_BUCK_VCORE_ELR_NUM: c_uint = 0x152a;
pub const MT6359P_BUCK_VCORE_ELR0: c_uint = 0x152c;
pub const MT6359P_BUCK_VGPU11_SSHUB_CON0: c_uint = 0x15aa;
pub const MT6359P_BUCK_VGPU11_ELR0: c_uint = 0x15b4;
pub const MT6359P_LDO_VSRAM_PROC1_ELR: c_uint = 0x1b44;
pub const MT6359P_LDO_VSRAM_PROC2_ELR: c_uint = 0x1b46;
pub const MT6359P_LDO_VSRAM_OTHERS_ELR: c_uint = 0x1b48;
pub const MT6359P_LDO_VSRAM_MD_ELR: c_uint = 0x1b4a;
pub const MT6359P_LDO_VEMC_ELR_0: c_uint = 0x1b4c;
pub const MT6359P_LDO_VFE28_CON0: c_uint = 0x1b88;
pub const MT6359P_LDO_VFE28_MON: c_uint = 0x1b8c;
pub const MT6359P_LDO_VXO22_CON0: c_uint = 0x1b9a;
pub const MT6359P_LDO_VXO22_MON: c_uint = 0x1b9e;
pub const MT6359P_LDO_VRF18_CON0: c_uint = 0x1bac;
pub const MT6359P_LDO_VRF18_MON: c_uint = 0x1bb0;
pub const MT6359P_LDO_VRF12_CON0: c_uint = 0x1bbe;
pub const MT6359P_LDO_VRF12_MON: c_uint = 0x1bc2;
pub const MT6359P_LDO_VEFUSE_CON0: c_uint = 0x1bd0;
pub const MT6359P_LDO_VEFUSE_MON: c_uint = 0x1bd4;
pub const MT6359P_LDO_VCN33_1_CON0: c_uint = 0x1be2;
pub const MT6359P_LDO_VCN33_1_MON: c_uint = 0x1be6;
pub const MT6359P_LDO_VCN33_1_MULTI_SW: c_uint = 0x1bf4;
pub const MT6359P_LDO_VCN33_2_CON0: c_uint = 0x1c08;
pub const MT6359P_LDO_VCN33_2_MON: c_uint = 0x1c0c;
pub const MT6359P_LDO_VCN33_2_MULTI_SW: c_uint = 0x1c1a;
pub const MT6359P_LDO_VCN13_CON0: c_uint = 0x1c1c;
pub const MT6359P_LDO_VCN13_MON: c_uint = 0x1c20;
pub const MT6359P_LDO_VCN18_CON0: c_uint = 0x1c2e;
pub const MT6359P_LDO_VCN18_MON: c_uint = 0x1c32;
pub const MT6359P_LDO_VA09_CON0: c_uint = 0x1c40;
pub const MT6359P_LDO_VA09_MON: c_uint = 0x1c44;
pub const MT6359P_LDO_VCAMIO_CON0: c_uint = 0x1c52;
pub const MT6359P_LDO_VCAMIO_MON: c_uint = 0x1c56;
pub const MT6359P_LDO_VA12_CON0: c_uint = 0x1c64;
pub const MT6359P_LDO_VA12_MON: c_uint = 0x1c68;
pub const MT6359P_LDO_VAUX18_CON0: c_uint = 0x1c88;
pub const MT6359P_LDO_VAUX18_MON: c_uint = 0x1c8c;
pub const MT6359P_LDO_VAUD18_CON0: c_uint = 0x1c9a;
pub const MT6359P_LDO_VAUD18_MON: c_uint = 0x1c9e;
pub const MT6359P_LDO_VIO18_CON0: c_uint = 0x1cac;
pub const MT6359P_LDO_VIO18_MON: c_uint = 0x1cb0;
pub const MT6359P_LDO_VEMC_CON0: c_uint = 0x1cbe;
pub const MT6359P_LDO_VEMC_MON: c_uint = 0x1cc2;
pub const MT6359P_LDO_VSIM1_CON0: c_uint = 0x1cd0;
pub const MT6359P_LDO_VSIM1_MON: c_uint = 0x1cd4;
pub const MT6359P_LDO_VSIM2_CON0: c_uint = 0x1ce2;
pub const MT6359P_LDO_VSIM2_MON: c_uint = 0x1ce6;
pub const MT6359P_LDO_VUSB_CON0: c_uint = 0x1d08;
pub const MT6359P_LDO_VUSB_MON: c_uint = 0x1d0c;
pub const MT6359P_LDO_VUSB_MULTI_SW: c_uint = 0x1d1a;
pub const MT6359P_LDO_VRFCK_CON0: c_uint = 0x1d1c;
pub const MT6359P_LDO_VRFCK_MON: c_uint = 0x1d20;
pub const MT6359P_LDO_VBBCK_CON0: c_uint = 0x1d2e;
pub const MT6359P_LDO_VBBCK_MON: c_uint = 0x1d32;
pub const MT6359P_LDO_VBIF28_CON0: c_uint = 0x1d40;
pub const MT6359P_LDO_VBIF28_MON: c_uint = 0x1d44;
pub const MT6359P_LDO_VIBR_CON0: c_uint = 0x1d52;
pub const MT6359P_LDO_VIBR_MON: c_uint = 0x1d56;
pub const MT6359P_LDO_VIO28_CON0: c_uint = 0x1d64;
pub const MT6359P_LDO_VIO28_MON: c_uint = 0x1d68;
pub const MT6359P_LDO_VM18_CON0: c_uint = 0x1d88;
pub const MT6359P_LDO_VM18_MON: c_uint = 0x1d8c;
pub const MT6359P_LDO_VUFS_CON0: c_uint = 0x1d9a;
pub const MT6359P_LDO_VUFS_MON: c_uint = 0x1d9e;
pub const MT6359P_LDO_VSRAM_PROC1_CON0: c_uint = 0x1e88;
pub const MT6359P_LDO_VSRAM_PROC1_MON: c_uint = 0x1e8c;
pub const MT6359P_LDO_VSRAM_PROC1_VOSEL1: c_uint = 0x1e90;
pub const MT6359P_LDO_VSRAM_PROC2_CON0: c_uint = 0x1ea8;
pub const MT6359P_LDO_VSRAM_PROC2_MON: c_uint = 0x1eac;
pub const MT6359P_LDO_VSRAM_PROC2_VOSEL1: c_uint = 0x1eb0;
pub const MT6359P_LDO_VSRAM_OTHERS_CON0: c_uint = 0x1f08;
pub const MT6359P_LDO_VSRAM_OTHERS_MON: c_uint = 0x1f0c;
pub const MT6359P_LDO_VSRAM_OTHERS_VOSEL1: c_uint = 0x1f10;
pub const MT6359P_LDO_VSRAM_OTHERS_SSHUB: c_uint = 0x1f28;
pub const MT6359P_LDO_VSRAM_MD_CON0: c_uint = 0x1f2e;
pub const MT6359P_LDO_VSRAM_MD_MON: c_uint = 0x1f32;
pub const MT6359P_LDO_VSRAM_MD_VOSEL1: c_uint = 0x1f36;
pub const MT6359P_VFE28_ANA_CON0: c_uint = 0x1f88;
pub const MT6359P_VAUX18_ANA_CON0: c_uint = 0x1f8c;
pub const MT6359P_VUSB_ANA_CON0: c_uint = 0x1f90;
pub const MT6359P_VBIF28_ANA_CON0: c_uint = 0x1f94;
pub const MT6359P_VCN33_1_ANA_CON0: c_uint = 0x1f98;
pub const MT6359P_VCN33_2_ANA_CON0: c_uint = 0x1f9c;
pub const MT6359P_VEMC_ANA_CON0: c_uint = 0x1fa0;
pub const MT6359P_VSIM1_ANA_CON0: c_uint = 0x1fa2;
pub const MT6359P_VSIM2_ANA_CON0: c_uint = 0x1fa6;
pub const MT6359P_VIO28_ANA_CON0: c_uint = 0x1faa;
pub const MT6359P_VIBR_ANA_CON0: c_uint = 0x1fae;
pub const MT6359P_VFE28_ELR_4: c_uint = 0x1fc0;
pub const MT6359P_VRF18_ANA_CON0: c_uint = 0x2008;
pub const MT6359P_VEFUSE_ANA_CON0: c_uint = 0x200c;
pub const MT6359P_VCN18_ANA_CON0: c_uint = 0x2010;
pub const MT6359P_VCAMIO_ANA_CON0: c_uint = 0x2014;
pub const MT6359P_VAUD18_ANA_CON0: c_uint = 0x2018;
pub const MT6359P_VIO18_ANA_CON0: c_uint = 0x201c;
pub const MT6359P_VM18_ANA_CON0: c_uint = 0x2020;
pub const MT6359P_VUFS_ANA_CON0: c_uint = 0x2024;
pub const MT6359P_VRF12_ANA_CON0: c_uint = 0x202a;
pub const MT6359P_VCN13_ANA_CON0: c_uint = 0x202e;
pub const MT6359P_VA09_ANA_CON0: c_uint = 0x2032;
pub const MT6359P_VRF18_ELR_3: c_uint = 0x204e;
pub const MT6359P_VXO22_ANA_CON0: c_uint = 0x2088;
pub const MT6359P_VRFCK_ANA_CON0: c_uint = 0x208c;
pub const MT6359P_VBBCK_ANA_CON0: c_uint = 0x2096;

pub const MT6359P_RG_BUCK_VGPU11_SSHUB_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359P_RG_BUCK_VGPU11_SSHUB_VOSEL_SHIFT: c_int = 4;

pub const MT6359P_RG_LDO_VEMC_VOSEL_0_MASK: c_uint = 0xF;
pub const MT6359P_RG_LDO_VEMC_VOSEL_0_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VXO22_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VRF18_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VRF12_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VEFUSE_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VCN33_1_EN_1_SHIFT: c_int = 15;

pub const MT6359P_RG_LDO_VCN33_2_EN_0_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VCN13_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VA09_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VCAMIO_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VA12_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VIO18_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VEMC_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VSIM1_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VSIM2_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VRFCK_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VBBCK_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VIBR_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VIO28_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VM18_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_LDO_VUFS_EN_SHIFT: c_int = 0;

pub const MT6359P_RG_VBBCK_VOSEL_MASK: c_uint = 0xF;
pub const MT6359P_RG_VBBCK_VOSEL_SHIFT: c_int = 4;

pub const TMA_KEY: c_uint = 0x9CA6;
