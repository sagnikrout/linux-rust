//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hns3pf/hclge_err.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2016-2017 Hisilicon Limited.

pub const HCLGE_MPF_RAS_INT_MIN_BD_NUM: c_int = 10;
pub const HCLGE_PF_RAS_INT_MIN_BD_NUM: c_int = 4;
pub const HCLGE_MPF_MSIX_INT_MIN_BD_NUM: c_int = 10;
pub const HCLGE_PF_MSIX_INT_MIN_BD_NUM: c_int = 4;
pub const HCLGE_RAS_PF_OTHER_INT_STS_REG: c_uint = 0x20B00;
pub const HCLGE_RAS_REG_NFE_MASK: c_uint = 0xFF00;
pub const HCLGE_RAS_REG_ROCEE_ERR_MASK: c_uint = 0x3000000;

pub const HCLGE_VECTOR0_REG_MSIX_MASK: c_uint = 0x1FF00;
pub const HCLGE_IMP_TCM_ECC_ERR_INT_EN: c_uint = 0xFFFF0000;
pub const HCLGE_IMP_TCM_ECC_ERR_INT_EN_MASK: c_uint = 0xFFFF0000;
pub const HCLGE_IMP_ITCM4_ECC_ERR_INT_EN: c_uint = 0x300;
pub const HCLGE_IMP_ITCM4_ECC_ERR_INT_EN_MASK: c_uint = 0x300;
pub const HCLGE_CMDQ_NIC_ECC_ERR_INT_EN: c_uint = 0xFFFF;
pub const HCLGE_CMDQ_NIC_ECC_ERR_INT_EN_MASK: c_uint = 0xFFFF;
pub const HCLGE_CMDQ_ROCEE_ECC_ERR_INT_EN: c_uint = 0xFFFF0000;
pub const HCLGE_CMDQ_ROCEE_ECC_ERR_INT_EN_MASK: c_uint = 0xFFFF0000;
pub const HCLGE_IMP_RD_POISON_ERR_INT_EN: c_uint = 0x0100;
pub const HCLGE_IMP_RD_POISON_ERR_INT_EN_MASK: c_uint = 0x0100;
pub const HCLGE_TQP_ECC_ERR_INT_EN: c_uint = 0x0FFF;
pub const HCLGE_TQP_ECC_ERR_INT_EN_MASK: c_uint = 0x0FFF;
pub const HCLGE_MSIX_SRAM_ECC_ERR_INT_EN_MASK: c_uint = 0x0F000000;
pub const HCLGE_MSIX_SRAM_ECC_ERR_INT_EN: c_uint = 0x0F000000;
pub const HCLGE_IGU_ERR_INT_EN: c_uint = 0x0000000F;
pub const HCLGE_IGU_ERR_INT_TYPE: c_uint = 0x00000660;
pub const HCLGE_IGU_ERR_INT_EN_MASK: c_uint = 0x000F;
pub const HCLGE_IGU_TNL_ERR_INT_EN: c_uint = 0x0002AABF;
pub const HCLGE_IGU_TNL_ERR_INT_EN_MASK: c_uint = 0x003F;
pub const HCLGE_PPP_MPF_ECC_ERR_INT0_EN: c_uint = 0xFFFFFFFF;
pub const HCLGE_PPP_MPF_ECC_ERR_INT0_EN_MASK: c_uint = 0xFFFFFFFF;
pub const HCLGE_PPP_MPF_ECC_ERR_INT1_EN: c_uint = 0xFFFFFFFF;
pub const HCLGE_PPP_MPF_ECC_ERR_INT1_EN_MASK: c_uint = 0xFFFFFFFF;
pub const HCLGE_PPP_PF_ERR_INT_EN: c_uint = 0x0003;
pub const HCLGE_PPP_PF_ERR_INT_EN_MASK: c_uint = 0x0003;
pub const HCLGE_PPP_MPF_ECC_ERR_INT2_EN: c_uint = 0x003F;
pub const HCLGE_PPP_MPF_ECC_ERR_INT2_EN_MASK: c_uint = 0x003F;
pub const HCLGE_PPP_MPF_ECC_ERR_INT3_EN: c_uint = 0x003F;
pub const HCLGE_PPP_MPF_ECC_ERR_INT3_EN_MASK: c_uint = 0x003F;
pub const HCLGE_TM_SCH_ECC_ERR_INT_EN: c_uint = 0x3;
pub const HCLGE_TM_QCN_ERR_INT_TYPE: c_uint = 0x29;
pub const HCLGE_TM_QCN_FIFO_INT_EN: c_uint = 0xFFFF00;
pub const HCLGE_TM_QCN_MEM_ERR_INT_EN: c_uint = 0xFFFFFF;
pub const HCLGE_NCSI_ERR_INT_EN: c_uint = 0x3;
pub const HCLGE_NCSI_ERR_INT_TYPE: c_uint = 0x9;
pub const HCLGE_MAC_COMMON_ERR_INT_EN: c_uint = 0x107FF;
pub const HCLGE_MAC_COMMON_ERR_INT_EN_MASK: c_uint = 0x107FF;

pub const HCLGE_PPU_MPF_ABNORMAL_INT2_EN: c_uint = 0x3FFF3FFF;
pub const HCLGE_PPU_MPF_ABNORMAL_INT2_EN_MASK: c_uint = 0x3FFF3FFF;
pub const HCLGE_PPU_MPF_ABNORMAL_INT2_EN2: c_uint = 0xB;
pub const HCLGE_PPU_MPF_ABNORMAL_INT2_EN2_MASK: c_uint = 0xB;

pub const HCLGE_SSU_BIT32_ECC_ERR_INT_EN: c_uint = 0x0101;
pub const HCLGE_SSU_BIT32_ECC_ERR_INT_EN_MASK: c_uint = 0x0101;

pub const HCLGE_SSU_PORT_BASED_ERR_INT_EN: c_uint = 0x0BFF;
pub const HCLGE_SSU_PORT_BASED_ERR_INT_EN_MASK: c_uint = 0x0BFF0000;

pub const HCLGE_SSU_PORT_INT_MSIX_MASK: c_uint = 0x7BFF;

pub const HCLGE_PPU_PF_INT_RAS_MASK: c_uint = 0x18;
pub const HCLGE_PPU_PF_INT_MSIX_MASK: c_uint = 0x26;
pub const HCLGE_PPU_PF_OVER_8BD_ERR_MASK: c_uint = 0x01;

pub const HCLGE_ROCEE_RAS_NFE_INT_EN: c_uint = 0xF;
pub const HCLGE_ROCEE_RAS_CE_INT_EN: c_uint = 0x1;
pub const HCLGE_ROCEE_RAS_NFE_INT_EN_MASK: c_uint = 0xF;
pub const HCLGE_ROCEE_RAS_CE_INT_EN_MASK: c_uint = 0x1;

pub const HCLGE_ROCEE_OVF_ERR_INT_MASK: c_uint = 0x10000;
pub const HCLGE_ROCEE_OVF_ERR_TYPE_MASK: c_uint = 0x3F;
pub const HCLGE_DESC_DATA_MAX: c_int = 8;
pub const HCLGE_REG_NUM_MAX: c_int = 256;
pub const HCLGE_DESC_NO_DATA_LEN: c_int = 8;
pub const HCLGE_BD_NUM_SSU_REG_0: c_int = 10;
pub const HCLGE_BD_NUM_SSU_REG_1: c_int = 15;
pub const HCLGE_BD_NUM_RPU_REG_0: c_int = 1;
pub const HCLGE_BD_NUM_RPU_REG_1: c_int = 2;
pub const HCLGE_BD_NUM_IGU_EGU_REG: c_int = 9;
pub const HCLGE_BD_NUM_GEN_REG: c_int = 8;
pub const HCLGE_MOD_REG_INFO_LEN_MAX: c_int = 256;
pub const HCLGE_MOD_REG_EXTRA_LEN: c_int = 11;
pub const HCLGE_MOD_REG_VALUE_LEN: c_int = 9;
pub const HCLGE_MOD_REG_GROUP_MAX_SIZE: c_int = 6;
pub const HCLGE_MOD_MSG_PARA_ARRAY_MAX_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_err_int_type {
    HCLGE_ERR_INT_MSIX = 0,
    HCLGE_ERR_INT_RAS_CE = 1,
    HCLGE_ERR_INT_RAS_NFE = 2,
    HCLGE_ERR_INT_RAS_FE = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_mod_name_list {
    MODULE_NONE		= 0,
    MODULE_BIOS_COMMON	= 1,
    MODULE_GE		= 2,
    MODULE_IGU_EGU		= 3,
    MODULE_LGE		= 4,
    MODULE_NCSI		= 5,
    MODULE_PPP		= 6,
    MODULE_QCN		= 7,
    MODULE_RCB_RX		= 8,
    MODULE_RTC		= 9,
    MODULE_SSU		= 10,
    MODULE_TM		= 11,
    MODULE_RCB_TX		= 12,
    MODULE_TXDMA		= 13,
    MODULE_MASTER		= 14,
    MODULE_HIMAC		= 15,
// add new MODULE NAME for NIC here in order
    MODULE_ROCEE_TOP	= 40,
    MODULE_ROCEE_TIMER	= 41,
    MODULE_ROCEE_MDB	= 42,
    MODULE_ROCEE_TSP	= 43,
    MODULE_ROCEE_TRP	= 44,
    MODULE_ROCEE_SCC	= 45,
    MODULE_ROCEE_CAEP	= 46,
    MODULE_ROCEE_GEN_AC	= 47,
    MODULE_ROCEE_QMM	= 48,
    MODULE_ROCEE_LSAN	= 49,
// add new MODULE NAME for RoCEE here in order
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_err_type_list {
    NONE_ERROR		= 0,
    FIFO_ERROR		= 1,
    MEMORY_ERROR		= 2,
    POISON_ERROR		= 3,
    MSIX_ECC_ERROR		= 4,
    TQP_INT_ECC_ERROR	= 5,
    PF_ABNORMAL_INT_ERROR	= 6,
    MPF_ABNORMAL_INT_ERROR	= 7,
    COMMON_ERROR		= 8,
    PORT_ERROR		= 9,
    ETS_ERROR		= 10,
    NCSI_ERROR		= 11,
    GLB_ERROR		= 12,
    LINK_ERROR		= 13,
    PTP_ERROR		= 14,
// add new ERROR TYPE for NIC here in order
    ROCEE_NORMAL_ERR	= 40,
    ROCEE_OVF_ERR		= 41,
    ROCEE_BUS_ERR		= 42,
// add new ERROR TYPE for ROCEE here in order
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_hw_blk {
    pub msk: u32,
    pub name: *const c_char,
    pub en): *mut *mut *mut int (config_err_int)(struct hclge_dev hdev, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_hw_error {
    pub int_msk: u32,
    pub msg: *const c_char,
    pub reset_level: hnae3_reset_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_hw_module_id {
    pub module_id: hclge_mod_name_list,
    pub msg: *const c_char,
    pub hdev): *mut *mut void (query_reg_info)(struct hclge_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_hw_type_id {
    pub type_id: hclge_err_type_list,
    pub msg: *const c_char,
    pub /: *mut *mut bool cause_by_vf; / indicate the error may from vf exception,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_sum_err_info {
    pub reset_type: u8,
    pub mod_num: u8,
    pub rsv: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mod_err_info {
    pub mod_id: u8,
    pub err_num: u8,
    pub rsv: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_type_reg_err_info {
    pub type_id: u8,
    pub reg_num: u8,
    pub rsv: [u8; 2],
    pub hclge_reg: [u32; HCLGE_REG_NUM_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mod_reg_info {
    pub reg_name: *const c_char,
    pub /: *mut *mut bool has_suffix; / add suffix for register name,
// the positions of reg values in hclge_desc.data
    pub reg_offset_group: [u8; HCLGE_MOD_REG_GROUP_MAX_SIZE],
    pub group_size: u8,
}

// This structure defines cmdq used to query the hardware module debug
// regisgers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mod_reg_common_msg {
    pub cmd: hclge_opcode_type,
    pub desc: *mut hclge_desc,
    pub /: *mut *mut u8 bd_num; / the bd number of hclge_desc used,
    pub /: *mut *mut bool need_para; / whether this cmdq needs to add para,
// the regs need to print
    pub result_regs: *const hclge_mod_reg_info,
    pub result_regs_size: u16,
}

extern "C" {
    pub fn hclge_config_mac_tnl_int(hdev: *mut hclge_dev, en: bool) -> c_int;
}
extern "C" {
    pub fn hclge_config_nic_hw_error(hdev: *mut hclge_dev, state: bool) -> c_int;
}
extern "C" {
    pub fn hclge_config_rocee_ras_interrupt(hdev: *mut hclge_dev, en: bool) -> c_int;
}
extern "C" {
    pub fn hclge_handle_all_hns_hw_errors(ae_dev: *mut hnae3_ae_dev);
}
extern "C" {
    pub fn hclge_find_error_source(hdev: *mut hclge_dev) -> bool;
}
extern "C" {
    pub fn hclge_handle_occurred_error(hdev: *mut hclge_dev);
}
extern "C" {
    pub fn hclge_handle_hw_ras_error(ae_dev: *mut hnae3_ae_dev) -> pci_ers_result_t;
}
extern "C" {
    pub fn hclge_handle_error_info_log(ae_dev: *mut hnae3_ae_dev) -> c_int;
}
extern "C" {
    pub fn hclge_handle_mac_tnl(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_handle_vf_queue_err_ras(hdev: *mut hclge_dev) -> c_int;
}
