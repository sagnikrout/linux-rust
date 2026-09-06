//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/apm/xgene/xgene_enet_hw.h
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
// Applied Micro X-Gene SoC Ethernet Driver
//
// Copyright (c) 2014, Applied Micro Circuits Corporation
// Authors: Iyappan Subramanian <isubramanian@apm.com>
// Ravi Patel <rapatel@apm.com>
// Keyur Chudgar <kchudgar@apm.com>
//

// clears and then set bits
// dst &= ~mask;
// dst |= (val << start) & mask;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_enet_rm {
    RM0,
    RM1,
    RM3 = 3
}

pub const CSR_RING_ID: c_uint = 0x0008;

pub const CSR_RING_ID_BUF: c_uint = 0x000c;
pub const CSR_PBM_COAL: c_uint = 0x0014;
pub const CSR_PBM_CTICK0: c_uint = 0x0018;
pub const CSR_PBM_CTICK1: c_uint = 0x001c;
pub const CSR_PBM_CTICK2: c_uint = 0x0020;
pub const CSR_PBM_CTICK3: c_uint = 0x0024;
pub const CSR_THRESHOLD0_SET1: c_uint = 0x0030;
pub const CSR_THRESHOLD1_SET1: c_uint = 0x0034;
pub const CSR_RING_NE_INT_MODE: c_uint = 0x017c;
pub const CSR_RING_CONFIG: c_uint = 0x006c;
pub const CSR_RING_WR_BASE: c_uint = 0x0070;
pub const NUM_RING_CONFIG: c_int = 5;
pub const BUFPOOL_MODE: c_int = 3;
pub const INC_DEC_CMD_ADDR: c_uint = 0x002c;
pub const UDP_HDR_SIZE: c_int = 2;
pub const BUF_LEN_CODE_2K: c_uint = 0x5000;

// Empty slot soft signature
pub const EMPTY_SLOT_INDEX: c_int = 1;

pub const WORK_DESC_SIZE: c_int = 32;
pub const BUFPOOL_DESC_SIZE: c_int = 16;

pub const SELTHRSH_POS: c_int = 3;
pub const SELTHRSH_LEN: c_int = 3;
pub const RINGADDRL_POS: c_int = 5;
pub const RINGADDRL_LEN: c_int = 27;
pub const RINGADDRH_POS: c_int = 0;
pub const RINGADDRH_LEN: c_int = 7;
pub const RINGSIZE_POS: c_int = 23;
pub const RINGSIZE_LEN: c_int = 3;
pub const RINGTYPE_POS: c_int = 19;
pub const RINGTYPE_LEN: c_int = 2;
pub const RINGMODE_POS: c_int = 20;
pub const RINGMODE_LEN: c_int = 3;
pub const RECOMTIMEOUTL_POS: c_int = 28;
pub const RECOMTIMEOUTL_LEN: c_int = 4;
pub const RECOMTIMEOUTH_POS: c_int = 0;
pub const RECOMTIMEOUTH_LEN: c_int = 3;
pub const NUMMSGSINQ_POS: c_int = 1;
pub const NUMMSGSINQ_LEN: c_int = 16;

pub const MAC_OFFSET: c_uint = 0x30;
pub const OFFSET_4: c_uint = 0x04;
pub const OFFSET_8: c_uint = 0x08;
pub const BLOCK_ETH_CSR_OFFSET: c_uint = 0x2000;
pub const BLOCK_ETH_CLE_CSR_OFFSET: c_uint = 0x6000;
pub const BLOCK_ETH_RING_IF_OFFSET: c_uint = 0x9000;
pub const BLOCK_ETH_CLKRST_CSR_OFFSET: c_uint = 0xc000;
pub const BLOCK_ETH_DIAG_CSR_OFFSET: c_uint = 0xD000;
pub const BLOCK_ETH_MAC_OFFSET: c_uint = 0x0000;
pub const BLOCK_ETH_STATS_OFFSET: c_uint = 0x0000;
pub const BLOCK_ETH_MAC_CSR_OFFSET: c_uint = 0x2800;
pub const CLKEN_ADDR: c_uint = 0xc208;
pub const SRST_ADDR: c_uint = 0xc200;
pub const MAC_ADDR_REG_OFFSET: c_uint = 0x00;
pub const MAC_COMMAND_REG_OFFSET: c_uint = 0x04;
pub const MAC_WRITE_REG_OFFSET: c_uint = 0x08;
pub const MAC_READ_REG_OFFSET: c_uint = 0x0c;
pub const MAC_COMMAND_DONE_REG_OFFSET: c_uint = 0x10;
pub const STAT_ADDR_REG_OFFSET: c_uint = 0x14;
pub const STAT_COMMAND_REG_OFFSET: c_uint = 0x18;
pub const STAT_WRITE_REG_OFFSET: c_uint = 0x1c;
pub const STAT_READ_REG_OFFSET: c_uint = 0x20;
pub const STAT_COMMAND_DONE_REG_OFFSET: c_uint = 0x24;
pub const PCS_ADDR_REG_OFFSET: c_uint = 0x00;
pub const PCS_COMMAND_REG_OFFSET: c_uint = 0x04;
pub const PCS_WRITE_REG_OFFSET: c_uint = 0x08;
pub const PCS_READ_REG_OFFSET: c_uint = 0x0c;
pub const PCS_COMMAND_DONE_REG_OFFSET: c_uint = 0x10;
pub const MII_MGMT_CONFIG_ADDR: c_uint = 0x20;
pub const MII_MGMT_COMMAND_ADDR: c_uint = 0x24;
pub const MII_MGMT_ADDRESS_ADDR: c_uint = 0x28;
pub const MII_MGMT_CONTROL_ADDR: c_uint = 0x2c;
pub const MII_MGMT_STATUS_ADDR: c_uint = 0x30;
pub const MII_MGMT_INDICATORS_ADDR: c_uint = 0x34;

pub const ENET_SPARE_CFG_REG_ADDR: c_uint = 0x0750;
pub const RSIF_CONFIG_REG_ADDR: c_uint = 0x0010;
pub const RSIF_RAM_DBG_REG0_ADDR: c_uint = 0x0048;
pub const RGMII_REG_0_ADDR: c_uint = 0x07e0;
pub const CFG_LINK_AGGR_RESUME_0_ADDR: c_uint = 0x07c8;
pub const DEBUG_REG_ADDR: c_uint = 0x0700;
pub const CFG_BYPASS_ADDR: c_uint = 0x0294;
pub const CLE_BYPASS_REG0_0_ADDR: c_uint = 0x0490;
pub const CLE_BYPASS_REG1_0_ADDR: c_uint = 0x0494;

pub const CSR_ECM_CFG_0_ADDR: c_uint = 0x0220;
pub const CSR_ECM_CFG_1_ADDR: c_uint = 0x0224;
pub const CSR_MULTI_DPF0_ADDR: c_uint = 0x0230;
pub const RXBUF_PAUSE_THRESH: c_uint = 0x0534;
pub const RXBUF_PAUSE_OFF_THRESH: c_uint = 0x0540;
pub const DEF_PAUSE_THRES: c_uint = 0x7d;
pub const DEF_PAUSE_OFF_THRES: c_uint = 0x6d;
pub const DEF_QUANTA: c_uint = 0x8000;
pub const NORM_PAUSE_OPCODE: c_uint = 0x0001;

pub const ICM_CONFIG0_REG_0_ADDR: c_uint = 0x0400;
pub const ICM_CONFIG2_REG_0_ADDR: c_uint = 0x0410;
pub const ECM_CONFIG0_REG_0_ADDR: c_uint = 0x0500;
pub const ECM_CONFIG0_REG_1_ADDR: c_uint = 0x0504;
pub const ICM_ECM_DROP_COUNT_REG0_ADDR: c_uint = 0x0508;
pub const ICM_ECM_DROP_COUNT_REG1_ADDR: c_uint = 0x050c;
pub const RX_DV_GATE_REG_0_ADDR: c_uint = 0x05fc;

pub const ENET_CFGSSQMIFPRESET_ADDR: c_uint = 0x14;
pub const ENET_CFGSSQMIWQRESET_ADDR: c_uint = 0x1c;
pub const ENET_CFGSSQMIWQASSOC_ADDR: c_uint = 0xe0;
pub const ENET_CFGSSQMIFPQASSOC_ADDR: c_uint = 0xdc;
pub const ENET_CFGSSQMIQMLITEFPQASSOC_ADDR: c_uint = 0xf0;
pub const ENET_CFGSSQMIQMLITEWQASSOC_ADDR: c_uint = 0xf4;
pub const ENET_CFG_MEM_RAM_SHUTDOWN_ADDR: c_uint = 0x70;
pub const ENET_BLOCK_MEM_RDY_ADDR: c_uint = 0x74;
pub const MAC_CONFIG_1_ADDR: c_uint = 0x00;
pub const MAC_CONFIG_2_ADDR: c_uint = 0x04;
pub const MAX_FRAME_LEN_ADDR: c_uint = 0x10;
pub const INTERFACE_CONTROL_ADDR: c_uint = 0x38;
pub const STATION_ADDR0_ADDR: c_uint = 0x40;
pub const STATION_ADDR1_ADDR: c_uint = 0x44;

pub const TR64_ADDR: c_uint = 0x20;
pub const TR127_ADDR: c_uint = 0x21;
pub const TR255_ADDR: c_uint = 0x22;
pub const TR511_ADDR: c_uint = 0x23;
pub const TR1K_ADDR: c_uint = 0x24;
pub const TRMAX_ADDR: c_uint = 0x25;
pub const TRMGV_ADDR: c_uint = 0x26;
pub const RFCS_ADDR: c_uint = 0x29;
pub const RMCA_ADDR: c_uint = 0x2a;
pub const RBCA_ADDR: c_uint = 0x2b;
pub const RXCF_ADDR: c_uint = 0x2c;
pub const RXPF_ADDR: c_uint = 0x2d;
pub const RXUO_ADDR: c_uint = 0x2e;
pub const RALN_ADDR: c_uint = 0x2f;
pub const RFLR_ADDR: c_uint = 0x30;
pub const RCDE_ADDR: c_uint = 0x31;
pub const RCSE_ADDR: c_uint = 0x32;
pub const RUND_ADDR: c_uint = 0x33;
pub const ROVR_ADDR: c_uint = 0x34;
pub const RFRG_ADDR: c_uint = 0x35;
pub const RJBR_ADDR: c_uint = 0x36;
pub const RDRP_ADDR: c_uint = 0x37;
pub const TMCA_ADDR: c_uint = 0x3a;
pub const TBCA_ADDR: c_uint = 0x3b;
pub const TXPF_ADDR: c_uint = 0x3c;
pub const TDFR_ADDR: c_uint = 0x3d;
pub const TEDF_ADDR: c_uint = 0x3e;
pub const TSCL_ADDR: c_uint = 0x3f;
pub const TMCL_ADDR: c_uint = 0x40;
pub const TLCL_ADDR: c_uint = 0x41;
pub const TXCL_ADDR: c_uint = 0x42;
pub const TNCL_ADDR: c_uint = 0x43;
pub const TPFH_ADDR: c_uint = 0x44;
pub const TDRP_ADDR: c_uint = 0x45;
pub const TJBR_ADDR: c_uint = 0x46;
pub const TFCS_ADDR: c_uint = 0x47;
pub const TXCF_ADDR: c_uint = 0x48;
pub const TOVR_ADDR: c_uint = 0x49;
pub const TUND_ADDR: c_uint = 0x4a;
pub const TFRG_ADDR: c_uint = 0x4b;
pub const DUMP_ADDR: c_uint = 0x27;

pub const TSO_IPPROTO_TCP: c_int = 1;
pub const USERINFO_POS: c_int = 0;
pub const USERINFO_LEN: c_int = 32;
pub const FPQNUM_POS: c_int = 32;
pub const FPQNUM_LEN: c_int = 12;
pub const ELERR_POS: c_int = 46;
pub const ELERR_LEN: c_int = 2;
pub const NV_POS: c_int = 50;
pub const NV_LEN: c_int = 1;
pub const LL_POS: c_int = 51;
pub const LL_LEN: c_int = 1;
pub const LERR_POS: c_int = 60;
pub const LERR_LEN: c_int = 3;
pub const STASH_POS: c_int = 52;
pub const STASH_LEN: c_int = 2;
pub const BUFDATALEN_POS: c_int = 48;
pub const BUFDATALEN_LEN: c_int = 15;
pub const DATAADDR_POS: c_int = 0;
pub const DATAADDR_LEN: c_int = 42;
pub const COHERENT_POS: c_int = 63;
pub const HENQNUM_POS: c_int = 48;
pub const HENQNUM_LEN: c_int = 12;
pub const TYPESEL_POS: c_int = 44;
pub const TYPESEL_LEN: c_int = 4;
pub const ETHHDR_POS: c_int = 12;
pub const ETHHDR_LEN: c_int = 8;

pub const TCPHDR_POS: c_int = 0;
pub const TCPHDR_LEN: c_int = 6;
pub const IPHDR_POS: c_int = 6;
pub const IPHDR_LEN: c_int = 6;
pub const MSS_POS: c_int = 20;
pub const MSS_LEN: c_int = 2;

pub const EC_LEN: c_int = 1;

pub const IS_LEN: c_int = 1;
pub const TYPE_ETH_WORK_MESSAGE_POS: c_int = 44;
pub const LL_BYTES_MSB_POS: c_int = 56;
pub const LL_BYTES_MSB_LEN: c_int = 8;
pub const LL_BYTES_LSB_POS: c_int = 48;
pub const LL_BYTES_LSB_LEN: c_int = 12;
pub const LL_LEN_POS: c_int = 48;
pub const LL_LEN_LEN: c_int = 8;

pub const TSO_MSS0_POS: c_int = 0;
pub const TSO_MSS0_LEN: c_int = 14;
pub const TSO_MSS1_POS: c_int = 16;
pub const TSO_MSS1_LEN: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_enet_raw_desc {
    pub m0: __le64,
    pub m1: __le64,
    pub m2: __le64,
    pub m3: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_enet_raw_desc16 {
    pub m0: __le64,
    pub m1: __le64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_enet_ring_cfgsize {
    RING_CFGSIZE_512B,
    RING_CFGSIZE_2KB,
    RING_CFGSIZE_16KB,
    RING_CFGSIZE_64KB,
    RING_CFGSIZE_512KB,
    RING_CFGSIZE_INVALID
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_enet_ring_type {
    RING_DISABLED,
    RING_REGULAR,
    RING_BUFPOOL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_ring_owner {
    RING_OWNER_ETH0,
    RING_OWNER_ETH1,
    RING_OWNER_CPU = 15,
    RING_OWNER_INVALID
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_enet_ring_bufnum {
    RING_BUFNUM_REGULAR = 0x0,
    RING_BUFNUM_BUFPOOL = 0x20,
    RING_BUFNUM_INVALID
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_enet_err_code {
    HBF_READ_DATA = 3,
    HBF_LL_READ = 4,
    BAD_WORK_MSG = 6,
    BUFPOOL_TIMEOUT = 15,
    INGRESS_CRC = 16,
    INGRESS_CHECKSUM = 17,
    INGRESS_TRUNC_FRAME = 18,
    INGRESS_PKT_LEN = 19,
    INGRESS_PKT_UNDER = 20,
    INGRESS_FIFO_OVERRUN = 21,
    INGRESS_CHECKSUM_COMPUTE = 26,
    ERR_CODE_INVALID
}

extern "C" {
    pub fn xgene_enet_mdio_config(pdata: *mut xgene_enet_pdata) -> c_int;
}
extern "C" {
    pub fn xgene_enet_mdio_remove(pdata: *mut xgene_enet_pdata);
}
extern "C" {
    pub fn xgene_ring_mgr_init(p: *mut xgene_enet_pdata) -> bool;
}
extern "C" {
    pub fn xgene_enet_phy_connect(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn xgene_enet_phy_disconnect(pdata: *mut xgene_enet_pdata);
}
extern "C" {
    pub fn xgene_enet_rd_mac(pdata: *mut xgene_enet_pdata, rd_addr: u32) -> u32;
}
extern "C" {
    pub fn xgene_enet_rd_stat(pdata: *mut xgene_enet_pdata, rd_addr: u32) -> u32;
}
