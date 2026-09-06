//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/mad.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2015 - 2017 Intel Corporation.
//

//
// OPA Traps
//

//
// Generic trap/notice other local changes flags (trap 144).
//
pub const OPA_NOTICE_TRAP_LWDE_CHG: c_uint = 0x08 /* Link Width Downgrade Enable;
// changed
//
pub const OPA_NOTICE_TRAP_LSE_CHG: c_uint = 0x04 /* Link Speed Enable changed */;
pub const OPA_NOTICE_TRAP_LWE_CHG: c_uint = 0x02 /* Link Width Enable changed */;
pub const OPA_NOTICE_TRAP_NODE_DESC_CHG: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_mad_notice_attr {
    pub generic_type: u8,
    pub prod_type_msb: u8,
    pub prod_type_lsb: __be16,
    pub trap_num: __be16,
    pub toggle_count: __be16,
    pub issuer_lid: __be32,
    pub reserved1: __be32,
    pub issuer_gid: ib_gid,
    pub details: [u8; 64],
    pub raw_data: },
    pub gid: ib_gid,
    pub ntc_64_65_66_67: } __packed,
    pub lid: __be32,
    pub ntc_128: } __packed,
    pub /: *mut *mut __be32 lid; / where violation happened,
    pub /: *mut *mut u8 port_num; / where violation happened,
    pub ntc_129_130_131: } __packed,
    pub /: *mut *mut __be32 lid; / LID where change occurred,
    pub /: *mut *mut __be32 new_cap_mask; / new capability mask,
    pub reserved2: __be16,
    pub cap_mask3: __be16,
    pub /: *mut *mut __be16 change_flags; / low 4 bits only,
    pub ntc_144: } __packed,
    pub new_sys_guid: __be64,
    pub /: *mut *mut __be32 lid; / lid where sys guid changed,
    pub ntc_145: } __packed,
    pub lid: __be32,
    pub dr_slid: __be32,
    pub method: u8,
    pub dr_trunc_hop: u8,
    pub attr_id: __be16,
    pub attr_mod: __be32,
    pub mkey: __be64,
    pub dr_rtn_path: [u8; 30],
    pub ntc_256: } __packed,
    pub lid1: __be32,
    pub lid2: __be32,
    pub key: __be32,
    pub /: *mut *mut u8 sl; / SL: high 5 bits,
    pub reserved3: [u8; 3],
    pub gid1: ib_gid,
    pub gid2: ib_gid,
    pub /: *mut *mut __be32 qp1; / high 8 bits reserved,
    pub /: *mut *mut __be32 qp2; / high 8 bits reserved,
    pub ntc_257_258: } __packed,
    pub /: *mut *mut __be16 flags; / low 8 bits reserved,
    pub pkey: __be16,
    pub lid1: __be32,
    pub lid2: __be32,
    pub /: *mut *mut u8 sl; / SL: high 5 bits,
    pub reserved4: [u8; 3],
    pub gid1: ib_gid,
    pub gid2: ib_gid,
    pub /: *mut *mut __be32 qp1; / high 8 bits reserved,
    pub /: *mut *mut __be32 qp2; / high 8 bits reserved,
    pub ntc_259: } __packed,
    pub lid: __be32,
    pub ntc_2048: } __packed,
}

pub const IB_VLARB_LOWPRI_0_31: c_int = 1;
pub const IB_VLARB_LOWPRI_32_63: c_int = 2;
pub const IB_VLARB_HIGHPRI_0_31: c_int = 3;
pub const IB_VLARB_HIGHPRI_32_63: c_int = 4;
pub const OPA_MAX_PREEMPT_CAP: c_int = 32;
pub const OPA_VLARB_LOW_ELEMENTS: c_int = 0;
pub const OPA_VLARB_HIGH_ELEMENTS: c_int = 1;
pub const OPA_VLARB_PREEMPT_ELEMENTS: c_int = 2;
pub const OPA_VLARB_PREEMPT_MATRIX: c_int = 3;

pub const LINK_SPEED_25G: c_int = 1;
pub const LINK_SPEED_12_5G: c_int = 2;
pub const LINK_WIDTH_DEFAULT: c_int = 4;
pub const DECIMAL_FACTORING: c_int = 1000;
//
// The default link width is multiplied by 1000
// to get accurate value after division.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_pma_portcounters_cong {
    pub reserved: u8,
    pub reserved1: u8,
    pub port_check_rate: __be16,
    pub symbol_error_counter: __be16,
    pub link_error_recovery_counter: u8,
    pub link_downed_counter: u8,
    pub port_rcv_errors: __be16,
    pub port_rcv_remphys_errors: __be16,
    pub port_rcv_switch_relay_errors: __be16,
    pub port_xmit_discards: __be16,
    pub port_xmit_constraint_errors: u8,
    pub port_rcv_constraint_errors: u8,
    pub reserved2: u8,
    pub /: *mut *mut u8 link_overrun_errors; / LocalLink: 7:4, BufferOverrun: 3:0,
    pub reserved3: __be16,
    pub vl15_dropped: __be16,
    pub port_xmit_data: __be64,
    pub port_rcv_data: __be64,
    pub port_xmit_packets: __be64,
    pub port_rcv_packets: __be64,
    pub port_xmit_wait: __be64,
    pub port_adr_events: __be64,
    pub __packed: },

pub const OPA_MAX_PREEMPT_CAP: c_int = 32;
pub const OPA_VLARB_LOW_ELEMENTS: c_int = 0;
pub const OPA_VLARB_HIGH_ELEMENTS: c_int = 1;
pub const OPA_VLARB_PREEMPT_ELEMENTS: c_int = 2;
pub const OPA_VLARB_PREEMPT_MATRIX: c_int = 3;
pub const HFI1_XMIT_RATE_UNSUPPORTED: c_uint = 0x0;
pub const HFI1_XMIT_RATE_PICO: c_uint = 0x7;
// number of 4nsec cycles equaling 2secs
pub const HFI1_CONG_TIMER_PSINTERVAL: c_uint = 0x1DCD64EC;
pub const IB_CC_SVCTYPE_RC: c_uint = 0x0;
pub const IB_CC_SVCTYPE_UC: c_uint = 0x1;
pub const IB_CC_SVCTYPE_RD: c_uint = 0x2;
pub const IB_CC_SVCTYPE_UD: c_uint = 0x3;
//
// There should be an equivalent IB #define for the following, but
// I cannot find it.
//
pub const OPA_CC_LOG_TYPE_HFI: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_hfi1_cong_log_event_internal {
    pub lqpn: u32,
    pub rqpn: u32,
    pub sl: u8,
    pub svc_type: u8,
    pub rlid: u32,
    pub /: *mut *mut u64 timestamp; / wider than 32 bits to detect 32 bit rollover,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_hfi1_cong_log_event {
    pub local_qp_cn_entry: [u8; 3],
    pub remote_qp_number_cn_entry: [u8; 3],
    pub /: *mut *mut u8 sl_svc_type_cn_entry; / 5 bits SL, 3 bits svc type,
    pub reserved: u8,
    pub remote_lid_cn_entry: __be32,
    pub timestamp_cn_entry: __be32,
    pub __packed: },
pub const OPA_CONG_LOG_ELEMS: c_int = 96;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_hfi1_cong_log {
    pub log_type: u8,
    pub congestion_flags: u8,
    pub threshold_event_counter: __be16,
    pub current_time_stamp: __be32,
    pub 8]: u8 threshold_cong_event_map[OPA_MAX_SLS /,
    pub events: [opa_hfi1_cong_log_event; OPA_CONG_LOG_ELEMS],
    pub __packed: },
pub const IB_CC_TABLE_CAP_DEFAULT: c_int = 31;
// Port control flags
pub const IB_CC_CCS_PC_SL_BASED: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_congestion_setting_entry {
    pub ccti_increase: u8,
    pub reserved: u8,
    pub ccti_timer: __be16,
    pub trigger_threshold: u8,
    pub /: *mut *mut u8 ccti_min; / min CCTI for cc table,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_congestion_setting_entry_shadow {
    pub ccti_increase: u8,
    pub reserved: u8,
    pub ccti_timer: u16,
    pub trigger_threshold: u8,
    pub /: *mut *mut u8 ccti_min; / min CCTI for cc table,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_congestion_setting_attr {
    pub control_map: __be32,
    pub port_control: __be16,
    pub entries: [opa_congestion_setting_entry; OPA_MAX_SLS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_congestion_setting_attr_shadow {
    pub control_map: u32,
    pub port_control: u16,
    pub entries: [opa_congestion_setting_entry_shadow; OPA_MAX_SLS],
    pub __packed: },
pub const IB_CC_TABLE_ENTRY_INCREASE_DEFAULT: c_int = 1;
pub const IB_CC_TABLE_ENTRY_TIMER_DEFAULT: c_int = 1;
// 64 Congestion Control table entries in a single MAD
pub const IB_CCT_ENTRIES: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cc_table_entry {
    pub /: *mut *mut __be16 entry; / shift:2, multiplier:14,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cc_table_entry_shadow {
    pub /: *mut *mut u16 entry; / shift:2, multiplier:14,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cc_table_attr {
    pub /: *mut *mut __be16 ccti_limit; / max CCTI for cc table,
    pub ccti_entries: [ib_cc_table_entry; IB_CCT_ENTRIES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cc_table_attr_shadow {
    pub /: *mut *mut u16 ccti_limit; / max CCTI for cc table,
    pub ccti_entries: [ib_cc_table_entry_shadow; IB_CCT_ENTRIES],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc_table_shadow {
    pub /: *mut *mut u16 ccti_limit; / max CCTI for cc table,
    pub entries: [ib_cc_table_entry_shadow; CC_TABLE_SHADOW_MAX],
    pub __packed: },
//
// struct cc_state combines the (active) per-port congestion control
// table, and the (active) per-SL congestion settings. cc_state data
// may need to be read in code paths that we want to be fast, so it
// is an RCU protected structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc_state {
    pub rcu: rcu_head,
    pub cct: cc_table_shadow,
    pub cong_setting: opa_congestion_setting_attr_shadow,
}

//
// OPA BufferControl MAD
//
// attribute modifier macros
pub const OPA_AM_NPORT_SHIFT: c_int = 24;
pub const OPA_AM_NPORT_MASK: c_uint = 0xff;

pub const OPA_AM_NBLK_SHIFT: c_int = 24;
pub const OPA_AM_NBLK_MASK: c_uint = 0xff;

pub const OPA_AM_START_BLK_SHIFT: c_int = 0;
pub const OPA_AM_START_BLK_MASK: c_uint = 0xff;

pub const OPA_AM_PORTNUM_SHIFT: c_int = 0;
pub const OPA_AM_PORTNUM_MASK: c_uint = 0xff;

pub const OPA_AM_ASYNC_SHIFT: c_int = 12;
pub const OPA_AM_ASYNC_MASK: c_uint = 0x1;

pub const OPA_AM_START_SM_CFG_SHIFT: c_int = 9;
pub const OPA_AM_START_SM_CFG_MASK: c_uint = 0x1;

pub const OPA_AM_CI_ADDR_SHIFT: c_int = 19;
pub const OPA_AM_CI_ADDR_MASK: c_uint = 0xfff;

pub const OPA_AM_CI_LEN_SHIFT: c_int = 13;
pub const OPA_AM_CI_LEN_MASK: c_uint = 0x3f;

// error info macros
pub const OPA_EI_STATUS_SMASK: c_uint = 0x80;
pub const OPA_EI_CODE_SMASK: c_uint = 0x0f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vl_limit {
    pub dedicated: __be16,
    pub shared: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct buffer_control {
    pub reserved: __be16,
    pub overall_shared_limit: __be16,
    pub vl: [vl_limit; OPA_MAX_VLS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sc2vlnt {
    pub /: *mut *mut u8 vlnt[32]; / 5 bit VL, 3 bits reserved,
}

//
// The PortSamplesControl.CounterMasks field is an array of 3 bit fields
// which specify the N'th counter's capabilities. See ch. 16.1.3.2.
// We support 5 counters which only count the mandatory quantities.
//

extern "C" {
    pub fn hfi1_event_pkey_change(dd: *mut hfi1_devdata, port: u32);
}
extern "C" {
    pub fn hfi1_handle_trap_timer(t: *mut timer_list);
}
extern "C" {
    pub fn tx_link_width(link_width: u16) -> u16;
}
//
// get_link_speed - determine whether 12.5G or 25G speed
// @link_speed: the speed of active link
// @return: Return 2 if link speed identified as 12.5G
// or return 1 if link speed is 25G.
//
// The function indirectly calculate required link speed
// value for convert_xmit_counter function. If the link
// speed is 25G, the function return as 1 as it is required
// by xmit counter conversion formula :-( 25G / link_speed).
// This conversion will provide value 1 if current
// link speed is 25G or 2 if 12.5G.This is done to avoid
// 12.5 float number conversion.
//
// convert_xmit_counter - calculate flit times for given xmit counter
// value
// @xmit_wait_val: current xmit counter value
// @link_width: width of active link
// @link_speed: speed of active link
// @return: return xmit counter value in flit times.
//
// link_speed) / DECIMAL_FACTORING;
