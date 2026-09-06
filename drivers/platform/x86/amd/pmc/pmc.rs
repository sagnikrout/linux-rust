//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/amd/pmc/pmc.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// AMD SoC Power Management Controller Driver
//
// Copyright (c) 2023, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Author: Mario Limonciello <mario.limonciello@amd.com>
//

// SMU communication registers
pub const AMD_PMC_REGISTER_RESPONSE: c_uint = 0x980;
pub const AMD_PMC_REGISTER_ARGUMENT: c_uint = 0x9BC;
pub const AMD_PMC_REGISTER_MESSAGE: c_uint = 0x538;
// SMU communication registers for 1Ah 20h SoC
pub const AMD_PMC_REGISTER_MSG_1AH_20H: c_uint = 0x938;
// SMU communication registers for 1Ah 80h SoC
pub const AMD_PMC_REGISTER_MSG_1AH_80H: c_uint = 0xA10;
pub const AMD_PMC_REGISTER_ARG_1AH_80H: c_uint = 0xA18;
pub const AMD_PMC_REGISTER_RSP_1AH_80H: c_uint = 0xA14;
// PMC Scratch Registers
pub const AMD_PMC_SCRATCH_REG_CZN: c_uint = 0x94;
pub const AMD_PMC_SCRATCH_REG_YC: c_uint = 0xD14;
pub const AMD_PMC_SCRATCH_REG_1AH: c_uint = 0xF14;
// STB Registers
pub const AMD_PMC_STB_S2IDLE_PREPARE: c_uint = 0xC6000001;
pub const AMD_PMC_STB_S2IDLE_RESTORE: c_uint = 0xC6000002;
pub const AMD_PMC_STB_S2IDLE_CHECK: c_uint = 0xC6000003;
// Base address of SMU for mapping physical address to virtual address
pub const AMD_PMC_MAPPING_SIZE: c_uint = 0x01000;
pub const AMD_PMC_BASE_ADDR_OFFSET: c_uint = 0x10000;
pub const AMD_PMC_BASE_ADDR_LO: c_uint = 0x13B102E8;
pub const AMD_PMC_BASE_ADDR_HI: c_uint = 0x13B102EC;

// SMU Response Codes
pub const AMD_PMC_RESULT_OK: c_uint = 0x01;
pub const AMD_PMC_RESULT_CMD_REJECT_BUSY: c_uint = 0xFC;
pub const AMD_PMC_RESULT_CMD_REJECT_PREREQ: c_uint = 0xFD;
pub const AMD_PMC_RESULT_CMD_UNKNOWN: c_uint = 0xFE;
pub const AMD_PMC_RESULT_FAILED: c_uint = 0xFF;
// FCH SSC Registers
pub const FCH_S0I3_ENTRY_TIME_L_OFFSET: c_uint = 0x30;
pub const FCH_S0I3_ENTRY_TIME_H_OFFSET: c_uint = 0x34;
pub const FCH_S0I3_EXIT_TIME_L_OFFSET: c_uint = 0x38;
pub const FCH_S0I3_EXIT_TIME_H_OFFSET: c_uint = 0x3C;
pub const FCH_SSC_MAPPING_SIZE: c_uint = 0x800;
pub const FCH_BASE_PHY_ADDR_LOW: c_uint = 0xFED81100;
pub const FCH_BASE_PHY_ADDR_HIGH: c_uint = 0x00000000;
// SMU Message Definations
pub const SMU_MSG_GETSMUVERSION: c_uint = 0x02;
pub const SMU_MSG_LOG_GETDRAM_ADDR_HI: c_uint = 0x04;
pub const SMU_MSG_LOG_GETDRAM_ADDR_LO: c_uint = 0x05;
pub const SMU_MSG_LOG_START: c_uint = 0x06;
pub const SMU_MSG_LOG_RESET: c_uint = 0x07;
pub const SMU_MSG_LOG_DUMP_DATA: c_uint = 0x08;
pub const SMU_MSG_GET_SUP_CONSTRAINTS: c_uint = 0x09;
pub const PMC_MSG_DELAY_MIN_US: c_int = 50;
pub const RESPONSE_REGISTER_LOOP_MAX: c_int = 20000;
pub const DELAY_MIN_US: c_int = 2000;
pub const DELAY_MAX_US: c_int = 3000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s2d_msg_port {
    MSG_PORT_PMC,
    MSG_PORT_S2D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_mp2_dev {
    pub mmio: *mut void __iomem,
    pub vslbase: *mut void __iomem,
    pub stbdata: *mut c_void,
    pub devres_gid: *mut c_void,
    pub pdev: *mut pci_dev,
    pub dma_addr: dma_addr_t,
    pub stb_len: c_int,
    pub is_stb_data: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stb_arg {
    pub s2d_msg_id: u32,
    pub msg: u32,
    pub arg: u32,
    pub resp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmc_bit_map {
    pub name: *const c_char,
    pub bit_mask: u32,
}

// SoC-specific information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmc_cpu_info {
    pub smu_msg: u32,
    pub smu_arg: u32,
    pub smu_rsp: u32,
    pub num_ips: u32,
    pub scratch_reg: u32,
    pub ips_ptr: *const amd_pmc_bit_map,
    pub os_hint: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmc_dev {
    pub regbase: *mut void __iomem,
    pub smu_virt_addr: *mut void __iomem,
    pub stb_virt_addr: *mut void __iomem,
    pub fch_virt_addr: *mut void __iomem,
    pub base_addr: u32,
    pub cpu_id: u32,
    pub dram_size: u32,
    pub active_ips: u32,
// SMU version information
    pub smu_program: u8,
    pub major: u8,
    pub minor: u8,
    pub rev: u8,
    pub msg_port: u8,
    pub dev: *mut device,
    pub rdev: *mut pci_dev,
    pub /: *mut *mut mutex lock; / generic mutex lock,
    pub dbgfs_dir: *mut dentry,
    pub quirks: *mut quirk_entry,
    pub disable_8042_wakeup: bool,
    pub is_first_check_after_suspend: bool,
    pub mp2: *mut amd_mp2_dev,
    pub stb_arg: stb_arg,
    pub cpu_info: *const amd_pmc_cpu_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_metrics {
    pub table_version: u32,
    pub hint_count: u32,
    pub s0i3_last_entry_status: u32,
    pub timein_s0i2: u32,
    pub timeentering_s0i3_lastcapture: u64,
    pub timeentering_s0i3_totaltime: u64,
    pub timeto_resume_to_os_lastcapture: u64,
    pub timeto_resume_to_os_totaltime: u64,
    pub timein_s0i3_lastcapture: u64,
    pub timein_s0i3_totaltime: u64,
    pub timein_swdrips_lastcapture: u64,
    pub timein_swdrips_totaltime: u64,
    pub timecondition_notmet_lastcapture: [u64; 32],
    pub timecondition_notmet_totaltime: [u64; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_pmc_def {
    MSG_TEST = 0x01,
    MSG_OS_HINT_PCO,
    MSG_OS_HINT_RN,
}

    pub dev): *mut void amd_pmc_process_restore_quirks(struct amd_pmc_dev,
    pub dev): *mut bool amd_pmc_quirk_need_suspend_delay(struct amd_pmc_dev,
    pub dev): *mut void amd_pmc_quirks_init(struct amd_pmc_dev,
    pub dev): *mut void amd_mp2_stb_init(struct amd_pmc_dev,
    pub dev): *mut void amd_mp2_stb_deinit(struct amd_pmc_dev,
// List of supported CPU/device IDs
pub const PCI_DEVICE_ID_AMD_CPU_ID_PCO: c_uint = 0x15D0;
pub const PCI_DEVICE_ID_AMD_CPU_ID_CZN: c_uint = 0x1630;
pub const PCI_DEVICE_ID_AMD_CPU_ID_VG: c_uint = 0x1645;
pub const PCI_DEVICE_ID_AMD_CPU_ID_YC: c_uint = 0x14B5;
pub const PCI_DEVICE_ID_AMD_CPU_ID_CB: c_uint = 0x14D8;
pub const PCI_DEVICE_ID_AMD_CPU_ID_PS: c_uint = 0x14E8;
pub const PCI_DEVICE_ID_AMD_CPU_ID_SP: c_uint = 0x14A4;
pub const PCI_DEVICE_ID_AMD_CPU_ID_SHP: c_uint = 0x153A;
// Backward compatibility aliases

pub const PCI_DEVICE_ID_AMD_1AH_M20H_ROOT: c_uint = 0x1507;
pub const PCI_DEVICE_ID_AMD_1AH_M60H_ROOT: c_uint = 0x1122;
pub const PCI_DEVICE_ID_AMD_1AH_M80H_ROOT: c_uint = 0x115b;
pub const PCI_DEVICE_ID_AMD_MP2_STB: c_uint = 0x172c;
    pub dev): *mut int amd_stb_s2d_init(struct amd_pmc_dev,
    pub buf): *mut *mut int amd_stb_read(struct amd_pmc_dev dev, u32,
    pub data): *mut *mut int amd_stb_write(struct amd_pmc_dev dev, u32,
    pub ret): *mut *mut *mut int amd_pmc_send_cmd(struct amd_pmc_dev dev, u32 arg, u32 data, u8 msg, bool,
