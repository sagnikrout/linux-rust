//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/opa_port_info.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2014-2020 Intel Corporation.  All rights reserved.
//

// Link Down / Neighbor Link Down Reason; indicated as follows:

pub const OPA_LINKDOWN_REASON_RCV_ERROR_0: c_int = 1;
pub const OPA_LINKDOWN_REASON_BAD_PKT_LEN: c_int = 2;
pub const OPA_LINKDOWN_REASON_PKT_TOO_LONG: c_int = 3;
pub const OPA_LINKDOWN_REASON_PKT_TOO_SHORT: c_int = 4;
pub const OPA_LINKDOWN_REASON_BAD_SLID: c_int = 5;
pub const OPA_LINKDOWN_REASON_BAD_DLID: c_int = 6;
pub const OPA_LINKDOWN_REASON_BAD_L2: c_int = 7;
pub const OPA_LINKDOWN_REASON_BAD_SC: c_int = 8;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_8: c_int = 9;
pub const OPA_LINKDOWN_REASON_BAD_MID_TAIL: c_int = 10;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_10: c_int = 11;
pub const OPA_LINKDOWN_REASON_PREEMPT_ERROR: c_int = 12;
pub const OPA_LINKDOWN_REASON_PREEMPT_VL15: c_int = 13;
pub const OPA_LINKDOWN_REASON_BAD_VL_MARKER: c_int = 14;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_14: c_int = 15;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_15: c_int = 16;
pub const OPA_LINKDOWN_REASON_BAD_HEAD_DIST: c_int = 17;
pub const OPA_LINKDOWN_REASON_BAD_TAIL_DIST: c_int = 18;
pub const OPA_LINKDOWN_REASON_BAD_CTRL_DIST: c_int = 19;
pub const OPA_LINKDOWN_REASON_BAD_CREDIT_ACK: c_int = 20;
pub const OPA_LINKDOWN_REASON_UNSUPPORTED_VL_MARKER: c_int = 21;
pub const OPA_LINKDOWN_REASON_BAD_PREEMPT: c_int = 22;
pub const OPA_LINKDOWN_REASON_BAD_CONTROL_FLIT: c_int = 23;
pub const OPA_LINKDOWN_REASON_EXCEED_MULTICAST_LIMIT: c_int = 24;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_24: c_int = 25;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_25: c_int = 26;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_26: c_int = 27;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_27: c_int = 28;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_28: c_int = 29;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_29: c_int = 30;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_30: c_int = 31;
pub const OPA_LINKDOWN_REASON_EXCESSIVE_BUFFER_OVERRUN: c_int = 32;
pub const OPA_LINKDOWN_REASON_UNKNOWN: c_int = 33;
// 34 -reserved
pub const OPA_LINKDOWN_REASON_REBOOT: c_int = 35;
pub const OPA_LINKDOWN_REASON_NEIGHBOR_UNKNOWN: c_int = 36;
// 37-38 reserved
pub const OPA_LINKDOWN_REASON_FM_BOUNCE: c_int = 39;
pub const OPA_LINKDOWN_REASON_SPEED_POLICY: c_int = 40;
pub const OPA_LINKDOWN_REASON_WIDTH_POLICY: c_int = 41;
// 42-48 reserved
pub const OPA_LINKDOWN_REASON_DISCONNECTED: c_int = 49;
pub const OPA_LINKDOWN_REASON_LOCAL_MEDIA_NOT_INSTALLED: c_int = 50;
pub const OPA_LINKDOWN_REASON_NOT_INSTALLED: c_int = 51;
pub const OPA_LINKDOWN_REASON_CHASSIS_CONFIG: c_int = 52;
// 53 reserved
pub const OPA_LINKDOWN_REASON_END_TO_END_NOT_INSTALLED: c_int = 54;
// 55 reserved
pub const OPA_LINKDOWN_REASON_POWER_POLICY: c_int = 56;
pub const OPA_LINKDOWN_REASON_LINKSPEED_POLICY: c_int = 57;
pub const OPA_LINKDOWN_REASON_LINKWIDTH_POLICY: c_int = 58;
// 59 reserved
pub const OPA_LINKDOWN_REASON_SWITCH_MGMT: c_int = 60;
pub const OPA_LINKDOWN_REASON_SMA_DISABLED: c_int = 61;
// 62 reserved
pub const OPA_LINKDOWN_REASON_TRANSIENT: c_int = 63;
// 64-255 reserved
// OPA Link Init reason; indicated as follows:
// 3-7; 11-15 reserved; 8-15 cleared on Polling->LinkUp
pub const OPA_LINKINIT_REASON_NOP: c_int = 0;

pub const OPA_LINK_SPEED_NOP: c_uint = 0x0000  /* no change */;
pub const OPA_LINK_SPEED_12_5G: c_uint = 0x0001  /* 12.5 Gbps */;
pub const OPA_LINK_SPEED_25G: c_uint = 0x0002  /* 25.78125 Gbps */;
pub const OPA_LINK_SPEED_50G: c_uint = 0x0004  /* 53.125 Gbps */;
pub const OPA_LINK_SPEED_100G: c_uint = 0x0008  /* 106.25 Gbps */;
pub const OPA_LINK_WIDTH_1X: c_uint = 0x0001;
pub const OPA_LINK_WIDTH_2X: c_uint = 0x0002;
pub const OPA_LINK_WIDTH_3X: c_uint = 0x0004;
pub const OPA_LINK_WIDTH_4X: c_uint = 0x0008;

// reserved (1 << 2)

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port_info_field_masks {
// vl.cap
    OPA_PI_MASK_VL_CAP                        = 0x1F,
// port_states.ledenable_offlinereason
    OPA_PI_MASK_OFFLINE_REASON                = 0x0F,
    OPA_PI_MASK_LED_ENABLE                    = 0x40,
// port_states.unsleepstate_downdefstate
    OPA_PI_MASK_UNSLEEP_STATE                 = 0xF0,
    OPA_PI_MASK_DOWNDEF_STATE                 = 0x0F,
// port_states.portphysstate_portstate
    OPA_PI_MASK_PORT_PHYSICAL_STATE           = 0xF0,
    OPA_PI_MASK_PORT_STATE                    = 0x0F,
// port_phys_conf
    OPA_PI_MASK_PORT_PHYSICAL_CONF            = 0x0F,
// collectivemask_multicastmask
    OPA_PI_MASK_COLLECT_MASK                  = 0x38,
    OPA_PI_MASK_MULTICAST_MASK                = 0x07,
// mkeyprotect_lmc
    OPA_PI_MASK_MKEY_PROT_BIT                 = 0xC0,
    OPA_PI_MASK_LMC                           = 0x0F,
// smsl
    OPA_PI_MASK_SMSL                          = 0x1F,
// partenforce_filterraw
// Filter Raw In/Out bits 1 and 2 were removed
    OPA_PI_MASK_LINKINIT_REASON               = 0xF0,
    OPA_PI_MASK_PARTITION_ENFORCE_IN          = 0x08,
    OPA_PI_MASK_PARTITION_ENFORCE_OUT         = 0x04,
// operational_vls
    OPA_PI_MASK_OPERATIONAL_VL                = 0x1F,
// sa_qp
    OPA_PI_MASK_SA_QP                         = 0x00FFFFFF,
// sm_trap_qp
    OPA_PI_MASK_SM_TRAP_QP                    = 0x00FFFFFF,
// localphy_overrun_errors
    OPA_PI_MASK_LOCAL_PHY_ERRORS              = 0xF0,
    OPA_PI_MASK_OVERRUN_ERRORS                = 0x0F,
// clientrereg_subnettimeout
    OPA_PI_MASK_CLIENT_REREGISTER             = 0x80,
    OPA_PI_MASK_SUBNET_TIMEOUT                = 0x1F,
// port_link_mode
    OPA_PI_MASK_PORT_LINK_SUPPORTED           = (0x001F << 10),
    OPA_PI_MASK_PORT_LINK_ENABLED             = (0x001F <<  5),
    OPA_PI_MASK_PORT_LINK_ACTIVE              = (0x001F <<  0),
// port_link_crc_mode
    OPA_PI_MASK_PORT_LINK_CRC_SUPPORTED       = 0x0F00,
    OPA_PI_MASK_PORT_LINK_CRC_ENABLED         = 0x00F0,
    OPA_PI_MASK_PORT_LINK_CRC_ACTIVE          = 0x000F,
// port_mode
    OPA_PI_MASK_PORT_MODE_SECURITY_CHECK      = 0x0001,
    OPA_PI_MASK_PORT_MODE_16B_TRAP_QUERY      = 0x0002,
    OPA_PI_MASK_PORT_MODE_PKEY_CONVERT        = 0x0004,
    OPA_PI_MASK_PORT_MODE_SC2SC_MAPPING       = 0x0008,
    OPA_PI_MASK_PORT_MODE_VL_MARKER           = 0x0010,
    OPA_PI_MASK_PORT_PASS_THROUGH             = 0x0020,
    OPA_PI_MASK_PORT_ACTIVE_OPTOMIZE          = 0x0040,
// flit_control.interleave
    OPA_PI_MASK_INTERLEAVE_DIST_SUP           = (0x0003 << 12),
    OPA_PI_MASK_INTERLEAVE_DIST_ENABLE        = (0x0003 << 10),
    OPA_PI_MASK_INTERLEAVE_MAX_NEST_TX        = (0x001F <<  5),
    OPA_PI_MASK_INTERLEAVE_MAX_NEST_RX        = (0x001F <<  0),

// port_error_action
    OPA_PI_MASK_EX_BUFFER_OVERRUN                  = 0x80000000,
// 7 bits reserved
    OPA_PI_MASK_FM_CFG_ERR_EXCEED_MULTICAST_LIMIT  = 0x00800000,
    OPA_PI_MASK_FM_CFG_BAD_CONTROL_FLIT            = 0x00400000,
    OPA_PI_MASK_FM_CFG_BAD_PREEMPT                 = 0x00200000,
    OPA_PI_MASK_FM_CFG_UNSUPPORTED_VL_MARKER       = 0x00100000,
    OPA_PI_MASK_FM_CFG_BAD_CRDT_ACK                = 0x00080000,
    OPA_PI_MASK_FM_CFG_BAD_CTRL_DIST               = 0x00040000,
    OPA_PI_MASK_FM_CFG_BAD_TAIL_DIST               = 0x00020000,
    OPA_PI_MASK_FM_CFG_BAD_HEAD_DIST               = 0x00010000,
// 2 bits reserved
    OPA_PI_MASK_PORT_RCV_BAD_VL_MARKER             = 0x00002000,
    OPA_PI_MASK_PORT_RCV_PREEMPT_VL15              = 0x00001000,
    OPA_PI_MASK_PORT_RCV_PREEMPT_ERROR             = 0x00000800,
// 1 bit reserved
    OPA_PI_MASK_PORT_RCV_BAD_MidTail               = 0x00000200,
// 1 bit reserved
    OPA_PI_MASK_PORT_RCV_BAD_SC                    = 0x00000080,
    OPA_PI_MASK_PORT_RCV_BAD_L2                    = 0x00000040,
    OPA_PI_MASK_PORT_RCV_BAD_DLID                  = 0x00000020,
    OPA_PI_MASK_PORT_RCV_BAD_SLID                  = 0x00000010,
    OPA_PI_MASK_PORT_RCV_PKTLEN_TOOSHORT           = 0x00000008,
    OPA_PI_MASK_PORT_RCV_PKTLEN_TOOLONG            = 0x00000004,
    OPA_PI_MASK_PORT_RCV_BAD_PKTLEN                = 0x00000002,
    OPA_PI_MASK_PORT_RCV_BAD_LT                    = 0x00000001,

// pass_through.res_drctl
    OPA_PI_MASK_PASS_THROUGH_DR_CONTROL       = 0x01,

// buffer_units
    OPA_PI_MASK_BUF_UNIT_VL15_INIT            = (0x00000FFF  << 11),
    OPA_PI_MASK_BUF_UNIT_VL15_CREDIT_RATE     = (0x0000001F  <<  6),
    OPA_PI_MASK_BUF_UNIT_CREDIT_ACK           = (0x00000003  <<  3),
    OPA_PI_MASK_BUF_UNIT_BUF_ALLOC            = (0x00000003  <<  0),

// neigh_mtu.pvlx_to_mtu
    OPA_PI_MASK_NEIGH_MTU_PVL0                = 0xF0,
    OPA_PI_MASK_NEIGH_MTU_PVL1                = 0x0F,

// neigh_mtu.vlstall_hoq_life
    OPA_PI_MASK_VL_STALL                      = (0x03 << 5),
    OPA_PI_MASK_HOQ_LIFE                      = (0x1F << 0),

// port_neigh_mode
    OPA_PI_MASK_NEIGH_MGMT_ALLOWED            = (0x01 << 3),
    OPA_PI_MASK_NEIGH_FW_AUTH_BYPASS          = (0x01 << 2),
    OPA_PI_MASK_NEIGH_NODE_TYPE               = (0x03 << 0),

// resptime_value
    OPA_PI_MASK_RESPONSE_TIME_VALUE           = 0x1F,

// mtucap
    OPA_PI_MASK_MTU_CAP                       = 0x0F,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_port_states {
    pub reserved: u8,
    pub /: *mut *mut u8 ledenable_offlinereason; / 1 res, 1 bit, 6 bits,
    pub reserved2: u8,
    pub /: *mut *mut u8 portphysstate_portstate; / 4 bits, 4 bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_port_state_info {
    pub port_states: opa_port_states,
    pub link_width_downgrade_tx_active: __be16,
    pub link_width_downgrade_rx_active: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_port_info {
    pub lid: __be32,
    pub flow_control_mask: __be32,
    pub /: *mut *mut u8 res; / was inittype,
    pub /: *mut *mut u8 cap; / 3 res, 5 bits,
    pub high_limit: __be16,
    pub preempt_limit: __be16,
    pub arb_high_cap: u8,
    pub arb_low_cap: u8,
    pub vl: },
    pub port_states: opa_port_states,
    pub /: *mut *mut u8 port_phys_conf; / 4 res, 4 bits,
    pub /: *mut *mut u8 collectivemask_multicastmask; / 2 res, 3, 3,
    pub /: *mut *mut u8 mkeyprotect_lmc; / 2 bits, 2 res, 4 bits,
    pub /: *mut *mut u8 smsl; / 3 res, 5 bits,
    pub /: *mut *mut u8 partenforce_filterraw; / bit fields,
    pub /: *mut *mut u8 operational_vls; / 3 res, 5 bits,
    pub pkey_8b: __be16,
    pub pkey_10b: __be16,
    pub mkey_violations: __be16,
    pub pkey_violations: __be16,
    pub qkey_violations: __be16,
    pub /: *mut *mut __be32 sm_trap_qp; / 8 bits, 24 bits,
    pub /: *mut *mut __be32 sa_qp; / 8 bits, 24 bits,
    pub neigh_port_num: u8,
    pub link_down_reason: u8,
    pub neigh_link_down_reason: u8,
    pub /: *mut *mut u8 clientrereg_subnettimeout; / 1 bit, 2 bits, 5,
    pub supported: __be16,
    pub enabled: __be16,
    pub active: __be16,
    pub link_speed: },
    pub supported: __be16,
    pub enabled: __be16,
    pub active: __be16,
    pub link_width: },
    pub supported: __be16,
    pub enabled: __be16,
    pub tx_active: __be16,
    pub rx_active: __be16,
    pub link_width_downgrade: },
    pub /: *mut *mut __be16 port_link_mode; / 1 res, 5 bits, 5 bits, 5 bits,
    pub /: *mut *mut __be16 port_ltp_crc_mode; / 4 res, 4 bits, 4 bits, 4 bits,
    pub /: *mut *mut __be16 port_mode; / 9 res, bit fields,
    pub supported: __be16,
    pub enabled: __be16,
    pub port_packet_format: },
    pub /: *mut *mut __be16 interleave; / 2 res, 2,2,5,5,
    pub min_initial: __be16,
    pub min_tail: __be16,
    pub large_pkt_limit: u8,
    pub small_pkt_limit: u8,
    pub max_small_pkt_limit: u8,
    pub preemption_limit: u8,
    pub preemption: },
    pub flit_control: },
    pub reserved4: __be32,
    pub /: *mut *mut __be32 port_error_action; / bit field,
    pub egress_port: u8,
    pub /: *mut *mut u8 res_drctl; / 7 res, 1,
    pub pass_through: },
    pub mkey_lease_period: __be16,
    pub /: *mut *mut __be32 buffer_units; / 9 res, 12, 5, 3, 3,
    pub reserved5: __be32,
    pub sm_lid: __be32,
    pub mkey: __be64,
    pub subnet_prefix: __be64,
    pub /: *mut *mut u8 pvlx_to_mtu[OPA_MAX_VLS/2]; / 4 bits, 4 bits,
    pub neigh_mtu: },
    pub /: *mut *mut u8 vlstall_hoqlife; / 3 bits, 5 bits,
    pub xmit_q: [}; OPA_MAX_VLS],
    pub addr: [u8; 16],
    pub ipaddr_ipv6: },
    pub addr: [u8; 4],
    pub ipaddr_ipv4: },
    pub reserved6: u32,
    pub reserved7: u32,
    pub reserved8: u32,
    pub neigh_node_guid: __be64,
    pub ib_cap_mask: __be32,
    pub /: *mut *mut __be16 reserved9; / was ib_cap_mask2,
    pub opa_cap_mask: __be16,
    pub /: *mut *mut __be32 reserved10; / was link_roundtrip_latency,
    pub overall_buffer_space: __be16,
    pub /: *mut *mut __be16 reserved11; / was max_credit_hint,
    pub diag_code: __be16,
    pub buffer: u8,
    pub wire: u8,
    pub replay_depth: },
    pub port_neigh_mode: u8,
    pub /: *mut *mut u8 mtucap; / 4 res, 4 bits,
    pub /: *mut *mut u8 resptimevalue; / 3 res, 5 bits,
    pub local_port_num: u8,
    pub reserved12: u8,
    pub /: *mut *mut u8 reserved13; / was guid_cap,
    pub __packed: },
