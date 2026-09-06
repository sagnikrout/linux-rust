//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx4/device.h
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
// Copyright (c) 2006, 2007 Cisco Systems, Inc.  All rights reserved.
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

pub const DEFAULT_UAR_PAGE_SHIFT: c_int = 12;
pub const MAX_MSIX: c_int = 128;
pub const MIN_MSIX_P_PORT: c_int = 5;

// work around: can't set values
// greater then this value when
// using 100 Mbps units.
//

pub const MLX4_RATELIMIT_DEFAULT: c_uint = 0x00ff;
pub const MLX4_ROCE_MAX_GIDS: c_int = 128;
pub const MLX4_ROCE_PF_GIDS: c_int = 16;
// base qkey for use in sriov tunnel-qp/proxy-qp communication.
// These qkeys must not be allowed for general use. This is a 64k range,
// and to test for violation, we use the mask (protect against future chg).
//

// Driver supports 3 different device methods to manage traffic steering:
// -device managed - High level API for ib and eth flow steering. FW is
// managing flow steering tables.
// - B0 steering mode - Common low level API for ib and (if supported) eth.
// - A0 steering mode - Limited low level API for eth. In case of IB,
// B0 mode is in use.
//
// bit enums for an 8-bit flags field indicating special use
// QPs which require special handling in qp_reserve_range.
// Currently, this only includes QPs used by the ETH interface,
// where we expect to use blueflame.  These QPs must not have
// bits 6 and 7 set in their qp number.
//
// This enum may use only bits 0..7.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_event {
    MLX4_EVENT_TYPE_COMP		   = 0x00,
    MLX4_EVENT_TYPE_PATH_MIG	   = 0x01,
    MLX4_EVENT_TYPE_COMM_EST	   = 0x02,
    MLX4_EVENT_TYPE_SQ_DRAINED	   = 0x03,
    MLX4_EVENT_TYPE_SRQ_QP_LAST_WQE	   = 0x13,
    MLX4_EVENT_TYPE_SRQ_LIMIT	   = 0x14,
    MLX4_EVENT_TYPE_CQ_ERROR	   = 0x04,
    MLX4_EVENT_TYPE_WQ_CATAS_ERROR	   = 0x05,
    MLX4_EVENT_TYPE_EEC_CATAS_ERROR	   = 0x06,
    MLX4_EVENT_TYPE_PATH_MIG_FAILED	   = 0x07,
    MLX4_EVENT_TYPE_WQ_INVAL_REQ_ERROR = 0x10,
    MLX4_EVENT_TYPE_WQ_ACCESS_ERROR	   = 0x11,
    MLX4_EVENT_TYPE_SRQ_CATAS_ERROR	   = 0x12,
    MLX4_EVENT_TYPE_LOCAL_CATAS_ERROR  = 0x08,
    MLX4_EVENT_TYPE_PORT_CHANGE	   = 0x09,
    MLX4_EVENT_TYPE_EQ_OVERFLOW	   = 0x0f,
    MLX4_EVENT_TYPE_ECC_DETECT	   = 0x0e,
    MLX4_EVENT_TYPE_CMD		   = 0x0a,
    MLX4_EVENT_TYPE_VEP_UPDATE	   = 0x19,
    MLX4_EVENT_TYPE_COMM_CHANNEL	   = 0x18,
    MLX4_EVENT_TYPE_OP_REQUIRED	   = 0x1a,
    MLX4_EVENT_TYPE_FATAL_WARNING	   = 0x1b,
    MLX4_EVENT_TYPE_FLR_EVENT	   = 0x1c,
    MLX4_EVENT_TYPE_PORT_MNG_CHG_EVENT = 0x1d,
    MLX4_EVENT_TYPE_RECOVERABLE_ERROR_EVENT  = 0x3e,
    MLX4_EVENT_TYPE_NONE		   = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slave_port_state {
    SLAVE_PORT_DOWN = 0,
    SLAVE_PENDING_UP,
    SLAVE_PORT_UP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slave_port_gen_event {
    SLAVE_PORT_GEN_EVENT_DOWN = 0,
    SLAVE_PORT_GEN_EVENT_UP,
    SLAVE_PORT_GEN_EVENT_NONE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slave_port_state_event {
    MLX4_PORT_STATE_DEV_EVENT_PORT_DOWN,
    MLX4_PORT_STATE_DEV_EVENT_PORT_UP,
    MLX4_PORT_STATE_IB_PORT_STATE_EVENT_GID_VALID,
    MLX4_PORT_STATE_IB_EVENT_GID_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_protocol {
    MLX4_PROT_IB_IPV6 = 0,
    MLX4_PROT_ETH,
    MLX4_PROT_IB_IPV4,
    MLX4_PROT_FCOE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_qp_region {
    MLX4_QP_REGION_FW = 0,
    MLX4_QP_REGION_RSS_RAW_ETH,
    MLX4_QP_REGION_BOTTOM = MLX4_QP_REGION_RSS_RAW_ETH,
    MLX4_QP_REGION_ETH_ADDR,
    MLX4_QP_REGION_FC_ADDR,
    MLX4_QP_REGION_FC_EXCH,
    MLX4_NUM_QP_REGION
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_port_type {
    MLX4_PORT_TYPE_NONE	= 0,
    MLX4_PORT_TYPE_IB	= 1,
    MLX4_PORT_TYPE_ETH	= 2,
    MLX4_PORT_TYPE_AUTO	= 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_special_vlan_idx {
    MLX4_NO_VLAN_IDX        = 0,
    MLX4_VLAN_MISS_IDX,
    MLX4_VLAN_REGULAR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_steer_type {
    MLX4_MC_STEER = 0,
    MLX4_UC_STEER,
    MLX4_NUM_STEERS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_resource_usage {
    MLX4_RES_USAGE_NONE,
    MLX4_RES_USAGE_DRIVER,
    MLX4_RES_USAGE_USER_VERBS,
}

//
// Max wqe size for rdma read is 512 bytes, so this
// limits our max_sge_rd as the wqe needs to fit:
// - ctrl segment (16 bytes)
// - rdma segment (16 bytes)
// - scatter elements (16 bytes each)
//
// Port mgmt change event handling
#[repr(C)]
#[derive(Copy, Clone)]
pub union sl2vl_tbl_to_u64 {
    pub sl8: [u8; 8],
    pub sl64: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_module_id {
    MLX4_MODULE_ID_SFP              = 0x3,
    MLX4_MODULE_ID_QSFP             = 0xC,
    MLX4_MODULE_ID_QSFP_PLUS        = 0xD,
    MLX4_MODULE_ID_QSFP28           = 0x11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_rate_limit_caps {
    pub /: *mut *mut u16 num_rates; / Number of different rates,
    pub min_unit: u8,
    pub min_val: u16,
    pub max_unit: u8,
    pub max_val: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_phys_caps {
    pub 1]: u32 gid_phys_table_len[MLX4_MAX_PORTS +,
    pub 1]: u32 pkey_phys_table_len[MLX4_MAX_PORTS +,
    pub num_phys_eqs: u32,
    pub base_sqpn: u32,
    pub base_proxy_sqpn: u32,
    pub base_tunnel_sqpn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_spec_qps {
    pub qp0_qkey: u32,
    pub qp0_proxy: u32,
    pub qp0_tunnel: u32,
    pub qp1_proxy: u32,
    pub qp1_tunnel: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_caps {
    pub fw_ver: u64,
    pub function: u32,
    pub num_ports: c_int,
    pub 1]: int vl_cap[MLX4_MAX_PORTS +,
    pub 1]: int ib_mtu_cap[MLX4_MAX_PORTS +,
    pub 1]: __be32 ib_port_def_cap[MLX4_MAX_PORTS +,
    pub 1]: u64 def_mac[MLX4_MAX_PORTS +,
    pub 1]: int eth_mtu_cap[MLX4_MAX_PORTS +,
    pub 1]: int gid_table_len[MLX4_MAX_PORTS +,
    pub 1]: int pkey_table_len[MLX4_MAX_PORTS +,
    pub 1]: int trans_type[MLX4_MAX_PORTS +,
    pub 1]: int vendor_oui[MLX4_MAX_PORTS +,
    pub 1]: int wavelength[MLX4_MAX_PORTS +,
    pub 1]: u64 trans_code[MLX4_MAX_PORTS +,
    pub local_ca_ack_delay: c_int,
    pub num_uars: c_int,
    pub uar_page_size: u32,
    pub bf_reg_size: c_int,
    pub bf_regs_per_page: c_int,
    pub max_sq_sg: c_int,
    pub max_rq_sg: c_int,
    pub num_qps: c_int,
    pub max_wqes: c_int,
    pub max_sq_desc_sz: c_int,
    pub max_rq_desc_sz: c_int,
    pub max_qp_init_rdma: c_int,
    pub max_qp_dest_rdma: c_int,
    pub max_tc_eth: c_int,
    pub spec_qps: *mut mlx4_spec_qps,
    pub num_srqs: c_int,
    pub max_srq_wqes: c_int,
    pub max_srq_sge: c_int,
    pub reserved_srqs: c_int,
    pub num_cqs: c_int,
    pub max_cqes: c_int,
    pub reserved_cqs: c_int,
    pub num_sys_eqs: c_int,
    pub num_eqs: c_int,
    pub reserved_eqs: c_int,
    pub num_comp_vectors: c_int,
    pub num_mpts: c_int,
    pub num_mtts: c_int,
    pub fmr_reserved_mtts: c_int,
    pub reserved_mtts: c_int,
    pub reserved_mrws: c_int,
    pub reserved_uars: c_int,
    pub num_mgms: c_int,
    pub num_amgms: c_int,
    pub reserved_mcgs: c_int,
    pub num_qp_per_mgm: c_int,
    pub steering_mode: c_int,
    pub dmfs_high_steer_mode: c_int,
    pub fs_log_max_ucast_qp_range_size: c_int,
    pub num_pds: c_int,
    pub reserved_pds: c_int,
    pub max_xrcds: c_int,
    pub reserved_xrcds: c_int,
    pub mtt_entry_sz: c_int,
    pub max_msg_sz: u32,
    pub page_size_cap: u32,
    pub flags: u64,
    pub flags2: u64,
    pub bmme_flags: u32,
    pub reserved_lkey: u32,
    pub stat_rate_support: u16,
    pub 1]: u8 port_width_cap[MLX4_MAX_PORTS +,
    pub max_gso_sz: c_int,
    pub max_rss_tbl_sz: c_int,
    pub reserved_qps_cnt: [c_int; MLX4_NUM_QP_REGION],
    pub reserved_qps: c_int,
    pub reserved_qps_base: [c_int; MLX4_NUM_QP_REGION],
    pub log_num_macs: c_int,
    pub log_num_vlans: c_int,
    pub 1]: mlx4_port_type port_type[MLX4_MAX_PORTS +,
    pub 1]: u8 supported_type[MLX4_MAX_PORTS +,
    pub 1]: u8 suggested_type[MLX4_MAX_PORTS +,
    pub 1]: u8 default_sense[MLX4_MAX_PORTS +,
    pub 1]: u32 port_mask[MLX4_MAX_PORTS +,
    pub 1]: mlx4_port_type possible_type[MLX4_MAX_PORTS +,
    pub max_counters: u32,
    pub 1]: u8 port_ib_mtu[MLX4_MAX_PORTS +,
    pub sqp_demux: u16,
    pub eqe_size: u32,
    pub cqe_size: u32,
    pub eqe_factor: u8,
    pub /: *mut *mut u32 userspace_caps; / userspace must be aware of these,
    pub /: *mut *mut u32 function_caps; / VFs must be aware of these,
    pub hca_core_clock: u16,
    pub 1]: u64 phys_port_id[MLX4_MAX_PORTS +,
    pub tunnel_offload_mode: c_int,
    pub 1]: u8 rx_checksum_flags_port[MLX4_MAX_PORTS +,
    pub 1]: u8 phv_bit[MLX4_MAX_PORTS +,
    pub alloc_res_qp_mask: u8,
    pub dmfs_high_rate_qpn_base: u32,
    pub dmfs_high_rate_qpn_range: u32,
    pub vf_caps: u32,
    pub 1]: bool wol_port[MLX4_MAX_PORTS +,
    pub rl_caps: mlx4_rate_limit_caps,
    pub health_buffer_addrs: u32,
    pub map_clock_to_user: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_buf_list {
    pub buf: *mut c_void,
    pub map: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_buf {
    pub direct: mlx4_buf_list,
    pub page_list: *mut mlx4_buf_list,
    pub nbufs: c_int,
    pub npages: c_int,
    pub page_shift: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_mtt {
    pub offset: u32,
    pub order: c_int,
    pub page_shift: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_db_pgdir {
    pub list: list_head,
    pub MLX4_DB_PER_PAGE): DECLARE_BITMAP(order0,,
    pub 2): DECLARE_BITMAP(order1, MLX4_DB_PER_PAGE /,
    pub bits: [*mut c_ulong; 2],
    pub db_page: *mut __be32,
    pub db_dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_db {
    pub db: *mut __be32,
    pub pgdir: *mut mlx4_db_pgdir,
    pub user_page: *mut mlx4_ib_user_db_page,
    pub u: },
    pub dma: dma_addr_t,
    pub index: c_int,
    pub order: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_hwq_resources {
    pub db: mlx4_db,
    pub mtt: mlx4_mtt,
    pub buf: mlx4_buf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_mr {
    pub mtt: mlx4_mtt,
    pub iova: u64,
    pub size: u64,
    pub key: u32,
    pub pd: u32,
    pub access: u32,
    pub enabled: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_mw_type {
    MLX4_MW_TYPE_1 = 1,
    MLX4_MW_TYPE_2 = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_mw {
    pub key: u32,
    pub pd: u32,
    pub type: mlx4_mw_type,
    pub enabled: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_uar {
    pub pfn: c_ulong,
    pub index: c_int,
    pub bf_list: list_head,
    pub free_bf_bmap: unsigned,
    pub map: *mut void __iomem,
    pub bf_map: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_bf {
    pub offset: c_uint,
    pub buf_size: c_int,
    pub uar: *mut mlx4_uar,
    pub reg: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_cq {
    pub ): *mut *mut void (comp) (struct mlx4_cq,
    pub mlx4_event): *mut *mut *mut void (event) (struct mlx4_cq , enum,
    pub uar: *mut mlx4_uar,
    pub cons_index: u32,
    pub irq: u16,
    pub set_ci_db: *mut __be32,
    pub arm_db: *mut __be32,
    pub arm_sn: c_int,
    pub cqn: c_int,
    pub vector: unsigned,
    pub refcount: refcount_t,
    pub free: completion,
    pub list: list_head,
    pub ): *mut *mut void (comp)(struct mlx4_cq,
    pub priv: *mut c_void,
    pub tasklet_ctx: },
    pub reset_notify_added: c_int,
    pub reset_notify: list_head,
    pub usage: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_qp {
    pub mlx4_event): *mut *mut *mut void (event) (struct mlx4_qp , enum,
    pub qpn: c_int,
    pub refcount: refcount_t,
    pub free: completion,
    pub usage: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_srq {
    pub mlx4_event): *mut *mut *mut void (event) (struct mlx4_srq , enum,
    pub srqn: c_int,
    pub max: c_int,
    pub max_gs: c_int,
    pub wqe_shift: c_int,
    pub refcount: refcount_t,
    pub free: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_av {
    pub port_pd: __be32,
    pub reserved1: u8,
    pub g_slid: u8,
    pub dlid: __be16,
    pub reserved2: u8,
    pub gid_index: u8,
    pub stat_rate: u8,
    pub hop_limit: u8,
    pub sl_tclass_flowlabel: __be32,
    pub dgid: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_eth_av {
    pub port_pd: __be32,
    pub reserved1: u8,
    pub smac_idx: u8,
    pub reserved2: u16,
    pub reserved3: u8,
    pub gid_index: u8,
    pub stat_rate: u8,
    pub hop_limit: u8,
    pub sl_tclass_flowlabel: __be32,
    pub dgid: [u8; 16],
    pub s_mac: [u8; 6],
    pub reserved4: [u8; 2],
    pub vlan: __be16,
    pub mac: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx4_ext_av {
    pub ib: mlx4_av,
    pub eth: mlx4_eth_av,
}

// Counters should be saturate once they reach their maximum value

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_counter {
    pub reserved1: [u8; 3],
    pub counter_mode: u8,
    pub num_ifc: __be32,
    pub reserved2: [u32; 2],
    pub rx_frames: __be64,
    pub rx_bytes: __be64,
    pub tx_frames: __be64,
    pub tx_bytes: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_quotas {
    pub qp: c_int,
    pub cq: c_int,
    pub srq: c_int,
    pub mpt: c_int,
    pub mtt: c_int,
    pub counter: c_int,
    pub xrcd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_vf_dev {
    pub min_port: u8,
    pub n_ports: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_fw_crdump {
    pub snapshot_enable: bool,
    pub region_crspace: *mut devlink_region,
    pub region_fw_health: *mut devlink_region,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_pci_status {
    MLX4_PCI_STATUS_DISABLED,
    MLX4_PCI_STATUS_ENABLED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_dev_persistent {
    pub pdev: *mut pci_dev,
    pub dev: *mut mlx4_dev,
    pub 1]: int nvfs[MLX4_MAX_PORTS +,
    pub num_vfs: c_int,
    pub 1]: mlx4_port_type curr_port_type[MLX4_MAX_PORTS +,
    pub 1]: mlx4_port_type curr_port_poss_type[MLX4_MAX_PORTS +,
    pub catas_work: work_struct,
    pub catas_wq: *mut workqueue_struct,
    pub /: *mut *mut mutex device_state_mutex; / protect HW state,
    pub state: u8,
    pub /: *mut *mut mutex interface_state_mutex; / protect SW state,
    pub interface_state: u8,
    pub /: *mut *mut mutex pci_status_mutex; / sync pci state,
    pub pci_status: mlx4_pci_status,
    pub crdump: mlx4_fw_crdump,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_dev {
    pub persist: *mut mlx4_dev_persistent,
    pub flags: c_ulong,
    pub num_slaves: c_ulong,
    pub caps: mlx4_caps,
    pub phys_caps: mlx4_phys_caps,
    pub quotas: mlx4_quotas,
    pub qp_table_tree: radix_tree_root,
    pub rev_id: u8,
    pub port_random_macs: u8,
    pub board_id: [c_char; MLX4_BOARD_ID_LEN],
    pub numa_node: c_int,
    pub oper_log_mgm_entry_size: c_int,
    pub 1]: u64 regid_promisc_array[MLX4_MAX_PORTS +,
    pub 1]: u64 regid_allmulti_array[MLX4_MAX_PORTS +,
    pub dev_vfs: *mut mlx4_vf_dev,
    pub uar_page_shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_adev {
    pub adev: auxiliary_device,
    pub mdev: *mut mlx4_dev,
    pub idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_clock_params {
    pub offset: u64,
    pub bar: u8,
    pub size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_eqe {
    pub reserved1: u8,
    pub type: u8,
    pub reserved2: u8,
    pub subtype: u8,
    pub raw: [u32; 6],
    pub cqn: __be32,
    pub comp: } __packed,
    pub reserved1: u16,
    pub token: __be16,
    pub reserved2: u32,
    pub reserved3: [u8; 3],
    pub status: u8,
    pub out_param: __be64,
    pub cmd: } __packed,
    pub qpn: __be32,
    pub qp: } __packed,
    pub srqn: __be32,
    pub srq: } __packed,
    pub cqn: __be32,
    pub reserved1: u32,
    pub reserved2: [u8; 3],
    pub syndrome: u8,
    pub cq_err: } __packed,
    pub reserved1: [u32; 2],
    pub port: __be32,
    pub port_change: } __packed,
pub const COMM_CHANNEL_BIT_ARRAY_SIZE: c_int = 4;
    pub reserved: u32,
    pub bit_vec: [u32; COMM_CHANNEL_BIT_ARRAY_SIZE],
    pub comm_channel_arm: } __packed,
    pub port: u8,
    pub reserved: [u8; 3],
    pub mac: __be64,
    pub mac_update: } __packed,
    pub slave_id: __be32,
    pub flr_event: } __packed,
    pub current_temperature: __be16,
    pub warning_threshold: __be16,
    pub warming: } __packed,
    pub reserved: [u8; 3],
    pub port: u8,
    pub mstr_sm_lid: __be16,
    pub port_lid: __be16,
    pub changed_attr: __be32,
    pub reserved: [u8; 3],
    pub mstr_sm_sl: u8,
    pub gid_prefix: __be64,
    pub port_info: } __packed,
    pub block_ptr: __be32,
    pub tbl_entries_mask: __be32,
    pub tbl_change_info: } __packed,
    pub sl2vl_table: [u8; 8],
    pub sl2vl_tbl_change_info: } __packed,
    pub params: },
    pub port_mgmt_change: } __packed,
    pub reserved: [u8; 3],
    pub port: u8,
    pub reserved1: [u32; 5],
    pub bad_cable: } __packed,
    pub event: },
    pub slave_id: u8,
    pub reserved3: [u8; 2],
    pub owner: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_init_port_param {
    pub set_guid0: c_int,
    pub set_node_guid: c_int,
    pub set_si_guid: c_int,
    pub mtu: u16,
    pub port_width_cap: c_int,
    pub vl_cap: u16,
    pub max_gid: u16,
    pub max_pkey: u16,
    pub guid0: u64,
    pub node_guid: u64,
    pub si_guid: u64,
}

pub const MAD_IFC_DATA_SZ: c_int = 192;
// MAD IFC Mailbox
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_mad_ifc {
    pub base_version: u8,
    pub mgmt_class: u8,
    pub class_version: u8,
    pub method: u8,
    pub status: __be16,
    pub class_specific: __be16,
    pub tid: __be64,
    pub attr_id: __be16,
    pub resv: __be16,
    pub attr_mod: __be32,
    pub mkey: __be64,
    pub dr_slid: __be16,
    pub dr_dlid: __be16,
    pub reserved: [u8; 28],
    pub data: [u8; MAD_IFC_DATA_SZ],
    pub __packed: },

    pub \: for ((port) = 1; (port) <= (dev)->caps.num_ports; (port)++),

    pub \: for ((port) = 1; (port) <= (dev)->caps.num_ports; (port)++),
pub const MLX4_INVALID_SLAVE_ID: c_uint = 0xFF;

    pub work): *mut void handle_port_mgmt_change_event(struct work_struct,
    pub dev->caps.function: return,
    pub MLX4_FLAG_MASTER: return dev->flags &,
    pub !!mlx4_is_master(dev): *mut *mut *mut 16  MLX4_MFUNC_MAX,
    pub dev->caps.reserved_qps_cnt[MLX4_QP_REGION_FW]): (qpn <,
    pub 8: *mut *mut int guest_proxy_base = dev->phys_caps.base_proxy_sqpn + slave,
    pub 1: return,
    pub 0: return,
    pub MLX4_FLAG_MASTER): return dev->flags & (MLX4_FLAG_SLAVE |,
    pub MLX4_FLAG_SLAVE: return dev->flags &,
    pub 1: return dev->caps.port_type[port] == MLX4_PORT_TYPE_IB ? 0 :,
    pub buf): *mut mlx4_buf,
    pub buf): *mut *mut void mlx4_buf_free(struct mlx4_dev dev, int size, struct mlx4_buf,
    pub offset: return buf->direct.buf +,
    pub 1)): (offset & (PAGE_SIZE -,
    pub MLX4_FLAG_BONDED): return !!(dev->flags &,
    pub mlx4_is_mfunc(dev)): return (mlx4_is_bonded(dev) &&,
    pub v2p_p2): u8,
    pub pdn): *mut *mut int mlx4_pd_alloc(struct mlx4_dev dev, u32,
    pub pdn): *mut *mut void mlx4_pd_free(struct mlx4_dev dev, u32,
    pub xrcdn): *mut *mut int mlx4_xrcd_alloc(struct mlx4_dev dev, u32,
    pub xrcdn): *mut *mut void mlx4_xrcd_free(struct mlx4_dev dev, u32,
    pub uar): *mut *mut int mlx4_uar_alloc(struct mlx4_dev dev, struct mlx4_uar,
    pub uar): *mut *mut void mlx4_uar_free(struct mlx4_dev dev, struct mlx4_uar,
    pub node): *mut *mut *mut int mlx4_bf_alloc(struct mlx4_dev dev, struct mlx4_bf bf, int,
    pub bf): *mut *mut void mlx4_bf_free(struct mlx4_dev dev, struct mlx4_bf,
    pub mtt): *mut mlx4_mtt,
    pub mtt): *mut *mut void mlx4_mtt_cleanup(struct mlx4_dev dev, struct mlx4_mtt,
    pub mtt): *mut *mut u64 mlx4_mtt_addr(struct mlx4_dev dev, struct mlx4_mtt,
    pub mr): *mut int npages, int page_shift, struct mlx4_mr,
    pub mr): *mut *mut int mlx4_mr_free(struct mlx4_dev dev, struct mlx4_mr,
    pub mr): *mut *mut int mlx4_mr_enable(struct mlx4_dev dev, struct mlx4_mr,
    pub mw): *mut mlx4_mw,
    pub mw): *mut *mut void mlx4_mw_free(struct mlx4_dev dev, struct mlx4_mw,
    pub mw): *mut *mut int mlx4_mw_enable(struct mlx4_dev dev, struct mlx4_mw,
    pub page_list): *mut int start_index, int npages, u64,
    pub buf): *mut mlx4_buf,
    pub order): *mut *mut *mut int mlx4_db_alloc(struct mlx4_dev dev, struct mlx4_db db, unsigned int,
    pub db): *mut *mut void mlx4_db_free(struct mlx4_dev dev, struct mlx4_db,
    pub size): c_int,
    pub size): c_int,
    pub user_cq): *mut *mut void buf_addr, bool,
    pub cq): *mut *mut void mlx4_cq_free(struct mlx4_dev dev, struct mlx4_cq,
    pub usage): *mut *mut int base, u8 flags, u8,
    pub cnt): *mut *mut void mlx4_qp_release_range(struct mlx4_dev dev, int base_qpn, int,
    pub qp): *mut *mut int mlx4_qp_alloc(struct mlx4_dev dev, int qpn, struct mlx4_qp,
    pub qp): *mut *mut void mlx4_qp_free(struct mlx4_dev dev, struct mlx4_qp,
    pub srq): *mut *mut mlx4_mtt mtt, u64 db_rec, mlx4_srq,
    pub srq): *mut *mut void mlx4_srq_free(struct mlx4_dev dev, struct mlx4_srq,
    pub limit_watermark): *mut *mut *mut int mlx4_srq_arm(struct mlx4_dev dev, struct mlx4_srq srq, int,
    pub limit_watermark): *mut *mut *mut int mlx4_srq_query(struct mlx4_dev dev, struct mlx4_srq srq, int,
    pub port): *mut *mut int mlx4_INIT_PORT(struct mlx4_dev dev, int,
    pub port): *mut *mut int mlx4_CLOSE_PORT(struct mlx4_dev dev, int,
    pub prot): int block_mcast_loopback, enum mlx4_protocol,
    pub prot): mlx4_protocol,
    pub reg_id): *mut mlx4_protocol protocol, u64,
    pub reg_id): mlx4_protocol protocol, u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_net_trans_rule_id {
    MLX4_NET_TRANS_RULE_ID_ETH = 0,
    MLX4_NET_TRANS_RULE_ID_IB,
    MLX4_NET_TRANS_RULE_ID_IPV6,
    MLX4_NET_TRANS_RULE_ID_IPV4,
    MLX4_NET_TRANS_RULE_ID_TCP,
    MLX4_NET_TRANS_RULE_ID_UDP,
    MLX4_NET_TRANS_RULE_ID_VXLAN,
    MLX4_NET_TRANS_RULE_NUM, /* should be last */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_net_trans_promisc_mode {
    MLX4_FS_REGULAR = 1,
    MLX4_FS_ALL_DEFAULT,
    MLX4_FS_MC_DEFAULT,
    MLX4_FS_MIRROR_RX_PORT,
    MLX4_FS_MIRROR_SX_PORT,
    MLX4_FS_UC_SNIFFER,
    MLX4_FS_MC_SNIFFER,
    MLX4_FS_MODE_NUM, /* should be last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_spec_eth {
    pub dst_mac: [u8; ETH_ALEN],
    pub dst_mac_msk: [u8; ETH_ALEN],
    pub src_mac: [u8; ETH_ALEN],
    pub src_mac_msk: [u8; ETH_ALEN],
    pub ether_type_enable: u8,
    pub ether_type: __be16,
    pub vlan_id_msk: __be16,
    pub vlan_id: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_spec_tcp_udp {
    pub dst_port: __be16,
    pub dst_port_msk: __be16,
    pub src_port: __be16,
    pub src_port_msk: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_spec_ipv4 {
    pub dst_ip: __be32,
    pub dst_ip_msk: __be32,
    pub src_ip: __be32,
    pub src_ip_msk: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_spec_ib {
    pub l3_qpn: __be32,
    pub qpn_msk: __be32,
    pub dst_gid: [u8; 16],
    pub dst_gid_msk: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_spec_vxlan {
    pub vni: __be32,
    pub vni_mask: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_spec_list {
    pub list: list_head,
    pub id: mlx4_net_trans_rule_id,
    pub eth: mlx4_spec_eth,
    pub ib: mlx4_spec_ib,
    pub ipv4: mlx4_spec_ipv4,
    pub tcp_udp: mlx4_spec_tcp_udp,
    pub vxlan: mlx4_spec_vxlan,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_net_trans_hw_rule_queue {
    MLX4_NET_TRANS_Q_FIFO,
    MLX4_NET_TRANS_Q_LIFO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_net_trans_rule {
    pub list: list_head,
    pub queue_mode: mlx4_net_trans_hw_rule_queue,
    pub exclusive: bool,
    pub allow_loopback: bool,
    pub promisc_mode: mlx4_net_trans_promisc_mode,
    pub port: u8,
    pub priority: u16,
    pub qpn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_net_trans_rule_hw_ctrl {
    pub prio: __be16,
    pub type: u8,
    pub flags: u8,
    pub rsvd1: u8,
    pub funcid: u8,
    pub vep: u8,
    pub port: u8,
    pub qpn: __be32,
    pub rsvd2: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_net_trans_rule_hw_ib {
    pub size: u8,
    pub rsvd1: u8,
    pub id: __be16,
    pub rsvd2: u32,
    pub l3_qpn: __be32,
    pub qpn_mask: __be32,
    pub dst_gid: [u8; 16],
    pub dst_gid_msk: [u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_net_trans_rule_hw_eth {
    pub size: u8,
    pub rsvd: u8,
    pub id: __be16,
    pub rsvd1: [u8; 6],
    pub dst_mac: [u8; 6],
    pub rsvd2: u16,
    pub dst_mac_msk: [u8; 6],
    pub rsvd3: u16,
    pub src_mac: [u8; 6],
    pub rsvd4: u16,
    pub src_mac_msk: [u8; 6],
    pub rsvd5: u8,
    pub ether_type_enable: u8,
    pub ether_type: __be16,
    pub vlan_tag_msk: __be16,
    pub vlan_tag: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_net_trans_rule_hw_tcp_udp {
    pub size: u8,
    pub rsvd: u8,
    pub id: __be16,
    pub rsvd1: [__be16; 3],
    pub dst_port: __be16,
    pub rsvd2: __be16,
    pub dst_port_msk: __be16,
    pub rsvd3: __be16,
    pub src_port: __be16,
    pub rsvd4: __be16,
    pub src_port_msk: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_net_trans_rule_hw_ipv4 {
    pub size: u8,
    pub rsvd: u8,
    pub id: __be16,
    pub rsvd1: __be32,
    pub dst_ip: __be32,
    pub dst_ip_msk: __be32,
    pub src_ip: __be32,
    pub src_ip_msk: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_net_trans_rule_hw_vxlan {
    pub size: u8,
    pub rsvd: u8,
    pub id: __be16,
    pub rsvd1: __be32,
    pub vni: __be32,
    pub vni_mask: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _rule_hw {
    pub size: u8,
    pub rsvd: u8,
    pub id: __be16,
}

extern "C" {
    pub fn mlx4_multicast_promisc_add(dev: *mut mlx4_dev, qpn: u32, port: u8) -> c_int;
}
extern "C" {
    pub fn mlx4_multicast_promisc_remove(dev: *mut mlx4_dev, qpn: u32, port: u8) -> c_int;
}
extern "C" {
    pub fn mlx4_unicast_promisc_add(dev: *mut mlx4_dev, qpn: u32, port: u8) -> c_int;
}
extern "C" {
    pub fn mlx4_unicast_promisc_remove(dev: *mut mlx4_dev, qpn: u32, port: u8) -> c_int;
}
extern "C" {
    pub fn mlx4_SET_MCAST_FLTR(dev: *mut mlx4_dev, port: u8, mac: u64, clear: u64, mode: u8) -> c_int;
}
extern "C" {
    pub fn mlx4_register_mac(dev: *mut mlx4_dev, port: u8, mac: u64) -> c_int;
}
extern "C" {
    pub fn mlx4_unregister_mac(dev: *mut mlx4_dev, port: u8, mac: u64);
}
extern "C" {
    pub fn mlx4_get_base_qpn(dev: *mut mlx4_dev, port: u8) -> c_int;
}
extern "C" {
    pub fn __mlx4_replace_mac(dev: *mut mlx4_dev, port: u8, qpn: c_int, new_mac: u64) -> c_int;
}
extern "C" {
    pub fn mlx4_SET_PORT_user_mac(dev: *mut mlx4_dev, port: u8, user_mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn mlx4_SET_PORT_user_mtu(dev: *mut mlx4_dev, port: u8, user_mtu: u16) -> c_int;
}
extern "C" {
    pub fn mlx4_SET_PORT_BEACON(dev: *mut mlx4_dev, port: u8, time: u16) -> c_int;
}
extern "C" {
    pub fn mlx4_SET_PORT_VXLAN(dev: *mut mlx4_dev, port: u8, steering: u8, enable: c_int) -> c_int;
}
extern "C" {
    pub fn set_phv_bit(dev: *mut mlx4_dev, port: u8, new_val: c_int) -> c_int;
}
extern "C" {
    pub fn get_phv_bit(dev: *mut mlx4_dev, port: u8, phv: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_find_cached_vlan(dev: *mut mlx4_dev, port: u8, vid: u16, idx: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_register_vlan(dev: *mut mlx4_dev, port: u8, vlan: u16, index: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_unregister_vlan(dev: *mut mlx4_dev, port: u8, vlan: u16);
}
extern "C" {
    pub fn mlx4_SYNC_TPT(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_test_interrupt(dev: *mut mlx4_dev, vector: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_test_async(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_get_eqs_per_port(dev: *mut mlx4_dev, port: u8) -> u32;
}
extern "C" {
    pub fn mlx4_is_eq_vector_valid(dev: *mut mlx4_dev, port: u8, vector: c_int) -> bool;
}
extern "C" {
    pub fn mlx4_assign_eq(dev: *mut mlx4_dev, port: u8, vector: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_release_eq(dev: *mut mlx4_dev, vec: c_int);
}
extern "C" {
    pub fn mlx4_is_eq_shared(dev: *mut mlx4_dev, vector: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_eq_get_irq(dev: *mut mlx4_dev, vec: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_get_phys_port_id(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_wol_read(dev: *mut mlx4_dev, config: *mut u64, port: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_wol_write(dev: *mut mlx4_dev, config: u64, port: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_counter_alloc(dev: *mut mlx4_dev, idx: *mut u32, usage: u8) -> c_int;
}
extern "C" {
    pub fn mlx4_counter_free(dev: *mut mlx4_dev, idx: u32);
}
extern "C" {
    pub fn mlx4_get_default_counter_index(dev: *mut mlx4_dev, port: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_get_admin_guid(dev: *mut mlx4_dev, entry: c_int, port: c_int) -> __be64;
}
extern "C" {
    pub fn mlx4_set_random_admin_guid(dev: *mut mlx4_dev, entry: c_int, port: c_int);
}
extern "C" {
    pub fn mlx4_flow_detach(dev: *mut mlx4_dev, reg_id: u64) -> c_int;
}
extern "C" {
    pub fn mlx4_hw_rule_sz(dev: *mut mlx4_dev, id: mlx4_net_trans_rule_id) -> c_int;
}
extern "C" {
    pub fn mlx4_get_parav_qkey(dev: *mut mlx4_dev, qpn: u32, qkey: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx4_is_slave_active(dev: *mut mlx4_dev, slave: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_gen_pkey_eqe(dev: *mut mlx4_dev, slave: c_int, port: u8) -> c_int;
}
extern "C" {
    pub fn mlx4_gen_guid_change_eqe(dev: *mut mlx4_dev, slave: c_int, port: u8) -> c_int;
}
extern "C" {
    pub fn mlx4_gen_slaves_port_mgt_ev(dev: *mut mlx4_dev, port: u8, attr: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_gen_port_state_change_eqe(dev: *mut mlx4_dev, slave: c_int, port: u8, port_subtype_change: u8) -> c_int;
}
extern "C" {
    pub fn mlx4_get_slave_port_state(dev: *mut mlx4_dev, slave: c_int, port: u8) -> slave_port_state;
}
extern "C" {
    pub fn set_and_calc_slave_port_state(dev: *mut mlx4_dev, slave: c_int, port: u8, event: c_int, gen_event: *mut slave_port_gen_event) -> c_int;
}
extern "C" {
    pub fn mlx4_put_slave_node_guid(dev: *mut mlx4_dev, slave: c_int, guid: __be64);
}
extern "C" {
    pub fn mlx4_get_slave_node_guid(dev: *mut mlx4_dev, slave: c_int) -> __be64;
}
extern "C" {
    pub fn mlx4_read_clock(dev: *mut mlx4_dev) -> u64;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_active_ports {
    pub MLX4_MAX_PORTS): DECLARE_BITMAP(ports,,
}

// Returns a bitmap of the physical ports which are assigned to slave
extern "C" {
    pub fn mlx4_get_active_ports(dev: *mut mlx4_dev, slave: c_int) -> mlx4_active_ports;
}
// Returns the physical port that represents the virtual port of the slave,
// or a value < 0 in case of an error. If a slave has 2 ports, the identity
// mapping is returned.
extern "C" {
    pub fn mlx4_slave_convert_port(dev: *mut mlx4_dev, slave: c_int, port: c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_slaves_pport {
    pub MLX4_MFUNC_MAX): DECLARE_BITMAP(slaves,,
}

// Returns a bitmap of all slaves that are assigned to port.
// Returns a bitmap of all slaves that are assigned exactly to all the
// the ports that are set in crit_ports.
// Returns the slave's virtual port that represents the physical port.
extern "C" {
    pub fn mlx4_phys_to_slave_port(dev: *mut mlx4_dev, slave: c_int, port: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_get_base_gid_ix(dev: *mut mlx4_dev, slave: c_int, port: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_config_vxlan_port(dev: *mut mlx4_dev, udp_port: __be16) -> c_int;
}
extern "C" {
    pub fn mlx4_disable_rx_port_check(dev: *mut mlx4_dev, dis: bool) -> c_int;
}
extern "C" {
    pub fn mlx4_config_roce_v2_port(dev: *mut mlx4_dev, udp_port: u16) -> c_int;
}
extern "C" {
    pub fn mlx4_virt2phy_port_map(dev: *mut mlx4_dev, port1: u32, port2: u32) -> c_int;
}
extern "C" {
    pub fn mlx4_vf_smi_enabled(dev: *mut mlx4_dev, slave: c_int, port: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_vf_get_enable_smi_admin(dev: *mut mlx4_dev, slave: c_int, port: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_mr_rereg_mem_cleanup(dev: *mut mlx4_dev, mr: *mut mlx4_mr);
}
extern "C" {
    pub fn mlx4_max_tc(dev: *mut mlx4_dev) -> c_int;
}
// Returns true if running in low memory profile (kdump kernel)
extern "C" {
    pub fn is_kdump_kernel() -> return;
}
// ACCESS REG commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_access_reg_method {
    MLX4_ACCESS_REG_QUERY = 0x1,
    MLX4_ACCESS_REG_WRITE = 0x2,
}

// ACCESS PTYS Reg command
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_ptys_proto {
    MLX4_PTYS_IB = 1<<0,
    MLX4_PTYS_EN = 1<<2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_ptys_flags {
    MLX4_PTYS_AN_DISABLE_CAP   = 1 << 5,
    MLX4_PTYS_AN_DISABLE_ADMIN = 1 << 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ptys_reg {
    pub flags: u8,
    pub local_port: u8,
    pub resrvd2: u8,
    pub proto_mask: u8,
    pub resrvd3: [__be32; 2],
    pub eth_proto_cap: __be32,
    pub ib_width_cap: __be16,
    pub ib_speed_cap: __be16,
    pub resrvd4: __be32,
    pub eth_proto_admin: __be32,
    pub ib_width_admin: __be16,
    pub ib_speed_admin: __be16,
    pub resrvd5: __be32,
    pub eth_proto_oper: __be32,
    pub ib_width_oper: __be16,
    pub ib_speed_oper: __be16,
    pub resrvd6: __be32,
    pub eth_proto_lp_adv: __be32,
    pub __packed: },
    pub ptys_reg): *mut mlx4_ptys_reg,
    pub params): *mut mlx4_clock_params,
    pub dev->uar_page_shift)): return (index << (PAGE_SHIFT -,
// The first 128 UARs are used for EQ doorbells
    pub dev->uar_page_shift)): return (128 >> (PAGE_SHIFT -,
