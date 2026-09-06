//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/cudbg_entity.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2017 Chelsio Communications.  All rights reserved.
//
pub const EDC0_FLAG: c_int = 0;
pub const EDC1_FLAG: c_int = 1;
pub const MC_FLAG: c_int = 2;
pub const MC0_FLAG: c_int = 3;
pub const MC1_FLAG: c_int = 4;
pub const HMA_FLAG: c_int = 5;
pub const CUDBG_ENTITY_SIGNATURE: c_uint = 0xCCEDB001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_mbox_log {
    pub entry: mbox_cmd,
    pub 8]: u32 hi[MBOX_LEN /,
    pub 8]: u32 lo[MBOX_LEN /,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_cim_qcfg {
    pub chip: u8,
    pub CIM_NUM_OBQ_T5]: u16 base[CIM_NUM_IBQ +,
    pub CIM_NUM_OBQ_T5]: u16 size[CIM_NUM_IBQ +,
    pub thres: [u16; CIM_NUM_IBQ],
    pub CIM_NUM_OBQ_T5]: *mut *mut u32 obq_wr[2,
    pub CIM_NUM_OBQ_T5)]: *mut *mut u32 stat[4  (CIM_NUM_IBQ +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_rss_vf_conf {
    pub rss_vf_vfl: u32,
    pub rss_vf_vfh: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_pm_stats {
    pub tx_cnt: [u32; T6_PM_NSTATS],
    pub rx_cnt: [u32; T6_PM_NSTATS],
    pub tx_cyc: [u64; T6_PM_NSTATS],
    pub rx_cyc: [u64; T6_PM_NSTATS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_hw_sched {
    pub kbps: [u32; NTX_SCHED],
    pub ipg: [u32; NTX_SCHED],
    pub pace_tab: [u32; NTX_SCHED],
    pub mode: u32,
    pub map: u32,
}

pub const SGE_QBASE_DATA_REG_NUM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_qbase_reg_field {
    pub reg_addr: u32,
    pub reg_data: [u32; SGE_QBASE_DATA_REG_NUM],
// Max supported PFs
    pub 1][SGE_QBASE_DATA_REG_NUM]: u32 pf_data_value[PCIE_FW_MASTER_M +,
// Max supported VFs
    pub 1][SGE_QBASE_DATA_REG_NUM]: u32 vf_data_value[T6_VF_M +,
    pub /: *mut *mut u32 vfcount; / Actual number of max vfs in current configuration,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ireg_field {
    pub ireg_addr: u32,
    pub ireg_data: u32,
    pub ireg_local_offset: u32,
    pub ireg_offset_range: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ireg_buf {
    pub tp_pio: ireg_field,
    pub outbuf: [u32; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_ulprx_la {
    pub 8]: *mut *mut u32 data[ULPRX_LA_SIZE,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_tp_la {
    pub size: u32,
    pub mode: u32,
    pub data: [u8; ],
}

// Memory region info relative to current memory (i.e. wrt 0).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_region_info {
    pub /: *mut *mut bool exist; / Does region exists in current memory?,
    pub /: *mut *mut u32 start; / Start wrt 0,
    pub /: *mut *mut u32 end; / End wrt 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_mem_desc {
    pub base: u32,
    pub limit: u32,
    pub idx: u32,
}

pub const CUDBG_MEMINFO_REV: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_meminfo {
    pub avail: [cudbg_mem_desc; 4],
    pub 3]: cudbg_mem_desc mem[ARRAY_SIZE(cudbg_region) +,
    pub avail_c: u32,
    pub mem_c: u32,
    pub up_ram_lo: u32,
    pub up_ram_hi: u32,
    pub up_extmem2_lo: u32,
    pub up_extmem2_hi: u32,
    pub rx_pages_data: [u32; 3],
    pub tx_pages_data: [u32; 4],
    pub p_structs: u32,
    pub reserved: [u32; 12],
    pub port_used: [u32; 4],
    pub port_alloc: [u32; 4],
    pub loopback_used: [u32; NCHAN],
    pub loopback_alloc: [u32; NCHAN],
    pub p_structs_free_cnt: u32,
    pub free_rx_cnt: u32,
    pub free_tx_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_cim_pif_la {
    pub size: c_int,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_clk_info {
    pub retransmit_min: u64,
    pub retransmit_max: u64,
    pub persist_timer_min: u64,
    pub persist_timer_max: u64,
    pub keepalive_idle_timer: u64,
    pub keepalive_interval: u64,
    pub initial_srtt: u64,
    pub finwait2_timer: u64,
    pub dack_timer: u32,
    pub res: u32,
    pub cclk_ps: u32,
    pub tre: u32,
    pub dack_re: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_tid_info_region {
    pub ntids: u32,
    pub nstids: u32,
    pub stid_base: u32,
    pub hash_base: u32,
    pub natids: u32,
    pub nftids: u32,
    pub ftid_base: u32,
    pub aftid_base: u32,
    pub aftid_end: u32,
    pub sftid_base: u32,
    pub nsftids: u32,
    pub uotid_base: u32,
    pub nuotids: u32,
    pub sb: u32,
    pub flags: u32,
    pub le_db_conf: u32,
    pub ip_users: u32,
    pub ipv6_users: u32,
    pub hpftid_base: u32,
    pub nhpftids: u32,
}

pub const CUDBG_TID_INFO_REV: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_tid_info_region_rev1 {
    pub ver_hdr: cudbg_ver_hdr,
    pub tid: cudbg_tid_info_region,
    pub tid_start: u32,
    pub reserved: [u32; 16],
}

pub const CUDBG_LOWMEM_MAX_CTXT_QIDS: c_int = 256;
pub const CUDBG_MAX_FL_QIDS: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_ch_cntxt {
    pub cntxt_type: u32,
    pub cntxt_id: u32,
    pub 4]: u32 data[SGE_CTXT_SIZE /,
}

pub const CUDBG_MAX_RPLC_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_mps_tcam {
    pub mask: u64,
    pub rplc: [u32; 8],
    pub idx: u32,
    pub cls_lo: u32,
    pub cls_hi: u32,
    pub rplc_size: u32,
    pub vniy: u32,
    pub vnix: u32,
    pub dip_hit: u32,
    pub vlan_vld: u32,
    pub repli: u32,
    pub ivlan: u16,
    pub addr: [u8; ETH_ALEN],
    pub lookup_type: u8,
    pub port_num: u8,
    pub reserved: [u8; 2],
}

pub const CUDBG_VPD_VER_ADDR: c_uint = 0x18c7;
pub const CUDBG_VPD_VER_LEN: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_vpd_data {
    pub 1]: u8 sn[SERNUM_LEN +,
    pub 1]: u8 bn[PN_LEN +,
    pub 1]: u8 na[MACADDR_LEN +,
    pub 1]: u8 mn[ID_LEN +,
    pub fw_major: u16,
    pub fw_minor: u16,
    pub fw_micro: u16,
    pub fw_build: u16,
    pub scfg_vers: u32,
    pub vpd_vers: u32,
}

pub const CUDBG_MAX_TCAM_TID: c_uint = 0x800;
pub const CUDBG_T6_CLIP: c_int = 1536;
pub const CUDBG_MAX_TID_COMP_EN: c_int = 6144;
pub const CUDBG_MAX_TID_COMP_DIS: c_int = 3072;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cudbg_le_entry_types {
    LE_ET_UNKNOWN = 0,
    LE_ET_TCAM_CON = 1,
    LE_ET_TCAM_SERVER = 2,
    LE_ET_TCAM_FILTER = 3,
    LE_ET_TCAM_CLIP = 4,
    LE_ET_TCAM_ROUTING = 5,
    LE_ET_HASH_CON = 6,
    LE_ET_INVALID_TID = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_tcam {
    pub filter_start: u32,
    pub server_start: u32,
    pub clip_start: u32,
    pub routing_start: u32,
    pub tid_hash_base: u32,
    pub max_tid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_tid_data {
    pub tid: u32,
    pub dbig_cmd: u32,
    pub dbig_conf: u32,
    pub dbig_rsp_stat: u32,
    pub data: [u32; NUM_LE_DB_DBGI_RSP_DATA_INSTANCES],
}

pub const CUDBG_NUM_ULPTX: c_int = 11;
pub const CUDBG_NUM_ULPTX_READ: c_int = 512;
pub const CUDBG_NUM_ULPTX_ASIC: c_int = 6;
pub const CUDBG_NUM_ULPTX_ASIC_READ: c_int = 128;
pub const CUDBG_ULPTX_LA_REV: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_ulptx_la {
    pub rdptr: [u32; CUDBG_NUM_ULPTX],
    pub wrptr: [u32; CUDBG_NUM_ULPTX],
    pub rddata: [u32; CUDBG_NUM_ULPTX],
    pub rd_data: [u32; CUDBG_NUM_ULPTX][CUDBG_NUM_ULPTX_READ],
    pub rdptr_asic: [u32; CUDBG_NUM_ULPTX_ASIC_READ],
    pub rddata_asic: [u32; CUDBG_NUM_ULPTX_ASIC_READ][CUDBG_NUM_ULPTX_ASIC],
}

pub const CUDBG_CHAC_PBT_ADDR: c_uint = 0x2800;
pub const CUDBG_CHAC_PBT_LRF: c_uint = 0x3000;
pub const CUDBG_CHAC_PBT_DATA: c_uint = 0x3800;
pub const CUDBG_PBT_DYNAMIC_ENTRIES: c_int = 8;
pub const CUDBG_PBT_STATIC_ENTRIES: c_int = 16;
pub const CUDBG_LRF_ENTRIES: c_int = 8;
pub const CUDBG_PBT_DATA_ENTRIES: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_pbt_tables {
    pub pbt_dynamic: [u32; CUDBG_PBT_DYNAMIC_ENTRIES],
    pub pbt_static: [u32; CUDBG_PBT_STATIC_ENTRIES],
    pub lrf_table: [u32; CUDBG_LRF_ENTRIES],
    pub pbt_data: [u32; CUDBG_PBT_DATA_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cudbg_qdesc_qtype {
    CUDBG_QTYPE_UNKNOWN = 0,
    CUDBG_QTYPE_NIC_TXQ,
    CUDBG_QTYPE_NIC_RXQ,
    CUDBG_QTYPE_NIC_FLQ,
    CUDBG_QTYPE_CTRLQ,
    CUDBG_QTYPE_FWEVTQ,
    CUDBG_QTYPE_INTRQ,
    CUDBG_QTYPE_PTP_TXQ,
    CUDBG_QTYPE_OFLD_TXQ,
    CUDBG_QTYPE_RDMA_RXQ,
    CUDBG_QTYPE_RDMA_FLQ,
    CUDBG_QTYPE_RDMA_CIQ,
    CUDBG_QTYPE_ISCSI_RXQ,
    CUDBG_QTYPE_ISCSI_FLQ,
    CUDBG_QTYPE_ISCSIT_RXQ,
    CUDBG_QTYPE_ISCSIT_FLQ,
    CUDBG_QTYPE_CRYPTO_TXQ,
    CUDBG_QTYPE_CRYPTO_RXQ,
    CUDBG_QTYPE_CRYPTO_FLQ,
    CUDBG_QTYPE_TLS_RXQ,
    CUDBG_QTYPE_TLS_FLQ,
    CUDBG_QTYPE_ETHOFLD_TXQ,
    CUDBG_QTYPE_ETHOFLD_RXQ,
    CUDBG_QTYPE_ETHOFLD_FLQ,
    CUDBG_QTYPE_MAX,
}

pub const CUDBG_QDESC_REV: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_qdesc_entry {
    pub data_size: u32,
    pub qtype: u32,
    pub qid: u32,
    pub desc_size: u32,
    pub num_desc: u32,
    pub /: *mut *mut u8 data[]; / Must be last,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudbg_qdesc_info {
    pub qdesc_entry_size: u32,
    pub num_queues: u32,
    pub /: *mut *mut u8 data[]; / Must be last,
}

pub const IREG_NUM_ELEM: c_int = 4;
pub const CUDBG_NUM_PCIE_CONFIG_REGS: c_uint = 0x61;
