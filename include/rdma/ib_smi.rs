//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/ib_smi.h
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
// Copyright (c) 2004 Mellanox Technologies Ltd.  All rights reserved.
// Copyright (c) 2004 Infinicon Corporation.  All rights reserved.
// Copyright (c) 2004 Intel Corporation.  All rights reserved.
// Copyright (c) 2004 Topspin Corporation.  All rights reserved.
// Copyright (c) 2004 Voltaire Corporation.  All rights reserved.
//

pub const IB_SMP_DATA_SIZE: c_int = 64;
pub const IB_SMP_MAX_PATH_HOPS: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_smp {
    pub base_version: u8,
    pub mgmt_class: u8,
    pub class_version: u8,
    pub method: u8,
    pub status: __be16,
    pub hop_ptr: u8,
    pub hop_cnt: u8,
    pub tid: __be64,
    pub attr_id: __be16,
    pub resv: __be16,
    pub attr_mod: __be32,
    pub mkey: __be64,
    pub dr_slid: __be16,
    pub dr_dlid: __be16,
    pub reserved: [u8; 28],
    pub data: [u8; IB_SMP_DATA_SIZE],
    pub initial_path: [u8; IB_SMP_MAX_PATH_HOPS],
    pub return_path: [u8; IB_SMP_MAX_PATH_HOPS],
    pub __packed: },

// Subnet management attributes

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_port_info {
    pub mkey: __be64,
    pub gid_prefix: __be64,
    pub lid: __be16,
    pub sm_lid: __be16,
    pub cap_mask: __be32,
    pub diag_code: __be16,
    pub mkey_lease_period: __be16,
    pub local_port_num: u8,
    pub link_width_enabled: u8,
    pub link_width_supported: u8,
    pub link_width_active: u8,
    pub /: *mut *mut u8 linkspeed_portstate; / 4 bits, 4 bits,
    pub /: *mut *mut u8 portphysstate_linkdown; / 4 bits, 4 bits,
    pub /: *mut *mut u8 mkeyprot_resv_lmc; / 2 bits, 3, 3,
    pub /: *mut *mut u8 linkspeedactive_enabled; / 4 bits, 4 bits,
    pub /: *mut *mut u8 neighbormtu_mastersmsl; / 4 bits, 4 bits,
    pub /: *mut *mut u8 vlcap_inittype; / 4 bits, 4 bits,
    pub vl_high_limit: u8,
    pub vl_arb_high_cap: u8,
    pub vl_arb_low_cap: u8,
    pub /: *mut *mut u8 inittypereply_mtucap; / 4 bits, 4 bits,
    pub /: *mut *mut u8 vlstallcnt_hoqlife; / 3 bits, 5 bits,
    pub /: *mut *mut u8 operationalvl_pei_peo_fpi_fpo; / 4 bits, 1, 1, 1, 1,
    pub mkey_violations: __be16,
    pub pkey_violations: __be16,
    pub qkey_violations: __be16,
    pub guid_cap: u8,
    pub /: *mut *mut u8 clientrereg_resv_subnetto; / 1 bit, 2 bits, 5,
    pub /: *mut *mut u8 resv_resptimevalue; / 3 bits, 5 bits,
    pub /: *mut *mut u8 localphyerrors_overrunerrors; / 4 bits, 4 bits,
    pub max_credit_hint: __be16,
    pub resv: u8,
    pub link_roundtrip_latency: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_node_info {
    pub base_version: u8,
    pub class_version: u8,
    pub node_type: u8,
    pub num_ports: u8,
    pub sys_guid: __be64,
    pub node_guid: __be64,
    pub port_guid: __be64,
    pub partition_cap: __be16,
    pub device_id: __be16,
    pub revision: __be32,
    pub local_port_num: u8,
    pub vendor_id: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_vl_weight_elem {
    pub /: *mut *mut u8 vl; / IB: VL is low 4 bits, upper 4 bits reserved,
// OPA: VL is low 5 bits, upper 3 bits reserved
    pub weight: u8,
}

//
// SM Trap/Notice numbers
//

//
// Other local changes flags (trap 144).
//
pub const IB_NOTICE_TRAP_LSE_CHG: c_uint = 0x04	/* Link Speed Enable changed */;
pub const IB_NOTICE_TRAP_LWE_CHG: c_uint = 0x02	/* Link Width Enable changed */;
pub const IB_NOTICE_TRAP_NODE_DESC_CHG: c_uint = 0x01;
//
// M_Key volation flags in dr_trunc_hop (trap 256).
//
pub const IB_NOTICE_TRAP_DR_NOTICE: c_uint = 0x80;
pub const IB_NOTICE_TRAP_DR_TRUNC: c_uint = 0x40;
//
// ib_init_query_mad - Initialize query MAD.
// @mad: MAD to initialize.
//
