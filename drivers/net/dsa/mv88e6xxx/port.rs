//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/mv88e6xxx/port.h
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
// Marvell 88E6xxx Switch Port Registers support
//
// Copyright (c) 2008 Marvell Semiconductor
//
// Copyright (c) 2016-2017 Savoir-faire Linux Inc.
// Vivien Didelot <vivien.didelot@savoirfairelinux.com>
//

// Offset 0x00: Port Status Register
pub const MV88E6XXX_PORT_STS: c_uint = 0x00;
pub const MV88E6XXX_PORT_STS_PAUSE_EN: c_uint = 0x8000;
pub const MV88E6XXX_PORT_STS_MY_PAUSE: c_uint = 0x4000;
pub const MV88E6XXX_PORT_STS_HD_FLOW: c_uint = 0x2000;
pub const MV88E6XXX_PORT_STS_PHY_DETECT: c_uint = 0x1000;
pub const MV88E6250_PORT_STS_LINK: c_uint = 0x1000;
pub const MV88E6250_PORT_STS_PORTMODE_MASK: c_uint = 0x0f00;
pub const MV88E6250_PORT_STS_PORTMODE_PHY_10_HALF: c_uint = 0x0800;
pub const MV88E6250_PORT_STS_PORTMODE_PHY_100_HALF: c_uint = 0x0900;
pub const MV88E6250_PORT_STS_PORTMODE_PHY_10_FULL: c_uint = 0x0a00;
pub const MV88E6250_PORT_STS_PORTMODE_PHY_100_FULL: c_uint = 0x0b00;
// - Modes with PHY suffix use output instead of input clock
// - Modes without RMII or RGMII use MII
// - Modes without speed do not have a fixed speed specified in the manual
// ("DC to x MHz" - variable clock support?)
//
pub const MV88E6250_PORT_STS_PORTMODE_MII_DISABLED: c_uint = 0x0000;
pub const MV88E6250_PORT_STS_PORTMODE_MII_100_RGMII: c_uint = 0x0100;
pub const MV88E6250_PORT_STS_PORTMODE_MII_DUAL_100_RMII_FULL_PHY: c_uint = 0x0200;
pub const MV88E6250_PORT_STS_PORTMODE_MII_200_RMII_FULL_PHY: c_uint = 0x0400;
pub const MV88E6250_PORT_STS_PORTMODE_MII_DUAL_100_RMII_FULL: c_uint = 0x0600;
pub const MV88E6250_PORT_STS_PORTMODE_MII_10_100_RMII_FULL: c_uint = 0x0700;
pub const MV88E6250_PORT_STS_PORTMODE_MII_HALF: c_uint = 0x0800;
pub const MV88E6250_PORT_STS_PORTMODE_MII_10_100_RMII_HALF_PHY: c_uint = 0x0900;
pub const MV88E6250_PORT_STS_PORTMODE_MII_FULL: c_uint = 0x0a00;
pub const MV88E6250_PORT_STS_PORTMODE_MII_10_100_RMII_FULL_PHY: c_uint = 0x0b00;
pub const MV88E6250_PORT_STS_PORTMODE_MII_10_HALF_PHY: c_uint = 0x0c00;
pub const MV88E6250_PORT_STS_PORTMODE_MII_100_HALF_PHY: c_uint = 0x0d00;
pub const MV88E6250_PORT_STS_PORTMODE_MII_10_FULL_PHY: c_uint = 0x0e00;
pub const MV88E6250_PORT_STS_PORTMODE_MII_100_FULL_PHY: c_uint = 0x0f00;
pub const MV88E6XXX_PORT_STS_LINK: c_uint = 0x0800;
pub const MV88E6XXX_PORT_STS_DUPLEX: c_uint = 0x0400;
pub const MV88E6XXX_PORT_STS_SPEED_MASK: c_uint = 0x0300;
pub const MV88E6XXX_PORT_STS_SPEED_10: c_uint = 0x0000;
pub const MV88E6XXX_PORT_STS_SPEED_100: c_uint = 0x0100;
pub const MV88E6XXX_PORT_STS_SPEED_1000: c_uint = 0x0200;
pub const MV88E6XXX_PORT_STS_SPEED_10000: c_uint = 0x0300;
pub const MV88E6352_PORT_STS_EEE: c_uint = 0x0040;
pub const MV88E6165_PORT_STS_AM_DIS: c_uint = 0x0040;
pub const MV88E6185_PORT_STS_MGMII: c_uint = 0x0040;
pub const MV88E6XXX_PORT_STS_TX_PAUSED: c_uint = 0x0020;
pub const MV88E6XXX_PORT_STS_FLOW_CTL: c_uint = 0x0010;
pub const MV88E6XXX_PORT_STS_CMODE_MASK: c_uint = 0x000f;
pub const MV88E6XXX_PORT_STS_CMODE_MII_PHY: c_uint = 0x0001;
pub const MV88E6XXX_PORT_STS_CMODE_MII: c_uint = 0x0002;
pub const MV88E6XXX_PORT_STS_CMODE_GMII: c_uint = 0x0003;
pub const MV88E6XXX_PORT_STS_CMODE_RMII_PHY: c_uint = 0x0004;
pub const MV88E6XXX_PORT_STS_CMODE_RMII: c_uint = 0x0005;
pub const MV88E6XXX_PORT_STS_CMODE_RGMII: c_uint = 0x0007;
pub const MV88E6XXX_PORT_STS_CMODE_100BASEX: c_uint = 0x0008;
pub const MV88E6XXX_PORT_STS_CMODE_1000BASEX: c_uint = 0x0009;
pub const MV88E6XXX_PORT_STS_CMODE_SGMII: c_uint = 0x000a;
pub const MV88E6XXX_PORT_STS_CMODE_2500BASEX: c_uint = 0x000b;
pub const MV88E6XXX_PORT_STS_CMODE_XAUI: c_uint = 0x000c;
pub const MV88E6XXX_PORT_STS_CMODE_RXAUI: c_uint = 0x000d;
pub const MV88E6393X_PORT_STS_CMODE_5GBASER: c_uint = 0x000c;
pub const MV88E6393X_PORT_STS_CMODE_10GBASER: c_uint = 0x000d;
pub const MV88E6393X_PORT_STS_CMODE_USXGMII: c_uint = 0x000e;
pub const MV88E6185_PORT_STS_CDUPLEX: c_uint = 0x0008;
pub const MV88E6185_PORT_STS_CMODE_MASK: c_uint = 0x0007;
pub const MV88E6185_PORT_STS_CMODE_GMII_FD: c_uint = 0x0000;
pub const MV88E6185_PORT_STS_CMODE_MII_100_FD_PS: c_uint = 0x0001;
pub const MV88E6185_PORT_STS_CMODE_MII_100: c_uint = 0x0002;
pub const MV88E6185_PORT_STS_CMODE_MII_10: c_uint = 0x0003;
pub const MV88E6185_PORT_STS_CMODE_SERDES: c_uint = 0x0004;
pub const MV88E6185_PORT_STS_CMODE_1000BASE_X: c_uint = 0x0005;
pub const MV88E6185_PORT_STS_CMODE_PHY: c_uint = 0x0006;
pub const MV88E6185_PORT_STS_CMODE_DISABLED: c_uint = 0x0007;
// Offset 0x01: MAC (or PCS or Physical) Control Register
pub const MV88E6XXX_PORT_MAC_CTL: c_uint = 0x01;
pub const MV88E6XXX_PORT_MAC_CTL_RGMII_DELAY_RXCLK: c_uint = 0x8000;
pub const MV88E6XXX_PORT_MAC_CTL_RGMII_DELAY_TXCLK: c_uint = 0x4000;
pub const MV88E6185_PORT_MAC_CTL_SYNC_OK: c_uint = 0x4000;
pub const MV88E6390_PORT_MAC_CTL_FORCE_SPEED: c_uint = 0x2000;
pub const MV88E6390_PORT_MAC_CTL_ALTSPEED: c_uint = 0x1000;
pub const MV88E6352_PORT_MAC_CTL_200BASE: c_uint = 0x1000;
pub const MV88E6XXX_PORT_MAC_CTL_EEE: c_uint = 0x0200;
pub const MV88E6XXX_PORT_MAC_CTL_FORCE_EEE: c_uint = 0x0100;
pub const MV88E6185_PORT_MAC_CTL_AN_EN: c_uint = 0x0400;
pub const MV88E6185_PORT_MAC_CTL_AN_RESTART: c_uint = 0x0200;
pub const MV88E6185_PORT_MAC_CTL_AN_DONE: c_uint = 0x0100;
pub const MV88E6XXX_PORT_MAC_CTL_FC: c_uint = 0x0080;
pub const MV88E6XXX_PORT_MAC_CTL_FORCE_FC: c_uint = 0x0040;
pub const MV88E6XXX_PORT_MAC_CTL_LINK_UP: c_uint = 0x0020;
pub const MV88E6XXX_PORT_MAC_CTL_FORCE_LINK: c_uint = 0x0010;
pub const MV88E6XXX_PORT_MAC_CTL_DUPLEX_FULL: c_uint = 0x0008;
pub const MV88E6XXX_PORT_MAC_CTL_FORCE_DUPLEX: c_uint = 0x0004;
pub const MV88E6XXX_PORT_MAC_CTL_SPEED_MASK: c_uint = 0x0003;
pub const MV88E6XXX_PORT_MAC_CTL_SPEED_10: c_uint = 0x0000;
pub const MV88E6XXX_PORT_MAC_CTL_SPEED_100: c_uint = 0x0001;
pub const MV88E6065_PORT_MAC_CTL_SPEED_200: c_uint = 0x0002;
pub const MV88E6XXX_PORT_MAC_CTL_SPEED_1000: c_uint = 0x0002;
pub const MV88E6390_PORT_MAC_CTL_SPEED_10000: c_uint = 0x0003;
pub const MV88E6XXX_PORT_MAC_CTL_SPEED_UNFORCED: c_uint = 0x0003;
// Offset 0x02: Jamming Control Register
pub const MV88E6097_PORT_JAM_CTL: c_uint = 0x02;
pub const MV88E6097_PORT_JAM_CTL_LIMIT_OUT_MASK: c_uint = 0xff00;
pub const MV88E6097_PORT_JAM_CTL_LIMIT_IN_MASK: c_uint = 0x00ff;
// Offset 0x02: Flow Control Register
pub const MV88E6390_PORT_FLOW_CTL: c_uint = 0x02;
pub const MV88E6390_PORT_FLOW_CTL_UPDATE: c_uint = 0x8000;
pub const MV88E6390_PORT_FLOW_CTL_PTR_MASK: c_uint = 0x7f00;
pub const MV88E6390_PORT_FLOW_CTL_LIMIT_IN: c_uint = 0x0000;
pub const MV88E6390_PORT_FLOW_CTL_LIMIT_OUT: c_uint = 0x0100;
pub const MV88E6390_PORT_FLOW_CTL_DATA_MASK: c_uint = 0x00ff;
// Offset 0x03: Switch Identifier Register
pub const MV88E6XXX_PORT_SWITCH_ID: c_uint = 0x03;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_MASK: c_uint = 0xfff0;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6020: c_uint = 0x0200;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6071: c_uint = 0x0710;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6085: c_uint = 0x04a0;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6095: c_uint = 0x0950;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6097: c_uint = 0x0990;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6190X: c_uint = 0x0a00;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6390X: c_uint = 0x0a10;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6131: c_uint = 0x1060;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6320: c_uint = 0x1150;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6123: c_uint = 0x1210;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6161: c_uint = 0x1610;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6165: c_uint = 0x1650;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6171: c_uint = 0x1710;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6172: c_uint = 0x1720;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6175: c_uint = 0x1750;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6176: c_uint = 0x1760;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6190: c_uint = 0x1900;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6191: c_uint = 0x1910;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6191X: c_uint = 0x1920;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6193X: c_uint = 0x1930;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6185: c_uint = 0x1a70;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6220: c_uint = 0x2200;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6240: c_uint = 0x2400;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6250: c_uint = 0x2500;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6361: c_uint = 0x2610;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6290: c_uint = 0x2900;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6321: c_uint = 0x3100;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6141: c_uint = 0x3400;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6341: c_uint = 0x3410;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6352: c_uint = 0x3520;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6350: c_uint = 0x3710;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6351: c_uint = 0x3750;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6390: c_uint = 0x3900;
pub const MV88E6XXX_PORT_SWITCH_ID_PROD_6393X: c_uint = 0x3930;
pub const MV88E6XXX_PORT_SWITCH_ID_REV_MASK: c_uint = 0x000f;
// Offset 0x04: Port Control Register
pub const MV88E6XXX_PORT_CTL0: c_uint = 0x04;
pub const MV88E6XXX_PORT_CTL0_USE_CORE_TAG: c_uint = 0x8000;
pub const MV88E6XXX_PORT_CTL0_SA_FILT_MASK: c_uint = 0xc000;
pub const MV88E6XXX_PORT_CTL0_SA_FILT_DISABLED: c_uint = 0x0000;
pub const MV88E6XXX_PORT_CTL0_SA_FILT_DROP_ON_LOCK: c_uint = 0x4000;
pub const MV88E6XXX_PORT_CTL0_SA_FILT_DROP_ON_UNLOCK: c_uint = 0x8000;
pub const MV88E6XXX_PORT_CTL0_SA_FILT_DROP_ON_CPU: c_uint = 0xc000;
pub const MV88E6XXX_PORT_CTL0_EGRESS_MODE_MASK: c_uint = 0x3000;
pub const MV88E6XXX_PORT_CTL0_EGRESS_MODE_UNMODIFIED: c_uint = 0x0000;
pub const MV88E6XXX_PORT_CTL0_EGRESS_MODE_UNTAGGED: c_uint = 0x1000;
pub const MV88E6XXX_PORT_CTL0_EGRESS_MODE_TAGGED: c_uint = 0x2000;
pub const MV88E6XXX_PORT_CTL0_EGRESS_MODE_ETHER_TYPE_DSA: c_uint = 0x3000;
pub const MV88E6XXX_PORT_CTL0_HEADER: c_uint = 0x0800;
pub const MV88E6XXX_PORT_CTL0_IGMP_MLD_SNOOP: c_uint = 0x0400;
pub const MV88E6XXX_PORT_CTL0_DOUBLE_TAG: c_uint = 0x0200;
pub const MV88E6XXX_PORT_CTL0_FRAME_MODE_MASK: c_uint = 0x0300;
pub const MV88E6XXX_PORT_CTL0_FRAME_MODE_NORMAL: c_uint = 0x0000;
pub const MV88E6XXX_PORT_CTL0_FRAME_MODE_DSA: c_uint = 0x0100;
pub const MV88E6XXX_PORT_CTL0_FRAME_MODE_PROVIDER: c_uint = 0x0200;
pub const MV88E6XXX_PORT_CTL0_FRAME_MODE_ETHER_TYPE_DSA: c_uint = 0x0300;
pub const MV88E6XXX_PORT_CTL0_DSA_TAG: c_uint = 0x0100;
pub const MV88E6XXX_PORT_CTL0_VLAN_TUNNEL: c_uint = 0x0080;
pub const MV88E6XXX_PORT_CTL0_TAG_IF_BOTH: c_uint = 0x0040;
pub const MV88E6185_PORT_CTL0_USE_IP: c_uint = 0x0020;
pub const MV88E6185_PORT_CTL0_USE_TAG: c_uint = 0x0010;
pub const MV88E6185_PORT_CTL0_FORWARD_UNKNOWN: c_uint = 0x0004;
pub const MV88E6352_PORT_CTL0_EGRESS_FLOODS_UC: c_uint = 0x0004;
pub const MV88E6352_PORT_CTL0_EGRESS_FLOODS_MC: c_uint = 0x0008;
pub const MV88E6XXX_PORT_CTL0_STATE_MASK: c_uint = 0x0003;
pub const MV88E6XXX_PORT_CTL0_STATE_DISABLED: c_uint = 0x0000;
pub const MV88E6XXX_PORT_CTL0_STATE_BLOCKING: c_uint = 0x0001;
pub const MV88E6XXX_PORT_CTL0_STATE_LEARNING: c_uint = 0x0002;
pub const MV88E6XXX_PORT_CTL0_STATE_FORWARDING: c_uint = 0x0003;
// Offset 0x05: Port Control 1
pub const MV88E6XXX_PORT_CTL1: c_uint = 0x05;
pub const MV88E6XXX_PORT_CTL1_MESSAGE_PORT: c_uint = 0x8000;
pub const MV88E6XXX_PORT_CTL1_TRUNK_PORT: c_uint = 0x4000;
pub const MV88E6XXX_PORT_CTL1_TRUNK_ID_MASK: c_uint = 0x0f00;
pub const MV88E6XXX_PORT_CTL1_TRUNK_ID_SHIFT: c_int = 8;
pub const MV88E6XXX_PORT_CTL1_FID_11_4_MASK: c_uint = 0x00ff;
// Offset 0x06: Port Based VLAN Map
pub const MV88E6XXX_PORT_BASE_VLAN: c_uint = 0x06;
pub const MV88E6XXX_PORT_BASE_VLAN_FID_3_0_MASK: c_uint = 0xf000;
// Offset 0x07: Default Port VLAN ID & Priority
pub const MV88E6XXX_PORT_DEFAULT_VLAN: c_uint = 0x07;
pub const MV88E6XXX_PORT_DEFAULT_VLAN_MASK: c_uint = 0x0fff;
// Offset 0x08: Port Control 2 Register
pub const MV88E6XXX_PORT_CTL2: c_uint = 0x08;
pub const MV88E6XXX_PORT_CTL2_IGNORE_FCS: c_uint = 0x8000;
pub const MV88E6XXX_PORT_CTL2_VTU_PRI_OVERRIDE: c_uint = 0x4000;
pub const MV88E6XXX_PORT_CTL2_SA_PRIO_OVERRIDE: c_uint = 0x2000;
pub const MV88E6XXX_PORT_CTL2_DA_PRIO_OVERRIDE: c_uint = 0x1000;
pub const MV88E6XXX_PORT_CTL2_JUMBO_MODE_MASK: c_uint = 0x3000;
pub const MV88E6XXX_PORT_CTL2_JUMBO_MODE_1522: c_uint = 0x0000;
pub const MV88E6XXX_PORT_CTL2_JUMBO_MODE_2048: c_uint = 0x1000;
pub const MV88E6XXX_PORT_CTL2_JUMBO_MODE_10240: c_uint = 0x2000;
pub const MV88E6XXX_PORT_CTL2_8021Q_MODE_MASK: c_uint = 0x0c00;
pub const MV88E6XXX_PORT_CTL2_8021Q_MODE_DISABLED: c_uint = 0x0000;
pub const MV88E6XXX_PORT_CTL2_8021Q_MODE_FALLBACK: c_uint = 0x0400;
pub const MV88E6XXX_PORT_CTL2_8021Q_MODE_CHECK: c_uint = 0x0800;
pub const MV88E6XXX_PORT_CTL2_8021Q_MODE_SECURE: c_uint = 0x0c00;
pub const MV88E6XXX_PORT_CTL2_DISCARD_TAGGED: c_uint = 0x0200;
pub const MV88E6XXX_PORT_CTL2_DISCARD_UNTAGGED: c_uint = 0x0100;
pub const MV88E6XXX_PORT_CTL2_MAP_DA: c_uint = 0x0080;
pub const MV88E6XXX_PORT_CTL2_DEFAULT_FORWARD: c_uint = 0x0040;
pub const MV88E6XXX_PORT_CTL2_EGRESS_MONITOR: c_uint = 0x0020;
pub const MV88E6XXX_PORT_CTL2_INGRESS_MONITOR: c_uint = 0x0010;
pub const MV88E6095_PORT_CTL2_CPU_PORT_MASK: c_uint = 0x000f;
// Offset 0x09: Egress Rate Control
pub const MV88E6XXX_PORT_EGRESS_RATE_CTL1: c_uint = 0x09;
// Offset 0x0A: Egress Rate Control 2
pub const MV88E6XXX_PORT_EGRESS_RATE_CTL2: c_uint = 0x0a;
// Offset 0x0B: Port Association Vector
pub const MV88E6XXX_PORT_ASSOC_VECTOR: c_uint = 0x0b;
pub const MV88E6XXX_PORT_ASSOC_VECTOR_HOLD_AT_1: c_uint = 0x8000;
pub const MV88E6XXX_PORT_ASSOC_VECTOR_INT_AGE_OUT: c_uint = 0x4000;
pub const MV88E6XXX_PORT_ASSOC_VECTOR_LOCKED_PORT: c_uint = 0x2000;
pub const MV88E6XXX_PORT_ASSOC_VECTOR_IGNORE_WRONG: c_uint = 0x1000;
pub const MV88E6XXX_PORT_ASSOC_VECTOR_REFRESH_LOCKED: c_uint = 0x0800;
// Offset 0x0C: Port ATU Control
pub const MV88E6XXX_PORT_ATU_CTL: c_uint = 0x0c;
// Offset 0x0D: Priority Override Register
pub const MV88E6XXX_PORT_PRI_OVERRIDE: c_uint = 0x0d;
pub const MV88E6XXX_PORT_PRI_OVERRIDE_TCAM_MODE_MASK: c_uint = 0x0003;
pub const MV88E6XXX_PORT_PRI_OVERRIDE_TCAM_MODE_48_BYTE: c_uint = 0x0001;
pub const MV88E6XXX_PORT_PRI_OVERRIDE_TCAM_MODE_96_BYTE: c_uint = 0x0002;
// Offset 0x0E: Policy Control Register
pub const MV88E6XXX_PORT_POLICY_CTL: c_uint = 0x0e;
pub const MV88E6XXX_PORT_POLICY_CTL_DA_MASK: c_uint = 0xc000;
pub const MV88E6XXX_PORT_POLICY_CTL_SA_MASK: c_uint = 0x3000;
pub const MV88E6XXX_PORT_POLICY_CTL_VTU_MASK: c_uint = 0x0c00;
pub const MV88E6XXX_PORT_POLICY_CTL_ETYPE_MASK: c_uint = 0x0300;
pub const MV88E6XXX_PORT_POLICY_CTL_PPPOE_MASK: c_uint = 0x00c0;
pub const MV88E6XXX_PORT_POLICY_CTL_VBAS_MASK: c_uint = 0x0030;
pub const MV88E6XXX_PORT_POLICY_CTL_OPT82_MASK: c_uint = 0x000c;
pub const MV88E6XXX_PORT_POLICY_CTL_UDP_MASK: c_uint = 0x0003;
pub const MV88E6XXX_PORT_POLICY_CTL_NORMAL: c_uint = 0x0000;
pub const MV88E6XXX_PORT_POLICY_CTL_MIRROR: c_uint = 0x0001;
pub const MV88E6XXX_PORT_POLICY_CTL_TRAP: c_uint = 0x0002;
pub const MV88E6XXX_PORT_POLICY_CTL_DISCARD: c_uint = 0x0003;
// Offset 0x0E: Policy & MGMT Control Register (FAMILY_6393X)
pub const MV88E6393X_PORT_POLICY_MGMT_CTL: c_uint = 0x0e;
pub const MV88E6393X_PORT_POLICY_MGMT_CTL_UPDATE: c_uint = 0x8000;
pub const MV88E6393X_PORT_POLICY_MGMT_CTL_PTR_MASK: c_uint = 0x3f00;
pub const MV88E6393X_PORT_POLICY_MGMT_CTL_DATA_MASK: c_uint = 0x00ff;
pub const MV88E6393X_PORT_POLICY_MGMT_CTL_PTR_01C280000000XLO: c_uint = 0x2000;
pub const MV88E6393X_PORT_POLICY_MGMT_CTL_PTR_01C280000000XHI: c_uint = 0x2100;
pub const MV88E6393X_PORT_POLICY_MGMT_CTL_PTR_01C280000002XLO: c_uint = 0x2400;
pub const MV88E6393X_PORT_POLICY_MGMT_CTL_PTR_01C280000002XHI: c_uint = 0x2500;
pub const MV88E6393X_PORT_POLICY_MGMT_CTL_PTR_INGRESS_DEST: c_uint = 0x3000;
pub const MV88E6393X_PORT_POLICY_MGMT_CTL_PTR_CPU_DEST: c_uint = 0x3800;
pub const MV88E6393X_PORT_POLICY_MGMT_CTL_CPU_DEST_MGMTPRI: c_uint = 0x00e0;
// Offset 0x0F: Port Special Ether Type
pub const MV88E6XXX_PORT_ETH_TYPE: c_uint = 0x0f;
pub const MV88E6XXX_PORT_ETH_TYPE_DEFAULT: c_uint = 0x9100;
// Offset 0x10: InDiscards Low Counter
pub const MV88E6XXX_PORT_IN_DISCARD_LO: c_uint = 0x10;
// Offset 0x10: Extended Port Control Command
pub const MV88E6393X_PORT_EPC_CMD: c_uint = 0x10;
pub const MV88E6393X_PORT_EPC_CMD_BUSY: c_uint = 0x8000;
pub const MV88E6393X_PORT_EPC_CMD_WRITE: c_uint = 0x3000;
pub const MV88E6393X_PORT_EPC_INDEX_PORT_ETYPE: c_uint = 0x02;
// Offset 0x11: Extended Port Control Data
pub const MV88E6393X_PORT_EPC_DATA: c_uint = 0x11;
// Offset 0x11: InDiscards High Counter
pub const MV88E6XXX_PORT_IN_DISCARD_HI: c_uint = 0x11;
// Offset 0x12: InFiltered Counter
pub const MV88E6XXX_PORT_IN_FILTERED: c_uint = 0x12;
// Offset 0x13: OutFiltered Counter
pub const MV88E6XXX_PORT_OUT_FILTERED: c_uint = 0x13;
// Offset 0x16: LED Control
pub const MV88E6XXX_PORT_LED_CONTROL: c_uint = 0x16;

// Selection masks valid for either port 1,2,3,4 or 5

// Selection control for LED 0 and 1, ports 5 and 6 only has LED 0
// Bits  Function
// 0..3  LED 0 control selector on ports 1-5
// 4..7  LED 1 control selector on ports 1-4 on port 5 this controls LED 0 of port 6
//
// Sel Port LED Function for the 6352 family:
// 0   1-4  0   Link/Act/Speed by Blink Rate (off=no link, on=link, blink=activity, blink speed=link speed)
// 1-4  1   Port 2's Special LED
// 5-6  0   Port 5 Link/Act (off=no link, on=link, blink=activity)
// 5-6  1   Port 6 Link/Act (off=no link, on=link 1000, blink=activity)
// 1   1-4  0   100/1000 Link/Act (off=no link, on=100 or 1000 link, blink=activity)
// 1-4  1   10/100 Link Act (off=no link, on=10 or 100 link, blink=activity)
// 5-6  0   Fiber 100 Link/Act (off=no link, on=link 100, blink=activity)
// 5-6  1   Fiber 1000 Link/Act (off=no link, on=link 1000, blink=activity)
// 2   1-4  0   1000 Link/Act (off=no link, on=link 1000, blink=activity)
// 1-4  1   10/100 Link/Act (off=no link, on=10 or 100 link, blink=activity)
// 5-6  0   Fiber 1000 Link/Act (off=no link, on=link 1000, blink=activity)
// 5-6  1   Fiber 100 Link/Act (off=no link, on=link 100, blink=activity)
// 3   1-4  0   Link/Act (off=no link, on=link, blink=activity)
// 1-4  1   1000 Link (off=no link, on=1000 link)
// 5-6  0   Port 0's Special LED
// 5-6  1   Fiber Link (off=no link, on=link)
// 4   1-4  0   Port 0's Special LED
// 1-4  1   Port 1's Special LED
// 5-6  0   Port 1's Special LED
// 5-6  1   Port 5 Link/Act (off=no link, on=link, blink=activity)
// 5   1-4  0   Reserved
// 1-4  1   Reserved
// 5-6  0   Port 2's Special LED
// 5-6  1   Port 6 Link (off=no link, on=link)
// 6   1-4  0   Duplex/Collision (off=half-duplex,on=full-duplex,blink=collision)
// 1-4  1   10/1000 Link/Act (off=no link, on=10 or 1000 link, blink=activity)
// 5-6  0   Port 5 Duplex/Collision (off=half-duplex, on=full-duplex, blink=col)
// 5-6  1   Port 6 Duplex/Collision (off=half-duplex, on=full-duplex, blink=col)
// 7   1-4  0   10/1000 Link/Act (off=no link, on=10 or 1000 link, blink=activity)
// 1-4  1   10/1000 Link (off=no link, on=10 or 1000 link)
// 5-6  0   Port 5 Link/Act/Speed by Blink rate (off=no link, on=link, blink=activity, blink speed=link speed)
// 5-6  1   Port 6 Link/Act/Speed by Blink rate (off=no link, on=link, blink=activity,  blink speed=link speed)
// 8   1-4  0   Link (off=no link, on=link)
// 1-4  1   Activity (off=no link, blink on=activity)
// 5-6  0   Port 6 Link/Act (off=no link, on=link, blink=activity)
// 5-6  1   Port 0's Special LED
// 9   1-4  0   10 Link (off=no link, on=10 link)
// 1-4  1   100 Link (off=no link, on=100 link)
// 5-6  0   Reserved
// 5-6  1   Port 1's Special LED
// a   1-4  0   10 Link/Act (off=no link, on=10 link, blink=activity)
// 1-4  1   100 Link/Act (off=no link, on=100 link, blink=activity)
// 5-6  0   Reserved
// 5-6  1   Port 2's Special LED
// b   1-4  0   100/1000 Link (off=no link, on=100 or 1000 link)
// 1-4  1   10/100 Link (off=no link, on=100 link, blink=activity)
// 5-6  0   Reserved
// 5-6  1   Reserved
// c     *  *   PTP Act (blink on=PTP activity)
// d     *  *   Force Blink
// e     *  *   Force Off
// f     *  *   Force On
//
// Select LED0 output
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SEL0: c_uint = 0x0;
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SEL1: c_uint = 0x1;
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SEL2: c_uint = 0x2;
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SEL3: c_uint = 0x3;
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SEL4: c_uint = 0x4;
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SEL5: c_uint = 0x5;
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SEL6: c_uint = 0x6;
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SEL7: c_uint = 0x7;
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SEL8: c_uint = 0x8;
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SEL9: c_uint = 0x9;
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SELA: c_uint = 0xa;
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SELB: c_uint = 0xb;
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SELC: c_uint = 0xc;
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SELD: c_uint = 0xd;
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SELE: c_uint = 0xe;
pub const MV88E6XXX_PORT_LED_CONTROL_LED0_SELF: c_uint = 0xf;

// Stretch and Blink Rate Control (Index 0x06 of LED Control)
// Pulse Stretch Selection for all LED's on this port

// Blink Rate Selection for all LEDs on this port
pub const MV88E6XXX_PORT_LED_CONTROL_0x06_BLINK_RATE_21MS: c_int = 0;
pub const MV88E6XXX_PORT_LED_CONTROL_0x06_BLINK_RATE_42MS: c_int = 1;
pub const MV88E6XXX_PORT_LED_CONTROL_0x06_BLINK_RATE_84MS: c_int = 2;
pub const MV88E6XXX_PORT_LED_CONTROL_0x06_BLINK_RATE_168MS: c_int = 3;
pub const MV88E6XXX_PORT_LED_CONTROL_0x06_BLINK_RATE_336MS: c_int = 4;
pub const MV88E6XXX_PORT_LED_CONTROL_0x06_BLINK_RATE_672MS: c_int = 5;
// Control for Special LED (Index 0x7 of LED Control on Port0)

// Control for Special LED (Index 0x7 of LED Control on Port 1)

// Control for Special LED (Index 0x7 of LED Control on Port 2)

// Offset 0x18: IEEE Priority Mapping Table
pub const MV88E6390_PORT_IEEE_PRIO_MAP_TABLE: c_uint = 0x18;
pub const MV88E6390_PORT_IEEE_PRIO_MAP_TABLE_UPDATE: c_uint = 0x8000;
pub const MV88E6390_PORT_IEEE_PRIO_MAP_TABLE_MASK: c_uint = 0x7000;
pub const MV88E6390_PORT_IEEE_PRIO_MAP_TABLE_INGRESS_PCP: c_uint = 0x0000;
pub const MV88E6390_PORT_IEEE_PRIO_MAP_TABLE_EGRESS_GREEN_PCP: c_uint = 0x1000;
pub const MV88E6390_PORT_IEEE_PRIO_MAP_TABLE_EGRESS_YELLOW_PCP: c_uint = 0x2000;
pub const MV88E6390_PORT_IEEE_PRIO_MAP_TABLE_EGRESS_AVB_PCP: c_uint = 0x3000;
pub const MV88E6390_PORT_IEEE_PRIO_MAP_TABLE_EGRESS_GREEN_DSCP: c_uint = 0x5000;
pub const MV88E6390_PORT_IEEE_PRIO_MAP_TABLE_EGRESS_YELLOW_DSCP: c_uint = 0x6000;
pub const MV88E6390_PORT_IEEE_PRIO_MAP_TABLE_EGRESS_AVB_DSCP: c_uint = 0x7000;
pub const MV88E6390_PORT_IEEE_PRIO_MAP_TABLE_PTR_MASK: c_uint = 0x0e00;
pub const MV88E6390_PORT_IEEE_PRIO_MAP_TABLE_DATA_MASK: c_uint = 0x01ff;
// Offset 0x18: Port IEEE Priority Remapping Registers (0-3)
pub const MV88E6095_PORT_IEEE_PRIO_REMAP_0123: c_uint = 0x18;
// Offset 0x19: Port IEEE Priority Remapping Registers (4-7)
pub const MV88E6095_PORT_IEEE_PRIO_REMAP_4567: c_uint = 0x19;
// Offset 0x1a: Magic undocumented errata register
pub const MV88E6XXX_PORT_RESERVED_1A: c_uint = 0x1a;
pub const MV88E6XXX_PORT_RESERVED_1A_BUSY: c_uint = 0x8000;
pub const MV88E6XXX_PORT_RESERVED_1A_WRITE: c_uint = 0x4000;
pub const MV88E6XXX_PORT_RESERVED_1A_READ: c_uint = 0x0000;
pub const MV88E6XXX_PORT_RESERVED_1A_PORT_SHIFT: c_int = 5;
pub const MV88E6XXX_PORT_RESERVED_1A_BLOCK_SHIFT: c_int = 10;
pub const MV88E6XXX_PORT_RESERVED_1A_CTRL_PORT: c_uint = 0x04;
pub const MV88E6XXX_PORT_RESERVED_1A_DATA_PORT: c_uint = 0x05;
pub const MV88E6341_PORT_RESERVED_1A_FORCE_CMODE: c_uint = 0x8000;
pub const MV88E6341_PORT_RESERVED_1A_SGMII_AN: c_uint = 0x2000;
extern "C" {
    pub fn mv88e6xxx_port_set_link(chip: *mut mv88e6xxx_chip, port: c_int, link: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_port_sync_link(chip: *mut mv88e6xxx_chip, port: c_int, mode: c_uint, isup: bool) -> c_int;
}
extern "C" {
    pub fn mv88e6185_port_sync_link(chip: *mut mv88e6xxx_chip, port: c_int, mode: c_uint, isup: bool) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_port_set_state(chip: *mut mv88e6xxx_chip, port: c_int, state: u8) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_port_set_vlan_map(chip: *mut mv88e6xxx_chip, port: c_int, map: u16) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_port_get_fid(chip: *mut mv88e6xxx_chip, port: c_int, fid: *mut u16) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_port_set_fid(chip: *mut mv88e6xxx_chip, port: c_int, fid: u16) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_port_get_pvid(chip: *mut mv88e6xxx_chip, port: c_int, pvid: *mut u16) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_port_set_pvid(chip: *mut mv88e6xxx_chip, port: c_int, pvid: u16) -> c_int;
}
extern "C" {
    pub fn mv88e6095_port_tag_remap(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6390_port_tag_remap(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6393x_port_mgmt_rsvd2cpu(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6095_port_egress_rate_limiting(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6097_port_egress_rate_limiting(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6185_port_get_cmode(chip: *mut mv88e6xxx_chip, port: c_int, cmode: *mut u8) -> c_int;
}
extern "C" {
    pub fn mv88e6352_port_get_cmode(chip: *mut mv88e6xxx_chip, port: c_int, cmode: *mut u8) -> c_int;
}

extern "C" {
    pub fn mv88e6xxx_port_setup_leds(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}

extern "C" {
    pub fn mv88e6xxx_port_set_map_da(chip: *mut mv88e6xxx_chip, port: c_int, map: bool) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_port_disable_learn_limit(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_port_disable_pri_override(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_port_hidden_wait(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_port_enable_tcam(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
