//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mlx4/mlx4_ib.h
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
// Copyright (c) 2006, 2007 Cisco Systems.  All rights reserved.
// Copyright (c) 2007, 2008 Mellanox Technologies. All rights reserved.
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

// module param to indicate if SM assigns the alias_GUID
pub const MLX4_IB_UC_STEER_QPN_ALIGN: c_int = 1;
pub const MLX4_IB_UC_MAX_NUM_QPS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_bar_type {
    HW_BAR_BF,
    HW_BAR_DB,
    HW_BAR_CLOCK,
    HW_BAR_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_ucontext {
    pub ibucontext: ib_ucontext,
    pub uar: mlx4_uar,
    pub db_page_list: list_head,
    pub db_page_mutex: mutex,
    pub wqn_ranges_list: list_head,
    pub /: *mut *mut mutex wqn_ranges_mutex; / protect wqn_ranges_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_pd {
    pub ibpd: ib_pd,
    pub pdn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_xrcd {
    pub ibxrcd: ib_xrcd,
    pub xrcdn: u32,
    pub pd: *mut ib_pd,
    pub cq: *mut ib_cq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_cq_buf {
    pub buf: mlx4_buf,
    pub mtt: mlx4_mtt,
    pub entry_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_cq_resize {
    pub buf: mlx4_ib_cq_buf,
    pub cqe: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_cq {
    pub ibcq: ib_cq,
    pub mcq: mlx4_cq,
    pub buf: mlx4_ib_cq_buf,
    pub resize_buf: *mut mlx4_ib_cq_resize,
    pub db: mlx4_db,
    pub lock: spinlock_t,
    pub resize_mutex: mutex,
    pub umem: *mut ib_umem,
    pub resize_umem: *mut ib_umem,
// List of qps that it serves.
    pub send_qp_list: list_head,
    pub recv_qp_list: list_head,
}

pub const MLX4_MR_PAGES_ALIGN: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_mr {
    pub ibmr: ib_mr,
    pub pages: *mut __be64,
    pub page_map: dma_addr_t,
    pub npages: u32,
    pub max_pages: u32,
    pub access_flags: c_int,
    pub mmr: mlx4_mr,
    pub umem: *mut ib_umem,
    pub page_map_size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_mw {
    pub ibmw: ib_mw,
    pub mmw: mlx4_mw,
}

pub const MAX_REGS_PER_FLOW: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_flow_reg_id {
    pub id: u64,
    pub mirror: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_flow {
    pub ibflow: ib_flow,
// translating DMFS verbs sniffer rule to FW API requires two reg IDs
    pub reg_id: [mlx4_flow_reg_id; MAX_REGS_PER_FLOW],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_wq {
    pub wrid: *mut u64,
    pub lock: spinlock_t,
    pub wqe_cnt: c_int,
    pub max_post: c_int,
    pub max_gs: c_int,
    pub offset: c_int,
    pub wqe_shift: c_int,
    pub head: unsigned,
    pub tail: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_ib_qp_flags {
    MLX4_IB_QP_LSO = IB_QP_CREATE_IPOIB_UD_LSO,
    MLX4_IB_QP_BLOCK_MULTICAST_LOOPBACK = IB_QP_CREATE_BLOCK_MULTICAST_LOOPBACK,
    MLX4_IB_QP_NETIF = IB_QP_CREATE_NETIF_QP,
    MLX4_IB_QP_SCATTER_FCS = IB_QP_CREATE_SCATTER_FCS,

// Mellanox specific flags start from IB_QP_CREATE_RESERVED_START
    MLX4_IB_ROCE_V2_GSI_QP = MLX4_IB_QP_CREATE_ROCE_V2_GSI,
    MLX4_IB_SRIOV_TUNNEL_QP = 1 << 30,
    MLX4_IB_SRIOV_SQP = 1 << 31,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_gid_entry {
    pub list: list_head,
    pub gid: ib_gid,
    pub added: c_int,
    pub port: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_ib_qp_type {
//
// IB_QPT_SMI and IB_QPT_GSI have to be the first two entries
// here (and in that order) since the MAD layer uses them as
// indices into a 2-entry table.
//
    MLX4_IB_QPT_SMI = IB_QPT_SMI,
    MLX4_IB_QPT_GSI = IB_QPT_GSI,

    MLX4_IB_QPT_RC = IB_QPT_RC,
    MLX4_IB_QPT_UC = IB_QPT_UC,
    MLX4_IB_QPT_UD = IB_QPT_UD,
    MLX4_IB_QPT_RAW_IPV6 = IB_QPT_RAW_IPV6,
    MLX4_IB_QPT_RAW_ETHERTYPE = IB_QPT_RAW_ETHERTYPE,
    MLX4_IB_QPT_RAW_PACKET = IB_QPT_RAW_PACKET,
    MLX4_IB_QPT_XRC_INI = IB_QPT_XRC_INI,
    MLX4_IB_QPT_XRC_TGT = IB_QPT_XRC_TGT,

    MLX4_IB_QPT_PROXY_SMI_OWNER	= 1 << 16,
    MLX4_IB_QPT_PROXY_SMI		= 1 << 17,
    MLX4_IB_QPT_PROXY_GSI		= 1 << 18,
    MLX4_IB_QPT_TUN_SMI_OWNER	= 1 << 19,
    MLX4_IB_QPT_TUN_SMI		= 1 << 20,
    MLX4_IB_QPT_TUN_GSI		= 1 << 21,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_ib_mad_ifc_flags {
    MLX4_MAD_IFC_IGNORE_MKEY	= 1,
    MLX4_MAD_IFC_IGNORE_BKEY	= 2,
    MLX4_MAD_IFC_IGNORE_KEYS	= (MLX4_MAD_IFC_IGNORE_MKEY |
    MLX4_MAD_IFC_IGNORE_BKEY),
    MLX4_MAD_IFC_NET_VIEW		= 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_tunnel_header {
    pub av: mlx4_av,
    pub remote_qpn: __be32,
    pub qkey: __be32,
    pub vlan: __be16,
    pub mac: [u8; 6],
    pub pkey_index: __be16,
    pub reserved: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_buf {
    pub addr: *mut c_void,
    pub map: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_rcv_tunnel_hdr {
    pub VLANs:: *mut *mut __be32 flags_src_qp; / flags[6:5] is defined for,
// 0x0 - no vlan was in the packet
// 0x01 - C-VLAN was in the packet
    pub /: *mut *mut u8 g_ml_path; / gid bit stands for ipv6/4 header in RoCE,
    pub reserved: u8,
    pub pkey_index: __be16,
    pub sl_vid: __be16,
    pub slid_mac_47_32: __be16,
    pub mac_31_0: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_proxy_sqp_hdr {
    pub grh: ib_grh,
    pub tun: mlx4_rcv_tunnel_hdr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_roce_smac_vlan_info {
    pub smac: u64,
    pub smac_index: c_int,
    pub smac_port: c_int,
    pub candidate_smac: u64,
    pub candidate_smac_index: c_int,
    pub candidate_smac_port: c_int,
    pub vid: u16,
    pub vlan_index: c_int,
    pub vlan_port: c_int,
    pub candidate_vid: u16,
    pub candidate_vlan_index: c_int,
    pub candidate_vlan_port: c_int,
    pub update_vid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_wqn_range {
    pub base_wqn: c_int,
    pub size: c_int,
    pub refcount: c_int,
    pub dirty: bool,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_rss {
    pub base_qpn_tbl_sz: c_uint,
    pub flags: u8,
    pub rss_key: [u8; MLX4_EN_RSS_KEY_SIZE],
}

//
// Largest possible UD header: send with GRH and immediate
// data plus 18 bytes for an Ethernet header with VLAN/802.1Q
// tag.  (LRH would only use 8 bytes, so Ethernet is the
// biggest case)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_sqp {
    pub pkey_index: c_int,
    pub qkey: u32,
    pub send_psn: u32,
    pub ud_header: ib_ud_header,
    pub header_buf: [u8; MLX4_IB_UD_HEADER_SIZE],
    pub roce_v2_gsi: *mut ib_qp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_qp {
    pub ibqp: ib_qp,
    pub ibwq: ib_wq,
}

// Number of RSS QP parents that uses this WQ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_srq {
    pub ibsrq: ib_srq,
    pub msrq: mlx4_srq,
    pub buf: mlx4_buf,
    pub db: mlx4_db,
    pub wrid: *mut u64,
    pub lock: spinlock_t,
    pub head: c_int,
    pub tail: c_int,
    pub wqe_ctr: u16,
    pub umem: *mut ib_umem,
    pub mtt: mlx4_mtt,
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_ah {
    pub ibah: ib_ah,
    pub av: mlx4_ext_av,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_rwq_ind_table {
    pub ib_rwq_ind_tbl: ib_rwq_ind_table,
}

//
// alias guid support
//
pub const NUM_PORT_ALIAS_GUID: c_int = 2;
pub const NUM_ALIAS_GUID_IN_REC: c_int = 8;
pub const NUM_ALIAS_GUID_REC_IN_PORT: c_int = 16;
pub const GUID_REC_SIZE: c_int = 8;
pub const NUM_ALIAS_GUID_PER_PORT: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_guid_alias_rec_status {
    MLX4_GUID_INFO_STATUS_IDLE,
    MLX4_GUID_INFO_STATUS_SET,
}

pub const GUID_STATE_NEED_PORT_INIT: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_guid_alias_rec_method {
    MLX4_GUID_INFO_RECORD_SET	= IB_MGMT_METHOD_SET,
    MLX4_GUID_INFO_RECORD_DELETE	= IB_SA_METHOD_DELETE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_sriov_alias_guid_info_rec_det {
    pub NUM_ALIAS_GUID_IN_REC]: *mut *mut u8 all_recs[GUID_REC_SIZE,
    pub valid*/: *mut *mut ib_sa_comp_mask guid_indexes; /indicates what from the 8 records are,
    pub record.*/: *mut *mut mlx4_guid_alias_rec_status status; /indicates the administraively status of the,
    pub guids_retry_schedule: [c_uint; NUM_ALIAS_GUID_IN_REC],
    pub time_to_run: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_sriov_alias_guid_port_rec_det {
    pub all_rec_per_port: [mlx4_sriov_alias_guid_info_rec_det; NUM_ALIAS_GUID_REC_IN_PORT],
    pub wq: *mut workqueue_struct,
    pub alias_guid_work: delayed_work,
    pub port: u32,
    pub state_flags: u32,
    pub parent: *mut mlx4_sriov_alias_guid,
    pub cb_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_sriov_alias_guid {
    pub ports_guid: [mlx4_sriov_alias_guid_port_rec_det; MLX4_MAX_PORTS],
    pub ag_work_lock: spinlock_t,
    pub sa_client: *mut ib_sa_client,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_demux_work {
    pub work: work_struct,
    pub dev: *mut mlx4_ib_dev,
    pub slave: c_int,
    pub do_init: c_int,
    pub port: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_tun_tx_buf {
    pub buf: mlx4_ib_buf,
    pub ah: *mut ib_ah,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_demux_pv_qp {
    pub qp: *mut ib_qp,
    pub proxy_qpt: ib_qp_type,
    pub ring: *mut mlx4_ib_buf,
    pub tx_ring: *mut mlx4_ib_tun_tx_buf,
    pub tx_lock: spinlock_t,
    pub tx_ix_head: unsigned,
    pub tx_ix_tail: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_ib_demux_pv_state {
    DEMUX_PV_STATE_DOWN,
    DEMUX_PV_STATE_STARTING,
    DEMUX_PV_STATE_ACTIVE,
    DEMUX_PV_STATE_DOWNING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_demux_pv_ctx {
    pub port: c_int,
    pub slave: c_int,
    pub state: mlx4_ib_demux_pv_state,
    pub has_smi: c_int,
    pub ib_dev: *mut ib_device,
    pub cq: *mut ib_cq,
    pub pd: *mut ib_pd,
    pub work: work_struct,
    pub wq: *mut workqueue_struct,
    pub wi_wq: *mut workqueue_struct,
    pub qp: [mlx4_ib_demux_pv_qp; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_demux_ctx {
    pub ib_dev: *mut ib_device,
    pub port: c_int,
    pub wq: *mut workqueue_struct,
    pub wi_wq: *mut workqueue_struct,
    pub ud_wq: *mut workqueue_struct,
    pub ud_lock: spinlock_t,
    pub subnet_prefix: core::sync::atomic::AtomicI64,
    pub guid_cache: [__be64; 128],
    pub dev: *mut mlx4_ib_dev,
// the following lock protects both mcg_table and mcg_mgid0_list
    pub mcg_table_lock: mutex,
    pub mcg_table: rb_root,
    pub mcg_mgid0_list: list_head,
    pub mcg_wq: *mut workqueue_struct,
    pub tun: *mut mlx4_ib_demux_pv_ctx,
    pub tid: core::sync::atomic::AtomicI32,
    pub /: *mut *mut int flushing; / flushing the work queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_sriov {
    pub demux: [mlx4_ib_demux_ctx; MLX4_MAX_PORTS],
    pub sqps: [*mut mlx4_ib_demux_pv_ctx; MLX4_MAX_PORTS],
// when using this spinlock you should use "irq" because
// it may be called from interrupt context.
    pub going_down_lock: spinlock_t,
    pub is_going_down: c_int,
    pub alias_guid: mlx4_sriov_alias_guid,
// CM paravirtualization fields
    pub pv_id_table: xarray,
    pub pv_id_next: u32,
    pub id_map_lock: spinlock_t,
    pub sl_id_map: rb_root,
    pub cm_list: list_head,
    pub xa_rej_tmout: xarray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gid_cache_context {
    pub real_index: c_int,
    pub refcount: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gid_entry {
    pub gid: ib_gid,
    pub gid_type: ib_gid_type,
    pub ctx: *mut gid_cache_context,
    pub vlan_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_port_gid_table {
    pub gids: [gid_entry; MLX4_MAX_PORT_GIDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_iboe {
    pub lock: spinlock_t,
    pub netdevs: [*mut net_device; MLX4_MAX_PORTS],
    pub mac: [core::sync::atomic::AtomicI64; MLX4_MAX_PORTS],
    pub nb: notifier_block,
    pub gids: [mlx4_port_gid_table; MLX4_MAX_PORTS],
    pub last_port_state: [ib_port_state; MLX4_MAX_PORTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_mgt {
    pub virt2phys_pkey: [u8; MLX4_MFUNC_MAX][MLX4_MAX_PORTS][MLX4_MAX_PORT_PKEYS],
    pub phys_pkey_cache: [u16; MLX4_MAX_PORTS][MLX4_MAX_PORT_PKEYS],
    pub pkey_port_list: [list_head; MLX4_MFUNC_MAX],
    pub device_parent: [*mut kobject; MLX4_MFUNC_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_iov_sysfs_attr {
    pub ctx: *mut c_void,
    pub kobj: *mut kobject,
    pub data: c_ulong,
    pub entry_num: u32,
    pub name: [c_char; 15],
    pub dentry: device_attribute,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_iov_sysfs_attr_ar {
    pub 1]: *mut *mut mlx4_ib_iov_sysfs_attr dentries[3  NUM_ALIAS_GUID_PER_PORT +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_iov_port {
    pub name: [c_char; 100],
    pub num: u8,
    pub dev: *mut mlx4_ib_dev,
    pub list: list_head,
    pub dentr_ar: *mut mlx4_ib_iov_sysfs_attr_ar,
    pub attr: ib_port_attr,
    pub cur_port: *mut kobject,
    pub admin_alias_parent: *mut kobject,
    pub gids_parent: *mut kobject,
    pub pkeys_parent: *mut kobject,
    pub mcgs_parent: *mut kobject,
    pub mcg_dentry: mlx4_ib_iov_sysfs_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_index {
    pub list: list_head,
    pub index: u32,
    pub allocated: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_counters {
    pub counters_list: list_head,
    pub /: *mut *mut mutex mutex; / mutex for accessing counters list,
    pub default_counter: u32,
}

pub const MLX4_DIAG_COUNTERS_TYPES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_diag_counters {
    pub descs: *mut rdma_stat_desc,
    pub offset: *mut u32,
    pub num_counters: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_dev {
    pub ib_dev: ib_device,
    pub dev: *mut mlx4_dev,
    pub num_ports: c_int,
    pub uar_map: *mut void __iomem,
    pub priv_uar: mlx4_uar,
    pub priv_pdn: u32,
    pub send_agent: [*mut ib_mad_agent; MLX4_MAX_PORTS][2],
    pub sm_ah: [*mut ib_ah; MLX4_MAX_PORTS],
    pub sm_lock: spinlock_t,
    pub sl2vl: [core::sync::atomic::AtomicI64; MLX4_MAX_PORTS],
    pub sriov: mlx4_ib_sriov,
    pub cap_mask_mutex: mutex,
    pub ib_active: bool,
    pub iboe: mlx4_ib_iboe,
    pub counters_table: [mlx4_ib_counters; MLX4_MAX_PORTS],
    pub eq_table: *mut c_int,
    pub iov_parent: *mut kobject,
    pub ports_parent: *mut kobject,
    pub dev_ports_parent: [*mut kobject; MLX4_MFUNC_MAX],
    pub iov_ports: [mlx4_ib_iov_port; MLX4_MAX_PORTS],
    pub pkeys: pkey_mgt,
    pub ib_uc_qpns_bitmap: *mut c_ulong,
    pub steer_qpn_count: c_int,
    pub steer_qpn_base: c_int,
    pub steering_support: c_int,
    pub qp1_proxy: [*mut mlx4_ib_qp; MLX4_MAX_PORTS],
// lock when destroying qp1_proxy and getting netdev events
    pub qp1_proxy_lock: [mutex; MLX4_MAX_PORTS],
    pub bond_next_port: u8,
// protect resources needed as part of reset flow
    pub reset_flow_resource_lock: spinlock_t,
    pub qp_list: list_head,
    pub diag_counters: [mlx4_ib_diag_counters; MLX4_DIAG_COUNTERS_TYPES],
    pub mlx_nb: notifier_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_event_work {
    pub work: work_struct,
    pub ib_dev: *mut mlx4_ib_dev,
    pub ib_eqe: mlx4_eqe,
    pub port: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_qp_tunnel_init_attr {
    pub init_attr: ib_qp_init_attr,
    pub slave: c_int,
    pub proxy_qp_type: ib_qp_type,
    pub port: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_uverbs_ex_query_device {
    pub comp_mask: __u32,
    pub reserved: __u32,
}

// 4k - 4G

extern "C" {
    pub fn container_of(_arg: ibdev, mlx4_ib_dev: struct, _arg: ib_dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibucontext, mlx4_ib_ucontext: struct, _arg: ibucontext) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibpd, mlx4_ib_pd: struct, _arg: ibpd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibxrcd, mlx4_ib_xrcd: struct, _arg: ibxrcd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibcq, mlx4_ib_cq: struct, _arg: ibcq) -> return;
}
extern "C" {
    pub fn container_of(_arg: mcq, mlx4_ib_cq: struct, _arg: mcq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmr, mlx4_ib_mr: struct, _arg: ibmr) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmw, mlx4_ib_mw: struct, _arg: ibmw) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibflow, mlx4_ib_flow: struct, _arg: ibflow) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibqp, mlx4_ib_qp: struct, _arg: ibqp) -> return;
}
extern "C" {
    pub fn container_of(_arg: mqp, mlx4_ib_qp: struct, _arg: mqp) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibsrq, mlx4_ib_srq: struct, _arg: ibsrq) -> return;
}
extern "C" {
    pub fn container_of(_arg: msrq, mlx4_ib_srq: struct, _arg: msrq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibah, mlx4_ib_ah: struct, _arg: ibah) -> return;
}
extern "C" {
    pub fn mlx4_ib_init_sriov(dev: *mut mlx4_ib_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_close_sriov(dev: *mut mlx4_ib_dev);
}
extern "C" {
    pub fn mlx4_ib_db_unmap_user(context: *mut mlx4_ib_ucontext, db: *mut mlx4_db);
}
extern "C" {
    pub fn mlx4_ib_dereg_mr(mr: *mut ib_mr, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_alloc_mw(mw: *mut ib_mw, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_dealloc_mw(mw: *mut ib_mw) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_modify_cq(cq: *mut ib_cq, cq_count: u16, cq_period: u16) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_destroy_cq(cq: *mut ib_cq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_poll_cq(ibcq: *mut ib_cq, num_entries: c_int, wc: *mut ib_wc) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_arm_cq(cq: *mut ib_cq, flags: ib_cq_notify_flags) -> c_int;
}
extern "C" {
    pub fn __mlx4_ib_cq_clean(cq: *mut mlx4_ib_cq, qpn: u32, srq: *mut mlx4_ib_srq);
}
extern "C" {
    pub fn mlx4_ib_cq_clean(cq: *mut mlx4_ib_cq, qpn: u32, srq: *mut mlx4_ib_srq);
}
extern "C" {
    pub fn mlx4_ib_query_ah(ibah: *mut ib_ah, ah_attr: *mut rdma_ah_attr) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_query_srq(srq: *mut ib_srq, srq_attr: *mut ib_srq_attr) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_destroy_srq(srq: *mut ib_srq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_free_srq_wqe(srq: *mut mlx4_ib_srq, wqe_index: c_int);
}
extern "C" {
    pub fn mlx4_ib_destroy_qp(qp: *mut ib_qp, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_drain_sq(qp: *mut ib_qp);
}
extern "C" {
    pub fn mlx4_ib_drain_rq(qp: *mut ib_qp);
}
extern "C" {
    pub fn mlx4_ib_mad_init(dev: *mut mlx4_ib_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_mad_cleanup(dev: *mut mlx4_ib_dev);
}
extern "C" {
    pub fn mlx4_ib_mcg_port_init(ctx: *mut mlx4_ib_demux_ctx) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_mcg_port_cleanup(ctx: *mut mlx4_ib_demux_ctx, destroy_wq: c_int);
}
extern "C" {
    pub fn clean_vf_mcast(ctx: *mut mlx4_ib_demux_ctx, slave: c_int);
}
extern "C" {
    pub fn mlx4_ib_mcg_init() -> c_int;
}
extern "C" {
    pub fn mlx4_ib_mcg_destroy();
}
extern "C" {
    pub fn mlx4_ib_find_real_gid(ibdev: *mut ib_device, port: u32, guid: __be64) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_tunnels_update_work(work: *mut work_struct);
}
extern "C" {
    pub fn mlx4_ib_get_new_demux_tid(ctx: *mut mlx4_ib_demux_ctx) -> __be64;
}
extern "C" {
    pub fn mlx4_ib_cm_paravirt_init(dev: *mut mlx4_ib_dev);
}
extern "C" {
    pub fn mlx4_ib_cm_paravirt_clean(dev: *mut mlx4_ib_dev, slave_id: c_int);
}
// alias guid support
extern "C" {
    pub fn mlx4_ib_init_alias_guid_work(dev: *mut mlx4_ib_dev, port: c_int);
}
extern "C" {
    pub fn mlx4_ib_init_alias_guid_service(dev: *mut mlx4_ib_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_destroy_alias_guid_service(dev: *mut mlx4_ib_dev);
}
extern "C" {
    pub fn mlx4_ib_invalidate_all_guid_record(dev: *mut mlx4_ib_dev, port: c_int);
}
extern "C" {
    pub fn mlx4_ib_get_aguid_comp_mask_from_ix(index: c_int) -> ib_sa_comp_mask;
}
extern "C" {
    pub fn mlx4_ib_device_register_sysfs(device: *mut mlx4_ib_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_device_unregister_sysfs(device: *mut mlx4_ib_dev);
}
extern "C" {
    pub fn mlx4_ib_gen_node_guid() -> __be64;
}
extern "C" {
    pub fn mlx4_ib_steer_qp_alloc(dev: *mut mlx4_ib_dev, count: c_int, qpn: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_ib_steer_qp_free(dev: *mut mlx4_ib_dev, qpn: u32, count: c_int);
}
extern "C" {
    pub fn mlx4_ib_sl2vl_update(mdev: *mut mlx4_ib_dev, port: c_int);
}
extern "C" {
    pub fn mlx4_ib_destroy_wq(wq: *mut ib_wq, udata: *mut ib_udata) -> c_int;
}
// num_of_mtts = ib_umem_num_dma_blocks(umem, pg_sz);
extern "C" {
    pub fn order_base_2(_arg: pg_sz) -> return;
}
extern "C" {
    pub fn mlx4_ib_cm_init() -> c_int;
}
extern "C" {
    pub fn mlx4_ib_cm_destroy();
}
extern "C" {
    pub fn mlx4_ib_qp_event_init() -> c_int;
}
extern "C" {
    pub fn mlx4_ib_qp_event_cleanup();
}
