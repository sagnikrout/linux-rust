//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_device.h
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

pub type blist_flags_t = __u64 ;
pub const SCSI_SENSE_BUFFERSIZE: c_int = 96;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_mode_data {
    pub length: __u32,
    pub block_descriptor_length: __u16,
    pub medium_type: __u8,
    pub device_specific: __u8,
    pub header_length: __u8,
    pub longlba:1: __u8,
}

//
// sdev state: If you alter this, you also need to alter scsi_sysfs.c
// (for the ascii descriptions) and the state model enforcer:
// scsi_lib:scsi_device_set_state().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_device_state {
    SDEV_CREATED = 1,	/* device created but not added to sysfs
// Only internal commands allowed (for inq)
    SDEV_RUNNING,		/* device properly configured
// All commands allowed
    SDEV_CANCEL,		/* beginning to delete device
// Only error handler commands allowed
    SDEV_DEL,		/* device deleted
// no commands allowed
    SDEV_QUIESCE,		/* Device quiescent.  No block commands
// will be accepted, only specials (which
// originate in the mid-layer)
    SDEV_OFFLINE,		/* Device offlined (by error handling or
// user request
    SDEV_TRANSPORT_OFFLINE,	/* Offlined by transport class error handler */
    SDEV_BLOCK,		/* Device blocked by scsi lld.  No
// scsi commands from user or midlayer
// should be issued to the scsi
// lld.
    SDEV_CREATED_BLOCK,	/* same as above but for created devices */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_scan_mode {
    SCSI_SCAN_INITIAL = 0,
    SCSI_SCAN_RESCAN,
    SCSI_SCAN_MANUAL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_device_event {
    SDEV_EVT_MEDIA_CHANGE	= 1,	/* media has changed */
    SDEV_EVT_INQUIRY_CHANGE_REPORTED,		/* 3F 03  UA reported */
    SDEV_EVT_CAPACITY_CHANGE_REPORTED,		/* 2A 09  UA reported */
    SDEV_EVT_SOFT_THRESHOLD_REACHED_REPORTED,	/* 38 07  UA reported */
    SDEV_EVT_MODE_PARAMETER_CHANGE_REPORTED,	/* 2A 01  UA reported */
    SDEV_EVT_LUN_CHANGE_REPORTED,			/* 3F 0E  UA reported */
    SDEV_EVT_ALUA_STATE_CHANGE_REPORTED,		/* 2A 06  UA reported */
    SDEV_EVT_POWER_ON_RESET_OCCURRED,		/* 29 00  UA reported */

    SDEV_EVT_FIRST		= SDEV_EVT_MEDIA_CHANGE,
    SDEV_EVT_LAST		= SDEV_EVT_POWER_ON_RESET_OCCURRED,

    SDEV_EVT_MAXBITS	= SDEV_EVT_LAST + 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_event {
    pub evt_type: scsi_device_event,
    pub node: list_head,
// put union of data structures, for non-simple event types,
// here
//
}

//
// struct scsi_vpd - SCSI Vital Product Data
// @rcu: For kfree_rcu().
// @len: Length in bytes of @data.
// @data: VPD data as defined in various T10 SCSI standard documents.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_vpd {
    pub rcu: rcu_head,
    pub len: c_int,
    pub data: [c_uchar; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_device {
    pub host: *mut Scsi_Host,
    pub request_queue: *mut request_queue,
// the next two are protected by the host->host_lock
    pub /: *mut *mut list_head siblings; / list of all devices on this host,
    pub /: *mut *mut list_head same_target_siblings; / just the devices sharing same target id,
    pub budget_map: sbitmap,
    pub /: *mut *mut atomic_t device_blocked; / Device returned QUEUE_FULL.,
    pub restarts: core::sync::atomic::AtomicI32,
    pub list_lock: spinlock_t,
    pub starved_entry: list_head,
    pub /: *mut *mut unsigned short queue_depth; / How deep of a queue we want,
    pub /: *mut *mut unsigned short max_queue_depth; / max queue depth,
    pub /: *mut *mut unsigned short last_queue_full_depth; / These two are used by,
    pub /: *mut *mut unsigned short last_queue_full_count; / scsi_track_queue_full(),
    pub /: *mut *mut unsigned long last_queue_full_time; / last queue full time,
    pub /: *mut *mut unsigned long queue_ramp_up_period; / ramp up period in jiffies,

    pub /: *mut *mut unsigned long last_queue_ramp_up; / last queue ramp up time,
    pub channel: unsigned int id,,
    pub lun: u64,
    pub using: *mut *mut unsigned int manufacturer; / Manufacturer of device, for,
// vendor-specific cmd's
    pub /: *mut *mut unsigned sector_size; / size in bytes,
    pub /: *mut *mut *mut void hostdata; / available to low-level driver,
    pub type: c_uchar,
    pub scsi_level: c_char,
    pub /: *mut *mut char inq_periph_qual; / PQ from INQUIRY data,
    pub inquiry_mutex: mutex,
    pub /: *mut *mut unsigned char inquiry_len; / valid bytes in 'inquiry',
    pub /: *mut *mut *mut unsigned char  inquiry; / INQUIRY response data,
    pub 1]: char vendor[INQUIRY_VENDOR_LEN +,
    pub 1]: char model[INQUIRY_MODEL_LEN +,
    pub 1]: char rev[INQUIRY_REVISION_LEN +,

    pub vpd_pg0: *mut scsi_vpd __rcu,
    pub vpd_pg83: *mut scsi_vpd __rcu,
    pub vpd_pg80: *mut scsi_vpd __rcu,
    pub vpd_pg89: *mut scsi_vpd __rcu,
    pub vpd_pgb0: *mut scsi_vpd __rcu,
    pub vpd_pgb1: *mut scsi_vpd __rcu,
    pub vpd_pgb2: *mut scsi_vpd __rcu,
    pub vpd_pgb7: *mut scsi_vpd __rcu,
    pub sdev_target: *mut scsi_target,
    pub in: *mut *mut blist_flags_t sdev_bflags; / black/white flags as also found,
// scsi_devinfo.[hc]. For now used only to
// pass settings from sdev_init to scsi
// core.
    pub /: *mut *mut unsigned int eh_timeout; / Error handling timeout,
//
// If true, let the high-level device driver (sd) manage the device
// power state for system suspend/resume (suspend to RAM and
// hibernation) operations.
//
    pub manage_system_start_stop:1: unsigned,
//
// If true, let the high-level device driver (sd) manage the device
// power state for runtime device suspand and resume operations.
//
    pub manage_runtime_start_stop:1: unsigned,
//
// If true, let the high-level device driver (sd) manage the device
// power state for system shutdown (power off) operations.
//
    pub manage_shutdown:1: unsigned,
//
// If true, let the high-level device driver (sd) manage the device
// power state for system restart (reboot) operations.
//
    pub manage_restart:1: unsigned,
//
// If set and if the device is runtime suspended, ask the high-level
// device driver (sd) to force a runtime resume of the device.
//
    pub force_runtime_start_on_system_start:1: unsigned,
//
// Set if the device is an ATA device.
//
    pub is_ata:1: unsigned,
    pub removable:1: unsigned,
    pub /: *mut *mut unsigned changed:1; / Data invalid due to media change,
    pub /: *mut *mut unsigned busy:1; / Used to prevent races,
    pub /: *mut *mut unsigned lockable:1; / Able to prevent media removal,
    pub /: *mut *mut unsigned locked:1; / Media removal disabled,
    pub be: *mut *mut unsigned borken:1; / Tell the Seagate driver to,
// painfully slow on this device
    pub /: *mut *mut unsigned disconnect:1; / can disconnect,
    pub /: *mut *mut unsigned soft_reset:1; / Uses soft reset option,
    pub /: *mut *mut unsigned sdtr:1; / Device supports SDTR messages,
    pub /: *mut *mut unsigned wdtr:1; / Device supports WDTR messages,
    pub /: *mut *mut unsigned ppr:1; / Device supports PPR messages,
    pub /: *mut *mut unsigned tagged_supported:1; / Supports SCSI-II tagged queuing,
    pub /: *mut *mut unsigned simple_tags:1; / simple queue tag messages are enabled,
    pub for: *mut *mut unsigned was_reset:1; / There was a bus reset on the bus,
// this device
    pub CHECK_CONDITION/UNIT_ATTN: *mut *mut unsigned expecting_cc_ua:1; / Expecting a,
// because we did a bus reset.
    pub /: *mut *mut unsigned use_10_for_rw:1; / first try 10-byte read / write,
    pub /: *mut *mut unsigned use_10_for_ms:1; / first try 10-byte mode sense/select,
    pub /: *mut *mut unsigned set_dbd_for_ms:1; / Set "DBD" field in mode sense,
    pub /: *mut *mut unsigned read_before_ms:1; / perform a READ before MODE SENSE,
    pub /: *mut *mut unsigned no_report_opcodes:1; / no REPORT SUPPORTED OPERATION CODES,
    pub /: *mut *mut unsigned no_write_same:1; / no WRITE SAME command,
    pub /: *mut *mut unsigned use_16_for_rw:1; / Use read/write(16) over read/write(10),
    pub /: *mut *mut unsigned use_16_for_sync:1; / Use sync (16) over sync (10),
    pub /: *mut *mut unsigned skip_ms_page_8:1; / do not use MODE SENSE page 0x08,
    pub /: *mut *mut unsigned skip_ms_page_3f:1; / do not use MODE SENSE page 0x3f,
    pub /: *mut *mut unsigned skip_vpd_pages:1; / do not read VPD pages,
    pub /: *mut *mut unsigned try_vpd_pages:1; / attempt to read VPD pages,
    pub /: *mut *mut unsigned use_192_bytes_for_3f:1; / ask for 192 bytes from page 0x3f,
    pub /: *mut *mut unsigned no_start_on_add:1; / do not issue start on add,
    pub /: *mut *mut unsigned allow_restart:1; / issue START_UNIT in error handler,
    pub /: *mut *mut unsigned start_stop_pwr_cond:1; / Set power cond. in START_STOP_UNIT,
    pub /: *mut *mut unsigned no_uld_attach:1; / disable connecting to upper level drivers,
    pub select_no_atn:1: unsigned,
    pub /: *mut *mut unsigned fix_capacity:1; / READ_CAPACITY is too high by 1,
    pub /: *mut *mut unsigned guess_capacity:1; / READ_CAPACITY might be too high by 1,
    pub /: *mut *mut unsigned retry_hwerror:1; / Retry HARDWARE_ERROR,
    pub on: *mut *mut unsigned last_sector_bug:1; / do not use multisector accesses,
    pub /: *mut *mut unsigned no_read_disc_info:1; / Avoid READ_DISC_INFO cmds,
    pub /: *mut *mut unsigned no_read_capacity_16:1; / Avoid READ_CAPACITY_16 cmds,
    pub /: *mut *mut unsigned try_rc_10_first:1; / Try READ_CAPACACITY_10 first,
    pub /: *mut *mut unsigned security_supported:1; / Supports Security Protocols,
    pub /: *mut *mut unsigned is_visible:1; / is the device visible in sysfs,
    pub /: *mut *mut unsigned wce_default_on:1; / Cache is ON by default,
    pub /: *mut *mut unsigned no_dif:1; / T10 PI (DIF) should be disabled,
    pub /: *mut *mut unsigned broken_fua:1; / Don't set FUA bit,
    pub /: *mut *mut unsigned lun_in_cdb:1; / Store LUN bits in CDB[1],
    pub /: *mut *mut unsigned unmap_limit_for_ws:1; / Use the UNMAP limit for WRITE SAME,
    pub device: *mut *mut unsigned rpm_autosuspend:1; / Enable runtime autosuspend at,
// creation time
    pub /: *mut *mut unsigned ignore_media_change:1; / Ignore MEDIA CHANGE on resume,
    pub /: *mut *mut unsigned silence_suspend:1; / Do not print runtime PM related messages,
    pub /: *mut *mut unsigned no_vpd_size:1; / No VPD size reported in header,
    pub /: *mut *mut unsigned cdl_supported:1; / Command duration limits supported,
    pub /: *mut *mut unsigned cdl_enable:1; / Enable/disable Command duration limits,
    pub /: *mut *mut unsigned int queue_stopped; / request queue is quiesced,
    pub /: *mut *mut bool offline_already; / Device offline message logged,
    pub /: *mut *mut atomic_t ua_new_media_ctr; / Counter for New Media UNIT ATTENTIONs,
    pub /: *mut *mut atomic_t ua_por_ctr; / Counter for Power On / Reset UAs,
    pub /: *mut *mut atomic_t disk_events_disable_depth; / disable depth for disk events,
    pub /: *mut *mut DECLARE_BITMAP(supported_events, SDEV_EVT_MAXBITS); / supported events,
    pub /: *mut *mut DECLARE_BITMAP(pending_events, SDEV_EVT_MAXBITS); / pending events,
    pub /: *mut *mut list_head event_list; / asserted events,
    pub event_work: work_struct,
    pub /: *mut *mut unsigned int max_device_blocked; / what device_blocked counts down from,
pub const SCSI_DEFAULT_DEVICE_BLOCKED: c_int = 3;
    pub iorequest_cnt: core::sync::atomic::AtomicI32,
    pub iodone_cnt: core::sync::atomic::AtomicI32,
    pub ioerr_cnt: core::sync::atomic::AtomicI32,
    pub iotmo_cnt: core::sync::atomic::AtomicI32,
    pub requeue_work: work_struct,
    pub handler: *mut scsi_device_handler,
    pub handler_data: *mut c_void,
    pub dma_drain_len: usize,
    pub dma_drain_buf: *mut c_void,
    pub sg_timeout: c_uint,
    pub sg_reserved_size: c_uint,
    pub bsg_dev: *mut bsg_device,
    pub access_state: c_uchar,
    pub state_mutex: mutex,
    pub sdev_state: scsi_device_state,
    pub quiesced_by: *mut task_struct,
    pub sdev_data: [c_ulong; ],
// C attribute field omitted

//
// like scmd_printk, but the device name is passed in
// as a string pointer
//
    pub ...): *const *const char ,,

    pub \: *mut *mut request __rq = scsi_cmd_to_rq((scmd));,
    pub \: __rq->q->disk->disk_name, ##a);,
    pub \: sdev_dbg((scmd)->device, fmt, ##a);,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_target_state {
    STARGET_CREATED = 1,
    STARGET_RUNNING,
    STARGET_REMOVE,
    STARGET_CREATED_REMOVE,
    STARGET_DEL,
}

//
// scsi_target: representation of a scsi target, for now, this is only
// used for single_lun devices. If no one has active IO to the target,
// starget_sdev_user is NULL, else it points to the active sdev.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_target {
    pub starget_sdev_user: *mut scsi_device,
    pub siblings: list_head,
    pub devices: list_head,
    pub dev: device,
    pub /: *mut *mut kref reap_ref; / last put renders target invisible,
    pub channel: c_uint,
    pub replace: *mut *mut unsigned int id; / target id ...,
// scsi_device.id eventually
    pub /: *mut *mut unsigned int create:1; / signal that it needs to be added,
    pub only: *mut *mut unsigned int single_lun:1; / Indicates we should,
// allow I/O to one of the luns
// for the device at a time.
    pub 0x1f: *mut *mut unsigned int pdt_1f_for_no_lun:1; / PDT =,
// means no lun present.
    pub use: *mut *mut unsigned int no_report_luns:1; / Don't,
// REPORT LUNS for scanning.
    pub reported: *mut *mut unsigned int expecting_lun_change:1; / A device has,
// a 3F/0E UA, other devices on
// the same target will also.
// commands actually active on LLD.
    pub target_busy: core::sync::atomic::AtomicI32,
    pub target_blocked: core::sync::atomic::AtomicI32,
//
// LLDs should set this in the sdev_init host template callout.
// If set to zero then there is not limit.
//
    pub can_queue: c_uint,
    pub max_target_blocked: c_uint,
pub const SCSI_DEFAULT_TARGET_BLOCKED: c_int = 3;
    pub scsi_level: c_char,
    pub state: scsi_target_state,
    pub /: *mut *mut *mut void hostdata; / available to low-level driver,
    pub /: *mut *mut unsigned long starget_data[]; / for the transport,
// starget_data must be the last element!!!!
// C attribute field omitted

    pub to_scsi_target(sdev->sdev_gendev.parent): return,

    pub hostdata): *mut uint, uint, u64, void,
    pub lun): uint target, u64,
    pub scsi_dh): *mut extern int scsi_register_device_handler(struct scsi_device_handler,
    pub ): *mut extern void scsi_remove_device(struct scsi_device,
    pub scsi_dh): *mut extern int scsi_unregister_device_handler(struct scsi_device_handler,
    pub sdev): *mut void scsi_attach_vpd(struct scsi_device,
    pub sdev): *mut void scsi_cdl_check(struct scsi_device,
    pub enable): *mut *mut int scsi_cdl_enable(struct scsi_device sdev, bool,
    pub ): *mut extern int __must_check scsi_device_get(struct scsi_device,
    pub ): *mut extern void scsi_device_put(struct scsi_device,
    pub u64): uint, uint,,
    pub u64): uint, uint,,
    pub )): *mut *mut *mut void (fn)(struct scsi_device , void,
    pub )): *mut c_void,
// only exposed to implement shost_for_each_device
    pub ): *mut scsi_device,
//
// shost_for_each_device - iterate over all devices of a host
// @sdev: the &struct scsi_device to use as a cursor
// @shost: the &struct scsi_host to iterate over
//
// Iterator that returns each device attached to @shost.  This loop
// takes a reference on each device and releases it at the end.  If
// you break out of the loop, you must call scsi_device_put(sdev).
//

    pub \: for ((sdev) = __scsi_iterate_devices((shost), NULL);,
    pub \: (sdev);,
//
// __shost_for_each_device - iterate over all devices of a host (UNLOCKED)
// @sdev: the &struct scsi_device to use as a cursor
// @shost: the &struct scsi_host to iterate over
//
// Iterator that returns each device attached to @shost.  It does _not_
// take a reference on the scsi_device, so the whole loop must be
// protected by shost->host_lock.
//
// Note: The only reason to use this is because you need to access the
// device list in interrupt context.  Otherwise you really want to use
// shost_for_each_device instead.
//

    pub int): *mut *mut extern int scsi_change_queue_depth(struct scsi_device ,,
    pub int): *mut *mut extern int scsi_track_queue_full(struct scsi_device ,,
    pub char): *mut *mut extern int scsi_set_medium_removal(struct scsi_device ,,
    pub ): *mut scsi_sense_hdr,
    pub ): *mut scsi_sense_hdr,
    pub sshdr): *mut int retries, struct scsi_sense_hdr,
    pub buf_len): c_int,
    pub sa): c_ushort,
    pub state): scsi_device_state,
    pub gfpflags): gfp_t,
    pub evt): *mut *mut extern void sdev_evt_send(struct scsi_device sdev, struct scsi_event,
    pub gfpflags): scsi_device_event evt_type, gfp_t,
    pub sdev): *mut extern int scsi_device_quiesce(struct scsi_device,
    pub sdev): *mut extern void scsi_device_resume(struct scsi_device,
    pub ): *mut extern void scsi_target_quiesce(struct scsi_target,
    pub ): *mut extern void scsi_target_resume(struct scsi_target,
    pub rescan): scsi_scan_mode,
    pub ): *mut extern void scsi_target_reap(struct scsi_target,
    pub dev): *mut *mut void scsi_block_targets(struct Scsi_Host shost, struct device,
    pub scsi_device_state): *mut *mut extern void scsi_target_unblock(struct device , enum,
    pub ): *mut extern void scsi_remove_target(struct device,
    pub scsi_device_state): *const *const extern char scsi_device_state_name(enum,
    pub ): *const extern int scsi_is_sdev_device(struct device,
    pub ): *const extern int scsi_is_target_device(struct device,
    pub len): *mut *mut extern void scsi_sanitize_inquiry_string(unsigned char s, int,
//
// scsi_execute_cmd users can set scsi_failure.result to have
// scsi_check_passthrough fail/retry a command. scsi_failure.result can be a
// specific host byte or message code, or SCMD_FAILURE_RESULT_ANY can be used
// to match any host or message code.
//
pub const SCMD_FAILURE_RESULT_ANY: c_uint = 0x7fffffff;
//
// Set scsi_failure.result to SCMD_FAILURE_STAT_ANY to fail/retry any failure
// scsi_status_is_good returns false for.
//
pub const SCMD_FAILURE_STAT_ANY: c_uint = 0xff;
//
// The following can be set to the scsi_failure sense, asc and ascq fields to
// match on any sense, ASC, or ASCQ value.
//
pub const SCMD_FAILURE_SENSE_ANY: c_uint = 0xff;
pub const SCMD_FAILURE_ASC_ANY: c_uint = 0xff;
pub const SCMD_FAILURE_ASCQ_ANY: c_uint = 0xff;
// Always retry a matching failure.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_failure {
    pub result: c_int,
    pub sense: u8,
    pub asc: u8,
    pub ascq: u8,
//
// Number of times scsi_execute_cmd will retry the failure. It does
// not count for the total_allowed.
//
    pub allowed: i8,
// Number of times the failure has been retried.
    pub retries: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_failures {
//
// If a scsi_failure does not have a retry limit setup this limit will
// be used.
//
    pub total_allowed: c_int,
    pub total_retries: c_int,
    pub failure_definitions: *mut scsi_failure,
}

// Optional arguments to scsi_execute_cmd
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_exec_args {
    pub /: *mut *mut *mut unsigned char sense; / sense buffer,
    pub /: *mut *mut unsigned int sense_len; / sense buffer len,
    pub /: *mut *mut *mut scsi_sense_hdr sshdr; / decoded sense header,
    pub /: *mut *mut blk_mq_req_flags_t req_flags; / BLK_MQ_REQ flags,
    pub /: *mut *mut int scmd_flags; / SCMD flags,
    pub /: *mut *mut *mut int resid; / residual length,
    pub /: *mut *mut *mut scsi_failures failures; / failures to retry,
}

extern "C" {
    pub fn scsi_failures_reset_retries(failures: *mut scsi_failures);
}
extern "C" {
    pub fn scsi_put_internal_cmd(scmd: *mut scsi_cmnd);
}
extern "C" {
    pub fn sdev_disable_disk_events(sdev: *mut scsi_device);
}
extern "C" {
    pub fn sdev_enable_disk_events(sdev: *mut scsi_device);
}
extern "C" {
    pub fn scsi_vpd_lun_id(: *mut scsi_device, : *mut c_char, _arg: usize) -> c_int;
}
extern "C" {
    pub fn scsi_vpd_lun_serial(: *mut scsi_device, : *mut c_char, _arg: usize) -> c_int;
}
extern "C" {
    pub fn scsi_vpd_tpg_id(: *mut scsi_device, : *mut c_int) -> c_int;
}

extern "C" {
    pub fn scsi_autopm_get_device(: *mut scsi_device) -> c_int;
}
extern "C" {
    pub fn scsi_autopm_put_device(: *mut scsi_device);
}

extern "C" {
    pub fn device_reprobe(_arg: &sdev->sdev_gendev) -> return;
}

//
// scsi_device_is_pseudo_dev() - Whether a device is a pseudo SCSI device.
// @sdev: SCSI device to examine
//
// A pseudo SCSI device can be used to allocate SCSI commands but does not show
// up in sysfs. Additionally, the logical unit information in *@sdev is made up.
//
// This function tests the LUN number instead of comparing @sdev with
// @sdev->host->pseudo_sdev because this function may be called before
// @sdev->host->pseudo_sdev has been initialized.
//
// checks for positions of the SCSI state machine
//
extern "C" {
    pub fn scsi_internal_device_block_nowait(sdev: *mut scsi_device) -> c_int;
}
// accessor functions for the SCSI parameters
//
// scsi_device_supports_vpd - test if a device supports VPD pages
// @sdev: the &struct scsi_device to test
//
// If the 'try_vpd_pages' flag is set it takes precedence.
// Otherwise we will assume VPD pages are supported if the
// SCSI level is at least SPC-3 and 'skip_vpd_pages' is not set.
//
// Attempt VPD inquiry if the device blacklist explicitly calls
// for it.
//
// Although VPD inquiries can go to SCSI-2 type devices,
// some USB ones crash on receiving them, and the pages
// we currently ask for are mandatory for SPC-2 and beyond
//
extern "C" {
    pub fn sbitmap_weight(_arg: &sdev->budget_map) -> return;
}
// Macros to access the UNIT ATTENTION counters

