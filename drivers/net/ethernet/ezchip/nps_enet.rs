//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ezchip/nps_enet.h
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
// Copyright(c) 2015 EZchip Technologies.
//
// default values
pub const NPS_ENET_NAPI_POLL_WEIGHT: c_uint = 0x2;
pub const NPS_ENET_MAX_FRAME_LENGTH: c_uint = 0x3FFF;
pub const NPS_ENET_GE_MAC_CFG_0_TX_FC_RETR: c_uint = 0x7;
pub const NPS_ENET_GE_MAC_CFG_0_RX_IFG: c_uint = 0x5;
pub const NPS_ENET_GE_MAC_CFG_0_TX_IFG: c_uint = 0xC;
pub const NPS_ENET_GE_MAC_CFG_0_TX_PR_LEN: c_uint = 0x7;
pub const NPS_ENET_GE_MAC_CFG_2_STAT_EN: c_uint = 0x3;
pub const NPS_ENET_GE_MAC_CFG_3_RX_IFG_TH: c_uint = 0x14;
pub const NPS_ENET_GE_MAC_CFG_3_MAX_LEN: c_uint = 0x3FFC;
pub const NPS_ENET_ENABLE: c_int = 1;
pub const NPS_ENET_DISABLE: c_int = 0;
// register definitions
pub const NPS_ENET_REG_TX_CTL: c_uint = 0x800;
pub const NPS_ENET_REG_TX_BUF: c_uint = 0x808;
pub const NPS_ENET_REG_RX_CTL: c_uint = 0x810;
pub const NPS_ENET_REG_RX_BUF: c_uint = 0x818;
pub const NPS_ENET_REG_BUF_INT_ENABLE: c_uint = 0x8C0;
pub const NPS_ENET_REG_GE_MAC_CFG_0: c_uint = 0x1000;
pub const NPS_ENET_REG_GE_MAC_CFG_1: c_uint = 0x1004;
pub const NPS_ENET_REG_GE_MAC_CFG_2: c_uint = 0x1008;
pub const NPS_ENET_REG_GE_MAC_CFG_3: c_uint = 0x100C;
pub const NPS_ENET_REG_GE_RST: c_uint = 0x1400;
pub const NPS_ENET_REG_PHASE_FIFO_CTL: c_uint = 0x1404;
// Tx control register masks and shifts
pub const TX_CTL_NT_MASK: c_uint = 0x7FF;
pub const TX_CTL_NT_SHIFT: c_int = 0;
pub const TX_CTL_ET_MASK: c_uint = 0x4000;
pub const TX_CTL_ET_SHIFT: c_int = 14;
pub const TX_CTL_CT_MASK: c_uint = 0x8000;
pub const TX_CTL_CT_SHIFT: c_int = 15;
// Rx control register masks and shifts
pub const RX_CTL_NR_MASK: c_uint = 0x7FF;
pub const RX_CTL_NR_SHIFT: c_int = 0;
pub const RX_CTL_CRC_MASK: c_uint = 0x2000;
pub const RX_CTL_CRC_SHIFT: c_int = 13;
pub const RX_CTL_ER_MASK: c_uint = 0x4000;
pub const RX_CTL_ER_SHIFT: c_int = 14;
pub const RX_CTL_CR_MASK: c_uint = 0x8000;
pub const RX_CTL_CR_SHIFT: c_int = 15;
// Interrupt enable for data buffer events register masks and shifts
pub const RX_RDY_MASK: c_uint = 0x1;
pub const RX_RDY_SHIFT: c_int = 0;
pub const TX_DONE_MASK: c_uint = 0x2;
pub const TX_DONE_SHIFT: c_int = 1;
// Gbps Eth MAC Configuration 0 register masks and shifts
pub const CFG_0_RX_EN_MASK: c_uint = 0x1;
pub const CFG_0_RX_EN_SHIFT: c_int = 0;
pub const CFG_0_TX_EN_MASK: c_uint = 0x2;
pub const CFG_0_TX_EN_SHIFT: c_int = 1;
pub const CFG_0_TX_FC_EN_MASK: c_uint = 0x4;
pub const CFG_0_TX_FC_EN_SHIFT: c_int = 2;
pub const CFG_0_TX_PAD_EN_MASK: c_uint = 0x8;
pub const CFG_0_TX_PAD_EN_SHIFT: c_int = 3;
pub const CFG_0_TX_CRC_EN_MASK: c_uint = 0x10;
pub const CFG_0_TX_CRC_EN_SHIFT: c_int = 4;
pub const CFG_0_RX_FC_EN_MASK: c_uint = 0x20;
pub const CFG_0_RX_FC_EN_SHIFT: c_int = 5;
pub const CFG_0_RX_CRC_STRIP_MASK: c_uint = 0x40;
pub const CFG_0_RX_CRC_STRIP_SHIFT: c_int = 6;
pub const CFG_0_RX_CRC_IGNORE_MASK: c_uint = 0x80;
pub const CFG_0_RX_CRC_IGNORE_SHIFT: c_int = 7;
pub const CFG_0_RX_LENGTH_CHECK_EN_MASK: c_uint = 0x100;
pub const CFG_0_RX_LENGTH_CHECK_EN_SHIFT: c_int = 8;
pub const CFG_0_TX_FC_RETR_MASK: c_uint = 0xE00;
pub const CFG_0_TX_FC_RETR_SHIFT: c_int = 9;
pub const CFG_0_RX_IFG_MASK: c_uint = 0xF000;
pub const CFG_0_RX_IFG_SHIFT: c_int = 12;
pub const CFG_0_TX_IFG_MASK: c_uint = 0x3F0000;
pub const CFG_0_TX_IFG_SHIFT: c_int = 16;
pub const CFG_0_RX_PR_CHECK_EN_MASK: c_uint = 0x400000;
pub const CFG_0_RX_PR_CHECK_EN_SHIFT: c_int = 22;
pub const CFG_0_NIB_MODE_MASK: c_uint = 0x800000;
pub const CFG_0_NIB_MODE_SHIFT: c_int = 23;
pub const CFG_0_TX_IFG_NIB_MASK: c_uint = 0xF000000;
pub const CFG_0_TX_IFG_NIB_SHIFT: c_int = 24;
pub const CFG_0_TX_PR_LEN_MASK: c_uint = 0xF0000000;
pub const CFG_0_TX_PR_LEN_SHIFT: c_int = 28;
// Gbps Eth MAC Configuration 1 register masks and shifts
pub const CFG_1_OCTET_0_MASK: c_uint = 0x000000FF;
pub const CFG_1_OCTET_0_SHIFT: c_int = 0;
pub const CFG_1_OCTET_1_MASK: c_uint = 0x0000FF00;
pub const CFG_1_OCTET_1_SHIFT: c_int = 8;
pub const CFG_1_OCTET_2_MASK: c_uint = 0x00FF0000;
pub const CFG_1_OCTET_2_SHIFT: c_int = 16;
pub const CFG_1_OCTET_3_MASK: c_uint = 0xFF000000;
pub const CFG_1_OCTET_3_SHIFT: c_int = 24;
// Gbps Eth MAC Configuration 2 register masks and shifts
pub const CFG_2_OCTET_4_MASK: c_uint = 0x000000FF;
pub const CFG_2_OCTET_4_SHIFT: c_int = 0;
pub const CFG_2_OCTET_5_MASK: c_uint = 0x0000FF00;
pub const CFG_2_OCTET_5_SHIFT: c_int = 8;
pub const CFG_2_DISK_MC_MASK: c_uint = 0x00100000;
pub const CFG_2_DISK_MC_SHIFT: c_int = 20;
pub const CFG_2_DISK_BC_MASK: c_uint = 0x00200000;
pub const CFG_2_DISK_BC_SHIFT: c_int = 21;
pub const CFG_2_DISK_DA_MASK: c_uint = 0x00400000;
pub const CFG_2_DISK_DA_SHIFT: c_int = 22;
pub const CFG_2_STAT_EN_MASK: c_uint = 0x3000000;
pub const CFG_2_STAT_EN_SHIFT: c_int = 24;
pub const CFG_2_TRANSMIT_FLUSH_EN_MASK: c_uint = 0x80000000;
pub const CFG_2_TRANSMIT_FLUSH_EN_SHIFT: c_int = 31;
// Gbps Eth MAC Configuration 3 register masks and shifts
pub const CFG_3_TM_HD_MODE_MASK: c_uint = 0x1;
pub const CFG_3_TM_HD_MODE_SHIFT: c_int = 0;
pub const CFG_3_RX_CBFC_EN_MASK: c_uint = 0x2;
pub const CFG_3_RX_CBFC_EN_SHIFT: c_int = 1;
pub const CFG_3_RX_CBFC_REDIR_EN_MASK: c_uint = 0x4;
pub const CFG_3_RX_CBFC_REDIR_EN_SHIFT: c_int = 2;
pub const CFG_3_REDIRECT_CBFC_SEL_MASK: c_uint = 0x18;
pub const CFG_3_REDIRECT_CBFC_SEL_SHIFT: c_int = 3;
pub const CFG_3_CF_DROP_MASK: c_uint = 0x20;
pub const CFG_3_CF_DROP_SHIFT: c_int = 5;
pub const CFG_3_CF_TIMEOUT_MASK: c_uint = 0x3C0;
pub const CFG_3_CF_TIMEOUT_SHIFT: c_int = 6;
pub const CFG_3_RX_IFG_TH_MASK: c_uint = 0x7C00;
pub const CFG_3_RX_IFG_TH_SHIFT: c_int = 10;
pub const CFG_3_TX_CBFC_EN_MASK: c_uint = 0x8000;
pub const CFG_3_TX_CBFC_EN_SHIFT: c_int = 15;
pub const CFG_3_MAX_LEN_MASK: c_uint = 0x3FFF0000;
pub const CFG_3_MAX_LEN_SHIFT: c_int = 16;
pub const CFG_3_EXT_OOB_CBFC_SEL_MASK: c_uint = 0xC0000000;
pub const CFG_3_EXT_OOB_CBFC_SEL_SHIFT: c_int = 30;
// GE MAC, PCS reset control register masks and shifts
pub const RST_SPCS_MASK: c_uint = 0x1;
pub const RST_SPCS_SHIFT: c_int = 0;
pub const RST_GMAC_0_MASK: c_uint = 0x100;
pub const RST_GMAC_0_SHIFT: c_int = 8;
// Tx phase sync FIFO control register masks and shifts
pub const PHASE_FIFO_CTL_RST_MASK: c_uint = 0x1;
pub const PHASE_FIFO_CTL_RST_SHIFT: c_int = 0;
pub const PHASE_FIFO_CTL_INIT_MASK: c_uint = 0x2;
pub const PHASE_FIFO_CTL_INIT_SHIFT: c_int = 1;
//
// struct nps_enet_priv - Storage of ENET's private information.
// @regs_base:      Base address of ENET memory-mapped control registers.
// @irq:            For RX/TX IRQ number.
// @tx_skb:         socket buffer of sent frame.
// @napi:           Structure for NAPI.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nps_enet_priv {
    pub regs_base: *mut void __iomem,
    pub irq: i32,
    pub tx_skb: *mut sk_buff,
    pub napi: napi_struct,
    pub ge_mac_cfg_2_value: u32,
    pub ge_mac_cfg_3_value: u32,
}

//
// nps_enet_reg_set - Sets ENET register with provided value.
// @priv:       Pointer to EZchip ENET private data structure.
// @reg:        Register offset from base address.
// @value:      Value to set in register.
//
// nps_enet_reg_get - Gets value of specified ENET register.
// @priv:       Pointer to EZchip ENET private data structure.
// @reg:        Register offset from base address.
//
// returns:     Value of requested register.
//
extern "C" {
    pub fn ioread32be(reg: priv->regs_base +) -> return;
}
