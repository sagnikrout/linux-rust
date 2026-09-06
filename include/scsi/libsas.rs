//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/libsas.h
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
// SAS host prototypes and structures header file
//
// Copyright (C) 2005 Adaptec, Inc.  All rights reserved.
// Copyright (C) 2005 Luben Tuikov <luben_tuikov@adaptec.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sas_phy_role {
    PHY_ROLE_NONE = 0,
    PHY_ROLE_TARGET = 0x40,
    PHY_ROLE_INITIATOR = 0x80,
}

// The events are mnemonically described in sas_dump.c
// so when updating/adding events here, please also
// update the other file too.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port_event {
    PORTE_BYTES_DMAED     = 0U,
    PORTE_BROADCAST_RCVD,
    PORTE_LINK_RESET_ERR,
    PORTE_TIMER_EVENT,
    PORTE_HARD_RESET,
    PORT_NUM_EVENTS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_event {
    PHYE_LOSS_OF_SIGNAL   = 0U,
    PHYE_OOB_DONE,
    PHYE_OOB_ERROR,
    PHYE_SPINUP_HOLD,             /* hot plug SATA, no COMWAKE sent */
    PHYE_RESUME_TIMEOUT,
    PHYE_SHUTDOWN,
    PHY_NUM_EVENTS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum discover_event {
    DISCE_DISCOVER_DOMAIN   = 0U,
    DISCE_REVALIDATE_DOMAIN,
    DISCE_SUSPEND,
    DISCE_RESUME,
    DISC_NUM_EVENTS,
}

// ---------- Expander Devices ----------
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum routing_attribute {
    DIRECT_ROUTING,
    SUBTRACTIVE_ROUTING,
    TABLE_ROUTING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ex_phy_state {
    PHY_EMPTY,
    PHY_VACANT,
    PHY_NOT_PRESENT,
    PHY_DEVICE_DISCOVERED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex_phy {
    pub phy_id: c_int,
    pub phy_state: ex_phy_state,
    pub attached_dev_type: sas_device_type,
    pub linkrate: sas_linkrate,
    pub attached_sata_host:1: u8,
    pub attached_sata_dev:1: u8,
    pub attached_sata_ps:1: u8,
    pub attached_tproto: sas_protocol,
    pub attached_iproto: sas_protocol,
    pub attached_sas_addr: [u8; SAS_ADDR_SIZE],
    pub attached_phy_id: u8,
    pub phy_change_count: c_int,
    pub routing_attr: routing_attribute,
    pub virtual:1: u8,
    pub last_da_index: c_int,
    pub phy: *mut sas_phy,
    pub port: *mut sas_port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct expander_device {
    pub children: list_head,
    pub ex_change_count: c_int,
    pub max_route_indexes: u16,
    pub num_phys: u8,
    pub t2t_supp:1: u8,
    pub configuring:1: u8,
    pub conf_route_table:1: u8,
    pub enclosure_logical_id: [u8; 8],
    pub ex_phy: *mut ex_phy,
    pub parent_port: *mut sas_port,
    pub cmd_mutex: mutex,
}

// ---------- SATA device ----------
pub const ATA_RESP_FIS_SIZE: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sata_device {
    pub class: c_uint,
    pub /: *mut *mut u8 port_no; / port number, if this is a PM (Port),
    pub ap: *mut ata_port,
    pub ata_host: *mut ata_host,
    pub /: *mut *mut smp_rps_resp rps_resp ____cacheline_aligned; / report_phy_sata_resp,
    pub fis: [u8; ATA_RESP_FIS_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_device {
    pub /: *mut *mut list_head eh_list_node; / pending a user requested eh action,
    pub reset_lun: scsi_lun,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct domain_device {
    pub done_lock: spinlock_t,
    pub dev_type: sas_device_type,
    pub linkrate: sas_linkrate,
    pub min_linkrate: sas_linkrate,
    pub max_linkrate: sas_linkrate,
    pub pathways: c_int,
    pub parent: *mut domain_device,
    pub /: *mut *mut list_head siblings; / devices on the same level,
    pub /: *mut *mut *mut asd_sas_port port; / shortcut to root of the tree,
    pub phy: *mut sas_phy,
    pub dev_list_node: list_head,
    pub /: *mut *mut list_head disco_list_node; / awaiting probe or destruct,
    pub iproto: sas_protocol,
    pub tproto: sas_protocol,
    pub rphy: *mut sas_rphy,
    pub sas_addr: [u8; SAS_ADDR_SIZE],
    pub hashed_sas_addr: [u8; HASHED_SAS_ADDR_SIZE],
    pub frame_rcvd: [u8; 32],
    pub ex_dev: expander_device,
    pub /: *mut *mut sata_device sata_dev; / STP & directly attached,
    pub ssp_dev: ssp_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_work {
    pub drain_node: list_head,
    pub work: work_struct,
}

extern "C" {
    pub fn dev_is_expander(_arg: dev->parent->dev_type) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_discovery_event {
    pub work: sas_work,
    pub port: *mut asd_sas_port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_discovery {
    pub disc_work: [sas_discovery_event; DISC_NUM_EVENTS],
    pub pending: c_ulong,
    pub fanout_sas_addr: [u8; SAS_ADDR_SIZE],
    pub eeds_a: [u8; SAS_ADDR_SIZE],
    pub eeds_b: [u8; SAS_ADDR_SIZE],
    pub max_level: c_int,
}

// The port struct is Class:RW, driver:RO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_sas_port {
// private:
    pub disc: sas_discovery,
    pub port_dev: *mut domain_device,
    pub dev_list_lock: spinlock_t,
    pub dev_list: list_head,
    pub disco_list: list_head,
    pub destroy_list: list_head,
    pub sas_port_del_list: list_head,
    pub linkrate: sas_linkrate,
    pub work: sas_work,
    pub suspended: c_int,
// public:
    pub id: c_int,
    pub sas_addr: [u8; SAS_ADDR_SIZE],
    pub attached_sas_addr: [u8; SAS_ADDR_SIZE],
    pub iproto: sas_protocol,
    pub tproto: sas_protocol,
    pub oob_mode: sas_oob_mode,
    pub phy_list_lock: spinlock_t,
    pub phy_list: list_head,
    pub num_phys: c_int,
    pub phy_mask: u32,
    pub ha: *mut sas_ha_struct,
    pub port: *mut sas_port,
    pub /: *mut *mut *mut void lldd_port; / not touched by the sas class code,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_sas_event {
    pub work: sas_work,
    pub phy: *mut asd_sas_phy,
    pub event: c_int,
}

pub const SAS_PHY_SHUTDOWN_THRES: c_int = 1024;
// The phy pretty much is controlled by the LLDD.
// The class only reads those fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_sas_phy {
// private:
    pub event_nr: core::sync::atomic::AtomicI32,
    pub in_shutdown: c_int,
    pub error: c_int,
    pub suspended: c_int,
    pub phy: *mut sas_phy,
// public:
// The following are class:RO, driver:R/W
    pub /: *mut *mut int enabled; / must be set,
    pub /: *mut *mut int id; / must be set,
    pub iproto: sas_protocol,
    pub tproto: sas_protocol,
    pub role: sas_phy_role,
    pub oob_mode: sas_oob_mode,
    pub linkrate: sas_linkrate,
    pub /: *mut *mut *mut u8 sas_addr; / must be set,
    pub /: *mut *mut u8 attached_sas_addr[SAS_ADDR_SIZE]; / class:RO, driver: R/W,
    pub frame_rcvd_lock: spinlock_t,
    pub /: *mut *mut *mut u8 frame_rcvd; / must be set,
    pub frame_rcvd_size: c_int,
    pub sas_prim_lock: spinlock_t,
    pub sas_prim: u32,
    pub /: *mut *mut list_head port_phy_el; / driver:RO,
    pub /: *mut *mut *mut asd_sas_port port; / Class:RW, driver: RO,
    pub /: *mut *mut *mut sas_ha_ha; / may be set; the class sets it anyway,
    pub /: *mut *mut *mut void lldd_phy; / not touched by the sas_class_code,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sas_ha_state {
    SAS_HA_REGISTERED,
    SAS_HA_DRAINING,
    SAS_HA_ATA_EH_ACTIVE,
    SAS_HA_FROZEN,
    SAS_HA_RESUMING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_ha_struct {
// private:
    pub /: *mut *mut list_head defer_q; / work queued while draining,
    pub drain_mutex: mutex,
    pub state: c_ulong,
    pub lock: spinlock_t,
    pub eh_active: c_int,
    pub eh_wait_q: wait_queue_head_t,
    pub eh_dev_q: list_head,
    pub disco_mutex: mutex,
    pub shost: *mut Scsi_Host,
// public:
    pub sas_ha_name: *mut c_char,
    pub /: *mut *mut *mut device dev; / should be set,
    pub event_q: *mut workqueue_struct,
    pub disco_q: *mut workqueue_struct,
    pub /: *mut *mut *mut u8 sas_addr; / must be set,
    pub hashed_sas_addr: [u8; HASHED_SAS_ADDR_SIZE],
    pub phy_port_lock: spinlock_t,
    pub /: *mut *mut *mut *mut asd_sas_phy sas_phy; / array of valid pointers, must be set,
    pub /: *mut *mut *mut *mut asd_sas_port sas_port; / array of valid pointers, must be set,
    pub /: *mut *mut int num_phys; / must be set, gt 0, static,
    pub match: *mut *mut int strict_wide_ports; / both sas_addr and attached_sas_addr must,
// their siblings when forming wide ports
    pub /: *mut *mut *mut void lldd_ha; / not touched by sas class code,
    pub /: *mut *mut list_head eh_done_q; / complete via scsi_eh_flush_done_q,
    pub /: *mut *mut list_head eh_ata_q; / scmds to promote from sas to ata eh,
    pub event_thres: c_int,
}

extern "C" {
    pub fn starget_to_domain_dev(_arg: sdev->sdev_target) -> return;
}
extern "C" {
    pub fn sdev_to_domain_dev(_arg: cmd->device) -> return;
}
// Before calling a notify event, LLDD should use this function
// when the link is severed (possibly from its tasklet).
// The idea is that the Class only reads those, while the LLDD,
// can R/W these (thus avoiding a race).
//

extern "C" {
    pub fn try_test_sas_gpio_gp_bit(od: c_uint, data: *mut u8, index: u8, count: u8) -> c_int;
}

// ---------- Tasks ----------
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum service_response {
    SAS_TASK_COMPLETE,
    SAS_TASK_UNDELIVERED = -1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum exec_status {
//
// Values 0..0x7f are used to return the SAM_STAT_* codes.  To avoid
// 'case value not in enumerated type' compiler warnings every value
// returned through the exec_status enum needs an alias with the SAS_
// prefix here.
//
    SAS_SAM_STAT_GOOD = SAM_STAT_GOOD,
    SAS_SAM_STAT_BUSY = SAM_STAT_BUSY,
    SAS_SAM_STAT_TASK_ABORTED = SAM_STAT_TASK_ABORTED,
    SAS_SAM_STAT_CHECK_CONDITION = SAM_STAT_CHECK_CONDITION,

    SAS_DEV_NO_RESPONSE = 0x80,
    SAS_DATA_UNDERRUN,
    SAS_DATA_OVERRUN,
    SAS_INTERRUPTED,
    SAS_QUEUE_FULL,
    SAS_DEVICE_UNKNOWN,
    SAS_OPEN_REJECT,
    SAS_OPEN_TO,
    SAS_PROTO_RESPONSE,
    SAS_PHY_DOWN,
    SAS_NAK_R_ERR,
    SAS_PENDING,
    SAS_ABORTED_TASK,
}

// When a task finishes with a response, the LLDD examines the
// response:
// - For an ATA task task_status_struct::stat is set to
// SAS_PROTO_RESPONSE, and the task_status_struct::buf is set to the
// contents of struct ata_task_resp.
// - For SSP tasks, if no data is present or status/TMF response
// is valid, task_status_struct::stat is set.  If data is present
// (SENSE data), the LLDD copies up to SAS_STATUS_BUF_SIZE, sets
// task_status_struct::buf_valid_size, and task_status_struct::stat is
// set to SAM_CHECK_COND.
//
// "buf" has format SCSI Sense for SSP task, or struct ata_task_resp
// for ATA task.
//
// "frame_len" is the total frame length, which could be more or less
// than actually copied.
//
// Tasks ending with response, always set the residual field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_task_resp {
    pub frame_len: u16,
    pub /: *mut *mut u8 ending_fis[ATA_RESP_FIS_SIZE]; / dev to host or data-in,
}

pub const SAS_STATUS_BUF_SIZE: c_int = 96;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_status_struct {
    pub resp: service_response,
    pub stat: exec_status,
    pub buf_valid_size: c_int,
    pub buf: [u8; SAS_STATUS_BUF_SIZE],
    pub residual: u32,
    pub open_rej_reason: sas_open_rej_reason,
}

// ATA and ATAPI task queuable to a SAS LLDD.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_ata_task {
    pub fis: host_to_dev_fis,
    pub /: *mut *mut u8 atapi_packet[16]; / 0 if not ATAPI task,
    pub /: *mut *mut u8 dma_xfer:1; / PIO:0 or DMA:1,
    pub use_ncq:1: u8,
    pub return_fis_on_success:1: u8,
    pub device_control_reg_update:1: u8,
    pub force_phy: bool,
    pub force_phy_id: c_int,
}

// LLDDs rely on these values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sas_internal_abort {
    SAS_INTERNAL_ABORT_SINGLE	= 0,
    SAS_INTERNAL_ABORT_DEV		= 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_internal_abort_task {
    pub type: sas_internal_abort,
    pub qid: c_uint,
    pub tag: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_smp_task {
    pub smp_req: scatterlist,
    pub smp_resp: scatterlist,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum task_attribute {
    TASK_ATTR_SIMPLE = 0,
    TASK_ATTR_HOQ    = 1,
    TASK_ATTR_ORDERED= 2,
    TASK_ATTR_ACA    = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_ssp_task {
    pub LUN: [u8; 8],
    pub task_attr: task_attribute,
    pub cmd: *mut scsi_cmnd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_tmf_task {
    pub tmf: u8,
    pub tag_of_task_to_be_managed: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_task {
    pub dev: *mut domain_device,
    pub task_state_lock: spinlock_t,
    pub task_state_flags: unsigned,
    pub task_proto: sas_protocol,
    pub ata_task: sas_ata_task,
    pub smp_task: sas_smp_task,
    pub ssp_task: sas_ssp_task,
    pub abort_task: sas_internal_abort_task,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_task_slow {
// standard/extra infrastructure for slow path commands (SMP and
// internal lldd commands
//
    pub timer: timer_list,
    pub completion: completion,
    pub task: *mut sas_task,
}

pub const SAS_TASK_STATE_PENDING: c_int = 1;
pub const SAS_TASK_STATE_DONE: c_int = 2;
pub const SAS_TASK_STATE_ABORTED: c_int = 4;
pub const SAS_TASK_NEED_DEV_RESET: c_int = 8;
extern "C" {
    pub fn scsi_cmd_to_rq(_arg: scmd) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_domain_function_template {
// The class calls these to notify the LLDD of an event.
    pub ): *mut *mut void (lldd_port_formed)(struct asd_sas_phy,
    pub ): *mut *mut void (lldd_port_deformed)(struct asd_sas_phy,
// The class calls these when a device is found or gone.
    pub ): *mut *mut int (lldd_dev_found)(struct domain_device,
    pub ): *mut *mut void (lldd_dev_gone)(struct domain_device,
    pub gfp_flags): *mut *mut *mut int (lldd_execute_task)(struct sas_task , gfp_t,
// Task Management Functions. Must be called from process context.
    pub ): *mut *mut int (lldd_abort_task)(struct sas_task,
    pub lun): *mut *mut *mut int (lldd_abort_task_set)(struct domain_device , u8,
    pub lun): *mut *mut *mut int (lldd_clear_task_set)(struct domain_device , u8,
    pub ): *mut *mut int (lldd_I_T_nexus_reset)(struct domain_device,
    pub ): *mut *mut int (lldd_ata_check_ready)(struct domain_device,
    pub ): *mut *mut void (lldd_ata_set_dmamode)(struct domain_device,
    pub lun): *mut *mut *mut int (lldd_lu_reset)(struct domain_device , u8,
    pub ): *mut *mut int (lldd_query_task)(struct sas_task,
// Special TMF callbacks
    pub dev): *mut *mut void (lldd_tmf_exec_complete)(struct domain_device,
    pub task): *mut *mut void (lldd_tmf_aborted)(struct sas_task,
    pub data): *mut *mut *mut bool (lldd_abort_timeout)(struct sas_task task, void,
// Port and Adapter management
    pub ): *mut *mut int (lldd_clear_nexus_port)(struct asd_sas_port,
    pub ): *mut *mut int (lldd_clear_nexus_ha)(struct sas_ha_struct,
// Phy management
    pub ): *mut *mut *mut int (lldd_control_phy)(struct asd_sas_phy , enum phy_func, void,
// GPIO support
    pub write_data): *mut u8 reg_index, u8 reg_count, u8,
}

extern "C" {
    pub fn sas_register_ha(: *mut sas_ha_struct) -> c_int;
}
extern "C" {
    pub fn sas_unregister_ha(: *mut sas_ha_struct) -> c_int;
}
extern "C" {
    pub fn sas_prep_resume_ha(sas_ha: *mut sas_ha_struct);
}
extern "C" {
    pub fn sas_resume_ha(sas_ha: *mut sas_ha_struct);
}
extern "C" {
    pub fn sas_suspend_ha(sas_ha: *mut sas_ha_struct);
}
extern "C" {
    pub fn sas_phy_reset(phy: *mut sas_phy, hard_reset: c_int) -> c_int;
}
extern "C" {
    pub fn sas_phy_enable(phy: *mut sas_phy, enable: c_int) -> c_int;
}
extern "C" {
    pub fn sas_target_alloc(: *mut scsi_target) -> c_int;
}
extern "C" {
    pub fn sas_sdev_configure(dev: *mut scsi_device, lim: *mut queue_limits) -> c_int;
}
extern "C" {
    pub fn sas_change_queue_depth(: *mut scsi_device, new_depth: c_int) -> c_int;
}
extern "C" {
    pub fn sas_task_abort(: *mut sas_task);
}
extern "C" {
    pub fn sas_eh_abort_handler(cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn sas_eh_device_reset_handler(cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn sas_eh_target_reset_handler(cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn sas_eh_timed_out(cmd: *mut scsi_cmnd) -> scsi_timeout_action;
}
extern "C" {
    pub fn sas_target_destroy(: *mut scsi_target);
}
extern "C" {
    pub fn sas_sdev_init(: *mut scsi_device) -> c_int;
}
extern "C" {
    pub fn sas_drain_work(ha: *mut sas_ha_struct) -> c_int;
}
extern "C" {
    pub fn sas_request_addr(shost: *mut Scsi_Host, addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn sas_abort_task_set(dev: *mut domain_device, lun: *mut u8) -> c_int;
}
extern "C" {
    pub fn sas_clear_task_set(dev: *mut domain_device, lun: *mut u8) -> c_int;
}
extern "C" {
    pub fn sas_lu_reset(dev: *mut domain_device, lun: *mut u8) -> c_int;
}
extern "C" {
    pub fn sas_query_task(task: *mut sas_task, tag: u16) -> c_int;
}
extern "C" {
    pub fn sas_abort_task(task: *mut sas_task, tag: u16) -> c_int;
}

