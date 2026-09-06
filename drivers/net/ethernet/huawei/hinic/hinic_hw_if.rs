//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_hw_if.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//

pub const HINIC_PCIE_LINK_DOWN: c_uint = 0xFFFFFFFF;
pub const HINIC_DMA_ATTR_ST_SHIFT: c_int = 0;
pub const HINIC_DMA_ATTR_AT_SHIFT: c_int = 8;
pub const HINIC_DMA_ATTR_PH_SHIFT: c_int = 10;
pub const HINIC_DMA_ATTR_NO_SNOOPING_SHIFT: c_int = 12;
pub const HINIC_DMA_ATTR_TPH_EN_SHIFT: c_int = 13;
pub const HINIC_DMA_ATTR_ST_MASK: c_uint = 0xFF;
pub const HINIC_DMA_ATTR_AT_MASK: c_uint = 0x3;
pub const HINIC_DMA_ATTR_PH_MASK: c_uint = 0x3;
pub const HINIC_DMA_ATTR_NO_SNOOPING_MASK: c_uint = 0x1;
pub const HINIC_DMA_ATTR_TPH_EN_MASK: c_uint = 0x1;

pub const HINIC_FA0_FUNC_IDX_SHIFT: c_int = 0;
pub const HINIC_FA0_PF_IDX_SHIFT: c_int = 10;
pub const HINIC_FA0_PCI_INTF_IDX_SHIFT: c_int = 14;
pub const HINIC_FA0_VF_IN_PF_SHIFT: c_int = 16;
// reserved members - off 16
pub const HINIC_FA0_FUNC_TYPE_SHIFT: c_int = 24;
pub const HINIC_FA0_FUNC_IDX_MASK: c_uint = 0x3FF;
pub const HINIC_FA0_PF_IDX_MASK: c_uint = 0xF;
pub const HINIC_FA0_PCI_INTF_IDX_MASK: c_uint = 0x3;
pub const HINIC_FA0_FUNC_TYPE_MASK: c_uint = 0x1;
pub const HINIC_FA0_VF_IN_PF_MASK: c_uint = 0xFF;

pub const HINIC_FA1_AEQS_PER_FUNC_SHIFT: c_int = 8;
// reserved members - off 10
pub const HINIC_FA1_CEQS_PER_FUNC_SHIFT: c_int = 12;
// reserved members - off 15
pub const HINIC_FA1_IRQS_PER_FUNC_SHIFT: c_int = 20;
pub const HINIC_FA1_DMA_ATTR_PER_FUNC_SHIFT: c_int = 24;
// reserved members - off 27
pub const HINIC_FA1_MGMT_INIT_STATUS_SHIFT: c_int = 30;
pub const HINIC_FA1_PF_INIT_STATUS_SHIFT: c_int = 31;
pub const HINIC_FA1_AEQS_PER_FUNC_MASK: c_uint = 0x3;
pub const HINIC_FA1_CEQS_PER_FUNC_MASK: c_uint = 0x7;
pub const HINIC_FA1_IRQS_PER_FUNC_MASK: c_uint = 0xF;
pub const HINIC_FA1_DMA_ATTR_PER_FUNC_MASK: c_uint = 0x7;
pub const HINIC_FA1_MGMT_INIT_STATUS_MASK: c_uint = 0x1;
pub const HINIC_FA1_PF_INIT_STATUS_MASK: c_uint = 0x1;

pub const HINIC_FA2_GLOBAL_VF_ID_OF_PF_SHIFT: c_int = 16;
pub const HINIC_FA2_GLOBAL_VF_ID_OF_PF_MASK: c_uint = 0x3FF;

pub const HINIC_FA4_OUTBOUND_STATE_SHIFT: c_int = 0;
pub const HINIC_FA4_DB_STATE_SHIFT: c_int = 1;
pub const HINIC_FA4_OUTBOUND_STATE_MASK: c_uint = 0x1;
pub const HINIC_FA4_DB_STATE_MASK: c_uint = 0x1;

pub const HINIC_FA5_PF_ACTION_SHIFT: c_int = 0;
pub const HINIC_FA5_PF_ACTION_MASK: c_uint = 0xFFFF;

pub const HINIC_PPF_ELECTION_IDX_SHIFT: c_int = 0;
pub const HINIC_PPF_ELECTION_IDX_MASK: c_uint = 0x1F;

pub const HINIC_MSIX_PENDING_LIMIT_SHIFT: c_int = 0;
pub const HINIC_MSIX_COALESC_TIMER_SHIFT: c_int = 8;
pub const HINIC_MSIX_LLI_TIMER_SHIFT: c_int = 16;
pub const HINIC_MSIX_LLI_CREDIT_SHIFT: c_int = 24;
pub const HINIC_MSIX_RESEND_TIMER_SHIFT: c_int = 29;
pub const HINIC_MSIX_PENDING_LIMIT_MASK: c_uint = 0xFF;
pub const HINIC_MSIX_COALESC_TIMER_MASK: c_uint = 0xFF;
pub const HINIC_MSIX_LLI_TIMER_MASK: c_uint = 0xFF;
pub const HINIC_MSIX_LLI_CREDIT_MASK: c_uint = 0x1F;
pub const HINIC_MSIX_RESEND_TIMER_MASK: c_uint = 0x7;

pub const HINIC_MSIX_CNT_RESEND_TIMER_SHIFT: c_int = 29;
pub const HINIC_MSIX_CNT_RESEND_TIMER_MASK: c_uint = 0x1;

pub const HINIC_PCI_CFG_REGS_BAR: c_int = 0;
pub const HINIC_PCI_INTR_REGS_BAR: c_int = 2;
pub const HINIC_PCI_DB_BAR: c_int = 4;
pub const HINIC_PCIE_ST_DISABLE: c_int = 0;
pub const HINIC_PCIE_AT_DISABLE: c_int = 0;
pub const HINIC_PCIE_PH_DISABLE: c_int = 0;

pub const HINIC_EQ_MSIX_COALESC_TIMER_DEFAULT: c_uint = 0xFF    /* max */;

pub const HINIC_PCI_MSIX_ENTRY_SIZE: c_int = 16;
pub const HINIC_PCI_MSIX_ENTRY_VECTOR_CTRL: c_int = 12;
pub const HINIC_PCI_MSIX_ENTRY_CTRL_MASKBIT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_pcie_nosnoop {
    HINIC_PCIE_SNOOP        = 0,
    HINIC_PCIE_NO_SNOOP     = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_pcie_tph {
    HINIC_PCIE_TPH_DISABLE  = 0,
    HINIC_PCIE_TPH_ENABLE   = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_func_type {
    HINIC_PF        = 0,
    HINIC_VF	    = 1,
    HINIC_PPF       = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_mod_type {
    HINIC_MOD_COMM  = 0,    /* HW communication module */
    HINIC_MOD_L2NIC = 1,    /* L2NIC module */
    HINIC_MOD_CFGM  = 7,    /* Configuration module */
    HINIC_MOD_HILINK = 14,  /* Hilink module */
    HINIC_MOD_MAX   = 15
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_node_id {
    HINIC_NODE_ID_MGMT = 21,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_pf_action {
    HINIC_PF_MGMT_INIT = 0x0,

    HINIC_PF_MGMT_ACTIVE = 0x11,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_outbound_state {
    HINIC_OUTBOUND_ENABLE  = 0,
    HINIC_OUTBOUND_DISABLE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_db_state {
    HINIC_DB_ENABLE  = 0,
    HINIC_DB_DISABLE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_msix_state {
    HINIC_MSIX_ENABLE,
    HINIC_MSIX_DISABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_func_attr {
    pub func_idx: u16,
    pub pf_idx: u8,
    pub pci_intf_idx: u8,
    pub func_type: hinic_func_type,
    pub ppf_idx: u8,
    pub num_irqs: u16,
    pub num_aeqs: u8,
    pub num_ceqs: u8,
    pub num_dma_attr: u8,
    pub global_vf_id_of_pf: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_hwif {
    pub pdev: *mut pci_dev,
    pub cfg_regs_bar: *mut void __iomem,
    pub intr_regs_base: *mut void __iomem,
    pub attr: hinic_func_attr,
}

extern "C" {
    pub fn be32_to_cpu()&out: *mut *mut (__be32) -> return;
}
extern "C" {
    pub fn hinic_msix_attr_cnt_clear(hwif: *mut hinic_hwif, msix_index: u16) -> c_int;
}
extern "C" {
    pub fn hinic_set_pf_action(hwif: *mut hinic_hwif, action: hinic_pf_action);
}
extern "C" {
    pub fn hinic_outbound_state_get(hwif: *mut hinic_hwif) -> hinic_outbound_state;
}
extern "C" {
    pub fn hinic_db_state_get(hwif: *mut hinic_hwif) -> hinic_db_state;
}
extern "C" {
    pub fn hinic_glb_pf_vf_offset(hwif: *mut hinic_hwif) -> u16;
}
extern "C" {
    pub fn hinic_global_func_id_hw(hwif: *mut hinic_hwif) -> u16;
}
extern "C" {
    pub fn hinic_pf_id_of_vf_hw(hwif: *mut hinic_hwif) -> u16;
}
extern "C" {
    pub fn hinic_init_hwif(hwif: *mut hinic_hwif, pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn hinic_free_hwif(hwif: *mut hinic_hwif);
}
