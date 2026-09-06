//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/scsi/zfcp_def.h
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


// SPDX-License-Identifier: GPL-2.0
//
// zfcp device driver
//
// Global definitions for the zfcp device driver.
//
// Copyright IBM Corp. 2002, 2020
//
// INCLUDES

// FSF SPECIFIC DEFINES
// ATTENTION: value must not be used by hardware
pub const FSF_QTCB_UNSOLICITED_STATUS: c_uint = 0x6305;
// ADAPTER/PORT/UNIT AND FSF_REQ STATUS FLAGS
//
// Note, the leftmost 12 status bits (3 nibbles) are common among adapter, port
// and unit. This is a mask for bitwise 'and' with status values.
//
pub const ZFCP_COMMON_FLAGS: c_uint = 0xfff00000;
// common status bits
pub const ZFCP_STATUS_COMMON_RUNNING: c_uint = 0x40000000;
pub const ZFCP_STATUS_COMMON_ERP_FAILED: c_uint = 0x20000000;
pub const ZFCP_STATUS_COMMON_UNBLOCKED: c_uint = 0x10000000;
pub const ZFCP_STATUS_COMMON_OPEN: c_uint = 0x04000000;
pub const ZFCP_STATUS_COMMON_ERP_INUSE: c_uint = 0x01000000;
pub const ZFCP_STATUS_COMMON_ACCESS_DENIED: c_uint = 0x00800000;
pub const ZFCP_STATUS_COMMON_ACCESS_BOXED: c_uint = 0x00400000;
pub const ZFCP_STATUS_COMMON_NOESC: c_uint = 0x00200000;
// adapter status
pub const ZFCP_STATUS_ADAPTER_MB_ACT: c_uint = 0x00000001;
pub const ZFCP_STATUS_ADAPTER_QDIOUP: c_uint = 0x00000002;
pub const ZFCP_STATUS_ADAPTER_SIOSL_ISSUED: c_uint = 0x00000004;
pub const ZFCP_STATUS_ADAPTER_XCONFIG_OK: c_uint = 0x00000008;
pub const ZFCP_STATUS_ADAPTER_HOST_CON_INIT: c_uint = 0x00000010;
pub const ZFCP_STATUS_ADAPTER_ERP_PENDING: c_uint = 0x00000100;
pub const ZFCP_STATUS_ADAPTER_LINK_UNPLUGGED: c_uint = 0x00000200;
pub const ZFCP_STATUS_ADAPTER_DATA_DIV_ENABLED: c_uint = 0x00000400;
// remote port status
pub const ZFCP_STATUS_PORT_PHYS_OPEN: c_uint = 0x00000001;
pub const ZFCP_STATUS_PORT_LINK_TEST: c_uint = 0x00000002;
// FSF request status (this does not have a common part)
pub const ZFCP_STATUS_FSFREQ_ERROR: c_uint = 0x00000008;
pub const ZFCP_STATUS_FSFREQ_CLEANUP: c_uint = 0x00000010;
pub const ZFCP_STATUS_FSFREQ_ABORTSUCCEEDED: c_uint = 0x00000040;
pub const ZFCP_STATUS_FSFREQ_ABORTNOTNEEDED: c_uint = 0x00000080;
pub const ZFCP_STATUS_FSFREQ_TMFUNCFAILED: c_uint = 0x00000200;
pub const ZFCP_STATUS_FSFREQ_DISMISSED: c_uint = 0x00001000;
pub const ZFCP_STATUS_FSFREQ_XDATAINCOMPLETE: c_uint = 0x00020000;
// STRUCTURE DEFINITIONS
//
// enum zfcp_erp_act_type - Type of ERP action object.
// @ZFCP_ERP_ACTION_REOPEN_LUN: LUN recovery.
// @ZFCP_ERP_ACTION_REOPEN_PORT: Port recovery.
// @ZFCP_ERP_ACTION_REOPEN_PORT_FORCED: Forced port recovery.
// @ZFCP_ERP_ACTION_REOPEN_ADAPTER: Adapter recovery.
//
// Values must fit into u8 because of code dependencies:
// zfcp_dbf_rec_trig(), &zfcp_dbf_rec_trigger.want, &zfcp_dbf_rec_trigger.need;
// zfcp_dbf_rec_run_lvl(), zfcp_dbf_rec_run(), &zfcp_dbf_rec_running.rec_action.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zfcp_erp_act_type {
    ZFCP_ERP_ACTION_REOPEN_LUN	   = 1,
    ZFCP_ERP_ACTION_REOPEN_PORT	   = 2,
    ZFCP_ERP_ACTION_REOPEN_PORT_FORCED = 3,
    ZFCP_ERP_ACTION_REOPEN_ADAPTER	   = 4,
}

//
// Values must fit into u16 because of code dependencies:
// zfcp_dbf_rec_run_lvl(), zfcp_dbf_rec_run(), zfcp_dbf_rec_run_wka(),
// &zfcp_dbf_rec_running.rec_step.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zfcp_erp_steps {
    ZFCP_ERP_STEP_UNINITIALIZED	= 0x0000,
    ZFCP_ERP_STEP_PHYS_PORT_CLOSING	= 0x0010,
    ZFCP_ERP_STEP_PORT_CLOSING	= 0x0100,
    ZFCP_ERP_STEP_PORT_OPENING	= 0x0800,
    ZFCP_ERP_STEP_LUN_CLOSING	= 0x1000,
    ZFCP_ERP_STEP_LUN_OPENING	= 0x2000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_erp_action {
    pub list: list_head,
    pub /: *mut *mut zfcp_erp_act_type type; / requested action code,
    pub /: *mut *mut *mut zfcp_adapter adapter; / device which should be recovered,
    pub port: *mut zfcp_port,
    pub sdev: *mut scsi_device,
    pub /: *mut *mut u32 status; / recovery status,
    pub /: *mut *mut zfcp_erp_steps step; / active step of this erp action,
    pub fsf_req_id: u64,
    pub timer: timer_list,
}

// holds various memory pools of an adapter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_adapter_mempool {
    pub erp_req: *mut mempool_t,
    pub gid_pn_req: *mut mempool_t,
    pub scsi_req: *mut mempool_t,
    pub scsi_abort: *mut mempool_t,
    pub status_read_req: *mut mempool_t,
    pub sr_data: *mut mempool_t,
    pub gid_pn: *mut mempool_t,
    pub qtcb_pool: *mut mempool_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_adapter {
    pub ref: kref,
    pub /: *mut *mut u64 peer_wwnn; / P2P peer WWNN,
    pub /: *mut *mut u64 peer_wwpn; / P2P peer WWPN,
    pub /: *mut *mut u32 peer_d_id; / P2P peer D_ID,
    pub /: *mut *mut *mut ccw_device ccw_device; / S/390 ccw device,
    pub qdio: *mut zfcp_qdio,
    pub /: *mut *mut u32 hydra_version; / Hydra version,
    pub fsf_lic_version: u32,
    pub /: *mut *mut u32 adapter_features; / FCP channel features,
    pub /: *mut *mut u32 connection_features; / host connection features,
    pub /: *mut *mut u32 hardware_version; / of FCP channel,
    pub /: *mut *mut u32 fc_security_algorithms; / of FCP channel,
    pub /: *mut *mut u32 fc_security_algorithms_old; / of FCP channel,
    pub /: *mut *mut u16 timer_ticks; / time int for a tick,
    pub /: *mut *mut *mut Scsi_Host scsi_host; / Pointer to mid-layer,
    pub /: *mut *mut list_head port_list; / remote port list,
    pub /: *mut *mut rwlock_t port_list_lock; / port list lock,
    pub /: *mut *mut u64 req_no; / unique FSF req number,
    pub req_list: *mut zfcp_reqlist,
    pub /: *mut *mut u32 fsf_req_seq_no; / FSF cmnd seq number,
    pub SCSI: *mut *mut rwlock_t abort_lock; / Protects against,
    pub reads*/: *mut *mut atomic_t stat_miss; / # missing status,
    pub stat_read_buf_num: c_uint,
    pub stat_work: work_struct,
    pub /: *mut *mut atomic_t status; / status of this adapter,
    pub this: *mut *mut list_head erp_ready_head; / error recovery for,
    pub erp_ready_wq: wait_queue_head_t,
    pub erp_running_head: list_head,
    pub erp_lock: rwlock_t,
    pub erp_done_wqh: wait_queue_head_t,
    pub /: *mut *mut zfcp_erp_action erp_action; / pending error recovery,
    pub erp_counter: core::sync::atomic::AtomicI32,
    pub erp: *mut *mut u32 erp_total_count; / total nr of enqueued,
    pub waiting: *mut *mut u32 erp_low_mem_count; / nr of erp actions,
    pub erp_thread: *mut task_struct,
    pub /: *mut *mut *mut zfcp_fc_wka_ports gs; / generic services,
    pub /: *mut *mut *mut zfcp_dbf dbf; / debug traces,
    pub /: *mut *mut zfcp_adapter_mempool pool; / Adapter memory pools,
    pub fc_stats: *mut fc_host_statistics,
    pub stats_reset_data: *mut fsf_qtcb_bottom_port,
    pub stats_reset: c_ulong,
    pub scan_work: delayed_work,
    pub ns_up_work: work_struct,
    pub service_level: service_level,
    pub work_queue: *mut workqueue_struct,
    pub dma_parms: device_dma_parameters,
    pub events: zfcp_fc_events,
    pub next_port_scan: c_ulong,
    pub diagnostics: *mut zfcp_diag_adapter,
    pub version_change_lost_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_port {
    pub dev: device,
    pub /: *mut *mut *mut fc_rport rport; / rport of fc transport class,
    pub /: *mut *mut list_head list; / list of remote ports,
    pub /: *mut *mut *mut zfcp_adapter adapter; / adapter used to access port,
    pub /: *mut *mut list_head unit_list; / head of logical unit list,
    pub /: *mut *mut rwlock_t unit_list_lock; / unit list lock,
    pub /: *mut *mut atomic_t units; / zfcp_unit count,
    pub /: *mut *mut atomic_t status; / status of this remote port,
    pub /: *mut *mut u64 wwnn; / WWNN if known,
    pub /: *mut *mut u64 wwpn; / WWPN,
    pub /: *mut *mut u32 d_id; / D_ID,
    pub /: *mut *mut u32 handle; / handle assigned by FSF,
    pub /: *mut *mut zfcp_erp_action erp_action; / pending error recovery,
    pub erp_counter: core::sync::atomic::AtomicI32,
    pub maxframe_size: u32,
    pub supported_classes: u32,
    pub connection_info: u32,
    pub connection_info_old: u32,
    pub gid_pn_work: work_struct,
    pub test_link_work: work_struct,
    pub rport_work: work_struct,
    pub rport_task: { RPORT_NONE, RPORT_ADD, RPORT_DEL },
    pub starget_id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_latency_record {
    pub min: u32,
    pub max: u32,
    pub sum: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_latency_cont {
    pub channel: zfcp_latency_record,
    pub fabric: zfcp_latency_record,
    pub counter: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_latencies {
    pub read: zfcp_latency_cont,
    pub write: zfcp_latency_cont,
    pub cmd: zfcp_latency_cont,
    pub lock: spinlock_t,
}

//
// struct zfcp_unit - LUN configured via zfcp sysfs
// @dev: struct device for sysfs representation and reference counting
// @list: entry in LUN/unit list per zfcp_port
// @port: reference to zfcp_port where this LUN is configured
// @fcp_lun: 64 bit LUN value
// @scsi_work: for running scsi_scan_target
//
// This is the representation of a LUN that has been configured for
// usage. The main data here is the 64 bit LUN value, data for
// running I/O and recovery is in struct zfcp_scsi_dev.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_unit {
    pub dev: device,
    pub list: list_head,
    pub port: *mut zfcp_port,
    pub fcp_lun: u64,
    pub scsi_work: work_struct,
}

//
// struct zfcp_scsi_dev - zfcp data per SCSI device
// @status: zfcp internal status flags
// @lun_handle: handle from "open lun" for issuing FSF requests
// @erp_action: zfcp erp data for opening and recovering this LUN
// @erp_counter: zfcp erp counter for this LUN
// @latencies: FSF channel and fabric latencies
// @port: zfcp_port where this LUN belongs to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_scsi_dev {
    pub status: core::sync::atomic::AtomicI32,
    pub lun_handle: u32,
    pub erp_action: zfcp_erp_action,
    pub erp_counter: core::sync::atomic::AtomicI32,
    pub latencies: zfcp_latencies,
    pub port: *mut zfcp_port,
}

//
// sdev_to_zfcp - Access zfcp LUN data for SCSI device
// @sdev: scsi_device where to get the zfcp_scsi_dev pointer
//
extern "C" {
    pub fn scsi_transport_device_data(_arg: sdev) -> return;
}
//
// zfcp_scsi_dev_lun - Return SCSI device LUN as 64 bit FCP LUN
// @sdev: SCSI device where to get the LUN from
//
// struct zfcp_fsf_req - basic FSF request structure
// @list: list of FSF requests
// @req_id: unique request ID
// @adapter: adapter this request belongs to
// @qdio_req: qdio queue related values
// @completion: used to signal the completion of the request
// @status: status of the request
// @qtcb: associated QTCB
// @data: private data
// @timer: timer data of this request
// @erp_action: reference to erp action if request issued on behalf of ERP
// @pool: reference to memory pool if used for this request
// @issued: time when request was send (STCK)
// @handler: handler which should be called to process response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_fsf_req {
    pub list: list_head,
    pub req_id: u64,
    pub adapter: *mut zfcp_adapter,
    pub qdio_req: zfcp_qdio_req,
    pub completion: completion,
    pub status: u32,
    pub qtcb: *mut fsf_qtcb,
    pub data: *mut c_void,
    pub timer: timer_list,
    pub erp_action: *mut zfcp_erp_action,
    pub pool: *mut mempool_t,
    pub issued: c_ulonglong,
    pub ): *mut *mut void (handler)(struct zfcp_fsf_req,
}
