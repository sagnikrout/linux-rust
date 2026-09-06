//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/wangxun/txgbe/txgbe_type.h
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

// Device IDs
pub const TXGBE_DEV_ID_SP1000: c_uint = 0x1001;
pub const TXGBE_DEV_ID_WX1820: c_uint = 0x2001;
pub const TXGBE_DEV_ID_AML5010: c_uint = 0x5010;
pub const TXGBE_DEV_ID_AML5110: c_uint = 0x5110;
pub const TXGBE_DEV_ID_AML5025: c_uint = 0x5025;
pub const TXGBE_DEV_ID_AML5125: c_uint = 0x5125;
pub const TXGBE_DEV_ID_AML5040: c_uint = 0x5040;
pub const TXGBE_DEV_ID_AML5140: c_uint = 0x5140;
// Subsystem IDs
// SFP
pub const TXGBE_ID_SP1000_SFP: c_uint = 0x0000;
pub const TXGBE_ID_WX1820_SFP: c_uint = 0x2000;
pub const TXGBE_ID_SFP: c_uint = 0x00;
// copper
pub const TXGBE_ID_SP1000_XAUI: c_uint = 0x1010;
pub const TXGBE_ID_WX1820_XAUI: c_uint = 0x2010;
pub const TXGBE_ID_XAUI: c_uint = 0x10;
pub const TXGBE_ID_SP1000_SGMII: c_uint = 0x1020;
pub const TXGBE_ID_WX1820_SGMII: c_uint = 0x2020;
pub const TXGBE_ID_SGMII: c_uint = 0x20;
// backplane
pub const TXGBE_ID_SP1000_KR_KX_KX4: c_uint = 0x1030;
pub const TXGBE_ID_WX1820_KR_KX_KX4: c_uint = 0x2030;
pub const TXGBE_ID_KR_KX_KX4: c_uint = 0x30;
// MAC Interface
pub const TXGBE_ID_SP1000_MAC_XAUI: c_uint = 0x1040;
pub const TXGBE_ID_WX1820_MAC_XAUI: c_uint = 0x2040;
pub const TXGBE_ID_MAC_XAUI: c_uint = 0x40;
pub const TXGBE_ID_SP1000_MAC_SGMII: c_uint = 0x1060;
pub const TXGBE_ID_WX1820_MAC_SGMII: c_uint = 0x2060;
pub const TXGBE_ID_MAC_SGMII: c_uint = 0x60;
// Combined interface
pub const TXGBE_ID_SFI_XAUI: c_uint = 0x50;
// Revision ID
pub const TXGBE_SP_MPW: c_int = 1;
// SP Registers
// chip control Registers
pub const TXGBE_MIS_RST: c_uint = 0x1000C;

pub const TXGBE_MIS_PRB_CTL: c_uint = 0x10010;

// FMGR Registers
pub const TXGBE_SPI_ILDR_STATUS: c_uint = 0x10120;

// Sensors for PVT(Process Voltage Temperature)
pub const TXGBE_TS_CTL: c_uint = 0x10300;

// MAC Misc Registers
pub const TXGBE_MAC_MISC_CTL: c_uint = 0x11F00;

// GPIO register bit

// Extended Interrupt Enable Set

// Port cfg registers
pub const TXGBE_CFG_PORT_ST: c_uint = 0x14404;

pub const TXGBE_CFG_VXLAN: c_uint = 0x14410;
pub const TXGBE_CFG_VXLAN_GPE: c_uint = 0x14414;
pub const TXGBE_CFG_GENEVE: c_uint = 0x14418;
// I2C registers
pub const TXGBE_I2C_BASE: c_uint = 0x14900;
// ETH PHY
pub const TXGBE_XPCS_IDA_ADDR: c_uint = 0x13000;
pub const TXGBE_XPCS_IDA_DATA: c_uint = 0x13004;
// Flow Director
pub const TXGBE_RDB_FDIR_DROP_QUEUE: c_int = 127;
pub const TXGBE_RDB_FDIR_CTL: c_uint = 0x19500;

pub const TXGBE_RDB_FDIR_SA: c_uint = 0x19518;
pub const TXGBE_RDB_FDIR_DA: c_uint = 0x1951C;
pub const TXGBE_RDB_FDIR_PORT: c_uint = 0x19520;
pub const TXGBE_RDB_FDIR_PORT_DESTINATION_SHIFT: c_int = 16;
pub const TXGBE_RDB_FDIR_FLEX: c_uint = 0x19524;
pub const TXGBE_RDB_FDIR_FLEX_FLEX_SHIFT: c_int = 16;
pub const TXGBE_RDB_FDIR_HASH: c_uint = 0x19528;

pub const TXGBE_RDB_FDIR_CMD: c_uint = 0x1952C;

pub const TXGBE_RDB_FDIR_DA4_MSK: c_uint = 0x1953C;
pub const TXGBE_RDB_FDIR_SA4_MSK: c_uint = 0x19540;
pub const TXGBE_RDB_FDIR_TCP_MSK: c_uint = 0x19544;
pub const TXGBE_RDB_FDIR_UDP_MSK: c_uint = 0x19548;
pub const TXGBE_RDB_FDIR_SCTP_MSK: c_uint = 0x19560;
pub const TXGBE_RDB_FDIR_HKEY: c_uint = 0x19568;
pub const TXGBE_RDB_FDIR_SKEY: c_uint = 0x1956C;
pub const TXGBE_RDB_FDIR_OTHER_MSK: c_uint = 0x19570;

// Amber Lite Registers
pub const TXGBE_PX_PF_BME: c_uint = 0x4B8;
pub const TXGBE_AML_MAC_TX_CFG: c_uint = 0x11000;

pub const TXGBE_RDM_RSC_CTL: c_uint = 0x1200C;

// Checksum and EEPROM pointers
pub const TXGBE_EEPROM_LAST_WORD: c_uint = 0x800;
pub const TXGBE_EEPROM_CHECKSUM: c_uint = 0x2F;
pub const TXGBE_EEPROM_SUM: c_uint = 0xBABA;
pub const TXGBE_EEPROM_VERSION_L: c_uint = 0x1D;
pub const TXGBE_EEPROM_VERSION_H: c_uint = 0x1E;
pub const TXGBE_ISCSI_BOOT_CONFIG: c_uint = 0x07;
pub const TXGBE_EEPROM_I2C_SRART_PTR: c_uint = 0x580;
pub const TXGBE_EEPROM_I2C_END_PTR: c_uint = 0x800;
pub const TXGBE_MAX_MSIX_VECTORS: c_int = 64;
pub const TXGBE_MAX_FDIR_INDICES: c_int = 63;
pub const TXGBE_MAX_RSS_INDICES: c_int = 63;

pub const TXGBE_MAX_TXQ: c_int = 128;
pub const TXGBE_MAX_RXQ: c_int = 128;
pub const TXGBE_RAR_ENTRIES: c_int = 128;
pub const TXGBE_MC_TBL_SIZE: c_int = 128;
pub const TXGBE_VFT_TBL_SIZE: c_int = 128;
pub const TXGBE_RX_PB_SIZE: c_int = 512;

pub const TXGBE_MAX_VFS_DRV_LIMIT: c_int = 63;
pub const TXGBE_DEFAULT_ATR_SAMPLE_RATE: c_int = 20;
// Software ATR hash keys
pub const TXGBE_ATR_BUCKET_HASH_KEY: c_uint = 0x3DAD14E2;
pub const TXGBE_ATR_SIGNATURE_HASH_KEY: c_uint = 0x174D3614;
// Software ATR input stream values and masks
pub const TXGBE_ATR_HASH_MASK: c_uint = 0x7fff;
pub const TXGBE_ATR_L4TYPE_MASK: c_uint = 0x3;
pub const TXGBE_ATR_L4TYPE_UDP: c_uint = 0x1;
pub const TXGBE_ATR_L4TYPE_TCP: c_uint = 0x2;
pub const TXGBE_ATR_L4TYPE_SCTP: c_uint = 0x3;
pub const TXGBE_ATR_L4TYPE_IPV6_MASK: c_uint = 0x4;
pub const TXGBE_ATR_L4TYPE_TUNNEL_MASK: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum txgbe_atr_flow_type {
    TXGBE_ATR_FLOW_TYPE_IPV4                = 0x0,
    TXGBE_ATR_FLOW_TYPE_UDPV4               = 0x1,
    TXGBE_ATR_FLOW_TYPE_TCPV4               = 0x2,
    TXGBE_ATR_FLOW_TYPE_SCTPV4              = 0x3,
    TXGBE_ATR_FLOW_TYPE_IPV6                = 0x4,
    TXGBE_ATR_FLOW_TYPE_UDPV6               = 0x5,
    TXGBE_ATR_FLOW_TYPE_TCPV6               = 0x6,
    TXGBE_ATR_FLOW_TYPE_SCTPV6              = 0x7,
    TXGBE_ATR_FLOW_TYPE_TUNNELED_IPV4       = 0x10,
    TXGBE_ATR_FLOW_TYPE_TUNNELED_UDPV4      = 0x11,
    TXGBE_ATR_FLOW_TYPE_TUNNELED_TCPV4      = 0x12,
    TXGBE_ATR_FLOW_TYPE_TUNNELED_SCTPV4     = 0x13,
    TXGBE_ATR_FLOW_TYPE_TUNNELED_IPV6       = 0x14,
    TXGBE_ATR_FLOW_TYPE_TUNNELED_UDPV6      = 0x15,
    TXGBE_ATR_FLOW_TYPE_TUNNELED_TCPV6      = 0x16,
    TXGBE_ATR_FLOW_TYPE_TUNNELED_SCTPV6     = 0x17,
}

// Flow Director ATR input struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub union txgbe_atr_input {
// Byte layout in order, all values with MSB first:
//
// vm_pool    - 1 byte
// flow_type  - 1 byte
// vlan_id    - 2 bytes
// dst_ip     - 16 bytes
// src_ip     - 16 bytes
// src_port   - 2 bytes
// dst_port   - 2 bytes
// flex_bytes - 2 bytes
// bkt_hash   - 2 bytes
//
    pub vm_pool: u8,
    pub flow_type: u8,
    pub vlan_id: __be16,
    pub dst_ip: [__be32; 4],
    pub src_ip: [__be32; 4],
    pub src_port: __be16,
    pub dst_port: __be16,
    pub flex_bytes: __be16,
    pub bkt_hash: __be16,
    pub formatted: },
    pub dword_stream: [__be32; 11],
}

// Flow Director compressed ATR hash input struct
#[repr(C)]
#[derive(Copy, Clone)]
pub union txgbe_atr_hash_dword {
    pub vm_pool: u8,
    pub flow_type: u8,
    pub vlan_id: __be16,
    pub formatted: },
    pub ip: __be32,
    pub src: __be16,
    pub dst: __be16,
    pub port: },
    pub flex_bytes: __be16,
    pub dword: __be32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum txgbe_fdir_pballoc_type {
    TXGBE_FDIR_PBALLOC_NONE = 0,
    TXGBE_FDIR_PBALLOC_64K  = 1,
    TXGBE_FDIR_PBALLOC_128K = 2,
    TXGBE_FDIR_PBALLOC_256K = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txgbe_fdir_filter {
    pub fdir_node: hlist_node,
    pub filter: txgbe_atr_input,
    pub sw_idx: u16,
    pub action: u64,
}

// TX/RX descriptor defines
pub const TXGBE_DEFAULT_TXD: c_int = 512;
pub const TXGBE_DEFAULT_TX_WORK: c_int = 256;

pub const TXGBE_DEFAULT_RXD: c_int = 512;
pub const TXGBE_DEFAULT_RX_WORK: c_int = 256;

pub const TXGBE_DEFAULT_RXD: c_int = 256;
pub const TXGBE_DEFAULT_RX_WORK: c_int = 128;

extern "C" {
    pub fn txgbe_down(wx: *mut wx);
}
extern "C" {
    pub fn txgbe_up(wx: *mut wx);
}
extern "C" {
    pub fn txgbe_setup_tc(dev: *mut net_device, tc: u8) -> c_int;
}
extern "C" {
    pub fn txgbe_do_reset(netdev: *mut net_device, reinit: bool);
}

pub const TXGBE_LINK_SPEED_UNKNOWN: c_int = 0;
pub const TXGBE_LINK_SPEED_10GB_FULL: c_int = 4;
pub const TXGBE_LINK_SPEED_25GB_FULL: c_uint = 0x10;
pub const TXGBE_LINK_SPEED_40GB_FULL: c_uint = 0x20;
pub const TXGBE_SFF_IDENTIFIER_SFP: c_uint = 0x3;
pub const TXGBE_SFF_IDENTIFIER_QSFP: c_uint = 0xC;
pub const TXGBE_SFF_IDENTIFIER_QSFP_PLUS: c_uint = 0xD;
pub const TXGBE_SFF_IDENTIFIER_QSFP28: c_uint = 0x11;
pub const TXGBE_SFF_DA_PASSIVE_CABLE: c_uint = 0x4;
pub const TXGBE_SFF_DA_ACTIVE_CABLE: c_uint = 0x8;
pub const TXGBE_SFF_DA_SPEC_ACTIVE_LIMIT: c_uint = 0x4;
pub const TXGBE_SFF_FCPI4_LIMITING: c_uint = 0x3;
pub const TXGBE_SFF_10GBASESR_CAPABLE: c_uint = 0x10;
pub const TXGBE_SFF_10GBASELR_CAPABLE: c_uint = 0x20;
pub const TXGBE_SFF_25GBASESR_CAPABLE: c_uint = 0x2;
pub const TXGBE_SFF_25GBASELR_CAPABLE: c_uint = 0x3;
pub const TXGBE_SFF_25GBASEER_CAPABLE: c_uint = 0x4;
pub const TXGBE_SFF_25GBASECR_91FEC: c_uint = 0xB;
pub const TXGBE_SFF_25GBASECR_74FEC: c_uint = 0xC;
pub const TXGBE_SFF_25GBASECR_NOFEC: c_uint = 0xD;

pub const TXGBE_SFF_ETHERNET_100G_CR4: c_uint = 0xB;

pub const FW_PHY_GET_LINK_CMD: c_uint = 0xC0;
pub const FW_PHY_SET_LINK_CMD: c_uint = 0xC1;
pub const FW_GET_MODULE_INFO_CMD: c_uint = 0xC5;
pub const FW_READ_EEPROM_CMD: c_uint = 0xC6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txgbe_sff_id {
    pub /: *mut *mut u8 identifier; / A0H 0x00,
    pub /: *mut *mut u8 com_1g_code; / A0H 0x06,
    pub /: *mut *mut u8 com_10g_code; / A0H 0x03,
    pub /: *mut *mut u8 com_25g_code; / A0H 0x24,
    pub /: *mut *mut u8 cable_spec; / A0H 0x3C,
    pub /: *mut *mut u8 cable_tech; / A0H 0x08,
    pub /: *mut *mut u8 vendor_oui0; / A0H 0x25,
    pub /: *mut *mut u8 vendor_oui1; / A0H 0x26,
    pub /: *mut *mut u8 vendor_oui2; / A0H 0x27,
    pub /: *mut *mut u8 transceiver_type; / A0H 0x83,
    pub /: *mut *mut u8 sff_opt1; / A0H 0xC0,
    pub reserved: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txgbe_hic_get_module_info {
    pub hdr: wx_hic_hdr,
    pub id: txgbe_sff_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txgbe_hic_ephy_setlink {
    pub hdr: wx_hic_hdr,
    pub speed: u8,
    pub duplex: u8,
    pub autoneg: u8,
    pub fec_mode: u8,
    pub resv: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txgbe_hic_ephy_getlink {
    pub hdr: wx_hic_hdr,
    pub speed: u8,
    pub duplex: u8,
    pub autoneg: u8,
    pub flow_ctl: u8,
    pub power: u8,
    pub fec_mode: u8,
    pub resv: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txgbe_hic_i2c_read {
    pub hdr: wx_hic_hdr,
    pub offset: __be32,
    pub length: __be32,
    pub page: u8,
    pub bank: u8,
    pub i2c_address: u8,
    pub resv: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum txgbe_swnodes {
    SWNODE_GPIO = 0,
    SWNODE_I2C,
    SWNODE_SFP,
    SWNODE_PHYLINK,
    SWNODE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txgbe_nodes {
    pub gpio_name: [c_char; 32],
    pub i2c_name: [c_char; 32],
    pub sfp_name: [c_char; 32],
    pub phylink_name: [c_char; 32],
    pub gpio_props: [property_entry; 2],
    pub i2c_props: [property_entry; 4],
    pub sfp_props: [property_entry; 9],
    pub phylink_props: [property_entry; 3],
    pub i2c_ref: [software_node_ref_args; 1],
    pub gpio0_ref: [software_node_ref_args; 1],
    pub gpio1_ref: [software_node_ref_args; 1],
    pub gpio2_ref: [software_node_ref_args; 1],
    pub gpio3_ref: [software_node_ref_args; 1],
    pub gpio4_ref: [software_node_ref_args; 1],
    pub gpio5_ref: [software_node_ref_args; 1],
    pub sfp_ref: [software_node_ref_args; 1],
    pub swnodes: [software_node; SWNODE_MAX],
    pub 1]: *const *const software_node group[SWNODE_MAX +,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum txgbe_misc_irqs {
    TXGBE_IRQ_LINK = 0,
    TXGBE_IRQ_GPIO,
    TXGBE_IRQ_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txgbe_irq {
    pub chip: irq_chip,
    pub domain: *mut irq_domain,
    pub nirqs: c_int,
    pub irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txgbe {
    pub wx: *mut wx,
    pub nodes: txgbe_nodes,
    pub misc: txgbe_irq,
    pub pcs: *mut phylink_pcs,
    pub sfp_dev: *mut platform_device,
    pub i2c_dev: *mut platform_device,
    pub clock: *mut clk_lookup,
    pub clk: *mut clk,
    pub gpio: *mut gpio_chip,
    pub link_irq: c_uint,
    pub gpio_irq: c_uint,
    pub eicr: u32,
// flow director
    pub fdir_filter_list: hlist_head,
    pub fdir_mask: txgbe_atr_input,
    pub fdir_filter_count: c_int,
    pub /: *mut *mut spinlock_t fdir_perfect_lock; / spinlock for FDIR,
    pub link_port: u8,
}
