//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thunderbolt/tb_regs.h
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
//
// Thunderbolt driver - Port/Switch config area registers
//
// Every thunderbolt device consists (logically) of a switch with multiple
// ports. Every port contains up to four config regions (HOPS, PORT, SWITCH,
// COUNTERS) which are used to configure the device.
//
// Copyright (c) 2014 Andreas Noever <andreas.noever@gmail.com>
// Copyright (C) 2018, Intel Corporation
//

//
// TODO: should be 63? But we do not know how to receive frames larger than 256
// bytes at the frame level. (header + checksum = 16, 60*4 = 240)
//
pub const TB_MAX_CONFIG_RW_LENGTH: c_int = 60;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_switch_cap {
    TB_SWITCH_CAP_TMU		= 0x03,
    TB_SWITCH_CAP_VSE		= 0x05,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_switch_vse_cap {
    TB_VSE_CAP_PLUG_EVENTS		= 0x01, /* also EEPROM */
    TB_VSE_CAP_TIME2		= 0x03,
    TB_VSE_CAP_CP_LP		= 0x04,
    TB_VSE_CAP_LINK_CONTROLLER	= 0x06, /* also IECS */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_port_cap {
    TB_PORT_CAP_PHY			= 0x01,
    TB_PORT_CAP_POWER		= 0x02,
    TB_PORT_CAP_TIME1		= 0x03,
    TB_PORT_CAP_ADAP		= 0x04,
    TB_PORT_CAP_VSE			= 0x05,
    TB_PORT_CAP_USB4		= 0x06,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_port_state {
    TB_PORT_DISABLED	= 0, /* tb_cap_phy.disable == 1 */
    TB_PORT_CONNECTING	= 1, /* retry */
    TB_PORT_UP		= 2,
    TB_PORT_TX_CL0S		= 3,
    TB_PORT_RX_CL0S		= 4,
    TB_PORT_CL1		= 5,
    TB_PORT_CL2		= 6,
    TB_PORT_UNPLUGGED	= 7,
}

// capability headers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_cap_basic {
    pub next: u8,
// enum tb_cap cap:8; prevent "narrower than values of its type"
    pub /: *mut *mut u8 cap; / if cap == 0x05 then we have a extended capability,
    pub __packed: },
//
// struct tb_cap_extended_short - Switch extended short capability
// @next: Pointer to the next capability. If @next and @length are zero
// then we have a long cap.
// @cap: Base capability ID (see &enum tb_switch_cap)
// @vsec_id: Vendor specific capability ID (see &enum switch_vse_cap)
// @length: Length of this capability
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_cap_extended_short {
    pub next: u8,
    pub cap: u8,
    pub vsec_id: u8,
    pub length: u8,
    pub __packed: },
//
// struct tb_cap_extended_long - Switch extended long capability
// @zero1: This field should be zero
// @cap: Base capability ID (see &enum tb_switch_cap)
// @vsec_id: Vendor specific capability ID (see &enum switch_vse_cap)
// @zero2: This field should be zero
// @next: Pointer to the next capability
// @length: Length of this capability
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_cap_extended_long {
    pub zero1: u8,
    pub cap: u8,
    pub vsec_id: u8,
    pub zero2: u8,
    pub next: u16,
    pub length: u16,
    pub __packed: },
//
// struct tb_cap_any - Structure capable of holding every capability
// @basic: Basic capability
// @extended_short: Vendor specific capability
// @extended_long: Vendor specific extended capability
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_cap_any {
    pub basic: tb_cap_basic,
    pub extended_short: tb_cap_extended_short,
    pub extended_long: tb_cap_extended_long,
}

// capabilities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_cap_link_controller {
    pub cap_header: tb_cap_extended_long,
    pub /: *mut *mut u32 count:4; / number of link controllers,
    pub unknown1:4: u32,
    pub /*: *mut u32 base_offset:8;,
// offset (into this capability) of the configuration
// area of the first link controller
//
    pub /: *mut *mut u32 length:12; / link controller configuration area length,
    pub /: *mut *mut u32 unknown2:4; / TODO check that length is correct,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_cap_phy {
    pub cap_header: tb_cap_basic,
    pub unknown1:16: u32,
    pub unknown2:14: u32,
    pub disable:1: bool,
    pub unknown3:11: u32,
    pub state:4: tb_port_state,
    pub unknown4:2: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_eeprom_ctl {
    pub /: *mut *mut bool fl_sk:1; / send pulse to transfer one bit,
    pub /: *mut *mut bool fl_cs:1; / set to 0 before access,
    pub /: *mut *mut bool fl_di:1; / to eeprom,
    pub /: *mut *mut bool fl_do:1; / from eeprom,
    pub /: *mut *mut bool bit_banging_enable:1; / set to 1 before access,
    pub /: *mut *mut bool not_present:1; / should be 0,
    pub unknown1:1: bool,
    pub /: *mut *mut bool present:1; / should be 1,
    pub unknown2:24: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_cap_plug_events {
    pub cap_header: tb_cap_extended_short,
    pub /: *mut *mut u32 __unknown1:2; / VSC_CS_1,
    pub /: *mut *mut u32 plug_events:5; / VSC_CS_1,
    pub /: *mut *mut u32 __unknown2:25; / VSC_CS_1,
    pub vsc_cs_2: u32,
    pub vsc_cs_3: u32,
    pub eeprom_ctl: tb_eeprom_ctl,
    pub /: *mut *mut u32 __unknown5[7]; / VSC_CS_5 -> VSC_CS_11,
    pub /: *mut *mut u32 drom_offset; / VSC_CS_12: 32 bit register, but eeprom addresses are 16 bit,
    pub __packed: },
// device headers
// Present on port 0 in TB_CFG_SWITCH at address zero.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_regs_switch_header {
// DWORD 0
    pub vendor_id: u16,
    pub device_id: u16,
// DWORD 1
    pub first_cap_offset:8: u32,
    pub upstream_port_number:6: u32,
    pub max_port_number:6: u32,
    pub depth:3: u32,
    pub __unknown1:1: u32,
    pub revision:8: u32,
// DWORD 2
    pub route_lo: u32,
// DWORD 3
    pub route_hi:31: u32,
    pub enabled:1: bool,
// DWORD 4
    pub /*: *mut u32 plug_events_delay:8;,
// RW, pause between plug events in
// milliseconds.
//
    pub cmuv:8: u32,
    pub __unknown4:8: u32,
    pub thunderbolt_version:8: u32,
    pub __packed: },
// Used with the router thunderbolt_version

pub const ROUTER_CS_1: c_uint = 0x01;
pub const ROUTER_CS_3: c_uint = 0x03;

pub const ROUTER_CS_4: c_uint = 0x04;
// Used with the router cmuv field
pub const ROUTER_CS_4_CMUV_V1: c_uint = 0x10;
pub const ROUTER_CS_4_CMUV_V2: c_uint = 0x20;
pub const ROUTER_CS_5: c_uint = 0x05;

pub const ROUTER_CS_6: c_uint = 0x06;

pub const ROUTER_CS_7: c_uint = 0x07;
pub const ROUTER_CS_9: c_uint = 0x09;
pub const ROUTER_CS_25: c_uint = 0x19;
pub const ROUTER_CS_26: c_uint = 0x1a;

pub const ROUTER_CS_26_STATUS_SHIFT: c_int = 24;

// USB4 router operations opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb4_switch_op {
    USB4_SWITCH_OP_QUERY_DP_RESOURCE = 0x10,
    USB4_SWITCH_OP_ALLOC_DP_RESOURCE = 0x11,
    USB4_SWITCH_OP_DEALLOC_DP_RESOURCE = 0x12,
    USB4_SWITCH_OP_NVM_WRITE = 0x20,
    USB4_SWITCH_OP_NVM_AUTH = 0x21,
    USB4_SWITCH_OP_NVM_READ = 0x22,
    USB4_SWITCH_OP_NVM_SET_OFFSET = 0x23,
    USB4_SWITCH_OP_DROM_READ = 0x24,
    USB4_SWITCH_OP_NVM_SECTOR_SIZE = 0x25,
    USB4_SWITCH_OP_BUFFER_ALLOC = 0x33,
}

// Router TMU configuration
pub const TMU_RTR_CS_0: c_uint = 0x00;

pub const TMU_RTR_CS_1: c_uint = 0x01;

pub const TMU_RTR_CS_1_LOCAL_TIME_NS_SHIFT: c_int = 16;
pub const TMU_RTR_CS_2: c_uint = 0x02;
pub const TMU_RTR_CS_3: c_uint = 0x03;

pub const TMU_RTR_CS_3_TS_PACKET_INTERVAL_SHIFT: c_int = 16;
pub const TMU_RTR_CS_15: c_uint = 0x0f;

pub const TMU_RTR_CS_18: c_uint = 0x12;

pub const TMU_RTR_CS_22: c_uint = 0x16;
pub const TMU_RTR_CS_24: c_uint = 0x18;
pub const TMU_RTR_CS_25: c_uint = 0x19;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_port_type {
    TB_TYPE_INACTIVE	= 0x000000,
    TB_TYPE_PORT		= 0x000001,
    TB_TYPE_NHI		= 0x000002,
// TB_TYPE_ETHERNET	= 0x020000, lower order bits are not known
// TB_TYPE_SATA		= 0x080000, lower order bits are not known
    TB_TYPE_DP_HDMI_IN	= 0x0e0101,
    TB_TYPE_DP_HDMI_OUT	= 0x0e0102,
    TB_TYPE_PCIE_DOWN	= 0x100101,
    TB_TYPE_PCIE_UP		= 0x100102,
    TB_TYPE_USB3_DOWN	= 0x200101,
    TB_TYPE_USB3_UP		= 0x200102,
}

// Present on every port in TB_CF_PORT at address zero.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_regs_port_header {
// DWORD 0
    pub vendor_id: u16,
    pub device_id: u16,
// DWORD 1
    pub first_cap_offset:8: u32,
    pub max_counters:11: u32,
    pub counters_support:1: u32,
    pub __unknown1:4: u32,
    pub revision:8: u32,
// DWORD 2
    pub type:24: tb_port_type,
    pub thunderbolt_version:8: u32,
// DWORD 3
    pub __unknown2:20: u32,
    pub port_number:6: u32,
    pub __unknown3:6: u32,
// DWORD 4
    pub nfc_credits: u32,
// DWORD 5
    pub max_in_hop_id:11: u32,
    pub max_out_hop_id:11: u32,
    pub __unknown4:10: u32,
// DWORD 6
    pub __unknown5: u32,
// DWORD 7
    pub __unknown6: u32,
    pub __packed: },
// Basic adapter configuration registers
pub const ADP_CS_4: c_uint = 0x04;

pub const ADP_CS_4_TOTAL_BUFFERS_SHIFT: c_int = 20;

pub const ADP_CS_5: c_uint = 0x05;

pub const ADP_CS_5_LCA_SHIFT: c_int = 22;

// TMU adapter registers
pub const TMU_ADP_CS_3: c_uint = 0x03;

pub const TMU_ADP_CS_6: c_uint = 0x06;

pub const TMU_ADP_CS_8: c_uint = 0x08;

pub const TMU_ADP_CS_9: c_uint = 0x09;

// Lane adapter registers
pub const LANE_ADP_CS_0: c_uint = 0x00;

pub const LANE_ADP_CS_0_SUPPORTED_SPEED_SHIFT: c_int = 16;

pub const LANE_ADP_CS_0_SUPPORTED_WIDTH_SHIFT: c_int = 20;
pub const LANE_ADP_CS_0_SUPPORTED_WIDTH_DUAL: c_uint = 0x2;

pub const LANE_ADP_CS_1: c_uint = 0x01;

pub const LANE_ADP_CS_1_TARGET_SPEED_GEN3: c_uint = 0xc;

pub const LANE_ADP_CS_1_TARGET_WIDTH_SHIFT: c_int = 4;
pub const LANE_ADP_CS_1_TARGET_WIDTH_SINGLE: c_uint = 0x1;
pub const LANE_ADP_CS_1_TARGET_WIDTH_DUAL: c_uint = 0x3;

pub const LANE_ADP_CS_1_TARGET_WIDTH_ASYM_TX: c_uint = 0x1;
pub const LANE_ADP_CS_1_TARGET_WIDTH_ASYM_RX: c_uint = 0x2;
pub const LANE_ADP_CS_1_TARGET_WIDTH_ASYM_DUAL: c_uint = 0x0;

pub const LANE_ADP_CS_1_CURRENT_SPEED_SHIFT: c_int = 16;
pub const LANE_ADP_CS_1_CURRENT_SPEED_GEN2: c_uint = 0x8;
pub const LANE_ADP_CS_1_CURRENT_SPEED_GEN3: c_uint = 0x4;
pub const LANE_ADP_CS_1_CURRENT_SPEED_GEN4: c_uint = 0x2;

pub const LANE_ADP_CS_1_CURRENT_WIDTH_SHIFT: c_int = 20;

// USB4 port registers
pub const PORT_CS_1: c_uint = 0x01;
pub const PORT_CS_1_LENGTH_SHIFT: c_int = 8;

pub const PORT_CS_1_TARGET_SHIFT: c_int = 16;
pub const PORT_CS_1_RETIMER_INDEX_SHIFT: c_int = 20;

pub const PORT_CS_2: c_uint = 0x02;
pub const PORT_CS_18: c_uint = 0x12;

pub const PORT_CS_19: c_uint = 0x13;

// Display Port adapter registers
pub const ADP_DP_CS_0: c_uint = 0x00;

pub const ADP_DP_CS_0_VIDEO_HOPID_SHIFT: c_int = 16;

pub const ADP_DP_CS_1_AUX_RX_HOPID_SHIFT: c_int = 11;
pub const ADP_DP_CS_2: c_uint = 0x02;

pub const ADP_DP_CS_2_NRD_MLR_SHIFT: c_int = 7;

pub const ADP_DP_CS_2_GR_SHIFT: c_int = 11;
pub const ADP_DP_CS_2_GR_0_25G: c_uint = 0x0;
pub const ADP_DP_CS_2_GR_0_5G: c_uint = 0x1;
pub const ADP_DP_CS_2_GR_1G: c_uint = 0x2;

pub const ADP_DP_CS_2_GROUP_ID_SHIFT: c_int = 13;

pub const ADP_DP_CS_2_CM_ID_SHIFT: c_int = 16;

pub const ADP_DP_CS_2_ESTIMATED_BW_SHIFT: c_int = 24;
pub const ADP_DP_CS_3: c_uint = 0x03;

pub const DP_LOCAL_CAP: c_uint = 0x04;
pub const DP_REMOTE_CAP: c_uint = 0x05;
// For DP IN adapter
pub const DP_STATUS: c_uint = 0x06;

pub const DP_STATUS_ALLOCATED_BW_SHIFT: c_int = 24;
// For DP OUT adapter
pub const DP_STATUS_CTRL: c_uint = 0x06;

pub const DP_COMMON_CAP: c_uint = 0x07;
// Only if DP IN supports BW allocation mode
pub const ADP_DP_CS_8: c_uint = 0x08;

//
// DP_COMMON_CAP offsets work also for DP_LOCAL_CAP and DP_REMOTE_CAP
// with exception of DPRX done.
//

pub const DP_COMMON_CAP_RATE_SHIFT: c_int = 8;
pub const DP_COMMON_CAP_RATE_RBR: c_uint = 0x0;
pub const DP_COMMON_CAP_RATE_HBR: c_uint = 0x1;
pub const DP_COMMON_CAP_RATE_HBR2: c_uint = 0x2;
pub const DP_COMMON_CAP_RATE_HBR3: c_uint = 0x3;

pub const DP_COMMON_CAP_LANES_SHIFT: c_int = 12;
pub const DP_COMMON_CAP_1_LANE: c_uint = 0x0;
pub const DP_COMMON_CAP_2_LANES: c_uint = 0x1;
pub const DP_COMMON_CAP_4_LANES: c_uint = 0x2;

// Only present if DP IN supports BW allocation mode
pub const ADP_DP_CS_8: c_uint = 0x08;

// PCIe adapter registers
pub const ADP_PCIE_CS_0: c_uint = 0x00;

pub const ADP_PCIE_CS_1: c_uint = 0x01;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_pcie_ltssm_state {
    USB4_PCIE_LTSSM_DETECT,
    USB4_PCIE_LTSSM_POLLING,
    USB4_PCIE_LTSSM_CONFIG,
    USB4_PCIE_LTSSM_CONFIG_IDLE,
    USB4_PCIE_LTSSM_RECOVERY,
    USB4_PCIE_LTSSM_RECOVERY_IDLE,
    USB4_PCIE_LTSSM_L0,
    USB4_PCIE_LTSSM_L1,
    USB4_PCIE_LTSSM_L2,
    USB4_PCIE_LTSSM_DISABLED,
    USB4_PCIE_LTSSM_HOT_RESET,
}

// USB adapter registers
pub const ADP_USB3_CS_0: c_uint = 0x00;

pub const ADP_USB3_CS_1: c_uint = 0x01;

pub const ADP_USB3_CS_1_CDBW_SHIFT: c_int = 12;

pub const ADP_USB3_CS_2: c_uint = 0x02;

pub const ADP_USB3_CS_2_ADBW_SHIFT: c_int = 12;

pub const ADP_USB3_CS_3: c_uint = 0x03;

pub const ADP_USB3_CS_4: c_uint = 0x04;

pub const ADP_USB3_CS_4_MSLR_SHIFT: c_int = 12;
pub const ADP_USB3_CS_4_MSLR_20G: c_uint = 0x1;
// Hop register from TB_CFG_HOPS. 8 byte per entry.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_regs_hop {
// DWORD 0
    pub /*: *mut u32 next_hop:11;,
// hop to take after sending the packet through
// out_port (on the incoming port of the next switch)
//
    pub /: *mut *mut u32 out_port:6; / next port of the path (on the same switch),
    pub initial_credits:7: u32,
    pub pmps:1: u32,
    pub /: *mut *mut u32 unknown1:6; / set to zero,
    pub enable:1: bool,
// DWORD 1
    pub weight:4: u32,
    pub /: *mut *mut u32 unknown2:4; / set to zero,
    pub priority:3: u32,
    pub drop_packages:1: bool,
    pub /: *mut *mut u32 counter:11; / index into TB_CFG_COUNTERS on this port,
    pub counter_enable:1: bool,
    pub ingress_fc:1: bool,
    pub egress_fc:1: bool,
    pub ingress_shared_buffer:1: bool,
    pub egress_shared_buffer:1: bool,
    pub pending:1: bool,
    pub /: *mut *mut u32 unknown3:3; / set to zero,
    pub __packed: },
// TMU Thunderbolt 3 registers
pub const TB_TIME_VSEC_3_CS_9: c_uint = 0x9;

pub const TB_TIME_VSEC_3_CS_26: c_uint = 0x1a;

//
// Used for Titan Ridge only. Bits are part of the same register: TMU_ADP_CS_6
// (see above) as in USB4 spec, but these specific bits are used for Titan Ridge
// only and are reserved in USB4 spec.
//

// Plug Events registers

pub const TB_PLUG_EVENTS_PCIE_WR_DATA: c_uint = 0x1b;
pub const TB_PLUG_EVENTS_PCIE_CMD: c_uint = 0x1c;

pub const TB_PLUG_EVENTS_PCIE_CMD_BR_SHIFT: c_int = 10;

pub const TB_PLUG_EVENTS_PCIE_CMD_WR: c_uint = 0x1;
pub const TB_PLUG_EVENTS_PCIE_CMD_COMMAND_SHIFT: c_int = 22;

pub const TB_PLUG_EVENTS_PCIE_CMD_COMMAND_VAL: c_uint = 0x2;

pub const TB_PLUG_EVENTS_PCIE_CMD_RD_DATA: c_uint = 0x1d;
// CP Low Power registers
pub const TB_LOW_PWR_C1_CL1: c_uint = 0x1;

pub const TB_LOW_PWR_C3_CL1: c_uint = 0x3;
// Common link controller registers
pub const TB_LC_DESC: c_uint = 0x02;

pub const TB_LC_DESC_SIZE_SHIFT: c_int = 8;

pub const TB_LC_DESC_PORT_SIZE_SHIFT: c_int = 16;

pub const TB_LC_FUSE: c_uint = 0x03;
pub const TB_LC_SNK_ALLOCATION: c_uint = 0x10;

pub const TB_LC_SNK_ALLOCATION_SNK0_CM: c_uint = 0x1;
pub const TB_LC_SNK_ALLOCATION_SNK1_SHIFT: c_int = 4;

pub const TB_LC_SNK_ALLOCATION_SNK1_CM: c_uint = 0x1;
pub const TB_LC_POWER: c_uint = 0x740;
// Link controller registers
pub const TB_LC_PORT_MODE: c_uint = 0x26;

pub const TB_LC_CS_42: c_uint = 0x2a;

pub const TB_LC_PORT_ATTR: c_uint = 0x8d;

pub const TB_LC_SX_CTRL: c_uint = 0x96;

pub const TB_LC_LINK_ATTR: c_uint = 0x97;

pub const TB_LC_LINK_REQ: c_uint = 0xad;

