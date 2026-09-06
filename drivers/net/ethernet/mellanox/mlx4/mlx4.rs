//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx4/mlx4.h
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


//
// Copyright (c) 2004, 2005 Topspin Communications.  All rights reserved.
// Copyright (c) 2005 Sun Microsystems, Inc. All rights reserved.
// Copyright (c) 2005, 2006, 2007 Cisco Systems.  All rights reserved.
// Copyright (c) 2005, 2006, 2007, 2008 Mellanox Technologies. All rights reserved.
// Copyright (c) 2004 Voltaire, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub const MLX4_FS_NUM_OF_L2_ADDR: c_int = 8;
pub const MLX4_FS_MGM_LOG_ENTRY_SIZE: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_mpt_state {
    MLX4_MPT_DISABLED = 0,
    MLX4_MPT_EN_HW,
    MLX4_MPT_EN_SW
}

pub const MLX4_COMM_TIME: c_int = 10000;
pub const MLX4_COMM_OFFLINE_TIME_OUT: c_int = 30000;
pub const MLX4_COMM_CMD_NA_OP: c_uint = 0x0;
// The flag indicates that the slave should delay the RESET cmd
pub const MLX4_DELAY_RESET_SLAVE: c_uint = 0xbbbbbbb;
// indicates how many retries will be done if we are in the middle of FLR
pub const NUM_OF_RESET_RETRIES: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_resource {
    RES_QP,
    RES_CQ,
    RES_SRQ,
    RES_XRCD,
    RES_MPT,
    RES_MTT,
    RES_MAC,
    RES_VLAN,
    RES_NPORT_ID,
    RES_COUNTER,
    RES_FS_RULE,
    RES_EQ,
    MLX4_NUM_OF_RESOURCE_TYPE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_alloc_mode {
    RES_OP_RESERVE,
    RES_OP_RESERVE_AND_MAP,
    RES_OP_MAP_ICM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_res_tracker_free_type {
    RES_TR_FREE_ALL,
    RES_TR_FREE_SLAVES_ONLY,
    RES_TR_FREE_STRUCTS_ONLY,
}

//
// Virtual HCR structures.
// mlx4_vhcr is the sw representation, in machine endianness
//
// mlx4_vhcr_cmd is the formalized structure, the one that is passed
// to FW to go through communication channel.
// It is big endian, and has the same structure as the physical HCR
// used by command interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_vhcr {
    pub in_param: u64,
    pub out_param: u64,
    pub in_modifier: u32,
    pub errno: u32,
    pub op: u16,
    pub token: u16,
    pub op_modifier: u8,
    pub e_bit: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_vhcr_cmd {
    pub in_param: __be64,
    pub in_modifier: __be32,
    pub reserved1: u32,
    pub out_param: __be64,
    pub token: __be16,
    pub reserved: u16,
    pub status: u8,
    pub flags: u8,
    pub opcode: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_cmd_info {
    pub opcode: u16,
    pub has_inbox: bool,
    pub has_outbox: bool,
    pub out_is_imm: bool,
    pub encode_slave_id: bool,
    pub inbox): *mut mlx4_cmd_mailbox,
    pub cmd): *mut mlx4_cmd_info,
}

pub const ALL_SLAVES: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_bitmap {
    pub last: u32,
    pub top: u32,
    pub max: u32,
    pub reserved_top: u32,
    pub mask: u32,
    pub avail: u32,
    pub effective_len: u32,
    pub lock: spinlock_t,
    pub table: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_buddy {
    pub bits: *mut c_ulong,
    pub num_free: *mut c_uint,
    pub max_order: u32,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_icm_table {
    pub virt: u64,
    pub num_icm: c_int,
    pub num_obj: u32,
    pub obj_size: c_int,
    pub lowmem: c_int,
    pub coherent: c_int,
    pub mutex: mutex,
    pub icm: *mut mlx4_icm,
}

pub const MLX4_MPT_STATUS_SW: c_uint = 0xF0;
pub const MLX4_MPT_STATUS_HW: c_uint = 0x00;
pub const MLX4_CQE_SIZE_MASK_STRIDE: c_uint = 0x3;
pub const MLX4_EQE_SIZE_MASK_STRIDE: c_uint = 0x30;
pub const MLX4_EQ_ASYNC: c_int = 0;

//
// Must be packed because mtt_seg is 64 bits but only aligned to 32 bits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_mpt_entry {
    pub flags: __be32,
    pub qpn: __be32,
    pub key: __be32,
    pub pd_flags: __be32,
    pub start: __be64,
    pub length: __be64,
    pub lkey: __be32,
    pub win_cnt: __be32,
    pub reserved1: [u8; 3],
    pub mtt_rep: u8,
    pub mtt_addr: __be64,
    pub mtt_sz: __be32,
    pub entity_size: __be32,
    pub first_byte_offset: __be32,
    pub __packed: },
//
// Must be packed because start is 64 bits but only aligned to 32 bits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_eq_context {
    pub flags: __be32,
    pub reserved1: [u16; 3],
    pub page_offset: __be16,
    pub log_eq_size: u8,
    pub reserved2: [u8; 4],
    pub eq_period: u8,
    pub reserved3: u8,
    pub eq_max_count: u8,
    pub reserved4: [u8; 3],
    pub intr: u8,
    pub log_page_size: u8,
    pub reserved5: [u8; 2],
    pub mtt_base_addr_h: u8,
    pub mtt_base_addr_l: __be32,
    pub reserved6: [u32; 2],
    pub consumer_index: __be32,
    pub producer_index: __be32,
    pub reserved7: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_cq_context {
    pub flags: __be32,
    pub reserved1: [u16; 3],
    pub page_offset: __be16,
    pub logsize_usrpage: __be32,
    pub cq_period: __be16,
    pub cq_max_count: __be16,
    pub reserved2: [u8; 3],
    pub comp_eqn: u8,
    pub log_page_size: u8,
    pub reserved3: [u8; 2],
    pub mtt_base_addr_h: u8,
    pub mtt_base_addr_l: __be32,
    pub last_notified_index: __be32,
    pub solicit_producer_index: __be32,
    pub consumer_index: __be32,
    pub producer_index: __be32,
    pub reserved4: [u32; 2],
    pub db_rec_addr: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_srq_context {
    pub state_logsize_srqn: __be32,
    pub logstride: u8,
    pub reserved1: u8,
    pub xrcd: __be16,
    pub pg_offset_cqn: __be32,
    pub reserved2: u32,
    pub log_page_size: u8,
    pub reserved3: [u8; 2],
    pub mtt_base_addr_h: u8,
    pub mtt_base_addr_l: __be32,
    pub pd: __be32,
    pub limit_watermark: __be16,
    pub wqe_cnt: __be16,
    pub reserved4: u16,
    pub wqe_counter: __be16,
    pub reserved5: u32,
    pub db_rec_addr: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_eq_tasklet {
    pub list: list_head,
    pub process_list: list_head,
    pub task: tasklet_struct,
// lock on completion tasklet list
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_eq {
    pub dev: *mut mlx4_dev,
    pub doorbell: *mut void __iomem,
    pub eqn: c_int,
    pub cons_index: u32,
    pub irq: u16,
    pub have_irq: u16,
    pub nent: c_int,
    pub page_list: *mut mlx4_buf_list,
    pub mtt: mlx4_mtt,
    pub tasklet_ctx: mlx4_eq_tasklet,
    pub actv_ports: mlx4_active_ports,
    pub ref_count: u32,
    pub affinity_mask: cpumask_var_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_slave_eqe {
    pub type: u8,
    pub port: u8,
    pub param: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_slave_event_eq_info {
    pub eqn: c_int,
    pub token: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_profile {
    pub num_qp: c_int,
    pub rdmarc_per_qp: c_int,
    pub num_srq: c_int,
    pub num_cq: c_int,
    pub num_mcg: c_int,
    pub num_mpt: c_int,
    pub num_mtt: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_fw {
    pub clr_int_base: u64,
    pub catas_offset: u64,
    pub comm_base: u64,
    pub clock_offset: u64,
    pub fw_icm: *mut mlx4_icm,
    pub aux_icm: *mut mlx4_icm,
    pub catas_size: u32,
    pub fw_pages: u16,
    pub clr_int_bar: u8,
    pub catas_bar: u8,
    pub comm_bar: u8,
    pub clock_bar: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_comm {
    pub slave_write: u32,
    pub slave_read: u32,
}

pub const VLAN_FLTR_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_vlan_fltr {
    pub entry: [__be32; VLAN_FLTR_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_mcast_entry {
    pub list: list_head,
    pub addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_promisc_qp {
    pub list: list_head,
    pub qpn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_steer_index {
    pub list: list_head,
    pub index: c_uint,
    pub duplicates: list_head,
}

pub const MLX4_EVENT_TYPES_NUM: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_slave_state {
    pub comm_toggle: u8,
    pub last_cmd: u8,
    pub init_port_mask: u8,
    pub active: bool,
    pub old_vlan_api: bool,
    pub vst_qinq_supported: bool,
    pub function: u8,
    pub vhcr_dma: dma_addr_t,
    pub 1]: u16 user_mtu[MLX4_MAX_PORTS +,
    pub 1]: u16 mtu[MLX4_MAX_PORTS +,
    pub 1]: __be32 ib_cap_mask[MLX4_MAX_PORTS +,
    pub eq: [mlx4_slave_eqe; MLX4_MFUNC_MAX_EQES],
    pub 1]: list_head mcast_filters[MLX4_MAX_PORTS +,
    pub 1]: *mut *mut mlx4_vlan_fltr vlan_filter[MLX4_MAX_PORTS +,
// event type to eq number lookup
    pub event_eq: [mlx4_slave_event_eq_info; MLX4_EVENT_TYPES_NUM],
    pub eq_pi: u16,
    pub eq_ci: u16,
    pub lock: spinlock_t,
// initialized via the kzalloc
    pub is_slave_going_down: u8,
    pub cookie: u32,
    pub 1]: slave_port_state port_state[MLX4_MAX_PORTS +,
}

pub const MLX4_VGT: c_int = 4095;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_vport_state {
    pub mac: u64,
    pub default_vlan: u16,
    pub default_qos: u8,
    pub vlan_proto: __be16,
    pub tx_rate: u32,
    pub spoofchk: bool,
    pub link_state: u32,
    pub qos_vport: u8,
    pub guid: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_vf_admin_state {
    pub 1]: mlx4_vport_state vport[MLX4_MAX_PORTS +,
    pub 1]: u8 enable_smi[MLX4_MAX_PORTS +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_vport_oper_state {
    pub state: mlx4_vport_state,
    pub mac_idx: c_int,
    pub vlan_idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_vf_oper_state {
    pub 1]: mlx4_vport_oper_state vport[MLX4_MAX_PORTS +,
    pub 1]: u8 smi_enabled[MLX4_MAX_PORTS +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slave_list {
    pub mutex: mutex,
    pub res_list: [list_head; MLX4_NUM_OF_RESOURCE_TYPE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_allocator {
    pub /: *mut *mut spinlock_t alloc_lock; / protect quotas,
    pub res_reserved: c_uint,
    pub res_port_rsvd: [c_uint; MLX4_MAX_PORTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_resource_tracker {
    pub lock: spinlock_t,
// tree for each resources
    pub res_tree: [rb_root; MLX4_NUM_OF_RESOURCE_TYPE],
// num_of_slave's lists, one per slave
    pub slave_list: *mut slave_list,
    pub res_alloc: [resource_allocator; MLX4_NUM_OF_RESOURCE_TYPE],
}

pub const SLAVE_EVENT_EQ_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_slave_event_eq {
    pub eqn: u32,
    pub cons: u32,
    pub prod: u32,
    pub event_lock: spinlock_t,
    pub event_eqe: [mlx4_eqe; SLAVE_EVENT_EQ_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_qos_manager {
    pub num_of_qos_vfs: c_int,
    pub MLX4_NUM_UP): DECLARE_BITMAP(priority_bm,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_master_qp0_state {
    pub proxy_qp0_active: c_int,
    pub qp0_active: c_int,
    pub port_active: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_mfunc_master_ctx {
    pub slave_state: *mut mlx4_slave_state,
    pub vf_admin: *mut mlx4_vf_admin_state,
    pub vf_oper: *mut mlx4_vf_oper_state,
    pub 1]: mlx4_master_qp0_state qp0_state[MLX4_MAX_PORTS +,
    pub 1]: int init_port_ref[MLX4_MAX_PORTS +,
    pub 1]: u16 max_mtu[MLX4_MAX_PORTS +,
    pub 1]: u16 max_user_mtu[MLX4_MAX_PORTS +,
    pub pptx: u8,
    pub pprx: u8,
    pub 1]: int disable_mcast_ref[MLX4_MAX_PORTS +,
    pub res_tracker: mlx4_resource_tracker,
    pub comm_wq: *mut workqueue_struct,
    pub comm_work: work_struct,
    pub slave_event_work: work_struct,
    pub slave_flr_event_work: work_struct,
    pub slave_state_lock: spinlock_t,
    pub comm_arm_bit_vector: [__be32; 4],
    pub cmd_eqe: mlx4_eqe,
    pub slave_eq: mlx4_slave_event_eq,
    pub gen_eqe_mutex: [mutex; MLX4_MFUNC_MAX],
    pub 1]: mlx4_qos_manager qos_ctl[MLX4_MAX_PORTS +,
    pub /: *mut *mut u32 next_slave; / mlx4_master_comm_channel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_mfunc {
    pub comm: *mut mlx4_comm __iomem,
    pub vhcr: *mut mlx4_vhcr_cmd,
    pub vhcr_dma: dma_addr_t,
    pub master: mlx4_mfunc_master_ctx,
}

pub const MGM_QPN_MASK: c_uint = 0x00FFFFFF;
pub const MGM_BLCK_LB_BIT: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_mgm {
    pub next_gid_index: __be32,
    pub members_count: __be32,
    pub reserved: [u32; 2],
    pub gid: [u8; 16],
    pub qp: [__be32; MLX4_MAX_QP_PER_MGM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_cmd {
    pub pool: *mut dma_pool,
    pub hcr: *mut void __iomem,
    pub slave_cmd_mutex: mutex,
    pub poll_sem: semaphore,
    pub event_sem: semaphore,
    pub switch_sem: rw_semaphore,
    pub max_cmds: c_int,
    pub context_lock: spinlock_t,
    pub free_head: c_int,
    pub context: *mut mlx4_cmd_context,
    pub token_mask: u16,
    pub use_events: u8,
    pub toggle: u8,
    pub comm_toggle: u8,
    pub initialized: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_vf_immed_vlan_work {
    pub work: work_struct,
    pub priv: *mut mlx4_priv,
    pub flags: c_int,
    pub slave: c_int,
    pub vlan_ix: c_int,
    pub orig_vlan_ix: c_int,
    pub port: u8,
    pub qos: u8,
    pub qos_vport: u8,
    pub vlan_id: u16,
    pub orig_vlan_id: u16,
    pub vlan_proto: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_uar_table {
    pub bitmap: mlx4_bitmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_mr_table {
    pub mpt_bitmap: mlx4_bitmap,
    pub mtt_buddy: mlx4_buddy,
    pub mtt_base: u64,
    pub mpt_base: u64,
    pub mtt_table: mlx4_icm_table,
    pub dmpt_table: mlx4_icm_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_cq_table {
    pub bitmap: mlx4_bitmap,
    pub lock: spinlock_t,
    pub tree: radix_tree_root,
    pub table: mlx4_icm_table,
    pub cmpt_table: mlx4_icm_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_eq_table {
    pub bitmap: mlx4_bitmap,
    pub irq_names: *mut c_char,
    pub clr_int: *mut void __iomem,
    pub uar_map: *mut void __iomem,
    pub clr_mask: u32,
    pub eq: *mut mlx4_eq,
    pub table: mlx4_icm_table,
    pub cmpt_table: mlx4_icm_table,
    pub have_irq: c_int,
    pub inta_pin: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_srq_table {
    pub bitmap: mlx4_bitmap,
    pub lock: spinlock_t,
    pub tree: radix_tree_root,
    pub table: mlx4_icm_table,
    pub cmpt_table: mlx4_icm_table,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_qp_table_zones {
    MLX4_QP_TABLE_ZONE_GENERAL,
    MLX4_QP_TABLE_ZONE_RSS,
    MLX4_QP_TABLE_ZONE_RAW_ETH,
    MLX4_QP_TABLE_ZONE_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_qp_table {
    pub bitmap_gen: *mut mlx4_bitmap,
    pub zones: *mut mlx4_zone_allocator,
    pub zones_uids: [u32; MLX4_QP_TABLE_ZONE_NUM],
    pub rdmarc_base: u32,
    pub rdmarc_shift: c_int,
    pub lock: spinlock_t,
    pub qp_table: mlx4_icm_table,
    pub auxc_table: mlx4_icm_table,
    pub altc_table: mlx4_icm_table,
    pub rdmarc_table: mlx4_icm_table,
    pub cmpt_table: mlx4_icm_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_mcg_table {
    pub mutex: mutex,
    pub bitmap: mlx4_bitmap,
    pub table: mlx4_icm_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_catas_err {
    pub map: *mut u32 __iomem,
    pub timer: timer_list,
    pub list: list_head,
}

pub const MLX4_MAX_MAC_NUM: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_mac_table {
    pub entries: [__be64; MLX4_MAX_MAC_NUM],
    pub refs: [c_int; MLX4_MAX_MAC_NUM],
    pub is_dup: [bool; MLX4_MAX_MAC_NUM],
    pub mutex: mutex,
    pub total: c_int,
    pub max: c_int,
}

pub const MLX4_ROCE_GID_ENTRY_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_roce_gid_entry {
    pub raw: [u8; MLX4_ROCE_GID_ENTRY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_roce_gid_table {
    pub roce_gids: [mlx4_roce_gid_entry; MLX4_ROCE_MAX_GIDS],
    pub mutex: mutex,
}

pub const MLX4_MAX_VLAN_NUM: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_vlan_table {
    pub entries: [__be32; MLX4_MAX_VLAN_NUM],
    pub refs: [c_int; MLX4_MAX_VLAN_NUM],
    pub is_dup: [c_int; MLX4_MAX_VLAN_NUM],
    pub mutex: mutex,
    pub total: c_int,
    pub max: c_int,
}

pub const SET_PORT_PROMISC_SHIFT: c_int = 31;
pub const SET_PORT_MC_PROMISC_SHIFT: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_set_port_general_context {
    pub reserved1: u16,
    pub flags2: u8,
    pub flags: u8,
    pub ignore_fcs: u8,
    pub roce_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_set_port_rqp_calc_context {
    pub base_qpn: __be32,
    pub rererved: u8,
    pub n_mac: u8,
    pub n_vlan: u8,
    pub n_prio: u8,
    pub reserved2: [u8; 3],
    pub mac_miss: u8,
    pub intra_no_vlan: u8,
    pub no_vlan: u8,
    pub intra_vlan_miss: u8,
    pub vlan_miss: u8,
    pub reserved3: [u8; 3],
    pub no_vlan_prio: u8,
    pub promisc: __be32,
    pub mcast: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_port_info {
    pub dev: *mut mlx4_dev,
    pub port: c_int,
    pub dev_name: [c_char; 16],
    pub port_attr: device_attribute,
    pub tmp_type: mlx4_port_type,
    pub dev_mtu_name: [c_char; 16],
    pub port_mtu_attr: device_attribute,
    pub mac_table: mlx4_mac_table,
    pub vlan_table: mlx4_vlan_table,
    pub gid_table: mlx4_roce_gid_table,
    pub base_qpn: c_int,
    pub rmap: *mut cpu_rmap,
    pub devlink_port: devlink_port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_sense {
    pub dev: *mut mlx4_dev,
    pub 1]: u8 do_sense_port[MLX4_MAX_PORTS +,
    pub 1]: u8 sense_allowed[MLX4_MAX_PORTS +,
    pub sense_poll: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_msix_ctl {
    pub MAX_MSIX): DECLARE_BITMAP(pool_bm,,
    pub pool_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_steer {
    pub promisc_qps: [list_head; MLX4_NUM_STEERS],
    pub steer_entries: [list_head; MLX4_NUM_STEERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_port_map {
    pub port1: u8,
    pub port2: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_priv {
    pub dev: mlx4_dev,
    pub adev: *mut mlx4_adev,
    pub adev_idx: c_int,
    pub event_nh: atomic_notifier_head,
    pub pci_dev_data: c_int,
    pub removed: c_int,
    pub pgdir_list: list_head,
    pub pgdir_mutex: mutex,
    pub fw: mlx4_fw,
    pub cmd: mlx4_cmd,
    pub mfunc: mlx4_mfunc,
    pub pd_bitmap: mlx4_bitmap,
    pub xrcd_bitmap: mlx4_bitmap,
    pub uar_table: mlx4_uar_table,
    pub mr_table: mlx4_mr_table,
    pub cq_table: mlx4_cq_table,
    pub eq_table: mlx4_eq_table,
    pub srq_table: mlx4_srq_table,
    pub qp_table: mlx4_qp_table,
    pub mcg_table: mlx4_mcg_table,
    pub counters_bitmap: mlx4_bitmap,
    pub def_counter: [c_int; MLX4_MAX_PORTS],
    pub catas_err: mlx4_catas_err,
    pub clr_base: *mut void __iomem,
    pub driver_uar: mlx4_uar,
    pub kar: *mut void __iomem,
    pub 1]: mlx4_port_info port[MLX4_MAX_PORTS +,
    pub sense: mlx4_sense,
    pub port_mutex: mutex,
    pub msix_ctl: mlx4_msix_ctl,
    pub steer: *mut mlx4_steer,
    pub bf_list: list_head,
    pub bf_mutex: mutex,
    pub bf_mapping: *mut io_mapping,
    pub clock_mapping: *mut void __iomem,
    pub reserved_mtts: c_int,
    pub fs_hash_mode: c_int,
    pub virt2phys_pkey: [u8; MLX4_MFUNC_MAX][MLX4_MAX_PORTS][MLX4_MAX_PORT_PKEYS],
    pub /: *mut *mut mlx4_port_map v2p; / cached port mapping configuration,
    pub /: *mut *mut mutex bond_mutex; / for bond mode,
    pub slave_node_guids: [__be64; MLX4_MFUNC_MAX],
    pub opreq_count: core::sync::atomic::AtomicI32,
    pub opreq_task: work_struct,
}

extern "C" {
    pub fn container_of(_arg: dev, mlx4_priv: struct, _arg: dev) -> return;
}

extern "C" {
    pub fn mlx4_bitmap_alloc(bitmap: *mut mlx4_bitmap) -> u32;
}
extern "C" {
    pub fn mlx4_bitmap_free(bitmap: *mut mlx4_bitmap, obj: u32, use_rr: c_int);
}
extern "C" {
    pub fn mlx4_bitmap_avail(bitmap: *mut mlx4_bitmap) -> u32;
}
extern "C" {
    pub fn mlx4_bitmap_cleanup(bitmap: *mut mlx4_bitmap);
}
extern "C" {
    pub fn mlx4_reset(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_alloc_eq_table(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_free_eq_table(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_init_pd_table(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_init_xrcd_table(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_init_uar_table(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_init_mr_table(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_init_eq_table(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_init_cq_table(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_init_qp_table(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_init_srq_table(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_init_mcg_table(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_cleanup_pd_table(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_cleanup_xrcd_table(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_cleanup_uar_table(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_cleanup_mr_table(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_cleanup_eq_table(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_cleanup_cq_table(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_cleanup_qp_table(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_cleanup_srq_table(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_cleanup_mcg_table(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn __mlx4_qp_alloc_icm(dev: *mut mlx4_dev, qpn: c_int) -> c_int;
}
extern "C" {
    pub fn __mlx4_qp_free_icm(dev: *mut mlx4_dev, qpn: c_int);
}
extern "C" {
    pub fn __mlx4_cq_alloc_icm(dev: *mut mlx4_dev, cqn: *mut c_int) -> c_int;
}
extern "C" {
    pub fn __mlx4_cq_free_icm(dev: *mut mlx4_dev, cqn: c_int);
}
extern "C" {
    pub fn __mlx4_srq_alloc_icm(dev: *mut mlx4_dev, srqn: *mut c_int) -> c_int;
}
extern "C" {
    pub fn __mlx4_srq_free_icm(dev: *mut mlx4_dev, srqn: c_int);
}
extern "C" {
    pub fn __mlx4_mpt_reserve(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn __mlx4_mpt_release(dev: *mut mlx4_dev, index: u32);
}
extern "C" {
    pub fn __mlx4_mpt_alloc_icm(dev: *mut mlx4_dev, index: u32) -> c_int;
}
extern "C" {
    pub fn __mlx4_mpt_free_icm(dev: *mut mlx4_dev, index: u32);
}
extern "C" {
    pub fn __mlx4_alloc_mtt_range(dev: *mut mlx4_dev, order: c_int) -> u32;
}
extern "C" {
    pub fn __mlx4_free_mtt_range(dev: *mut mlx4_dev, first_seg: u32, order: c_int);
}
extern "C" {
    pub fn __mlx4_qp_release_range(dev: *mut mlx4_dev, base_qpn: c_int, cnt: c_int);
}
extern "C" {
    pub fn __mlx4_register_mac(dev: *mut mlx4_dev, port: u8, mac: u64) -> c_int;
}
extern "C" {
    pub fn __mlx4_unregister_mac(dev: *mut mlx4_dev, port: u8, mac: u64);
}
extern "C" {
    pub fn __mlx4_counter_alloc(dev: *mut mlx4_dev, idx: *mut u32) -> c_int;
}
extern "C" {
    pub fn __mlx4_counter_free(dev: *mut mlx4_dev, idx: u32);
}
extern "C" {
    pub fn __mlx4_xrcd_alloc(dev: *mut mlx4_dev, xrcdn: *mut u32) -> c_int;
}
extern "C" {
    pub fn __mlx4_xrcd_free(dev: *mut mlx4_dev, xrcdn: u32);
}
extern "C" {
    pub fn mlx4_start_catas_poll(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_stop_catas_poll(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_catas_init(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_catas_end(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_crdump_init(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_crdump_end(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_restart_one(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_adev_init(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_adev_cleanup(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_register_device(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_unregister_device(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_master_comm_channel(work: *mut work_struct);
}
extern "C" {
    pub fn mlx4_gen_slave_eqe(work: *mut work_struct);
}
extern "C" {
    pub fn mlx4_master_handle_slave_flr(work: *mut work_struct);
}
extern "C" {
    pub fn mlx4_GEN_EQE(dev: *mut mlx4_dev, slave: c_int, eqe: *mut mlx4_eqe) -> c_int;
}
extern "C" {
    pub fn mlx4_cmd_init(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_cmd_cleanup(dev: *mut mlx4_dev, cleanup_mask: c_int);
}
extern "C" {
    pub fn mlx4_multi_func_init(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_ARM_COMM_CHANNEL(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_multi_func_cleanup(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_cmd_event(dev: *mut mlx4_dev, token: u16, status: u8, out_param: u64);
}
extern "C" {
    pub fn mlx4_cmd_use_events(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_cmd_use_polling(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_cq_tasklet_cb(t: *mut tasklet_struct);
}
extern "C" {
    pub fn mlx4_cq_completion(dev: *mut mlx4_dev, cqn: u32);
}
extern "C" {
    pub fn mlx4_cq_event(dev: *mut mlx4_dev, cqn: u32, event_type: c_int);
}
extern "C" {
    pub fn mlx4_qp_event(dev: *mut mlx4_dev, qpn: u32, event_type: c_int);
}
extern "C" {
    pub fn mlx4_srq_event(dev: *mut mlx4_dev, srqn: u32, event_type: c_int);
}
extern "C" {
    pub fn mlx4_enter_error_state(persist: *mut mlx4_dev_persistent);
}
extern "C" {
    pub fn mlx4_comm_internal_err(slave_read: u32) -> c_int;
}
extern "C" {
    pub fn mlx4_crdump_collect(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_start_sense(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_stop_sense(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_sense_init(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_init_mac_table(dev: *mut mlx4_dev, table: *mut mlx4_mac_table);
}
extern "C" {
    pub fn mlx4_init_vlan_table(dev: *mut mlx4_dev, table: *mut mlx4_vlan_table);
}
extern "C" {
    pub fn __mlx4_unregister_vlan(dev: *mut mlx4_dev, port: u8, vlan: u16);
}
extern "C" {
    pub fn __mlx4_register_vlan(dev: *mut mlx4_dev, port: u8, vlan: u16, index: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_bond_vlan_table(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_unbond_vlan_table(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_bond_mac_table(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_unbond_mac_table(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_SET_PORT(dev: *mut mlx4_dev, port: u8, pkey_tbl_sz: c_int) -> c_int;
}
// resource tracker functions
extern "C" {
    pub fn mlx4_delete_all_resources_for_slave(dev: *mut mlx4_dev, slave_id: c_int);
}
extern "C" {
    pub fn mlx4_reset_roce_gids(dev: *mut mlx4_dev, slave: c_int);
}
extern "C" {
    pub fn mlx4_init_resource_tracker(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_get_port_ib_caps(dev: *mut mlx4_dev, port: u8, caps: *mut __be32) -> c_int;
}
extern "C" {
    pub fn mlx4_get_mgm_entry_size(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_get_qp_per_mgm(dev: *mut mlx4_dev) -> c_int;
}
// arg = (*arg & 0xffffffff00000000ULL) | (u64) val;
// arg = (*arg & 0xffffffff) | ((u64) val << 32);
pub const NOT_MASKED_PD_BITS: c_int = 17;
extern "C" {
    pub fn mlx4_vf_immed_vlan_work_handler(_work: *mut work_struct);
}
extern "C" {
    pub fn mlx4_init_quotas(dev: *mut mlx4_dev);
}
// for VFs, replace zero MACs with randomly-generated MACs at driver start
extern "C" {
    pub fn mlx4_replace_zero_macs(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_get_slave_num_gids(dev: *mut mlx4_dev, slave: c_int, port: c_int) -> c_int;
}
// Returns the VF index of slave
extern "C" {
    pub fn mlx4_get_vf_indx(dev: *mut mlx4_dev, slave: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_config_mad_demux(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_do_bond(dev: *mut mlx4_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn mlx4_bond_fs_rules(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_unbond_fs_rules(dev: *mut mlx4_dev) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_zone_flags {
    MLX4_ZONE_ALLOW_ALLOC_FROM_LOWER_PRIO	= 1UL << 0,
    MLX4_ZONE_ALLOW_ALLOC_FROM_EQ_PRIO	= 1UL << 1,
    MLX4_ZONE_FALLBACK_TO_HIGHER_PRIO	= 1UL << 2,
    MLX4_ZONE_USE_RR			= 1UL << 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_zone_alloc_flags {
// No two objects could overlap between zones. UID
// could be left unused. If this flag is given and
// two overlapped zones are used, an object will be free'd
// from the smallest possible matching zone.
//
    MLX4_ZONE_ALLOC_FLAGS_NO_OVERLAP	= 1UL << 0,
}

// Create a new zone allocator
// Attach a mlx4_bitmap <bitmap> of priority <priority> to the zone allocator
// <zone_alloc>. Allocating an object from this zone adds an offset <offset>.
// Similarly, when searching for an object to free, this offset it taken into
// account. The use_rr mlx4_ib parameter for allocating objects from this <bitmap>
// is given through the MLX4_ZONE_USE_RR flag in <flags>.
// When an allocation fails, <zone_alloc> tries to allocate from other zones
// according to the policy set by <flags>. <puid> is the unique identifier
// received to this zone.
//
// Remove bitmap indicated by <uid> from <zone_alloc>
extern "C" {
    pub fn mlx4_zone_remove_one(zone_alloc: *mut mlx4_zone_allocator, uid: u32) -> c_int;
}
// Delete the zone allocator <zone_alloc. This function doesn't destroy
// the attached bitmaps.
//
extern "C" {
    pub fn mlx4_zone_allocator_destroy(zone_alloc: *mut mlx4_zone_allocator);
}
// Allocate <count> objects with align <align> and skip_mask <skip_mask>
// from the mlx4_bitmap whose uid is <uid>. The bitmap which we actually
// allocated from is returned in <puid>. If the allocation fails, a negative
// number is returned. Otherwise, the offset of the first object is returned.
//
// If <zones> was allocated with MLX4_ZONE_ALLOC_FLAGS_NO_OVERLAP, instead of
// specifying the uid when freeing an object, zone allocator could figure it by
// itself. Other parameters are similar to mlx4_zone_free.
//
extern "C" {
    pub fn mlx4_zone_free_entries_unique(zones: *mut mlx4_zone_allocator, obj: u32, count: u32) -> u32;
}
// Returns a pointer to mlx4_bitmap that was attached to <zones> with <uid>
