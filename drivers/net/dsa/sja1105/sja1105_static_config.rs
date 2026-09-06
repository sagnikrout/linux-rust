//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/sja1105/sja1105_static_config.h
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


// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2016-2018 NXP
// Copyright (c) 2018-2019, Vladimir Oltean <olteanv@gmail.com>
//

pub const SJA1105_NUM_PORTS: c_int = 5;
pub const SJA1110_NUM_PORTS: c_int = 11;

pub const SJA1105_NUM_TC: c_int = 8;
pub const SJA1105_SIZE_SPI_MSG_HEADER: c_int = 4;

pub const SJA1105_SIZE_DEVICE_ID: c_int = 4;
pub const SJA1105_SIZE_TABLE_HEADER: c_int = 12;
pub const SJA1105_SIZE_SCHEDULE_ENTRY: c_int = 8;
pub const SJA1110_SIZE_SCHEDULE_ENTRY: c_int = 12;
pub const SJA1105_SIZE_SCHEDULE_ENTRY_POINTS_ENTRY: c_int = 4;
pub const SJA1110_SIZE_SCHEDULE_ENTRY_POINTS_ENTRY: c_int = 8;
pub const SJA1105_SIZE_VL_LOOKUP_ENTRY: c_int = 12;
pub const SJA1105_SIZE_VL_POLICING_ENTRY: c_int = 8;
pub const SJA1105_SIZE_VL_FORWARDING_ENTRY: c_int = 4;
pub const SJA1105_SIZE_L2_POLICING_ENTRY: c_int = 8;
pub const SJA1105_SIZE_VLAN_LOOKUP_ENTRY: c_int = 8;
pub const SJA1110_SIZE_VLAN_LOOKUP_ENTRY: c_int = 12;
pub const SJA1105_SIZE_L2_FORWARDING_ENTRY: c_int = 8;
pub const SJA1105_SIZE_L2_FORWARDING_PARAMS_ENTRY: c_int = 12;
pub const SJA1105_SIZE_RETAGGING_ENTRY: c_int = 8;
pub const SJA1105_SIZE_XMII_PARAMS_ENTRY: c_int = 4;
pub const SJA1110_SIZE_XMII_PARAMS_ENTRY: c_int = 8;
pub const SJA1105_SIZE_SCHEDULE_PARAMS_ENTRY: c_int = 12;
pub const SJA1105_SIZE_SCHEDULE_ENTRY_POINTS_PARAMS_ENTRY: c_int = 4;
pub const SJA1105_SIZE_VL_FORWARDING_PARAMS_ENTRY: c_int = 12;
pub const SJA1105ET_SIZE_L2_LOOKUP_ENTRY: c_int = 12;
pub const SJA1105ET_SIZE_MAC_CONFIG_ENTRY: c_int = 28;
pub const SJA1105ET_SIZE_L2_LOOKUP_PARAMS_ENTRY: c_int = 4;
pub const SJA1105ET_SIZE_GENERAL_PARAMS_ENTRY: c_int = 40;
pub const SJA1105ET_SIZE_AVB_PARAMS_ENTRY: c_int = 12;
pub const SJA1105ET_SIZE_CBS_ENTRY: c_int = 16;
pub const SJA1105PQRS_SIZE_L2_LOOKUP_ENTRY: c_int = 20;
pub const SJA1110_SIZE_L2_LOOKUP_ENTRY: c_int = 24;
pub const SJA1105PQRS_SIZE_MAC_CONFIG_ENTRY: c_int = 32;
pub const SJA1105PQRS_SIZE_L2_LOOKUP_PARAMS_ENTRY: c_int = 16;
pub const SJA1110_SIZE_L2_LOOKUP_PARAMS_ENTRY: c_int = 28;
pub const SJA1105PQRS_SIZE_GENERAL_PARAMS_ENTRY: c_int = 44;
pub const SJA1110_SIZE_GENERAL_PARAMS_ENTRY: c_int = 56;
pub const SJA1105PQRS_SIZE_AVB_PARAMS_ENTRY: c_int = 16;
pub const SJA1105PQRS_SIZE_CBS_ENTRY: c_int = 20;
pub const SJA1110_SIZE_PCP_REMAPPING_ENTRY: c_int = 4;
// UM10944.pdf Page 11, Table 2. Configuration Blocks
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sja1105_blk_idx {
    BLK_IDX_SCHEDULE = 0,
    BLK_IDX_SCHEDULE_ENTRY_POINTS,
    BLK_IDX_VL_LOOKUP,
    BLK_IDX_VL_POLICING,
    BLK_IDX_VL_FORWARDING,
    BLK_IDX_L2_LOOKUP,
    BLK_IDX_L2_POLICING,
    BLK_IDX_VLAN_LOOKUP,
    BLK_IDX_L2_FORWARDING,
    BLK_IDX_MAC_CONFIG,
    BLK_IDX_SCHEDULE_PARAMS,
    BLK_IDX_SCHEDULE_ENTRY_POINTS_PARAMS,
    BLK_IDX_VL_FORWARDING_PARAMS,
    BLK_IDX_L2_LOOKUP_PARAMS,
    BLK_IDX_L2_FORWARDING_PARAMS,
    BLK_IDX_AVB_PARAMS,
    BLK_IDX_GENERAL_PARAMS,
    BLK_IDX_RETAGGING,
    BLK_IDX_CBS,
    BLK_IDX_XMII_PARAMS,
    BLK_IDX_PCP_REMAPPING,
    BLK_IDX_MAX,
// Fake block indices that are only valid for dynamic access
    BLK_IDX_MGMT_ROUTE,
    BLK_IDX_MAX_DYN,
    BLK_IDX_INVAL = -1,
}

pub const SJA1105_MAX_SCHEDULE_COUNT: c_int = 1024;
pub const SJA1110_MAX_SCHEDULE_COUNT: c_int = 4096;
pub const SJA1105_MAX_SCHEDULE_ENTRY_POINTS_COUNT: c_int = 2048;
pub const SJA1105_MAX_VL_LOOKUP_COUNT: c_int = 1024;
pub const SJA1110_MAX_VL_LOOKUP_COUNT: c_int = 4096;
pub const SJA1105_MAX_VL_POLICING_COUNT: c_int = 1024;
pub const SJA1110_MAX_VL_POLICING_COUNT: c_int = 4096;
pub const SJA1105_MAX_VL_FORWARDING_COUNT: c_int = 1024;
pub const SJA1110_MAX_VL_FORWARDING_COUNT: c_int = 4096;
pub const SJA1105_MAX_L2_LOOKUP_COUNT: c_int = 1024;
pub const SJA1105_MAX_L2_POLICING_COUNT: c_int = 45;
pub const SJA1110_MAX_L2_POLICING_COUNT: c_int = 110;
pub const SJA1105_MAX_VLAN_LOOKUP_COUNT: c_int = 4096;
pub const SJA1105_MAX_L2_FORWARDING_COUNT: c_int = 13;
pub const SJA1110_MAX_L2_FORWARDING_COUNT: c_int = 19;
pub const SJA1105_MAX_MAC_CONFIG_COUNT: c_int = 5;
pub const SJA1110_MAX_MAC_CONFIG_COUNT: c_int = 11;
pub const SJA1105_MAX_SCHEDULE_PARAMS_COUNT: c_int = 1;
pub const SJA1105_MAX_SCHEDULE_ENTRY_POINTS_PARAMS_COUNT: c_int = 1;
pub const SJA1105_MAX_VL_FORWARDING_PARAMS_COUNT: c_int = 1;
pub const SJA1105_MAX_L2_LOOKUP_PARAMS_COUNT: c_int = 1;
pub const SJA1105_MAX_L2_FORWARDING_PARAMS_COUNT: c_int = 1;
pub const SJA1105_MAX_GENERAL_PARAMS_COUNT: c_int = 1;
pub const SJA1105_MAX_RETAGGING_COUNT: c_int = 32;
pub const SJA1105_MAX_XMII_PARAMS_COUNT: c_int = 1;
pub const SJA1105_MAX_AVB_PARAMS_COUNT: c_int = 1;
pub const SJA1105ET_MAX_CBS_COUNT: c_int = 10;
pub const SJA1105PQRS_MAX_CBS_COUNT: c_int = 16;
pub const SJA1110_MAX_CBS_COUNT: c_int = 80;
pub const SJA1110_MAX_PCP_REMAPPING_COUNT: c_int = 11;
pub const SJA1105_MAX_FRAME_MEMORY: c_int = 929;
pub const SJA1110_MAX_FRAME_MEMORY: c_int = 1820;
pub const SJA1105_FRAME_MEMORY_RETAGGING_OVERHEAD: c_int = 19;
pub const SJA1105_VL_FRAME_MEMORY: c_int = 100;
pub const SJA1105E_DEVICE_ID: c_uint = 0x9C00000Cull;
pub const SJA1105T_DEVICE_ID: c_uint = 0x9E00030Eull;
pub const SJA1105PR_DEVICE_ID: c_uint = 0xAF00030Eull;
pub const SJA1105QS_DEVICE_ID: c_uint = 0xAE00030Eull;
pub const SJA1110_DEVICE_ID: c_uint = 0xB700030Full;
pub const SJA1105ET_PART_NO: c_uint = 0x9A83;
pub const SJA1105P_PART_NO: c_uint = 0x9A84;
pub const SJA1105Q_PART_NO: c_uint = 0x9A85;
pub const SJA1105R_PART_NO: c_uint = 0x9A86;
pub const SJA1105S_PART_NO: c_uint = 0x9A87;
pub const SJA1110A_PART_NO: c_uint = 0x1110;
pub const SJA1110B_PART_NO: c_uint = 0x1111;
pub const SJA1110C_PART_NO: c_uint = 0x1112;
pub const SJA1110D_PART_NO: c_uint = 0x1113;
pub const SJA1110_ACU: c_uint = 0x1c4400;
pub const SJA1110_RGU: c_uint = 0x1c6000;
pub const SJA1110_CGU: c_uint = 0x1c6400;

pub const SJA1105_RSV_ADDR: c_uint = 0xffffffffffffffffull;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_schedule_entry {
    pub winstindex: u64,
    pub winend: u64,
    pub winst: u64,
    pub destports: u64,
    pub setvalid: u64,
    pub txen: u64,
    pub resmedia_en: u64,
    pub resmedia: u64,
    pub vlindex: u64,
    pub delta: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_schedule_params_entry {
    pub subscheind: [u64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_general_params_entry {
    pub vllupformat: u64,
    pub mirr_ptacu: u64,
    pub switchid: u64,
    pub hostprio: u64,
    pub mac_fltres1: u64,
    pub mac_fltres0: u64,
    pub mac_flt1: u64,
    pub mac_flt0: u64,
    pub incl_srcpt1: u64,
    pub incl_srcpt0: u64,
    pub send_meta1: u64,
    pub send_meta0: u64,
    pub casc_port: u64,
    pub host_port: u64,
    pub mirr_port: u64,
    pub vlmarker: u64,
    pub vlmask: u64,
    pub tpid: u64,
    pub ignore2stf: u64,
    pub tpid2: u64,
// P/Q/R/S only
    pub queue_ts: u64,
    pub egrmirrvid: u64,
    pub egrmirrpcp: u64,
    pub egrmirrdei: u64,
    pub replay_port: u64,
// SJA1110 only
    pub tte_en: u64,
    pub tdmaconfigidx: u64,
    pub header_type: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_schedule_entry_points_entry {
    pub subschindx: u64,
    pub delta: u64,
    pub address: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_schedule_entry_points_params_entry {
    pub clksrc: u64,
    pub actsubsch: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_vlan_lookup_entry {
    pub ving_mirr: u64,
    pub vegr_mirr: u64,
    pub vmemb_port: u64,
    pub vlan_bc: u64,
    pub tag_port: u64,
    pub vlanid: u64,
    pub /: *mut *mut u64 type_entry; / SJA1110 only,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_l2_lookup_entry {
    pub vlanid: u64,
    pub macaddr: u64,
    pub destports: u64,
    pub enfport: u64,
    pub index: u64,
// P/Q/R/S only
    pub mask_iotag: u64,
    pub mask_vlanid: u64,
    pub mask_macaddr: u64,
    pub mask_srcport: u64,
    pub iotag: u64,
    pub srcport: u64,
    pub lockeds: u64,
// LOCKEDS=1: Static FDB entries
// TSREG is deprecated in SJA1110, TRAP is supported only
// in SJA1110.
//
    pub trap: u64,
    pub tsreg: u64,
    pub mirrvlan: u64,
    pub takets: u64,
    pub mirr: u64,
    pub retag: u64,
}

// LOCKEDS=0: Dynamically learned FDB entries
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_l2_lookup_params_entry {
    pub /: *mut *mut u64 maxaddrp[SJA1105_MAX_NUM_PORTS]; / P/Q/R/S only,
    pub /: *mut *mut u64 start_dynspc; / P/Q/R/S only,
    pub /: *mut *mut u64 drpnolearn; / P/Q/R/S only,
    pub /: *mut *mut u64 use_static; / P/Q/R/S only,
    pub /: *mut *mut u64 owr_dyn; / P/Q/R/S only,
    pub /: *mut *mut u64 learn_once; / P/Q/R/S only,
    pub /: *mut *mut u64 maxage; / Shared,
    pub /: *mut *mut u64 dyn_tbsz; / E/T only,
    pub /: *mut *mut u64 poly; / E/T only,
    pub /: *mut *mut u64 shared_learn; / Shared,
    pub /: *mut *mut u64 no_enf_hostprt; / Shared,
    pub /: *mut *mut u64 no_mgmt_learn; / Shared,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_l2_forwarding_entry {
    pub bc_domain: u64,
    pub reach_port: u64,
    pub fl_domain: u64,
// This is actually max(SJA1105_NUM_TC, SJA1105_MAX_NUM_PORTS)
    pub vlan_pmap: [u64; SJA1105_MAX_NUM_PORTS],
    pub type_egrpcp2outputq: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_l2_forwarding_params_entry {
    pub max_dynp: u64,
    pub part_spc: [u64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_l2_policing_entry {
    pub sharindx: u64,
    pub smax: u64,
    pub rate: u64,
    pub maxlen: u64,
    pub partition: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_avb_params_entry {
    pub cas_master: u64,
    pub destmeta: u64,
    pub srcmeta: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_mac_config_entry {
    pub top: [u64; 8],
    pub base: [u64; 8],
    pub enabled: [u64; 8],
    pub ifg: u64,
    pub speed: u64,
    pub tp_delin: u64,
    pub tp_delout: u64,
    pub maxage: u64,
    pub vlanprio: u64,
    pub vlanid: u64,
    pub ing_mirr: u64,
    pub egr_mirr: u64,
    pub drpnona664: u64,
    pub drpdtag: u64,
    pub drpuntag: u64,
    pub retag: u64,
    pub dyn_learn: u64,
    pub egress: u64,
    pub ingress: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_retagging_entry {
    pub egr_port: u64,
    pub ing_port: u64,
    pub vlan_ing: u64,
    pub vlan_egr: u64,
    pub do_not_learn: u64,
    pub use_dest_ports: u64,
    pub destports: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_cbs_entry {
    pub /: *mut *mut u64 port; / Not used for SJA1110,
    pub /: *mut *mut u64 prio; / Not used for SJA1110,
    pub credit_hi: u64,
    pub credit_lo: u64,
    pub send_slope: u64,
    pub idle_slope: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_xmii_params_entry {
    pub phy_mac: [u64; SJA1105_MAX_NUM_PORTS],
    pub xmii_mode: [u64; SJA1105_MAX_NUM_PORTS],
// The SJA1110 insists being a snowflake, and requires SGMII,
// 2500base-x and internal MII ports connected to the 100base-TX PHY to
// set this bit. We set it unconditionally from the high-level logic,
// and only sja1110_xmii_params_entry_packing writes it to the static
// config. I have no better name for it than "special".
//
    pub special: [u64; SJA1105_MAX_NUM_PORTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1110_pcp_remapping_entry {
    pub egrpcp: [u64; SJA1105_NUM_TC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_vl_lookup_entry {
    pub format: u64,
    pub port: u64,
// SJA1105_VL_FORMAT_PSFP
    pub destports: u64,
    pub iscritical: u64,
    pub macaddr: u64,
    pub vlanid: u64,
    pub vlanprior: u64,
}

// SJA1105_VL_FORMAT_ARINC664
// Not part of hardware structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_vl_policing_entry {
    pub type: u64,
    pub maxlen: u64,
    pub sharindx: u64,
    pub bag: u64,
    pub jitter: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_vl_forwarding_entry {
    pub type: u64,
    pub priority: u64,
    pub partition: u64,
    pub destports: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_vl_forwarding_params_entry {
    pub partspc: [u64; 8],
    pub debugen: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_table_header {
    pub block_id: u64,
    pub len: u64,
    pub crc: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_table_ops {
    pub op): *mut *mut *mut *mut size_t (packing)(void buf, void entry_ptr, enum packing_op,
    pub unpacked_entry_size: usize,
    pub packed_entry_size: usize,
    pub max_entry_count: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_table {
    pub ops: *const sja1105_table_ops,
    pub entry_count: usize,
    pub entries: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_static_config {
    pub device_id: u64,
    pub tables: [sja1105_table; BLK_IDX_MAX],
}

extern "C" {
    pub fn sja1105_table_header_packing(buf: *mut c_void, hdr: *mut c_void, op: packing_op) -> usize;
}
extern "C" {
    pub fn sja1105_static_config_free(config: *mut sja1105_static_config);
}
extern "C" {
    pub fn sja1105_table_delete_entry(table: *mut sja1105_table, i: c_int) -> c_int;
}
extern "C" {
    pub fn sja1105_table_resize(table: *mut sja1105_table, new_count: usize) -> c_int;
}
extern "C" {
    pub fn sja1105_crc32(buf: *const c_void, len: usize) -> u32;
}
extern "C" {
    pub fn sja1105_pack(buf: *mut c_void, val: *const u64, start: c_int, end: c_int, len: usize);
}
extern "C" {
    pub fn sja1105_unpack(buf: *const c_void, val: *mut u64, start: c_int, end: c_int, len: usize);
}
// Common implementations for the static and dynamic configs
