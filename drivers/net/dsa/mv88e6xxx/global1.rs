//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/mv88e6xxx/global1.h
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
// Marvell 88E6xxx Switch Global (1) Registers support
//
// Copyright (c) 2008 Marvell Semiconductor
//
// Copyright (c) 2016-2017 Savoir-faire Linux Inc.
// Vivien Didelot <vivien.didelot@savoirfairelinux.com>
//

// Offset 0x00: Switch Global Status Register
pub const MV88E6XXX_G1_STS: c_uint = 0x00;
pub const MV88E6352_G1_STS_PPU_STATE: c_uint = 0x8000;
pub const MV88E6185_G1_STS_PPU_STATE_MASK: c_uint = 0xc000;
pub const MV88E6185_G1_STS_PPU_STATE_DISABLED_RST: c_uint = 0x0000;
pub const MV88E6185_G1_STS_PPU_STATE_INITIALIZING: c_uint = 0x4000;
pub const MV88E6185_G1_STS_PPU_STATE_DISABLED: c_uint = 0x8000;
pub const MV88E6185_G1_STS_PPU_STATE_POLLING: c_uint = 0xc000;
pub const MV88E6XXX_G1_STS_INIT_READY: c_uint = 0x0800;
pub const MV88E6393X_G1_STS_IRQ_DEVICE_2: c_int = 9;
pub const MV88E6XXX_G1_STS_IRQ_AVB: c_int = 8;
pub const MV88E6XXX_G1_STS_IRQ_DEVICE: c_int = 7;
pub const MV88E6XXX_G1_STS_IRQ_STATS: c_int = 6;
pub const MV88E6XXX_G1_STS_IRQ_VTU_PROB: c_int = 5;
pub const MV88E6XXX_G1_STS_IRQ_VTU_DONE: c_int = 4;
pub const MV88E6XXX_G1_STS_IRQ_ATU_PROB: c_int = 3;
pub const MV88E6XXX_G1_STS_IRQ_ATU_DONE: c_int = 2;
pub const MV88E6XXX_G1_STS_IRQ_TCAM_DONE: c_int = 1;
pub const MV88E6XXX_G1_STS_IRQ_EEPROM_DONE: c_int = 0;
// Offset 0x01: Switch MAC Address Register Bytes 0 & 1
// Offset 0x02: Switch MAC Address Register Bytes 2 & 3
// Offset 0x03: Switch MAC Address Register Bytes 4 & 5
//
pub const MV88E6XXX_G1_MAC_01: c_uint = 0x01;
pub const MV88E6XXX_G1_MAC_23: c_uint = 0x02;
pub const MV88E6XXX_G1_MAC_45: c_uint = 0x03;
// Offset 0x01: ATU FID Register
pub const MV88E6352_G1_ATU_FID: c_uint = 0x01;
// Offset 0x02: VTU FID Register
pub const MV88E6352_G1_VTU_FID: c_uint = 0x02;
pub const MV88E6352_G1_VTU_FID_VID_POLICY: c_uint = 0x1000;
pub const MV88E6352_G1_VTU_FID_MASK: c_uint = 0x0fff;
// Offset 0x03: VTU SID Register
pub const MV88E6352_G1_VTU_SID: c_uint = 0x03;
pub const MV88E6352_G1_VTU_SID_MASK: c_uint = 0x3f;
// Offset 0x04: Switch Global Control Register
pub const MV88E6XXX_G1_CTL1: c_uint = 0x04;
pub const MV88E6XXX_G1_CTL1_SW_RESET: c_uint = 0x8000;
pub const MV88E6XXX_G1_CTL1_PPU_ENABLE: c_uint = 0x4000;
pub const MV88E6352_G1_CTL1_DISCARD_EXCESS: c_uint = 0x2000;
pub const MV88E6185_G1_CTL1_SCHED_PRIO: c_uint = 0x0800;
pub const MV88E6185_G1_CTL1_MAX_FRAME_1632: c_uint = 0x0400;
pub const MV88E6185_G1_CTL1_RELOAD_EEPROM: c_uint = 0x0200;
pub const MV88E6393X_G1_CTL1_DEVICE2_EN: c_uint = 0x0200;
pub const MV88E6XXX_G1_CTL1_DEVICE_EN: c_uint = 0x0080;
pub const MV88E6XXX_G1_CTL1_STATS_DONE_EN: c_uint = 0x0040;
pub const MV88E6XXX_G1_CTL1_VTU_PROBLEM_EN: c_uint = 0x0020;
pub const MV88E6XXX_G1_CTL1_VTU_DONE_EN: c_uint = 0x0010;
pub const MV88E6XXX_G1_CTL1_ATU_PROBLEM_EN: c_uint = 0x0008;
pub const MV88E6XXX_G1_CTL1_ATU_DONE_EN: c_uint = 0x0004;
pub const MV88E6XXX_G1_CTL1_TCAM_EN: c_uint = 0x0002;
pub const MV88E6XXX_G1_CTL1_EEPROM_DONE_EN: c_uint = 0x0001;
// Offset 0x05: VTU Operation Register
pub const MV88E6XXX_G1_VTU_OP: c_uint = 0x05;
pub const MV88E6XXX_G1_VTU_OP_BUSY: c_uint = 0x8000;
pub const MV88E6XXX_G1_VTU_OP_MASK: c_uint = 0x7000;
pub const MV88E6XXX_G1_VTU_OP_FLUSH_ALL: c_uint = 0x1000;
pub const MV88E6XXX_G1_VTU_OP_NOOP: c_uint = 0x2000;
pub const MV88E6XXX_G1_VTU_OP_VTU_LOAD_PURGE: c_uint = 0x3000;
pub const MV88E6XXX_G1_VTU_OP_VTU_GET_NEXT: c_uint = 0x4000;
pub const MV88E6XXX_G1_VTU_OP_STU_LOAD_PURGE: c_uint = 0x5000;
pub const MV88E6XXX_G1_VTU_OP_STU_GET_NEXT: c_uint = 0x6000;
pub const MV88E6XXX_G1_VTU_OP_GET_CLR_VIOLATION: c_uint = 0x7000;

pub const MV88E6XXX_G1_VTU_OP_SPID_MASK: c_uint = 0xf;
// Offset 0x06: VTU VID Register
pub const MV88E6XXX_G1_VTU_VID: c_uint = 0x06;
pub const MV88E6XXX_G1_VTU_VID_MASK: c_uint = 0x0fff;
pub const MV88E6390_G1_VTU_VID_PAGE: c_uint = 0x2000;
pub const MV88E6XXX_G1_VTU_VID_VALID: c_uint = 0x1000;
// Offset 0x07: VTU/STU Data Register 1
// Offset 0x08: VTU/STU Data Register 2
// Offset 0x09: VTU/STU Data Register 3
//
pub const MV88E6XXX_G1_VTU_DATA1: c_uint = 0x07;
pub const MV88E6XXX_G1_VTU_DATA2: c_uint = 0x08;
pub const MV88E6XXX_G1_VTU_DATA3: c_uint = 0x09;
pub const MV88E6XXX_G1_VTU_STU_DATA_MASK: c_uint = 0x0003;
pub const MV88E6XXX_G1_VTU_DATA_MEMBER_TAG_UNMODIFIED: c_uint = 0x0000;
pub const MV88E6XXX_G1_VTU_DATA_MEMBER_TAG_UNTAGGED: c_uint = 0x0001;
pub const MV88E6XXX_G1_VTU_DATA_MEMBER_TAG_TAGGED: c_uint = 0x0002;
pub const MV88E6XXX_G1_VTU_DATA_MEMBER_TAG_NON_MEMBER: c_uint = 0x0003;
pub const MV88E6XXX_G1_STU_DATA_PORT_STATE_DISABLED: c_uint = 0x0000;
pub const MV88E6XXX_G1_STU_DATA_PORT_STATE_BLOCKING: c_uint = 0x0001;
pub const MV88E6XXX_G1_STU_DATA_PORT_STATE_LEARNING: c_uint = 0x0002;
pub const MV88E6XXX_G1_STU_DATA_PORT_STATE_FORWARDING: c_uint = 0x0003;
// Offset 0x0A: ATU Control Register
pub const MV88E6XXX_G1_ATU_CTL: c_uint = 0x0a;
pub const MV88E6XXX_G1_ATU_CTL_LEARN2ALL: c_uint = 0x0008;
pub const MV88E6161_G1_ATU_CTL_HASH_MASK: c_uint = 0x0003;
// Offset 0x0B: ATU Operation Register
pub const MV88E6XXX_G1_ATU_OP: c_uint = 0x0b;
pub const MV88E6XXX_G1_ATU_OP_BUSY: c_uint = 0x8000;
pub const MV88E6XXX_G1_ATU_OP_MASK: c_uint = 0x7000;
pub const MV88E6XXX_G1_ATU_OP_NOOP: c_uint = 0x0000;
pub const MV88E6XXX_G1_ATU_OP_FLUSH_MOVE_ALL: c_uint = 0x1000;
pub const MV88E6XXX_G1_ATU_OP_FLUSH_MOVE_NON_STATIC: c_uint = 0x2000;
pub const MV88E6XXX_G1_ATU_OP_LOAD_DB: c_uint = 0x3000;
pub const MV88E6XXX_G1_ATU_OP_GET_NEXT_DB: c_uint = 0x4000;
pub const MV88E6XXX_G1_ATU_OP_FLUSH_MOVE_ALL_DB: c_uint = 0x5000;
pub const MV88E6XXX_G1_ATU_OP_FLUSH_MOVE_NON_STATIC_DB: c_uint = 0x6000;
pub const MV88E6XXX_G1_ATU_OP_GET_CLR_VIOLATION: c_uint = 0x7000;

// Offset 0x0C: ATU Data Register
pub const MV88E6XXX_G1_ATU_DATA: c_uint = 0x0c;
pub const MV88E6XXX_G1_ATU_DATA_TRUNK: c_uint = 0x8000;
pub const MV88E6XXX_G1_ATU_DATA_TRUNK_ID_MASK: c_uint = 0x00f0;
pub const MV88E6XXX_G1_ATU_DATA_PORT_VECTOR_MASK: c_uint = 0x3ff0;
pub const MV88E6XXX_G1_ATU_DATA_STATE_MASK: c_uint = 0x000f;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_UNUSED: c_uint = 0x0000;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_AGE_1_OLDEST: c_uint = 0x0001;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_AGE_2: c_uint = 0x0002;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_AGE_3: c_uint = 0x0003;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_AGE_4: c_uint = 0x0004;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_AGE_5: c_uint = 0x0005;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_AGE_6: c_uint = 0x0006;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_AGE_7_NEWEST: c_uint = 0x0007;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_STATIC_POLICY: c_uint = 0x0008;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_STATIC_POLICY_PO: c_uint = 0x0009;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_STATIC_AVB_NRL: c_uint = 0x000a;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_STATIC_AVB_NRL_PO: c_uint = 0x000b;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_STATIC_DA_MGMT: c_uint = 0x000c;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_STATIC_DA_MGMT_PO: c_uint = 0x000d;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_STATIC: c_uint = 0x000e;
pub const MV88E6XXX_G1_ATU_DATA_STATE_UC_STATIC_PO: c_uint = 0x000f;
pub const MV88E6XXX_G1_ATU_DATA_STATE_MC_UNUSED: c_uint = 0x0000;
pub const MV88E6XXX_G1_ATU_DATA_STATE_MC_STATIC_POLICY: c_uint = 0x0004;
pub const MV88E6XXX_G1_ATU_DATA_STATE_MC_STATIC_AVB_NRL: c_uint = 0x0005;
pub const MV88E6XXX_G1_ATU_DATA_STATE_MC_STATIC_DA_MGMT: c_uint = 0x0006;
pub const MV88E6XXX_G1_ATU_DATA_STATE_MC_STATIC: c_uint = 0x0007;
pub const MV88E6XXX_G1_ATU_DATA_STATE_MC_STATIC_POLICY_PO: c_uint = 0x000c;
pub const MV88E6XXX_G1_ATU_DATA_STATE_MC_STATIC_AVB_NRL_PO: c_uint = 0x000d;
pub const MV88E6XXX_G1_ATU_DATA_STATE_MC_STATIC_DA_MGMT_PO: c_uint = 0x000e;
pub const MV88E6XXX_G1_ATU_DATA_STATE_MC_STATIC_PO: c_uint = 0x000f;
// Offset 0x0D: ATU MAC Address Register Bytes 0 & 1
// Offset 0x0E: ATU MAC Address Register Bytes 2 & 3
// Offset 0x0F: ATU MAC Address Register Bytes 4 & 5
//
pub const MV88E6XXX_G1_ATU_MAC01: c_uint = 0x0d;
pub const MV88E6XXX_G1_ATU_MAC23: c_uint = 0x0e;
pub const MV88E6XXX_G1_ATU_MAC45: c_uint = 0x0f;
// Offset 0x10: IP-PRI Mapping Register 0
// Offset 0x11: IP-PRI Mapping Register 1
// Offset 0x12: IP-PRI Mapping Register 2
// Offset 0x13: IP-PRI Mapping Register 3
// Offset 0x14: IP-PRI Mapping Register 4
// Offset 0x15: IP-PRI Mapping Register 5
// Offset 0x16: IP-PRI Mapping Register 6
// Offset 0x17: IP-PRI Mapping Register 7
//
pub const MV88E6XXX_G1_IP_PRI_0: c_uint = 0x10;
pub const MV88E6XXX_G1_IP_PRI_1: c_uint = 0x11;
pub const MV88E6XXX_G1_IP_PRI_2: c_uint = 0x12;
pub const MV88E6XXX_G1_IP_PRI_3: c_uint = 0x13;
pub const MV88E6XXX_G1_IP_PRI_4: c_uint = 0x14;
pub const MV88E6XXX_G1_IP_PRI_5: c_uint = 0x15;
pub const MV88E6XXX_G1_IP_PRI_6: c_uint = 0x16;
pub const MV88E6XXX_G1_IP_PRI_7: c_uint = 0x17;
// Offset 0x18: IEEE-PRI Register
pub const MV88E6XXX_G1_IEEE_PRI: c_uint = 0x18;
// Offset 0x19: Core Tag Type
pub const MV88E6185_G1_CORE_TAG_TYPE: c_uint = 0x19;
// Offset 0x1A: Monitor Control
pub const MV88E6185_G1_MONITOR_CTL: c_uint = 0x1a;
pub const MV88E6185_G1_MONITOR_CTL_INGRESS_DEST_MASK: c_uint = 0xf000;
pub const MV88E6185_G1_MONITOR_CTL_EGRESS_DEST_MASK: c_uint = 0x0f00;
pub const MV88E6185_G1_MONITOR_CTL_ARP_DEST_MASK: c_uint = 0x00f0;
pub const MV88E6352_G1_MONITOR_CTL_CPU_DEST_MASK: c_uint = 0x00f0;
pub const MV88E6352_G1_MONITOR_CTL_MIRROR_DEST_MASK: c_uint = 0x000f;
// Offset 0x1A: Monitor & MGMT Control Register
pub const MV88E6390_G1_MONITOR_MGMT_CTL: c_uint = 0x1a;
pub const MV88E6390_G1_MONITOR_MGMT_CTL_UPDATE: c_uint = 0x8000;
pub const MV88E6390_G1_MONITOR_MGMT_CTL_PTR_MASK: c_uint = 0x3f00;
pub const MV88E6390_G1_MONITOR_MGMT_CTL_PTR_0180C200000XLO: c_uint = 0x0000;
pub const MV88E6390_G1_MONITOR_MGMT_CTL_PTR_0180C200000XHI: c_uint = 0x0100;
pub const MV88E6390_G1_MONITOR_MGMT_CTL_PTR_0180C200002XLO: c_uint = 0x0200;
pub const MV88E6390_G1_MONITOR_MGMT_CTL_PTR_0180C200002XHI: c_uint = 0x0300;
pub const MV88E6390_G1_MONITOR_MGMT_CTL_PTR_INGRESS_DEST: c_uint = 0x2000;
pub const MV88E6390_G1_MONITOR_MGMT_CTL_PTR_EGRESS_DEST: c_uint = 0x2100;
pub const MV88E6390_G1_MONITOR_MGMT_CTL_PTR_CPU_DEST: c_uint = 0x3000;
pub const MV88E6390_G1_MONITOR_MGMT_CTL_PTR_PTP_CPU_DEST: c_uint = 0x3200;
pub const MV88E6390_G1_MONITOR_MGMT_CTL_PTR_CPU_DEST_MGMTPRI: c_uint = 0x00e0;
pub const MV88E6390_G1_MONITOR_MGMT_CTL_DATA_MASK: c_uint = 0x00ff;
// Offset 0x1C: Global Control 2
pub const MV88E6XXX_G1_CTL2: c_uint = 0x1c;
pub const MV88E6185_G1_CTL2_CASCADE_PORT_MASK: c_uint = 0xf000;
pub const MV88E6185_G1_CTL2_CASCADE_PORT_NONE: c_uint = 0xe000;
pub const MV88E6185_G1_CTL2_CASCADE_PORT_MULTI: c_uint = 0xf000;
pub const MV88E6352_G1_CTL2_HEADER_TYPE_MASK: c_uint = 0xc000;
pub const MV88E6352_G1_CTL2_HEADER_TYPE_ORIG: c_uint = 0x0000;
pub const MV88E6352_G1_CTL2_HEADER_TYPE_MGMT: c_uint = 0x4000;
pub const MV88E6390_G1_CTL2_HEADER_TYPE_LAG: c_uint = 0x8000;
pub const MV88E6352_G1_CTL2_RMU_MODE_MASK: c_uint = 0x3000;
pub const MV88E6352_G1_CTL2_RMU_MODE_DISABLED: c_uint = 0x0000;
pub const MV88E6352_G1_CTL2_RMU_MODE_PORT_4: c_uint = 0x1000;
pub const MV88E6352_G1_CTL2_RMU_MODE_PORT_5: c_uint = 0x2000;
pub const MV88E6352_G1_CTL2_RMU_MODE_PORT_6: c_uint = 0x3000;
pub const MV88E6085_G1_CTL2_DA_CHECK: c_uint = 0x4000;
pub const MV88E6085_G1_CTL2_P10RM: c_uint = 0x2000;
pub const MV88E6085_G1_CTL2_RM_ENABLE: c_uint = 0x1000;
pub const MV88E6352_G1_CTL2_DA_CHECK: c_uint = 0x0800;
pub const MV88E6390_G1_CTL2_RMU_MODE_MASK: c_uint = 0x0700;
pub const MV88E6390_G1_CTL2_RMU_MODE_PORT_0: c_uint = 0x0000;
pub const MV88E6390_G1_CTL2_RMU_MODE_PORT_1: c_uint = 0x0100;
pub const MV88E6390_G1_CTL2_RMU_MODE_PORT_9: c_uint = 0x0200;
pub const MV88E6390_G1_CTL2_RMU_MODE_PORT_10: c_uint = 0x0300;
pub const MV88E6390_G1_CTL2_RMU_MODE_ALL_DSA: c_uint = 0x0600;
pub const MV88E6390_G1_CTL2_RMU_MODE_DISABLED: c_uint = 0x0700;
pub const MV88E6390_G1_CTL2_HIST_MODE_MASK: c_uint = 0x00c0;
pub const MV88E6390_G1_CTL2_HIST_MODE_RX: c_uint = 0x0040;
pub const MV88E6390_G1_CTL2_HIST_MODE_TX: c_uint = 0x0080;
pub const MV88E6352_G1_CTL2_CTR_MODE_MASK: c_uint = 0x0060;
pub const MV88E6390_G1_CTL2_CTR_MODE: c_uint = 0x0020;
pub const MV88E6XXX_G1_CTL2_DEVICE_NUMBER_MASK: c_uint = 0x001f;
// Offset 0x1D: Stats Operation Register
pub const MV88E6XXX_G1_STATS_OP: c_uint = 0x1d;
pub const MV88E6XXX_G1_STATS_OP_BUSY: c_uint = 0x8000;
pub const MV88E6XXX_G1_STATS_OP_NOP: c_uint = 0x0000;
pub const MV88E6XXX_G1_STATS_OP_FLUSH_ALL: c_uint = 0x1000;
pub const MV88E6XXX_G1_STATS_OP_FLUSH_PORT: c_uint = 0x2000;
pub const MV88E6XXX_G1_STATS_OP_READ_CAPTURED: c_uint = 0x4000;
pub const MV88E6XXX_G1_STATS_OP_CAPTURE_PORT: c_uint = 0x5000;
pub const MV88E6XXX_G1_STATS_OP_HIST_RX: c_uint = 0x0400;
pub const MV88E6XXX_G1_STATS_OP_HIST_TX: c_uint = 0x0800;
pub const MV88E6XXX_G1_STATS_OP_HIST_RX_TX: c_uint = 0x0c00;
pub const MV88E6XXX_G1_STATS_OP_BANK_1_BIT_9: c_uint = 0x0200;
pub const MV88E6XXX_G1_STATS_OP_BANK_1_BIT_10: c_uint = 0x0400;
// Offset 0x1E: Stats Counter Register Bytes 3 & 2
// Offset 0x1F: Stats Counter Register Bytes 1 & 0
//
pub const MV88E6XXX_G1_STATS_COUNTER_32: c_uint = 0x1e;
pub const MV88E6XXX_G1_STATS_COUNTER_01: c_uint = 0x1f;
extern "C" {
    pub fn mv88e6xxx_g1_read(chip: *mut mv88e6xxx_chip, reg: c_int, val: *mut u16) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g1_write(chip: *mut mv88e6xxx_chip, reg: c_int, val: u16) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g1_set_switch_mac(chip: *mut mv88e6xxx_chip, addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn mv88e6185_g1_reset(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6352_g1_reset(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6250_g1_reset(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g1_wait_eeprom_done(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6250_g1_wait_eeprom_done_prereset(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6185_g1_ppu_enable(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6185_g1_ppu_disable(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6185_g1_set_max_frame_size(chip: *mut mv88e6xxx_chip, mtu: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g1_stats_snapshot(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6320_g1_stats_snapshot(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6390_g1_stats_snapshot(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6095_g1_stats_set_histogram(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6390_g1_stats_set_histogram(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g1_stats_read(chip: *mut mv88e6xxx_chip, stat: c_int, val: *mut u32);
}
extern "C" {
    pub fn mv88e6xxx_g1_stats_clear(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6095_g1_set_cpu_port(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6390_g1_set_cpu_port(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6390_g1_set_ptp_cpu_port(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6390_g1_mgmt_rsvd2cpu(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6085_g1_ip_pri_map(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6085_g1_ieee_pri_map(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6250_g1_ieee_pri_map(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6185_g1_set_cascade_port(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6085_g1_rmu_disable(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6352_g1_rmu_disable(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6390_g1_rmu_disable(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g1_set_device_number(chip: *mut mv88e6xxx_chip, index: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g1_atu_set_learn2all(chip: *mut mv88e6xxx_chip, learn2all: bool) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g1_atu_flush(chip: *mut mv88e6xxx_chip, fid: u16, all: bool) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g1_atu_prob_irq_setup(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g1_atu_prob_irq_free(chip: *mut mv88e6xxx_chip);
}
extern "C" {
    pub fn mv88e6165_g1_atu_get_hash(chip: *mut mv88e6xxx_chip, hash: *mut u8) -> c_int;
}
extern "C" {
    pub fn mv88e6165_g1_atu_set_hash(chip: *mut mv88e6xxx_chip, hash: u8) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g1_vtu_flush(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g1_vtu_prob_irq_setup(chip: *mut mv88e6xxx_chip) -> c_int;
}
extern "C" {
    pub fn mv88e6xxx_g1_vtu_prob_irq_free(chip: *mut mv88e6xxx_chip);
}
extern "C" {
    pub fn mv88e6xxx_g1_atu_get_next(chip: *mut mv88e6xxx_chip, fid: u16) -> c_int;
}
