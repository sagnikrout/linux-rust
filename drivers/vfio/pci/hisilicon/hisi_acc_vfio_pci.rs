//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vfio/pci/hisilicon/hisi_acc_vfio_pci.h
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
// Copyright (c) 2021 HiSilicon Ltd.

pub const MB_POLL_PERIOD_US: c_int = 10;
pub const MB_POLL_TIMEOUT_US: c_int = 1000;
pub const QM_CACHE_WB_START: c_uint = 0x204;
pub const QM_CACHE_WB_DONE: c_uint = 0x208;
pub const QM_MB_CMD_PAUSE_QM: c_uint = 0xe;
pub const QM_ABNORMAL_INT_STATUS: c_uint = 0x100008;
pub const QM_IFC_INT_STATUS: c_uint = 0x0028;
pub const SEC_CORE_INT_STATUS: c_uint = 0x301008;
pub const HPRE_HAC_INT_STATUS: c_uint = 0x301800;
pub const HZIP_CORE_INT_STATUS: c_uint = 0x3010AC;
pub const QM_VFT_CFG_RDY: c_uint = 0x10006c;
pub const QM_VFT_CFG_OP_WR: c_uint = 0x100058;
pub const QM_VFT_CFG_TYPE: c_uint = 0x10005c;
pub const QM_VFT_CFG: c_uint = 0x100060;
pub const QM_VFT_CFG_OP_ENABLE: c_uint = 0x100054;
pub const QM_VFT_CFG_DATA_L: c_uint = 0x100064;
pub const QM_VFT_CFG_DATA_H: c_uint = 0x100068;
pub const ERROR_CHECK_TIMEOUT: c_int = 100;
pub const CHECK_DELAY_TIME: c_int = 100;
pub const QM_RESET_WAIT_TIMEOUT: c_int = 60000;
pub const QM_SQC_VFT_BASE_SHIFT_V2: c_int = 28;

pub const QM_SQC_VFT_NUM_SHIFT_V2: c_int = 45;

pub const QM_MB_CMD_NOT_READY: c_uint = 0xffffffff;
// RW regs
pub const QM_REGS_MAX_LEN: c_int = 7;
pub const QM_REG_ADDR_OFFSET: c_uint = 0x0004;

pub const QM_XQC_ADDR_LOW: c_uint = 0x1;
pub const QM_XQC_ADDR_HIGH: c_uint = 0x2;
pub const QM_VF_AEQ_INT_MASK: c_uint = 0x0004;
pub const QM_VF_EQ_INT_MASK: c_uint = 0x000c;
pub const QM_IFC_INT_SOURCE_V: c_uint = 0x0020;
pub const QM_IFC_INT_MASK: c_uint = 0x0024;
pub const QM_IFC_INT_SET_V: c_uint = 0x002c;
pub const QM_QUE_ISO_CFG_V: c_uint = 0x0030;
pub const QM_PAGE_SIZE: c_uint = 0x0034;

pub const QM_EQC_PF_DW0: c_uint = 0x1c00;
pub const QM_AEQC_PF_DW0: c_uint = 0x1c20;
pub const ACC_DRV_MAJOR_VER: c_int = 1;
pub const ACC_DRV_MINOR_VER: c_int = 0;

pub const ACC_DEV_MAGIC_V2: c_uint = 0xAACCFEEDDECADEDE;
pub const QM_MIG_REGION_OFFSET: c_uint = 0x180000;
pub const QM_MIG_REGION_SIZE: c_uint = 0x2000;
//
// On HW_ACC_MIG_VF_CTRL mode, the configuration domain supporting live
// migration functionality is located in the latter 32KB of the VF's BAR2.
// The Guest is only provided with the first 32KB of the VF's BAR2.
// On HW_ACC_MIG_PF_CTRL mode, the configuration domain supporting live
// migration functionality is located in the PF's BAR2, and the entire 64KB
// of the VF's BAR2 is allocated to the Guest.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_drv_mode {
    HW_ACC_MIG_VF_CTRL = 0,
    HW_ACC_MIG_PF_CTRL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acc_vf_data {

// QM match information
    pub acc_magic: u64,
    pub qp_num: u32,
    pub dev_id: u32,
    pub que_iso_cfg: u32,
    pub qp_base: u32,
    pub vf_qm_state: u32,
// QM reserved match information
    pub major_ver: u16,
    pub minor_ver: u16,
    pub qm_rsv_state: [u32; 2],
// QM RW regs
    pub aeq_int_mask: u32,
    pub eq_int_mask: u32,
    pub ifc_int_source: u32,
    pub ifc_int_mask: u32,
    pub ifc_int_set: u32,
    pub page_size: u32,
// QM_EQC_DW has 7 regs
    pub qm_eqc_dw: [u32; 7],
// QM_AEQC_DW has 7 regs
    pub qm_aeqc_dw: [u32; 7],
// QM reserved 5 regs
    pub qm_rsv_regs: [u32; 5],
    pub padding: u32,
// QM memory init information
    pub eqe_dma: u64,
    pub aeqe_dma: u64,
    pub sqc_dma: u64,
    pub cqc_dma: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_acc_vf_migration_file {
    pub filp: *mut file,
    pub lock: mutex,
    pub disabled: bool,
    pub hisi_acc_vdev: *mut hisi_acc_vf_core_device,
    pub vf_data: acc_vf_data,
    pub total_length: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_acc_vf_core_device {
    pub core_device: vfio_pci_core_device,
    pub match_done: u8,
    pub set_reset_flag: bool,
//
// io_base is only valid when dev_opened is true,
// which is protected by open_mutex.
//
    pub dev_opened: bool,
// Ensure the accuracy of dev_opened operation
    pub open_mutex: mutex,
// For migration state
    pub state_mutex: mutex,
    pub mig_state: vfio_device_mig_state,
    pub pf_dev: *mut pci_dev,
    pub vf_dev: *mut pci_dev,
    pub pf_qm: *mut hisi_qm,
    pub vf_qm: hisi_qm,
    pub drv_mode: hw_drv_mode,
//
// vf_qm_state represents the QM_VF_STATE register value.
// It is set by Guest driver for the ACC VF dev indicating
// the driver has loaded and configured the dev correctly.
//
    pub vf_qm_state: u32,
    pub vf_id: c_int,
    pub resuming_migf: *mut hisi_acc_vf_migration_file,
    pub saving_migf: *mut hisi_acc_vf_migration_file,
//
// It holds migration data corresponding to the last migration
// and is used by the debugfs interface to report it.
//
    pub debug_migf: *mut hisi_acc_vf_migration_file,
}
