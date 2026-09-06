//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/usb/aqc111.h
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
// Aquantia Corp. Aquantia AQtion USB to 5GbE Controller
// Copyright (C) 2003-2005 David Hollis <dhollis@davehollis.com>
// Copyright (C) 2005 Phil Chang <pchang23@sbcglobal.net>
// Copyright (C) 2002-2003 TiVo Inc.
// Copyright (C) 2017-2018 ASIX
// Copyright (C) 2018 Aquantia Corp.
//

pub const AQ_MCAST_FILTER_SIZE: c_int = 8;
pub const AQ_MAX_MCAST: c_int = 64;
pub const AQ_ACCESS_MAC: c_uint = 0x01;
pub const AQ_FLASH_PARAMETERS: c_uint = 0x20;
pub const AQ_PHY_POWER: c_uint = 0x31;
pub const AQ_WOL_CFG: c_uint = 0x60;
pub const AQ_PHY_OPS: c_uint = 0x61;
pub const AQ_USB_PHY_SET_TIMEOUT: c_int = 10000;
pub const AQ_USB_SET_TIMEOUT: c_int = 4000;
// Feature.

// SFR Reg.
pub const SFR_GENERAL_STATUS: c_uint = 0x03;
pub const SFR_CHIP_STATUS: c_uint = 0x05;
pub const SFR_RX_CTL: c_uint = 0x0B;
pub const SFR_RX_CTL_TXPADCRC: c_uint = 0x0400;
pub const SFR_RX_CTL_IPE: c_uint = 0x0200;
pub const SFR_RX_CTL_DROPCRCERR: c_uint = 0x0100;
pub const SFR_RX_CTL_START: c_uint = 0x0080;
pub const SFR_RX_CTL_RF_WAK: c_uint = 0x0040;
pub const SFR_RX_CTL_AP: c_uint = 0x0020;
pub const SFR_RX_CTL_AM: c_uint = 0x0010;
pub const SFR_RX_CTL_AB: c_uint = 0x0008;
pub const SFR_RX_CTL_AMALL: c_uint = 0x0002;
pub const SFR_RX_CTL_PRO: c_uint = 0x0001;
pub const SFR_RX_CTL_STOP: c_uint = 0x0000;
pub const SFR_INTER_PACKET_GAP_0: c_uint = 0x0D;
pub const SFR_NODE_ID: c_uint = 0x10;
pub const SFR_MULTI_FILTER_ARRY: c_uint = 0x16;
pub const SFR_MEDIUM_STATUS_MODE: c_uint = 0x22;
pub const SFR_MEDIUM_XGMIIMODE: c_uint = 0x0001;
pub const SFR_MEDIUM_FULL_DUPLEX: c_uint = 0x0002;
pub const SFR_MEDIUM_RXFLOW_CTRLEN: c_uint = 0x0010;
pub const SFR_MEDIUM_TXFLOW_CTRLEN: c_uint = 0x0020;
pub const SFR_MEDIUM_JUMBO_EN: c_uint = 0x0040;
pub const SFR_MEDIUM_RECEIVE_EN: c_uint = 0x0100;
pub const SFR_MONITOR_MODE: c_uint = 0x24;
pub const SFR_MONITOR_MODE_EPHYRW: c_uint = 0x01;
pub const SFR_MONITOR_MODE_RWLC: c_uint = 0x02;
pub const SFR_MONITOR_MODE_RWMP: c_uint = 0x04;
pub const SFR_MONITOR_MODE_RWWF: c_uint = 0x08;
pub const SFR_MONITOR_MODE_RW_FLAG: c_uint = 0x10;
pub const SFR_MONITOR_MODE_PMEPOL: c_uint = 0x20;
pub const SFR_MONITOR_MODE_PMETYPE: c_uint = 0x40;
pub const SFR_PHYPWR_RSTCTL: c_uint = 0x26;
pub const SFR_PHYPWR_RSTCTL_BZ: c_uint = 0x0010;
pub const SFR_PHYPWR_RSTCTL_IPRL: c_uint = 0x0020;
pub const SFR_VLAN_ID_ADDRESS: c_uint = 0x2A;
pub const SFR_VLAN_ID_CONTROL: c_uint = 0x2B;
pub const SFR_VLAN_CONTROL_WE: c_uint = 0x0001;
pub const SFR_VLAN_CONTROL_RD: c_uint = 0x0002;
pub const SFR_VLAN_CONTROL_VSO: c_uint = 0x0010;
pub const SFR_VLAN_CONTROL_VFE: c_uint = 0x0020;
pub const SFR_VLAN_ID_DATA0: c_uint = 0x2C;
pub const SFR_VLAN_ID_DATA1: c_uint = 0x2D;
pub const SFR_RX_BULKIN_QCTRL: c_uint = 0x2E;
pub const SFR_RX_BULKIN_QCTRL_TIME: c_uint = 0x01;
pub const SFR_RX_BULKIN_QCTRL_IFG: c_uint = 0x02;
pub const SFR_RX_BULKIN_QCTRL_SIZE: c_uint = 0x04;
pub const SFR_RX_BULKIN_QTIMR_LOW: c_uint = 0x2F;
pub const SFR_RX_BULKIN_QTIMR_HIGH: c_uint = 0x30;
pub const SFR_RX_BULKIN_QSIZE: c_uint = 0x31;
pub const SFR_RX_BULKIN_QIFG: c_uint = 0x32;
pub const SFR_RXCOE_CTL: c_uint = 0x34;
pub const SFR_RXCOE_IP: c_uint = 0x01;
pub const SFR_RXCOE_TCP: c_uint = 0x02;
pub const SFR_RXCOE_UDP: c_uint = 0x04;
pub const SFR_RXCOE_ICMP: c_uint = 0x08;
pub const SFR_RXCOE_IGMP: c_uint = 0x10;
pub const SFR_RXCOE_TCPV6: c_uint = 0x20;
pub const SFR_RXCOE_UDPV6: c_uint = 0x40;
pub const SFR_RXCOE_ICMV6: c_uint = 0x80;
pub const SFR_TXCOE_CTL: c_uint = 0x35;
pub const SFR_TXCOE_IP: c_uint = 0x01;
pub const SFR_TXCOE_TCP: c_uint = 0x02;
pub const SFR_TXCOE_UDP: c_uint = 0x04;
pub const SFR_TXCOE_ICMP: c_uint = 0x08;
pub const SFR_TXCOE_IGMP: c_uint = 0x10;
pub const SFR_TXCOE_TCPV6: c_uint = 0x20;
pub const SFR_TXCOE_UDPV6: c_uint = 0x40;
pub const SFR_TXCOE_ICMV6: c_uint = 0x80;
pub const SFR_BM_INT_MASK: c_uint = 0x41;
pub const SFR_BMRX_DMA_CONTROL: c_uint = 0x43;
pub const SFR_BMRX_DMA_EN: c_uint = 0x80;
pub const SFR_BMTX_DMA_CONTROL: c_uint = 0x46;
pub const SFR_PAUSE_WATERLVL_LOW: c_uint = 0x54;
pub const SFR_PAUSE_WATERLVL_HIGH: c_uint = 0x55;
pub const SFR_ARC_CTRL: c_uint = 0x9E;
pub const SFR_SWP_CTRL: c_uint = 0xB1;
pub const SFR_TX_PAUSE_RESEND_T: c_uint = 0xB2;
pub const SFR_ETH_MAC_PATH: c_uint = 0xB7;
pub const SFR_RX_PATH_READY: c_uint = 0x01;
pub const SFR_BULK_OUT_CTRL: c_uint = 0xB9;
pub const SFR_BULK_OUT_FLUSH_EN: c_uint = 0x01;
pub const SFR_BULK_OUT_EFF_EN: c_uint = 0x02;
pub const AQ_FW_VER_MAJOR: c_uint = 0xDA;
pub const AQ_FW_VER_MINOR: c_uint = 0xDB;
pub const AQ_FW_VER_REV: c_uint = 0xDC;
// PHY_OPS

pub const AQ_ADV_MASK: c_uint = 0x0F;

pub const AQ_DSH_RETRIES_SHIFT: c_uint = 0x18;
pub const AQ_DSH_RETRIES_MASK: c_uint = 0xF000000;
pub const AQ_WOL_FLAG_MP: c_uint = 0x2;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aqc111_wol_cfg {
    pub hw_addr: [u8; 6],
    pub flags: u8,
    pub rsvd: [u8; 283],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aqc111_data {
    pub rxctl: u16,
    pub rx_checksum: u8,
    pub link_speed: u8,
    pub link: u8,
    pub autoneg: u8,
    pub advertised_speed: u32,
    pub major: u8,
    pub minor: u8,
    pub rev: u8,
    pub fw_ver: },
    pub phy_cfg: u32,
    pub wol_flags: u8,
}

pub const AQ_LS_MASK: c_uint = 0x8000;
pub const AQ_SPEED_MASK: c_uint = 0x7F00;
pub const AQ_SPEED_SHIFT: c_uint = 0x0008;
pub const AQ_INT_SPEED_5G: c_uint = 0x000F;
pub const AQ_INT_SPEED_2_5G: c_uint = 0x0010;
pub const AQ_INT_SPEED_1G: c_uint = 0x0011;
pub const AQ_INT_SPEED_100M: c_uint = 0x0013;
// TX Descriptor
pub const AQ_TX_DESC_LEN_MASK: c_uint = 0x1FFFFF;

pub const AQ_TX_DESC_MSS_MASK: c_uint = 0x7FFF;
pub const AQ_TX_DESC_MSS_SHIFT: c_uint = 0x20;
pub const AQ_TX_DESC_VLAN_MASK: c_uint = 0xFFFF;
pub const AQ_TX_DESC_VLAN_SHIFT: c_uint = 0x30;
pub const AQ_RX_HW_PAD: c_uint = 0x02;
// RX Packet Descriptor

pub const AQ_RX_PD_L4_TYPE_MASK: c_uint = 0x1C;
pub const AQ_RX_PD_L4_UDP: c_uint = 0x04;
pub const AQ_RX_PD_L4_TCP: c_uint = 0x10;
pub const AQ_RX_PD_L3_TYPE_MASK: c_uint = 0x60;
pub const AQ_RX_PD_L3_IP: c_uint = 0x20;
pub const AQ_RX_PD_L3_IP6: c_uint = 0x40;

pub const AQ_RX_PD_LEN_MASK: c_uint = 0x7FFF0000;
pub const AQ_RX_PD_LEN_SHIFT: c_uint = 0x10;
pub const AQ_RX_PD_VLAN_SHIFT: c_uint = 0x20;
// RX Descriptor header
pub const AQ_RX_DH_PKT_CNT_MASK: c_uint = 0x1FFF;
pub const AQ_RX_DH_DESC_OFFSET_MASK: c_uint = 0xFFFFE000;
pub const AQ_RX_DH_DESC_OFFSET_SHIFT: c_uint = 0x0D;
// xHCI & EHCI & OHCI
// Jumbo packet
