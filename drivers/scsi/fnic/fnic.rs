//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/fnic.h
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
// Copyright 2008 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

pub const FABRIC_LOGO_MAX_RETRY: c_int = 3;
pub const DESC_CLEAN_LOW_WATERMARK: c_int = 8;

pub const FNIC_DFLT_QUEUE_DEPTH: c_int = 256;

pub const LUN0_DELAY_TIME: c_int = 9;

//
// Tag bits used for special requests.
//

//
// Command flags to identify the type of command and for other future
// use.
//
pub const FNIC_NO_FLAGS: c_int = 0;

pub const FNIC_FCOE_MAX_CMD_LEN: c_int = 16;
// Retry supported by rport (returned by PRLI service parameters)
pub const FNIC_FC_RP_FLAGS_RETRY: c_uint = 0x1;
// Cisco vendor id
pub const PCI_VENDOR_ID_CISCO: c_uint = 0x1137;
pub const PCI_DEVICE_ID_CISCO_VIC_FC: c_uint = 0x0045	/* fc vnic */;
// sereno pcie switch
pub const PCI_DEVICE_ID_CISCO_SERENO: c_uint = 0x004e;
pub const PCI_DEVICE_ID_CISCO_CRUZ: c_uint = 0x007a	/* Cruz */;
pub const PCI_DEVICE_ID_CISCO_BODEGA: c_uint = 0x0131	/* Bodega */;
pub const PCI_DEVICE_ID_CISCO_BEVERLY: c_uint = 0x025f	/* Beverly */;
// Sereno
pub const PCI_SUBDEVICE_ID_CISCO_VASONA: c_uint = 0x004f	/* vasona mezz */;
pub const PCI_SUBDEVICE_ID_CISCO_COTATI: c_uint = 0x0084	/* cotati mlom */;
pub const PCI_SUBDEVICE_ID_CISCO_LEXINGTON: c_uint = 0x0085	/* lexington pcie */;
pub const PCI_SUBDEVICE_ID_CISCO_ICEHOUSE: c_uint = 0x00cd	/* Icehouse */;
pub const PCI_SUBDEVICE_ID_CISCO_KIRKWOODLAKE: c_uint = 0x00ce	/* KirkwoodLake pcie */;
pub const PCI_SUBDEVICE_ID_CISCO_SUSANVILLE: c_uint = 0x012e	/* Susanville MLOM */;
pub const PCI_SUBDEVICE_ID_CISCO_TORRANCE: c_uint = 0x0139	/* Torrance MLOM */;
// Cruz
pub const PCI_SUBDEVICE_ID_CISCO_CALISTOGA: c_uint = 0x012c	/* Calistoga MLOM */;
pub const PCI_SUBDEVICE_ID_CISCO_MOUNTAINVIEW: c_uint = 0x0137	/* Cruz Mezz */;
// Cruz MountTian SIOC
pub const PCI_SUBDEVICE_ID_CISCO_MOUNTTIAN: c_uint = 0x014b;
pub const PCI_SUBDEVICE_ID_CISCO_CLEARLAKE: c_uint = 0x014d	/* ClearLake pcie */;
// Cruz MountTian2 SIOC
pub const PCI_SUBDEVICE_ID_CISCO_MOUNTTIAN2: c_uint = 0x0157;
pub const PCI_SUBDEVICE_ID_CISCO_CLAREMONT: c_uint = 0x015d	/* Claremont MLOM */;
// Bodega
// VIC 1457 PCIe mLOM
pub const PCI_SUBDEVICE_ID_CISCO_BRADBURY: c_uint = 0x0218;
pub const PCI_SUBDEVICE_ID_CISCO_BRENTWOOD: c_uint = 0x0217	/* VIC 1455 PCIe */;
// VIC 1487 PCIe mLOM
pub const PCI_SUBDEVICE_ID_CISCO_BURLINGAME: c_uint = 0x021a;
pub const PCI_SUBDEVICE_ID_CISCO_BAYSIDE: c_uint = 0x0219	/* VIC 1485 PCIe */;
// VIC 1440 Mezz mLOM
pub const PCI_SUBDEVICE_ID_CISCO_BAKERSFIELD: c_uint = 0x0215;
pub const PCI_SUBDEVICE_ID_CISCO_BOONVILLE: c_uint = 0x0216	/* VIC 1480 Mezz */;
pub const PCI_SUBDEVICE_ID_CISCO_BENICIA: c_uint = 0x024a	/* VIC 1495 */;
pub const PCI_SUBDEVICE_ID_CISCO_BEAUMONT: c_uint = 0x024b	/* VIC 1497 */;
pub const PCI_SUBDEVICE_ID_CISCO_BRISBANE: c_uint = 0x02af	/* VIC 1467 */;
pub const PCI_SUBDEVICE_ID_CISCO_BENTON: c_uint = 0x02b0	/* VIC 1477 */;
pub const PCI_SUBDEVICE_ID_CISCO_TWIN_RIVER: c_uint = 0x02cf	/* VIC 14425 */;
pub const PCI_SUBDEVICE_ID_CISCO_TWIN_PEAK: c_uint = 0x02d0	/* VIC 14825 */;
// Beverly
pub const PCI_SUBDEVICE_ID_CISCO_BERN: c_uint = 0x02de	/* VIC 15420 */;
pub const PCI_SUBDEVICE_ID_CISCO_STOCKHOLM: c_uint = 0x02dd	/* VIC 15428 */;
pub const PCI_SUBDEVICE_ID_CISCO_KRAKOW: c_uint = 0x02dc	/* VIC 15411 */;
pub const PCI_SUBDEVICE_ID_CISCO_LUCERNE: c_uint = 0x02db	/* VIC 15231 */;
pub const PCI_SUBDEVICE_ID_CISCO_TURKU: c_uint = 0x02e8	/* VIC 15238 */;
pub const PCI_SUBDEVICE_ID_CISCO_TURKU_PLUS: c_uint = 0x02f3	/* VIC 15237 */;
pub const PCI_SUBDEVICE_ID_CISCO_ZURICH: c_uint = 0x02df	/* VIC 15230 */;
pub const PCI_SUBDEVICE_ID_CISCO_RIGA: c_uint = 0x02e0	/* VIC 15427 */;
pub const PCI_SUBDEVICE_ID_CISCO_GENEVA: c_uint = 0x02e1	/* VIC 15422 */;
pub const PCI_SUBDEVICE_ID_CISCO_HELSINKI: c_uint = 0x02e4	/* VIC 15235 */;
pub const PCI_SUBDEVICE_ID_CISCO_GOTHENBURG: c_uint = 0x02f2	/* VIC 15425 */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_pcie_device {
    pub device: u32,
    pub desc: *mut u8,
    pub subsystem_device: u32,
    pub subsys_desc: *mut u8,
}

//
// fnic private data per SCSI command.
// These fields are locked by the hashed io_req_lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_cmd_priv {
    pub io_req: *mut fnic_io_req,
    pub state: fnic_ioreq_state,
    pub flags: u32,
    pub abts_status: u16,
    pub lr_status: u16,
}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
pub const FCPIO_INVALID_CODE: c_uint = 0x100 /* hdr_status value unused by firmware */;

pub const FNIC_MAX_FCP_TARGET: c_int = 256;
pub const FNIC_PCI_OFFSET: c_int = 2;
//
// state_flags to identify host state along along with fnic's state
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reset_states {
    NOT_IN_PROGRESS = 0,
    IN_PROGRESS,
    RESET_ERROR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rscn_type {
    NOT_PC_RSCN = 0,
    PC_RSCN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pc_rscn_handling_status {
    PC_RSCN_HANDLING_NOT_IN_PROGRESS = 0,
    PC_RSCN_HANDLING_IN_PROGRESS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pc_rscn_handling_feature {
    PC_RSCN_HANDLING_FEATURE_OFF = 0,
    PC_RSCN_HANDLING_FEATURE_ON
}

pub const FNIC_MAIN_LOGGING: c_uint = 0x01;
pub const FNIC_FCS_LOGGING: c_uint = 0x02;
pub const FNIC_SCSI_LOGGING: c_uint = 0x04;
pub const FNIC_ISR_LOGGING: c_uint = 0x08;
pub const FNIC_FDLS_LOGGING: c_uint = 0x10;
pub const FNIC_NVME_LOGGING: c_uint = 0x20;
pub const FNIC_FIP_LOGGING: c_uint = 0x40;

pub const FNIC_WQ_COPY_MAX: c_int = 64;
pub const FNIC_WQ_MAX: c_int = 1;
pub const FNIC_RQ_MAX: c_int = 1;

pub const FNIC_DFLT_IO_COMPLETIONS: c_int = 256;
pub const FNIC_MQ_CQ_INDEX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fnic_intx_intr_index {
    FNIC_INTX_WQ_RQ_COPYWQ,
    FNIC_INTX_DUMMY,
    FNIC_INTX_NOTIFY,
    FNIC_INTX_ERR,
    FNIC_INTX_INTR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fnic_msix_intr_index {
    FNIC_MSIX_RQ,
    FNIC_MSIX_WQ,
    FNIC_MSIX_WQ_COPY,
    FNIC_MSIX_ERR_NOTIFY = FNIC_MSIX_WQ_COPY + FNIC_WQ_COPY_MAX,
    FNIC_MSIX_INTR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_msix_entry {
    pub requested: c_int,
    pub 11]: char devname[IFNAMSIZ +,
    pub ): *mut *mut irqreturn_t (isr)(int, void,
    pub devid: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fnic_state {
    FNIC_IN_FC_MODE = 0,
    FNIC_IN_FC_TRANS_ETH_MODE,
    FNIC_IN_ETH_MODE,
    FNIC_IN_ETH_TRANS_FC_MODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fnic_role_e {
    FNIC_ROLE_FCP_INITIATOR = 0,
    FNIC_ROLE_NVME_INITIATOR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fnic_evt {
    FNIC_EVT_START_VLAN_DISC = 1,
    FNIC_EVT_START_FCF_DISC = 2,
    FNIC_EVT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_frame_list {
//
// Link to frame lists
//
    pub links: list_head,
    pub fp: *mut c_void,
    pub frame_len: c_int,
    pub rx_ethhdr_stripped: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_tag_t {
    pub free_list: list_head,
    pub tag_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_event {
    pub list: list_head,
    pub fnic: *mut fnic,
    pub event: fnic_evt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_cpy_wq {
    pub hw_lock_flags: c_ulong,
    pub active_ioreq_count: u16,
    pub ioreq_table_size: u16,
    pub io_req_table: *mut ____cacheline_aligned struct fnic_io_req,
}

// Per-instance private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic {
    pub fnic_num: c_int,
    pub role: fnic_role_e,
    pub iport: fnic_iport_s,
    pub host: *mut Scsi_Host,
    pub bar0: vnic_dev_bar,
    pub msix: [fnic_msix_entry; FNIC_MSIX_INTR_MAX],
    pub stats: *mut vnic_stats,
    pub /: *mut *mut unsigned long stats_time; / time of stats update,
    pub /: *mut *mut unsigned long stats_reset_time; / time of stats reset,
    pub nic_cfg: *mut vnic_nic_cfg,
    pub name: [c_char; IFNAMSIZ],
    pub /: *mut *mut timer_list notify_timer; / used for MSI interrupts,
    pub fnic_max_tag_id: c_uint,
    pub err_intr_offset: c_uint,
    pub link_intr_offset: c_uint,
    pub wq_count: c_uint,
    pub cq_count: c_uint,
    pub reset_completion_wait: completion,
    pub sgreset_mutex: mutex,
    pub /: *mut *mut spinlock_t sgreset_lock; / lock for sgreset,
    pub sgreset_sc: *mut scsi_cmnd,
    pub fnic_stats_debugfs_host: *mut dentry,
    pub fnic_stats_debugfs_file: *mut dentry,
    pub fnic_reset_debugfs_file: *mut dentry,
    pub reset_stats: c_uint,
    pub io_cmpl_skip: core::sync::atomic::AtomicI64,
    pub fnic_stats: fnic_stats,
    pub /: *mut *mut u32 vlan_hw_insert:1; / let hw insert the tag,
    pub /: *mut *mut u32 in_remove:1; / fnic device in removal,
    pub /: *mut *mut u32 stop_rx_link_events:1; / stop proc. rx frames, link events,
    pub fw_reset_done: *mut completion,
    pub reset_in_progress: u32,
    pub /: *mut *mut atomic_t in_flight; / io counter,
    pub internal_reset_inprogress: bool,
    pub /: *mut *mut u32 _reserved; / fill hole,
    pub /: *mut *mut unsigned long state_flags; / protected by host lock,
    pub state: fnic_state,
    pub fnic_lock: spinlock_t,
    pub lock_flags: c_ulong,
    pub /: *mut *mut u16 vlan_id; / VLAN tag including priority,
    pub data_src_addr: [u8; ETH_ALEN],
    pub /: *mut *mut u64 fcp_input_bytes; / internal statistic,
    pub /: *mut *mut u64 fcp_output_bytes; / internal statistic,
    pub link_down_cnt: u32,
    pub soft_reset_count: u32,
    pub link_status: c_int,
    pub list: list_head,
    pub links: list_head,
    pub pdev: *mut pci_dev,
    pub config: vnic_fc_config,
    pub vdev: *mut vnic_dev,
    pub raw_wq_count: c_uint,
    pub wq_copy_count: c_uint,
    pub rq_count: c_uint,
    pub fw_ack_index: [c_int; FNIC_WQ_COPY_MAX],
    pub fw_ack_recd: [c_ushort; FNIC_WQ_COPY_MAX],
    pub wq_copy_desc_low: [c_ushort; FNIC_WQ_COPY_MAX],
    pub intr_count: c_uint,
    pub legacy_pba: *mut u32 __iomem,
    pub tags: *mut fnic_host_tag,
    pub io_req_pool: *mut mempool_t,
    pub io_sgl_pool: [*mut mempool_t; FNIC_SGL_NUM_CACHES],
    pub copy_wq_base: c_uint,
    pub link_work: work_struct,
    pub frame_work: work_struct,
    pub flush_work: work_struct,
    pub frame_queue: list_head,
    pub tx_queue: list_head,
    pub frame_pool: *mut mempool_t,
    pub frame_elem_pool: *mut mempool_t,
    pub frame_recv_pool: *mut mempool_t,
    pub tport_work: work_struct,
    pub tport_event_list: list_head,
    pub subsys_desc: [c_char; 14],
    pub subsys_desc_len: c_int,
    pub pc_rscn_handling_status: c_int,
// FIP related data members  -- start
    pub vlan): *mut *mut *mut void (set_vlan)(struct fnic , u16,
    pub fip_frame_work: work_struct,
    pub fip_timer_work: work_struct,
    pub fip_frame_queue: list_head,
    pub fip_timer: timer_list,
    pub vlans_lock: spinlock_t,
    pub retry_fip_timer: timer_list,
    pub fcs_ka_timer: timer_list,
    pub enode_ka_timer: timer_list,
    pub vn_ka_timer: timer_list,
    pub vlan_list: list_head,
// FIP related data members  -- end
// NVME data members
    pub fnic_nvmef_debugfs_host: *mut dentry,
    pub fnic_nvmef_debugfs_file: *mut dentry,
    pub nvfnic_tag_map: sbitmap,
    pub nvme_io_cmpl_work: work_struct,
    pub nvme_io_event_queued: core::sync::atomic::AtomicI32,
    pub nvme_io_event_llist: llist_head,
    pub nvme_lport_unreg_done: *mut completion,
// copy work queue cache line section
    pub hw_copy_wq: [____cacheline_aligned struct vnic_wq_copy; FNIC_WQ_COPY_MAX],
    pub sw_copy_wq: [____cacheline_aligned struct fnic_cpy_wq; FNIC_WQ_COPY_MAX],
// completion queue cache line section
    pub cq: [____cacheline_aligned struct vnic_cq; FNIC_CQ_MAX],
    pub wq_copy_lock: [spinlock_t; FNIC_WQ_COPY_MAX],
// work queue cache line section
    pub wq: [____cacheline_aligned struct vnic_wq; FNIC_WQ_MAX],
    pub wq_lock: [spinlock_t; FNIC_WQ_MAX],
// receive queue cache line section
    pub rq: [____cacheline_aligned struct vnic_rq; FNIC_RQ_MAX],
// interrupt resource cache line section
    pub intr: [____cacheline_aligned struct vnic_intr; FNIC_MSIX_INTR_MAX],
}

extern "C" {
    pub fn fnic_clear_intr_mode(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_set_intr_mode(fnic: *mut fnic) -> c_int;
}
extern "C" {
    pub fn fnic_set_intr_mode_msix(fnic: *mut fnic) -> c_int;
}
extern "C" {
    pub fn fnic_free_intr(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_request_intr(fnic: *mut fnic) -> c_int;
}
extern "C" {
    pub fn fnic_free_wq_buf(wq: *mut vnic_wq, buf: *mut vnic_wq_buf);
}
extern "C" {
    pub fn fnic_handle_frame(work: *mut work_struct);
}
extern "C" {
    pub fn fnic_tport_event_handler(work: *mut work_struct);
}
extern "C" {
    pub fn fnic_handle_link(work: *mut work_struct);
}
extern "C" {
    pub fn fnic_handle_event(work: *mut work_struct);
}
extern "C" {
    pub fn fdls_reclaim_oxid_handler(work: *mut work_struct);
}
extern "C" {
    pub fn fdls_schedule_oxid_free(iport: *mut fnic_iport_s, active_oxid: *mut u16);
}
extern "C" {
    pub fn fdls_schedule_oxid_free_retry_work(work: *mut work_struct);
}
extern "C" {
    pub fn fnic_rq_cmpl_handler(fnic: *mut fnic, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn fnic_alloc_rq_frame(rq: *mut vnic_rq) -> c_int;
}
extern "C" {
    pub fn fnic_free_rq_buf(rq: *mut vnic_rq, buf: *mut vnic_rq_buf);
}
extern "C" {
    pub fn fnic_flush_tx(work: *mut work_struct);
}
extern "C" {
    pub fn fnic_update_mac_locked(: *mut fnic, new: *mut u8);
}
extern "C" {
    pub fn fnic_abort_cmd(: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn fnic_device_reset(: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn fnic_eh_host_reset_handler(sc: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn fnic_host_reset(shost: *mut Scsi_Host) -> c_int;
}
extern "C" {
    pub fn fnic_reset(shost: *mut Scsi_Host);
}
extern "C" {
    pub fn fnic_issue_fc_host_lip(shost: *mut Scsi_Host) -> c_int;
}
extern "C" {
    pub fn fnic_get_host_port_state(shost: *mut Scsi_Host);
}
extern "C" {
    pub fn fnic_fcpio_reset(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_wq_copy_cmpl_handler(fnic: *mut fnic, copy_work_to_do: c_int, cq_index: c_uint) -> c_int;
}
extern "C" {
    pub fn fnic_wq_cmpl_handler(fnic: *mut fnic, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn fnic_flogi_reg_handler(fnic: *mut fnic, _arg: u32) -> c_int;
}
extern "C" {
    pub fn fnic_fw_reset_handler(fnic: *mut fnic) -> c_int;
}
extern "C" {
    pub fn fnic_terminate_rport_io(: *mut fc_rport);
}
extern "C" {
    pub fn fnic_mq_map_queues_cpus(host: *mut Scsi_Host);
}
extern "C" {
    pub fn fnic_log_q_error(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_handle_link_event(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_stats_debugfs_init(fnic: *mut fnic) -> c_int;
}
extern "C" {
    pub fn fnic_stats_debugfs_remove(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_nvmef_debugfs_init(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_nvmef_debugfs_remove(fnic: *mut fnic);
}
extern "C" {
    pub fn nvfnic_get_nvmef_info(fnic: *mut fnic, info: *mut fnic_nvmef_info) -> c_int;
}
extern "C" {
    pub fn fnic_is_abts_pending(: *mut fnic, : *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn fnic_handle_fip_frame(work: *mut work_struct);
}
extern "C" {
    pub fn fnic_reset_work_handler(work: *mut work_struct);
}
extern "C" {
    pub fn fnic_handle_fip_event(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_fcoe_reset_vlans(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_handle_fip_timer(t: *mut timer_list);
}
extern "C" {
    pub fn __fnic_set_state_flags(: *mut fnic, long: unsigned, long: unsigned);
}
extern "C" {
    pub fn fnic_dump_fchost_stats(: *mut Scsi_Host, : *mut fc_host_statistics);
}
extern "C" {
    pub fn fnic_free_txq(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_free_rxq(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_fdls_link_status_change(fnic: *mut fnic, linkup: c_int);
}
extern "C" {
    pub fn fnic_delete_fcp_tports(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_flush_tport_event_list(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_count_ioreqs_wq(fnic: *mut fnic, hwq: u32, portid: u32) -> c_int;
}
extern "C" {
    pub fn fnic_count_ioreqs(fnic: *mut fnic, portid: u32) -> c_uint;
}
extern "C" {
    pub fn fnic_count_all_ioreqs(fnic: *mut fnic) -> c_uint;
}
extern "C" {
    pub fn fnic_scsi_unload(fnic: *mut fnic);
}
extern "C" {
    pub fn fnic_scsi_unload_cleanup(fnic: *mut fnic);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_scsi_iter_data {
    pub fnic: *mut fnic,
    pub data1: *mut c_void,
    pub data2: *mut c_void,
    pub data2): *mut *mut void data1, void,
}

