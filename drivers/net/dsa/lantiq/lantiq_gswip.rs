//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/lantiq/lantiq_gswip.h
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

// GSWIP MDIO Registers
pub const GSWIP_MDIO_GLOB: c_uint = 0x00;

pub const GSWIP_MDIO_CTRL: c_uint = 0x08;

pub const GSWIP_MDIO_CTRL_PHYAD_MASK: c_uint = 0x1f;
pub const GSWIP_MDIO_CTRL_PHYAD_SHIFT: c_int = 5;
pub const GSWIP_MDIO_CTRL_REGAD_MASK: c_uint = 0x1f;
pub const GSWIP_MDIO_READ: c_uint = 0x09;
pub const GSWIP_MDIO_WRITE: c_uint = 0x0A;
pub const GSWIP_MDIO_MDC_CFG0: c_uint = 0x0B;
pub const GSWIP_MDIO_MDC_CFG1: c_uint = 0x0C;

pub const GSWIP_MDIO_PHY_LINK_MASK: c_uint = 0x6000;
pub const GSWIP_MDIO_PHY_LINK_AUTO: c_uint = 0x0000;
pub const GSWIP_MDIO_PHY_LINK_DOWN: c_uint = 0x4000;
pub const GSWIP_MDIO_PHY_LINK_UP: c_uint = 0x2000;
pub const GSWIP_MDIO_PHY_SPEED_MASK: c_uint = 0x1800;
pub const GSWIP_MDIO_PHY_SPEED_AUTO: c_uint = 0x1800;
pub const GSWIP_MDIO_PHY_SPEED_M10: c_uint = 0x0000;
pub const GSWIP_MDIO_PHY_SPEED_M100: c_uint = 0x0800;
pub const GSWIP_MDIO_PHY_SPEED_G1: c_uint = 0x1000;
pub const GSWIP_MDIO_PHY_FDUP_MASK: c_uint = 0x0600;
pub const GSWIP_MDIO_PHY_FDUP_AUTO: c_uint = 0x0000;
pub const GSWIP_MDIO_PHY_FDUP_EN: c_uint = 0x0200;
pub const GSWIP_MDIO_PHY_FDUP_DIS: c_uint = 0x0600;
pub const GSWIP_MDIO_PHY_FCONTX_MASK: c_uint = 0x0180;
pub const GSWIP_MDIO_PHY_FCONTX_AUTO: c_uint = 0x0000;
pub const GSWIP_MDIO_PHY_FCONTX_EN: c_uint = 0x0100;
pub const GSWIP_MDIO_PHY_FCONTX_DIS: c_uint = 0x0180;
pub const GSWIP_MDIO_PHY_FCONRX_MASK: c_uint = 0x0060;
pub const GSWIP_MDIO_PHY_FCONRX_AUTO: c_uint = 0x0000;
pub const GSWIP_MDIO_PHY_FCONRX_EN: c_uint = 0x0020;
pub const GSWIP_MDIO_PHY_FCONRX_DIS: c_uint = 0x0060;
pub const GSWIP_MDIO_PHY_ADDR_MASK: c_uint = 0x001f;

// GSWIP MII Registers

pub const GSWIP_MII_CFG_MODE_MIIP: c_uint = 0x0;
pub const GSWIP_MII_CFG_MODE_MIIM: c_uint = 0x1;
pub const GSWIP_MII_CFG_MODE_RMIIP: c_uint = 0x2;
pub const GSWIP_MII_CFG_MODE_RMIIM: c_uint = 0x3;
pub const GSWIP_MII_CFG_MODE_RGMII: c_uint = 0x4;
pub const GSWIP_MII_CFG_MODE_GMII: c_uint = 0x9;
pub const GSWIP_MII_CFG_MODE_MASK: c_uint = 0xf;
pub const GSWIP_MII_CFG_RATE_M2P5: c_uint = 0x00;
pub const GSWIP_MII_CFG_RATE_M25: c_uint = 0x10;
pub const GSWIP_MII_CFG_RATE_M125: c_uint = 0x20;
pub const GSWIP_MII_CFG_RATE_M50: c_uint = 0x30;
pub const GSWIP_MII_CFG_RATE_AUTO: c_uint = 0x40;
pub const GSWIP_MII_CFG_RATE_MASK: c_uint = 0x70;
pub const GSWIP_MII_PCDU0: c_uint = 0x01;
pub const GSWIP_MII_PCDU1: c_uint = 0x03;
pub const GSWIP_MII_PCDU5: c_uint = 0x05;

// GSWIP Core Registers
pub const GSWIP_SWRES: c_uint = 0x000;

pub const GSWIP_VERSION: c_uint = 0x013;

pub const GSWIP_VERSION_2_0: c_uint = 0x100;
pub const GSWIP_VERSION_2_1: c_uint = 0x021;
pub const GSWIP_VERSION_2_2: c_uint = 0x122;
pub const GSWIP_VERSION_2_2_ETC: c_uint = 0x022;
// The hardware has the 'major/minor' version bytes in the wrong order
// preventing numerical comparisons. Swap the bytes of the 16-bit value
// to end up with REV being the most significant byte and MOD being the
// least significant byte, which then allows comparing it with the
// value stored in struct gswip_priv.
//

pub const GSWIP_BM_RAM_ADDR: c_uint = 0x044;
pub const GSWIP_BM_RAM_CTRL: c_uint = 0x045;

pub const GSWIP_BM_QUEUE_GCTRL: c_uint = 0x04A;

// buffer management Port Configuration Register

// buffer management Port Control Register

// PCE

pub const GSWIP_PCE_TBL_MASK: c_uint = 0x448;

pub const GSWIP_PCE_TBL_ADDR: c_uint = 0x44E;
pub const GSWIP_PCE_TBL_CTRL: c_uint = 0x44F;

pub const GSWIP_PCE_TBL_CTRL_OPMOD_ADRD: c_uint = 0x00;
pub const GSWIP_PCE_TBL_CTRL_OPMOD_ADWR: c_uint = 0x20;
pub const GSWIP_PCE_TBL_CTRL_OPMOD_KSRD: c_uint = 0x40;
pub const GSWIP_PCE_TBL_CTRL_OPMOD_KSWR: c_uint = 0x60;

pub const GSWIP_PCE_PMAP1: c_uint = 0x453	/* Monitoring port map */;
pub const GSWIP_PCE_PMAP2: c_uint = 0x454	/* Default Multicast port map */;
pub const GSWIP_PCE_PMAP3: c_uint = 0x455	/* Default Unknown Unicast port map */;
pub const GSWIP_PCE_GCTRL_0: c_uint = 0x456;

pub const GSWIP_PCE_GCTRL_1: c_uint = 0x457;

pub const GSWIP_PCE_PCTRL_0_PSTATE_LISTEN: c_uint = 0x0;
pub const GSWIP_PCE_PCTRL_0_PSTATE_RX: c_uint = 0x1;
pub const GSWIP_PCE_PCTRL_0_PSTATE_TX: c_uint = 0x2;
pub const GSWIP_PCE_PCTRL_0_PSTATE_LEARNING: c_uint = 0x3;
pub const GSWIP_PCE_PCTRL_0_PSTATE_FORWARDING: c_uint = 0x7;

// Ethernet Switch PCE Port Control Register 3

pub const GSWIP_MAC_FLEN: c_uint = 0x8C5;

pub const GSWIP_MAC_CTRL_0_FCON_MASK: c_uint = 0x0070;
pub const GSWIP_MAC_CTRL_0_FCON_AUTO: c_uint = 0x0000;
pub const GSWIP_MAC_CTRL_0_FCON_RX: c_uint = 0x0010;
pub const GSWIP_MAC_CTRL_0_FCON_TX: c_uint = 0x0020;
pub const GSWIP_MAC_CTRL_0_FCON_RXTX: c_uint = 0x0030;
pub const GSWIP_MAC_CTRL_0_FCON_NONE: c_uint = 0x0040;
pub const GSWIP_MAC_CTRL_0_FDUP_MASK: c_uint = 0x000C;
pub const GSWIP_MAC_CTRL_0_FDUP_AUTO: c_uint = 0x0000;
pub const GSWIP_MAC_CTRL_0_FDUP_EN: c_uint = 0x0004;
pub const GSWIP_MAC_CTRL_0_FDUP_DIS: c_uint = 0x000C;
pub const GSWIP_MAC_CTRL_0_GMII_MASK: c_uint = 0x0003;
pub const GSWIP_MAC_CTRL_0_GMII_AUTO: c_uint = 0x0000;
pub const GSWIP_MAC_CTRL_0_GMII_MII: c_uint = 0x0001;
pub const GSWIP_MAC_CTRL_0_GMII_RGMII: c_uint = 0x0002;

// Ethernet Switch Fetch DMA Port Control Register

// Ethernet Switch Store DMA Port Control Register

pub const GSWIP_TABLE_ACTIVE_VLAN: c_uint = 0x01;
pub const GSWIP_TABLE_VLAN_MAPPING: c_uint = 0x02;
pub const GSWIP_TABLE_MAC_BRIDGE: c_uint = 0x0b;

// Maximum packet size supported by the switch. In theory this should be 10240,
// but long packets currently cause lock-ups with an MTU of over 2526. Medium
// packets are sometimes dropped (e.g. TCP over 2477, UDP over 2516-2519, ICMP
// over 2526), hence an MTU value of 2400 seems safe. This issue only affects
// packet reception. This is probably caused by the PPA engine, which is on the
// RX part of the device. Packet transmission works properly up to 10240.
//
pub const GSWIP_MAX_PACKET_LENGTH: c_int = 2400;
pub const GSWIP_VLAN_UNAWARE_PVID: c_int = 0;
pub const GSWIP_MAX_PORTS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gswip_pce_microcode {
    pub val_3: u16,
    pub val_2: u16,
    pub val_1: u16,
    pub val_0: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gswip_hw_info {
    pub max_ports: c_int,
    pub allowed_cpu_ports: c_uint,
    pub mii_cfg: [i16; GSWIP_MAX_PORTS],
    pub mii_pcdu: [i16; GSWIP_MAX_PORTS],
    pub supports_2500m: bool,
    pub (*pce_microcode)[]: *const gswip_pce_microcode,
    pub pce_microcode_size: usize,
    pub tag_protocol: dsa_tag_protocol,
    pub config): *mut phylink_config,
    pub interface): phy_interface_t,
    pub port): *mut *mut *mut int (port_setup)(struct dsa_switch ds, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gswip_gphy_fw {
    pub clk_gate: *mut clk,
    pub reset: *mut reset_control,
    pub fw_addr_offset: u32,
    pub fw_name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gswip_vlan {
    pub bridge: *mut net_device,
    pub vid: u16,
    pub fid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gswip_priv {
    pub gswip: *mut regmap,
    pub mdio: *mut regmap,
    pub mii: *mut regmap,
    pub hw_info: *const gswip_hw_info,
    pub gphy_fw_name_cfg: *const xway_gphy_match_data,
    pub ds: *mut dsa_switch,
    pub dev: *mut device,
    pub rcu_regmap: *mut regmap,
    pub vlans: [gswip_vlan; 64],
    pub num_gphy_fw: c_int,
    pub gphy_fw: *mut gswip_gphy_fw,
    pub pce_table_lock: mutex,
    pub version: u16,
}

extern "C" {
    pub fn gswip_probe_common(priv: *mut gswip_priv, version: u32) -> c_int;
}
