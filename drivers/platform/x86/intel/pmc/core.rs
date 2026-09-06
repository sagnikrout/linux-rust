//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/intel/pmc/core.h
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
// Intel Core SoC Power Management Controller Header File
//
// Copyright (c) 2016, Intel Corporation.
// All Rights Reserved.
//
// Authors: Rajneesh Bhardwaj <rajneesh.bhardwaj@intel.com>
// Vishwanath Somayaji <vishwanath.somayaji@intel.com>
//

pub const PMC_BASE_ADDR_DEFAULT: c_uint = 0xFE000000;
pub const MAX_NUM_PMC: c_int = 3;
pub const S0IX_BLK_SIZE: c_int = 4;
// PCH query
pub const LPM_HEADER_OFFSET: c_int = 1;
pub const LPM_REG_COUNT: c_int = 28;
pub const LPM_MODE_OFFSET: c_int = 1;
// Sunrise Point Power Management Controller PCI Device ID
pub const SPT_PMC_PCI_DEVICE_ID: c_uint = 0x9d21;
pub const SPT_PMC_BASE_ADDR_OFFSET: c_uint = 0x48;
pub const SPT_PMC_SLP_S0_RES_COUNTER_OFFSET: c_uint = 0x13c;
pub const SPT_PMC_PM_CFG_OFFSET: c_uint = 0x18;
pub const SPT_PMC_PM_STS_OFFSET: c_uint = 0x1c;
pub const SPT_PMC_MTPMC_OFFSET: c_uint = 0x20;
pub const SPT_PMC_MFPMC_OFFSET: c_uint = 0x38;
pub const SPT_PMC_LTR_IGNORE_OFFSET: c_uint = 0x30C;
pub const SPT_PMC_VRIC1_OFFSET: c_uint = 0x31c;
pub const SPT_PMC_MPHY_CORE_STS_0: c_uint = 0x1143;
pub const SPT_PMC_MPHY_CORE_STS_1: c_uint = 0x1142;
pub const SPT_PMC_MPHY_COM_STS_0: c_uint = 0x1155;
pub const SPT_PMC_MMIO_REG_LEN: c_uint = 0x1000;
pub const SPT_PMC_SLP_S0_RES_COUNTER_STEP: c_uint = 0x68;

pub const MTPMC_MASK: c_uint = 0xffff0000;
pub const PPFEAR_MAX_NUM_ENTRIES: c_int = 13;
pub const SPT_PPFEAR_NUM_ENTRIES: c_int = 5;
pub const SPT_PMC_READ_DISABLE_BIT: c_uint = 0x16;
pub const SPT_PMC_MSG_FULL_STS_BIT: c_uint = 0x18;
pub const NUM_RETRIES: c_int = 100;
pub const SPT_NUM_IP_IGN_ALLOWED: c_int = 17;
pub const SPT_PMC_LTR_CUR_PLT: c_uint = 0x350;
pub const SPT_PMC_LTR_CUR_ASLT: c_uint = 0x354;
pub const SPT_PMC_LTR_SPA: c_uint = 0x360;
pub const SPT_PMC_LTR_SPB: c_uint = 0x364;
pub const SPT_PMC_LTR_SATA: c_uint = 0x368;
pub const SPT_PMC_LTR_GBE: c_uint = 0x36C;
pub const SPT_PMC_LTR_XHCI: c_uint = 0x370;
pub const SPT_PMC_LTR_RESERVED: c_uint = 0x374;
pub const SPT_PMC_LTR_ME: c_uint = 0x378;
pub const SPT_PMC_LTR_EVA: c_uint = 0x37C;
pub const SPT_PMC_LTR_SPC: c_uint = 0x380;
pub const SPT_PMC_LTR_AZ: c_uint = 0x384;
pub const SPT_PMC_LTR_LPSS: c_uint = 0x38C;
pub const SPT_PMC_LTR_CAM: c_uint = 0x390;
pub const SPT_PMC_LTR_SPD: c_uint = 0x394;
pub const SPT_PMC_LTR_SPE: c_uint = 0x398;
pub const SPT_PMC_LTR_ESPI: c_uint = 0x39C;
pub const SPT_PMC_LTR_SCC: c_uint = 0x3A0;
pub const SPT_PMC_LTR_ISH: c_uint = 0x3A4;
// Sunrise Point: PGD PFET Enable Ack Status Registers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ppfear_regs {
    SPT_PMC_XRAM_PPFEAR0A = 0x590,
    SPT_PMC_XRAM_PPFEAR0B,
    SPT_PMC_XRAM_PPFEAR0C,
    SPT_PMC_XRAM_PPFEAR0D,
    SPT_PMC_XRAM_PPFEAR1A,
}

// Cannonlake Power Management Controller register offsets
pub const CNP_PMC_SLPS0_DBG_OFFSET: c_uint = 0x10B4;
pub const CNP_PMC_PM_CFG_OFFSET: c_uint = 0x1818;
pub const CNP_PMC_SLP_S0_RES_COUNTER_OFFSET: c_uint = 0x193C;
pub const CNP_PMC_LTR_IGNORE_OFFSET: c_uint = 0x1B0C;
// Cannonlake: PGD PFET Enable Ack Status Register(s) start
pub const CNP_PMC_HOST_PPFEAR0A: c_uint = 0x1D90;

pub const CNP_PMC_MMIO_REG_LEN: c_uint = 0x2000;
pub const CNP_PPFEAR_NUM_ENTRIES: c_int = 8;
pub const CNP_PMC_READ_DISABLE_BIT: c_int = 22;
pub const CNP_NUM_IP_IGN_ALLOWED: c_int = 19;
pub const CNP_PMC_LTR_CUR_PLT: c_uint = 0x1B50;
pub const CNP_PMC_LTR_CUR_ASLT: c_uint = 0x1B54;
pub const CNP_PMC_LTR_SPA: c_uint = 0x1B60;
pub const CNP_PMC_LTR_SPB: c_uint = 0x1B64;
pub const CNP_PMC_LTR_SATA: c_uint = 0x1B68;
pub const CNP_PMC_LTR_GBE: c_uint = 0x1B6C;
pub const CNP_PMC_LTR_XHCI: c_uint = 0x1B70;
pub const CNP_PMC_LTR_RESERVED: c_uint = 0x1B74;
pub const CNP_PMC_LTR_ME: c_uint = 0x1B78;
pub const CNP_PMC_LTR_EVA: c_uint = 0x1B7C;
pub const CNP_PMC_LTR_SPC: c_uint = 0x1B80;
pub const CNP_PMC_LTR_AZ: c_uint = 0x1B84;
pub const CNP_PMC_LTR_LPSS: c_uint = 0x1B8C;
pub const CNP_PMC_LTR_CAM: c_uint = 0x1B90;
pub const CNP_PMC_LTR_SPD: c_uint = 0x1B94;
pub const CNP_PMC_LTR_SPE: c_uint = 0x1B98;
pub const CNP_PMC_LTR_ESPI: c_uint = 0x1B9C;
pub const CNP_PMC_LTR_SCC: c_uint = 0x1BA0;
pub const CNP_PMC_LTR_ISH: c_uint = 0x1BA4;
pub const CNP_PMC_LTR_CNV: c_uint = 0x1BF0;
pub const CNP_PMC_LTR_EMMC: c_uint = 0x1BF4;
pub const CNP_PMC_LTR_UFSX2: c_uint = 0x1BF8;

pub const ICL_PPFEAR_NUM_ENTRIES: c_int = 9;
pub const ICL_NUM_IP_IGN_ALLOWED: c_int = 20;
pub const ICL_PMC_LTR_WIGIG: c_uint = 0x1BFC;
pub const ICL_PMC_SLP_S0_RES_COUNTER_STEP: c_uint = 0x64;
pub const LPM_MAX_NUM_MODES: c_int = 8;

pub const TGL_PMC_SLP_S0_RES_COUNTER_STEP: c_uint = 0x7A;
pub const TGL_PMC_LTR_THC0: c_uint = 0x1C04;
pub const TGL_PMC_LTR_THC1: c_uint = 0x1C08;
pub const TGL_NUM_IP_IGN_ALLOWED: c_int = 23;

pub const ADL_PMC_LTR_SPF: c_uint = 0x1C00;
pub const ADL_NUM_IP_IGN_ALLOWED: c_int = 23;
pub const ADL_PMC_SLP_S0_RES_COUNTER_OFFSET: c_uint = 0x1098;
//
// Tigerlake Power Management Controller register offsets
//
pub const TGL_LPM_STS_LATCH_EN_OFFSET: c_uint = 0x1C34;
pub const TGL_LPM_EN_OFFSET: c_uint = 0x1C78;
pub const TGL_LPM_RESIDENCY_OFFSET: c_uint = 0x1C80;
// Tigerlake Low Power Mode debug registers
pub const TGL_LPM_STATUS_OFFSET: c_uint = 0x1C3C;
pub const TGL_LPM_LIVE_STATUS_OFFSET: c_uint = 0x1C5C;
pub const TGL_LPM_PRI_OFFSET: c_uint = 0x1C7C;
pub const TGL_LPM_NUM_MAPS: c_int = 6;
// Tigerlake PSON residency register
pub const TGL_PSON_RESIDENCY_OFFSET: c_uint = 0x18f8;
pub const TGL_PSON_RES_COUNTER_STEP: c_uint = 0x7A;
// Extended Test Mode Register 3 (CNL and later)
pub const ETR3_OFFSET: c_uint = 0x1048;

// Extended Test Mode Register LPM bits (TGL and later

// Alder Lake Power Management Controller register offsets
pub const ADL_LPM_EN_OFFSET: c_uint = 0x179C;
pub const ADL_LPM_RESIDENCY_OFFSET: c_uint = 0x17A4;
pub const ADL_LPM_NUM_MODES: c_int = 2;
pub const ADL_LPM_NUM_MAPS: c_int = 14;
// Alder Lake Low Power Mode debug registers
pub const ADL_LPM_STATUS_OFFSET: c_uint = 0x170C;
pub const ADL_LPM_PRI_OFFSET: c_uint = 0x17A0;
pub const ADL_LPM_STATUS_LATCH_EN_OFFSET: c_uint = 0x1704;
pub const ADL_LPM_LIVE_STATUS_OFFSET: c_uint = 0x1764;
// Meteor Lake Power Management Controller register offsets
pub const MTL_LPM_EN_OFFSET: c_uint = 0x1798;
pub const MTL_LPM_RESIDENCY_OFFSET: c_uint = 0x17A0;
// Meteor Lake Low Power Mode debug registers
pub const MTL_LPM_PRI_OFFSET: c_uint = 0x179C;
pub const MTL_LPM_STATUS_LATCH_EN_OFFSET: c_uint = 0x16F8;
pub const MTL_LPM_STATUS_OFFSET: c_uint = 0x1700;
pub const MTL_LPM_LIVE_STATUS_OFFSET: c_uint = 0x175C;
pub const MTL_PMC_LTR_IOE_PMC: c_uint = 0x1C0C;
pub const MTL_PMC_LTR_ESE: c_uint = 0x1BAC;
pub const MTL_PMC_LTR_RESERVED: c_uint = 0x1BA4;
pub const MTL_IOE_PMC_MMIO_REG_LEN: c_uint = 0x23A4;
pub const MTL_SOCM_NUM_IP_IGN_ALLOWED: c_int = 25;
pub const MTL_SOC_PMC_MMIO_REG_LEN: c_uint = 0x2708;
pub const MTL_PMC_LTR_SPG: c_uint = 0x1B74;
pub const ARL_SOCS_PMC_LTR_RESERVED: c_uint = 0x1B88;
pub const ARL_SOCS_NUM_IP_IGN_ALLOWED: c_int = 26;
pub const ARL_PMC_LTR_DMI3: c_uint = 0x1BE4;
pub const ARL_PCH_PMC_MMIO_REG_LEN: c_uint = 0x2720;
// Meteor Lake PGD PFET Enable Ack Status
pub const MTL_SOCM_PPFEAR_NUM_ENTRIES: c_int = 8;
pub const MTL_IOE_PPFEAR_NUM_ENTRIES: c_int = 10;
pub const ARL_SOCS_PPFEAR_NUM_ENTRIES: c_int = 9;
// Die C6 from PUNIT telemetry
pub const MTL_PMT_DMU_DIE_C6_OFFSET: c_int = 15;
pub const MTL_PMT_DMU_GUID: c_uint = 0x1A067102;
pub const ARL_PMT_DMU_GUID: c_uint = 0x1A06A102;
pub const ARL_H_PMT_DMU_GUID: c_uint = 0x1A06A101;
pub const LNL_PMC_MMIO_REG_LEN: c_uint = 0x2708;
pub const LNL_PMC_LTR_OSSE: c_uint = 0x1B88;
pub const LNL_NUM_IP_IGN_ALLOWED: c_int = 27;
pub const LNL_PPFEAR_NUM_ENTRIES: c_int = 12;
pub const LNL_S0IX_BLOCKER_OFFSET: c_uint = 0x2004;
// Panther Lake Power Management Controller register offsets
pub const PTL_LPM_NUM_MAPS: c_int = 14;
pub const PTL_PMC_LTR_SATA2: c_uint = 0x1B90;
pub const PTL_PMC_LTR_PMC: c_uint = 0x1BA8;
pub const PTL_PMC_LTR_CUR_ASLT: c_uint = 0x1C28;
pub const PTL_PMC_LTR_CUR_PLT: c_uint = 0x1C2C;
pub const PTL_PCD_PMC_MMIO_REG_LEN: c_uint = 0x31A8;
pub const PTL_NUM_S0IX_BLOCKER: c_int = 106;
pub const PTL_BLK_REQ_OFFSET: c_int = 55;
// Wildcat Lake
pub const WCL_PMC_LTR_RESERVED: c_uint = 0x1B64;
pub const WCL_PCD_PMC_MMIO_REG_LEN: c_uint = 0x3178;
pub const WCL_NUM_S0IX_BLOCKER: c_int = 94;
pub const WCL_BLK_REQ_OFFSET: c_int = 50;
// Nova Lake
pub const NVL_PCDH_PPFEAR_NUM_ENTRIES: c_int = 13;
pub const NVL_PCDH_PMC_MMIO_REG_LEN: c_uint = 0x363c;
pub const NVL_PCDS_PMC_MMIO_REG_LEN: c_uint = 0x3118;
pub const NVL_PCHS_PMC_MMIO_REG_LEN: c_uint = 0x30d8;
pub const NVL_LPM_PRI_OFFSET: c_uint = 0x17a4;
pub const NVL_LPM_EN_OFFSET: c_uint = 0x17a0;
pub const NVL_LPM_RESIDENCY_OFFSET: c_uint = 0x17a8;
pub const NVL_LPM_LIVE_STATUS_OFFSET: c_uint = 0x1760;
pub const NVL_LPM_NUM_MAPS: c_int = 15;
pub const NVL_PCDH_NUM_S0IX_BLOCKER: c_int = 107;
pub const NVL_PCDS_NUM_S0IX_BLOCKER: c_int = 71;
pub const NVL_PCHS_NUM_S0IX_BLOCKER: c_int = 54;
pub const NVL_PCDS_PMC_LTR_RESERVED: c_uint = 0x1bac;
pub const NVL_PCDH_BLK_REQ_OFFSET: c_int = 53;
pub const NVL_PCDS_BLK_REQ_OFFSET: c_int = 18;
pub const NVL_PCHS_BLK_REQ_OFFSET: c_int = 46;
pub const NVL_PMT_PC_GUID: c_uint = 0x13000101;
pub const NVL_PMT_DMU_GUID: c_uint = 0x1a000101;
pub const NVL_LTR_BLK_OFFSET: c_int = 64;
pub const NVL_PKGC_BLK_OFFSET: c_int = 4;
pub const NVL_PMT_DMU_DIE_C6_OFFSET: c_int = 25;
// SSRAM PMC Device ID
// LNL
pub const PMC_DEVID_LNL_SOCM: c_uint = 0xa87f;
// PTL
pub const PMC_DEVID_PTL_PCDH: c_uint = 0xe37f;
pub const PMC_DEVID_PTL_PCDP: c_uint = 0xe47f;
// WCL
pub const PMC_DEVID_WCL_PCDN: c_uint = 0x4d7f;
// ARL
pub const PMC_DEVID_ARL_SOCM: c_uint = 0x777f;
pub const PMC_DEVID_ARL_SOCS: c_uint = 0xae7f;
pub const PMC_DEVID_ARL_IOEP: c_uint = 0x7ecf;
pub const PMC_DEVID_ARL_PCHS: c_uint = 0x7f27;
// MTL
pub const PMC_DEVID_MTL_SOCM: c_uint = 0x7e7f;
pub const PMC_DEVID_MTL_IOEP: c_uint = 0x7ecf;
pub const PMC_DEVID_MTL_IOEM: c_uint = 0x7ebf;
// NVL
pub const PMC_DEVID_NVL_PCDH: c_uint = 0xd37e;
pub const PMC_DEVID_NVL_PCDS: c_uint = 0xd47e;
pub const PMC_DEVID_NVL_PCHS: c_uint = 0x6e27;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_bit_map {
    pub name: *const c_char,
    pub bit_mask: u32,
    pub blk: u8,
}

//
// struct pmc_reg_map - Structure used to define parameter unique to a
// @pfear_sts:		Maps name of IP block to PPFEAR* bit
// @mphy_sts:		Maps name of MPHY lane to MPHY status lane status bit
// @pll_sts:		Maps name of PLL to corresponding bit status
// @slps0_dbg_maps:	Array of SLP_S0_DBG* registers containing debug info
// @ltr_show_sts:	Maps PCH IP Names to their MMIO register offsets
// @s0ix_blocker_maps:	Maps name of IP block to S0ix blocker counter
// @slp_s0_offset:	PWRMBASE offset to read SLP_S0 residency
// @ltr_ignore_offset:	PWRMBASE offset to read/write LTR ignore bit
// @regmap_length:	Length of memory to map from PWRMBASE address to access
// @ppfear0_offset:	PWRMBASE offset to read PPFEAR
// @ppfear_buckets:	Number of 8 bits blocks to read all IP blocks from
// PPFEAR
// @pm_cfg_offset:	PWRMBASE offset to PM_CFG register
// @pm_read_disable_bit: Bit index to read PMC_READ_DISABLE
// @slps0_dbg_offset:	PWRMBASE offset to SLP_S0_DEBUG_REG
// @s0ix_blocker_offset PWRMBASE offset to S0ix blocker counter
// @num_s0ix_blocker:	Number of S0ix blockers
// @blocker_req_offset:	Telemetry offset to S0ix blocker low power mode substate requirement table
// @lpm_req_guid:	Telemetry GUID to read low power mode substate requirement table
//
// Each PCH has unique set of register offsets and bit indexes. This structure
// captures them to have a common implementation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_reg_map {
    pub pfear_sts: *const pmc_bit_map,
    pub mphy_sts: *const pmc_bit_map,
    pub pll_sts: *const pmc_bit_map,
    pub slps0_dbg_maps: *const pmc_bit_map,
    pub ltr_show_sts: *const pmc_bit_map,
    pub msr_sts: *const pmc_bit_map,
    pub lpm_sts: *const pmc_bit_map,
    pub s0ix_blocker_maps: *const pmc_bit_map,
    pub slp_s0_offset: u32,
    pub slp_s0_res_counter_step: c_int,
    pub ltr_ignore_offset: u32,
    pub regmap_length: c_int,
    pub ppfear0_offset: u32,
    pub ppfear_buckets: c_int,
    pub pm_cfg_offset: u32,
    pub pm_read_disable_bit: c_int,
    pub slps0_dbg_offset: u32,
    pub ltr_ignore_max: u32,
    pub pm_vric1_offset: u32,
    pub s0ix_blocker_offset: u32,
    pub num_s0ix_blocker: u32,
    pub blocker_req_offset: u32,
// Low Power Mode registers
    pub lpm_num_maps: c_int,
    pub lpm_num_modes: c_int,
    pub lpm_res_counter_step_x2: c_int,
    pub lpm_sts_latch_en_offset: u32,
    pub lpm_en_offset: u32,
    pub lpm_priority_offset: u32,
    pub lpm_residency_offset: u32,
    pub lpm_status_offset: u32,
    pub lpm_live_status_offset: u32,
    pub etr3_offset: u32,
    pub lpm_reg_index: *const u8,
    pub pson_residency_offset: u32,
    pub pson_residency_counter_step: u32,
// GUID for telemetry regions
    pub lpm_req_guid: u32,
}

//
// struct pmc_info - Structure to keep pmc info
// @devid:		device id of the pmc device
// @map:		pointer to a pmc_reg_map struct that contains platform
// specific attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_info {
    pub devid: u16,
    pub map: *const pmc_reg_map,
}

//
// struct pmc - pmc private info structure
// @base_addr:		contains pmc base address
// @regbase:		pointer to io-remapped memory location
// @map:		pointer to pmc_reg_map struct that contains platform
// specific attributes
// @lpm_req_regs:	List of substate requirements
// @ltr_ign:		Holds LTR ignore data while suspended
// @num_lpm_modes:	Count of enabled modes
// @lpm_en_modes:	Array of enabled modes from lowest to highest priority
// @devid:		Device ID of the SSRAM device
//
// pmc contains info about one power management controller device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc {
    pub base_addr: u64,
    pub regbase: *mut void __iomem,
    pub map: *const pmc_reg_map,
    pub lpm_req_regs: *mut u32,
    pub ltr_ign: u32,
    pub num_lpm_modes: u8,
    pub lpm_en_modes: [u8; LPM_MAX_NUM_MODES],
    pub devid: u16,
}

//
// struct pmc_dev - pmc device structure
// @devs:		pointer to an array of pmc pointers
// @pdev:		pointer to platform_device struct
// @crystal_freq:	crystal frequency from cpuid
// @dbgfs_dir:		path to debugfs interface
// @pmc_xram_read_bit:	flag to indicate whether PMC XRAM shadow registers
// used to read MPHY PG and PLL status are available
// @mutex_lock:		mutex to complete one transcation
// @pkgc_res_cnt:	Array of PKGC residency counters
// @num_of_pkgc:	Number of PKGC
// @s0ix_counter:	S0ix residency (step adjusted)
// @suspend:		Function to perform platform specific suspend
// @resume:		Function to perform platform specific resume
//
// @pkgc_ltr_blocker_counters: Array of PKGC LTR blocker counters
// @pkgc_ltr_blocker_offset: Offset to PKGC LTR blockers in telemetry region
// @pkgc_blocker_counters: Array of PKGC blocker counters
// @pkgc_blocker_offset: Offset to PKGC blocker in telemetry region
//
// pmc_dev contains info about power management controller device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_dev {
    pub pmcs: [*mut pmc; MAX_NUM_PMC],
    pub dbgfs_dir: *mut dentry,
    pub pdev: *mut platform_device,
    pub crystal_freq: c_uint,
    pub pmc_xram_read_bit: c_int,
    pub /: *mut *mut mutex lock; / generic mutex lock for PMC Core,
    pub s0ix_counter: u64,
    pub pmcdev): *mut *mut void (suspend)(struct pmc_dev,
    pub pmcdev): *mut *mut int (resume)(struct pmc_dev,
    pub pkgc_res_cnt: *mut u64,
    pub num_of_pkgc: u8,
    pub die_c6_offset: u32,
    pub pc_ep: *mut telem_endpoint,
    pub punit_ep: *mut telem_endpoint,
    pub regmap_list: *mut pmc_info,
    pub pkgc_ltr_blocker_counters: *const c_char,
    pub pkgc_ltr_blocker_offset: u32,
    pub pkgc_blocker_counters: *const c_char,
    pub pkgc_blocker_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pmc_index {
    PMC_IDX_MAIN,
    PMC_IDX_IOE,
    PMC_IDX_PCH,
    PMC_IDX_MAX
}

//
// struct pmc_dev_info - Structure to keep PMC device info
// @dmu_guids:		List of Die Management Unit GUID
// @pc_guid:		GUID for telemetry region to read PKGC blocker info
// @pkgc_ltr_blocker_offset: Offset to PKGC LTR blockers in telemetry region
// @pkgc_blocker_offset:Offset to PKGC blocker in telemetry region
// @num_pmcs:		Number of entries in @pmc_list
// @pmc_list:		Index list of available PMC
// @regmap_list:	Pointer to a list of pmc_info structure that could be
// available for the platform. When set, this field implies
// SSRAM support.
// @map:		Pointer to a pmc_reg_map struct that contains platform
// specific attributes of the primary PMC
// @sub_req_show:	File operations to show substate requirements
// @pkgc_ltr_blocker_counters: Array of PKGC LTR blocker counters
// @pkgc_blocker_counters: Array of PKGC blocker counters
// @suspend:		Function to perform platform specific suspend
// @resume:		Function to perform platform specific resume
// @init:		Function to perform platform specific init action
// @sub_req:		Function to achieve low power mode substate requirements
// @ssram_hidden:	Some SSRAM devices are hidden on this platform
// @die_c6_offset:	Telemetry offset to read Die C6 residency
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_dev_info {
    pub dmu_guids: *mut u32,
    pub pc_guid: u32,
    pub pkgc_ltr_blocker_offset: u32,
    pub pkgc_blocker_offset: u32,
    pub num_pmcs: u8,
    pub pmc_list: *const u8,
    pub regmap_list: *mut pmc_info,
    pub map: *const pmc_reg_map,
    pub sub_req_show: *const file_operations,
    pub pkgc_ltr_blocker_counters: *const c_char,
    pub pkgc_blocker_counters: *const c_char,
    pub pmcdev): *mut *mut void (suspend)(struct pmc_dev,
    pub pmcdev): *mut *mut int (resume)(struct pmc_dev,
    pub pmc_dev_info): *mut *mut *mut int (init)(struct pmc_dev pmcdev, struct pmc_dev_info,
    pub ep): *mut *mut *mut *mut int (sub_req)(struct pmc_dev pmcdev, struct pmc pmc, struct telem_endpoint,
    pub ssram_hidden: bool,
    pub die_c6_offset: u32,
}

extern "C" {
    pub fn pmc_core_get_tgl_lpm_reqs(pdev: *mut platform_device);
}
extern "C" {
    pub fn pmc_core_send_ltr_ignore(pmcdev: *mut pmc_dev, value: u32, ignore: c_int) -> c_int;
}
extern "C" {
    pub fn pmc_core_resume_common(pmcdev: *mut pmc_dev) -> c_int;
}
extern "C" {
    pub fn get_primary_reg_base(pmc: *mut pmc) -> c_int;
}
extern "C" {
    pub fn pmc_core_punit_pmt_init(pmcdev: *mut pmc_dev, pmc_dev_info: *mut pmc_dev_info);
}
extern "C" {
    pub fn pmc_core_set_device_d3(device: c_uint);
}
extern "C" {
    pub fn generic_core_init(pmcdev: *mut pmc_dev, pmc_dev_info: *mut pmc_dev_info) -> c_int;
}
extern "C" {
    pub fn cnl_suspend(pmcdev: *mut pmc_dev);
}
extern "C" {
    pub fn cnl_resume(pmcdev: *mut pmc_dev) -> c_int;
}
extern "C" {
    pub fn pmc_core_pmt_get_lpm_req(pmcdev: *mut pmc_dev, pmc: *mut pmc, ep: *mut telem_endpoint) -> c_int;
}

// Avoid checkpatch warning
