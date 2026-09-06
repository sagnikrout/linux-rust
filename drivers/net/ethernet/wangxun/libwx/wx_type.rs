//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/wangxun/libwx/wx_type.h
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
// Copyright (c) 2015 - 2022 Beijing WangXun Technology Co., Ltd.

pub const WX_NCSI_SUP: c_uint = 0x8000;
pub const WX_NCSI_MASK: c_uint = 0x8000;
pub const WX_WOL_SUP: c_uint = 0x4000;
pub const WX_WOL_MASK: c_uint = 0x4000;
// MSI-X capability fields masks
pub const WX_PCIE_MSIX_TBL_SZ_MASK: c_uint = 0x7FF;
pub const WX_PCI_LINK_STATUS: c_uint = 0xB2;
pub const WX_MAX_PF_MACVLANS: c_int = 15;
pub const WX_MAX_VF_MC_ENTRIES: c_int = 30;
// Global Registers

// chip control Registers
pub const WX_MIS_PWR: c_uint = 0x10000;
pub const WX_MIS_RST: c_uint = 0x1000C;

pub const WX_MIS_ST: c_uint = 0x10028;

pub const WX_MIS_SWSM: c_uint = 0x1002C;

pub const WX_MIS_RST_ST: c_uint = 0x10030;
pub const WX_MIS_RST_ST_RST_INI_SHIFT: c_int = 8;

// FMGR Registers
pub const WX_SPI_CMD: c_uint = 0x10104;
pub const WX_SPI_CMD_READ_DWORD: c_uint = 0x1;
pub const WX_SPI_CLK_DIV: c_uint = 0x3;

pub const WX_SPI_DATA: c_uint = 0x10108;

pub const WX_SPI_STATUS: c_uint = 0x1010C;

pub const WX_SPI_ILDR_STATUS: c_uint = 0x10120;
// Sensors for PVT(Process Voltage Temperature)
pub const WX_TS_EN: c_uint = 0x10304;

pub const WX_TS_ALARM_THRE: c_uint = 0x1030C;
pub const WX_TS_DALARM_THRE: c_uint = 0x10310;
pub const WX_TS_INT_EN: c_uint = 0x10314;

pub const WX_TS_ALARM_ST: c_uint = 0x10318;

// statistic
pub const WX_TX_FRAME_CNT_GOOD_BAD_L: c_uint = 0x1181C;
pub const WX_TX_BC_FRAMES_GOOD_L: c_uint = 0x11824;
pub const WX_TX_MC_FRAMES_GOOD_L: c_uint = 0x1182C;
pub const WX_RX_FRAME_CNT_GOOD_BAD_L: c_uint = 0x11900;
pub const WX_RX_BC_FRAMES_GOOD_L: c_uint = 0x11918;
pub const WX_RX_MC_FRAMES_GOOD_L: c_uint = 0x11920;
pub const WX_RX_CRC_ERROR_FRAMES_L: c_uint = 0x11928;
pub const WX_RX_LEN_ERROR_FRAMES_L: c_uint = 0x11978;
pub const WX_RX_UNDERSIZE_FRAMES_GOOD: c_uint = 0x11938;
pub const WX_RX_OVERSIZE_FRAMES_GOOD: c_uint = 0x1193C;
pub const WX_MAC_LXOFFRXC: c_uint = 0x11988;
pub const WX_MAC_LXONRXC: c_uint = 0x11E0C;
pub const WX_MAC_LXOFFRXC_AML: c_uint = 0x11F80;
pub const WX_MAC_LXONRXC_AML: c_uint = 0x11F84;
// Receive DMA registers

pub const WX_RDM_RSC_CTL: c_uint = 0x1200C;

pub const WX_RDM_DCACHE_CTL: c_uint = 0x120A8;

pub const WX_RDM_DRP_PKT: c_uint = 0x12500;
pub const WX_RDM_PKT_CNT: c_uint = 0x12504;
pub const WX_RDM_BYTE_CNT_LSB: c_uint = 0x12508;
pub const WX_RDM_BMC2OS_CNT: c_uint = 0x12510;
// Port Registers
// port cfg Registers
pub const WX_CFG_PORT_CTL: c_uint = 0x14400;

pub const WX_CFG_PORT_ST: c_uint = 0x14404;

pub const WX_CFG_PORT_CTL_NUM_VT_NONE: c_int = 0;

// GPIO Registers
pub const WX_GPIO_DR: c_uint = 0x14800;

pub const WX_GPIO_DDR: c_uint = 0x14804;

pub const WX_GPIO_CTL: c_uint = 0x14808;
pub const WX_GPIO_INTEN: c_uint = 0x14830;

pub const WX_GPIO_INTMASK: c_uint = 0x14834;
pub const WX_GPIO_INTTYPE_LEVEL: c_uint = 0x14838;
pub const WX_GPIO_POLARITY: c_uint = 0x1483C;
pub const WX_GPIO_INTSTATUS: c_uint = 0x14844;
pub const WX_GPIO_EOI: c_uint = 0x1484C;
pub const WX_GPIO_EXT: c_uint = 0x14850;
// Transmit DMA registers
// transmit global control
pub const WX_TDM_CTL: c_uint = 0x18000;

// TDM CTL BIT

pub const WX_TDM_RP_IDX: c_uint = 0x1820C;
pub const WX_TDM_PKT_CNT: c_uint = 0x18308;
pub const WX_TDM_BYTE_CNT_LSB: c_uint = 0x1830C;
pub const WX_TDM_OS2BMC_CNT: c_uint = 0x18314;
pub const WX_TDM_RP_RATE: c_uint = 0x18404;
// RDB registers
// receive packet buffer
pub const WX_RDB_PB_CTL: c_uint = 0x19000;

pub const WX_RDB_PB_SZ_SHIFT: c_int = 10;
// statistic
pub const WX_RDB_PFCMACDAL: c_uint = 0x19210;
pub const WX_RDB_PFCMACDAH: c_uint = 0x19214;
pub const WX_RDB_LXOFFTXC: c_uint = 0x19218;
pub const WX_RDB_LXONTXC: c_uint = 0x1921C;
// Flow Control Registers
pub const WX_RDB_RFCV: c_uint = 0x19200;
pub const WX_RDB_RFCL: c_uint = 0x19220;

pub const WX_RDB_RFCH: c_uint = 0x19260;

pub const WX_RDB_RFCRT: c_uint = 0x192A0;
pub const WX_RDB_RFCC: c_uint = 0x192A4;

// ring assignment

pub const WX_RDB_RA_CTL: c_uint = 0x194F4;

pub const WX_RDB_FDIR_MATCH: c_uint = 0x19558;
pub const WX_RDB_FDIR_MISS: c_uint = 0x1955C;
// VM RSS

// PSR Registers
// psr control
pub const WX_PSR_CTL: c_uint = 0x15000;
pub const WX_PSR_VM_CTL: c_uint = 0x151B0;
// Header split receive

pub const WX_PSR_CTL_MO_SHIFT: c_int = 5;

pub const WX_PSR_MAX_SZ: c_uint = 0x15020;
pub const WX_PSR_VLAN_CTL: c_uint = 0x15088;

// EType Queue Filter

pub const WX_PSR_ETYPE_SWC_FILTER_1588: c_int = 3;

// 1588
pub const WX_PSR_1588_MSG: c_uint = 0x15120;

pub const WX_PSR_1588_STMPL: c_uint = 0x151E8;
pub const WX_PSR_1588_STMPH: c_uint = 0x151A4;
pub const WX_PSR_1588_CTL: c_uint = 0x15188;

// mcasst/ucast overflow tbl

// VM L2 contorl

// Management
pub const WX_PSR_MNG_FLEX_SEL: c_uint = 0x1582C;

pub const WX_PSR_LAN_FLEX_SEL: c_uint = 0x15B8C;

pub const WX_PSR_WKUP_CTL: c_uint = 0x15B80;
// Wake Up Filter Control Bit

// vlan tbl

// mac switcher
pub const WX_PSR_MAC_SWC_AD_L: c_uint = 0x16200;
pub const WX_PSR_MAC_SWC_AD_H: c_uint = 0x16204;

pub const WX_PSR_MAC_SWC_VM_L: c_uint = 0x16208;
pub const WX_PSR_MAC_SWC_VM_H: c_uint = 0x1620C;
pub const WX_PSR_MAC_SWC_IDX: c_uint = 0x16210;
pub const WX_CLEAR_VMDQ_ALL: c_uint = 0xFFFFFFFFU;
// vlan switch
pub const WX_PSR_VLAN_SWC: c_uint = 0x16220;
pub const WX_PSR_VLAN_SWC_VM_L: c_uint = 0x16224;
pub const WX_PSR_VLAN_SWC_VM_H: c_uint = 0x16228;

pub const WX_PSR_VLAN_SWC_IDX: c_uint = 0x16230         /* 64 vlan entries */;
// VLAN pool filtering masks

pub const WX_PSR_VLAN_SWC_ENTRIES: c_int = 64;

// RSEC
// general rsec
pub const WX_RSC_CTL: c_uint = 0x17000;

pub const WX_RSC_ST: c_uint = 0x17004;

// Transmit DMA registers
// transmit global control

// Per VF Port VLAN insertion rules

// TDB

pub const WX_TXPKT_SIZE_MAX: c_uint = 0xA /* Max Tx Packet size */;
// TSEC
// Security Control Registers
pub const WX_TSC_CTL: c_uint = 0x1D000;

pub const WX_TSC_ST: c_uint = 0x1D004;

pub const WX_TSC_BUF_AE: c_uint = 0x1D00C;

// 1588
pub const WX_TSC_1588_CTL: c_uint = 0x11F00;

pub const WX_TSC_1588_STMPL: c_uint = 0x11F04;
pub const WX_TSC_1588_STMPH: c_uint = 0x11F08;
pub const WX_TSC_1588_SYSTIML: c_uint = 0x11F0C;
pub const WX_TSC_1588_SYSTIMH: c_uint = 0x11F10;
pub const WX_TSC_1588_INC: c_uint = 0x11F14;
pub const WX_TSC_1588_INT_ST: c_uint = 0x11F20;

pub const WX_TSC_1588_INT_EN: c_uint = 0x11F24;

pub const WX_TSC_1588_AUX_CTL: c_uint = 0x11F28;

// MNG
pub const WX_MNG_SWFW_SYNC: c_uint = 0x1E008;

pub const WX_MNG_MBOX: c_uint = 0x1E100;
pub const WX_MNG_MBOX_CTL: c_uint = 0x1E044;

pub const WX_MNG_BMC2OS_CNT: c_uint = 0x1E090;
pub const WX_MNG_OS2BMC_CNT: c_uint = 0x1E094;
pub const WX_SW2FW_MBOX_CMD: c_uint = 0x1E0A0;

pub const WX_SW2FW_MBOX: c_uint = 0x1E200;
pub const WX_FW2SW_MBOX: c_uint = 0x1E300;
// ETH MAC
pub const WX_MAC_TX_CFG: c_uint = 0x11000;

pub const WX_MAC_RX_CFG: c_uint = 0x11004;

pub const WX_MAC_PKT_FLT: c_uint = 0x11008;

pub const WX_MAC_WDG_TIMEOUT: c_uint = 0x1100C;
pub const WX_MAC_RX_FLOW_CTRL: c_uint = 0x11090;

pub const WX_MAC_WDG_TIMEOUT_WTO_DELTA: c_int = 2;
// MDIO Registers
pub const WX_MSCA: c_uint = 0x11200;

pub const WX_MSCC: c_uint = 0x11204;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WX_MSCA_CMD_value {
    WX_MSCA_CMD_RSV = 0,
    WX_MSCA_CMD_WRITE,
    WX_MSCA_CMD_POST_READ,
    WX_MSCA_CMD_READ,
}

pub const WX_MDIO_CLAUSE_SELECT: c_uint = 0x11220;
pub const WX_MMC_CONTROL: c_uint = 0x11800;

// BAR registers
// Interrupt Registers
pub const WX_BME_CTL: c_uint = 0x12020;
pub const WX_PX_MISC_IC: c_uint = 0x100;
pub const WX_PX_MISC_ICS: c_uint = 0x104;
pub const WX_PX_MISC_IEN: c_uint = 0x108;
pub const WX_PX_INTA: c_uint = 0x110;
pub const WX_PX_GPIE: c_uint = 0x118;

pub const WX_PX_ISB_ADDR_L: c_uint = 0x160;
pub const WX_PX_ISB_ADDR_H: c_uint = 0x164;
pub const WX_PX_TRANSACTION_PENDING: c_uint = 0x168;
pub const WX_PX_ITRSEL: c_uint = 0x180;

pub const WX_PX_MISC_IVAR: c_uint = 0x4FC;

pub const WX_PX_IVAR_ALLOC_VAL: c_uint = 0x80 /* Interrupt Allocation valid */;
pub const WX_7K_ITR: c_int = 595;
pub const WX_12K_ITR: c_int = 336;
pub const WX_20K_ITR: c_int = 200;
pub const WX_MIN_RSC_ITR: c_int = 24;
pub const WX_SP_MAX_EITR: c_uint = 0x00000FF8U;
pub const WX_AML_MAX_EITR: c_uint = 0x00000FFFU;
pub const WX_EM_MAX_EITR: c_uint = 0x00007FFCU;
// transmit DMA Registers

// Transmit Config masks

pub const WX_PX_TR_CFG_THRE_SHIFT: c_int = 8;

// Receive DMA Registers

// PX_RR_CFG bit definitions

pub const WX_PX_RR_CFG_RR_THER_SHIFT: c_int = 16;

// + at bit 8 offset (<< 12)
// = (<< 6)
//

pub const WX_PX_RR_CFG_RR_SIZE_SHIFT: c_int = 1;

// Number of 80 microseconds we wait for PCI Express master disable
pub const WX_PCI_MASTER_DISABLE_TIMEOUT: c_int = 80000;
pub const WX_RSS_64Q_MASK: c_uint = 0x3F;
pub const WX_RSS_8Q_MASK: c_uint = 0x7;
pub const WX_RSS_4Q_MASK: c_uint = 0x3;
pub const WX_RSS_2Q_MASK: c_uint = 0x1;
pub const WX_RSS_DISABLED_MASK: c_uint = 0x0;
pub const WX_VMDQ_4Q_MASK: c_uint = 0x7C;
pub const WX_VMDQ_2Q_MASK: c_uint = 0x7E;
pub const WX_VMDQ_1Q_MASK: c_uint = 0x7F;
// Manageablility Host Interface defines

pub const WX_HIC_HDR_INDEX_MAX: c_int = 255;
pub const FW_READ_SHADOW_RAM_CMD: c_uint = 0x31;
pub const FW_READ_SHADOW_RAM_LEN: c_uint = 0x6;
pub const FW_DEFAULT_CHECKSUM: c_uint = 0xFF /* checksum always 0xFF */;
pub const FW_NVM_DATA_OFFSET: c_int = 3;
pub const FW_MAX_READ_BUFFER_SIZE: c_int = 244;
pub const FW_RESET_CMD: c_uint = 0xDF;
pub const FW_RESET_LEN: c_uint = 0x2;
pub const FW_CEM_HDR_LEN: c_uint = 0x4;

pub const FW_CEM_MAX_RETRIES: c_int = 3;
pub const FW_CEM_RESP_STATUS_SUCCESS: c_uint = 0x1;
pub const FW_PPS_SET_CMD: c_uint = 0xF6;
pub const FW_PPS_SET_LEN: c_uint = 0x14;
pub const WX_SW_REGION_PTR: c_uint = 0x1C;
pub const WX_MAC_STATE_DEFAULT: c_uint = 0x1;
pub const WX_MAC_STATE_MODIFIED: c_uint = 0x2;
pub const WX_MAC_STATE_IN_USE: c_uint = 0x4;
// BitTimes (BT) conversion

// Calculate Delay to respond to PFC
pub const WX_PFC_D: c_int = 672;
// Calculate Cable Delay

// Calculate Delay incurred from higher layer
pub const WX_HD: c_int = 6144;
// Calculate Interface Delay
pub const WX_PHY_D: c_int = 12800;
pub const WX_MAC_D: c_int = 4096;

// Calculate PCI Bus delay for low thresholds
pub const WX_PCI_DELAY: c_int = 10000;
// Calculate delay value in bit times

// Calculate low threshold delay values

// flow control
pub const WX_DEFAULT_FCPAUSE: c_uint = 0xFFFF;
pub const WX_MAX_RXD: c_int = 8192;
pub const WX_MAX_TXD: c_int = 8192;
pub const WX_MIN_RXD: c_int = 128;
pub const WX_MIN_TXD: c_int = 128;
// Number of Transmit and Receive Descriptors must be a multiple of 128
pub const WX_REQ_RX_DESCRIPTOR_MULTIPLE: c_int = 128;
pub const WX_REQ_TX_DESCRIPTOR_MULTIPLE: c_int = 128;

// Supported Rx Buffer Sizes

pub const WX_RXBUFFER_2K: c_int = 2048;
pub const WX_RXBUFFER_3K: c_int = 3072;

// Tx Descriptors needed, worst case

// Receive Descriptor bit definitions

// RSS Hash results

pub const WX_RXD_RSSTYPE_IPV4_TCP: c_uint = 0x00000001U;
pub const WX_RXD_RSSTYPE_IPV6_TCP: c_uint = 0x00000003U;
pub const WX_RXD_RSSTYPE_IPV4_SCTP: c_uint = 0x00000004U;
pub const WX_RXD_RSSTYPE_IPV6_SCTP: c_uint = 0x00000006U;
pub const WX_RXD_RSSTYPE_IPV4_UDP: c_uint = 0x00000007U;
pub const WX_RXD_RSSTYPE_IPV6_UDP: c_uint = 0x00000008U;

// TUN
pub const WX_PTYPE_TUN_IPV4: c_uint = 0x80;
pub const WX_PTYPE_TUN_IPV6: c_uint = 0xC0;
// PKT for TUN
pub const WX_PTYPE_PKT_IPIP: c_uint = 0x00 /* IP+IP */;
pub const WX_PTYPE_PKT_IG: c_uint = 0x10 /* IP+GRE */;
pub const WX_PTYPE_PKT_IGM: c_uint = 0x20 /* IP+GRE+MAC */;
pub const WX_PTYPE_PKT_IGMV: c_uint = 0x30 /* IP+GRE+MAC+VLAN */;
// PKT for !TUN
pub const WX_PTYPE_PKT_MAC: c_uint = 0x10;
pub const WX_PTYPE_PKT_IP: c_uint = 0x20;
// TYP for PKT=mac
pub const WX_PTYPE_TYP_MAC: c_uint = 0x01;
// TYP for PKT=ip
pub const WX_PTYPE_PKT_IPV6: c_uint = 0x08;
pub const WX_PTYPE_TYP_IPFRAG: c_uint = 0x01;
pub const WX_PTYPE_TYP_IP: c_uint = 0x02;
pub const WX_PTYPE_TYP_UDP: c_uint = 0x03;
pub const WX_PTYPE_TYP_TCP: c_uint = 0x04;
pub const WX_PTYPE_TYP_SCTP: c_uint = 0x05;
// Packet type non-ip values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_l2_ptypes {
    WX_PTYPE_L2_ABORTED = (WX_PTYPE_PKT_MAC),
    WX_PTYPE_L2_MAC = (WX_PTYPE_PKT_MAC | WX_PTYPE_TYP_MAC),

    WX_PTYPE_L2_IPV4_FRAG = (WX_PTYPE_PKT_IP | WX_PTYPE_TYP_IPFRAG),
    WX_PTYPE_L2_IPV4 = (WX_PTYPE_PKT_IP | WX_PTYPE_TYP_IP),
    WX_PTYPE_L2_IPV4_UDP = (WX_PTYPE_PKT_IP | WX_PTYPE_TYP_UDP),
    WX_PTYPE_L2_IPV4_TCP = (WX_PTYPE_PKT_IP | WX_PTYPE_TYP_TCP),
    WX_PTYPE_L2_IPV4_SCTP = (WX_PTYPE_PKT_IP | WX_PTYPE_TYP_SCTP),
    WX_PTYPE_L2_IPV6_FRAG = (WX_PTYPE_PKT_IP | WX_PTYPE_PKT_IPV6 |
    WX_PTYPE_TYP_IPFRAG),
    WX_PTYPE_L2_IPV6 = (WX_PTYPE_PKT_IP | WX_PTYPE_PKT_IPV6 |
    WX_PTYPE_TYP_IP),
    WX_PTYPE_L2_IPV6_UDP = (WX_PTYPE_PKT_IP | WX_PTYPE_PKT_IPV6 |
    WX_PTYPE_TYP_UDP),
    WX_PTYPE_L2_IPV6_TCP = (WX_PTYPE_PKT_IP | WX_PTYPE_PKT_IPV6 |
    WX_PTYPE_TYP_TCP),
    WX_PTYPE_L2_IPV6_SCTP = (WX_PTYPE_PKT_IP | WX_PTYPE_PKT_IPV6 |
    WX_PTYPE_TYP_SCTP),

    WX_PTYPE_L2_TUN4_MAC = (WX_PTYPE_TUN_IPV4 | WX_PTYPE_PKT_IGM),
    WX_PTYPE_L2_TUN6_MAC = (WX_PTYPE_TUN_IPV6 | WX_PTYPE_PKT_IGM),
}

pub const WX_RXD_RSCCNT_SHIFT: c_int = 17;

pub const WX_RXD_NEXTP_SHIFT: c_int = 4;
// Transmit Descriptor Config Masks

// Adv Transmit Descriptor Config Masks

pub const WX_TXD_TAG_TPID_SEL_SHIFT: c_int = 11;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_tx_flags {
// cmd_type flags
    WX_TX_FLAGS_HW_VLAN	= 0x01,
    WX_TX_FLAGS_TSO		= 0x02,
    WX_TX_FLAGS_TSTAMP	= 0x04,

// olinfo flags
    WX_TX_FLAGS_CC		= 0x08,
    WX_TX_FLAGS_IPV4	= 0x10,
    WX_TX_FLAGS_CSUM	= 0x20,
    WX_TX_FLAGS_OUTER_IPV4	= 0x100,
    WX_TX_FLAGS_LINKSEC	= 0x200,
    WX_TX_FLAGS_IPSEC	= 0x400,

// software defined flags
    WX_TX_FLAGS_SW_VLAN	= 0x40,
}

// VLAN info

pub const WX_TX_FLAGS_VLAN_SHIFT: c_int = 16;
// wx_dec_ptype.mac: outer mac
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_dec_ptype_mac {
    WX_DEC_PTYPE_MAC_IP	= 0,
    WX_DEC_PTYPE_MAC_L2	= 2,
    WX_DEC_PTYPE_MAC_FCOE	= 3,
}

// wx_dec_ptype.[e]ip: outer&encaped ip
pub const WX_DEC_PTYPE_IP_FRAG: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_dec_ptype_ip {
    WX_DEC_PTYPE_IP_NONE = 0,
    WX_DEC_PTYPE_IP_IPV4 = 1,
    WX_DEC_PTYPE_IP_IPV6 = 2,
    WX_DEC_PTYPE_IP_FGV4 = WX_DEC_PTYPE_IP_FRAG | WX_DEC_PTYPE_IP_IPV4,
    WX_DEC_PTYPE_IP_FGV6 = WX_DEC_PTYPE_IP_FRAG | WX_DEC_PTYPE_IP_IPV6,
}

// wx_dec_ptype.etype: encaped type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_dec_ptype_etype {
    WX_DEC_PTYPE_ETYPE_NONE	= 0,
    WX_DEC_PTYPE_ETYPE_IPIP	= 1,	/* IP+IP */
    WX_DEC_PTYPE_ETYPE_IG	= 2,	/* IP+GRE */
    WX_DEC_PTYPE_ETYPE_IGM	= 3,	/* IP+GRE+MAC */
    WX_DEC_PTYPE_ETYPE_IGMV	= 4,	/* IP+GRE+MAC+VLAN */
}

// wx_dec_ptype.proto: payload proto
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_dec_ptype_prot {
    WX_DEC_PTYPE_PROT_NONE	= 0,
    WX_DEC_PTYPE_PROT_UDP	= 1,
    WX_DEC_PTYPE_PROT_TCP	= 2,
    WX_DEC_PTYPE_PROT_SCTP	= 3,
    WX_DEC_PTYPE_PROT_ICMP	= 4,
    WX_DEC_PTYPE_PROT_TS	= 5,	/* time sync */
}

// wx_dec_ptype.layer: payload layer
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_dec_ptype_layer {
    WX_DEC_PTYPE_LAYER_NONE = 0,
    WX_DEC_PTYPE_LAYER_PAY2 = 1,
    WX_DEC_PTYPE_LAYER_PAY3 = 2,
    WX_DEC_PTYPE_LAYER_PAY4 = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_dec_ptype {
    pub known:1: u32,
    pub /: *mut *mut u32 mac:2; / outer mac,
    pub ip*/: *mut *mut u32 ip:3; / outer,
    pub /: *mut *mut u32 etype:3; / encaped type,
    pub /: *mut *mut u32 eip:3; / encaped ip,
    pub /: *mut *mut u32 prot:4; / payload proto,
    pub /: *mut *mut u32 layer:3; / payload layer,
}

// macro to make the table lines short

// Host Interface Command Structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_hic_hdr {
    pub cmd: u8,
    pub buf_len: u8,
    pub cmd_resv: u8,
    pub ret_status: u8,
    pub cmd_or_resp: },
    pub checksum: u8,
    pub index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_hic_hdr2_req {
    pub cmd: u8,
    pub buf_lenh: u8,
    pub buf_lenl: u8,
    pub checksum: u8,
    pub index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_hic_hdr2_rsp {
    pub cmd: u8,
    pub buf_lenl: u8,
    pub /: *mut *mut u8 buf_lenh_status; / 7-5: high bits of buf_len, 4-0: status,
    pub checksum: u8,
    pub index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union wx_hic_hdr2 {
    pub req: wx_hic_hdr2_req,
    pub rsp: wx_hic_hdr2_rsp,
}

// These need to be dword aligned
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_hic_read_shadow_ram {
    pub hdr: wx_hic_hdr2,
    pub address: u32,
    pub length: u16,
    pub pad2: u16,
    pub data: u16,
    pub pad3: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_hic_reset {
    pub hdr: wx_hic_hdr,
    pub lan_id: u16,
    pub reset_type: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_hic_set_pps {
    pub hdr: wx_hic_hdr,
    pub lan_id: u8,
    pub enable: u8,
    pub pad2: u16,
    pub nsec: u64,
    pub cycles: u64,
}

// Bus parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_bus_info {
    pub func: u8,
    pub device: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_mbx_info {
    pub size: u16,
    pub mailbox: u32,
    pub udelay: u32,
    pub timeout: u32,
// lock mbx access
    pub mbx_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_thermal_sensor_data {
    pub temp: i16,
    pub alarm_thresh: i16,
    pub dalarm_thresh: i16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_mac_type {
    wx_mac_unknown = 0,
    wx_mac_sp,
    wx_mac_em,
    wx_mac_aml,
    wx_mac_aml40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_media_type {
    wx_media_unknown = 0,
    wx_media_fiber,
    wx_media_copper,
    wx_media_backplane
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum em_mac_type {
    em_mac_type_unknown = 0,
    em_mac_type_mdi,
    em_mac_type_rgmii
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_mac_info {
    pub type: wx_mac_type,
    pub set_lben: bool,
    pub addr: [u8; ETH_ALEN],
    pub perm_addr: [u8; ETH_ALEN],
    pub mta_shadow: [u32; 128],
    pub mc_filter_type: i32,
    pub mcft_size: u32,
    pub vft_shadow: [u32; 128],
    pub vft_size: u32,
    pub num_rar_entries: u32,
    pub rx_pb_size: u32,
    pub tx_pb_size: u32,
    pub max_tx_queues: u32,
    pub max_rx_queues: u32,
    pub max_msix_vectors: u16,
    pub sensor: wx_thermal_sensor_data,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_eeprom_type {
    wx_eeprom_uninitialized = 0,
    wx_eeprom_spi,
    wx_flash,
    wx_eeprom_none /* No NVM support */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_eeprom_info {
    pub type: wx_eeprom_type,
    pub semaphore_delay: u32,
    pub word_size: u16,
    pub sw_region_offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_addr_filter_info {
    pub num_mc_addrs: u32,
    pub mta_in_use: u32,
    pub user_set_promisc: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_mac_addr {
    pub addr: [u8; ETH_ALEN],
    pub /: *mut *mut u16 state; / bitmask,
    pub pools: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_reset_type {
    WX_LAN_RESET = 0,
    WX_SW_RESET,
    WX_GLOBAL_RESET
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_cb {
    pub dma: dma_addr_t,
    pub /: *mut *mut u16 append_cnt; / number of skb's appended,
    pub dma_released: bool,
}

// Transmit Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub union wx_tx_desc {
    pub /: *mut *mut __le64 buffer_addr; / Address of descriptor's data buf,
    pub cmd_type_len: __le32,
    pub olinfo_status: __le32,
    pub read: },
    pub /: *mut *mut __le64 rsvd; / Reserved,
    pub nxtseq_seed: __le32,
    pub status: __le32,
    pub wb: },
}

// Receive Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub union wx_rx_desc {
    pub /: *mut *mut __le64 pkt_addr; / Packet buffer address,
    pub /: *mut *mut __le64 hdr_addr; / Header buffer address,
    pub read: },
    pub data: __le32,
    pub /: *mut *mut __le16 pkt_info; / RSS, Pkt type,
    pub /: *mut *mut __le16 hdr_info; / Splithdr, hdrlen,
    pub hs_rss: },
    pub lo_dword: },
    pub /: *mut *mut __le32 rss; / RSS Hash,
    pub /: *mut *mut __le16 ip_id; / IP id,
    pub /: *mut *mut __le16 csum; / Packet Checksum,
    pub csum_ip: },
    pub hi_dword: },
    pub lower: },
    pub /: *mut *mut __le32 status_error; / ext status/error,
    pub /: *mut *mut __le16 length; / Packet length,
    pub /: *mut *mut __le16 vlan; / VLAN tag,
    pub upper: },
    pub /: *mut *mut } wb; / writeback,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_tx_context_desc {
    pub vlan_macip_lens: __le32,
    pub seqnum_seed: __le32,
    pub type_tucmd_mlhl: __le32,
    pub mss_l4len_idx: __le32,
}

// if _flag is in _input, return _result

// wrapper around a pointer to a socket buffer,
// so a DMA handle can be stored along with the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_tx_buffer {
    pub next_to_watch: *mut wx_tx_desc,
    pub time_stamp: c_ulong,
    pub skb: *mut sk_buff,
    pub bytecount: c_uint,
    pub gso_segs: c_ushort,
    pub protocol: __be16,
    pub tx_flags: u32,
    pub next_eop: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_rx_buffer {
    pub skb: *mut sk_buff,
    pub dma: dma_addr_t,
    pub page: *mut page,
    pub page_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_queue_stats {
    pub packets: u64,
    pub bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_tx_queue_stats {
    pub restart_queue: u64,
    pub tx_busy: u64,
    pub tx_done_old: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_rx_queue_stats {
    pub non_eop_descs: u64,
    pub csum_good_cnt: u64,
    pub csum_err: u64,
    pub alloc_rx_buff_failed: u64,
    pub rsc_count: u64,
    pub rsc_flush: u64,
}

// iterator for handling rings in ring container

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_ring_state {
    WX_TX_DETECT_HANG,
    WX_HANG_CHECK_ARMED,
    WX_RING_STATE_NBITS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_ring_container {
    pub /: *mut *mut *mut wx_ring ring; / pointer to linked list of rings,
    pub /: *mut *mut unsigned int total_bytes; / total bytes processed this int,
    pub /: *mut *mut unsigned int total_packets; / total packets processed this int,
    pub /: *mut *mut u8 count; / total number of rings in vector,
    pub /: *mut *mut u8 itr; / current ITR setting for ring,
    pub /: *mut *mut dim dim; / data for net_dim algorithm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_ring {
    pub /: *mut *mut *mut wx_ring next; / pointer to next ring in q_vector,
    pub /: *mut *mut *mut wx_q_vector q_vector; / backpointer to host q_vector,
    pub /: *mut *mut *mut net_device netdev; / netdev ring belongs to,
    pub /: *mut *mut *mut device dev; / device for DMA mapping,
    pub page_pool: *mut page_pool,
    pub /: *mut *mut *mut void desc; / descriptor ring memory,
    pub tx_buffer_info: *mut wx_tx_buffer,
    pub rx_buffer_info: *mut wx_rx_buffer,
}

// the hardware register offset
// associated with this ring, which is
// different for DCB and RSS modes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_q_vector {
    pub wx: *mut wx,
    pub /: *mut *mut int cpu; / CPU for DCA,
    pub numa_node: c_int,
    pub for: *mut *mut u16 v_idx; / index of q_vector within array, also used,
// finding the bit in EICR and friends that
// represents the vector for this ring
//
    pub /: *mut *mut u16 itr; / Interrupt throttle rate written to EITR,
    pub tx: wx_ring_container rx,,
    pub napi: napi_struct,
    pub /: *mut *mut rcu_head rcu; / to avoid race with update stats on free,
    pub /: *mut *mut u16 total_events; / number of interrupts processed,
    pub 17]: char name[IFNAMSIZ +,
// for dynamic allocation of rings associated with this q_vector
    pub ____cacheline_internodealigned_in_smp: wx_ring ring[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_ring_feature {
    pub /: *mut *mut u16 limit; / upper limit on feature indices,
    pub /: *mut *mut u16 indices; / current value of indices,
    pub /: *mut *mut u16 mask; / Mask used for feature to ring mapping,
    pub /: *mut *mut u16 offset; / offset to start of feature,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_ring_f_enum {
    RING_F_NONE = 0,
    RING_F_VMDQ,
    RING_F_RSS,
    RING_F_FDIR,
    RING_F_ARRAY_SIZE  /* must be last in enum set */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_isb_idx {
    WX_ISB_HEADER,
    WX_ISB_MISC,
    WX_ISB_VEC0,
    WX_ISB_VEC1,
    WX_ISB_MAX
}

// Flow Control Settings
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_fc_mode {
    wx_fc_none = 0,
    wx_fc_rx_pause,
    wx_fc_tx_pause,
    wx_fc_full
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_fc_info {
    pub /: *mut *mut u32 high_water; / Flow Ctrl High-water,
    pub /: *mut *mut u32 low_water; / Flow Ctrl Low-water,
    pub /: *mut *mut wx_fc_mode mode; / Flow Control Mode,
}

// Statistics counters collected by the MAC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_hw_stats {
    pub gprc: u64,
    pub gptc: u64,
    pub gorc: u64,
    pub gotc: u64,
    pub tpr: u64,
    pub tpt: u64,
    pub bprc: u64,
    pub bptc: u64,
    pub mprc: u64,
    pub mptc: u64,
    pub roc: u64,
    pub ruc: u64,
    pub lxonrxc: u64,
    pub lxoffrxc: u64,
    pub lxontxc: u64,
    pub lxofftxc: u64,
    pub o2bgptc: u64,
    pub b2ospc: u64,
    pub o2bspc: u64,
    pub b2ogprc: u64,
    pub rdmdrop: u64,
    pub crcerrs: u64,
    pub rlec: u64,
    pub qmprc: u64,
    pub fdirmatch: u64,
    pub fdirmiss: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_last_stats {
    pub qmprc: [u32; 128],
    pub lxoffrxc: u32,
    pub lxonrxc: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_state {
    WX_STATE_DOWN,
    WX_STATE_RESETTING,
    WX_STATE_SWFW_BUSY,
    WX_STATE_PTP_RUNNING,
    WX_STATE_PTP_TX_IN_PROGRESS,
    WX_STATE_SERVICE_SCHED,
    WX_STATE_DISABLED,
    WX_STATE_RES_FREED,
    WX_STATE_NBITS		/* must be last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_data_storage {
    pub vfdev: *mut pci_dev,
    pub vf_mac_addr: [c_uchar; ETH_ALEN],
    pub spoofchk_enabled: bool,
    pub link_enable: bool,
    pub trusted: bool,
    pub xcast_mode: c_int,
    pub vf_api: c_uint,
    pub clear_to_send: bool,
    pub /: *mut *mut u16 pf_vlan; / When set, guest VLAN config not allowed.,
    pub pf_qos: u16,
    pub pf_set_mac: bool,
    pub vf_mc_hashes: [u16; WX_MAX_VF_MC_ENTRIES],
    pub num_vf_mc_hashes: u16,
    pub vlan_count: u16,
    pub link_state: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_macvlans {
    pub mvlist: list_head,
    pub vf: c_int,
    pub free: bool,
    pub is_macvlan: bool,
    pub vf_macvlan: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_rss_flow_map {
    pub flow_type: u8,
    pub data: u32,
    pub flag: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_pf_flags {
    WX_FLAG_MULTI_64_FUNC,
    WX_FLAG_SWFW_RING,
    WX_FLAG_VMDQ_ENABLED,
    WX_FLAG_VLAN_PROMISC,
    WX_FLAG_SRIOV_ENABLED,
    WX_FLAG_IRQ_VECTOR_SHARED,
    WX_FLAG_FDIR_CAPABLE,
    WX_FLAG_FDIR_HASH,
    WX_FLAG_FDIR_PERFECT,
    WX_FLAG_RSC_CAPABLE,
    WX_FLAG_RSC_ENABLED,
    WX_FLAG_RX_HWTSTAMP_ENABLED,
    WX_FLAG_RX_HWTSTAMP_IN_REGISTER,
    WX_FLAG_PTP_PPS_ENABLED,
    WX_FLAG_NEED_LINK_CONFIG,
    WX_FLAG_NEED_MODULE_RESET,
    WX_FLAG_NEED_UPDATE_LINK,
    WX_FLAG_NEED_DO_RESET,
    WX_FLAG_RX_MERGE_ENABLED,
    WX_FLAG_TXHEAD_WB_ENABLED,
    WX_FLAG_NEED_PCIE_RECOVERY,
    WX_PF_FLAGS_NBITS               /* must be last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx {
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub WX_STATE_NBITS): DECLARE_BITMAP(state,,
    pub WX_PF_FLAGS_NBITS): DECLARE_BITMAP(flags,,
    pub priv: *mut c_void,
    pub hw_addr: *mut u8 __iomem,
    pub /: *mut *mut *mut u8 __iomem b4_addr; / vf only,
    pub pdev: *mut pci_dev,
    pub netdev: *mut net_device,
    pub bus: wx_bus_info,
    pub mbx: wx_mbx_info,
    pub mac: wx_mac_info,
    pub mac_type: em_mac_type,
    pub media_type: wx_media_type,
    pub eeprom: wx_eeprom_info,
    pub addr_ctrl: wx_addr_filter_info,
    pub fc: wx_fc_info,
    pub mac_table: *mut wx_mac_addr,
    pub device_id: u16,
    pub vendor_id: u16,
    pub subsystem_device_id: u16,
    pub subsystem_vendor_id: u16,
    pub revision_id: u8,
    pub oem_ssid: u16,
    pub oem_svid: u16,
    pub msg_enable: u16,
    pub adapter_stopped: bool,
    pub tpid: [u16; 8],
    pub eeprom_id: [c_char; 32],
    pub driver_name: *mut c_char,
    pub reset_type: wx_reset_type,
    pub swfw_index: u8,
// PHY stuff
    pub notify_down: bool,
    pub link: c_uint,
    pub speed: c_int,
    pub duplex: c_int,
    pub phydev: *mut phy_device,
    pub phylink: *mut phylink,
    pub phylink_config: phylink_config,
    pub wol_hw_supported: bool,
    pub ncsi_enabled: bool,
    pub gpio_ctrl: bool,
    pub gpio_lock: raw_spinlock_t,
// Tx fast path data
    pub num_tx_queues: c_int,
    pub tx_itr_setting: u16,
    pub tx_work_limit: u16,
// Rx fast path data
    pub num_rx_queues: c_int,
    pub rx_itr_setting: u16,
    pub rx_work_limit: u16,
    pub adaptive_itr: bool,
    pub /: *mut *mut int num_q_vectors; / current number of q_vectors for device,
    pub /: *mut *mut int max_q_vectors; / upper limit of q_vectors for device,
    pub tx_ring_count: u32,
    pub rx_ring_count: u32,
    pub ____cacheline_aligned_in_smp: *mut *mut wx_ring tx_ring[64],
    pub rx_ring: [*mut wx_ring; 64],
    pub q_vector: [*mut wx_q_vector; 64],
    pub num_rx_pools: c_int,
    pub num_rx_queues_per_pool: c_int,
    pub queues_per_pool: c_uint,
    pub msix_q_entries: *mut msix_entry,
    pub msix_entry: *mut msix_entry,
    pub ring_feature: [wx_ring_feature; RING_F_ARRAY_SIZE],
// misc interrupt status block
    pub isb_dma: dma_addr_t,
    pub isb_mem: *mut u32,
    pub isb_tag: [u32; WX_ISB_MAX],
    pub misc_irq_domain: bool,
    pub eims_other: u32,
    pub eims_enable_mask: u32,
pub const WX_MAX_RETA_ENTRIES: c_int = 128;
pub const WX_RSS_INDIR_TBL_MAX: c_int = 64;
    pub rss_indir_tbl: [u8; WX_MAX_RETA_ENTRIES],
    pub rss_flags: u8,
    pub rss_enabled: bool,

    pub rss_key: *mut u32,
    pub wol: u32,
    pub bd_number: u16,
    pub default_up: bool,
    pub stats: wx_hw_stats,
    pub last_stats: wx_last_stats,
    pub /: *mut *mut spinlock_t hw_stats_lock; / spinlock for accessing to hw stats,
    pub tx_busy: u64,
    pub non_eop_descs: u64,
    pub restart_queue: u64,
    pub hw_csum_rx_good: u64,
    pub hw_csum_rx_error: u64,
    pub alloc_rx_buff_failed: u64,
    pub rsc_count: u64,
    pub rsc_flush: u64,
    pub num_vfs: c_uint,
    pub vfinfo: *mut vf_data_storage,
    pub vf_mvs: vf_macvlans,
    pub mv_list: *mut vf_macvlans,
    pub fwd_bitmask: c_ulong,
    pub atr_sample_rate: u32,
    pub ptype): *mut *mut *mut *mut void (atr)(struct wx_ring ring, struct wx_tx_buffer first, u8,
    pub wx): *mut *mut void (configure_fdir)(struct wx,
    pub tc): *mut *mut *mut int (setup_tc)(struct net_device netdev, u8,
    pub reinit): *mut *mut *mut void (do_reset)(struct net_device netdev, bool,
    pub wx): *mut *mut void (down_suspend)(struct wx,
    pub wx): *mut *mut int (ptp_setup_sdp)(struct wx,
    pub wx): *mut *mut void (set_num_queues)(struct wx,
    pub pps_enabled: bool,
    pub pps_width: u64,
    pub pps_edge_start: u64,
    pub pps_edge_end: u64,
    pub sec_to_cc: u64,
    pub base_incval: u32,
    pub tx_hwtstamp_pkts: u32,
    pub tx_hwtstamp_timeouts: u32,
    pub tx_hwtstamp_skipped: u32,
    pub tx_hwtstamp_errors: u32,
    pub rx_hwtstamp_cleared: u32,
    pub last_overflow_check: c_ulong,
    pub last_rx_ptp_check: c_ulong,
    pub ptp_tx_start: c_ulong,
    pub /: *mut *mut seqlock_t hw_tc_lock; / seqlock for ptp,
    pub hw_cc: cyclecounter,
    pub hw_tc: timecounter,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_caps: ptp_clock_info,
    pub tstamp_config: kernel_hwtstamp_config,
    pub ptp_tx_skb: *mut sk_buff,
    pub service_timer: timer_list,
    pub service_task: work_struct,
    pub reset_task: work_struct,
    pub reset_wq: *mut workqueue_struct,
    pub /: *mut *mut mutex reset_lock; / mutex for reset,
}

// register operations

extern "C" {
    pub fn rd32(_arg: wx, _arg: reg) -> return;
}
extern "C" {
    pub fn rd32(_arg: wx, 0xB500: reg +) -> return;
}
extern "C" {
    pub fn wr32(_arg: wx, _arg: reg, _arg: value) -> return;
}
extern "C" {
    pub fn wr32(_arg: wx, 0xB500: reg +, _arg: value) -> return;
}
// last = val;
// On some domestic CPU platforms, sometimes IO is not synchronized with
// flushing memory, here use readl() to flush PCI read and write.
//

extern "C" {
    pub fn container_of(_arg: config, wx: struct, _arg: phylink_config) -> return;
}

