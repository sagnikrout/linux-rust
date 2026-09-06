//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/airoha/airoha_regs.h
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
// Copyright (c) 2024 AIROHA Inc
// Author: Lorenzo Bianconi <lorenzo@kernel.org>
//

// FE
pub const PSE_BASE: c_uint = 0x0100;
pub const CSR_IFC_BASE: c_uint = 0x0200;
pub const CDM1_BASE: c_uint = 0x0400;
pub const GDM1_BASE: c_uint = 0x0500;
pub const PPE1_BASE: c_uint = 0x0c00;
pub const PPE2_BASE: c_uint = 0x1c00;
pub const CDM2_BASE: c_uint = 0x1400;
pub const GDM2_BASE: c_uint = 0x1500;
pub const GDM3_BASE: c_uint = 0x1100;
pub const GDM4_BASE: c_uint = 0x2500;

pub const REG_FE_DMA_GLO_CFG: c_uint = 0x0000;

pub const REG_FE_RST_GLO_CFG: c_uint = 0x0004;

pub const REG_FE_FOE_TS: c_uint = 0x0010;
pub const REG_FE_WAN_PORT: c_uint = 0x0024;

pub const REG_FE_WAN_MAC_H: c_uint = 0x0030;
pub const REG_FE_LAN_MAC_H: c_uint = 0x0040;

pub const REG_FE_CDM1_OQ_MAP0: c_uint = 0x0050;
pub const REG_FE_CDM1_OQ_MAP1: c_uint = 0x0054;
pub const REG_FE_CDM1_OQ_MAP2: c_uint = 0x0058;
pub const REG_FE_CDM1_OQ_MAP3: c_uint = 0x005c;
pub const REG_FE_PCE_CFG: c_uint = 0x0070;

pub const REG_FE_PSE_QUEUE_CFG_WR: c_uint = 0x0080;

pub const REG_FE_PSE_QUEUE_CFG_VAL: c_uint = 0x0084;

pub const PSE_FQ_CFG: c_uint = 0x008c;

pub const REG_FE_PSE_BUF_SET: c_uint = 0x0090;

pub const REG_PSE_SHARE_USED_THD: c_uint = 0x0094;

pub const REG_GDM_MISC_CFG: c_uint = 0x0148;

pub const REG_FE_VIP_PORT_EN: c_uint = 0x01f0;
pub const REG_FE_IFC_PORT_EN: c_uint = 0x01f4;

pub const PPE_HASH_SEED: c_uint = 0x12345678;

pub const REG_IP_FRAG_FP: c_uint = 0x2010;

pub const REG_MC_VLAN_EN: c_uint = 0x2100;

pub const REG_MC_VLAN_CFG: c_uint = 0x2104;

pub const REG_MC_VLAN_DATA: c_uint = 0x2108;

pub const REG_SRC_PORT_FC_MAP6: c_uint = 0x2298;

pub const FC_MAP6_DEF_VALUE: c_uint = 0x1b1a1918;
pub const REG_WAN_MTU0: c_uint = 0x2300;

pub const REG_CDM5_RX_OQ1_DROP_CNT: c_uint = 0x29d4;
// QDMA
pub const REG_QDMA_GLOBAL_CFG: c_uint = 0x0004;

pub const REG_FWD_DSCP_BASE: c_uint = 0x0010;
pub const REG_FWD_BUF_BASE: c_uint = 0x0014;
pub const REG_HW_FWD_DSCP_CFG: c_uint = 0x0018;

// QDMA_CSR_INT_ENABLE1

// QDMA_CSR_INT_ENABLE2

// QDMA_CSR_INT_ENABLE3

// QDMA_CSR_INT_ENABLE4

// QDMA_CSR_INT_ENABLE5

pub const REG_INGRESS_TRTCM_CFG: c_uint = 0x0070;

pub const REG_LMGR_INIT_CFG: c_uint = 0x1000;

pub const REG_FWD_DSCP_LOW_THR: c_uint = 0x1004;

pub const REG_EGRESS_RATE_METER_CFG: c_uint = 0x100c;

pub const REG_EGRESS_TRTCM_CFG: c_uint = 0x1010;

pub const REG_TXWRR_MODE_CFG: c_uint = 0x1020;

pub const REG_TXWRR_WEIGHT_CFG: c_uint = 0x1024;

pub const REG_PSE_BUF_USAGE_CFG: c_uint = 0x1028;

pub const REG_GLB_TRTCM_CFG: c_uint = 0x1080;

pub const REG_TXQ_CNGST_CFG: c_uint = 0x10a0;

pub const REG_SLA_TRTCM_CFG: c_uint = 0x1150;

// CTRL

// DATA

// TX MSG0

// TX MSG1

// RX MSG0

// RX MSG1

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_qdma_desc {
    pub rsv: __le32,
    pub ctrl: __le32,
    pub addr: __le32,
    pub data: __le32,
    pub msg0: __le32,
    pub msg1: __le32,
    pub msg2: __le32,
    pub msg3: __le32,
}

// CTRL0

// CTRL1

// CTRL2

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_qdma_fwd_desc {
    pub addr: __le32,
    pub ctrl0: __le32,
    pub ctrl1: __le32,
    pub ctrl2: __le32,
    pub msg0: __le32,
    pub msg1: __le32,
    pub rsv0: __le32,
    pub rsv1: __le32,
}
