//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/b53/b53_regs.h
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


//
// B53 register definitions
//
// Copyright (C) 2004 Broadcom Corporation
// Copyright (C) 2011-2013 Jonas Gorski <jogo@openwrt.org>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// Management Port (SMP) Page offsets
pub const B53_CTRL_PAGE: c_uint = 0x00 /* Control */;
pub const B53_STAT_PAGE: c_uint = 0x01 /* Status */;
pub const B53_MGMT_PAGE: c_uint = 0x02 /* Management Mode */;
pub const B53_MIB_AC_PAGE: c_uint = 0x03 /* MIB Autocast */;
pub const B53_ARLCTRL_PAGE: c_uint = 0x04 /* ARL Control */;
pub const B53_ARLIO_PAGE: c_uint = 0x05 /* ARL Access */;
pub const B53_FRAMEBUF_PAGE: c_uint = 0x06 /* Management frame access */;
pub const B53_MEM_ACCESS_PAGE: c_uint = 0x08 /* Memory access */;
pub const B53_IEEE_PAGE: c_uint = 0x0a /* IEEE 802.1X */;
// PHY Registers

pub const B53_IM_PORT_PAGE: c_uint = 0x18 /* Inverse MII Port (to EMAC) */;
pub const B53_ALL_PORT_PAGE: c_uint = 0x19 /* All ports MII (broadcast) */;
// MIB registers

// Quality of Service (QoS) Registers
pub const B53_QOS_PAGE: c_uint = 0x30;
// Port VLAN Page
pub const B53_PVLAN_PAGE: c_uint = 0x31;
// VLAN Registers
pub const B53_VLAN_PAGE: c_uint = 0x34;
// Jumbo Frame Registers
pub const B53_JUMBO_PAGE: c_uint = 0x40;
// EAP Registers
pub const B53_EAP_PAGE: c_uint = 0x42;
// EEE Control Registers Page
pub const B53_EEE_PAGE: c_uint = 0x92;
// CFP Configuration Registers Page
pub const B53_CFP_PAGE: c_uint = 0xa1;
//
// Control Page registers
//
// Port Control Register (8 bit)

pub const PORT_CTRL_STP_STATE_S: c_int = 5;

// SMP Control Register (8 bit)
pub const B53_SMP_CTRL: c_uint = 0x0a;
// Switch Mode Control Register (8 bit)
pub const B53_SWITCH_MODE: c_uint = 0x0b;

// IMP Port state override register (8 bit)
pub const B53_PORT_OVERRIDE_CTRL: c_uint = 0x0e;

pub const PORT_OVERRIDE_SPEED_S: c_int = 2;

// Power-down mode control (8 bit)
pub const B53_PD_MODE_CTRL_25: c_uint = 0x0f;
pub const PD_MODE_PORT_MASK: c_uint = 0x1f;
// Bit 0 also powers down the switch.

// IP Multicast control (8 bit)
pub const B53_IP_MULTICAST_CTRL: c_uint = 0x21;

// Switch control (8 bit)
pub const B53_SWITCH_CTRL: c_uint = 0x22;

// Protected Port Selection (16 bit)
pub const B53_PROTECTED_PORT_SEL: c_uint = 0x24;
pub const B53_PROTECTED_PORT_SEL_25: c_uint = 0x26;
// (16 bit)
pub const B53_UC_FLOOD_MASK: c_uint = 0x32;
pub const B53_MC_FLOOD_MASK: c_uint = 0x34;
pub const B53_IPMC_FLOOD_MASK: c_uint = 0x36;
pub const B53_DIS_LEARNING: c_uint = 0x3c;
//
// Override Ports 0-7 State on devices with xMII interfaces (8 bit)
//
// For port 8 still use B53_PORT_OVERRIDE_CTRL
// Please note that not all ports are available on every hardware, e.g. BCM5301X
// don't include overriding port 6, BCM63xx also have some limitations.
//

pub const GMII_PO_SPEED_S: c_int = 2;

pub const B53_RGMII_CTRL_IMP: c_uint = 0x60;

// Software reset register (8 bit)
pub const B53_SOFTRESET: c_uint = 0x79;

// Fast Aging Control register (8 bit)
pub const B53_FAST_AGE_CTRL: c_uint = 0x88;

// Fast Aging Port Control register (8 bit)
pub const B53_FAST_AGE_PORT_CTRL: c_uint = 0x89;
// Fast Aging VID Control register (16 bit)
pub const B53_FAST_AGE_VID_CTRL: c_uint = 0x8a;
//
// Status Page registers
//
// Link Status Summary Register (16bit)
pub const B53_LINK_STAT: c_uint = 0x00;
// Link Status Change Register (16 bit)
pub const B53_LINK_STAT_CHANGE: c_uint = 0x02;
// Port Speed Summary Register (16 bit for FE, 32 bit for GE)
pub const B53_SPEED_STAT: c_uint = 0x04;

pub const SPEED_STAT_10M: c_int = 0;
pub const SPEED_STAT_100M: c_int = 1;
pub const SPEED_STAT_1000M: c_int = 2;
// Duplex Status Summary (16 bit)
pub const B53_DUPLEX_STAT_FE: c_uint = 0x06;
pub const B53_DUPLEX_STAT_GE: c_uint = 0x08;
pub const B53_DUPLEX_STAT_63XX: c_uint = 0x0c;
// Revision ID register for BCM5325
pub const B53_REV_ID_25: c_uint = 0x50;
// Strap Value (48 bit)
pub const B53_STRAP_VALUE: c_uint = 0x70;

//
// Management Mode Page Registers
//
// Global Management Config Register (8 bit)
pub const B53_GLOBAL_CONFIG: c_uint = 0x00;
pub const GC_RESET_MIB: c_uint = 0x01;
pub const GC_RX_BPDU_EN: c_uint = 0x02;
pub const GC_MIB_AC_HDR_EN: c_uint = 0x10;
pub const GC_MIB_AC_EN: c_uint = 0x20;
pub const GC_FRM_MGMT_PORT_M: c_uint = 0xC0;
pub const GC_FRM_MGMT_PORT_04: c_uint = 0x00;
pub const GC_FRM_MGMT_PORT_MII: c_uint = 0x80;
// Broadcom Header control register (8 bit)
pub const B53_BRCM_HDR: c_uint = 0x03;

// Aging Time control register (32 bit)
pub const B53_AGING_TIME_CONTROL: c_uint = 0x06;
pub const B53_AGING_TIME_CONTROL_63XX: c_uint = 0x08;

pub const AGE_TIME_MASK: c_uint = 0x7ffff;
pub const AGE_TIME_MAX: c_int = 1048575;
// Mirror capture control register (16 bit)
pub const B53_MIR_CAP_CTL: c_uint = 0x10;
pub const CAP_PORT_MASK: c_uint = 0xf;

// Ingress mirror control register (16 bit)
pub const B53_IG_MIR_CTL: c_uint = 0x12;
pub const MIRROR_MASK: c_uint = 0x1ff;

pub const MIRROR_FILTER_MASK: c_uint = 0x3;
pub const MIRROR_FILTER_SHIFT: c_int = 14;
pub const MIRROR_ALL: c_int = 0;
pub const MIRROR_DA: c_int = 1;
pub const MIRROR_SA: c_int = 2;
// Ingress mirror divider register (16 bit)
pub const B53_IG_MIR_DIV: c_uint = 0x14;
pub const IN_MIRROR_DIV_MASK: c_uint = 0x3ff;
// Ingress mirror MAC address register (48 bit)
pub const B53_IG_MIR_MAC: c_uint = 0x16;
// Egress mirror control register (16 bit)
pub const B53_EG_MIR_CTL: c_uint = 0x1C;
// Egress mirror divider register (16 bit)
pub const B53_EG_MIR_DIV: c_uint = 0x1E;
// Egress mirror MAC address register (48 bit)
pub const B53_EG_MIR_MAC: c_uint = 0x20;
// Device ID register (8 or 32 bit)
pub const B53_DEVICE_ID: c_uint = 0x30;
// Revision ID register (8 bit)
pub const B53_REV_ID: c_uint = 0x40;
// Broadcom header RX control (16 bit)
pub const B53_BRCM_HDR_RX_DIS: c_uint = 0x60;
// Broadcom header TX control (16 bit)
pub const B53_BRCM_HDR_TX_DIS: c_uint = 0x62;
//
// ARL Access Page Registers
//
// VLAN Table Access Register (8 bit)
pub const B53_VT_ACCESS: c_uint = 0x80;
pub const B53_VT_ACCESS_9798: c_uint = 0x60 /* for BCM5397/BCM5398 */;
pub const B53_VT_ACCESS_63XX: c_uint = 0x60 /* for BCM6328/62/68 */;
pub const VTA_CMD_WRITE: c_int = 0;
pub const VTA_CMD_READ: c_int = 1;
pub const VTA_CMD_CLEAR: c_int = 2;

// VLAN Table Index Register (16 bit)
pub const B53_VT_INDEX: c_uint = 0x81;
pub const B53_VT_INDEX_9798: c_uint = 0x61;
pub const B53_VT_INDEX_63XX: c_uint = 0x62;
// VLAN Table Entry Register (32 bit)
pub const B53_VT_ENTRY: c_uint = 0x83;
pub const B53_VT_ENTRY_9798: c_uint = 0x63;
pub const B53_VT_ENTRY_63XX: c_uint = 0x64;
pub const VTE_MEMBERS: c_uint = 0x1ff;
pub const VTE_UNTAG_S: c_int = 9;

//
// ARL I/O Registers
//
// ARL Table Read/Write Register (8 bit)
pub const B53_ARLTBL_RW_CTRL: c_uint = 0x00;

// MAC Address Index Register (48 bit)
pub const B53_MAC_ADDR_IDX: c_uint = 0x02;
// VLAN ID Index Register (16 bit)
pub const B53_VLAN_ID_IDX: c_uint = 0x08;
// ARL Table MAC/VID Entry N Registers (64 bit)
//
// BCM5325 and BCM5365 share most definitions below
//

pub const ARLTBL_MAC_MASK: c_uint = 0xffffffffffffULL;
pub const ARLTBL_VID_S: c_int = 48;
pub const ARLTBL_VID_MASK: c_uint = 0xfff;
pub const ARLTBL_DATA_PORT_ID_S_25: c_int = 48;

// ARL Table Data Entry N Registers (32 bit)

pub const ARLTBL_DATA_PORT_ID_MASK: c_uint = 0x1ff;

// BCM5389 ARL Table Data Entry N Register format (16 bit)

// BCM5325/BCM565 ARL Table VID Entry N Registers (8 bit)

// Maximum number of bin entries in the ARL for all switches
pub const B53_ARLTBL_MAX_BIN_ENTRIES: c_int = 4;
// ARL Search Control Register (8 bit)
pub const B53_ARL_SRCH_CTL: c_uint = 0x50;
pub const B53_ARL_SRCH_CTL_25: c_uint = 0x20;
pub const B53_ARL_SRCH_CTL_89: c_uint = 0x30;

// ARL Search Address Register (16 bit)
pub const B53_ARL_SRCH_ADDR: c_uint = 0x51;
pub const B53_ARL_SRCH_ADDR_25: c_uint = 0x22;
pub const B53_ARL_SRCH_ADDR_65: c_uint = 0x24;
pub const B53_ARL_SRCH_ADDR_89: c_uint = 0x31;
pub const B53_ARL_SRCH_ADDR_63XX: c_uint = 0x32;

// ARL Search MAC/VID Result (64 bit)
pub const B53_ARL_SRCH_RSTL_0_MACVID: c_uint = 0x60;
pub const B53_ARL_SRCH_RSLT_MACVID_89: c_uint = 0x33;
pub const B53_ARL_SRCH_RSLT_MACVID_63XX: c_uint = 0x34;
// Single register search result on 5325/5365
pub const B53_ARL_SRCH_RSTL_0_MACVID_25: c_uint = 0x24;
pub const ARL_SRCH_RSLT_PORT_ID_S_25: c_int = 48;

pub const ARL_SRCH_RSLT_VID_S_25: c_int = 53;

// BCM5325/5365 Search result extend register (8 bit)
pub const B53_ARL_SRCH_RSLT_EXT_25: c_uint = 0x2c;

// ARL Search Data Result (32 bit)
pub const B53_ARL_SRCH_RSTL_0: c_uint = 0x68;
// BCM5389 ARL Search Data Result (16 bit)
pub const B53_ARL_SRCH_RSLT_89: c_uint = 0x3b;

// 63XX ARL Search Data Result (16 bit)
pub const B53_ARL_SRCH_RSLT_63XX: c_uint = 0x3c;

//
// IEEE 802.1X Registers
//
// Multicast DLF Drop Control register (16 bit)
pub const B53_IEEE_MCAST_DLF: c_uint = 0x94;

// Unicast DLF Drop Control register (16 bit)
pub const B53_IEEE_UCAST_DLF: c_uint = 0x96;

//
// Port VLAN Registers
//
// Port VLAN mask (16 bit) IMP port is always 8, also on 5325 & co

// Join all VLANs register (16 bit)
pub const B53_JOIN_ALL_VLAN_EN: c_uint = 0x50;
//
// 802.1Q Page Registers
//
// Global QoS Control (8 bit)
pub const B53_QOS_GLOBAL_CTL: c_uint = 0x00;
// Enable 802.1Q for individual Ports (16 bit)
pub const B53_802_1P_EN: c_uint = 0x04;
//
// VLAN Page Registers
//
// VLAN Control 0 (8 bit)
pub const B53_VLAN_CTRL0: c_uint = 0x00;
pub const VC0_8021PF_CTRL_MASK: c_uint = 0x3;
pub const VC0_8021PF_CTRL_NONE: c_uint = 0x0;
pub const VC0_8021PF_CTRL_CHANGE_PRI: c_uint = 0x1;
pub const VC0_8021PF_CTRL_CHANGE_VID: c_uint = 0x2;
pub const VC0_8021PF_CTRL_CHANGE_BOTH: c_uint = 0x3;
pub const VC0_8021QF_CTRL_MASK: c_uint = 0xc;
pub const VC0_8021QF_CTRL_CHANGE_PRI: c_uint = 0x1;
pub const VC0_8021QF_CTRL_CHANGE_VID: c_uint = 0x2;
pub const VC0_8021QF_CTRL_CHANGE_BOTH: c_uint = 0x3;

// VLAN Control 1 (8 bit)
pub const B53_VLAN_CTRL1: c_uint = 0x01;

// VLAN Control 2 (8 bit)
pub const B53_VLAN_CTRL2: c_uint = 0x02;
// VLAN Control 3 (8 bit when BCM5325, 16 bit else)
pub const B53_VLAN_CTRL3: c_uint = 0x03;
pub const B53_VLAN_CTRL3_63XX: c_uint = 0x04;

// VLAN Control 4 (8 bit)
pub const B53_VLAN_CTRL4: c_uint = 0x05;
pub const B53_VLAN_CTRL4_25: c_uint = 0x04;
pub const B53_VLAN_CTRL4_63XX: c_uint = 0x06;
pub const VC4_ING_VID_CHECK_S: c_int = 6;

// VLAN Control 5 (8 bit)
pub const B53_VLAN_CTRL5: c_uint = 0x06;
pub const B53_VLAN_CTRL5_25: c_uint = 0x05;
pub const B53_VLAN_CTRL5_63XX: c_uint = 0x07;

// VLAN Control 6 (8 bit)
pub const B53_VLAN_CTRL6: c_uint = 0x07;
pub const B53_VLAN_CTRL6_63XX: c_uint = 0x08;
// VLAN Table Access Register (16 bit)
pub const B53_VLAN_TABLE_ACCESS_25: c_uint = 0x06	/* BCM5325E/5350 */;
pub const B53_VLAN_TABLE_ACCESS_65: c_uint = 0x08	/* BCM5365 */;
pub const VTA_VID_LOW_MASK_25: c_uint = 0xf;
pub const VTA_VID_LOW_MASK_65: c_uint = 0xff;
pub const VTA_VID_HIGH_S_25: c_int = 4;
pub const VTA_VID_HIGH_S_65: c_int = 8;

pub const VTA_RW_STATE_RD: c_int = 0;

// VLAN Read/Write Registers for (16/32 bit)
pub const B53_VLAN_WRITE_25: c_uint = 0x08;
pub const B53_VLAN_WRITE_65: c_uint = 0x0a;
pub const B53_VLAN_READ: c_uint = 0x0c;
pub const VA_MEMBER_MASK: c_uint = 0x3f;
pub const VA_UNTAG_S_25: c_int = 6;
pub const VA_UNTAG_MASK_25: c_uint = 0x3f;
pub const VA_UNTAG_S_65: c_int = 7;
pub const VA_UNTAG_MASK_65: c_uint = 0x1f;
pub const VA_VID_HIGH_S: c_int = 12;

// VLAN Port Default Tag (16 bit)

//
// Jumbo Frame Page Registers
//
// Jumbo Enable Port Mask (bit i == port i enabled) (32 bit)
pub const B53_JUMBO_PORT_MASK: c_uint = 0x01;
pub const B53_JUMBO_PORT_MASK_63XX: c_uint = 0x04;

// Good Frame Max Size without 802.1Q TAG (16 bit)
pub const B53_JUMBO_MAX_SIZE: c_uint = 0x05;
pub const B53_JUMBO_MAX_SIZE_63XX: c_uint = 0x08;
pub const JMS_MIN_SIZE: c_int = 1518;
pub const JMS_MAX_SIZE: c_int = 9724;
//
// EAP Page Registers
//

pub const EAP_MODE_SHIFT: c_int = 51;
pub const EAP_MODE_SHIFT_63XX: c_int = 50;

pub const EAP_MODE_BASIC: c_int = 0;
pub const EAP_MODE_SIMPLIFIED: c_int = 3;
//
// EEE Configuration Page Registers
//
// EEE Enable control register (16 bit)
pub const B53_EEE_EN_CTRL: c_uint = 0x00;
// EEE LPI assert status register (16 bit)
pub const B53_EEE_LPI_ASSERT_STS: c_uint = 0x02;
// EEE LPI indicate status register (16 bit)
pub const B53_EEE_LPI_INDICATE: c_uint = 0x4;
// EEE Receiving idle symbols status register (16 bit)
pub const B53_EEE_RX_IDLE_SYM_STS: c_uint = 0x6;
// EEE Pipeline timer register (32 bit)
pub const B53_EEE_PIP_TIMER: c_uint = 0xC;
// EEE Sleep timer Gig register (32 bit)

// EEE Sleep timer FE register (32 bit)

// EEE Minimum LP timer Gig register (32 bit)

// EEE Minimum LP timer FE register (32 bit)

// EEE Wake timer Gig register (16 bit)

// EEE Wake timer FE register (16 bit)

//
// CFP Configuration Page Registers
//
// CFP Control Register with ports map (8 bit)
pub const B53_CFP_CTRL: c_uint = 0x00;
