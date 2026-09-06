//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeon_ep/octep_regs_cnxk_pf.h
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
// Marvell Octeon EP (EndPoint) Ethernet Driver
//
// Copyright (C) 2020 Marvell.
//
// ############################ RST #########################
pub const CNXK_RST_BOOT: c_uint = 0x000087E006001600ULL;
pub const CNXK_RST_CHIP_DOMAIN_W1S: c_uint = 0x000087E006001810ULL;
pub const CNXK_RST_CORE_DOMAIN_W1S: c_uint = 0x000087E006001820ULL;
pub const CNXK_RST_CORE_DOMAIN_W1C: c_uint = 0x000087E006001828ULL;
pub const CNXK_CONFIG_XPANSION_BAR: c_uint = 0x38;
pub const CNXK_CONFIG_PCIE_CAP: c_uint = 0x70;
pub const CNXK_CONFIG_PCIE_DEVCAP: c_uint = 0x74;
pub const CNXK_CONFIG_PCIE_DEVCTL: c_uint = 0x78;
pub const CNXK_CONFIG_PCIE_LINKCAP: c_uint = 0x7C;
pub const CNXK_CONFIG_PCIE_LINKCTL: c_uint = 0x80;
pub const CNXK_CONFIG_PCIE_SLOTCAP: c_uint = 0x84;
pub const CNXK_CONFIG_PCIE_SLOTCTL: c_uint = 0x88;
pub const CNXK_PCIE_SRIOV_FDL: c_uint = 0x188      /* 0x98 */;
pub const CNXK_PCIE_SRIOV_FDL_BIT_POS: c_uint = 0x10;
pub const CNXK_PCIE_SRIOV_FDL_MASK: c_uint = 0xFF;
pub const CNXK_CONFIG_PCIE_FLTMSK: c_uint = 0x720;
// ################# Offsets of RING, EPF, MAC #########################

// ################# Scratch Registers #########################
pub const CNXK_SDP_EPF_SCRATCH: c_uint = 0x209E0;
// ################# Window Registers #########################
pub const CNXK_SDP_WIN_WR_ADDR64: c_uint = 0x20000;
pub const CNXK_SDP_WIN_RD_ADDR64: c_uint = 0x20010;
pub const CNXK_SDP_WIN_WR_DATA64: c_uint = 0x20020;
pub const CNXK_SDP_WIN_WR_MASK_REG: c_uint = 0x20030;
pub const CNXK_SDP_WIN_RD_DATA64: c_uint = 0x20040;
pub const CNXK_SDP_MAC_NUMBER: c_uint = 0x2C100;
// ################# Global Previliged registers #########################
pub const CNXK_SDP_EPF_RINFO: c_uint = 0x209F0;

// SDP Function select
pub const CNXK_SDP_FUNC_SEL_EPF_BIT_POS: c_int = 7;
pub const CNXK_SDP_FUNC_SEL_FUNC_BIT_POS: c_int = 0;
// ##### RING IN (Into device from PCI: Tx Ring) REGISTERS ####
pub const CNXK_SDP_R_IN_CONTROL_START: c_uint = 0x10000;
pub const CNXK_SDP_R_IN_ENABLE_START: c_uint = 0x10010;
pub const CNXK_SDP_R_IN_INSTR_BADDR_START: c_uint = 0x10020;
pub const CNXK_SDP_R_IN_INSTR_RSIZE_START: c_uint = 0x10030;
pub const CNXK_SDP_R_IN_INSTR_DBELL_START: c_uint = 0x10040;
pub const CNXK_SDP_R_IN_CNTS_START: c_uint = 0x10050;
pub const CNXK_SDP_R_IN_INT_LEVELS_START: c_uint = 0x10060;
pub const CNXK_SDP_R_IN_PKT_CNT_START: c_uint = 0x10080;
pub const CNXK_SDP_R_IN_BYTE_CNT_START: c_uint = 0x10090;

// Rings per Virtual Function

// Number of instructions to be read in one MAC read request.
// setting to Max value(4)
//

// ##### RING OUT (out from device to PCI host: Rx Ring) REGISTERS ####
pub const CNXK_SDP_R_OUT_CNTS_START: c_uint = 0x10100;
pub const CNXK_SDP_R_OUT_INT_LEVELS_START: c_uint = 0x10110;
pub const CNXK_SDP_R_OUT_SLIST_BADDR_START: c_uint = 0x10120;
pub const CNXK_SDP_R_OUT_SLIST_RSIZE_START: c_uint = 0x10130;
pub const CNXK_SDP_R_OUT_SLIST_DBELL_START: c_uint = 0x10140;
pub const CNXK_SDP_R_OUT_CONTROL_START: c_uint = 0x10150;
pub const CNXK_SDP_R_OUT_WMARK_START: c_uint = 0x10160;
pub const CNXK_SDP_R_OUT_ENABLE_START: c_uint = 0x10170;
pub const CNXK_SDP_R_OUT_PKT_CNT_START: c_uint = 0x10180;
pub const CNXK_SDP_R_OUT_BYTE_CNT_START: c_uint = 0x10190;

// ------------------ R_OUT Masks ----------------

// ############### Interrupt Moderation Registers ###############
pub const CNXK_SDP_R_IN_INT_MDRT_CTL0_START: c_uint = 0x10280;
pub const CNXK_SDP_R_IN_INT_MDRT_CTL1_START: c_uint = 0x102A0;
pub const CNXK_SDP_R_IN_INT_MDRT_DBG_START: c_uint = 0x102C0;
pub const CNXK_SDP_R_OUT_INT_MDRT_CTL0_START: c_uint = 0x10380;
pub const CNXK_SDP_R_OUT_INT_MDRT_CTL1_START: c_uint = 0x103A0;
pub const CNXK_SDP_R_OUT_INT_MDRT_DBG_START: c_uint = 0x103C0;
pub const CNXK_SDP_R_MBOX_ISM_START: c_uint = 0x10500;
pub const CNXK_SDP_R_OUT_CNTS_ISM_START: c_uint = 0x10510;
pub const CNXK_SDP_R_IN_CNTS_ISM_START: c_uint = 0x10520;

// ##################### Mail Box Registers ##########################
// INT register for VF. when a MBOX write from PF happed to a VF,
// corresponding bit will be set in this register as well as in
// PF_VF_INT register.
//
// This is a RO register, the int can be cleared by writing 1 to PF_VF_INT
//
// Basically first 3 are from PF to VF. The last one is data from VF to PF
pub const CNXK_SDP_R_MBOX_PF_VF_DATA_START: c_uint = 0x10210;
pub const CNXK_SDP_R_MBOX_PF_VF_INT_START: c_uint = 0x10220;
pub const CNXK_SDP_R_MBOX_VF_PF_DATA_START: c_uint = 0x10230;
pub const CNXK_SDP_MBOX_VF_PF_DATA_START: c_uint = 0x24000;
pub const CNXK_SDP_MBOX_PF_VF_DATA_START: c_uint = 0x22000;

// ##################### Interrupt Registers ##########################
pub const CNXK_SDP_R_ERR_TYPE_START: c_uint = 0x10400;

pub const CNXK_SDP_R_MBOX_ISM_START: c_uint = 0x10500;
pub const CNXK_SDP_R_OUT_CNTS_ISM_START: c_uint = 0x10510;
pub const CNXK_SDP_R_IN_CNTS_ISM_START: c_uint = 0x10520;

pub const CNXK_SDP_EPF_MBOX_RINT_START: c_uint = 0x20100;
pub const CNXK_SDP_EPF_MBOX_RINT_W1S_START: c_uint = 0x20120;
pub const CNXK_SDP_EPF_MBOX_RINT_ENA_W1C_START: c_uint = 0x20140;
pub const CNXK_SDP_EPF_MBOX_RINT_ENA_W1S_START: c_uint = 0x20160;
pub const CNXK_SDP_EPF_VFIRE_RINT_START: c_uint = 0x20180;
pub const CNXK_SDP_EPF_VFIRE_RINT_W1S_START: c_uint = 0x201A0;
pub const CNXK_SDP_EPF_VFIRE_RINT_ENA_W1C_START: c_uint = 0x201C0;
pub const CNXK_SDP_EPF_VFIRE_RINT_ENA_W1S_START: c_uint = 0x201E0;
pub const CNXK_SDP_EPF_IRERR_RINT: c_uint = 0x20200;
pub const CNXK_SDP_EPF_IRERR_RINT_W1S: c_uint = 0x20210;
pub const CNXK_SDP_EPF_IRERR_RINT_ENA_W1C: c_uint = 0x20220;
pub const CNXK_SDP_EPF_IRERR_RINT_ENA_W1S: c_uint = 0x20230;
pub const CNXK_SDP_EPF_VFORE_RINT_START: c_uint = 0x20240;
pub const CNXK_SDP_EPF_VFORE_RINT_W1S_START: c_uint = 0x20260;
pub const CNXK_SDP_EPF_VFORE_RINT_ENA_W1C_START: c_uint = 0x20280;
pub const CNXK_SDP_EPF_VFORE_RINT_ENA_W1S_START: c_uint = 0x202A0;
pub const CNXK_SDP_EPF_ORERR_RINT: c_uint = 0x20320;
pub const CNXK_SDP_EPF_ORERR_RINT_W1S: c_uint = 0x20330;
pub const CNXK_SDP_EPF_ORERR_RINT_ENA_W1C: c_uint = 0x20340;
pub const CNXK_SDP_EPF_ORERR_RINT_ENA_W1S: c_uint = 0x20350;
pub const CNXK_SDP_EPF_OEI_RINT: c_uint = 0x20400;
pub const CNXK_SDP_EPF_OEI_RINT_W1S: c_uint = 0x20500;
pub const CNXK_SDP_EPF_OEI_RINT_ENA_W1C: c_uint = 0x20600;
pub const CNXK_SDP_EPF_OEI_RINT_ENA_W1S: c_uint = 0x20700;
pub const CNXK_SDP_EPF_DMA_RINT: c_uint = 0x20800;
pub const CNXK_SDP_EPF_DMA_RINT_W1S: c_uint = 0x20810;
pub const CNXK_SDP_EPF_DMA_RINT_ENA_W1C: c_uint = 0x20820;
pub const CNXK_SDP_EPF_DMA_RINT_ENA_W1S: c_uint = 0x20830;
pub const CNXK_SDP_EPF_DMA_INT_LEVEL_START: c_uint = 0x20840;
pub const CNXK_SDP_EPF_DMA_CNT_START: c_uint = 0x20860;
pub const CNXK_SDP_EPF_DMA_TIM_START: c_uint = 0x20880;
pub const CNXK_SDP_EPF_MISC_RINT: c_uint = 0x208A0;
pub const CNXK_SDP_EPF_MISC_RINT_W1S: c_uint = 0x208B0;
pub const CNXK_SDP_EPF_MISC_RINT_ENA_W1C: c_uint = 0x208C0;
pub const CNXK_SDP_EPF_MISC_RINT_ENA_W1S: c_uint = 0x208D0;
pub const CNXK_SDP_EPF_DMA_VF_RINT_START: c_uint = 0x208E0;
pub const CNXK_SDP_EPF_DMA_VF_RINT_W1S_START: c_uint = 0x20900;
pub const CNXK_SDP_EPF_DMA_VF_RINT_ENA_W1C_START: c_uint = 0x20920;
pub const CNXK_SDP_EPF_DMA_VF_RINT_ENA_W1S_START: c_uint = 0x20940;
pub const CNXK_SDP_EPF_PP_VF_RINT_START: c_uint = 0x20960;
pub const CNXK_SDP_EPF_PP_VF_RINT_W1S_START: c_uint = 0x20980;
pub const CNXK_SDP_EPF_PP_VF_RINT_ENA_W1C_START: c_uint = 0x209A0;
pub const CNXK_SDP_EPF_PP_VF_RINT_ENA_W1S_START: c_uint = 0x209C0;

// ------------------ Interrupt Masks ----------------

// ####################### Ring Mapping Registers ##################################
pub const CNXK_SDP_EPVF_RING_START: c_uint = 0x26000;
pub const CNXK_SDP_IN_RING_TB_MAP_START: c_uint = 0x28000;
pub const CNXK_SDP_IN_RATE_LIMIT_START: c_uint = 0x2A000;
pub const CNXK_SDP_MAC_PF_RING_CTL_START: c_uint = 0x2C000;

// Number of non-queue interrupts in CNXKxx
pub const CNXK_NUM_NON_IOQ_INTR: c_int = 32;
// bit 0 for control mbox interrupt

// bit 1 for firmware heartbeat interrupt

// Register defines for use with CNXK_PEMX_PFX_CSX_PFCFGX
pub const CNXK_PCIEEP_VSECST_CTL: c_uint = 0x418;
pub const CNXK_PEM_BAR4_INDEX: c_int = 7;
pub const CNXK_PEM_BAR4_INDEX_SIZE: c_uint = 0x400000ULL;

