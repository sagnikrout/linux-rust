//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/mscc/mscc_ptp.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Driver for Microsemi VSC85xx PHYs
//
// Copyright (c) 2020 Microsemi Corporation
//
// 1588 page Registers
pub const MSCC_PHY_TS_BIU_ADDR_CNTL: c_int = 16;
pub const BIU_ADDR_EXE: c_uint = 0x8000;
pub const BIU_ADDR_READ: c_uint = 0x4000;
pub const BIU_ADDR_WRITE: c_uint = 0x0000;

pub const BIU_ADDR_CNT_MAX: c_int = 8;
pub const MSCC_PHY_TS_CSR_DATA_LSB: c_int = 17;
pub const MSCC_PHY_TS_CSR_DATA_MSB: c_int = 18;
pub const MSCC_PHY_1588_INGR_VSC85XX_INT_STATUS: c_uint = 0x002d;
pub const MSCC_PHY_1588_VSC85XX_INT_STATUS: c_uint = 0x004d;
pub const VSC85XX_1588_INT_FIFO_ADD: c_uint = 0x0004;
pub const VSC85XX_1588_INT_FIFO_OVERFLOW: c_uint = 0x0001;
pub const MSCC_PHY_1588_INGR_VSC85XX_INT_MASK: c_uint = 0x002e;
pub const MSCC_PHY_1588_VSC85XX_INT_MASK: c_uint = 0x004e;

// TS CSR addresses
pub const MSCC_PHY_ANA_ETH1_NTX_PROT: c_uint = 0x0000;

pub const ANA_ETH1_NTX_PROT_PTP_OAM: c_uint = 0x0005;
pub const ANA_ETH1_NTX_PROT_MPLS: c_uint = 0x0004;
pub const ANA_ETH1_NTX_PROT_IP_UDP_ACH_2: c_uint = 0x0003;
pub const ANA_ETH1_NTX_PROT_IP_UDP_ACH_1: c_uint = 0x0002;
pub const ANA_ETH1_NTX_PROT_ETH2: c_uint = 0x0001;
pub const MSCC_PHY_PTP_IFACE_CTRL: c_uint = 0x0000;
pub const PTP_IFACE_CTRL_CLK_ENA: c_uint = 0x0040;
pub const PTP_IFACE_CTRL_INGR_BYPASS: c_uint = 0x0008;
pub const PTP_IFACE_CTRL_EGR_BYPASS: c_uint = 0x0004;
pub const PTP_IFACE_CTRL_MII_PROT: c_uint = 0x0003;
pub const PTP_IFACE_CTRL_GMII_PROT: c_uint = 0x0002;
pub const PTP_IFACE_CTRL_XGMII_64_PROT: c_uint = 0x0000;
pub const MSCC_PHY_ANA_ETH1_NTX_PROT_VLAN_TPID: c_uint = 0x0001;

pub const MSCC_PHY_PTP_ANALYZER_MODE: c_uint = 0x0001;
pub const PTP_ANA_SPLIT_ENCAP_FLOW: c_uint = 0x1000000;

pub const MSCC_PHY_ANA_ETH1_NXT_PROT_TAG: c_uint = 0x0002;
pub const ANA_ETH1_NXT_PROT_TAG_ENA: c_uint = 0x0001;
pub const MSCC_PHY_PTP_MODE_CTRL: c_uint = 0x0002;

pub const PTP_MODE_CTRL_PKT_MODE: c_uint = 0x0004;
pub const MSCC_PHY_ANA_ETH1_NXT_PROT_ETYPE_MATCH: c_uint = 0x0003;
pub const ANA_ETH1_NXT_PROT_ETYPE_MATCH_ENA: c_uint = 0x10000;

pub const MSCC_PHY_PTP_VERSION_CODE: c_uint = 0x0003;

pub const PTP_IP_VERSION_2_1: c_uint = 0x0021;

pub const ETH1_FLOW_ENA: c_uint = 0x0001;

pub const ANA_ETH1_FLOW_MATCH_VLAN_TAG2: c_uint = 0x0200;
pub const ANA_ETH1_FLOW_MATCH_VLAN_VERIFY: c_uint = 0x0010;

pub const ANA_ETH1_FLOW_ADDR_MATCH2_ANY_MULTICAST: c_uint = 0x400000;
pub const ANA_ETH1_FLOW_ADDR_MATCH2_ANY_UNICAST: c_uint = 0x200000;
pub const ANA_ETH1_FLOW_ADDR_MATCH2_FULL_ADDR: c_uint = 0x100000;

pub const ANA_ETH1_FLOW_ADDR_MATCH2_SRC_DEST: c_uint = 0x020000;
pub const ANA_ETH1_FLOW_ADDR_MATCH2_SRC: c_uint = 0x010000;
pub const ANA_ETH1_FLOW_ADDR_MATCH2_DEST: c_uint = 0x000000;

pub const MSCC_PHY_PTP_LTC_CTRL: c_uint = 0x0010;

pub const PTP_LTC_CTRL_AUTO_ADJ_UPDATE: c_uint = 0x0010;
pub const PTP_LTC_CTRL_ADD_SUB_1NS_REQ: c_uint = 0x0008;
pub const PTP_LTC_CTRL_ADD_1NS: c_uint = 0x0004;
pub const PTP_LTC_CTRL_SAVE_ENA: c_uint = 0x0002;
pub const PTP_LTC_CTRL_LOAD_ENA: c_uint = 0x0001;
pub const MSCC_PHY_PTP_LTC_LOAD_SEC_MSB: c_uint = 0x0011;

pub const MSCC_PHY_PTP_LTC_LOAD_SEC_LSB: c_uint = 0x0012;

pub const MSCC_PHY_PTP_LTC_LOAD_NS: c_uint = 0x0013;

pub const MSCC_PHY_PTP_LTC_SAVED_SEC_MSB: c_uint = 0x0014;
pub const MSCC_PHY_PTP_LTC_SAVED_SEC_LSB: c_uint = 0x0015;
pub const MSCC_PHY_PTP_LTC_SAVED_NS: c_uint = 0x0016;
pub const MSCC_PHY_PTP_LTC_SEQUENCE: c_uint = 0x0017;

pub const MSCC_PHY_PTP_LTC_SEQ: c_uint = 0x0018;
pub const PTP_LTC_SEQ_ADD_SUB: c_uint = 0x80000;

pub const MSCC_PHY_PTP_LTC_AUTO_ADJ: c_uint = 0x001a;

pub const PTP_AUTO_ADJ_SUB_1NS: c_uint = 0x80000000;
pub const PTP_AUTO_ADJ_ADD_1NS: c_uint = 0x40000000;
pub const MSCC_PHY_PTP_LTC_1PPS_WIDTH_ADJ: c_uint = 0x001b;

pub const MSCC_PHY_PTP_TSTAMP_FIFO_SI: c_uint = 0x0020;
pub const PTP_TSTAMP_FIFO_SI_EN: c_uint = 0x0001;
pub const MSCC_PHY_PTP_INGR_PREDICTOR: c_uint = 0x0022;
pub const PTP_INGR_PREDICTOR_EN: c_uint = 0x0001;
pub const MSCC_PHY_PTP_EGR_PREDICTOR: c_uint = 0x0026;
pub const PTP_EGR_PREDICTOR_EN: c_uint = 0x0001;
pub const MSCC_PHY_PTP_INGR_TSP_CTRL: c_uint = 0x0035;
pub const PHY_PTP_INGR_TSP_CTRL_FRACT_NS: c_uint = 0x0004;
pub const PHY_PTP_INGR_TSP_CTRL_LOAD_DELAYS: c_uint = 0x0001;
pub const MSCC_PHY_PTP_INGR_LOCAL_LATENCY: c_uint = 0x0037;

pub const MSCC_PHY_PTP_INGR_DELAY_FIFO: c_uint = 0x003a;
pub const PTP_INGR_DELAY_FIFO_DEPTH_MACSEC: c_uint = 0x0013;
pub const PTP_INGR_DELAY_FIFO_DEPTH_DEFAULT: c_uint = 0x000f;

pub const PTP_INGR_TS_FIFO_EMPTY: c_uint = 0x80000000;
pub const MSCC_PHY_PTP_INGR_REWRITER_CTRL: c_uint = 0x0044;
pub const PTP_INGR_REWRITER_REDUCE_PREAMBLE: c_uint = 0x0010;
pub const PTP_INGR_REWRITER_FLAG_VAL: c_uint = 0x0008;

pub const MSCC_PHY_PTP_EGR_STALL_LATENCY: c_uint = 0x004f;
pub const MSCC_PHY_PTP_EGR_TSP_CTRL: c_uint = 0x0055;
pub const PHY_PTP_EGR_TSP_CTRL_FRACT_NS: c_uint = 0x0004;
pub const PHY_PTP_EGR_TSP_CTRL_LOAD_DELAYS: c_uint = 0x0001;
pub const MSCC_PHY_PTP_EGR_LOCAL_LATENCY: c_uint = 0x0057;

pub const MSCC_PHY_PTP_EGR_DELAY_FIFO: c_uint = 0x005a;
pub const PTP_EGR_DELAY_FIFO_DEPTH_MACSEC: c_uint = 0x0013;
pub const PTP_EGR_DELAY_FIFO_DEPTH_DEFAULT: c_uint = 0x000f;
pub const MSCC_PHY_PTP_EGR_TS_FIFO_CTRL: c_uint = 0x005b;
pub const PTP_EGR_TS_FIFO_RESET: c_uint = 0x10000;

pub const PTP_EGR_TS_FIFO_EMPTY: c_uint = 0x80000000;

pub const MSCC_PHY_PTP_EGR_REWRITER_CTRL: c_uint = 0x0064;
pub const PTP_EGR_REWRITER_REDUCE_PREAMBLE: c_uint = 0x0010;
pub const PTP_EGR_REWRITER_FLAG_VAL: c_uint = 0x0008;

pub const MSCC_PHY_PTP_SERIAL_TOD_IFACE: c_uint = 0x006e;
pub const PTP_SERIAL_TOD_IFACE_LS_AUTO_CLR: c_uint = 0x0004;
pub const MSCC_PHY_PTP_LTC_OFFSET: c_uint = 0x0070;

pub const MSCC_PHY_PTP_ACCUR_CFG_STATUS: c_uint = 0x0074;
pub const PTP_ACCUR_PPS_OUT_CALIB_ERR: c_uint = 0x20000;
pub const PTP_ACCUR_PPS_OUT_CALIB_DONE: c_uint = 0x10000;
pub const PTP_ACCUR_PPS_IN_CALIB_ERR: c_uint = 0x4000;
pub const PTP_ACCUR_PPS_IN_CALIB_DONE: c_uint = 0x2000;
pub const PTP_ACCUR_EGR_SOF_CALIB_ERR: c_uint = 0x1000;
pub const PTP_ACCUR_EGR_SOF_CALIB_DONE: c_uint = 0x0800;
pub const PTP_ACCUR_INGR_SOF_CALIB_ERR: c_uint = 0x0400;
pub const PTP_ACCUR_INGR_SOF_CALIB_DONE: c_uint = 0x0200;
pub const PTP_ACCUR_LOAD_SAVE_CALIB_ERR: c_uint = 0x0100;
pub const PTP_ACCUR_LOAD_SAVE_CALIB_DONE: c_uint = 0x0080;
pub const PTP_ACCUR_CALIB_TRIGG: c_uint = 0x0040;
pub const PTP_ACCUR_PPS_OUT_BYPASS: c_uint = 0x0010;
pub const PTP_ACCUR_PPS_IN_BYPASS: c_uint = 0x0008;
pub const PTP_ACCUR_EGR_SOF_BYPASS: c_uint = 0x0004;
pub const PTP_ACCUR_INGR_SOF_BYPASS: c_uint = 0x0002;
pub const PTP_ACCUR_LOAD_SAVE_BYPASS: c_uint = 0x0001;
pub const MSCC_PHY_ANA_ETH2_NTX_PROT: c_uint = 0x0090;

pub const ANA_ETH2_NTX_PROT_PTP_OAM: c_uint = 0x0005;
pub const ANA_ETH2_NTX_PROT_MPLS: c_uint = 0x0004;
pub const ANA_ETH2_NTX_PROT_IP_UDP_ACH_2: c_uint = 0x0003;
pub const ANA_ETH2_NTX_PROT_IP_UDP_ACH_1: c_uint = 0x0002;
pub const ANA_ETH2_NTX_PROT_ETH2: c_uint = 0x0001;
pub const MSCC_PHY_ANA_ETH2_NXT_PROT_ETYPE_MATCH: c_uint = 0x0003;
pub const ANA_ETH2_NXT_PROT_ETYPE_MATCH_ENA: c_uint = 0x10000;

pub const MSCC_PHY_ANA_MPLS_COMP_NXT_COMP: c_uint = 0x0120;

pub const ANA_MPLS_NTX_PROT_PTP_OAM: c_uint = 0x0005;
pub const ANA_MPLS_NTX_PROT_MPLS: c_uint = 0x0004;
pub const ANA_MPLS_NTX_PROT_IP_UDP_ACH_2: c_uint = 0x0003;
pub const ANA_MPLS_NTX_PROT_IP_UDP_ACH_1: c_uint = 0x0002;
pub const ANA_MPLS_NTX_PROT_ETH2: c_uint = 0x0001;

pub const MSCC_ANA_IP1_NXT_PROT_NXT_COMP: c_uint = 0x01b0;

pub const ANA_IP1_NXT_PROT_NXT_COMP_PTP_OAM: c_uint = 0x0005;
pub const ANA_IP1_NXT_PROT_NXT_COMP_IP_UDP_ACH2: c_uint = 0x0003;
pub const MSCC_ANA_IP1_NXT_PROT_IP1_MODE: c_uint = 0x01b1;
pub const ANA_IP1_NXT_PROT_FLOW_OFFSET_IPV4: c_uint = 0x0c00;
pub const ANA_IP1_NXT_PROT_FLOW_OFFSET_IPV6: c_uint = 0x0800;
pub const ANA_IP1_NXT_PROT_IPV6: c_uint = 0x0001;
pub const ANA_IP1_NXT_PROT_IPV4: c_uint = 0x0000;
pub const MSCC_ANA_IP1_NXT_PROT_IP_MATCH1: c_uint = 0x01b2;

pub const MSCC_ANA_IP1_NXT_PROT_MATCH2_UPPER: c_uint = 0x01b3;
pub const MSCC_ANA_IP1_NXT_PROT_MATCH2_LOWER: c_uint = 0x01b4;
pub const MSCC_ANA_IP1_NXT_PROT_MASK2_UPPER: c_uint = 0x01b5;
pub const MSCC_ANA_IP1_NXT_PROT_MASK2_LOWER: c_uint = 0x01b6;
pub const MSCC_ANA_IP1_NXT_PROT_OFFSET2: c_uint = 0x01b7;

pub const MSCC_ANA_IP1_NXT_PROT_UDP_CHKSUM: c_uint = 0x01b8;

pub const IP1_NXT_PROT_UDP_CHKSUM_UPDATE: c_uint = 0x0002;
pub const IP1_NXT_PROT_UDP_CHKSUM_CLEAR: c_uint = 0x0001;

pub const IP1_FLOW_MATCH_DEST_SRC_ADDR: c_uint = 0x0200;
pub const IP1_FLOW_MATCH_DEST_ADDR: c_uint = 0x0100;
pub const IP1_FLOW_MATCH_SRC_ADDR: c_uint = 0x0000;

pub const IP1_FLOW_ENA: c_uint = 0x0001;

pub const MSCC_ANA_IP2_NXT_PROT_NXT_COMP: c_uint = 0x0240;

pub const ANA_IP2_NXT_PROT_NXT_COMP_PTP_OAM: c_uint = 0x0005;
pub const ANA_IP2_NXT_PROT_NXT_COMP_IP_UDP_ACH2: c_uint = 0x0003;
pub const MSCC_ANA_IP2_NXT_PROT_UDP_CHKSUM: c_uint = 0x0248;

pub const PTP_FLOW_ENA: c_uint = 0x0001;

pub const PTP_FLOW_MSG_TYPE_MASK: c_uint = 0x0F000000;
pub const PTP_FLOW_MSG_PDELAY_RESP: c_uint = 0x04000000;
pub const PTP_FLOW_MSG_PDELAY_REQ: c_uint = 0x02000000;
pub const PTP_FLOW_MSG_DELAY_REQ: c_uint = 0x01000000;
pub const PTP_FLOW_MSG_SYNC: c_uint = 0x00000000;

pub const PTP_FLOW_DOMAIN_RANGE_ENA: c_uint = 0x0001;

pub const PTP_FLOW_PTP_ACTION_MOD_FRAME_STATUS_UPDATE: c_uint = 0x10000000;

pub const PTP_FLOW_PTP_ACTION_SUB_DELAY_ASYM: c_uint = 0x00200000;
pub const PTP_FLOW_PTP_ACTION_ADD_DELAY_ASYM: c_uint = 0x00100000;

pub const PTP_FLOW_PTP_ACTION_SAVE_LOCAL_TIME: c_uint = 0x00000010;

pub const PTP_FLOW_PTP_0_FIELD_PTP_FRAME: c_uint = 0x8000;
pub const PTP_FLOW_PTP_0_FIELD_RSVRD_CHECK: c_uint = 0x4000;

pub const MSCC_ANA_PTP_IP_CHKSUM_SEL: c_uint = 0x0330;
pub const ANA_PTP_IP_CHKSUM_SEL_IP_COMP_2: c_uint = 0x0001;
pub const ANA_PTP_IP_CHKSUM_SEL_IP_COMP_1: c_uint = 0x0000;
pub const MSCC_PHY_ANA_FSB_CFG: c_uint = 0x331;

pub const ANA_FSB_ADDR_FROM_IP2: c_uint = 0x0003;
pub const ANA_FSB_ADDR_FROM_IP1: c_uint = 0x0002;
pub const ANA_FSB_ADDR_FROM_ETH2: c_uint = 0x0001;
pub const ANA_FSB_ADDR_FROM_ETH1: c_uint = 0x0000;

pub const COMP_MAX_FLOWS: c_int = 8;
pub const PTP_COMP_MAX_FLOWS: c_int = 6;
pub const PPS_WIDTH_ADJ: c_uint = 0x1dcd6500;

// PHC clock available frequencies.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ptp_cmd {
    PTP_NOP = 0,
    PTP_WRITE_1588 = 5,
    PTP_WRITE_NS = 7,
    PTP_SAVE_IN_TS_FIFO = 11, /* invalid when writing in reg */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc85xx_ptphdr {
    pub /: *mut *mut u8 tsmt; / transportSpecific | messageType,
    pub /: *mut *mut u8 ver; / reserved0 | versionPTP,
    pub msglen: __be16,
    pub domain: u8,
    pub rsrvd1: u8,
    pub flags: __be16,
    pub correction: __be64,
    pub rsrvd2: __be32,
    pub clk_identity: __be64,
    pub src_port_id: __be16,
    pub seq_id: __be16,
    pub ctrl: u8,
    pub log_interval: u8,
    pub __attribute__((__packed__)): },
// Represents an entry in the timestamping FIFO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc85xx_ts_fifo {
    pub ns: u32,
    pub secs:48: u64,
    pub sig: [u8; 16],
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc85xx_ptp {
    pub phydev: *mut phy_device,
    pub ptp_clock: *mut ptp_clock,
    pub caps: ptp_clock_info,
    pub tx_queue: sk_buff_head,
    pub tx_type: hwtstamp_tx_types,
    pub rx_filter: hwtstamp_rx_filters,
    pub configured:1: u8,
}
