//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/mac.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2019-2020  Realtek Corporation
//

pub const MAC_MEM_DUMP_PAGE_SIZE_AX: c_uint = 0x40000;
pub const MAC_MEM_DUMP_PAGE_SIZE_BE: c_uint = 0x80000;
pub const ADDR_CAM_ENT_SIZE: c_uint = 0x40;
pub const ADDR_CAM_ENT_SHORT_SIZE: c_uint = 0x20;
pub const BSSID_CAM_ENT_SIZE: c_uint = 0x08;
pub const HFC_PAGE_UNIT: c_int = 64;
pub const RPWM_TRY_CNT: c_int = 3;
pub const CPU_IO_RX_RETRY_CNT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_hwmod_sel {
    RTW89_DMAC_SEL = 0,
    RTW89_CMAC_SEL = 1,

    RTW89_MAC_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_fwd_target {
    RTW89_FWD_DONT_CARE    = 0,
    RTW89_FWD_TO_HOST      = 1,
    RTW89_FWD_TO_WLAN_CPU  = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_wd_dma_intvl {
    RTW89_MAC_WD_DMA_INTVL_0S,
    RTW89_MAC_WD_DMA_INTVL_256NS,
    RTW89_MAC_WD_DMA_INTVL_512NS,
    RTW89_MAC_WD_DMA_INTVL_768NS,
    RTW89_MAC_WD_DMA_INTVL_1US,
    RTW89_MAC_WD_DMA_INTVL_1_5US,
    RTW89_MAC_WD_DMA_INTVL_2US,
    RTW89_MAC_WD_DMA_INTVL_4US,
    RTW89_MAC_WD_DMA_INTVL_8US,
    RTW89_MAC_WD_DMA_INTVL_16US,
    RTW89_MAC_WD_DMA_INTVL_DEF = 0xFE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_multi_tag_num {
    RTW89_MAC_TAG_NUM_1,
    RTW89_MAC_TAG_NUM_2,
    RTW89_MAC_TAG_NUM_3,
    RTW89_MAC_TAG_NUM_4,
    RTW89_MAC_TAG_NUM_5,
    RTW89_MAC_TAG_NUM_6,
    RTW89_MAC_TAG_NUM_7,
    RTW89_MAC_TAG_NUM_8,
    RTW89_MAC_TAG_NUM_DEF = 0xFE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_lbc_tmr {
    RTW89_MAC_LBC_TMR_8US = 0,
    RTW89_MAC_LBC_TMR_16US,
    RTW89_MAC_LBC_TMR_32US,
    RTW89_MAC_LBC_TMR_64US,
    RTW89_MAC_LBC_TMR_128US,
    RTW89_MAC_LBC_TMR_256US,
    RTW89_MAC_LBC_TMR_512US,
    RTW89_MAC_LBC_TMR_1MS,
    RTW89_MAC_LBC_TMR_2MS,
    RTW89_MAC_LBC_TMR_4MS,
    RTW89_MAC_LBC_TMR_8MS,
    RTW89_MAC_LBC_TMR_DEF = 0xFE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_cpuio_op_cmd_type {
    CPUIO_OP_CMD_GET_1ST_PID = 0,
    CPUIO_OP_CMD_GET_NEXT_PID = 1,
    CPUIO_OP_CMD_ENQ_TO_TAIL = 4,
    CPUIO_OP_CMD_ENQ_TO_HEAD = 5,
    CPUIO_OP_CMD_DEQ = 8,
    CPUIO_OP_CMD_DEQ_ENQ_ALL = 9,
    CPUIO_OP_CMD_DEQ_ENQ_TO_TAIL = 12
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_wde_dle_port_id {
    WDE_DLE_PORT_ID_DISPATCH = 0,
    WDE_DLE_PORT_ID_PKTIN = 1,
    WDE_DLE_PORT_ID_CMAC0 = 3,
    WDE_DLE_PORT_ID_CMAC1 = 4,
    WDE_DLE_PORT_ID_CPU_IO = 6,
    WDE_DLE_PORT_ID_WDRLS = 7,
    WDE_DLE_PORT_ID_END = 8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_wde_dle_queid_wdrls {
    WDE_DLE_QUEID_TXOK = 0,
    WDE_DLE_QUEID_DROP_RETRY_LIMIT = 1,
    WDE_DLE_QUEID_DROP_LIFETIME_TO = 2,
    WDE_DLE_QUEID_DROP_MACID_DROP = 3,
    WDE_DLE_QUEID_NO_REPORT = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_ple_dle_port_id {
    PLE_DLE_PORT_ID_DISPATCH = 0,
    PLE_DLE_PORT_ID_MPDU = 1,
    PLE_DLE_PORT_ID_SEC = 2,
    PLE_DLE_PORT_ID_CMAC0 = 3,
    PLE_DLE_PORT_ID_CMAC1 = 4,
    PLE_DLE_PORT_ID_WDRLS = 5,
    PLE_DLE_PORT_ID_CPU_IO = 6,
    PLE_DLE_PORT_ID_PLRLS = 7,
    PLE_DLE_PORT_ID_END = 8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_ple_dle_queid_plrls {
    PLE_DLE_QUEID_NO_REPORT = 0x0
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_machdr_frame_type {
    RTW89_MGNT = 0,
    RTW89_CTRL = 1,
    RTW89_DATA = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_dle_dfi_type {
    DLE_DFI_TYPE_FREEPG	= 0,
    DLE_DFI_TYPE_QUOTA	= 1,
    DLE_DFI_TYPE_PAGELLT	= 2,
    DLE_DFI_TYPE_PKTINFO	= 3,
    DLE_DFI_TYPE_PREPKTLLT	= 4,
    DLE_DFI_TYPE_NXTPKTLLT	= 5,
    DLE_DFI_TYPE_QLNKTBL	= 6,
    DLE_DFI_TYPE_QEMPTY	= 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_dle_wde_quota_id {
    WDE_QTAID_HOST_IF = 0,
    WDE_QTAID_WLAN_CPU = 1,
    WDE_QTAID_DATA_CPU = 2,
    WDE_QTAID_PKTIN = 3,
    WDE_QTAID_CPUIO = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_dle_ple_quota_id {
    PLE_QTAID_B0_TXPL = 0,
    PLE_QTAID_B1_TXPL = 1,
    PLE_QTAID_C2H = 2,
    PLE_QTAID_H2C = 3,
    PLE_QTAID_WLAN_CPU = 4,
    PLE_QTAID_MPDU = 5,
    PLE_QTAID_CMAC0_RX = 6,
    PLE_QTAID_CMAC1_RX = 7,
    PLE_QTAID_CMAC1_BBRPT = 8,
    PLE_QTAID_WDRLS = 9,
    PLE_QTAID_CPUIO = 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_dle_ctrl_type {
    DLE_CTRL_TYPE_WDE = 0,
    DLE_CTRL_TYPE_PLE = 1,
    DLE_CTRL_TYPE_NUM = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_ax_l0_to_l1_event {
    MAC_AX_L0_TO_L1_CHIF_IDLE = 0,
    MAC_AX_L0_TO_L1_CMAC_DMA_IDLE = 1,
    MAC_AX_L0_TO_L1_RLS_PKID = 2,
    MAC_AX_L0_TO_L1_PTCL_IDLE = 3,
    MAC_AX_L0_TO_L1_RX_QTA_LOST = 4,
    MAC_AX_L0_TO_L1_DLE_STAT_HANG = 5,
    MAC_AX_L0_TO_L1_PCIE_STUCK = 6,
    MAC_AX_L0_TO_L1_EVENT_MAX = 15,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_phy_rpt_size {
    MAC_AX_PHY_RPT_SIZE_0 = 0,
    MAC_AX_PHY_RPT_SIZE_8 = 1,
    MAC_AX_PHY_RPT_SIZE_16 = 2,
    MAC_AX_PHY_RPT_SIZE_24 = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_hdr_cnv_size {
    MAC_AX_HDR_CNV_SIZE_0 = 0,
    MAC_AX_HDR_CNV_SIZE_32 = 1,
    MAC_AX_HDR_CNV_SIZE_64 = 2,
    MAC_AX_HDR_CNV_SIZE_96 = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_wow_fw_status {
    WOWLAN_NOT_READY = 0x00,
    WOWLAN_SLEEP_READY = 0x01,
    WOWLAN_RESUME_READY = 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_dbg_port_sel {
// CMAC 0 related
    RTW89_DBG_PORT_SEL_PTCL_C0 = 0,
    RTW89_DBG_PORT_SEL_SCH_C0,
    RTW89_DBG_PORT_SEL_TMAC_C0,
    RTW89_DBG_PORT_SEL_RMAC_C0,
    RTW89_DBG_PORT_SEL_RMACST_C0,
    RTW89_DBG_PORT_SEL_RMAC_PLCP_C0,
    RTW89_DBG_PORT_SEL_TRXPTCL_C0,
    RTW89_DBG_PORT_SEL_TX_INFOL_C0,
    RTW89_DBG_PORT_SEL_TX_INFOH_C0,
    RTW89_DBG_PORT_SEL_TXTF_INFOL_C0,
    RTW89_DBG_PORT_SEL_TXTF_INFOH_C0,
// CMAC 1 related
    RTW89_DBG_PORT_SEL_PTCL_C1,
    RTW89_DBG_PORT_SEL_SCH_C1,
    RTW89_DBG_PORT_SEL_TMAC_C1,
    RTW89_DBG_PORT_SEL_RMAC_C1,
    RTW89_DBG_PORT_SEL_RMACST_C1,
    RTW89_DBG_PORT_SEL_RMAC_PLCP_C1,
    RTW89_DBG_PORT_SEL_TRXPTCL_C1,
    RTW89_DBG_PORT_SEL_TX_INFOL_C1,
    RTW89_DBG_PORT_SEL_TX_INFOH_C1,
    RTW89_DBG_PORT_SEL_TXTF_INFOL_C1,
    RTW89_DBG_PORT_SEL_TXTF_INFOH_C1,
// DLE related
    RTW89_DBG_PORT_SEL_WDE_BUFMGN_FREEPG,
    RTW89_DBG_PORT_SEL_WDE_BUFMGN_QUOTA,
    RTW89_DBG_PORT_SEL_WDE_BUFMGN_PAGELLT,
    RTW89_DBG_PORT_SEL_WDE_BUFMGN_PKTINFO,
    RTW89_DBG_PORT_SEL_WDE_QUEMGN_PREPKT,
    RTW89_DBG_PORT_SEL_WDE_QUEMGN_NXTPKT,
    RTW89_DBG_PORT_SEL_WDE_QUEMGN_QLNKTBL,
    RTW89_DBG_PORT_SEL_WDE_QUEMGN_QEMPTY,
    RTW89_DBG_PORT_SEL_PLE_BUFMGN_FREEPG,
    RTW89_DBG_PORT_SEL_PLE_BUFMGN_QUOTA,
    RTW89_DBG_PORT_SEL_PLE_BUFMGN_PAGELLT,
    RTW89_DBG_PORT_SEL_PLE_BUFMGN_PKTINFO,
    RTW89_DBG_PORT_SEL_PLE_QUEMGN_PREPKT,
    RTW89_DBG_PORT_SEL_PLE_QUEMGN_NXTPKT,
    RTW89_DBG_PORT_SEL_PLE_QUEMGN_QLNKTBL,
    RTW89_DBG_PORT_SEL_PLE_QUEMGN_QEMPTY,
    RTW89_DBG_PORT_SEL_PKTINFO,
// DISPATCHER related
    RTW89_DBG_PORT_SEL_DSPT_HDT_TX0,
    RTW89_DBG_PORT_SEL_DSPT_HDT_TX1,
    RTW89_DBG_PORT_SEL_DSPT_HDT_TX2,
    RTW89_DBG_PORT_SEL_DSPT_HDT_TX3,
    RTW89_DBG_PORT_SEL_DSPT_HDT_TX4,
    RTW89_DBG_PORT_SEL_DSPT_HDT_TX5,
    RTW89_DBG_PORT_SEL_DSPT_HDT_TX6,
    RTW89_DBG_PORT_SEL_DSPT_HDT_TX7,
    RTW89_DBG_PORT_SEL_DSPT_HDT_TX8,
    RTW89_DBG_PORT_SEL_DSPT_HDT_TX9,
    RTW89_DBG_PORT_SEL_DSPT_HDT_TXA,
    RTW89_DBG_PORT_SEL_DSPT_HDT_TXB,
    RTW89_DBG_PORT_SEL_DSPT_HDT_TXC,
    RTW89_DBG_PORT_SEL_DSPT_HDT_TXD,
    RTW89_DBG_PORT_SEL_DSPT_HDT_TXE,
    RTW89_DBG_PORT_SEL_DSPT_HDT_TXF,
    RTW89_DBG_PORT_SEL_DSPT_CDT_TX0,
    RTW89_DBG_PORT_SEL_DSPT_CDT_TX1,
    RTW89_DBG_PORT_SEL_DSPT_CDT_TX3,
    RTW89_DBG_PORT_SEL_DSPT_CDT_TX4,
    RTW89_DBG_PORT_SEL_DSPT_CDT_TX5,
    RTW89_DBG_PORT_SEL_DSPT_CDT_TX6,
    RTW89_DBG_PORT_SEL_DSPT_CDT_TX7,
    RTW89_DBG_PORT_SEL_DSPT_CDT_TX8,
    RTW89_DBG_PORT_SEL_DSPT_CDT_TX9,
    RTW89_DBG_PORT_SEL_DSPT_CDT_TXA,
    RTW89_DBG_PORT_SEL_DSPT_CDT_TXB,
    RTW89_DBG_PORT_SEL_DSPT_CDT_TXC,
    RTW89_DBG_PORT_SEL_DSPT_HDT_RX0,
    RTW89_DBG_PORT_SEL_DSPT_HDT_RX1,
    RTW89_DBG_PORT_SEL_DSPT_HDT_RX2,
    RTW89_DBG_PORT_SEL_DSPT_HDT_RX3,
    RTW89_DBG_PORT_SEL_DSPT_HDT_RX4,
    RTW89_DBG_PORT_SEL_DSPT_HDT_RX5,
    RTW89_DBG_PORT_SEL_DSPT_CDT_RX_P0,
    RTW89_DBG_PORT_SEL_DSPT_CDT_RX_P0_0,
    RTW89_DBG_PORT_SEL_DSPT_CDT_RX_P0_1,
    RTW89_DBG_PORT_SEL_DSPT_CDT_RX_P0_2,
    RTW89_DBG_PORT_SEL_DSPT_CDT_RX_P1,
    RTW89_DBG_PORT_SEL_DSPT_STF_CTRL,
    RTW89_DBG_PORT_SEL_DSPT_ADDR_CTRL,
    RTW89_DBG_PORT_SEL_DSPT_WDE_INTF,
    RTW89_DBG_PORT_SEL_DSPT_PLE_INTF,
    RTW89_DBG_PORT_SEL_DSPT_FLOW_CTRL,
// PCIE related
    RTW89_DBG_PORT_SEL_PCIE_TXDMA,
    RTW89_DBG_PORT_SEL_PCIE_RXDMA,
    RTW89_DBG_PORT_SEL_PCIE_CVT,
    RTW89_DBG_PORT_SEL_PCIE_CXPL,
    RTW89_DBG_PORT_SEL_PCIE_IO,
    RTW89_DBG_PORT_SEL_PCIE_MISC,
    RTW89_DBG_PORT_SEL_PCIE_MISC2,

// keep last
    RTW89_DBG_PORT_SEL_LAST,
    RTW89_DBG_PORT_SEL_MAX = RTW89_DBG_PORT_SEL_LAST,
    RTW89_DBG_PORT_SEL_INVALID = RTW89_DBG_PORT_SEL_LAST,
}

// SRAM mem dump
pub const R_AX_INDIR_ACCESS_ENTRY: c_uint = 0x40000;
pub const R_BE_INDIR_ACCESS_ENTRY: c_uint = 0x80000;
pub const AXIDMA_BASE_ADDR: c_uint = 0x18006000;
pub const STA_SCHED_BASE_ADDR: c_uint = 0x18808000;
pub const RXPLD_FLTR_CAM_BASE_ADDR: c_uint = 0x18813000;
pub const SECURITY_CAM_BASE_ADDR: c_uint = 0x18814000;
pub const WOW_CAM_BASE_ADDR: c_uint = 0x18815000;
pub const CMAC_TBL_BASE_ADDR: c_uint = 0x18840000;
pub const ADDR_CAM_BASE_ADDR: c_uint = 0x18850000;
pub const BSSID_CAM_BASE_ADDR: c_uint = 0x18853000;
pub const BA_CAM_BASE_ADDR: c_uint = 0x18854000;
pub const BCN_IE_CAM0_BASE_ADDR: c_uint = 0x18855000;
pub const SHARED_BUF_BASE_ADDR: c_uint = 0x18700000;
pub const DMAC_TBL_BASE_ADDR: c_uint = 0x18800000;
pub const SHCUT_MACHDR_BASE_ADDR: c_uint = 0x18800800;
pub const BCN_IE_CAM1_BASE_ADDR: c_uint = 0x188A0000;
pub const TXD_FIFO_0_BASE_ADDR: c_uint = 0x18856200;
pub const TXD_FIFO_1_BASE_ADDR: c_uint = 0x188A1080;
pub const TXD_FIFO_0_BASE_ADDR_V1: c_uint = 0x18856400 /* for 8852C */;
pub const TXD_FIFO_1_BASE_ADDR_V1: c_uint = 0x188A1080 /* for 8852C */;
pub const TXDATA_FIFO_0_BASE_ADDR: c_uint = 0x18856000;
pub const TXDATA_FIFO_1_BASE_ADDR: c_uint = 0x188A1000;
pub const CPU_LOCAL_BASE_ADDR: c_uint = 0x18003000;
pub const WD_PAGE_BASE_ADDR_BE: c_uint = 0x0;
pub const CPU_LOCAL_BASE_ADDR_BE: c_uint = 0x18003000;
pub const AXIDMA_BASE_ADDR_BE: c_uint = 0x18006000;
pub const SHARED_BUF_BASE_ADDR_BE: c_uint = 0x18700000;
pub const DMAC_TBL_BASE_ADDR_BE: c_uint = 0x18800000;
pub const SHCUT_MACHDR_BASE_ADDR_BE: c_uint = 0x18800800;
pub const STA_SCHED_BASE_ADDR_BE: c_uint = 0x18818000;
pub const NAT25_CAM_BASE_ADDR_BE: c_uint = 0x18820000;
pub const RXPLD_FLTR_CAM_BASE_ADDR_BE: c_uint = 0x18823000;
pub const SEC_CAM_BASE_ADDR_BE: c_uint = 0x18824000;
pub const SEC_CAM_BASE_ADDR_BE_8922D: c_uint = 0x1882C000;
pub const WOW_CAM_BASE_ADDR_BE: c_uint = 0x18828000;
pub const MLD_TBL_BASE_ADDR_BE: c_uint = 0x18829000;
pub const RX_CLSF_CAM_BASE_ADDR_BE: c_uint = 0x1882A000;
pub const CMAC_TBL_BASE_ADDR_BE: c_uint = 0x18840000;
pub const ADDR_CAM_BASE_ADDR_BE: c_uint = 0x18850000;
pub const BSSID_CAM_BASE_ADDR_BE: c_uint = 0x18858000;
pub const BA_CAM_BASE_ADDR_BE: c_uint = 0x18859000;
pub const BCN_IE_CAM0_BASE_ADDR_BE: c_uint = 0x18860000;
pub const TXDATA_FIFO_0_BASE_ADDR_BE: c_uint = 0x18861000;
pub const TXD_FIFO_0_BASE_ADDR_BE: c_uint = 0x18862000;
pub const BCN_IE_CAM1_BASE_ADDR_BE: c_uint = 0x18880000;
pub const TXDATA_FIFO_1_BASE_ADDR_BE: c_uint = 0x18881000;
pub const TXD_FIFO_1_BASE_ADDR_BE: c_uint = 0x18881800;
pub const DCPU_LOCAL_BASE_ADDR_BE: c_uint = 0x19C02000;
pub const CCTL_INFO_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_mem_sel {
    RTW89_MAC_MEM_AXIDMA,
    RTW89_MAC_MEM_SHARED_BUF,
    RTW89_MAC_MEM_DMAC_TBL,
    RTW89_MAC_MEM_SHCUT_MACHDR,
    RTW89_MAC_MEM_STA_SCHED,
    RTW89_MAC_MEM_RXPLD_FLTR_CAM,
    RTW89_MAC_MEM_SECURITY_CAM,
    RTW89_MAC_MEM_WOW_CAM,
    RTW89_MAC_MEM_CMAC_TBL,
    RTW89_MAC_MEM_ADDR_CAM,
    RTW89_MAC_MEM_BA_CAM,
    RTW89_MAC_MEM_BCN_IE_CAM0,
    RTW89_MAC_MEM_BCN_IE_CAM1,
    RTW89_MAC_MEM_TXD_FIFO_0,
    RTW89_MAC_MEM_TXD_FIFO_1,
    RTW89_MAC_MEM_TXDATA_FIFO_0,
    RTW89_MAC_MEM_TXDATA_FIFO_1,
    RTW89_MAC_MEM_CPU_LOCAL,
    RTW89_MAC_MEM_BSSID_CAM,
    RTW89_MAC_MEM_TXD_FIFO_0_V1,
    RTW89_MAC_MEM_TXD_FIFO_1_V1,
    RTW89_MAC_MEM_WD_PAGE,
    RTW89_MAC_MEM_MLD_TBL,

// keep last
    RTW89_MAC_MEM_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rpwm_req_pwr_state {
    RTW89_MAC_RPWM_REQ_PWR_STATE_ACTIVE = 0,
    RTW89_MAC_RPWM_REQ_PWR_STATE_BAND0_RFON = 1,
    RTW89_MAC_RPWM_REQ_PWR_STATE_BAND1_RFON = 2,
    RTW89_MAC_RPWM_REQ_PWR_STATE_BAND0_RFOFF = 3,
    RTW89_MAC_RPWM_REQ_PWR_STATE_BAND1_RFOFF = 4,
    RTW89_MAC_RPWM_REQ_PWR_STATE_CLK_GATED = 5,
    RTW89_MAC_RPWM_REQ_PWR_STATE_PWR_GATED = 6,
    RTW89_MAC_RPWM_REQ_PWR_STATE_HIOE_PWR_GATED = 7,
    RTW89_MAC_RPWM_REQ_PWR_STATE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pwr_cfg {
    pub addr: u16,
    pub cv_msk: u8,
    pub intf_msk: u8,
    pub base:4: u8,
    pub cmd:4: u8,
    pub msk: u8,
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_c2h_ofld_func {
    RTW89_MAC_C2H_FUNC_EFUSE_DUMP,
    RTW89_MAC_C2H_FUNC_READ_RSP,
    RTW89_MAC_C2H_FUNC_PKT_OFLD_RSP,
    RTW89_MAC_C2H_FUNC_BCN_RESEND,
    RTW89_MAC_C2H_FUNC_MACID_PAUSE,
    RTW89_MAC_C2H_FUNC_TSF32_TOGL_RPT = 0x6,
    RTW89_MAC_C2H_FUNC_SCANOFLD_RSP = 0x9,
    RTW89_MAC_C2H_FUNC_TX_DUTY_RPT = 0xa,
    RTW89_MAC_C2H_FUNC_BCNFLTR_RPT = 0xd,
    RTW89_MAC_C2H_FUNC_OFLD_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_c2h_info_func {
    RTW89_MAC_C2H_FUNC_REC_ACK,
    RTW89_MAC_C2H_FUNC_DONE_ACK,
    RTW89_MAC_C2H_FUNC_C2H_LOG,
    RTW89_MAC_C2H_FUNC_BCN_CNT,
    RTW89_MAC_C2H_FUNC_BCN_UPD_DONE = 0x06,
    RTW89_MAC_C2H_FUNC_INFO_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_c2h_mcc_func {
    RTW89_MAC_C2H_FUNC_MCC_RCV_ACK = 0,
    RTW89_MAC_C2H_FUNC_MCC_REQ_ACK = 1,
    RTW89_MAC_C2H_FUNC_MCC_TSF_RPT = 2,
    RTW89_MAC_C2H_FUNC_MCC_STATUS_RPT = 3,

    NUM_OF_RTW89_MAC_C2H_FUNC_MCC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_c2h_misc_func {
    RTW89_MAC_C2H_FUNC_TX_REPORT = 1,

    NUM_OF_RTW89_MAC_C2H_FUNC_MISC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_c2h_mlo_func {
    RTW89_MAC_C2H_FUNC_MLO_GET_TBL			= 0x0,
    RTW89_MAC_C2H_FUNC_MLO_EMLSR_TRANS_DONE		= 0x1,
    RTW89_MAC_C2H_FUNC_MLO_EMLSR_STA_CFG_DONE	= 0x2,
    RTW89_MAC_C2H_FUNC_MCMLO_RELINK_RPT		= 0x3,
    RTW89_MAC_C2H_FUNC_MCMLO_SN_SYNC_RPT		= 0x4,
    RTW89_MAC_C2H_FUNC_MLO_LINK_CFG_STAT		= 0x5,
    RTW89_MAC_C2H_FUNC_MLO_DM_DBG_DUMP		= 0x6,

    NUM_OF_RTW89_MAC_C2H_FUNC_MLO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_c2h_mrc_func {
    RTW89_MAC_C2H_FUNC_MRC_TSF_RPT = 0,
    RTW89_MAC_C2H_FUNC_MRC_STATUS_RPT = 1,

    NUM_OF_RTW89_MAC_C2H_FUNC_MRC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_c2h_wow_func {
    RTW89_MAC_C2H_FUNC_AOAC_REPORT,

    NUM_OF_RTW89_MAC_C2H_FUNC_WOW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_c2h_ap_func {
    RTW89_MAC_C2H_FUNC_PWR_INT_NOTIFY = 0,

    NUM_OF_RTW89_MAC_C2H_FUNC_AP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_c2h_class {
    RTW89_MAC_C2H_CLASS_INFO = 0x0,
    RTW89_MAC_C2H_CLASS_OFLD = 0x1,
    RTW89_MAC_C2H_CLASS_TWT = 0x2,
    RTW89_MAC_C2H_CLASS_WOW = 0x3,
    RTW89_MAC_C2H_CLASS_MCC = 0x4,
    RTW89_MAC_C2H_CLASS_FWDBG = 0x5,
    RTW89_MAC_C2H_CLASS_MISC = 0x9,
    RTW89_MAC_C2H_CLASS_MLO = 0xc,
    RTW89_MAC_C2H_CLASS_MRC = 0xe,
    RTW89_MAC_C2H_CLASS_AP = 0x18,
    RTW89_MAC_C2H_CLASS_ROLE = 0x1b,
    RTW89_MAC_C2H_CLASS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_mcc_status {
    RTW89_MAC_MCC_ADD_ROLE_OK = 0,
    RTW89_MAC_MCC_START_GROUP_OK = 1,
    RTW89_MAC_MCC_STOP_GROUP_OK = 2,
    RTW89_MAC_MCC_DEL_GROUP_OK = 3,
    RTW89_MAC_MCC_RESET_GROUP_OK = 4,
    RTW89_MAC_MCC_SWITCH_CH_OK = 5,
    RTW89_MAC_MCC_TXNULL0_OK = 6,
    RTW89_MAC_MCC_TXNULL1_OK = 7,

    RTW89_MAC_MCC_SWITCH_EARLY = 10,
    RTW89_MAC_MCC_TBTT = 11,
    RTW89_MAC_MCC_DURATION_START = 12,
    RTW89_MAC_MCC_DURATION_END = 13,

    RTW89_MAC_MCC_ADD_ROLE_FAIL = 20,
    RTW89_MAC_MCC_START_GROUP_FAIL = 21,
    RTW89_MAC_MCC_STOP_GROUP_FAIL = 22,
    RTW89_MAC_MCC_DEL_GROUP_FAIL = 23,
    RTW89_MAC_MCC_RESET_GROUP_FAIL = 24,
    RTW89_MAC_MCC_SWITCH_CH_FAIL = 25,
    RTW89_MAC_MCC_TXNULL0_FAIL = 26,
    RTW89_MAC_MCC_TXNULL1_FAIL = 27,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_mrc_status {
    RTW89_MAC_MRC_START_SCH_OK = 0,
    RTW89_MAC_MRC_STOP_SCH_OK = 1,
    RTW89_MAC_MRC_DEL_SCH_OK = 2,
    RTW89_MAC_MRC_EMPTY_SCH_FAIL = 16,
    RTW89_MAC_MRC_ROLE_NOT_EXIST_FAIL = 17,
    RTW89_MAC_MRC_DATA_NOT_FOUND_FAIL = 18,
    RTW89_MAC_MRC_GET_NEXT_SLOT_FAIL = 19,
    RTW89_MAC_MRC_ALT_ROLE_FAIL = 20,
    RTW89_MAC_MRC_ADD_PSTIMER_FAIL = 21,
    RTW89_MAC_MRC_MALLOC_FAIL = 22,
    RTW89_MAC_MRC_SWITCH_CH_FAIL = 23,
    RTW89_MAC_MRC_TXNULL0_FAIL = 24,
    RTW89_MAC_MRC_PORT_FUNC_EN_FAIL = 25,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_ax_coex {
pub const RTW89_MAC_AX_COEX_RTK_MODE: c_int = 0;
pub const RTW89_MAC_AX_COEX_CSR_MODE: c_int = 1;
    pub pta_mode: u8,
pub const RTW89_MAC_AX_COEX_INNER: c_int = 0;
pub const RTW89_MAC_AX_COEX_OUTPUT: c_int = 1;
pub const RTW89_MAC_AX_COEX_INPUT: c_int = 2;
    pub direction: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_ax_plt {

    pub band: u8,
    pub tx: u8,
    pub rx: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_bf_rrsc_rate {
    RTW89_MAC_BF_RRSC_6M = 0,
    RTW89_MAC_BF_RRSC_9M = 1,
    RTW89_MAC_BF_RRSC_12M,
    RTW89_MAC_BF_RRSC_18M,
    RTW89_MAC_BF_RRSC_24M,
    RTW89_MAC_BF_RRSC_36M,
    RTW89_MAC_BF_RRSC_48M,
    RTW89_MAC_BF_RRSC_54M,
    RTW89_MAC_BF_RRSC_HT_MSC0,
    RTW89_MAC_BF_RRSC_HT_MSC1,
    RTW89_MAC_BF_RRSC_HT_MSC2,
    RTW89_MAC_BF_RRSC_HT_MSC3,
    RTW89_MAC_BF_RRSC_HT_MSC4,
    RTW89_MAC_BF_RRSC_HT_MSC5,
    RTW89_MAC_BF_RRSC_HT_MSC6,
    RTW89_MAC_BF_RRSC_HT_MSC7,
    RTW89_MAC_BF_RRSC_VHT_MSC0,
    RTW89_MAC_BF_RRSC_VHT_MSC1,
    RTW89_MAC_BF_RRSC_VHT_MSC2,
    RTW89_MAC_BF_RRSC_VHT_MSC3,
    RTW89_MAC_BF_RRSC_VHT_MSC4,
    RTW89_MAC_BF_RRSC_VHT_MSC5,
    RTW89_MAC_BF_RRSC_VHT_MSC6,
    RTW89_MAC_BF_RRSC_VHT_MSC7,
    RTW89_MAC_BF_RRSC_HE_MSC0,
    RTW89_MAC_BF_RRSC_HE_MSC1,
    RTW89_MAC_BF_RRSC_HE_MSC2,
    RTW89_MAC_BF_RRSC_HE_MSC3,
    RTW89_MAC_BF_RRSC_HE_MSC4,
    RTW89_MAC_BF_RRSC_HE_MSC5,
    RTW89_MAC_BF_RRSC_HE_MSC6,
    RTW89_MAC_BF_RRSC_HE_MSC7 = 31,
    RTW89_MAC_BF_RRSC_MAX = 32
}

pub const MAC_REG_POOL_COUNT: c_int = 10;

pub const RTW89_MAC_AX_BAND_REG_OFFSET: c_uint = 0x2000;
pub const RTW89_MAC_BE_BAND_REG_OFFSET: c_uint = 0x4000;
pub const PTCL_IDLE_POLL_CNT: c_int = 10000;
pub const SW_CVR_DUR_US: c_int = 8;
pub const SW_CVR_CNT: c_int = 8;

pub const DLE_WAIT_CNT: c_int = 2000;
pub const TRXCFG_WAIT_CNT: c_int = 2000;
pub const RTW89_WDE_PG_64: c_int = 64;
pub const RTW89_WDE_PG_128: c_int = 128;
pub const RTW89_WDE_PG_256: c_int = 256;
pub const S_AX_WDE_PAGE_SEL_64: c_int = 0;
pub const S_AX_WDE_PAGE_SEL_128: c_int = 1;
pub const S_AX_WDE_PAGE_SEL_256: c_int = 2;
pub const RTW89_PLE_PG_64: c_int = 64;
pub const RTW89_PLE_PG_128: c_int = 128;
pub const RTW89_PLE_PG_256: c_int = 256;
pub const S_AX_PLE_PAGE_SEL_64: c_int = 0;
pub const S_AX_PLE_PAGE_SEL_128: c_int = 1;
pub const S_AX_PLE_PAGE_SEL_256: c_int = 2;

pub const QEMP_ACQ_GRP_MACID_NUM: c_int = 8;
pub const QEMP_ACQ_GRP_QSEL_SH: c_int = 4;
pub const QEMP_ACQ_GRP_QSEL_MASK: c_uint = 0xF;
pub const SDIO_LOCAL_BASE_ADDR: c_uint = 0x80000000;
pub const PWR_CMD_WRITE: c_int = 0;
pub const PWR_CMD_POLL: c_int = 1;
pub const PWR_CMD_DELAY: c_int = 2;
pub const PWR_CMD_END: c_int = 3;

pub const PWR_INTF_MSK_ALL: c_uint = 0x7;
pub const PWR_BASE_MAC: c_int = 0;
pub const PWR_BASE_USB: c_int = 1;
pub const PWR_BASE_PCIE: c_int = 2;
pub const PWR_BASE_SDIO: c_int = 3;

pub const PWR_CV_MSK_ALL: c_uint = 0xFF;
pub const PWR_DELAY_US: c_int = 0;
pub const PWR_DELAY_MS: c_int = 1;
// STA scheduler
pub const SS_MACID_SH: c_int = 8;
pub const SS_TX_LEN_MSK: c_uint = 0x1FFFFF;
pub const SS_CTRL1_R_TX_LEN: c_int = 5;
pub const SS_CTRL1_R_NEXT_LINK: c_int = 20;
pub const SS_LINK_SIZE: c_int = 256;
// MAC debug port
pub const TMAC_DBG_SEL_C0: c_uint = 0xA5;
pub const RMAC_DBG_SEL_C0: c_uint = 0xA6;
pub const TRXPTCL_DBG_SEL_C0: c_uint = 0xA7;
pub const TMAC_DBG_SEL_C1: c_uint = 0xB5;
pub const RMAC_DBG_SEL_C1: c_uint = 0xB6;
pub const TRXPTCL_DBG_SEL_C1: c_uint = 0xB7;
pub const FW_PROG_CNTR_DBG_SEL: c_uint = 0xF2;
pub const PCIE_TXDMA_DBG_SEL: c_uint = 0x30;
pub const PCIE_RXDMA_DBG_SEL: c_uint = 0x31;
pub const PCIE_CVT_DBG_SEL: c_uint = 0x32;
pub const PCIE_CXPL_DBG_SEL: c_uint = 0x33;
pub const PCIE_IO_DBG_SEL: c_uint = 0x37;
pub const PCIE_MISC_DBG_SEL: c_uint = 0x38;
pub const PCIE_MISC2_DBG_SEL: c_uint = 0x00;
pub const MAC_DBG_SEL: c_int = 1;
pub const RMAC_CMAC_DBG_SEL: c_int = 1;
// TRXPTCL dbg port sel
pub const TRXPTRL_DBG_SEL_TMAC: c_int = 0;
pub const TRXPTRL_DBG_SEL_RMAC: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_cpuio_ctrl {
    pub pkt_num: u16,
    pub start_pktid: u16,
    pub end_pktid: u16,
    pub cmd_type: u8,
    pub macid: u8,
    pub src_pid: u8,
    pub src_qid: u8,
    pub dst_pid: u8,
    pub dst_qid: u8,
    pub pktid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_dbg_port_info {
    pub sel_addr: u32,
    pub sel_byte: u8,
    pub sel_msk: u32,
    pub srt: u32,
    pub end: u32,
    pub rd_addr: u32,
    pub rd_byte: u8,
    pub rd_msk: u32,
}

pub const QLNKTBL_ADDR_INFO_SEL_0: c_int = 0;
pub const QLNKTBL_ADDR_INFO_SEL_1: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_dle_dfi_ctrl {
    pub type: rtw89_mac_dle_ctrl_type,
    pub target: u32,
    pub addr: u32,
    pub out_data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_dle_dfi_quota {
    pub dle_type: rtw89_mac_dle_ctrl_type,
    pub qtaid: u32,
    pub rsv_pgnum: u16,
    pub use_pgnum: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_dle_dfi_qempty {
    pub dle_type: rtw89_mac_dle_ctrl_type,
    pub grpsel: u32,
    pub qempty: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_dle_rsvd_qt_type {
    DLE_RSVD_QT_MPDU_INFO,
    DLE_RSVD_QT_B0_CSI,
    DLE_RSVD_QT_B1_CSI,
    DLE_RSVD_QT_B0_LMR,
    DLE_RSVD_QT_B1_LMR,
    DLE_RSVD_QT_B0_FTM,
    DLE_RSVD_QT_B1_FTM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_dle_rsvd_qt_cfg {
    pub pktid: u16,
    pub pg_num: u16,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_error_scenario {
    RTW89_RXI300_ERROR		= 1,
    RTW89_WCPU_CPU_EXCEPTION	= 2,
    RTW89_WCPU_ASSERTION		= 3,
}

// Define DBG and recovery enum
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_ax_err_info {
// Get error info

// L0
    MAC_AX_ERR_L0_ERR_CMAC0 = 0x0001,
    MAC_AX_ERR_L0_ERR_CMAC1 = 0x0002,
    MAC_AX_ERR_L0_RESET_DONE = 0x0003,
    MAC_AX_ERR_L0_PROMOTE_TO_L1 = 0x0010,

// L1
    MAC_AX_ERR_L1_PREERR_DMAC = 0x999,
    MAC_AX_ERR_L1_ERR_DMAC = 0x1000,
    MAC_AX_ERR_L1_RESET_DISABLE_DMAC_DONE = 0x1001,
    MAC_AX_ERR_L1_RESET_RECOVERY_DONE = 0x1002,
    MAC_AX_ERR_L1_PROMOTE_TO_L2 = 0x1010,
    MAC_AX_ERR_L1_RCVY_STOP_DONE = 0x1011,

// L2
// address hole (master)
    MAC_AX_ERR_L2_ERR_AH_DMA = 0x2000,
    MAC_AX_ERR_L2_ERR_AH_HCI = 0x2010,
    MAC_AX_ERR_L2_ERR_AH_RLX4081 = 0x2020,
    MAC_AX_ERR_L2_ERR_AH_IDDMA = 0x2030,
    MAC_AX_ERR_L2_ERR_AH_HIOE = 0x2040,
    MAC_AX_ERR_L2_ERR_AH_IPSEC = 0x2050,
    MAC_AX_ERR_L2_ERR_AH_RX4281 = 0x2060,
    MAC_AX_ERR_L2_ERR_AH_OTHERS = 0x2070,

// AHB bridge timeout (master)
    MAC_AX_ERR_L2_ERR_AHB_TO_DMA = 0x2100,
    MAC_AX_ERR_L2_ERR_AHB_TO_HCI = 0x2110,
    MAC_AX_ERR_L2_ERR_AHB_TO_RLX4081 = 0x2120,
    MAC_AX_ERR_L2_ERR_AHB_TO_IDDMA = 0x2130,
    MAC_AX_ERR_L2_ERR_AHB_TO_HIOE = 0x2140,
    MAC_AX_ERR_L2_ERR_AHB_TO_IPSEC = 0x2150,
    MAC_AX_ERR_L2_ERR_AHB_TO_RX4281 = 0x2160,
    MAC_AX_ERR_L2_ERR_AHB_TO_OTHERS = 0x2170,

// APB_SA bridge timeout (master + slave)
    MAC_AX_ERR_L2_ERR_APB_SA_TO_DMA_WVA = 0x2200,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_DMA_UART = 0x2201,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_DMA_CPULOCAL = 0x2202,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_DMA_AXIDMA = 0x2203,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_DMA_HIOE = 0x2204,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_DMA_IDDMA = 0x2205,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_DMA_IPSEC = 0x2206,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_DMA_WON = 0x2207,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_DMA_WDMAC = 0x2208,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_DMA_WCMAC = 0x2209,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_DMA_OTHERS = 0x220A,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HCI_WVA = 0x2210,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HCI_UART = 0x2211,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HCI_CPULOCAL = 0x2212,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HCI_AXIDMA = 0x2213,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HCI_HIOE = 0x2214,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HCI_IDDMA = 0x2215,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HCI_IPSEC = 0x2216,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HCI_WDMAC = 0x2218,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HCI_WCMAC = 0x2219,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HCI_OTHERS = 0x221A,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RLX4081_WVA = 0x2220,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RLX4081_UART = 0x2221,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RLX4081_CPULOCAL = 0x2222,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RLX4081_AXIDMA = 0x2223,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RLX4081_HIOE = 0x2224,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RLX4081_IDDMA = 0x2225,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RLX4081_IPSEC = 0x2226,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RLX4081_WON = 0x2227,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RLX4081_WDMAC = 0x2228,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RLX4081_WCMAC = 0x2229,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RLX4081_OTHERS = 0x222A,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IDDMA_WVA = 0x2230,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IDDMA_UART = 0x2231,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IDDMA_CPULOCAL = 0x2232,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IDDMA_AXIDMA = 0x2233,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IDDMA_HIOE = 0x2234,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IDDMA_IDDMA = 0x2235,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IDDMA_IPSEC = 0x2236,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IDDMA_WON = 0x2237,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IDDMA_WDMAC = 0x2238,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IDDMA_WCMAC = 0x2239,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IDDMA_OTHERS = 0x223A,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HIOE_WVA = 0x2240,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HIOE_UART = 0x2241,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HIOE_CPULOCAL = 0x2242,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HIOE_AXIDMA = 0x2243,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HIOE_HIOE = 0x2244,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HIOE_IDDMA = 0x2245,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HIOE_IPSEC = 0x2246,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HIOE_WON = 0x2247,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HIOE_WDMAC = 0x2248,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HIOE_WCMAC = 0x2249,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_HIOE_OTHERS = 0x224A,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IPSEC_WVA = 0x2250,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IPSEC_UART = 0x2251,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IPSEC_CPULOCAL = 0x2252,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IPSEC_AXIDMA = 0x2253,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IPSEC_HIOE = 0x2254,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IPSEC_IDDMA = 0x2255,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IPSEC_IPSEC = 0x2256,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IPSEC_WON = 0x2257,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IPSEC_WDMAC = 0x2258,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IPSEC_WCMAC = 0x2259,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_IPSEC_OTHERS = 0x225A,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RX4281_WVA = 0x2260,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RX4281_UART = 0x2261,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RX4281_CPULOCAL = 0x2262,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RX4281_AXIDMA = 0x2263,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RX4281_HIOE = 0x2264,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RX4281_IDDMA = 0x2265,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RX4281_IPSEC = 0x2266,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RX4281_WON = 0x2267,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RX4281_WDMAC = 0x2268,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RX4281_WCMAC = 0x2269,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_RX4281_OTHERS = 0x226A,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_OTHERS_WVA = 0x2270,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_OTHERS_UART = 0x2271,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_OTHERS_CPULOCAL = 0x2272,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_OTHERS_AXIDMA = 0x2273,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_OTHERS_HIOE = 0x2274,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_OTHERS_IDDMA = 0x2275,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_OTHERS_IPSEC = 0x2276,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_OTHERS_WON = 0x2277,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_OTHERS_WDMAC = 0x2278,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_OTHERS_WCMAC = 0x2279,
    MAC_AX_ERR_L2_ERR_APB_SA_TO_OTHERS_OTHERS = 0x227A,

// APB_BBRF bridge timeout (master)
    MAC_AX_ERR_L2_ERR_APB_BBRF_TO_DMA = 0x2300,
    MAC_AX_ERR_L2_ERR_APB_BBRF_TO_HCI = 0x2310,
    MAC_AX_ERR_L2_ERR_APB_BBRF_TO_RLX4081 = 0x2320,
    MAC_AX_ERR_L2_ERR_APB_BBRF_TO_IDDMA = 0x2330,
    MAC_AX_ERR_L2_ERR_APB_BBRF_TO_HIOE = 0x2340,
    MAC_AX_ERR_L2_ERR_APB_BBRF_TO_IPSEC = 0x2350,
    MAC_AX_ERR_L2_ERR_APB_BBRF_TO_RX4281 = 0x2360,
    MAC_AX_ERR_L2_ERR_APB_BBRF_TO_OTHERS = 0x2370,
    MAC_AX_ERR_L2_RESET_DONE = 0x2400,
    MAC_AX_ERR_L2_ERR_WDT_TIMEOUT_INT = 0x2599,
    MAC_AX_ERR_CPU_EXCEPTION = 0x3000,
    MAC_AX_ERR_ASSERTION = 0x4000,
    MAC_AX_ERR_RXI300 = 0x5000,
    MAC_AX_GET_ERR_MAX,
    MAC_AX_DUMP_SHAREBUFF_INDICATOR = 0x80000000,

// set error info
    MAC_AX_ERR_L1_DISABLE_EN = 0x0001,
    MAC_AX_ERR_L1_RCVY_EN = 0x0002,
    MAC_AX_ERR_L1_RCVY_STOP_REQ = 0x0003,
    MAC_AX_ERR_L1_RCVY_START_REQ = 0x0004,
    MAC_AX_ERR_L1_RESET_START_DMAC = 0x000A,
    MAC_AX_ERR_L0_CFG_NOTIFY = 0x0010,
    MAC_AX_ERR_L0_CFG_DIS_NOTIFY = 0x0011,
    MAC_AX_ERR_L0_CFG_HANDSHAKE = 0x0012,
    MAC_AX_ERR_L0_RCVY_EN = 0x0013,
    MAC_AX_ERR_L0_RESET_FORCE = 0x0020,
    MAC_AX_ERR_L0_RESET_FORCE_C1 = 0x0021,
    MAC_AX_ERR_L1_RESET_FORCE = 0x0022,
    MAC_AX_SET_ERR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_size_set {
    pub hfc_preccfg_pcie: rtw89_hfc_prec_cfg,
    pub hfc_prec_cfg_c0: rtw89_hfc_prec_cfg,
    pub hfc_prec_cfg_c2: rtw89_hfc_prec_cfg,
    pub hfc_prec_cfg_c3: rtw89_hfc_prec_cfg,
    pub hfc_prec_cfg_c5: rtw89_hfc_prec_cfg,
    pub hfc_prec_cfg_c6: rtw89_hfc_prec_cfg,
    pub wde_size0: rtw89_dle_size,
    pub wde_size1: rtw89_dle_size,
    pub wde_size0_v1: rtw89_dle_size,
    pub wde_size3_v1: rtw89_dle_size,
    pub wde_size4: rtw89_dle_size,
    pub wde_size4_v1: rtw89_dle_size,
    pub wde_size5_v1: rtw89_dle_size,
    pub wde_size6: rtw89_dle_size,
    pub wde_size7: rtw89_dle_size,
    pub wde_size7_v1: rtw89_dle_size,
    pub wde_size8_v1: rtw89_dle_size,
    pub wde_size9: rtw89_dle_size,
    pub wde_size16_v1: rtw89_dle_size,
    pub wde_size17: rtw89_dle_size,
    pub wde_size18: rtw89_dle_size,
    pub wde_size18_v1: rtw89_dle_size,
    pub wde_size19: rtw89_dle_size,
    pub wde_size22_v1: rtw89_dle_size,
    pub wde_size23: rtw89_dle_size,
    pub wde_size30: rtw89_dle_size,
    pub wde_size31: rtw89_dle_size,
    pub ple_size0: rtw89_dle_size,
    pub ple_size1: rtw89_dle_size,
    pub ple_size0_v1: rtw89_dle_size,
    pub ple_size3_v1: rtw89_dle_size,
    pub ple_size4_v1: rtw89_dle_size,
    pub ple_size4: rtw89_dle_size,
    pub ple_size6: rtw89_dle_size,
    pub ple_size6_v1: rtw89_dle_size,
    pub ple_size7_v1: rtw89_dle_size,
    pub ple_size8: rtw89_dle_size,
    pub ple_size9: rtw89_dle_size,
    pub ple_size17: rtw89_dle_size,
    pub ple_size18: rtw89_dle_size,
    pub ple_size19: rtw89_dle_size,
    pub ple_size20_v1: rtw89_dle_size,
    pub ple_size22_v1: rtw89_dle_size,
    pub ple_size27: rtw89_dle_size,
    pub ple_size29_v1: rtw89_dle_size,
    pub ple_size31: rtw89_dle_size,
    pub ple_size34: rtw89_dle_size,
    pub wde_qt0: rtw89_wde_quota,
    pub wde_qt1: rtw89_wde_quota,
    pub wde_qt0_v1: rtw89_wde_quota,
    pub wde_qt3: rtw89_wde_quota,
    pub wde_qt4: rtw89_wde_quota,
    pub wde_qt4_v1: rtw89_wde_quota,
    pub wde_qt5_v1: rtw89_wde_quota,
    pub wde_qt6: rtw89_wde_quota,
    pub wde_qt7: rtw89_wde_quota,
    pub wde_qt7_v1: rtw89_wde_quota,
    pub wde_qt8_v1: rtw89_wde_quota,
    pub wde_qt16: rtw89_wde_quota,
    pub wde_qt17: rtw89_wde_quota,
    pub wde_qt18: rtw89_wde_quota,
    pub wde_qt19_v1: rtw89_wde_quota,
    pub wde_qt23: rtw89_wde_quota,
    pub wde_qt23_v1: rtw89_wde_quota,
    pub wde_qt30: rtw89_wde_quota,
    pub wde_qt31: rtw89_wde_quota,
    pub ple_qt0: rtw89_ple_quota,
    pub ple_qt1: rtw89_ple_quota,
    pub ple_qt4: rtw89_ple_quota,
    pub ple_qt5: rtw89_ple_quota,
    pub ple_qt5_v2: rtw89_ple_quota,
    pub ple_qt6_v1: rtw89_ple_quota,
    pub ple_qt7_v1: rtw89_ple_quota,
    pub ple_qt8_v1: rtw89_ple_quota,
    pub ple_qt9: rtw89_ple_quota,
    pub ple_qt9_v1: rtw89_ple_quota,
    pub ple_qt12_v1: rtw89_ple_quota,
    pub ple_qt13: rtw89_ple_quota,
    pub ple_qt13_v1: rtw89_ple_quota,
    pub ple_qt14_v1: rtw89_ple_quota,
    pub ple_qt15_v1: rtw89_ple_quota,
    pub ple_qt18: rtw89_ple_quota,
    pub ple_qt25: rtw89_ple_quota,
    pub ple_qt26: rtw89_ple_quota,
    pub ple_qt27: rtw89_ple_quota,
    pub ple_qt28: rtw89_ple_quota,
    pub ple_qt42: rtw89_ple_quota,
    pub ple_qt43: rtw89_ple_quota,
    pub ple_qt44: rtw89_ple_quota,
    pub ple_qt44_v2: rtw89_ple_quota,
    pub ple_qt45: rtw89_ple_quota,
    pub ple_qt45_v2: rtw89_ple_quota,
    pub ple_qt46: rtw89_ple_quota,
    pub ple_qt47: rtw89_ple_quota,
    pub ple_qt47_v2: rtw89_ple_quota,
    pub ple_qt57: rtw89_ple_quota,
    pub ple_qt58: rtw89_ple_quota,
    pub ple_qt59: rtw89_ple_quota,
    pub ple_qt61: rtw89_ple_quota,
    pub ple_qt62: rtw89_ple_quota,
    pub ple_qt64_v2: rtw89_ple_quota,
    pub ple_qt65_v2: rtw89_ple_quota,
    pub ple_qt78: rtw89_ple_quota,
    pub ple_qt79: rtw89_ple_quota,
    pub ple_qt_52a_wow: rtw89_ple_quota,
    pub ple_qt_52b_wow: rtw89_ple_quota,
    pub ple_qt_52bt_wow: rtw89_ple_quota,
    pub ple_qt_51b_wow: rtw89_ple_quota,
    pub ple_rsvd_qt0: rtw89_rsvd_quota,
    pub ple_rsvd_qt1: rtw89_rsvd_quota,
    pub ple_rsvd_qt1_v1: rtw89_rsvd_quota,
    pub ple_rsvd_qt2: rtw89_rsvd_quota,
    pub ple_rsvd_qt9: rtw89_rsvd_quota,
    pub rsvd0_size0: rtw89_dle_rsvd_size,
    pub rsvd0_size3: rtw89_dle_rsvd_size,
    pub rsvd0_size5: rtw89_dle_rsvd_size,
    pub rsvd0_size6: rtw89_dle_rsvd_size,
    pub rsvd0_size17: rtw89_dle_rsvd_size,
    pub rsvd1_size0: rtw89_dle_rsvd_size,
    pub rsvd1_size2: rtw89_dle_rsvd_size,
    pub rsvd1_size3: rtw89_dle_rsvd_size,
    pub dle_input3: rtw89_dle_input,
    pub dle_input20: rtw89_dle_input,
    pub dle_input28: rtw89_dle_input,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_mu_gid_addr {
    pub position_en: [u32; 2],
    pub position: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_gen_def {
    pub band1_offset: u32,
    pub filter_model_addr: u32,
    pub indir_access_addr: u32,
    pub mem_base_addrs: *const u32,
    pub mem_page_size: u32,
    pub rx_fltr: u32,
    pub default_rx_fltr: u32,
    pub port_base: *const rtw89_port_reg,
    pub agg_len_ht: u32,
    pub ps_status: u32,
    pub mu_gid: *const rtw89_mac_mu_gid_addr,
    pub boot_dbg: u32,
    pub muedca_ctrl: rtw89_reg_def,
    pub bfee_ctrl: rtw89_reg_def,
    pub narrow_bw_ru_dis: rtw89_reg_def,
    pub wow_ctrl: rtw89_reg_def,
    pub agg_limit: rtw89_reg_def,
    pub ra_agg_limit: rtw89_reg_def,
    pub txcnt_limit: rtw89_reg_def,
    pub sel): rtw89_mac_hwmod_sel,
    pub rtwdev): *mut *mut int (sys_init)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (trx_init)(struct rtw89_dev,
    pub mode): rtw89_qta_mode,
    pub rtwdev): *mut *mut void (clr_aon_intr)(struct rtw89_dev,
    pub en): *mut *mut *mut void (err_imr_ctrl)(struct rtw89_dev rtwdev, bool,
    pub rtwdev): *mut *mut int (mac_func_en)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (hci_func_en)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (dmac_func_pre_en)(struct rtw89_dev,
    pub enable): *mut *mut *mut void (dle_func_en)(struct rtw89_dev rtwdev, bool,
    pub enable): *mut *mut *mut void (dle_clk_en)(struct rtw89_dev rtwdev, bool,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub mac_idx): u8,
    pub enable): *mut *mut *mut int (cfg_ppdu_status)(struct rtw89_dev rtwdev, u8 mac_idx, bool,
    pub enable): *mut *mut *mut void (cfg_phy_rpt)(struct rtw89_dev rtwdev, u8 mac_idx, bool,
    pub normal): *mut *mut *mut void (set_edcca_mode)(struct rtw89_dev rtwdev, u8 mac_idx, bool,
    pub vlv): *mut *mut *mut void (set_vcore_cfg)(struct rtw89_dev rtwdev, u8,
    pub cfg): *const *const *const int (dle_mix_cfg)(struct rtw89_dev rtwdev, struct rtw89_dle_mem,
    pub wde_or_ple): *mut *mut *mut int (chk_dle_rdy)(struct rtw89_dev rtwdev, bool,
    pub pkt_id): *mut *mut *mut int (dle_buf_req)(struct rtw89_dev rtwdev, u16 buf_len, bool wd, u16,
    pub h2c_en): *mut *mut *mut void (hfc_func_en)(struct rtw89_dev rtwdev, bool en, bool,
    pub rtwdev): *mut *mut void (hfc_h2c_cfg)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (hfc_mix_cfg)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (hfc_get_mix_info)(struct rtw89_dev,
    pub ext_wde_min_qt_wcpu): u16,
    pub max_cfg): *const rtw89_ple_quota,
    pub wd): *mut *mut rtw89_cpuio_ctrl ctrl_para, bool,
    pub band1_en): *mut *mut *mut int (dle_quota_change)(struct rtw89_dev rtwdev, bool,
    pub rtwdev): *mut *mut int (reset_pwr_state)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (disable_cpu)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (fwdl_preconfig)(struct rtw89_dev,
    pub include_bb): bool dlfw, bool,
    pub type): *mut *mut *mut u8 (fwdl_get_status)(struct rtw89_dev rtwdev, enum rtw89_fwdl_check_type,
    pub h2c_or_fwdl): *mut *mut *mut int (fwdl_check_path_ready)(struct rtw89_dev rtwdev, bool,
    pub mode): *mut *mut *mut void (fwdl_secure_idmem_share_mode)(struct rtw89_dev rtwdev, u8,
    pub rtwdev): *mut *mut int (parse_efuse_map)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (parse_phycap_map)(struct rtw89_dev,
    pub idle): *mut *mut *mut int (cnv_efuse_state)(struct rtw89_dev rtwdev, bool,
    pub rtwdev): *mut *mut int (efuse_read_fw_secure)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (efuse_read_ecv)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (efuse_read_thermal_k)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (efuse_read_pwr_data)(struct rtw89_dev,
    pub plt): *mut *mut *mut int (cfg_plt)(struct rtw89_dev rtwdev, struct rtw89_mac_ax_plt,
    pub band): *mut *mut *mut u16 (get_plt_cnt)(struct rtw89_dev rtwdev, u8,
    pub cr): *mut u32 reg_base, u32,
    pub mask): *mut *mut *mut int (write_xtal_si)(struct rtw89_dev rtwdev, u8 offset, u8 val, u8,
    pub val): *mut *mut *mut int (read_xtal_si)(struct rtw89_dev rtwdev, u8 offset, u8,
    pub rtwdev): *mut *mut void (dump_qta_lost)(struct rtw89_dev,
    pub err): mac_ax_err_info,
    pub rtwdev): *mut *mut bool (is_txq_empty)(struct rtw89_dev,
    pub rtwvif_link): *mut rtw89_vif_link,
    pub rtwdev): *mut *mut void (free_chan_list)(struct rtw89_dev,
    pub rtwvif_link): *mut rtw89_vif_link,
    pub rtwvif_link): *mut rtw89_vif_link,
    pub wowlan): bool,
    pub enable_wow): *mut *mut *mut int (wow_config_mac)(struct rtw89_dev rtwdev, bool,
}

extern "C" {
    pub fn rtw89_mac_reg_by_idx(_arg: rtwdev, 0x40: *mut *mut base + port, _arg: mac_idx) -> return;
}
extern "C" {
    pub fn rtw89_read32(_arg: rtwdev, _arg: reg) -> return;
}
extern "C" {
    pub fn rtw89_read32_mask(_arg: rtwdev, _arg: reg, _arg: mask) -> return;
}
extern "C" {
    pub fn rtw89_mac_pwr_on(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_pwr_off(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_mac_preinit(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_init(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_hfc_init(rtwdev: *mut rtw89_dev, reset: bool, en: bool, h2c_en: bool) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_is_qta_dbcc(rtwdev: *mut rtw89_dev, mode: rtw89_qta_mode) -> bool;
}
extern "C" {
    pub fn rtw89_mac_write_lte(rtwdev: *mut rtw89_dev, offset: u32, val: u32) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_read_lte(rtwdev: *mut rtw89_dev, offset: u32, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_dle_dfi_cfg(rtwdev: *mut rtw89_dev, ctrl: *mut rtw89_mac_dle_dfi_ctrl) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_dump_dmac_err_status(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_mac_add_vif(rtwdev: *mut rtw89_dev, vif: *mut rtw89_vif_link) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_port_update(rtwdev: *mut rtw89_dev, rtwvif_link: *mut rtw89_vif_link) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_stop_ap(rtwdev: *mut rtw89_dev, rtwvif_link: *mut rtw89_vif_link);
}
extern "C" {
    pub fn rtw89_mac_enable_beacon_for_ap_vifs(rtwdev: *mut rtw89_dev, en: bool);
}
extern "C" {
    pub fn rtw89_mac_remove_vif(rtwdev: *mut rtw89_dev, vif: *mut rtw89_vif_link) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_enable_bb_rf(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_disable_bb_rf(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_get_err_status(rtwdev: *mut rtw89_dev) -> u32;
}
extern "C" {
    pub fn rtw89_mac_set_err_status(rtwdev: *mut rtw89_dev, err: u32) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_setup_phycap(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_resume_sch_tx(rtwdev: *mut rtw89_dev, mac_idx: u8, tx_en: u32) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_resume_sch_tx_v1(rtwdev: *mut rtw89_dev, mac_idx: u8, tx_en: u32) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_resume_sch_tx_v2(rtwdev: *mut rtw89_dev, mac_idx: u8, tx_en: u32) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_cfg_phy_rpt_be(rtwdev: *mut rtw89_dev, mac_idx: u8, enable: bool);
}
extern "C" {
    pub fn rtw89_mac_cfg_ppdu_status(_arg: rtwdev, _arg: RTW89_MAC_1, _arg: enable) -> return;
}
extern "C" {
    pub fn rtw89_mac_set_rx_fltr(rtwdev: *mut rtw89_dev, mac_idx: u8, rx_fltr: u32);
}
extern "C" {
    pub fn rtw89_mac_update_rts_threshold(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_mac_flush_txq(rtwdev: *mut rtw89_dev, queues: u32, drop: bool);
}
extern "C" {
    pub fn rtw89_mac_coex_init(rtwdev: *mut rtw89_dev, coex: *const rtw89_mac_ax_coex) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_cfg_sb(rtwdev: *mut rtw89_dev, val: u32);
}
extern "C" {
    pub fn rtw89_mac_get_sb(rtwdev: *mut rtw89_dev) -> u32;
}
extern "C" {
    pub fn rtw89_mac_get_ctrl_path(rtwdev: *mut rtw89_dev) -> bool;
}
extern "C" {
    pub fn rtw89_mac_cfg_ctrl_path(rtwdev: *mut rtw89_dev, wl: bool) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_cfg_ctrl_path_v1(rtwdev: *mut rtw89_dev, wl: bool) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_cfg_ctrl_path_v2(rtwdev: *mut rtw89_dev, wl: bool) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_power_mode_change(rtwdev: *mut rtw89_dev, enter: bool);
}
extern "C" {
    pub fn rtw89_mac_notify_wake(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn _rtw89_mac_bf_monitor_track(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_mac_bfee_ctrl(rtwdev: *mut rtw89_dev, mac_idx: u8, en: bool);
}
extern "C" {
    pub fn rtw89_mac_vif_init(rtwdev: *mut rtw89_dev, rtwvif_link: *mut rtw89_vif_link) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_vif_deinit(rtwdev: *mut rtw89_dev, rtwvif_link: *mut rtw89_vif_link) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_set_macid_pause(rtwdev: *mut rtw89_dev, macid: u8, pause: bool) -> c_int;
}
// val = rtw89_read32(rtwdev, cr);
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_xtal_si_offset {
    XTAL0 = 0x0,
    XTAL3 = 0x3,
    XTAL_SI_XTAL_SC_XI = 0x04,

    XTAL_SI_XTAL_SC_XO = 0x05,

    XTAL_SI_XREF_MODE = 0x0B,
    XTAL_SI_PWR_CUT = 0x10,

    XTAL_SI_AONLDO_CTRL = 0x10,
    XTAL_SI_XTAL_DRV = 0x15,

    XTAL_SI_XTAL_PLL = 0x16,
    XTAL_SI_XTAL_XMD_2 = 0x24,

    XTAL_SI_XTAL_XMD_4 = 0x26,

    XTAL_SI_XREF_RF1 = 0x2D,
    XTAL_SI_XREF_RF2 = 0x2E,
    XTAL_SI_CV = 0x41,

    XTAL_SI_LOW_ADDR = 0x62,

    XTAL_SI_CTRL = 0x63,

    XTAL_SI_READ_VAL = 0x7A,
    XTAL_SI_WL_RFC_S0 = 0x80,

    XTAL_SI_WL_RFC_S1 = 0x81,

    XTAL_SI_ANAPAR_WL = 0x90,

    XTAL_SI_SRAM_CTRL = 0xA1,

    XTAL_SI_APBT = 0xD1,
    XTAL_SI_PLL = 0xE0,
    XTAL_SI_PLL_1 = 0xE1,
    XTAL_SI_CHIP_ID_L = 0xFD,
    XTAL_SI_CHIP_ID_H = 0xFE,
}

extern "C" {
    pub fn rtw89_mac_pkt_drop_vif(rtwdev: *mut rtw89_dev, rtwvif: *mut rtw89_vif);
}
extern "C" {
    pub fn rtw89_mac_resize_ple_rx_quota(rtwdev: *mut rtw89_dev, wow: bool) -> c_int;
}
extern "C" {
    pub fn rtw89_mac_hw_mgnt_sec(rtwdev: *mut rtw89_dev, wow: bool);
}
extern "C" {
    pub fn rtw89_mac_cpu_io_rx(rtwdev: *mut rtw89_dev, wow_enable: bool) -> c_int;
}
//
// At this point, new scan request is acknowledged by firmware,
// so scan events of previous scan request become obsoleted.
// Purge the queued scan events to prevent interference to
// current new request.
//
// firmware maintains a 4-bit sequence number
// if skb having the similar seq number is still in the queue,
// this means the queue is overflowed - it isn't normal and
// should indicate firmware doesn't provide TX reports in time;
// report the old skb as dropped, we can't do much more here
//
// The RTL8922DE will re-enable pre-load function after verification.
