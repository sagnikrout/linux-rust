//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/mt6359/registers.h
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
// PMIC Registers
pub const MT6359_SWCID: c_uint = 0xa;
pub const MT6359_TOPSTATUS: c_uint = 0x2a;
pub const MT6359_TOP_RST_MISC: c_uint = 0x14c;
pub const MT6359_MISC_TOP_INT_CON0: c_uint = 0x188;
pub const MT6359_MISC_TOP_INT_STATUS0: c_uint = 0x194;
pub const MT6359_TOP_INT_STATUS0: c_uint = 0x19e;
pub const MT6359_SCK_TOP_INT_CON0: c_uint = 0x528;
pub const MT6359_SCK_TOP_INT_STATUS0: c_uint = 0x534;
pub const MT6359_EOSC_CALI_CON0: c_uint = 0x53a;
pub const MT6359_EOSC_CALI_CON1: c_uint = 0x53c;
pub const MT6359_RTC_MIX_CON0: c_uint = 0x53e;
pub const MT6359_RTC_MIX_CON1: c_uint = 0x540;
pub const MT6359_RTC_MIX_CON2: c_uint = 0x542;
pub const MT6359_RTC_DSN_ID: c_uint = 0x580;
pub const MT6359_RTC_DSN_REV0: c_uint = 0x582;
pub const MT6359_RTC_DBI: c_uint = 0x584;
pub const MT6359_RTC_DXI: c_uint = 0x586;
pub const MT6359_RTC_BBPU: c_uint = 0x588;
pub const MT6359_RTC_IRQ_STA: c_uint = 0x58a;
pub const MT6359_RTC_IRQ_EN: c_uint = 0x58c;
pub const MT6359_RTC_CII_EN: c_uint = 0x58e;
pub const MT6359_RTC_AL_MASK: c_uint = 0x590;
pub const MT6359_RTC_TC_SEC: c_uint = 0x592;
pub const MT6359_RTC_TC_MIN: c_uint = 0x594;
pub const MT6359_RTC_TC_HOU: c_uint = 0x596;
pub const MT6359_RTC_TC_DOM: c_uint = 0x598;
pub const MT6359_RTC_TC_DOW: c_uint = 0x59a;
pub const MT6359_RTC_TC_MTH: c_uint = 0x59c;
pub const MT6359_RTC_TC_YEA: c_uint = 0x59e;
pub const MT6359_RTC_AL_SEC: c_uint = 0x5a0;
pub const MT6359_RTC_AL_MIN: c_uint = 0x5a2;
pub const MT6359_RTC_AL_HOU: c_uint = 0x5a4;
pub const MT6359_RTC_AL_DOM: c_uint = 0x5a6;
pub const MT6359_RTC_AL_DOW: c_uint = 0x5a8;
pub const MT6359_RTC_AL_MTH: c_uint = 0x5aa;
pub const MT6359_RTC_AL_YEA: c_uint = 0x5ac;
pub const MT6359_RTC_OSC32CON: c_uint = 0x5ae;
pub const MT6359_RTC_POWERKEY1: c_uint = 0x5b0;
pub const MT6359_RTC_POWERKEY2: c_uint = 0x5b2;
pub const MT6359_RTC_PDN1: c_uint = 0x5b4;
pub const MT6359_RTC_PDN2: c_uint = 0x5b6;
pub const MT6359_RTC_SPAR0: c_uint = 0x5b8;
pub const MT6359_RTC_SPAR1: c_uint = 0x5ba;
pub const MT6359_RTC_PROT: c_uint = 0x5bc;
pub const MT6359_RTC_DIFF: c_uint = 0x5be;
pub const MT6359_RTC_CALI: c_uint = 0x5c0;
pub const MT6359_RTC_WRTGR: c_uint = 0x5c2;
pub const MT6359_RTC_CON: c_uint = 0x5c4;
pub const MT6359_RTC_SEC_CTRL: c_uint = 0x5c6;
pub const MT6359_RTC_INT_CNT: c_uint = 0x5c8;
pub const MT6359_RTC_SEC_DAT0: c_uint = 0x5ca;
pub const MT6359_RTC_SEC_DAT1: c_uint = 0x5cc;
pub const MT6359_RTC_SEC_DAT2: c_uint = 0x5ce;
pub const MT6359_RTC_SEC_DSN_ID: c_uint = 0x600;
pub const MT6359_RTC_SEC_DSN_REV0: c_uint = 0x602;
pub const MT6359_RTC_SEC_DBI: c_uint = 0x604;
pub const MT6359_RTC_SEC_DXI: c_uint = 0x606;
pub const MT6359_RTC_TC_SEC_SEC: c_uint = 0x608;
pub const MT6359_RTC_TC_MIN_SEC: c_uint = 0x60a;
pub const MT6359_RTC_TC_HOU_SEC: c_uint = 0x60c;
pub const MT6359_RTC_TC_DOM_SEC: c_uint = 0x60e;
pub const MT6359_RTC_TC_DOW_SEC: c_uint = 0x610;
pub const MT6359_RTC_TC_MTH_SEC: c_uint = 0x612;
pub const MT6359_RTC_TC_YEA_SEC: c_uint = 0x614;
pub const MT6359_RTC_SEC_CK_PDN: c_uint = 0x616;
pub const MT6359_RTC_SEC_WRTGR: c_uint = 0x618;
pub const MT6359_PSC_TOP_INT_CON0: c_uint = 0x910;
pub const MT6359_PSC_TOP_INT_STATUS0: c_uint = 0x91c;
pub const MT6359_BM_TOP_INT_CON0: c_uint = 0xc32;
pub const MT6359_BM_TOP_INT_CON1: c_uint = 0xc38;
pub const MT6359_BM_TOP_INT_STATUS0: c_uint = 0xc4a;
pub const MT6359_BM_TOP_INT_STATUS1: c_uint = 0xc4c;
pub const MT6359_HK_TOP_INT_CON0: c_uint = 0xf92;
pub const MT6359_HK_TOP_INT_STATUS0: c_uint = 0xf9e;
pub const MT6359_BUCK_TOP_INT_CON0: c_uint = 0x1418;
pub const MT6359_BUCK_TOP_INT_STATUS0: c_uint = 0x1424;
pub const MT6359_BUCK_VPU_CON0: c_uint = 0x1488;
pub const MT6359_BUCK_VPU_DBG0: c_uint = 0x14a6;
pub const MT6359_BUCK_VPU_DBG1: c_uint = 0x14a8;
pub const MT6359_BUCK_VPU_ELR0: c_uint = 0x14ac;
pub const MT6359_BUCK_VCORE_CON0: c_uint = 0x1508;
pub const MT6359_BUCK_VCORE_DBG0: c_uint = 0x1526;
pub const MT6359_BUCK_VCORE_DBG1: c_uint = 0x1528;
pub const MT6359_BUCK_VCORE_SSHUB_CON0: c_uint = 0x152a;
pub const MT6359_BUCK_VCORE_ELR0: c_uint = 0x1534;
pub const MT6359_BUCK_VGPU11_CON0: c_uint = 0x1588;
pub const MT6359_BUCK_VGPU11_DBG0: c_uint = 0x15a6;
pub const MT6359_BUCK_VGPU11_DBG1: c_uint = 0x15a8;
pub const MT6359_BUCK_VGPU11_ELR0: c_uint = 0x15ac;
pub const MT6359_BUCK_VMODEM_CON0: c_uint = 0x1688;
pub const MT6359_BUCK_VMODEM_DBG0: c_uint = 0x16a6;
pub const MT6359_BUCK_VMODEM_DBG1: c_uint = 0x16a8;
pub const MT6359_BUCK_VMODEM_ELR0: c_uint = 0x16ae;
pub const MT6359_BUCK_VPROC1_CON0: c_uint = 0x1708;
pub const MT6359_BUCK_VPROC1_DBG0: c_uint = 0x1726;
pub const MT6359_BUCK_VPROC1_DBG1: c_uint = 0x1728;
pub const MT6359_BUCK_VPROC1_ELR0: c_uint = 0x172e;
pub const MT6359_BUCK_VPROC2_CON0: c_uint = 0x1788;
pub const MT6359_BUCK_VPROC2_DBG0: c_uint = 0x17a6;
pub const MT6359_BUCK_VPROC2_DBG1: c_uint = 0x17a8;
pub const MT6359_BUCK_VPROC2_ELR0: c_uint = 0x17b2;
pub const MT6359_BUCK_VS1_CON0: c_uint = 0x1808;
pub const MT6359_BUCK_VS1_DBG0: c_uint = 0x1826;
pub const MT6359_BUCK_VS1_DBG1: c_uint = 0x1828;
pub const MT6359_BUCK_VS1_ELR0: c_uint = 0x1834;
pub const MT6359_BUCK_VS2_CON0: c_uint = 0x1888;
pub const MT6359_BUCK_VS2_DBG0: c_uint = 0x18a6;
pub const MT6359_BUCK_VS2_DBG1: c_uint = 0x18a8;
pub const MT6359_BUCK_VS2_ELR0: c_uint = 0x18b4;
pub const MT6359_BUCK_VPA_CON0: c_uint = 0x1908;
pub const MT6359_BUCK_VPA_CON1: c_uint = 0x190e;
pub const MT6359_BUCK_VPA_CFG0: c_uint = 0x1910;
pub const MT6359_BUCK_VPA_CFG1: c_uint = 0x1912;
pub const MT6359_BUCK_VPA_DBG0: c_uint = 0x1914;
pub const MT6359_BUCK_VPA_DBG1: c_uint = 0x1916;
pub const MT6359_VGPUVCORE_ANA_CON2: c_uint = 0x198e;
pub const MT6359_VGPUVCORE_ANA_CON13: c_uint = 0x19a4;
pub const MT6359_VPROC1_ANA_CON3: c_uint = 0x19b2;
pub const MT6359_VPROC2_ANA_CON3: c_uint = 0x1a0e;
pub const MT6359_VMODEM_ANA_CON3: c_uint = 0x1a1a;
pub const MT6359_VPU_ANA_CON3: c_uint = 0x1a26;
pub const MT6359_VS1_ANA_CON0: c_uint = 0x1a2c;
pub const MT6359_VS2_ANA_CON0: c_uint = 0x1a34;
pub const MT6359_VPA_ANA_CON0: c_uint = 0x1a3c;
pub const MT6359_LDO_TOP_INT_CON0: c_uint = 0x1b14;
pub const MT6359_LDO_TOP_INT_CON1: c_uint = 0x1b1a;
pub const MT6359_LDO_TOP_INT_STATUS0: c_uint = 0x1b28;
pub const MT6359_LDO_TOP_INT_STATUS1: c_uint = 0x1b2a;
pub const MT6359_LDO_VSRAM_PROC1_ELR: c_uint = 0x1b40;
pub const MT6359_LDO_VSRAM_PROC2_ELR: c_uint = 0x1b42;
pub const MT6359_LDO_VSRAM_OTHERS_ELR: c_uint = 0x1b44;
pub const MT6359_LDO_VSRAM_MD_ELR: c_uint = 0x1b46;
pub const MT6359_LDO_VFE28_CON0: c_uint = 0x1b88;
pub const MT6359_LDO_VFE28_MON: c_uint = 0x1b8a;
pub const MT6359_LDO_VXO22_CON0: c_uint = 0x1b98;
pub const MT6359_LDO_VXO22_MON: c_uint = 0x1b9a;
pub const MT6359_LDO_VRF18_CON0: c_uint = 0x1ba8;
pub const MT6359_LDO_VRF18_MON: c_uint = 0x1baa;
pub const MT6359_LDO_VRF12_CON0: c_uint = 0x1bb8;
pub const MT6359_LDO_VRF12_MON: c_uint = 0x1bba;
pub const MT6359_LDO_VEFUSE_CON0: c_uint = 0x1bc8;
pub const MT6359_LDO_VEFUSE_MON: c_uint = 0x1bca;
pub const MT6359_LDO_VCN33_1_CON0: c_uint = 0x1bd8;
pub const MT6359_LDO_VCN33_1_MON: c_uint = 0x1bda;
pub const MT6359_LDO_VCN33_1_MULTI_SW: c_uint = 0x1be8;
pub const MT6359_LDO_VCN33_2_CON0: c_uint = 0x1c08;
pub const MT6359_LDO_VCN33_2_MON: c_uint = 0x1c0a;
pub const MT6359_LDO_VCN33_2_MULTI_SW: c_uint = 0x1c18;
pub const MT6359_LDO_VCN13_CON0: c_uint = 0x1c1a;
pub const MT6359_LDO_VCN13_MON: c_uint = 0x1c1c;
pub const MT6359_LDO_VCN18_CON0: c_uint = 0x1c2a;
pub const MT6359_LDO_VCN18_MON: c_uint = 0x1c2c;
pub const MT6359_LDO_VA09_CON0: c_uint = 0x1c3a;
pub const MT6359_LDO_VA09_MON: c_uint = 0x1c3c;
pub const MT6359_LDO_VCAMIO_CON0: c_uint = 0x1c4a;
pub const MT6359_LDO_VCAMIO_MON: c_uint = 0x1c4c;
pub const MT6359_LDO_VA12_CON0: c_uint = 0x1c5a;
pub const MT6359_LDO_VA12_MON: c_uint = 0x1c5c;
pub const MT6359_LDO_VAUX18_CON0: c_uint = 0x1c88;
pub const MT6359_LDO_VAUX18_MON: c_uint = 0x1c8a;
pub const MT6359_LDO_VAUD18_CON0: c_uint = 0x1c98;
pub const MT6359_LDO_VAUD18_MON: c_uint = 0x1c9a;
pub const MT6359_LDO_VIO18_CON0: c_uint = 0x1ca8;
pub const MT6359_LDO_VIO18_MON: c_uint = 0x1caa;
pub const MT6359_LDO_VEMC_CON0: c_uint = 0x1cb8;
pub const MT6359_LDO_VEMC_MON: c_uint = 0x1cba;
pub const MT6359_LDO_VSIM1_CON0: c_uint = 0x1cc8;
pub const MT6359_LDO_VSIM1_MON: c_uint = 0x1cca;
pub const MT6359_LDO_VSIM2_CON0: c_uint = 0x1cd8;
pub const MT6359_LDO_VSIM2_MON: c_uint = 0x1cda;
pub const MT6359_LDO_VUSB_CON0: c_uint = 0x1d08;
pub const MT6359_LDO_VUSB_MON: c_uint = 0x1d0a;
pub const MT6359_LDO_VUSB_MULTI_SW: c_uint = 0x1d18;
pub const MT6359_LDO_VRFCK_CON0: c_uint = 0x1d1a;
pub const MT6359_LDO_VRFCK_MON: c_uint = 0x1d1c;
pub const MT6359_LDO_VBBCK_CON0: c_uint = 0x1d2a;
pub const MT6359_LDO_VBBCK_MON: c_uint = 0x1d2c;
pub const MT6359_LDO_VBIF28_CON0: c_uint = 0x1d3a;
pub const MT6359_LDO_VBIF28_MON: c_uint = 0x1d3c;
pub const MT6359_LDO_VIBR_CON0: c_uint = 0x1d4a;
pub const MT6359_LDO_VIBR_MON: c_uint = 0x1d4c;
pub const MT6359_LDO_VIO28_CON0: c_uint = 0x1d5a;
pub const MT6359_LDO_VIO28_MON: c_uint = 0x1d5c;
pub const MT6359_LDO_VM18_CON0: c_uint = 0x1d88;
pub const MT6359_LDO_VM18_MON: c_uint = 0x1d8a;
pub const MT6359_LDO_VUFS_CON0: c_uint = 0x1d98;
pub const MT6359_LDO_VUFS_MON: c_uint = 0x1d9a;
pub const MT6359_LDO_VSRAM_PROC1_CON0: c_uint = 0x1e88;
pub const MT6359_LDO_VSRAM_PROC1_MON: c_uint = 0x1e8a;
pub const MT6359_LDO_VSRAM_PROC1_VOSEL1: c_uint = 0x1e8e;
pub const MT6359_LDO_VSRAM_PROC2_CON0: c_uint = 0x1ea6;
pub const MT6359_LDO_VSRAM_PROC2_MON: c_uint = 0x1ea8;
pub const MT6359_LDO_VSRAM_PROC2_VOSEL1: c_uint = 0x1eac;
pub const MT6359_LDO_VSRAM_OTHERS_CON0: c_uint = 0x1f08;
pub const MT6359_LDO_VSRAM_OTHERS_MON: c_uint = 0x1f0a;
pub const MT6359_LDO_VSRAM_OTHERS_VOSEL1: c_uint = 0x1f0e;
pub const MT6359_LDO_VSRAM_OTHERS_SSHUB: c_uint = 0x1f26;
pub const MT6359_LDO_VSRAM_MD_CON0: c_uint = 0x1f2c;
pub const MT6359_LDO_VSRAM_MD_MON: c_uint = 0x1f2e;
pub const MT6359_LDO_VSRAM_MD_VOSEL1: c_uint = 0x1f32;
pub const MT6359_VFE28_ANA_CON0: c_uint = 0x1f88;
pub const MT6359_VAUX18_ANA_CON0: c_uint = 0x1f8c;
pub const MT6359_VUSB_ANA_CON0: c_uint = 0x1f90;
pub const MT6359_VBIF28_ANA_CON0: c_uint = 0x1f94;
pub const MT6359_VCN33_1_ANA_CON0: c_uint = 0x1f98;
pub const MT6359_VCN33_2_ANA_CON0: c_uint = 0x1f9c;
pub const MT6359_VEMC_ANA_CON0: c_uint = 0x1fa0;
pub const MT6359_VSIM1_ANA_CON0: c_uint = 0x1fa4;
pub const MT6359_VSIM2_ANA_CON0: c_uint = 0x1fa8;
pub const MT6359_VIO28_ANA_CON0: c_uint = 0x1fac;
pub const MT6359_VIBR_ANA_CON0: c_uint = 0x1fb0;
pub const MT6359_VRF18_ANA_CON0: c_uint = 0x2008;
pub const MT6359_VEFUSE_ANA_CON0: c_uint = 0x200c;
pub const MT6359_VCN18_ANA_CON0: c_uint = 0x2010;
pub const MT6359_VCAMIO_ANA_CON0: c_uint = 0x2014;
pub const MT6359_VAUD18_ANA_CON0: c_uint = 0x2018;
pub const MT6359_VIO18_ANA_CON0: c_uint = 0x201c;
pub const MT6359_VM18_ANA_CON0: c_uint = 0x2020;
pub const MT6359_VUFS_ANA_CON0: c_uint = 0x2024;
pub const MT6359_VRF12_ANA_CON0: c_uint = 0x202a;
pub const MT6359_VCN13_ANA_CON0: c_uint = 0x202e;
pub const MT6359_VA09_ANA_CON0: c_uint = 0x2032;
pub const MT6359_VA12_ANA_CON0: c_uint = 0x2036;
pub const MT6359_VXO22_ANA_CON0: c_uint = 0x2088;
pub const MT6359_VRFCK_ANA_CON0: c_uint = 0x208c;
pub const MT6359_VBBCK_ANA_CON0: c_uint = 0x2094;
pub const MT6359_AUD_TOP_INT_CON0: c_uint = 0x2328;
pub const MT6359_AUD_TOP_INT_STATUS0: c_uint = 0x2334;

pub const MT6359_RG_BUCK_VPU_LP_SHIFT: c_int = 1;

pub const MT6359_DA_VPU_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_DA_VPU_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VPU_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_RG_BUCK_VPU_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VCORE_LP_SHIFT: c_int = 1;

pub const MT6359_DA_VCORE_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_DA_VCORE_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VCORE_SSHUB_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_RG_BUCK_VCORE_SSHUB_VOSEL_SHIFT: c_int = 4;

pub const MT6359_RG_BUCK_VCORE_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_RG_BUCK_VCORE_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VGPU11_LP_SHIFT: c_int = 1;

pub const MT6359_DA_VGPU11_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_DA_VGPU11_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VGPU11_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_RG_BUCK_VGPU11_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VMODEM_LP_SHIFT: c_int = 1;

pub const MT6359_DA_VMODEM_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_DA_VMODEM_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VMODEM_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_RG_BUCK_VMODEM_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VPROC1_LP_SHIFT: c_int = 1;

pub const MT6359_DA_VPROC1_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_DA_VPROC1_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VPROC1_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_RG_BUCK_VPROC1_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VPROC2_LP_SHIFT: c_int = 1;

pub const MT6359_DA_VPROC2_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_DA_VPROC2_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VPROC2_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_RG_BUCK_VPROC2_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VS1_LP_SHIFT: c_int = 1;

pub const MT6359_DA_VS1_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_DA_VS1_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VS1_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_RG_BUCK_VS1_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VS2_LP_SHIFT: c_int = 1;

pub const MT6359_DA_VS2_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_DA_VS2_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VS2_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_RG_BUCK_VS2_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_BUCK_VPA_LP_SHIFT: c_int = 1;

pub const MT6359_RG_BUCK_VPA_VOSEL_MASK: c_uint = 0x3F;
pub const MT6359_RG_BUCK_VPA_VOSEL_SHIFT: c_int = 0;

pub const MT6359_DA_VPA_VOSEL_MASK: c_uint = 0x3F;
pub const MT6359_DA_VPA_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_VGPU11_FCCM_SHIFT: c_int = 9;

pub const MT6359_RG_VCORE_FCCM_SHIFT: c_int = 5;

pub const MT6359_RG_VPROC1_FCCM_SHIFT: c_int = 1;

pub const MT6359_RG_VPROC2_FCCM_SHIFT: c_int = 1;

pub const MT6359_RG_VMODEM_FCCM_SHIFT: c_int = 1;

pub const MT6359_RG_VPU_FCCM_SHIFT: c_int = 1;

pub const MT6359_RG_VS1_FPWM_SHIFT: c_int = 3;

pub const MT6359_RG_VS2_FPWM_SHIFT: c_int = 3;

pub const MT6359_RG_VPA_MODESET_SHIFT: c_int = 1;

pub const MT6359_RG_LDO_VSRAM_PROC1_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_RG_LDO_VSRAM_PROC1_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VSRAM_PROC2_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_RG_LDO_VSRAM_PROC2_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VSRAM_OTHERS_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_RG_LDO_VSRAM_OTHERS_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VSRAM_MD_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_RG_LDO_VSRAM_MD_VOSEL_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VXO22_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VRF18_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VRF12_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VEFUSE_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VCN33_1_EN_0_MASK: c_uint = 0x1;
pub const MT6359_RG_LDO_VCN33_1_EN_0_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VCN33_1_EN_1_SHIFT: c_int = 15;

pub const MT6359_RG_LDO_VCN33_2_EN_0_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VCN33_2_EN_1_MASK: c_uint = 0x1;
pub const MT6359_RG_LDO_VCN33_2_EN_1_SHIFT: c_int = 15;

pub const MT6359_RG_LDO_VCN13_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VA09_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VCAMIO_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VA12_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VIO18_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VEMC_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VSIM1_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VSIM2_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VUSB_EN_0_MASK: c_uint = 0x1;
pub const MT6359_RG_LDO_VUSB_EN_0_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VUSB_EN_1_MASK: c_uint = 0x1;
pub const MT6359_RG_LDO_VUSB_EN_1_SHIFT: c_int = 15;

pub const MT6359_RG_LDO_VRFCK_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VBBCK_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VIBR_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VIO28_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VM18_EN_SHIFT: c_int = 0;

pub const MT6359_RG_LDO_VUFS_EN_SHIFT: c_int = 0;

pub const MT6359_DA_VSRAM_PROC1_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_DA_VSRAM_PROC1_VOSEL_SHIFT: c_int = 8;

pub const MT6359_DA_VSRAM_PROC2_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_DA_VSRAM_PROC2_VOSEL_SHIFT: c_int = 8;

pub const MT6359_DA_VSRAM_OTHERS_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_DA_VSRAM_OTHERS_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_LDO_VSRAM_OTHERS_SSHUB_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_RG_LDO_VSRAM_OTHERS_SSHUB_VOSEL_SHIFT: c_int = 1;

pub const MT6359_DA_VSRAM_MD_VOSEL_MASK: c_uint = 0x7F;
pub const MT6359_DA_VSRAM_MD_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VCN33_1_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VCN33_1_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VCN33_2_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VCN33_2_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VEMC_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VEMC_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VSIM1_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VSIM1_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VSIM2_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VSIM2_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VIO28_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VIO28_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VIBR_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VIBR_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VRF18_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VRF18_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VEFUSE_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VEFUSE_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VCAMIO_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VCAMIO_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VIO18_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VIO18_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VM18_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VM18_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VUFS_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VUFS_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VRF12_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VRF12_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VCN13_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VCN13_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VA09_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VA09_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VA12_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VA12_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VXO22_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VXO22_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VRFCK_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VRFCK_VOSEL_SHIFT: c_int = 8;

pub const MT6359_RG_VBBCK_VOSEL_MASK: c_uint = 0xF;
pub const MT6359_RG_VBBCK_VOSEL_SHIFT: c_int = 8;
