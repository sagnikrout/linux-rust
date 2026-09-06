//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/main.h
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
// Copyright (c) 2015 - 2021 Intel Corporation

pub const IRDMA_FW_VER_DEFAULT: c_int = 2;
pub const IRDMA_HW_VER: c_int = 2;
pub const IRDMA_ARP_ADD: c_int = 1;
pub const IRDMA_ARP_DELETE: c_int = 2;
pub const IRDMA_ARP_RESOLVE: c_int = 3;
pub const IRDMA_MACIP_ADD: c_int = 1;
pub const IRDMA_MACIP_DELETE: c_int = 2;

pub const IW_CEQ_SIZE: c_int = 2048;
pub const IW_AEQ_SIZE: c_int = 2048;

pub const IW_FIRST_QPN: c_int = 1;
pub const IW_SW_CONTEXT_ALIGN: c_int = 1024;
pub const MAX_DPC_ITERATIONS: c_int = 128;
pub const IRDMA_EVENT_TIMEOUT_MS: c_int = 5000;
pub const IRDMA_VCHNL_EVENT_TIMEOUT: c_int = 100000;
pub const IRDMA_RST_TIMEOUT_HZ: c_int = 4;
pub const IRDMA_NO_QSET: c_uint = 0xffff;
pub const IW_CFG_FPM_QP_COUNT: c_int = 32768;
pub const IRDMA_MAX_PAGES_PER_FMR: c_int = 262144;
pub const IRDMA_MIN_PAGES_PER_FMR: c_int = 1;
pub const IRDMA_CQP_COMPL_RQ_WQE_FLUSHED: c_int = 2;
pub const IRDMA_CQP_COMPL_SQ_WQE_FLUSHED: c_int = 3;
pub const IRDMA_Q_TYPE_PE_AEQ: c_uint = 0x80;
pub const IRDMA_Q_INVALID_IDX: c_uint = 0xffff;
pub const IRDMA_REM_ENDPOINT_TRK_QPID: c_int = 3;
pub const IRDMA_DRV_OPT_ENA_MPA_VER_0: c_uint = 0x00000001;
pub const IRDMA_DRV_OPT_DISABLE_MPA_CRC: c_uint = 0x00000002;
pub const IRDMA_DRV_OPT_DISABLE_FIRST_WRITE: c_uint = 0x00000004;
pub const IRDMA_DRV_OPT_DISABLE_INTF: c_uint = 0x00000008;
pub const IRDMA_DRV_OPT_ENA_MSI: c_uint = 0x00000010;
pub const IRDMA_DRV_OPT_DUAL_LOGICAL_PORT: c_uint = 0x00000020;
pub const IRDMA_DRV_OPT_NO_INLINE_DATA: c_uint = 0x00000080;
pub const IRDMA_DRV_OPT_DISABLE_INT_MOD: c_uint = 0x00000100;
pub const IRDMA_DRV_OPT_DISABLE_VIRT_WQ: c_uint = 0x00000200;
pub const IRDMA_DRV_OPT_ENA_PAU: c_uint = 0x00000400;
pub const IRDMA_DRV_OPT_MCAST_LOGPORT_MAP: c_uint = 0x00000800;

pub const IRDMA_ROCE_CWND_DEFAULT: c_uint = 0x400;
pub const IRDMA_ROCE_ACKCREDS_DEFAULT: c_uint = 0x1E;

pub const IRDMA_NUM_AEQ_MSIX: c_int = 1;
pub const IRDMA_MIN_MSIX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum init_completion_state {
    INVALID_STATE = 0,
    INITIAL_STATE,
    CQP_CREATED,
    HMC_OBJS_CREATED,
    HW_RSRC_INITIALIZED,
    CCQ_CREATED,
    CEQ0_CREATED,
    CEQS_CREATED,
    PBLE_CHUNK_MEM,
    AEQ_CREATED,
    ILQ_CREATED,
    IEQ_CREATED, /* Last state of probe */
    IP_ADDR_REGISTERED,  /* Last state of open */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_rsrc_limits {
    pub qplimit: u32,
    pub mrlimit: u32,
    pub cqlimit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cqp_err_info {
    pub maj: u16,
    pub min: u16,
    pub desc: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cqp_compl_info {
    pub op_ret_val: u32,
    pub maj_err_code: u16,
    pub min_err_code: u16,
    pub error: bool,
    pub op_code: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cqp_request {
    pub info: cqp_cmds_info,
    pub comp: completion,
    pub list: list_head,
    pub refcnt: refcount_t,
    pub cqp_request): *mut *mut void (callback_fcn)(struct irdma_cqp_request,
    pub param: *mut c_void,
    pub compl_info: irdma_cqp_compl_info,
    pub waiting:1: bool,
    pub dynamic:1: bool,
    pub pending:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cqp {
    pub sc_cqp: irdma_sc_cqp,
    pub /: *mut *mut spinlock_t req_lock; / protect CQP request list,
    pub /: *mut *mut spinlock_t compl_lock; / protect CQP completion processing,
    pub waitq: wait_queue_head_t,
    pub remove_wq: wait_queue_head_t,
    pub sq: irdma_dma_mem,
    pub host_ctx: irdma_dma_mem,
    pub scratch_array: *mut u64,
    pub cqp_requests: *mut irdma_cqp_request,
    pub oop_op_array: *mut irdma_ooo_cqp_op,
    pub cqp_avail_reqs: list_head,
    pub cqp_pending_reqs: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_ccq {
    pub sc_cq: irdma_sc_cq,
    pub mem_cq: irdma_dma_mem,
    pub shadow_area: irdma_dma_mem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_ceq {
    pub sc_ceq: irdma_sc_ceq,
    pub mem: irdma_dma_mem,
    pub irq: u32,
    pub msix_idx: u32,
    pub rf: *mut irdma_pci_f,
    pub dpc_tasklet: tasklet_struct,
    pub /: *mut *mut spinlock_t ce_lock; / sync cq destroy with cq completion event notification,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_aeq {
    pub sc_aeq: irdma_sc_aeq,
    pub mem: irdma_dma_mem,
    pub palloc: irdma_pble_alloc,
    pub virtual_map: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_arp_entry {
    pub ip_addr: [u32; 4],
    pub mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_msix_vector {
    pub idx: u32,
    pub irq: u32,
    pub cpu_affinity: u32,
    pub ceq_id: u32,
    pub mask: cpumask_t,
    pub name: [c_char; IRDMA_IRQ_NAME_STR_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_mc_table_info {
    pub mgn: u32,
    pub dest_ip: [u32; 4],
    pub lan_fwd:1: bool,
    pub ipv4_valid:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc_table_list {
    pub list: list_head,
    pub mc_info: irdma_mc_table_info,
    pub mc_grp_ctx: irdma_mcast_grp_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_qv_info {
    pub /: *mut *mut u32 v_idx; / msix_vector,
    pub ceq_idx: u16,
    pub aeq_idx: u16,
    pub itr_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_qvlist_info {
    pub num_vectors: u32,
    pub __counted_by(num_vectors): irdma_qv_info qv_info[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_gen_ops {
    pub rf): *mut *mut void (request_reset)(struct irdma_pci_f,
    pub tc_node): *mut irdma_ws_node,
    pub tc_node): *mut irdma_ws_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_pci_f {
    pub reset:1: bool,
    pub rsrc_created:1: bool,
    pub msix_shared:1: bool,
    pub hwqp1_rsvd:1: bool,
    pub rsrc_profile: u8,
    pub hmc_info_mem: *mut u8,
    pub mem_rsrc: *mut u8,
    pub rdma_ver: u8,
    pub rst_to: u8,
    pub pf_id: u8,
    pub protocol_used: irdma_protocol_used,
    pub sd_type: u32,
    pub msix_count: u32,
    pub max_mr: u32,
    pub max_qp: u32,
    pub max_cq: u32,
    pub max_srq: u32,
    pub next_srq: u32,
    pub max_ah: u32,
    pub next_ah: u32,
    pub max_mcg: u32,
    pub next_mcg: u32,
    pub max_pd: u32,
    pub next_qp: u32,
    pub next_cq: u32,
    pub next_pd: u32,
    pub max_mr_size: u32,
    pub max_cqe: u32,
    pub mr_stagmask: u32,
    pub used_pds: u32,
    pub used_cqs: u32,
    pub used_srqs: u32,
    pub used_mrs: u32,
    pub used_qps: u32,
    pub arp_table_size: u32,
    pub next_arp_index: u32,
    pub ceqs_count: u32,
    pub next_ws_node_id: u32,
    pub max_ws_node_id: u32,
    pub limits_sel: u32,
    pub allocated_ws_nodes: *mut c_ulong,
    pub allocated_qps: *mut c_ulong,
    pub allocated_cqs: *mut c_ulong,
    pub allocated_srqs: *mut c_ulong,
    pub allocated_mrs: *mut c_ulong,
    pub allocated_pds: *mut c_ulong,
    pub allocated_mcgs: *mut c_ulong,
    pub allocated_ahs: *mut c_ulong,
    pub allocated_arps: *mut c_ulong,
    pub init_state: init_completion_state,
    pub sc_dev: irdma_sc_dev,
    pub pcidev: *mut pci_dev,
    pub cdev: *mut c_void,
    pub hw: irdma_hw,
    pub cqp: irdma_cqp,
    pub ccq: irdma_ccq,
    pub aeq: irdma_aeq,
    pub ceqlist: *mut irdma_ceq,
    pub pble_rsrc: *mut irdma_hmc_pble_rsrc,
    pub arp_table: *mut irdma_arp_entry,
    pub access*/: *mut *mut spinlock_t arp_lock; /protect ARP table,
    pub /: *mut *mut spinlock_t rsrc_lock; / protect HW resource array access,
    pub access*/: *mut *mut spinlock_t qptable_lock; /protect QP table,
    pub access*/: *mut *mut spinlock_t cqtable_lock; /protect CQ table,
    pub qp_table: *mut irdma_qp,
    pub cq_table: *mut irdma_cq,
    pub /: *mut *mut spinlock_t qh_list_lock; / protect mc_qht_list,
    pub mc_qht_list: mc_table_list,
    pub iw_msixtbl: *mut irdma_msix_vector,
    pub iw_qvlist: *mut irdma_qvlist_info,
    pub dpc_tasklet: tasklet_struct,
    pub msix_entries: *mut msix_entry,
    pub obj_mem: irdma_dma_mem,
    pub obj_next: irdma_dma_mem,
    pub vchnl_msgs: core::sync::atomic::AtomicI32,
    pub vchnl_waitq: wait_queue_head_t,
    pub cqp_cmpl_wq: *mut workqueue_struct,
    pub cqp_cmpl_work: work_struct,
    pub vchnl_wq: *mut workqueue_struct,
    pub default_vsi: irdma_sc_vsi,
    pub back_fcn: *mut c_void,
    pub gen_ops: irdma_gen_ops,
    pub iwdev: *mut irdma_device,
    pub 8): DECLARE_HASHTABLE(ah_hash_tbl,,
    pub /: *mut *mut mutex ah_tbl_lock; / protect AH hash table access,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_device {
    pub ibdev: ib_device,
    pub rf: *mut irdma_pci_f,
    pub netdev: *mut net_device,
    pub cleanup_wq: *mut workqueue_struct,
    pub vsi: irdma_sc_vsi,
    pub cm_core: irdma_cm_core,
    pub roce_cwnd: u32,
    pub roce_ackcreds: u32,
    pub vendor_id: u32,
    pub vendor_part_id: u32,
    pub push_mode: u32,
    pub rcv_wnd: u32,
    pub mac_ip_table_idx: u16,
    pub vsi_num: u16,
    pub vport_id: u16,
    pub rcv_wscale: u8,
    pub iw_status: u8,
    pub roce_mode:1: bool,
    pub roce_dcqcn_en:1: bool,
    pub dcb_vlan_mode:1: bool,
    pub iw_ooo:1: bool,
    pub is_vport:1: bool,
    pub init_state: init_completion_state,
    pub suspend_wq: wait_queue_head_t,
}

extern "C" {
    pub fn container_of(_arg: ibdev, irdma_device: struct, _arg: ibdev) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibucontext, irdma_ucontext: struct, _arg: ibucontext) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibpd, irdma_pd: struct, _arg: ibpd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibah, irdma_ah: struct, _arg: ibah) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmr, irdma_mr: struct, _arg: ibmr) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmw, irdma_mr: struct, _arg: ibmw) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibcq, irdma_cq: struct, _arg: ibcq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibqp, irdma_qp: struct, _arg: ibqp) -> return;
}
extern "C" {
    pub fn container_of(_arg: dev, irdma_pci_f: struct, _arg: sc_dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibsrq, irdma_srq: struct, _arg: ibsrq) -> return;
}
//
// irdma_alloc_resource - allocate a resource
// @iwdev: device pointer
// @resource_array: resource bit array:
// @max_resources: maximum resource number
// @req_resources_num: Allocated resource number
// @next: next free id
//
// next = rsrc_num + 1;
// next = 0;
// req_rsrc_num = rsrc_num;
//
// irdma_free_resource - free a resource
// @iwdev: device pointer
// @resource_array: resource array for the resource_num
// @resource_num: resource number to free
//
extern "C" {
    pub fn irdma_ctrl_init_hw(rf: *mut irdma_pci_f) -> c_int;
}
extern "C" {
    pub fn irdma_ctrl_deinit_hw(rf: *mut irdma_pci_f);
}
extern "C" {
    pub fn irdma_rt_deinit_hw(iwdev: *mut irdma_device);
}
extern "C" {
    pub fn irdma_qp_add_ref(ibqp: *mut ib_qp);
}
extern "C" {
    pub fn irdma_qp_rem_ref(ibqp: *mut ib_qp);
}
extern "C" {
    pub fn irdma_free_lsmm_rsrc(iwqp: *mut irdma_qp);
}
extern "C" {
    pub fn irdma_flush_wqes(iwqp: *mut irdma_qp, flush_mask: u32);
}
extern "C" {
    pub fn irdma_alloc_local_mac_entry(rf: *mut irdma_pci_f, mac_tbl_idx: *mut u16) -> c_int;
}
extern "C" {
    pub fn irdma_add_local_mac_entry(rf: *mut irdma_pci_f, mac_addr: *const u8, idx: u16) -> c_int;
}
extern "C" {
    pub fn irdma_del_local_mac_entry(rf: *mut irdma_pci_f, idx: u16);
}
extern "C" {
    pub fn irdma_initialize_hw_rsrc(rf: *mut irdma_pci_f) -> u32;
}
extern "C" {
    pub fn irdma_port_ibevent(iwdev: *mut irdma_device);
}
extern "C" {
    pub fn irdma_cm_disconn(qp: *mut irdma_qp);
}
extern "C" {
    pub fn irdma_cq_add_ref(ibcq: *mut ib_cq);
}
extern "C" {
    pub fn irdma_cq_rem_ref(ibcq: *mut ib_cq);
}
extern "C" {
    pub fn irdma_cq_wq_destroy(rf: *mut irdma_pci_f, cq: *mut irdma_sc_cq);
}
extern "C" {
    pub fn irdma_srq_event(srq: *mut irdma_sc_srq);
}
extern "C" {
    pub fn irdma_srq_wq_destroy(rf: *mut irdma_pci_f, srq: *mut irdma_sc_srq);
}
extern "C" {
    pub fn irdma_cleanup_pending_cqp_op(rf: *mut irdma_pci_f);
}
extern "C" {
    pub fn irdma_get_timeout_threshold(dev: *mut irdma_sc_dev) -> c_int;
}
extern "C" {
    pub fn irdma_qp_suspend_resume(qp: *mut irdma_sc_qp, suspend: bool) -> c_int;
}
extern "C" {
    pub fn irdma_receive_ilq(vsi: *mut irdma_sc_vsi, rbuf: *mut irdma_puda_buf);
}
extern "C" {
    pub fn irdma_free_sqbuf(vsi: *mut irdma_sc_vsi, bufp: *mut c_void);
}
extern "C" {
    pub fn irdma_free_qp_rsrc(iwqp: *mut irdma_qp);
}
extern "C" {
    pub fn irdma_setup_cm_core(iwdev: *mut irdma_device, ver: u8) -> c_int;
}
extern "C" {
    pub fn irdma_cleanup_cm_core(cm_core: *mut irdma_cm_core);
}
extern "C" {
    pub fn irdma_send_syn(cm_node: *mut irdma_cm_node, sendack: u32) -> c_int;
}
extern "C" {
    pub fn irdma_send_reset(cm_node: *mut irdma_cm_node) -> c_int;
}
extern "C" {
    pub fn irdma_copy_ip_ntohl(dst: *mut u32, src: *mut __be32);
}
extern "C" {
    pub fn irdma_copy_ip_htonl(dst: *mut __be32, src: *mut u32);
}
extern "C" {
    pub fn irdma_get_vlan_ipv4(addr: *mut u32) -> u16;
}
extern "C" {
    pub fn irdma_get_vlan_mac_ipv6(addr: *mut u32, vlan_id: *mut u16, mac: *mut u8);
}
extern "C" {
    pub fn irdma_upload_qp_context(iwqp: *mut irdma_qp, freeze: bool, raw: bool) -> c_int;
}
extern "C" {
    pub fn irdma_cqp_ce_handler(rf: *mut irdma_pci_f, cq: *mut irdma_sc_cq);
}
extern "C" {
    pub fn irdma_gsi_ud_qp_ah_cb(cqp_request: *mut irdma_cqp_request);
}
extern "C" {
    pub fn irdma_add_ip(iwdev: *mut irdma_device);
}
extern "C" {
    pub fn cqp_compl_worker(work: *mut work_struct);
}
extern "C" {
    pub fn irdma_log_invalid_mtu(mtu: u16, dev: *mut irdma_sc_dev);
}
