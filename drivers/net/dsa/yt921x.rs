//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/yt921x.h
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
//
// Copyright (c) 2025 David Yang
//

pub const YT921X_SMI_ADDR: c_int = 0;

pub const YT921X_SMI_WRITE: c_int = 0;

pub const YT921X_SWITCHID_NUM: c_int = 4;
pub const YT921X_RST: c_uint = 0x80000;

pub const YT921X_FUNC: c_uint = 0x80004;

pub const YT921X_CHIP_ID: c_uint = 0x80008;

pub const YT921X_EXT_CPU_PORT: c_uint = 0x8000c;

pub const YT921X_CPU_TAG_TPID: c_uint = 0x80010;

// Same as ETH_P_YT921X, but this represents the true HW default, while the
// former is a local convention chosen by us.
//
pub const YT921X_CPU_TAG_TPID_TPID_DEFAULT: c_uint = 0x9988;
pub const YT921X_PVID_SEL: c_uint = 0x80014;

pub const YT921X_SERDES_CTRL: c_uint = 0x80028;

pub const YT921X_IO_LEVEL: c_uint = 0x80030;

pub const YT921X_MAC_ADDR_HI2: c_uint = 0x80080;
pub const YT921X_MAC_ADDR_LO4: c_uint = 0x80084;

pub const YT921X_PON_STRAP_FUNC: c_uint = 0x80320;
pub const YT921X_PON_STRAP_VAL: c_uint = 0x80324;
pub const YT921X_PON_STRAP_CAP: c_uint = 0x80328;

pub const YT921X_SENSOR: c_uint = 0x8036c;

pub const YT921X_TEMP: c_uint = 0x80374;
pub const YT921X_CHIP_MODE: c_uint = 0x80388;

pub const YT921X_XMII_CTRL: c_uint = 0x80394;

pub const YT921X_EEE_CTRL: c_uint = 0xb0000;

pub const YT921X_MIB_CTRL: c_uint = 0xc0004;

pub const YT921X_MIB_DATA_RX_BROADCAST: c_uint = 0x00;
pub const YT921X_MIB_DATA_RX_PAUSE: c_uint = 0x04;
pub const YT921X_MIB_DATA_RX_MULTICAST: c_uint = 0x08;
pub const YT921X_MIB_DATA_RX_CRC_ERR: c_uint = 0x0c;
pub const YT921X_MIB_DATA_RX_ALIGN_ERR: c_uint = 0x10;
pub const YT921X_MIB_DATA_RX_UNDERSIZE_ERR: c_uint = 0x14;
pub const YT921X_MIB_DATA_RX_FRAG_ERR: c_uint = 0x18;
pub const YT921X_MIB_DATA_RX_PKT_SZ_64: c_uint = 0x1c;
pub const YT921X_MIB_DATA_RX_PKT_SZ_65_TO_127: c_uint = 0x20;
pub const YT921X_MIB_DATA_RX_PKT_SZ_128_TO_255: c_uint = 0x24;
pub const YT921X_MIB_DATA_RX_PKT_SZ_256_TO_511: c_uint = 0x28;
pub const YT921X_MIB_DATA_RX_PKT_SZ_512_TO_1023: c_uint = 0x2c;
pub const YT921X_MIB_DATA_RX_PKT_SZ_1024_TO_1518: c_uint = 0x30;
pub const YT921X_MIB_DATA_RX_PKT_SZ_1519_TO_MAX: c_uint = 0x34;
// 0x38: unused
pub const YT921X_MIB_DATA_RX_GOOD_BYTES: c_uint = 0x3c;
// 0x40: 64 bytes
pub const YT921X_MIB_DATA_RX_BAD_BYTES: c_uint = 0x44;
// 0x48: 64 bytes
pub const YT921X_MIB_DATA_RX_OVERSIZE_ERR: c_uint = 0x4c;
pub const YT921X_MIB_DATA_RX_DROPPED: c_uint = 0x50;
pub const YT921X_MIB_DATA_TX_BROADCAST: c_uint = 0x54;
pub const YT921X_MIB_DATA_TX_PAUSE: c_uint = 0x58;
pub const YT921X_MIB_DATA_TX_MULTICAST: c_uint = 0x5c;
pub const YT921X_MIB_DATA_TX_UNDERSIZE_ERR: c_uint = 0x60;
pub const YT921X_MIB_DATA_TX_PKT_SZ_64: c_uint = 0x64;
pub const YT921X_MIB_DATA_TX_PKT_SZ_65_TO_127: c_uint = 0x68;
pub const YT921X_MIB_DATA_TX_PKT_SZ_128_TO_255: c_uint = 0x6c;
pub const YT921X_MIB_DATA_TX_PKT_SZ_256_TO_511: c_uint = 0x70;
pub const YT921X_MIB_DATA_TX_PKT_SZ_512_TO_1023: c_uint = 0x74;
pub const YT921X_MIB_DATA_TX_PKT_SZ_1024_TO_1518: c_uint = 0x78;
pub const YT921X_MIB_DATA_TX_PKT_SZ_1519_TO_MAX: c_uint = 0x7c;
// 0x80: unused
pub const YT921X_MIB_DATA_TX_GOOD_BYTES: c_uint = 0x84;
// 0x88: 64 bytes
pub const YT921X_MIB_DATA_TX_COLLISION: c_uint = 0x8c;
pub const YT921X_MIB_DATA_TX_EXCESSIVE_COLLISION: c_uint = 0x90;
pub const YT921X_MIB_DATA_TX_MULTIPLE_COLLISION: c_uint = 0x94;
pub const YT921X_MIB_DATA_TX_SINGLE_COLLISION: c_uint = 0x98;
pub const YT921X_MIB_DATA_TX_PKT: c_uint = 0x9c;
pub const YT921X_MIB_DATA_TX_DEFERRED: c_uint = 0xa0;
pub const YT921X_MIB_DATA_TX_LATE_COLLISION: c_uint = 0xa4;
pub const YT921X_MIB_DATA_RX_OAM: c_uint = 0xa8;
pub const YT921X_MIB_DATA_TX_OAM: c_uint = 0xac;
pub const YT921X_EDATA_CTRL: c_uint = 0xe0000;

pub const YT921X_EDATA_DATA: c_uint = 0xe0004;

pub const YT921X_SYS_CLK: c_uint = 0xe0040;

pub const YT9215_SYS_CLK_125M: c_int = 0;
pub const YT9218_SYS_CLK_167M: c_int = 0;
pub const YT921X_SYS_CLK_143M: c_int = 1;
pub const YT921X_EXT_MBUS_OP: c_uint = 0x6a000;
pub const YT921X_INT_MBUS_OP: c_uint = 0xf0000;

pub const YT921X_EXT_MBUS_CTRL: c_uint = 0x6a004;
pub const YT921X_INT_MBUS_CTRL: c_uint = 0xf0004;

pub const YT921X_EXT_MBUS_DOUT: c_uint = 0x6a008;
pub const YT921X_INT_MBUS_DOUT: c_uint = 0xf0008;
pub const YT921X_EXT_MBUS_DIN: c_uint = 0x6a00c;
pub const YT921X_INT_MBUS_DIN: c_uint = 0xf000c;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yt921x_app_selector {
    YT921X_APP_SEL_MAC_SA,
    YT921X_APP_SEL_MAC_DA,
    YT921X_APP_SEL_VID,
    YT921X_APP_SEL_ACL,
    YT921X_APP_SEL_DSCP,
    YT921X_APP_SEL_CVLAN_PCP,
    YT921X_APP_SEL_SVLAN_PCP,
// The physical port, i.e. YT921X_PORT_QOS_PRIO
    YT921X_APP_SEL_PORT,
    YT921X_APP_SEL_NUM
}

pub const YT921X_VLAN_IGR_FILTER: c_uint = 0x180280;

pub const YT921X_AGEING: c_uint = 0x180440;

pub const YT921X_FDB_IN0: c_uint = 0x180454;
pub const YT921X_FDB_IN1: c_uint = 0x180458;
pub const YT921X_FDB_IN2: c_uint = 0x18045c;
pub const YT921X_FDB_OP: c_uint = 0x180460;

pub const YT921X_FDB_RESULT: c_uint = 0x180464;

pub const YT921X_FDB_OUT0: c_uint = 0x1804b0;

pub const YT921X_FDB_OUT1: c_uint = 0x1804b4;

pub const YT921X_FDB_OUT2: c_uint = 0x1804b8;

pub const YT921X_FILTER_UNK_UCAST: c_uint = 0x180508;
pub const YT921X_FILTER_UNK_MCAST: c_uint = 0x18050c;
pub const YT921X_FILTER_MCAST: c_uint = 0x180510;
pub const YT921X_FILTER_BCAST: c_uint = 0x180514;

pub const YT921X_VLAN_EGR_FILTER: c_uint = 0x180598;

pub const YT921X_CPU_COPY: c_uint = 0x180690;

pub const YT921X_ACL_PERMIT_UNMATCH: c_uint = 0x1806a0;

pub const YT921X_ACT_UNK_UCAST: c_uint = 0x180734;
pub const YT921X_ACT_UNK_MCAST: c_uint = 0x180738;

// NEVER use this action; see comments in the tag driver

pub const YT921X_FDB_HW_FLUSH: c_uint = 0x180958;

pub const YT921X_ACL_BLK_KEEP: c_uint = 0x201000;

pub const YT921X_ACL_PORT: c_uint = 0x202000;

pub const YT921X_ACL_BLK_CMD: c_uint = 0x202004;

// KEY_* fields need no masks

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yt921x_acl_type {
    YT921X_ACL_TYPE_NA,
    YT921X_ACL_TYPE_MAC_DA0,
    YT921X_ACL_TYPE_MAC_SA0,
    YT921X_ACL_TYPE_MAC_DA1_SA1,
    YT921X_ACL_TYPE_VLAN,
    YT921X_ACL_TYPE_VTAG,
    YT921X_ACL_TYPE_IPV4_DA,
    YT921X_ACL_TYPE_IPV4_SA,
    YT921X_ACL_TYPE_IPV6_DA0,
    YT921X_ACL_TYPE_IPV6_DA1,
    YT921X_ACL_TYPE_IPV6_DA2,
    YT921X_ACL_TYPE_IPV6_DA3,
    YT921X_ACL_TYPE_IPV6_SA0,
    YT921X_ACL_TYPE_IPV6_SA1,
    YT921X_ACL_TYPE_IPV6_SA2,
    YT921X_ACL_TYPE_IPV6_SA3,
    YT921X_ACL_TYPE_MISC,
    YT921X_ACL_TYPE_L4,
    YT921X_ACL_TYPE_UDF0,
    YT921X_ACL_TYPE_UDF1,
    YT921X_ACL_TYPE_UDF2,
    YT921X_ACL_TYPE_UDF3,
    YT921X_ACL_TYPE_UDF4,
    YT921X_ACL_TYPE_UDF5,
    YT921X_ACL_TYPE_UDF6,
    YT921X_ACL_TYPE_UDF7,
    YT921X_ACL_TYPE_ETHERTYPE,
    YT921X_ACL_TYPE_NUM
}

// Range: turn KEY:MASK into MIN:MAX

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yt921x_l2_type {
    YT921X_L2_TYPE_ETH,
    YT921X_L2_TYPE_ETHV2,
    YT921X_L2_TYPE_ETHSAP,
    YT921X_L2_TYPE_ETHSNAP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yt921x_l3_type {
    YT921X_L3_TYPE_OTHER,
    YT921X_L3_TYPE_IPV4,
    YT921X_L3_TYPE_IPV6,
    YT921X_L3_TYPE_ARP,
    YT921X_L3_TYPE_LLDP,
    YT921X_L3_TYPE_PAE,
    YT921X_L3_TYPE_ERP,
    YT921X_L3_TYPE_SLOW_PROTOCOL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yt921x_l4_type {
    YT921X_L4_TYPE_OTHER,
    YT921X_L4_TYPE_TCP,
    YT921X_L4_TYPE_UDP,
    YT921X_L4_TYPE_UDPLITE,
    YT921X_L4_TYPE_ICMP,
    YT921X_L4_TYPE_IGMP,
    YT921X_L4_TYPE_MLD,
    YT921X_L4_TYPE_ND,
}

pub const YT921X_LAG_HASH: c_uint = 0x210090;

pub const YT921X_METER_SLOT: c_uint = 0x220104;

pub const YT921X_MIRROR: c_uint = 0x300300;

pub const YT921X_PORT_SHAPE_SLOT: c_uint = 0x34000c;

pub const YT921X_EDATA_EXTMODE: c_uint = 0xfb;
pub const YT921X_EDATA_LEN: c_uint = 0x100;
pub const YT921X_FDB_NUM: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yt921x_fdb_entry_status {
    YT921X_FDB_ENTRY_STATUS_INVALID = 0,
    YT921X_FDB_ENTRY_STATUS_MIN_TIME = 1,
    YT921X_FDB_ENTRY_STATUS_MOVE_AGING_MAX_TIME = 3,
    YT921X_FDB_ENTRY_STATUS_MAX_TIME = 5,
    YT921X_FDB_ENTRY_STATUS_PENDING = 6,
    YT921X_FDB_ENTRY_STATUS_STATIC = 7,
}

pub const YT921X_MSTI_NUM: c_int = 16;

// Custom meters only, not including dedicated port meters (11)
pub const YT921X_METER_NUM: c_int = 64;
pub const YT921X_METER_SLOT_MIN: c_int = 80;

pub const YT921X_PORT_SHAPE_SLOT_MIN: c_int = 80;

pub const YT921X_LAG_NUM: c_int = 2;
pub const YT921X_LAG_PORT_NUM: c_int = 4;
pub const YT921X_PRIO_NUM: c_int = 8;
pub const YT9215_MAJOR: c_uint = 0x9002;
pub const YT9218_MAJOR: c_uint = 0x9001;
// required for a hard reset
pub const YT921X_RST_DELAY_US: c_int = 10000;
pub const YT921X_FRAME_SIZE_MAX: c_uint = 0x2400  /* 9216 */;
pub const YT921X_TAG_LEN: c_int = 8;
pub const YT921X_ACL_BLK_NUM: c_int = 48;
pub const YT921X_ACL_ENT_PER_BLK: c_int = 8;

pub const YT921X_UDF_NUM: c_int = 8;
// 8 internal + 2 external + 1 mcu
pub const YT921X_PORT_NUM: c_int = 11;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yt921x_mib {
    pub rx_broadcast: u64,
    pub rx_pause: u64,
    pub rx_multicast: u64,
    pub rx_crc_errors: u64,
    pub rx_alignment_errors: u64,
    pub rx_undersize_errors: u64,
    pub rx_fragment_errors: u64,
    pub rx_64byte: u64,
    pub rx_65_127byte: u64,
    pub rx_128_255byte: u64,
    pub rx_256_511byte: u64,
    pub rx_512_1023byte: u64,
    pub rx_1024_1518byte: u64,
    pub rx_jumbo: u64,
    pub rx_good_bytes: u64,
    pub rx_bad_bytes: u64,
    pub rx_oversize_errors: u64,
    pub rx_dropped: u64,
    pub tx_broadcast: u64,
    pub tx_pause: u64,
    pub tx_multicast: u64,
    pub tx_undersize_errors: u64,
    pub tx_64byte: u64,
    pub tx_65_127byte: u64,
    pub tx_128_255byte: u64,
    pub tx_256_511byte: u64,
    pub tx_512_1023byte: u64,
    pub tx_1024_1518byte: u64,
    pub tx_jumbo: u64,
    pub tx_good_bytes: u64,
    pub tx_collisions: u64,
    pub tx_aborted_errors: u64,
    pub tx_multiple_collisions: u64,
    pub tx_single_collisions: u64,
    pub tx_good: u64,
    pub tx_deferred: u64,
    pub tx_late_collisions: u64,
    pub rx_oam: u64,
    pub tx_oam: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yt921x_acl_entry {
    pub key: [u32; 2],
    pub mask: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yt921x_acl_rule {
    pub tag: c_ulong,
    pub type: tc_setup_type,
    pub action: [u32; 3],
    pub sw_assisted: bool,
    pub mask: u8,
    pub entries: [yt921x_acl_entry; YT921X_ACL_ENT_PER_BLK],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yt921x_acl_blk {
    pub rules: [*mut yt921x_acl_rule; YT921X_ACL_ENT_PER_BLK],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yt921x_port {
    pub index: c_uchar,
    pub hairpin: bool,
    pub isolated: bool,
    pub mib_read: delayed_work,
    pub mib: yt921x_mib,
    pub rx_frames: u64,
    pub tx_frames: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yt921x_reg_ops {
    pub valp): *mut *mut *mut int (read)(void context, u32 reg, u32,
    pub val): *mut *mut *mut int (write)(void context, u32 reg, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yt921x_priv {
    pub ds: dsa_switch,
    pub info: *const yt921x_info,
    pub meter_slot_ns: c_uint,
    pub port_shape_slot_ns: c_uint,
// cache of dsa_cpu_ports(ds)
    pub cpu_ports_mask: u16,
    pub cycle_ns: c_uchar,
// protect the access to the switch registers
    pub reg_lock: mutex,
    pub reg_ops: *const yt921x_reg_ops,
    pub reg_ctx: *mut c_void,
// mdio master bus
    pub mbus_int: *mut mii_bus,
    pub mbus_ext: *mut mii_bus,
    pub ports: [yt921x_port; YT921X_PORT_NUM],
    pub eee_ports_mask: u16,
    pub YT921X_METER_NUM): DECLARE_BITMAP(meters_map,,
    pub acl_masks: [u8; YT921X_ACL_BLK_NUM],
    pub acl_blks: [*mut yt921x_acl_blk; YT921X_ACL_BLK_NUM],
}
