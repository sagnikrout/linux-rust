//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi/gaudi_masks.h
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

// Useful masks for bits in various registers

// RESET registers configuration

pub const UNIT_RST_L_PSOC_SHIFT: c_int = 0;
pub const UNIT_RST_L_PCIE_SHIFT: c_int = 1;
pub const UNIT_RST_L_PCIE_IF_SHIFT: c_int = 2;
pub const UNIT_RST_L_HBM_S_PLL_SHIFT: c_int = 3;
pub const UNIT_RST_L_TPC_S_PLL_SHIFT: c_int = 4;
pub const UNIT_RST_L_MME_S_PLL_SHIFT: c_int = 5;
pub const UNIT_RST_L_CPU_PLL_SHIFT: c_int = 6;
pub const UNIT_RST_L_PCIE_PLL_SHIFT: c_int = 7;
pub const UNIT_RST_L_NIC_S_PLL_SHIFT: c_int = 8;
pub const UNIT_RST_L_HBM_N_PLL_SHIFT: c_int = 9;
pub const UNIT_RST_L_TPC_N_PLL_SHIFT: c_int = 10;
pub const UNIT_RST_L_MME_N_PLL_SHIFT: c_int = 11;
pub const UNIT_RST_L_NIC_N_PLL_SHIFT: c_int = 12;
pub const UNIT_RST_L_DMA_W_PLL_SHIFT: c_int = 13;
pub const UNIT_RST_L_SIF_W_PLL_SHIFT: c_int = 14;
pub const UNIT_RST_L_MESH_W_PLL_SHIFT: c_int = 15;
pub const UNIT_RST_L_SRAM_W_PLL_SHIFT: c_int = 16;
pub const UNIT_RST_L_DMA_E_PLL_SHIFT: c_int = 17;
pub const UNIT_RST_L_SIF_E_PLL_SHIFT: c_int = 18;
pub const UNIT_RST_L_MESH_E_PLL_SHIFT: c_int = 19;
pub const UNIT_RST_L_SRAM_E_PLL_SHIFT: c_int = 20;
pub const UNIT_RST_L_TPC_0_SHIFT: c_int = 21;
pub const UNIT_RST_L_TPC_1_SHIFT: c_int = 22;
pub const UNIT_RST_L_TPC_2_SHIFT: c_int = 23;
pub const UNIT_RST_L_TPC_3_SHIFT: c_int = 24;
pub const UNIT_RST_L_TPC_4_SHIFT: c_int = 25;
pub const UNIT_RST_L_TPC_5_SHIFT: c_int = 26;
pub const UNIT_RST_L_TPC_6_SHIFT: c_int = 27;
pub const UNIT_RST_L_TPC_7_SHIFT: c_int = 28;
pub const UNIT_RST_L_MME_0_SHIFT: c_int = 29;
pub const UNIT_RST_L_MME_1_SHIFT: c_int = 30;
pub const UNIT_RST_L_MME_2_SHIFT: c_int = 31;
pub const UNIT_RST_H_MME_3_SHIFT: c_int = 0;
pub const UNIT_RST_H_HBM_0_SHIFT: c_int = 1;
pub const UNIT_RST_H_HBM_1_SHIFT: c_int = 2;
pub const UNIT_RST_H_HBM_2_SHIFT: c_int = 3;
pub const UNIT_RST_H_HBM_3_SHIFT: c_int = 4;
pub const UNIT_RST_H_NIC_0_SHIFT: c_int = 5;
pub const UNIT_RST_H_NIC_1_SHIFT: c_int = 6;
pub const UNIT_RST_H_NIC_2_SHIFT: c_int = 7;
pub const UNIT_RST_H_NIC_3_SHIFT: c_int = 8;
pub const UNIT_RST_H_NIC_4_SHIFT: c_int = 9;
pub const UNIT_RST_H_SM_0_SHIFT: c_int = 10;
pub const UNIT_RST_H_SM_1_SHIFT: c_int = 11;
pub const UNIT_RST_H_SM_2_SHIFT: c_int = 12;
pub const UNIT_RST_H_SM_3_SHIFT: c_int = 13;
pub const UNIT_RST_H_IF_0_SHIFT: c_int = 14;
pub const UNIT_RST_H_IF_1_SHIFT: c_int = 15;
pub const UNIT_RST_H_IF_2_SHIFT: c_int = 16;
pub const UNIT_RST_H_IF_3_SHIFT: c_int = 17;
pub const UNIT_RST_H_DMA_0_SHIFT: c_int = 18;
pub const UNIT_RST_H_DMA_1_SHIFT: c_int = 19;
pub const UNIT_RST_H_CPU_SHIFT: c_int = 20;
pub const UNIT_RST_H_MMU_SHIFT: c_int = 21;

// CPU_CA53_CFG_ARM_RST_CONTROL
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NCPUPORESET_SHIFT: c_int = 0;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NCPUPORESET_MASK: c_uint = 0x3;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NCORERESET_SHIFT: c_int = 4;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NCORERESET_MASK: c_uint = 0x30;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NL2RESET_SHIFT: c_int = 8;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NL2RESET_MASK: c_uint = 0x100;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NPRESETDBG_SHIFT: c_int = 12;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NPRESETDBG_MASK: c_uint = 0x1000;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NMBISTRESET_SHIFT: c_int = 16;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_NMBISTRESET_MASK: c_uint = 0x10000;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_WARMRSTREQ_SHIFT: c_int = 20;
pub const CPU_CA53_CFG_ARM_RST_CONTROL_WARMRSTREQ_MASK: c_uint = 0x300000;

// QM_IDLE_MASK is valid for all engines QM idle check

// CGM_IDLE_MASK is valid for all engines CGM idle check

pub const MME0_CTRL_ARCH_STATUS_SB_A_EMPTY_MASK: c_uint = 0x80;
pub const MME0_CTRL_ARCH_STATUS_SB_B_EMPTY_MASK: c_uint = 0x100;
pub const MME0_CTRL_ARCH_STATUS_WBC_AXI_IDLE_MASK: c_uint = 0x1000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum axi_id {
    AXI_ID_MME,
    AXI_ID_TPC,
    AXI_ID_DMA,
    AXI_ID_NIC,	/* Local NIC */
    AXI_ID_PCI,
    AXI_ID_CPU,
    AXI_ID_PSOC,
    AXI_ID_MMU,
    AXI_ID_NIC_FT	/* Feed-Through NIC */
}

// RAZWI initiator ID is built from the location in the chip and the AXI ID
pub const RAZWI_INITIATOR_AXI_ID_SHIFT: c_int = 20;
pub const RAZWI_INITIATOR_AXI_ID_MASK: c_uint = 0xF;
pub const RAZWI_INITIATOR_X_SHIFT: c_int = 24;
pub const RAZWI_INITIATOR_X_MASK: c_uint = 0xF;
pub const RAZWI_INITIATOR_Y_SHIFT: c_int = 28;
pub const RAZWI_INITIATOR_Y_MASK: c_uint = 0x7;

pub const PSOC_ETR_AXICTL_PROTCTRLBIT1_SHIFT: c_int = 1;
pub const PSOC_ETR_AXICTL_PROTCTRLBIT0_MASK: c_uint = 0x1;
pub const PSOC_ETR_AXICTL_PROTCTRLBIT1_MASK: c_uint = 0x2;
pub const PSOC_ETR_AXICTL_WRBURSTLEN_MASK: c_uint = 0xF00;
// STLB_CACHE_INV
pub const STLB_CACHE_INV_PRODUCER_INDEX_SHIFT: c_int = 0;
pub const STLB_CACHE_INV_PRODUCER_INDEX_MASK: c_uint = 0xFF;
pub const STLB_CACHE_INV_INDEX_MASK_SHIFT: c_int = 8;
pub const STLB_CACHE_INV_INDEX_MASK_MASK: c_uint = 0xFF00;
pub const MME_ACC_ACC_STALL_R_SHIFT: c_int = 0;
pub const MME_SBAB_SB_STALL_R_SHIFT: c_int = 0;
pub const PCIE_WRAP_LBW_PROT_OVR_RD_EN_MASK: c_uint = 0x700;
pub const PCIE_WRAP_LBW_PROT_OVR_WR_EN_MASK: c_uint = 0x7000;
pub const PCIE_WRAP_LBW_DRAIN_CFG_EN_SHIFT: c_int = 0;
pub const PCIE_WRAP_HBW_DRAIN_CFG_EN_SHIFT: c_int = 0;
// DMA_IF_HBM_CRED_EN
pub const DMA_IF_HBM_CRED_EN_READ_CREDIT_EN_SHIFT: c_int = 0;
pub const DMA_IF_HBM_CRED_EN_READ_CREDIT_EN_MASK: c_uint = 0x1;
pub const DMA_IF_HBM_CRED_EN_WRITE_CREDIT_EN_SHIFT: c_int = 1;
pub const DMA_IF_HBM_CRED_EN_WRITE_CREDIT_EN_MASK: c_uint = 0x2;
pub const DMA_IF_DOWN_CHX_SCRAM_SRAM_EN_VAL_SHIFT: c_int = 0;
pub const DMA_IF_DOWN_CHX_SCRAM_HBM_EN_VAL_SHIFT: c_int = 0;
pub const DMA_IF_DOWN_CHX_E2E_HBM_EN_VAL_SHIFT: c_int = 0;
pub const DMA_IF_DOWN_CHX_E2E_PCI_EN_VAL_SHIFT: c_int = 0;
pub const IF_RTR_CTRL_SCRAM_SRAM_EN_VAL_SHIFT: c_int = 0;
pub const IF_RTR_CTRL_SCRAM_HBM_EN_VAL_SHIFT: c_int = 0;
pub const IF_RTR_CTRL_E2E_HBM_EN_VAL_SHIFT: c_int = 0;
pub const IF_RTR_CTRL_E2E_PCI_EN_VAL_SHIFT: c_int = 0;
// MMU_UP_PAGE_ERROR_CAPTURE
pub const MMU_UP_PAGE_ERROR_CAPTURE_VA_49_32_MASK: c_uint = 0x3FFFF;
pub const MMU_UP_PAGE_ERROR_CAPTURE_ENTRY_VALID_MASK: c_uint = 0x40000;
// MMU_UP_ACCESS_ERROR_CAPTURE
pub const MMU_UP_ACCESS_ERROR_CAPTURE_VA_49_32_MASK: c_uint = 0x3FFFF;
pub const MMU_UP_ACCESS_ERROR_CAPTURE_ENTRY_VALID_MASK: c_uint = 0x40000;
pub const QM_ARB_ERR_MSG_EN_CHOISE_OVF_MASK: c_uint = 0x1;
pub const QM_ARB_ERR_MSG_EN_CHOISE_WDT_MASK: c_uint = 0x2;
pub const QM_ARB_ERR_MSG_EN_AXI_LBW_ERR_MASK: c_uint = 0x4;

pub const PCIE_AUX_FLR_CTRL_HW_CTRL_MASK: c_uint = 0x1;
pub const PCIE_AUX_FLR_CTRL_INT_MASK_MASK: c_uint = 0x2;
pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_STATUS_0_VALID_SHIFT: c_int = 0;
pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_STATUS_0_VALID_MASK: c_uint = 0x1;
pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_STATUS_0_PENDING_SHIFT: c_int = 1;
pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_STATUS_0_PENDING_MASK: c_uint = 0x1FE;
pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_SID_SHIFT: c_int = 0;
pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_SID_MASK: c_uint = 0xFF;
pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_MASK_SHIFT: c_int = 8;
pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_MASK_MASK: c_uint = 0xFF00;
pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_SOP_SHIFT: c_int = 16;
pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_SOP_MASK: c_uint = 0x10000;
pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_SOD_SHIFT: c_int = 17;
pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_SOD_MASK: c_uint = 0xFFFE0000;
pub const TPC0_QM_CP_STS_0_FENCE_ID_SHIFT: c_int = 20;
pub const TPC0_QM_CP_STS_0_FENCE_ID_MASK: c_uint = 0x300000;
pub const TPC0_QM_CP_STS_0_FENCE_IN_PROGRESS_SHIFT: c_int = 22;
pub const TPC0_QM_CP_STS_0_FENCE_IN_PROGRESS_MASK: c_uint = 0x400000;
