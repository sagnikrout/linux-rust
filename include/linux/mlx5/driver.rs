//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx5/driver.h
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
// Copyright (c) 2013-2015, Mellanox Technologies. All rights reserved.
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_sqp_t {
    MLX5_SQP_SMI		= 0,
    MLX5_SQP_GSI		= 1,
    MLX5_SQP_IEEE_1588	= 2,
    MLX5_SQP_SNIFFER	= 3,
    MLX5_SQP_SYNC_UMR	= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_qpts_trust_state {
    MLX5_QPTS_TRUST_PCP  = 1,
    MLX5_QPTS_TRUST_DSCP = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_dcbx_oper_mode {
    MLX5E_DCBX_PARAM_VER_OPER_HOST  = 0x0,
    MLX5E_DCBX_PARAM_VER_OPER_AUTO  = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_page_fault_resume_flags {
    MLX5_PAGE_FAULT_RESUME_REQUESTOR = 1 << 0,
    MLX5_PAGE_FAULT_RESUME_WRITE	 = 1 << 1,
    MLX5_PAGE_FAULT_RESUME_RDMA	 = 1 << 2,
    MLX5_PAGE_FAULT_RESUME_ERROR	 = 1 << 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbg_rsc_type {
    MLX5_DBG_RSC_QP,
    MLX5_DBG_RSC_EQ,
    MLX5_DBG_RSC_CQ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port_state_policy {
    MLX5_POLICY_DOWN	= 0,
    MLX5_POLICY_UP		= 1,
    MLX5_POLICY_FOLLOW	= 2,
    MLX5_POLICY_INVALID	= 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_coredev_type {
    MLX5_COREDEV_PF,
    MLX5_COREDEV_VF,
    MLX5_COREDEV_SF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_field_desc {
    pub i: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_rsc_debug {
    pub dev: *mut mlx5_core_dev,
    pub object: *mut c_void,
    pub type: dbg_rsc_type,
    pub root: *mut dentry,
    pub fields: [mlx5_field_desc; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_dev_event {
    MLX5_DEV_EVENT_SYS_ERROR = 128, /* 0 - 127 are FW events */
    MLX5_DEV_EVENT_PORT_AFFINITY = 129,
    MLX5_DEV_EVENT_MULTIPORT_ESW = 130,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_port_status {
    MLX5_PORT_UP        = 1,
    MLX5_PORT_DOWN      = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_cmdif_state {
    MLX5_CMDIF_STATE_UNINITIALIZED,
    MLX5_CMDIF_STATE_UP,
    MLX5_CMDIF_STATE_DOWN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cmd_first {
    pub data: [__be32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cmd_msg {
    pub list: list_head,
    pub parent: *mut cmd_msg_cache,
    pub len: u32,
    pub first: mlx5_cmd_first,
    pub next: *mut mlx5_cmd_mailbox,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cmd_debug {
    pub dbg_root: *mut dentry,
    pub in_msg: *mut c_void,
    pub out_msg: *mut c_void,
    pub status: u8,
    pub inlen: u16,
    pub outlen: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_msg_cache {
// protect block chain allocations
//
    pub lock: spinlock_t,
    pub head: list_head,
    pub max_inbox_size: c_uint,
    pub num_ent: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cmd_stats {
    pub sum: u64,
    pub n: u64,
// number of times command failed
    pub failed: u64,
// number of times command failed on bad status returned by FW
    pub failed_mbox_status: u64,
// last command failed returned errno
    pub last_failed_errno: u32,
// last bad status returned by FW
    pub last_failed_mbox_status: u8,
// last command failed syndrome returned by FW
    pub last_failed_syndrome: u32,
    pub root: *mut dentry,
// protect command average calculations
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cmd {
    pub nb: mlx5_nb,
// members which needs to be queried or reinitialized each reload
    pub cmdif_rev: u16,
    pub log_sz: u8,
    pub log_stride: u8,
    pub max_reg_cmds: c_int,
    pub bitmask: c_ulong,
    pub sem: semaphore,
    pub pages_sem: semaphore,
    pub throttle_sem: semaphore,
    pub unprivileged_sem: semaphore,
    pub privileged_uids: xarray,
    pub vars: },
    pub state: mlx5_cmdif_state,
    pub cmd_alloc_buf: *mut c_void,
    pub alloc_dma: dma_addr_t,
    pub alloc_size: c_int,
    pub cmd_buf: *mut c_void,
    pub dma: dma_addr_t,
// protect command queue allocations
//
    pub alloc_lock: spinlock_t,
// protect token allocations
//
    pub token_lock: spinlock_t,
    pub token: u8,
    pub wq_name: [c_char; MLX5_CMD_WQ_MAX_NAME],
    pub wq: *mut workqueue_struct,
    pub mode: c_int,
    pub allowed_opcode: u16,
    pub ent_arr: [*mut mlx5_cmd_work_ent; MLX5_MAX_COMMANDS],
    pub pool: *mut dma_pool,
    pub dbg: mlx5_cmd_debug,
    pub cache: [cmd_msg_cache; MLX5_NUM_COMMAND_CACHES],
    pub checksum_disabled: c_int,
    pub stats: xarray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cmd_mailbox {
    pub buf: *mut c_void,
    pub dma: dma_addr_t,
    pub next: *mut mlx5_cmd_mailbox,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_buf_list {
    pub buf: *mut c_void,
    pub map: dma_addr_t,
    pub frag_page: *mut mlx5_dma_pool_page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_frag_buf {
    pub frags: *mut mlx5_buf_list,
    pub npages: c_int,
    pub size: c_int,
    pub page_shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_frag_buf_ctrl {
    pub frags: *mut mlx5_buf_list,
    pub sz_m1: u32,
    pub frag_sz_m1: u16,
    pub strides_offset: u16,
    pub log_sz: u8,
    pub log_stride: u8,
    pub log_frag_strides: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_core_psv {
    pub psv_idx: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psv_layout {
    pub pd: u32,
    pub syndrome: u16,
    pub reserved: u16,
    pub bg: u16,
    pub app_tag: u16,
    pub ref_tag: u32,
    pub psv: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_core_sig_ctx {
    pub psv_memory: mlx5_core_psv,
    pub psv_wire: mlx5_core_psv,
    pub err_item: ib_sig_err,
    pub sig_status_checked: bool,
    pub sig_err_exists: bool,
    pub sigerr_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_res_type {
    MLX5_RES_QP	= MLX5_EVENT_QUEUE_TYPE_QP,
    MLX5_RES_RQ	= MLX5_EVENT_QUEUE_TYPE_RQ,
    MLX5_RES_SQ	= MLX5_EVENT_QUEUE_TYPE_SQ,
    MLX5_RES_SRQ	= 3,
    MLX5_RES_XSRQ	= 4,
    MLX5_RES_XRQ	= 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_core_rsc_common {
    pub res: mlx5_res_type,
    pub refcount: refcount_t,
    pub free: completion,
    pub invalid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_uars_page {
    pub map: *mut void __iomem,
    pub wc: bool,
    pub index: u32,
    pub list: list_head,
    pub bfregs: c_uint,
    pub /: *mut *mut *mut unsigned long reg_bitmap; / for non fast path bf regs,
    pub fp_bitmap: *mut c_ulong,
    pub reg_avail: c_uint,
    pub fp_avail: c_uint,
    pub ref_count: kref,
    pub mdev: *mut mlx5_core_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_bfreg_head {
// protect blue flame registers allocations
    pub lock: mutex,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_bfreg_data {
    pub reg_head: mlx5_bfreg_head,
    pub wc_head: mlx5_bfreg_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_sq_bfreg {
    pub map: *mut void __iomem,
    pub up: *mut mlx5_uars_page,
    pub wc: bool,
    pub index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_core_health {
    pub health: *mut health_buffer __iomem,
    pub health_counter: *mut __be32 __iomem,
    pub timer: timer_list,
    pub prev: u32,
    pub miss_counter: c_int,
    pub synd: u8,
    pub fatal_error: u32,
    pub crdump_size: u32,
    pub wq: *mut workqueue_struct,
    pub flags: c_ulong,
    pub fatal_report_work: work_struct,
    pub report_work: work_struct,
    pub fw_reporter: *mut devlink_health_reporter,
    pub fw_fatal_reporter: *mut devlink_health_reporter,
    pub vnic_reporter: *mut devlink_health_reporter,
    pub update_fw_log_ts_work: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vf_context {
    pub enabled: c_int,
    pub port_guid: u64,
    pub node_guid: u64,
// Valid bits are used to validate administrative guid only.
// Enabled after ndo_set_vf_guid
//
    pub port_guid_valid:1: u8,
    pub node_guid_valid:1: u8,
    pub policy: port_state_policy,
    pub notifier: blocking_notifier_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_core_sriov {
    pub vfs_ctx: *mut mlx5_vf_context,
    pub num_vfs: c_int,
    pub max_vfs: u16,
    pub max_ec_vfs: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_rate_limit {
    pub rate: u32,
    pub max_burst_sz: u32,
    pub typical_pkt_sz: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_rl_entry {
    pub rl_raw: [u8; MLX5_ST_SZ_BYTES(set_pp_rate_limit_context)],
    pub refcount: u64,
    pub index: u16,
    pub uid: u16,
    pub 1: u8 dedicated :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_rl_table {
// protect rate limit table
    pub rl_lock: mutex,
    pub max_size: u16,
    pub max_rate: u32,
    pub min_rate: u32,
    pub rl_entry: *mut mlx5_rl_entry,
    pub refcount: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_core_roce {
    pub ft: *mut mlx5_flow_table,
    pub fg: *mut mlx5_flow_group,
    pub allow_rule: *mut mlx5_flow_handle,
}

// Set during device detach to block any further devices
// creation/deletion on drivers rescan. Unset during device attach.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_adev {
    pub adev: auxiliary_device,
    pub mdev: *mut mlx5_core_dev,
    pub idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_debugfs_entries {
    pub dbg_root: *mut dentry,
    pub qp_debugfs: *mut dentry,
    pub eq_debugfs: *mut dentry,
    pub cq_debugfs: *mut dentry,
    pub cmdif_debugfs: *mut dentry,
    pub frag_buf_dma_pools_debugfs: *mut dentry,
    pub db_dma_pools_debugfs: *mut dentry,
    pub pages_debugfs: *mut dentry,
    pub lag_debugfs: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_func_type {
    MLX5_SELF,
    MLX5_VF,
    MLX5_SF,
    MLX5_HOST_PF,
    MLX5_SPF,
    MLX5_EC_VF,
    MLX5_FUNC_TYPE_NUM,
    MLX5_FUNC_TYPE_NONE = MLX5_FUNC_TYPE_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_page_mgt_mode {
    MLX5_PAGE_MGT_MODE_FUNC_ID,
    MLX5_PAGE_MGT_MODE_VHCA_ID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_priv {
// IRQ table valid only for real pci devices PF or VF
    pub irq_table: *mut mlx5_irq_table,
    pub eq_table: *mut mlx5_eq_table,
// pages stuff
    pub pg_nb: mlx5_nb,
    pub pg_wq: *mut workqueue_struct,
    pub page_root_xa: xarray,
    pub reg_pages: core::sync::atomic::AtomicI32,
    pub free_list: list_head,
    pub fw_pages: u32,
    pub page_counters: [u32; MLX5_FUNC_TYPE_NUM],
    pub fw_pages_alloc_failed: u32,
    pub give_pages_dropped: u32,
    pub reclaim_pages_discard: u32,
    pub page_mgt_mode: mlx5_page_mgt_mode,
    pub health: mlx5_core_health,
    pub traps: list_head,
    pub dbg: mlx5_debugfs_entries,
// start: alloc stuff
// protect buffer allocation according to numa node
    pub alloc_mutex: mutex,
    pub numa_node: c_int,
    pub frag_buf_node_pools: *mut mlx5_frag_buf_node_pools,
    pub db_node_pools: *mut mlx5_dma_pool,
// end: alloc stuff
    pub adev: *mut mlx5_adev,
    pub adev_idx: c_int,
    pub sw_vhca_id: c_int,
    pub events: *mut mlx5_events,
    pub vhca_events: *mut mlx5_vhca_events,
    pub steering: *mut mlx5_flow_steering,
    pub mpfs: *mut mlx5_mpfs,
    pub esw_n_head: blocking_notifier_head,
    pub eswitch: *mut mlx5_eswitch,
    pub sriov: mlx5_core_sriov,
    pub lag: *mut mlx5_lag,
    pub flags: u32,
    pub devc: *mut mlx5_devcom_dev,
    pub hca_devcom_comp: *mut mlx5_devcom_comp_dev,
    pub fw_reset: *mut mlx5_fw_reset,
    pub roce: mlx5_core_roce,
    pub fc_stats: *mut mlx5_fc_stats,
    pub rl_table: mlx5_rl_table,
    pub ft_pool: *mut mlx5_ft_pool,
    pub bfregs: mlx5_bfreg_data,
    pub bfreg: mlx5_sq_bfreg,

    pub vhca_state_nb: mlx5_nb,
    pub vhca_state_n_head: blocking_notifier_head,
    pub sf_dev_nb: notifier_block,
    pub sf_dev_table: *mut mlx5_sf_dev_table,
    pub parent_mdev: *mut mlx5_core_dev,

    pub sf_hw_table_vhca_nb: notifier_block,
    pub sf_hw_table: *mut mlx5_sf_hw_table,
    pub sf_table_esw_nb: notifier_block,
    pub sf_table_vhca_nb: notifier_block,
    pub sf_table_mdev_nb: notifier_block,
    pub sf_table: *mut mlx5_sf_table,

    pub lag_nh: blocking_notifier_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_device_state {
    MLX5_DEVICE_STATE_UP = 1,
    MLX5_DEVICE_STATE_INTERNAL_ERROR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_interface_state {
    MLX5_INTERFACE_STATE_UP = BIT(0),
    MLX5_BREAK_FW_WAIT = BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_pci_status {
    MLX5_PCI_STATUS_DISABLED,
    MLX5_PCI_STATUS_ENABLED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_pagefault_type_flags {
    MLX5_PFAULT_REQUESTOR = 1 << 0,
    MLX5_PFAULT_WRITE     = 1 << 1,
    MLX5_PFAULT_RDMA      = 1 << 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_td {
// protects tirs list changes while tirs refresh
    pub list_lock: mutex,
    pub tirs_list: list_head,
    pub tdn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_resources {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_hw_objs {
    pub pdn: u32,
    pub td: mlx5_td,
    pub mkey: u32,
    pub bfregs: *mut mlx5_sq_bfreg,
    pub num_bfregs: c_uint,
pub const MLX5_MAX_NUM_TC: c_int = 8;
    pub tisn: [u32; MLX5_MAX_PORTS][MLX5_MAX_NUM_TC],
    pub tisn_valid: bool,
    pub hw_objs: },
    pub uplink_netdev: *mut net_device,
    pub tracker: netdevice_tracker,
    pub uplink_netdev_lock: mutex,
    pub dek_priv: *mut mlx5_crypto_dek_priv,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_sw_icm_type {
    MLX5_SW_ICM_TYPE_STEERING,
    MLX5_SW_ICM_TYPE_HEADER_MODIFY,
    MLX5_SW_ICM_TYPE_HEADER_MODIFY_PATTERN,
    MLX5_SW_ICM_TYPE_SW_ENCAP,
}

pub const MLX5_MAX_RESERVED_GIDS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_rsvd_gids {
    pub start: c_uint,
    pub count: c_uint,
    pub ida: ida,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_profile {
    pub mask: u64,
    pub log_max_qp: u8,
    pub num_cmd_caches: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_hca_cap {
    pub cur: [u32; MLX5_UN_SZ_DW(hca_cap_union)],
    pub max: [u32; MLX5_UN_SZ_DW(hca_cap_union)],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_wc_state {
    MLX5_WC_STATE_UNINITIALIZED,
    MLX5_WC_STATE_UNSUPPORTED,
    MLX5_WC_STATE_SUPPORTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_core_dev {
    pub device: *mut device,
    pub coredev_type: mlx5_coredev_type,
    pub pdev: *mut pci_dev,
// sync pci state
    pub pci_status_mutex: mutex,
    pub pci_status: mlx5_pci_status,
    pub rev_id: u8,
    pub board_id: [c_char; MLX5_BOARD_ID_LEN],
    pub cmd: mlx5_cmd,
    pub hca: [*mut mlx5_hca_cap; MLX5_CAP_NUM],
    pub pcam: [u32; MLX5_ST_SZ_DW(pcam_reg)],
    pub mcam: [u32; MLX5_MCAM_REGS_NUM][MLX5_ST_SZ_DW(mcam_reg)],
    pub fpga: [u32; MLX5_ST_SZ_DW(fpga_cap)],
    pub qcam: [u32; MLX5_ST_SZ_DW(qcam_reg)],
    pub embedded_cpu: u8,
    pub caps: },
    pub timeouts: *mut mlx5_timeouts,
    pub sys_image_guid: u64,
    pub iseg: *mut mlx5_init_seg __iomem,
    pub bar_addr: phys_addr_t,
    pub state: mlx5_device_state,
// sync interface state
    pub intf_state_mutex: mutex,
    pub lock_key: lock_class_key,
    pub intf_state: c_ulong,
    pub priv: mlx5_priv,
    pub profile: mlx5_profile,
    pub issi: u32,
    pub mlx5e_res: mlx5e_resources,
    pub dm: *mut mlx5_dm,
    pub st: *mut mlx5_st,
    pub vxlan: *mut mlx5_vxlan,
    pub geneve: *mut mlx5_geneve,
    pub reserved_gids: mlx5_rsvd_gids,
    pub roce_en: u32,
    pub roce: },

    pub fpga: *mut mlx5_fpga_device,

    pub clock: *mut mlx5_clock,
    pub clock_state: *mut mlx5_clock_dev_state,
    pub clock_info: *mut mlx5_ib_clock_info,
    pub tracer: *mut mlx5_fw_tracer,
    pub rsc_dump: *mut mlx5_rsc_dump,
    pub vsc_addr: u32,
    pub hv_vhca: *mut mlx5_hv_vhca,
    pub hwmon: *mut mlx5_hwmon,
    pub num_block_tc: u64,
    pub num_block_ipsec: u64,

    pub macsec_fs: *mut mlx5_macsec_fs,
// MACsec notifier chain to sync MACsec core and IB database
    pub macsec_nh: blocking_notifier_head,

    pub num_ipsec_offloads: u64,
    pub sd: *mut mlx5_sd,
    pub wc_state: mlx5_wc_state,
// sync write combining state
    pub wc_state_lock: mutex,
    pub shd: *mut devlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_db {
    pub db: *mut __be32,
    pub pool_page: *mut mlx5_dma_pool_page,
    pub user_page: *mut mlx5_ib_user_db_page,
    pub u: },
    pub dma: dma_addr_t,
    pub index: c_int,
}

pub const MLX5_DEFAULT_NUM_DOORBELLS: c_int = 8;
extern "C" {
    pub fn void(status: *mut *mut mlx5_cmd_cbk_t)(int, context: *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cmd_work_ent {
    pub state: c_ulong,
    pub in: *mut mlx5_cmd_msg,
    pub out: *mut mlx5_cmd_msg,
    pub uout: *mut c_void,
    pub uout_size: c_int,
    pub callback: mlx5_cmd_cbk_t,
    pub cb_timeout_work: delayed_work,
    pub context: *mut c_void,
    pub idx: c_int,
    pub handling: completion,
    pub slotted: completion,
    pub done: completion,
    pub cmd: *mut mlx5_cmd,
    pub work: work_struct,
    pub lay: *mut mlx5_cmd_layout,
    pub ret: c_int,
    pub page_queue: c_int,
    pub status: u8,
    pub token: u8,
    pub ts1: u64,
    pub ts2: u64,
    pub op: u16,
    pub polling: bool,
// Track the max comp handlers
    pub refcnt: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_port_state {
    MLX5_AAA_111
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_hca_vport_context {
    pub field_select: u32,
    pub sm_virt_aware: bool,
    pub has_smi: bool,
    pub has_raw: bool,
    pub policy: port_state_policy,
    pub phys_state: phy_port_state,
    pub vport_state: ib_port_state,
    pub port_physical_state: u8,
    pub sys_image_guid: u64,
    pub port_guid: u64,
    pub node_guid: u64,
    pub cap_mask1: u32,
    pub cap_mask1_perm: u32,
    pub cap_mask2: u16,
    pub cap_mask2_perm: u16,
    pub lid: u16,
    pub /: *mut *mut u8 init_type_reply; / bitmask: see ib spec 14.2.5.6 InitTypeReply,
    pub lmc: u8,
    pub subnet_timeout: u8,
    pub sm_lid: u16,
    pub sm_sl: u8,
    pub qkey_violation_counter: u16,
    pub pkey_violation_counter: u16,
    pub grh_required: bool,
    pub num_plane: u8,
}

extern "C" {
    pub fn min_t(_arg: u32, fbc->strides_offset: last_frag_stride_idx -, _arg: fbc->sz_m1) -> return;
}
extern "C" {
    pub fn mlx5_cmd_use_events(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_cmd_use_polling(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_cmd_allowed_opcode(dev: *mut mlx5_core_dev, opcode: u16);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_async_ctx {
    pub dev: *mut mlx5_core_dev,
    pub num_inflight: core::sync::atomic::AtomicI32,
    pub inflight_done: completion,
}

extern "C" {
    pub fn void(status: *mut *mut mlx5_async_cbk_t)(int, context: *mut mlx5_async_work) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_async_work {
    pub ctx: *mut mlx5_async_ctx,
    pub user_callback: mlx5_async_cbk_t,
    pub /: *mut *mut u16 opcode; / cmd opcode,
    pub /: *mut *mut u16 op_mod; / cmd op_mod,
    pub throttle_locked:1: u8,
    pub unpriv_locked:1: u8,
    pub /: *mut *mut *mut void out; / pointer to the cmd output buffer,
}

extern "C" {
    pub fn mlx5_cmd_cleanup_async_ctx(ctx: *mut mlx5_async_ctx);
}
extern "C" {
    pub fn mlx5_cmd_out_err(dev: *mut mlx5_core_dev, opcode: u16, op_mod: u16, out: *mut c_void);
}
extern "C" {
    pub fn mlx5_cmd_do(dev: *mut mlx5_core_dev, in: *mut c_void, in_size: c_int, out: *mut c_void, out_size: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_check(dev: *mut mlx5_core_dev, err: c_int, in: *mut c_void, out: *mut c_void) -> c_int;
}

extern "C" {
    pub fn mlx5_cmd_is_down(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_cmd_add_privileged_uid(dev: *mut mlx5_core_dev, uid: u16) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_remove_privileged_uid(dev: *mut mlx5_core_dev, uid: u16);
}
extern "C" {
    pub fn mlx5_core_uplink_netdev_set(mdev: *mut mlx5_core_dev, netdev: *mut net_device);
}
extern "C" {
    pub fn mlx5_core_uplink_netdev_event_replay(mdev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_core_mp_event_replay(dev: *mut mlx5_core_dev, event: u32, data: *mut c_void);
}
extern "C" {
    pub fn mlx5_health_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_health_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_start_health_poll(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_stop_health_poll(dev: *mut mlx5_core_dev, disable_health: bool);
}
extern "C" {
    pub fn mlx5_start_health_fw_log_up(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_drain_health_wq(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_trigger_health_work(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_frag_buf_free(dev: *mut mlx5_core_dev, buf: *mut mlx5_frag_buf);
}
extern "C" {
    pub fn mlx5_core_destroy_mkey(dev: *mut mlx5_core_dev, mkey: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_core_alloc_pd(dev: *mut mlx5_core_dev, pdn: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_core_dealloc_pd(dev: *mut mlx5_core_dev, pdn: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_pagealloc_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_pagealloc_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_pagealloc_start(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_pagealloc_stop(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_pages_debugfs_init(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_pages_debugfs_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_pages_by_func_type_debugfs_init(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_pages_by_func_type_debugfs_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_satisfy_startup_pages(dev: *mut mlx5_core_dev, boot: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_reclaim_startup_pages(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_register_debugfs();
}
extern "C" {
    pub fn mlx5_unregister_debugfs();
}
extern "C" {
    pub fn mlx5_fill_page_frag_array_perm(buf: *mut mlx5_frag_buf, pas: *mut __be64, perm: u8);
}
extern "C" {
    pub fn mlx5_fill_page_frag_array(frag_buf: *mut mlx5_frag_buf, pas: *mut __be64);
}
extern "C" {
    pub fn mlx5_comp_eqn_get(dev: *mut mlx5_core_dev, vecidx: u16, eqn: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_core_attach_mcg(dev: *mut mlx5_core_dev, mgid: *mut ib_gid, qpn: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_core_detach_mcg(dev: *mut mlx5_core_dev, mgid: *mut ib_gid, qpn: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_qp_debugfs_init(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_qp_debugfs_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_db_alloc_node(_arg: dev, _arg: db, _arg: dev->priv.numa_node) -> return;
}
extern "C" {
    pub fn mlx5_db_free(dev: *mut mlx5_core_dev, db: *mut mlx5_db);
}
extern "C" {
    pub fn mlx5_cmdif_debugfs_init(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_cmdif_debugfs_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_core_destroy_psv(dev: *mut mlx5_core_dev, psv_num: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_core_get_terminate_scatter_list_mkey(dev: *mut mlx5_core_dev) -> __be32;
}
extern "C" {
    pub fn mlx5_core_put_rsc(common: *mut mlx5_core_rsc_common);
}
extern "C" {
    pub fn mlx5_init_rl_table(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_cleanup_rl_table(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_rl_remove_rate(dev: *mut mlx5_core_dev, rl: *mut mlx5_rate_limit);
}
extern "C" {
    pub fn mlx5_rl_is_in_range(dev: *mut mlx5_core_dev, rate: u32) -> bool;
}
extern "C" {
    pub fn mlx5_rl_remove_rate_raw(dev: *mut mlx5_core_dev, index: u16);
}
extern "C" {
    pub fn mlx5_free_bfreg(mdev: *mut mlx5_core_dev, bfreg: *mut mlx5_sq_bfreg);
}
extern "C" {
    pub fn mlx5_comp_vectors_max(dev: *mut mlx5_core_dev) -> c_uint;
}
extern "C" {
    pub fn mlx5_comp_vector_get_cpu(dev: *mut mlx5_core_dev, vector: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_core_reserved_gids_count(dev: *mut mlx5_core_dev) -> c_uint;
}
// Async-atomic event notifier used by mlx5 core to forward FW
// evetns received from event queue to mlx5 consumers.
// Optimise event queue dipatching.
//
extern "C" {
    pub fn mlx5_notifier_register(dev: *mut mlx5_core_dev, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn mlx5_notifier_unregister(dev: *mut mlx5_core_dev, nb: *mut notifier_block) -> c_int;
}
// Async-atomic event notifier used for forwarding
// evetns from the event queue into the to mlx5 events dispatcher,
// eswitch, clock and others.
//
extern "C" {
    pub fn mlx5_eq_notifier_register(dev: *mut mlx5_core_dev, nb: *mut mlx5_nb) -> c_int;
}
extern "C" {
    pub fn mlx5_eq_notifier_unregister(dev: *mut mlx5_core_dev, nb: *mut mlx5_nb) -> c_int;
}
// Blocking event notifier used to forward SW events, used for slow path
extern "C" {
    pub fn mlx5_blocking_notifier_register(dev: *mut mlx5_core_dev, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn mlx5_blocking_notifier_unregister(dev: *mut mlx5_core_dev, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn mlx5_core_query_vendor_id(mdev: *mut mlx5_core_dev, vendor_id: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_create_vport_lag(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_destroy_vport_lag(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_lag_is_roce(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_lag_is_sriov(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_lag_is_active(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_lag_query_bond_speed(dev: *mut mlx5_core_dev, speed: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_lag_mode_is_hash(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_lag_is_master(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_lag_is_shared_fdb(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_lag_is_mpesw(dev: *mut mlx5_core_dev) -> bool;
}

extern "C" {
    pub fn mlx5_lag_get_num_ports(dev: *mut mlx5_core_dev) -> u8;
}
extern "C" {
    pub fn mlx5_put_uars_page(mdev: *mut mlx5_core_dev, up: *mut mlx5_uars_page);
}

extern "C" {
    pub fn mlx5_st_dealloc_index(dev: *mut mlx5_core_dev, st_index: u16) -> c_int;
}

extern "C" {
    pub fn mlx5_vf_put_core_dev(mdev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_core_is_pf(MLX5_CAP_ESW(dev: dev) &&, _arg: ecpf_vport_exists) -> return;
}
// LACP owner conditions:
// 1) Function is physical.
// 2) LAG is supported by FW.
// 3) LAG is managed by driver (currently the only option).
//
extern "C" {
    pub fn MLX5_CAP_GEN(_arg: dev, _arg: native_port_num) -> return;
}
extern "C" {
    pub fn PCI_FUNC(_arg: dev->pdev->devfn) -> return;
}
extern "C" {
    pub fn mlx5_is_roce_on(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn MLX5_CAP_GEN(_arg: dev, _arg: roce) -> return;
}
// If RoCE cap is read-only in FW, get RoCE state from devlink
// in order to support RoCE enable/disable feature
//
extern "C" {
    pub fn mlx5_is_roce_on(_arg: dev) -> return;
}

extern "C" {
    pub fn mlx5_wc_support_get(mdev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn devlink_net(_arg: priv_to_devlink(dev)) -> return;
}
pub const MLX5_SW_IMAGE_GUID_MAX_BYTES: c_int = 9;
