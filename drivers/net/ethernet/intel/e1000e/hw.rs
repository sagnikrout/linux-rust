//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/e1000e/hw.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.

pub const E1000_DEV_ID_82571EB_COPPER: c_uint = 0x105E;
pub const E1000_DEV_ID_82571EB_FIBER: c_uint = 0x105F;
pub const E1000_DEV_ID_82571EB_SERDES: c_uint = 0x1060;
pub const E1000_DEV_ID_82571EB_QUAD_COPPER: c_uint = 0x10A4;
pub const E1000_DEV_ID_82571PT_QUAD_COPPER: c_uint = 0x10D5;
pub const E1000_DEV_ID_82571EB_QUAD_FIBER: c_uint = 0x10A5;
pub const E1000_DEV_ID_82571EB_QUAD_COPPER_LP: c_uint = 0x10BC;
pub const E1000_DEV_ID_82571EB_SERDES_DUAL: c_uint = 0x10D9;
pub const E1000_DEV_ID_82571EB_SERDES_QUAD: c_uint = 0x10DA;
pub const E1000_DEV_ID_82572EI_COPPER: c_uint = 0x107D;
pub const E1000_DEV_ID_82572EI_FIBER: c_uint = 0x107E;
pub const E1000_DEV_ID_82572EI_SERDES: c_uint = 0x107F;
pub const E1000_DEV_ID_82572EI: c_uint = 0x10B9;
pub const E1000_DEV_ID_82573E: c_uint = 0x108B;
pub const E1000_DEV_ID_82573E_IAMT: c_uint = 0x108C;
pub const E1000_DEV_ID_82573L: c_uint = 0x109A;
pub const E1000_DEV_ID_82574L: c_uint = 0x10D3;
pub const E1000_DEV_ID_82574LA: c_uint = 0x10F6;
pub const E1000_DEV_ID_82583V: c_uint = 0x150C;
pub const E1000_DEV_ID_80003ES2LAN_COPPER_DPT: c_uint = 0x1096;
pub const E1000_DEV_ID_80003ES2LAN_SERDES_DPT: c_uint = 0x1098;
pub const E1000_DEV_ID_80003ES2LAN_COPPER_SPT: c_uint = 0x10BA;
pub const E1000_DEV_ID_80003ES2LAN_SERDES_SPT: c_uint = 0x10BB;
pub const E1000_DEV_ID_ICH8_82567V_3: c_uint = 0x1501;
pub const E1000_DEV_ID_ICH8_IGP_M_AMT: c_uint = 0x1049;
pub const E1000_DEV_ID_ICH8_IGP_AMT: c_uint = 0x104A;
pub const E1000_DEV_ID_ICH8_IGP_C: c_uint = 0x104B;
pub const E1000_DEV_ID_ICH8_IFE: c_uint = 0x104C;
pub const E1000_DEV_ID_ICH8_IFE_GT: c_uint = 0x10C4;
pub const E1000_DEV_ID_ICH8_IFE_G: c_uint = 0x10C5;
pub const E1000_DEV_ID_ICH8_IGP_M: c_uint = 0x104D;
pub const E1000_DEV_ID_ICH9_IGP_AMT: c_uint = 0x10BD;
pub const E1000_DEV_ID_ICH9_BM: c_uint = 0x10E5;
pub const E1000_DEV_ID_ICH9_IGP_M_AMT: c_uint = 0x10F5;
pub const E1000_DEV_ID_ICH9_IGP_M: c_uint = 0x10BF;
pub const E1000_DEV_ID_ICH9_IGP_M_V: c_uint = 0x10CB;
pub const E1000_DEV_ID_ICH9_IGP_C: c_uint = 0x294C;
pub const E1000_DEV_ID_ICH9_IFE: c_uint = 0x10C0;
pub const E1000_DEV_ID_ICH9_IFE_GT: c_uint = 0x10C3;
pub const E1000_DEV_ID_ICH9_IFE_G: c_uint = 0x10C2;
pub const E1000_DEV_ID_ICH10_R_BM_LM: c_uint = 0x10CC;
pub const E1000_DEV_ID_ICH10_R_BM_LF: c_uint = 0x10CD;
pub const E1000_DEV_ID_ICH10_R_BM_V: c_uint = 0x10CE;
pub const E1000_DEV_ID_ICH10_D_BM_LM: c_uint = 0x10DE;
pub const E1000_DEV_ID_ICH10_D_BM_LF: c_uint = 0x10DF;
pub const E1000_DEV_ID_ICH10_D_BM_V: c_uint = 0x1525;
pub const E1000_DEV_ID_PCH_M_HV_LM: c_uint = 0x10EA;
pub const E1000_DEV_ID_PCH_M_HV_LC: c_uint = 0x10EB;
pub const E1000_DEV_ID_PCH_D_HV_DM: c_uint = 0x10EF;
pub const E1000_DEV_ID_PCH_D_HV_DC: c_uint = 0x10F0;
pub const E1000_DEV_ID_PCH2_LV_LM: c_uint = 0x1502;
pub const E1000_DEV_ID_PCH2_LV_V: c_uint = 0x1503;
pub const E1000_DEV_ID_PCH_LPT_I217_LM: c_uint = 0x153A;
pub const E1000_DEV_ID_PCH_LPT_I217_V: c_uint = 0x153B;
pub const E1000_DEV_ID_PCH_LPTLP_I218_LM: c_uint = 0x155A;
pub const E1000_DEV_ID_PCH_LPTLP_I218_V: c_uint = 0x1559;
pub const E1000_DEV_ID_PCH_I218_LM2: c_uint = 0x15A0;
pub const E1000_DEV_ID_PCH_I218_V2: c_uint = 0x15A1;
pub const E1000_DEV_ID_PCH_I218_LM3: c_uint = 0x15A2	/* Wildcat Point PCH */;
pub const E1000_DEV_ID_PCH_I218_V3: c_uint = 0x15A3	/* Wildcat Point PCH */;
pub const E1000_DEV_ID_PCH_SPT_I219_LM: c_uint = 0x156F	/* SPT PCH */;
pub const E1000_DEV_ID_PCH_SPT_I219_V: c_uint = 0x1570	/* SPT PCH */;
pub const E1000_DEV_ID_PCH_SPT_I219_LM2: c_uint = 0x15B7	/* SPT-H PCH */;
pub const E1000_DEV_ID_PCH_SPT_I219_V2: c_uint = 0x15B8	/* SPT-H PCH */;
pub const E1000_DEV_ID_PCH_LBG_I219_LM3: c_uint = 0x15B9	/* LBG PCH */;
pub const E1000_DEV_ID_PCH_SPT_I219_LM4: c_uint = 0x15D7;
pub const E1000_DEV_ID_PCH_SPT_I219_V4: c_uint = 0x15D8;
pub const E1000_DEV_ID_PCH_SPT_I219_LM5: c_uint = 0x15E3;
pub const E1000_DEV_ID_PCH_SPT_I219_V5: c_uint = 0x15D6;
pub const E1000_DEV_ID_PCH_CNP_I219_LM6: c_uint = 0x15BD;
pub const E1000_DEV_ID_PCH_CNP_I219_V6: c_uint = 0x15BE;
pub const E1000_DEV_ID_PCH_CNP_I219_LM7: c_uint = 0x15BB;
pub const E1000_DEV_ID_PCH_CNP_I219_V7: c_uint = 0x15BC;
pub const E1000_DEV_ID_PCH_ICP_I219_LM8: c_uint = 0x15DF;
pub const E1000_DEV_ID_PCH_ICP_I219_V8: c_uint = 0x15E0;
pub const E1000_DEV_ID_PCH_ICP_I219_LM9: c_uint = 0x15E1;
pub const E1000_DEV_ID_PCH_ICP_I219_V9: c_uint = 0x15E2;
pub const E1000_DEV_ID_PCH_CMP_I219_LM10: c_uint = 0x0D4E;
pub const E1000_DEV_ID_PCH_CMP_I219_V10: c_uint = 0x0D4F;
pub const E1000_DEV_ID_PCH_CMP_I219_LM11: c_uint = 0x0D4C;
pub const E1000_DEV_ID_PCH_CMP_I219_V11: c_uint = 0x0D4D;
pub const E1000_DEV_ID_PCH_CMP_I219_LM12: c_uint = 0x0D53;
pub const E1000_DEV_ID_PCH_CMP_I219_V12: c_uint = 0x0D55;
pub const E1000_DEV_ID_PCH_TGP_I219_LM13: c_uint = 0x15FB;
pub const E1000_DEV_ID_PCH_TGP_I219_V13: c_uint = 0x15FC;
pub const E1000_DEV_ID_PCH_TGP_I219_LM14: c_uint = 0x15F9;
pub const E1000_DEV_ID_PCH_TGP_I219_V14: c_uint = 0x15FA;
pub const E1000_DEV_ID_PCH_TGP_I219_LM15: c_uint = 0x15F4;
pub const E1000_DEV_ID_PCH_TGP_I219_V15: c_uint = 0x15F5;
pub const E1000_DEV_ID_PCH_RPL_I219_LM23: c_uint = 0x0DC5;
pub const E1000_DEV_ID_PCH_RPL_I219_V23: c_uint = 0x0DC6;
pub const E1000_DEV_ID_PCH_ADP_I219_LM16: c_uint = 0x1A1E;
pub const E1000_DEV_ID_PCH_ADP_I219_V16: c_uint = 0x1A1F;
pub const E1000_DEV_ID_PCH_ADP_I219_LM17: c_uint = 0x1A1C;
pub const E1000_DEV_ID_PCH_ADP_I219_V17: c_uint = 0x1A1D;
pub const E1000_DEV_ID_PCH_RPL_I219_LM22: c_uint = 0x0DC7;
pub const E1000_DEV_ID_PCH_RPL_I219_V22: c_uint = 0x0DC8;
pub const E1000_DEV_ID_PCH_MTP_I219_LM18: c_uint = 0x550A;
pub const E1000_DEV_ID_PCH_MTP_I219_V18: c_uint = 0x550B;
pub const E1000_DEV_ID_PCH_ADP_I219_LM19: c_uint = 0x550C;
pub const E1000_DEV_ID_PCH_ADP_I219_V19: c_uint = 0x550D;
pub const E1000_DEV_ID_PCH_LNP_I219_LM20: c_uint = 0x550E;
pub const E1000_DEV_ID_PCH_LNP_I219_V20: c_uint = 0x550F;
pub const E1000_DEV_ID_PCH_LNP_I219_LM21: c_uint = 0x5510;
pub const E1000_DEV_ID_PCH_LNP_I219_V21: c_uint = 0x5511;
pub const E1000_DEV_ID_PCH_ARL_I219_LM24: c_uint = 0x57A0;
pub const E1000_DEV_ID_PCH_ARL_I219_V24: c_uint = 0x57A1;
pub const E1000_DEV_ID_PCH_PTP_I219_LM25: c_uint = 0x57B3;
pub const E1000_DEV_ID_PCH_PTP_I219_V25: c_uint = 0x57B4;
pub const E1000_DEV_ID_PCH_PTP_I219_LM27: c_uint = 0x57B7;
pub const E1000_DEV_ID_PCH_PTP_I219_V27: c_uint = 0x57B8;
pub const E1000_DEV_ID_PCH_NVL_I219_LM29: c_uint = 0x57B9;
pub const E1000_DEV_ID_PCH_NVL_I219_V29: c_uint = 0x57BA;
pub const E1000_REVISION_4: c_int = 4;
pub const E1000_FUNC_1: c_int = 1;
pub const E1000_ALT_MAC_ADDRESS_OFFSET_LAN0: c_int = 0;
pub const E1000_ALT_MAC_ADDRESS_OFFSET_LAN1: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_mac_type {
    e1000_82571,
    e1000_82572,
    e1000_82573,
    e1000_82574,
    e1000_82583,
    e1000_80003es2lan,
    e1000_ich8lan,
    e1000_ich9lan,
    e1000_ich10lan,
    e1000_pchlan,
    e1000_pch2lan,
    e1000_pch_lpt,
    e1000_pch_spt,
    e1000_pch_cnp,
    e1000_pch_tgp,
    e1000_pch_adp,
    e1000_pch_mtp,
    e1000_pch_lnp,
    e1000_pch_ptp,
    e1000_pch_nvp,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_media_type {
    e1000_media_type_unknown = 0,
    e1000_media_type_copper = 1,
    e1000_media_type_fiber = 2,
    e1000_media_type_internal_serdes = 3,
    e1000_num_media_types
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_nvm_type {
    e1000_nvm_unknown = 0,
    e1000_nvm_none,
    e1000_nvm_eeprom_spi,
    e1000_nvm_flash_hw,
    e1000_nvm_flash_sw
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_nvm_override {
    e1000_nvm_override_none = 0,
    e1000_nvm_override_spi_small,
    e1000_nvm_override_spi_large
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_phy_type {
    e1000_phy_unknown = 0,
    e1000_phy_none,
    e1000_phy_m88,
    e1000_phy_igp,
    e1000_phy_igp_2,
    e1000_phy_gg82563,
    e1000_phy_igp_3,
    e1000_phy_ife,
    e1000_phy_bm,
    e1000_phy_82578,
    e1000_phy_82577,
    e1000_phy_82579,
    e1000_phy_i217,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_bus_width {
    e1000_bus_width_unknown = 0,
    e1000_bus_width_pcie_x1,
    e1000_bus_width_pcie_x2,
    e1000_bus_width_pcie_x4 = 4,
    e1000_bus_width_pcie_x8 = 8,
    e1000_bus_width_32,
    e1000_bus_width_64,
    e1000_bus_width_reserved
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_1000t_rx_status {
    e1000_1000t_rx_status_not_ok = 0,
    e1000_1000t_rx_status_ok,
    e1000_1000t_rx_status_undefined = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_rev_polarity {
    e1000_rev_polarity_normal = 0,
    e1000_rev_polarity_reversed,
    e1000_rev_polarity_undefined = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_fc_mode {
    e1000_fc_none = 0,
    e1000_fc_rx_pause,
    e1000_fc_tx_pause,
    e1000_fc_full,
    e1000_fc_default = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_ms_type {
    e1000_ms_hw_default = 0,
    e1000_ms_force_master,
    e1000_ms_force_slave,
    e1000_ms_auto
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_smart_speed {
    e1000_smart_speed_default = 0,
    e1000_smart_speed_on,
    e1000_smart_speed_off
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_serdes_link_state {
    e1000_serdes_link_down = 0,
    e1000_serdes_link_autoneg_progress,
    e1000_serdes_link_autoneg_complete,
    e1000_serdes_link_forced_up
}

// Receive Descriptor - Extended
#[repr(C)]
#[derive(Copy, Clone)]
pub union e1000_rx_desc_extended {
    pub buffer_addr: __le64,
    pub reserved: __le64,
    pub read: },
    pub /: *mut *mut __le32 mrq; / Multiple Rx Queues,
    pub /: *mut *mut __le32 rss; / RSS Hash,
    pub /: *mut *mut __le16 ip_id; / IP id,
    pub /: *mut *mut __le16 csum; / Packet Checksum,
    pub csum_ip: },
    pub hi_dword: },
    pub lower: },
    pub /: *mut *mut __le32 status_error; / ext status/error,
    pub length: __le16,
    pub /: *mut *mut __le16 vlan; / VLAN tag,
    pub upper: },
    pub /: *mut *mut } wb; / writeback,
}

pub const MAX_PS_BUFFERS: c_int = 4;
// Number of packet split data buffers (not including the header buffer)

// Receive Descriptor - Packet Split
#[repr(C)]
#[derive(Copy, Clone)]
pub union e1000_rx_desc_packet_split {
// one buffer for protocol header(s), three data buffers
    pub buffer_addr: [__le64; MAX_PS_BUFFERS],
    pub read: },
    pub /: *mut *mut __le32 mrq; / Multiple Rx Queues,
    pub /: *mut *mut __le32 rss; / RSS Hash,
    pub /: *mut *mut __le16 ip_id; / IP id,
    pub /: *mut *mut __le16 csum; / Packet Checksum,
    pub csum_ip: },
    pub hi_dword: },
    pub lower: },
    pub /: *mut *mut __le32 status_error; / ext status/error,
    pub /: *mut *mut __le16 length0; / length of buffer 0,
    pub /: *mut *mut __le16 vlan; / VLAN tag,
    pub middle: },
    pub header_status: __le16,
// length of buffers 1-3
    pub length: [__le16; PS_PAGE_BUFFERS],
    pub upper: },
    pub reserved: __le64,
    pub /: *mut *mut } wb; / writeback,
}

// Transmit Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_tx_desc {
    pub /: *mut *mut __le64 buffer_addr; / Address of the descriptor's data buffer,
    pub data: __le32,
    pub /: *mut *mut __le16 length; / Data buffer length,
    pub /: *mut *mut u8 cso; / Checksum offset,
    pub /: *mut *mut u8 cmd; / Descriptor control,
    pub flags: },
    pub lower: },
    pub data: __le32,
    pub /: *mut *mut u8 status; / Descriptor status,
    pub /: *mut *mut u8 css; / Checksum start,
    pub special: __le16,
    pub fields: },
    pub upper: },
}

// Offload Context Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_context_desc {
    pub ip_config: __le32,
    pub /: *mut *mut u8 ipcss; / IP checksum start,
    pub /: *mut *mut u8 ipcso; / IP checksum offset,
    pub /: *mut *mut __le16 ipcse; / IP checksum end,
    pub ip_fields: },
    pub lower_setup: },
    pub tcp_config: __le32,
    pub /: *mut *mut u8 tucss; / TCP checksum start,
    pub /: *mut *mut u8 tucso; / TCP checksum offset,
    pub /: *mut *mut __le16 tucse; / TCP checksum end,
    pub tcp_fields: },
    pub upper_setup: },
    pub cmd_and_length: __le32,
    pub data: __le32,
    pub /: *mut *mut u8 status; / Descriptor status,
    pub /: *mut *mut u8 hdr_len; / Header length,
    pub /: *mut *mut __le16 mss; / Maximum segment size,
    pub fields: },
    pub tcp_seg_setup: },
}

// Offload data descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_data_desc {
    pub /: *mut *mut __le64 buffer_addr; / Address of the descriptor's buffer address,
    pub data: __le32,
    pub /: *mut *mut __le16 length; / Data buffer length,
    pub typ_len_ext: u8,
    pub cmd: u8,
    pub flags: },
    pub lower: },
    pub data: __le32,
    pub /: *mut *mut u8 status; / Descriptor status,
    pub /: *mut *mut u8 popts; / Packet Options,
    pub special: __le16,
    pub fields: },
    pub upper: },
}

// Statistics counters collected by the MAC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_hw_stats {
    pub crcerrs: u64,
    pub algnerrc: u64,
    pub symerrs: u64,
    pub rxerrc: u64,
    pub mpc: u64,
    pub scc: u64,
    pub ecol: u64,
    pub mcc: u64,
    pub latecol: u64,
    pub colc: u64,
    pub dc: u64,
    pub tncrs: u64,
    pub sec: u64,
    pub cexterr: u64,
    pub rlec: u64,
    pub xonrxc: u64,
    pub xontxc: u64,
    pub xoffrxc: u64,
    pub xofftxc: u64,
    pub fcruc: u64,
    pub prc64: u64,
    pub prc127: u64,
    pub prc255: u64,
    pub prc511: u64,
    pub prc1023: u64,
    pub prc1522: u64,
    pub gprc: u64,
    pub bprc: u64,
    pub mprc: u64,
    pub gptc: u64,
    pub gorc: u64,
    pub gotc: u64,
    pub rnbc: u64,
    pub ruc: u64,
    pub rfc: u64,
    pub roc: u64,
    pub rjc: u64,
    pub mgprc: u64,
    pub mgpdc: u64,
    pub mgptc: u64,
    pub tor: u64,
    pub tot: u64,
    pub tpr: u64,
    pub tpt: u64,
    pub ptc64: u64,
    pub ptc127: u64,
    pub ptc255: u64,
    pub ptc511: u64,
    pub ptc1023: u64,
    pub ptc1522: u64,
    pub mptc: u64,
    pub bptc: u64,
    pub tsctc: u64,
    pub tsctfc: u64,
    pub iac: u64,
    pub icrxptc: u64,
    pub icrxatc: u64,
    pub ictxptc: u64,
    pub ictxatc: u64,
    pub ictxqec: u64,
    pub ictxqmtc: u64,
    pub icrxdmtc: u64,
    pub icrxoc: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_phy_stats {
    pub idle_errors: u32,
    pub receive_errors: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_host_mng_dhcp_cookie {
    pub signature: u32,
    pub status: u8,
    pub reserved0: u8,
    pub vlan_id: u16,
    pub reserved1: u32,
    pub reserved2: u16,
    pub reserved3: u8,
    pub checksum: u8,
}

// Host Interface "Rev 1"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_host_command_header {
    pub command_id: u8,
    pub command_length: u8,
    pub command_options: u8,
    pub checksum: u8,
}

pub const E1000_HI_MAX_DATA_LENGTH: c_int = 252;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_host_command_info {
    pub command_header: e1000_host_command_header,
    pub command_data: [u8; E1000_HI_MAX_DATA_LENGTH],
}

// Host Interface "Rev 2"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_host_mng_command_header {
    pub command_id: u8,
    pub checksum: u8,
    pub reserved1: u16,
    pub reserved2: u16,
    pub command_length: u16,
}

pub const E1000_HI_MAX_MNG_DATA_LENGTH: c_uint = 0x6F8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_host_mng_command_info {
    pub command_header: e1000_host_mng_command_header,
    pub command_data: [u8; E1000_HI_MAX_MNG_DATA_LENGTH],
}

// Function pointers for the MAC.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_mac_operations {
    pub ): *mut *mut s32 (id_led_init)(struct e1000_hw,
    pub ): *mut *mut s32 (blink_led)(struct e1000_hw,
    pub ): *mut *mut bool (check_mng_mode)(struct e1000_hw,
    pub ): *mut *mut s32 (check_for_link)(struct e1000_hw,
    pub ): *mut *mut s32 (cleanup_led)(struct e1000_hw,
    pub ): *mut *mut void (clear_hw_cntrs)(struct e1000_hw,
    pub ): *mut *mut void (clear_vfta)(struct e1000_hw,
    pub ): *mut *mut s32 (get_bus_info)(struct e1000_hw,
    pub ): *mut *mut void (set_lan_id)(struct e1000_hw,
    pub ): *mut *mut *mut *mut s32 (get_link_up_info)(struct e1000_hw , u16 , u16,
    pub ): *mut *mut s32 (led_on)(struct e1000_hw,
    pub ): *mut *mut s32 (led_off)(struct e1000_hw,
    pub u32): *mut *mut *mut *mut void (update_mc_addr_list)(struct e1000_hw , u8 ,,
    pub ): *mut *mut s32 (reset_hw)(struct e1000_hw,
    pub ): *mut *mut s32 (init_hw)(struct e1000_hw,
    pub ): *mut *mut s32 (setup_link)(struct e1000_hw,
    pub ): *mut *mut s32 (setup_physical_interface)(struct e1000_hw,
    pub ): *mut *mut s32 (setup_led)(struct e1000_hw,
    pub u32): *mut *mut *mut void (write_vfta)(struct e1000_hw , u32,,
    pub ): *mut *mut void (config_collision_dist)(struct e1000_hw,
    pub u32): *mut *mut *mut *mut int (rar_set)(struct e1000_hw , u8 ,,
    pub ): *mut *mut s32 (read_mac_addr)(struct e1000_hw,
    pub ): *mut *mut u32 (rar_get_count)(struct e1000_hw,
}

// When to use various PHY register access functions:
//
// Func   Caller
// Function      Does   Does    When to use
// ~~~~~~~~~~~~  ~~~~~  ~~~~~~  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// X_reg         L,P,A  n/a     for simple PHY reg accesses
// X_reg_locked  P,A    L       for multiple accesses of different regs
// on different pages
// X_reg_page    A      L,P     for multiple accesses of different regs
// on the same page
//
// Where X=[read|write], L=locking, P=sets page, A=register access
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_phy_operations {
    pub ): *mut *mut s32 (acquire)(struct e1000_hw,
    pub ): *mut *mut s32 (cfg_on_link_up)(struct e1000_hw,
    pub ): *mut *mut s32 (check_polarity)(struct e1000_hw,
    pub ): *mut *mut s32 (check_reset_block)(struct e1000_hw,
    pub ): *mut *mut s32 (commit)(struct e1000_hw,
    pub ): *mut *mut s32 (force_speed_duplex)(struct e1000_hw,
    pub hw): *mut *mut s32 (get_cfg_done)(struct e1000_hw,
    pub ): *mut *mut s32 (get_cable_length)(struct e1000_hw,
    pub ): *mut *mut s32 (get_info)(struct e1000_hw,
    pub u16): *mut *mut *mut s32 (set_page)(struct e1000_hw ,,
    pub ): *mut *mut *mut s32 (read_reg)(struct e1000_hw , u32, u16,
    pub ): *mut *mut *mut s32 (read_reg_locked)(struct e1000_hw , u32, u16,
    pub ): *mut *mut *mut s32 (read_reg_page)(struct e1000_hw , u32, u16,
    pub ): *mut *mut void (release)(struct e1000_hw,
    pub ): *mut *mut s32 (reset)(struct e1000_hw,
    pub bool): *mut *mut *mut s32 (set_d0_lplu_state)(struct e1000_hw ,,
    pub bool): *mut *mut *mut s32 (set_d3_lplu_state)(struct e1000_hw ,,
    pub u16): *mut *mut *mut s32 (write_reg)(struct e1000_hw , u32,,
    pub u16): *mut *mut *mut s32 (write_reg_locked)(struct e1000_hw , u32,,
    pub u16): *mut *mut *mut s32 (write_reg_page)(struct e1000_hw , u32,,
    pub ): *mut *mut void (power_up)(struct e1000_hw,
    pub ): *mut *mut void (power_down)(struct e1000_hw,
}

// Function pointers for the NVM.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_nvm_operations {
    pub ): *mut *mut s32 (acquire)(struct e1000_hw,
    pub ): *mut *mut *mut s32 (read)(struct e1000_hw , u16, u16, u16,
    pub ): *mut *mut void (release)(struct e1000_hw,
    pub ): *mut *mut void (reload)(struct e1000_hw,
    pub ): *mut *mut s32 (update)(struct e1000_hw,
    pub ): *mut *mut *mut s32 (valid_led_default)(struct e1000_hw , u16,
    pub ): *mut *mut s32 (validate)(struct e1000_hw,
    pub ): *mut *mut *mut s32 (write)(struct e1000_hw , u16, u16, u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_mac_info {
    pub ops: e1000_mac_operations,
    pub addr: [u8; ETH_ALEN],
    pub perm_addr: [u8; ETH_ALEN],
    pub type: e1000_mac_type,
    pub collision_delta: u32,
    pub ledctl_default: u32,
    pub ledctl_mode1: u32,
    pub ledctl_mode2: u32,
    pub mc_filter_type: u32,
    pub tx_packet_delta: u32,
    pub txcw: u32,
    pub current_ifs_val: u16,
    pub ifs_max_val: u16,
    pub ifs_min_val: u16,
    pub ifs_ratio: u16,
    pub ifs_step_size: u16,
    pub mta_reg_count: u16,
// Maximum size of the MTA register table in all supported adapters
pub const MAX_MTA_REG: c_int = 128;
    pub mta_shadow: [u32; MAX_MTA_REG],
    pub rar_entry_count: u16,
    pub forced_speed_duplex: u8,
    pub adaptive_ifs: bool,
    pub has_fwsm: bool,
    pub arc_subsystem_valid: bool,
    pub autoneg: bool,
    pub autoneg_failed: bool,
    pub get_link_status: bool,
    pub in_ifs_mode: bool,
    pub serdes_has_link: bool,
    pub tx_pkt_filtering: bool,
    pub serdes_link_state: e1000_serdes_link_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_phy_info {
    pub ops: e1000_phy_operations,
    pub type: e1000_phy_type,
    pub local_rx: e1000_1000t_rx_status,
    pub remote_rx: e1000_1000t_rx_status,
    pub ms_type: e1000_ms_type,
    pub original_ms_type: e1000_ms_type,
    pub cable_polarity: e1000_rev_polarity,
    pub smart_speed: e1000_smart_speed,
    pub addr: u32,
    pub id: u32,
    pub /: *mut *mut u32 reset_delay_us; / in usec,
    pub revision: u32,
    pub retry_count: u32,
    pub media_type: e1000_media_type,
    pub autoneg_advertised: u16,
    pub autoneg_mask: u16,
    pub cable_length: u16,
    pub max_cable_length: u16,
    pub min_cable_length: u16,
    pub mdix: u8,
    pub disable_polarity_correction: bool,
    pub is_mdix: bool,
    pub polarity_correction: bool,
    pub speed_downgraded: bool,
    pub autoneg_wait_to_complete: bool,
    pub retry_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_nvm_info {
    pub ops: e1000_nvm_operations,
    pub type: e1000_nvm_type,
    pub override: e1000_nvm_override,
    pub flash_bank_size: u32,
    pub flash_base_addr: u32,
    pub word_size: u16,
    pub delay_usec: u16,
    pub address_bits: u16,
    pub opcode_bits: u16,
    pub page_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_bus_info {
    pub width: e1000_bus_width,
    pub func: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_fc_info {
    pub /: *mut *mut u32 high_water; / Flow control high-water mark,
    pub /: *mut *mut u32 low_water; / Flow control low-water mark,
    pub /: *mut *mut u16 pause_time; / Flow control pause timer,
    pub /: *mut *mut u16 refresh_time; / Flow control refresh timer,
    pub /: *mut *mut bool send_xon; / Flow control send XON,
    pub /: *mut *mut bool strict_ieee; / Strict IEEE mode,
    pub /: *mut *mut e1000_fc_mode current_mode; / FC mode in effect,
    pub /: *mut *mut e1000_fc_mode requested_mode; / FC mode requested by caller,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_dev_spec_82571 {
    pub laa_is_present: bool,
    pub smb_counter: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_dev_spec_80003es2lan {
    pub mdic_wa_enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_shadow_ram {
    pub value: u16,
    pub modified: bool,
}

pub const E1000_ICH8_SHADOW_RAM_WORDS: c_int = 2048;
// I218 PHY Ultra Low Power (ULP) states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_ulp_state {
    e1000_ulp_state_unknown,
    e1000_ulp_state_off,
    e1000_ulp_state_on,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_dev_spec_ich8lan {
    pub kmrn_lock_loss_workaround_enabled: bool,
    pub shadow_ram: [e1000_shadow_ram; E1000_ICH8_SHADOW_RAM_WORDS],
    pub nvm_k1_enabled: bool,
    pub eee_disable: bool,
    pub eee_lp_ability: u16,
    pub ulp_state: e1000_ulp_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_hw {
    pub adapter: *mut e1000_adapter,
    pub hw_addr: *mut void __iomem,
    pub flash_address: *mut void __iomem,
    pub mac: e1000_mac_info,
    pub fc: e1000_fc_info,
    pub phy: e1000_phy_info,
    pub nvm: e1000_nvm_info,
    pub bus: e1000_bus_info,
    pub mng_cookie: e1000_host_mng_dhcp_cookie,
    pub e82571: e1000_dev_spec_82571,
    pub e80003es2lan: e1000_dev_spec_80003es2lan,
    pub ich8lan: e1000_dev_spec_ich8lan,
    pub dev_spec: },
}

