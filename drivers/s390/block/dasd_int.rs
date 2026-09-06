//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/block/dasd_int.h
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
// Author(s)......: Holger Smolinski <Holger.Smolinski@de.ibm.com>
// Horst Hummel <Horst.Hummel@de.ibm.com>
// Martin Schwidefsky <schwidefsky@de.ibm.com>
// Bugreports.to..: <Linux390@de.ibm.com>
// Copyright IBM Corp. 1999, 2009
//
// we keep old device allocation scheme; IOW, minors are still in 0..255

//
// States a dasd device can have:
// new: the dasd_device structure is allocated.
// known: the discipline for the device is identified.
// basic: the device can do basic i/o.
// unfmt: the device could not be analyzed (format is unknown).
// ready: partition detection is done and the device is can do block io.
// online: the device accepts requests from the block device queue.
//
// Things to do for startup state transitions:
// new -> known: find discipline for the device and create devfs entries.
// known -> basic: request irq line for the device.
// basic -> ready: do the initial analysis, e.g. format detection,
// do block device setup and detect partitions.
// ready -> online: schedule the device tasklet.
// Things to do for shutdown state transitions:
// online -> ready: just set the new device state.
// ready -> basic: flush requests from the block device layer, clear
// partition information and reset format information.
// basic -> known: terminate all requests and free irq.
// known -> new: remove devfs entries and forget discipline.
//
pub const DASD_STATE_NEW: c_int = 0;
pub const DASD_STATE_KNOWN: c_int = 1;
pub const DASD_STATE_BASIC: c_int = 2;
pub const DASD_STATE_UNFMT: c_int = 3;
pub const DASD_STATE_READY: c_int = 4;
pub const DASD_STATE_ONLINE: c_int = 5;

// DASD discipline magic
pub const DASD_ECKD_MAGIC: c_uint = 0xC5C3D2C4;
pub const DASD_DIAG_MAGIC: c_uint = 0xC4C9C1C7;
pub const DASD_FBA_MAGIC: c_uint = 0xC6C2C140;
//
// SECTION: Type definitions
//
// BIT DEFINITIONS FOR SENSE DATA
pub const DASD_SENSE_BIT_0: c_uint = 0x80;
pub const DASD_SENSE_BIT_1: c_uint = 0x40;
pub const DASD_SENSE_BIT_2: c_uint = 0x20;
pub const DASD_SENSE_BIT_3: c_uint = 0x10;
// BIT DEFINITIONS FOR SIM SENSE
pub const DASD_SIM_SENSE: c_uint = 0x0F;
pub const DASD_SIM_MSG_TO_OP: c_uint = 0x03;
pub const DASD_SIM_LOG: c_uint = 0x0C;
// lock class for nested cdev lock
pub const CDEV_NESTED_FIRST: c_int = 1;
pub const CDEV_NESTED_SECOND: c_int = 2;
//
// SECTION: MACROs for klogd and s390 debug feature (dbf)
//

// definition of dbf debug levels

// Macro to calculate number of blocks per page

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_ccw_req {
    pub /: *mut *mut unsigned int magic; / Eye catcher,
    pub /: *mut *mut int intrc; / internal error, e.g. from start_IO,
    pub /: *mut *mut list_head devlist; / for dasd_device request queue,
    pub /: *mut *mut list_head blocklist; / for dasd_block request queue,
    pub /: *mut *mut *mut dasd_block block; / the originating block device,
    pub /: *mut *mut *mut dasd_device memdev; / the device used to allocate this,
    pub /: *mut *mut *mut dasd_device startdev; / device the request is started on,
    pub /: *mut *mut *mut dasd_device basedev; / base device if no block->base,
    pub /: *mut *mut *mut void cpaddr; / address of ccw or tcw,
    pub /: *mut *mut short retries; / A retry counter,
    pub /: *mut *mut unsigned char cpmode; / 0 = cmd mode, 1 = itcw,
    pub /: *mut *mut char status; / status of this request,
    pub /: *mut *mut char lpm; / logical path mask,
    pub /: *mut *mut unsigned long flags; / flags of this request,
    pub dq: *mut dasd_queue,
    pub /: *mut *mut unsigned long starttime; / jiffies time of request start,
    pub /: *mut *mut unsigned long expires; / expiration period in jiffies,
    pub /: *mut *mut *mut void data; / pointer to data area,
    pub /: *mut *mut irb irb; / device status in case of an error,
    pub /: *mut *mut *mut dasd_ccw_req refers; / ERP-chain queueing.,
    pub /: *mut *mut *mut void function; / originating ERP action,
    pub mem_chunk: *mut c_void,
    pub /: *mut *mut unsigned long buildclk; / TOD-clock of request generation,
    pub /: *mut *mut unsigned long startclk; / TOD-clock of request start,
    pub /: *mut *mut unsigned long stopclk; / TOD-clock of request interrupt,
    pub /: *mut *mut unsigned long endclk; / TOD-clock of request termination,
    pub data): *mut *mut *mut void (callback)(struct dasd_ccw_req , void,
    pub callback_data: *mut c_void,
    pub /: *mut *mut unsigned int proc_bytes; / bytes for partial completion,
    pub /: *mut *mut unsigned int trkcount; / count formatted tracks,
    pub /: *mut *mut *mut void filldata; / address of filler data,
    pub format: *mut dasd_format_entry,
    pub start_trk: sector_t,
    pub end_trk: sector_t,
    pub collision: bool,
}

//
// dasd_ccw_req -> status can be:
//
pub const DASD_CQR_FILLED: c_uint = 0x00	/* request is ready to be processed */;
pub const DASD_CQR_DONE: c_uint = 0x01	/* request is completed successfully */;
pub const DASD_CQR_NEED_ERP: c_uint = 0x02	/* request needs recovery action */;
pub const DASD_CQR_IN_ERP: c_uint = 0x03	/* request is in recovery */;
pub const DASD_CQR_FAILED: c_uint = 0x04	/* request is finally failed */;
pub const DASD_CQR_TERMINATED: c_uint = 0x05	/* request was stopped by driver */;
pub const DASD_CQR_ABORTED: c_uint = 0x06	/* request was replaced and will be deleted */;
pub const DASD_CQR_QUEUED: c_uint = 0x80	/* request is queued to be processed */;
pub const DASD_CQR_IN_IO: c_uint = 0x81	/* request is currently in IO */;
pub const DASD_CQR_ERROR: c_uint = 0x82	/* request is completed with error */;
pub const DASD_CQR_CLEAR_PENDING: c_uint = 0x83	/* request is clear pending */;
pub const DASD_CQR_CLEARED: c_uint = 0x84	/* request was cleared */;
pub const DASD_CQR_SUCCESS: c_uint = 0x85	/* request was successful */;
pub const DASD_CQR_ABORT: c_uint = 0x86	/* request was replaced and will not be handled */;
// default expiration time
pub const DASD_EXPIRES: c_int = 300;
pub const DASD_EXPIRES_MAX: c_int = 40000000;
pub const DASD_RETRIES: c_int = 256;
pub const DASD_RETRIES_MAX: c_int = 32768;
// per dasd_ccw_req flags

// stolen. Should not be combined with
// DASD_CQR_FLAGS_USE_ERP
//
// The following flags are used to suppress output of certain errors.
//

pub const DASD_REQ_PER_DEV: c_int = 4;
// Signature for error recovery functions.
//
// A single CQR can only contain a maximum of 255 CCWs. It is limited by
// the locate record and locate record extended count value which can only hold
// 1 Byte max.
//
pub const DASD_CQR_MAX_CCW: c_int = 255;
//
// Unique identifier for dasd device.
//
pub const UA_NOT_CONFIGURED: c_uint = 0x00;
pub const UA_BASE_DEVICE: c_uint = 0x01;
pub const UA_BASE_PAV_ALIAS: c_uint = 0x02;
pub const UA_HYPER_PAV_ALIAS: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_uid {
    pub type: __u8,
    pub vendor: [c_char; 4],
    pub serial: [c_char; 15],
    pub ssid: __u16,
    pub real_unit_addr: __u8,
    pub base_unit_addr: __u8,
    pub vduit: [c_char; 33],
}

// SSID   */ 4 + 1 + /* unit addr */ 2 + 1 +	\
// vduit */ 32 + 1)
//
// PPRC Status data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_pprc_header {
    pub /: *mut *mut __u8 entries; / 0 Number of device entries,
    pub /: *mut *mut __u8 unused; / 1 unused,
    pub /: *mut *mut __u16 entry_length; / 2-3 Length of device entry,
    pub /: *mut *mut __u32 unused2; / 4-7 unused,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_pprc_dev_info {
    pub /: *mut *mut __u8 state; / 0 Copy State,
    pub /: *mut *mut __u8 flags; / 1 Flags,
    pub /: *mut *mut __u8 reserved1[2]; / 2-3 reserved,
    pub /: *mut *mut __u8 prim_lss; / 4 Primary device LSS,
    pub /: *mut *mut __u8 primary; / 5 Primary device address,
    pub /: *mut *mut __u8 sec_lss; / 6 Secondary device LSS,
    pub /: *mut *mut __u8 secondary; / 7 Secondary device address,
    pub /: *mut *mut __u16 pprc_id; / 8-9 Peer-to-Peer Remote Copy ID,
    pub /: *mut *mut __u8 reserved2[12]; / 10-21 reserved,
    pub /: *mut *mut __u16 prim_cu_ssid; / 22-23 Primary Control Unit SSID,
    pub /: *mut *mut __u8 reserved3[12]; / 24-35 reserved,
    pub /: *mut *mut __u16 sec_cu_ssid; / 36-37 Secondary Control Unit SSID,
    pub /: *mut *mut __u8 reserved4[90]; / 38-127 reserved,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_pprc_data_sc4 {
    pub header: dasd_pprc_header,
    pub dev_info: [dasd_pprc_dev_info; 5],
    pub __packed: },
pub const DASD_BUS_ID_SIZE: c_int = 20;
pub const DASD_CP_ENTRIES: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_copy_entry {
    pub busid: [c_char; DASD_BUS_ID_SIZE],
    pub device: *mut dasd_device,
    pub primary: bool,
    pub configured: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_copy_relation {
    pub entry: [dasd_copy_entry; DASD_CP_ENTRIES],
    pub active: *mut dasd_copy_entry,
}

//
// the struct dasd_discipline is
// sth like a table of virtual functions, if you think of dasd_eckd
// inheriting dasd...
// no, currently we are not planning to reimplement the driver in C++
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_discipline {
    pub owner: *mut module,
    pub /: *mut *mut char ebcname[8]; / a name used for tagging and printks,
    pub /: *mut *mut char name[8]; / a name used for tagging and printks,
    pub has_discard: bool,
    pub /: *mut *mut list_head list; / used for list of disciplines,
//
// Device recognition functions. check_device is used to verify
// the sense data and the information returned by read device
// characteristics. It returns 0 if the discipline can be used
// for the device in question. uncheck_device is called during
// device shutdown to deregister a device from its discipline.
//
    pub ): *mut *mut int (check_device) (struct dasd_device,
    pub ): *mut *mut void (uncheck_device) (struct dasd_device,
//
// do_analysis is used in the step from device state "basic" to
// state "accept". It returns 0 if the device can be made ready,
// it returns -EMEDIUMTYPE if the device can't be made ready or
// -EAGAIN if do_analysis started a ccw that needs to complete
// before the analysis may be repeated.
//
    pub ): *mut *mut int (do_analysis) (struct dasd_block,
//
// This function is called, when new paths become available.
// Disciplins may use this callback to do necessary setup work,
// e.g. verify that new path is compatible with the current
// configuration.
//
    pub __u8): *mut *mut *mut int (pe_handler)(struct dasd_device , __u8,,
//
// Last things to do when a device is set online, and first things
// when it is set offline.
//
    pub ): *mut *mut int (basic_to_ready) (struct dasd_device,
    pub ): *mut *mut int (online_to_ready) (struct dasd_device,
    pub ): *mut *mut int (basic_to_known)(struct dasd_device,
    pub ): *mut *mut unsigned int (max_sectors)(struct dasd_block,
// (struct dasd_device *);
// Device operation functions. build_cp creates a ccw chain for
// a block device request, start_io starts the request and
// term_IO cancels it (e.g. in case of a timeout). format_device
// formats the device and check_device_format compares the format of
// a device with the expected format_data.
// handle_terminated_request allows to examine a cqr and prepare
// it for retry.
//
    pub ): *mut request,
    pub ): *mut *mut int (start_IO) (struct dasd_ccw_req,
    pub ): *mut *mut int (term_IO) (struct dasd_ccw_req,
    pub ): *mut *mut void (handle_terminated_request) (struct dasd_ccw_req,
    pub int): *mut *mut format_data_t ,,
    pub int): *mut *mut format_check_t ,,
    pub ): *mut *mut *mut int (free_cp) (struct dasd_ccw_req , struct request,
//
// Error recovery functions. examine_error() returns a value that
// indicates what to do for an error condition. If examine_error()
// returns 'dasd_era_recover' erp_action() is called to create a
// special error recovery ccw. erp_postaction() is called after
// an error recovery ccw has finished its execution. dump_sense
// is called for every error condition to print the sense data
// to the console.
//
    pub ): *mut *mut dasd_erp_fn_t(erp_action) (struct dasd_ccw_req,
    pub ): *mut *mut dasd_erp_fn_t(erp_postaction) (struct dasd_ccw_req,
    pub ): *mut irb,
    pub ): *mut *mut *mut *mut void (dump_sense_dbf) (struct dasd_device , struct irb , char,
    pub ): *mut irb,
// i/o control functions.
    pub ): *mut *mut *mut int (fill_geometry) (struct dasd_block , struct hd_geometry,
    pub ): *mut *mut *mut int (fill_info) (struct dasd_device , struct dasd_information2_t,
    pub ): *mut *mut *mut int (ioctl) (struct dasd_block , unsigned int, void __user,
// reload device after state change
    pub ): *mut *mut int (reload) (struct dasd_device,
    pub ): *mut *mut *mut int (get_uid) (struct dasd_device , struct dasd_uid,
    pub ): *mut *mut void (kick_validate) (struct dasd_device,
    pub __u8): *mut *mut *mut int (check_attention)(struct dasd_device ,,
    pub ): *mut *mut int (host_access_count)(struct dasd_device,
    pub ): *mut *mut *mut int (hosts_print)(struct dasd_device , struct seq_file,
    pub ): *mut *mut *mut void (handle_hpf_error)(struct dasd_device , struct irb,
    pub ): *mut *mut void (disable_hpf)(struct dasd_device,
    pub ): *mut *mut int (hpf_enabled)(struct dasd_device,
    pub __u8): *mut *mut *mut void (reset_path)(struct dasd_device ,,
//
// Extent Space Efficient (ESE) relevant functions
//
    pub ): *mut *mut int (is_ese)(struct dasd_device,
    pub ): *mut *mut int (ese_capable)(struct dasd_device,
// Whether the volume is formatted on demand (thin), from the label
    pub ): *mut *mut int (on_demand_format)(struct dasd_device,
// Fill discard queue limits
    pub ): *mut *mut *mut void (disc_limits)(struct dasd_block , struct queue_limits,
// Capacity
    pub ): *mut *mut int (space_allocated)(struct dasd_device,
    pub ): *mut *mut int (space_configured)(struct dasd_device,
    pub ): *mut *mut int (logical_capacity)(struct dasd_device,
    pub ): *mut *mut *mut int (release_space)(struct dasd_device , struct format_data_t,
// Extent Pool
    pub ): *mut *mut int (ext_pool_id)(struct dasd_device,
    pub ): *mut *mut int (ext_size)(struct dasd_device,
    pub ): *mut *mut int (ext_pool_cap_at_warnlevel)(struct dasd_device,
    pub ): *mut *mut int (ext_pool_warn_thrshld)(struct dasd_device,
    pub ): *mut *mut int (ext_pool_oos)(struct dasd_device,
    pub ): *mut *mut *mut int (ext_pool_exhaust)(struct dasd_device , struct dasd_ccw_req,
    pub ): *mut *mut *mut *mut void (ese_format)(struct dasd_device , struct dasd_ccw_req , struct irb,
    pub ): *mut *mut *mut int (ese_read)(struct dasd_ccw_req , struct irb,
    pub ): *mut *mut *mut int (pprc_status)(struct dasd_device , struct dasd_pprc_data_sc4,
    pub ): *mut *mut bool (pprc_enabled)(struct dasd_device,
    pub ): *mut *mut *mut *mut int (copy_pair_swap)(struct dasd_device , char , char,
    pub ): *mut *mut int (device_ping)(struct dasd_device,
}

// Trigger IDs for extended error reporting DASD EER and autoquiesce
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eer_trigger {
    DASD_EER_FATALERROR = 1,
    DASD_EER_NOPATH,
    DASD_EER_STATECHANGE,
    DASD_EER_PPRCSUSPEND,
    DASD_EER_NOSPC,
    DASD_EER_TIMEOUTS,
    DASD_EER_STARTIO,

// enum end marker, only add new trigger above
    DASD_EER_MAX,
    DASD_EER_AUTOQUIESCE = 31, /* internal only */
}

// DASD path handling
pub const DASD_PATH_OPERATIONAL: c_int = 1;
pub const DASD_PATH_TBV: c_int = 2;
pub const DASD_PATH_PP: c_int = 3;
pub const DASD_PATH_NPP: c_int = 4;
pub const DASD_PATH_MISCABLED: c_int = 5;
pub const DASD_PATH_NOHPF: c_int = 6;
pub const DASD_PATH_CUIR: c_int = 7;
pub const DASD_PATH_IFCC: c_int = 8;
pub const DASD_PATH_FCSEC: c_int = 9;

// FC Endpoint Security Capabilities
pub const DASD_FC_SECURITY_UNSUP: c_int = 0;
pub const DASD_FC_SECURITY_AUTH: c_int = 1;
pub const DASD_FC_SECURITY_ENC_FCSP2: c_int = 2;
pub const DASD_FC_SECURITY_ENC_ERAS: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_path {
    pub flags: c_ulong,
    pub cssid: u8,
    pub ssid: u8,
    pub chpid: u8,
    pub conf_data: *mut dasd_conf_data,
    pub error_count: core::sync::atomic::AtomicI32,
    pub errorclk: c_ulong,
    pub fc_security: u8,
    pub kobj: kobject,
    pub in_sysfs: bool,
}

// Memory for the dasd_path kobject is freed when dasd_free_device() is called
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_profile_info {
// legacy part of profile data, as in dasd_profile_info_t
    pub /: *mut *mut unsigned int dasd_io_reqs; / number of requests processed,
    pub /: *mut *mut unsigned int dasd_io_sects; / number of sectors processed,
    pub /: *mut *mut unsigned int dasd_io_secs[32]; / histogram of request's sizes,
    pub /: *mut *mut unsigned int dasd_io_times[32]; / histogram of requests's times,
    pub /: *mut *mut unsigned int dasd_io_timps[32]; / h. of requests's times per sector,
    pub /: *mut *mut unsigned int dasd_io_time1[32]; / hist. of time from build to start,
    pub /: *mut *mut unsigned int dasd_io_time2[32]; / hist. of time from start to irq,
    pub /: *mut *mut unsigned int dasd_io_time2ps[32]; / hist. of time from start to irq,
    pub /: *mut *mut unsigned int dasd_io_time3[32]; / hist. of time from irq to end,
    pub /: *mut *mut unsigned int dasd_io_nr_req[32]; / hist. of # of requests in chanq,
// new data
    pub /: *mut *mut timespec64 starttod; / time of start or last reset,
    pub /: *mut *mut unsigned int dasd_io_alias; / requests using an alias,
    pub /: *mut *mut unsigned int dasd_io_tpm; / requests using transport mode,
    pub /: *mut *mut unsigned int dasd_read_reqs; / total number of read requests,
    pub /: *mut *mut unsigned int dasd_read_sects; / total number read sectors,
    pub /: *mut *mut unsigned int dasd_read_alias; / read request using an alias,
    pub /: *mut *mut unsigned int dasd_read_tpm; / read requests in transport mode,
    pub /: *mut *mut unsigned int dasd_read_secs[32]; / histogram of request's sizes,
    pub /: *mut *mut unsigned int dasd_read_times[32]; / histogram of requests's times,
    pub /: *mut *mut unsigned int dasd_read_time1[32]; / hist. time from build to start,
    pub /: *mut *mut unsigned int dasd_read_time2[32]; / hist. of time from start to irq,
    pub /: *mut *mut unsigned int dasd_read_time3[32]; / hist. of time from irq to end,
    pub /: *mut *mut unsigned int dasd_read_nr_req[32]; / hist. of # of requests in chanq,
    pub /: *mut *mut unsigned long dasd_sum_times; / sum of request times,
    pub /: *mut *mut unsigned long dasd_sum_time_str; / sum of time from build to start,
    pub /: *mut *mut unsigned long dasd_sum_time_irq; / sum of time from start to irq,
    pub /: *mut *mut unsigned long dasd_sum_time_end; / sum of time from irq to end,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_profile {
    pub dentry: *mut dentry,
    pub data: *mut dasd_profile_info,
    pub lock: spinlock_t,
}

//
// concurrent ESE format ranges in flight; also caps a WRITE_FULL_TRACK's
// track count, which the LRE track bitmask limits to 16
//
pub const DASD_NR_FORMAT_ENTRIES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_format_entry {
    pub list: list_head,
    pub cqr: *mut dasd_ccw_req,
    pub start_trk: sector_t,
    pub end_trk: sector_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_device {
// Block device stuff.
    pub block: *mut dasd_block,
    pub devindex: c_uint,
    pub /: *mut *mut unsigned long flags; / per device flags,
    pub /: *mut *mut unsigned short features; / copy of devmap-features (read-only!),
// extended error reporting stuff (eer)
    pub eer_cqr: *mut dasd_ccw_req,
// Device discipline stuff.
    pub discipline: *mut dasd_discipline,
    pub base_discipline: *mut dasd_discipline,
    pub private: *mut c_void,
    pub path: [dasd_path; 8],
    pub opm: __u8,
// Device state and target state.
    pub target: int state,,
    pub state_mutex: mutex,
    pub /: *mut *mut int stopped; / device (ccw_device_start) was stopped,
// reference count.
    pub ref_count: core::sync::atomic::AtomicI32,
// ccw queue and memory for static ccw/erp buffers.
    pub ccw_queue: list_head,
    pub mem_lock: spinlock_t,
    pub ccw_mem: *mut c_void,
    pub fill_mem: *mut c_void,
    pub erp_mem: *mut c_void,
    pub ese_mem: *mut c_void,
    pub nulldata: *mut c_void,
    pub ccw_chunks: list_head,
    pub fill_chunks: list_head,
    pub erp_chunks: list_head,
    pub ese_chunks: list_head,
    pub tasklet_scheduled: core::sync::atomic::AtomicI32,
    pub tasklet: tasklet_struct,
    pub kick_work: work_struct,
    pub reload_device: work_struct,
    pub kick_validate: work_struct,
    pub suc_work: work_struct,
    pub requeue_requests: work_struct,
    pub timer: timer_list,
    pub debug_area: *mut debug_info_t,
    pub cdev: *mut ccw_device,
// hook for alias management
    pub alias_list: list_head,
// default expiration time in s
    pub default_expires: c_ulong,
    pub default_retries: c_ulong,
    pub blk_timeout: c_ulong,
    pub path_thrhld: c_ulong,
    pub path_interval: c_ulong,
    pub debugfs_dentry: *mut dentry,
    pub hosts_dentry: *mut dentry,
    pub profile: dasd_profile,
    pub format_entry: [dasd_format_entry; DASD_NR_FORMAT_ENTRIES],
    pub paths_info: *mut kset,
    pub copy: *mut dasd_copy_relation,
    pub aq_mask: c_ulong,
    pub aq_timeouts: c_uint,
// ESE fulltrack write control (see full_track_bias sysfs attribute)
    pub /: *mut *mut unsigned int ft_bias; / aggressiveness 0..100: 0=off, 100=always,
    pub /: *mut *mut unsigned int fulltrack; / internal: use WRITE_FULL_TRACK for aligned writes,
// adaptive heuristic (active for ft_bias 1..99), derived from ft_bias
    pub /: *mut *mut unsigned int ese_probe_state; / heuristic FSM state,
    pub /: *mut *mut unsigned int ese_probe_interval; / IOs between evaluations,
    pub /: *mut *mut atomic_t ese_io_cnt; / IO counter for current window,
    pub /: *mut *mut atomic_t ese_nrf_window; / NRF/INV_TRACK_FORMAT events in window,
    pub /: *mut *mut unsigned int ese_heu_start_interval; / IOs before first probe,
    pub /: *mut *mut unsigned int ese_heu_probe_window; / IOs in probe window,
    pub /: *mut *mut unsigned int ese_heu_max_interval; / max IOs between probes (backoff cap),
    pub /: *mut *mut unsigned int ese_heu_nrf_high; / NRF per-mille threshold → activate ft1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_block {
// Block device stuff.
    pub gdp: *mut gendisk,
    pub request_queue_lock: spinlock_t,
    pub tag_set: blk_mq_tag_set,
    pub bdev_file: *mut file,
    pub open_count: core::sync::atomic::AtomicI32,
    pub /: *mut *mut unsigned long blocks; / size of volume in blocks,
    pub /: *mut *mut unsigned int bp_block; / bytes per block,
    pub /: *mut *mut unsigned int s2b_shift; / log2 (bp_block/512),
    pub base: *mut dasd_device,
    pub ccw_queue: list_head,
    pub queue_lock: spinlock_t,
    pub tasklet_scheduled: core::sync::atomic::AtomicI32,
    pub tasklet: tasklet_struct,
    pub timer: timer_list,
    pub debugfs_dentry: *mut dentry,
    pub profile: dasd_profile,
    pub format_list: list_head,
    pub format_lock: spinlock_t,
    pub trkcount: core::sync::atomic::AtomicI32,
//
// ESE format CQRs staged from hardirq, spliced into
// ccw_queue in dasd_block_tasklet under queue_lock. Direct enqueue from
// the IRQ handler would invert the queue_lock / ccwdev_lock order.
//
    pub ese_staging: list_head,
// lock for ese_staging
    pub ese_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_attention_data {
    pub device: *mut dasd_device,
    pub lpum: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_queue {
    pub lock: spinlock_t,
}

// reasons why device (ccw_device_start) was stopped

//
// ESE fulltrack write aggressiveness (full_track_bias sysfs attribute), 0..100:
// 0   - never use proactively WRITE_FULL_TRACK
// 100 - always use proactively WRITE_FULL_TRACK, no probing
// 1..99 - adaptive; higher means switch to ft more eagerly
// WRITE_FULL_TRACK has an advantage on sparse formatted ESE devices
// but it has an overall penalty for maximum throughput for fully
// formatted devices.
// The default of 50 tries to balance both and do some probing in between
// to choose the best mode for default IO.
//
pub const DASD_FT_BIAS_MAX: c_int = 100;
pub const DASD_FT_BIAS_DEFAULT: c_int = 50;
// ESE fulltrack heuristic FSM states (adaptive range, ft_bias 1..99)

//
// Heuristic parameters are derived from ft_bias by linear interpolation,
// anchored so that ft_bias == 50 reproduces the previously shipped defaults
// and ft_bias == 100 is the most aggressive end of the range.
// probe_window is constant.
//
pub const DASD_ESE_HEU_PROBE_WINDOW: c_int = 100;

pub const DASD_ESE_HEU_NRF_HIGH_A100: c_int = 1;

pub const DASD_ESE_HEU_START_A100: c_int = 500;

pub const DASD_ESE_HEU_MAX_A100: c_int = 20000;
// per device flags

// confuse this with the user specified
// read-only feature.
//

extern "C" {
    pub fn dasd_put_device_wake(: *mut dasd_device);
}
//
// return values to be returned from the copy pair swap function
// 0x00: swap successful
// 0x01: swap data invalid
// 0x02: no active device found
// 0x03: wrong primary specified
// 0x04: secondary device not found
// 0x05: swap already running
//
pub const DASD_COPYPAIRSWAP_SUCCESS: c_int = 0;
pub const DASD_COPYPAIRSWAP_INVALID: c_int = 1;
pub const DASD_COPYPAIRSWAP_NOACTIVE: c_int = 2;
pub const DASD_COPYPAIRSWAP_PRIMARY: c_int = 3;
pub const DASD_COPYPAIRSWAP_SECONDARY: c_int = 4;
pub const DASD_COPYPAIRSWAP_MULTIPLE: c_int = 5;
//
// Reference count inliners
//
// The static memory in ccw_mem and erp_mem is managed by a sorted
// list of free memory chunks.
//
// Find out the left neighbour in chunk_list.
// Try to merge with right neighbour = next element from left.
// Try to merge with left neighbour.
//
// Check if bsize is in { 512, 1024, 2048, 4096 }
//
// return the callback data of the original request in case there are
// ERP requests build on top of it
//
// true when device is ese device and ft_bias selects the adaptive
// heuristic (neither hard endpoint)
//
// Linear interpolation of a heuristic parameter between its value at aggr==50
// (v50) and its value at aggr==100 (v100).
//
// Apply the ft_bias knob. For the hard endpoints just pin the mode; for the
// adaptive range derive the heuristic parameters from ft_bias and (re)start
// the FSM in ft1 so a freshly sparse device avoids the NRF penalty right away.
//
// externals in dasd.c
pub const DASD_PROFILE_OFF: c_int = 0;
pub const DASD_PROFILE_ON: c_int = 1;
pub const DASD_PROFILE_GLOBAL_ONLY: c_int = 2;
extern "C" {
    pub fn dasd_sfree_request(: *mut dasd_ccw_req, : *mut dasd_device);
}
extern "C" {
    pub fn dasd_ffree_request(: *mut dasd_ccw_req, : *mut dasd_device);
}
extern "C" {
    pub fn dasd_wakeup_cb(: *mut dasd_ccw_req, : *mut c_void);
}
extern "C" {
    pub fn dasd_free_device(: *mut dasd_device);
}
extern "C" {
    pub fn dasd_free_block(: *mut dasd_block);
}
extern "C" {
    pub fn dasd_times_out(req: *mut request) -> blk_eh_timer_return;
}
extern "C" {
    pub fn dasd_enable_device(: *mut dasd_device);
}
extern "C" {
    pub fn dasd_set_target_state(: *mut dasd_device, _arg: c_int);
}
extern "C" {
    pub fn dasd_kick_device(: *mut dasd_device);
}
extern "C" {
    pub fn dasd_reload_device(: *mut dasd_device);
}
extern "C" {
    pub fn dasd_schedule_requeue(: *mut dasd_device);
}
extern "C" {
    pub fn dasd_add_request_head(: *mut dasd_ccw_req);
}
extern "C" {
    pub fn dasd_add_request_tail(: *mut dasd_ccw_req);
}
extern "C" {
    pub fn dasd_start_IO(: *mut dasd_ccw_req) -> c_int;
}
extern "C" {
    pub fn dasd_term_IO(: *mut dasd_ccw_req) -> c_int;
}
extern "C" {
    pub fn dasd_schedule_device_bh(: *mut dasd_device);
}
extern "C" {
    pub fn dasd_schedule_block_bh(: *mut dasd_block);
}
extern "C" {
    pub fn dasd_sleep_on(: *mut dasd_ccw_req) -> c_int;
}
extern "C" {
    pub fn dasd_sleep_on_queue(: *mut list_head) -> c_int;
}
extern "C" {
    pub fn dasd_sleep_on_immediatly(: *mut dasd_ccw_req) -> c_int;
}
extern "C" {
    pub fn dasd_sleep_on_queue_interruptible(: *mut list_head) -> c_int;
}
extern "C" {
    pub fn dasd_sleep_on_interruptible(: *mut dasd_ccw_req) -> c_int;
}
extern "C" {
    pub fn dasd_device_set_timer(: *mut dasd_device, _arg: c_int);
}
extern "C" {
    pub fn dasd_device_clear_timer(: *mut dasd_device);
}
extern "C" {
    pub fn dasd_block_set_timer(: *mut dasd_block, _arg: c_int);
}
extern "C" {
    pub fn dasd_block_clear_timer(: *mut dasd_block);
}
extern "C" {
    pub fn dasd_cancel_req(: *mut dasd_ccw_req) -> c_int;
}
extern "C" {
    pub fn dasd_flush_device_queue(: *mut dasd_device) -> c_int;
}
extern "C" {
    pub fn dasd_generic_probe(: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn dasd_generic_free_discipline(: *mut dasd_device);
}
extern "C" {
    pub fn dasd_generic_remove(cdev: *mut ccw_device);
}
extern "C" {
    pub fn dasd_generic_set_online(: *mut ccw_device, : *mut dasd_discipline) -> c_int;
}
extern "C" {
    pub fn dasd_generic_set_offline(cdev: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn dasd_generic_notify(: *mut ccw_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn dasd_generic_last_path_gone(: *mut dasd_device) -> c_int;
}
extern "C" {
    pub fn dasd_generic_path_operational(: *mut dasd_device) -> c_int;
}
extern "C" {
    pub fn dasd_generic_shutdown(: *mut ccw_device);
}
extern "C" {
    pub fn dasd_generic_handle_state_change(: *mut dasd_device);
}
extern "C" {
    pub fn dasd_generic_uc_handler(: *mut ccw_device, : *mut irb) -> uc_todo;
}
extern "C" {
    pub fn dasd_generic_path_event(: *mut ccw_device, : *mut c_int);
}
extern "C" {
    pub fn dasd_generic_verify_path(: *mut dasd_device, _arg: __u8) -> c_int;
}
extern "C" {
    pub fn dasd_generic_space_exhaust(: *mut dasd_device, : *mut dasd_ccw_req);
}
extern "C" {
    pub fn dasd_generic_space_avail(: *mut dasd_device);
}
extern "C" {
    pub fn dasd_generic_requeue_all_requests(: *mut dasd_device) -> c_int;
}
extern "C" {
    pub fn dasd_generic_read_dev_chars(: *mut dasd_device, _arg: c_int, : *mut c_void, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn dasd_device_set_stop_bits(: *mut dasd_device, _arg: c_int);
}
extern "C" {
    pub fn dasd_device_remove_stop_bits(: *mut dasd_device, _arg: c_int);
}
extern "C" {
    pub fn dasd_device_is_ro(: *mut dasd_device) -> c_int;
}
extern "C" {
    pub fn dasd_profile_reset(: *mut dasd_profile);
}
extern "C" {
    pub fn dasd_profile_on(: *mut dasd_profile) -> c_int;
}
extern "C" {
    pub fn dasd_profile_off(: *mut dasd_profile);
}
// externals in dasd_devmap.c
extern "C" {
    pub fn dasd_devmap_init() -> c_int;
}
extern "C" {
    pub fn dasd_devmap_exit();
}
extern "C" {
    pub fn dasd_delete_device(: *mut dasd_device);
}
extern "C" {
    pub fn dasd_get_feature(: *mut ccw_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn dasd_set_feature(: *mut ccw_device, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn dasd_path_create_kobj(: *mut dasd_device, _arg: c_int);
}
extern "C" {
    pub fn dasd_path_create_kobjects(: *mut dasd_device);
}
extern "C" {
    pub fn dasd_path_remove_kobjects(: *mut dasd_device);
}
extern "C" {
    pub fn dasd_add_link_to_gendisk(: *mut gendisk, : *mut dasd_device);
}
extern "C" {
    pub fn dasd_busid_known(: *const c_char) -> c_int;
}
// externals in dasd_gendisk.c
extern "C" {
    pub fn dasd_gendisk_init() -> c_int;
}
extern "C" {
    pub fn dasd_gendisk_exit();
}
extern "C" {
    pub fn dasd_gendisk_alloc(: *mut dasd_block) -> c_int;
}
extern "C" {
    pub fn dasd_gendisk_free(: *mut dasd_block);
}
extern "C" {
    pub fn dasd_scan_partitions(: *mut dasd_block) -> c_int;
}
extern "C" {
    pub fn dasd_destroy_partitions(: *mut dasd_block);
}
// externals in dasd_ioctl.c
extern "C" {
    pub fn dasd_set_read_only(bdev: *mut block_device, ro: bool) -> c_int;
}
// externals in dasd_proc.c
extern "C" {
    pub fn dasd_proc_init() -> c_int;
}
extern "C" {
    pub fn dasd_proc_exit();
}
// externals in dasd_erp.c
extern "C" {
    pub fn dasd_free_erp_request(: *mut dasd_ccw_req, : *mut dasd_device);
}
extern "C" {
    pub fn dasd_log_sense(: *mut dasd_ccw_req, : *mut irb);
}
extern "C" {
    pub fn dasd_log_sense_dbf(cqr: *mut dasd_ccw_req, irb: *mut irb);
}
// externals in dasd_3990_erp.c
extern "C" {
    pub fn dasd_3990_erp_handle_sim(: *mut dasd_device, : *mut c_char);
}
// externals in dasd_eer.c

extern "C" {
    pub fn dasd_eer_init() -> c_int;
}
extern "C" {
    pub fn dasd_eer_exit();
}
extern "C" {
    pub fn dasd_eer_enable(: *mut dasd_device) -> c_int;
}
extern "C" {
    pub fn dasd_eer_disable(: *mut dasd_device);
}
extern "C" {
    pub fn dasd_eer_snss(: *mut dasd_device);
}

// DASD path handling functions
//
// helper functions to modify bit masks for a given channel path for a device
//
extern "C" {
    pub fn test_bit(_arg: DASD_PATH_OPERATIONAL, _arg: &device->path[chp].flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: DASD_PATH_TBV, _arg: &device->path[chp].flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: DASD_PATH_FCSEC, _arg: &device->path[chp].flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: DASD_PATH_NPP, _arg: &device->path[chp].flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: DASD_PATH_PP, _arg: &device->path[chp].flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: DASD_PATH_CUIR, _arg: &device->path[chp].flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: DASD_PATH_IFCC, _arg: &device->path[chp].flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: DASD_PATH_MISCABLED, _arg: &device->path[chp].flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: DASD_PATH_NOHPF, _arg: &device->path[chp].flags) -> return;
}
//
// get functions for path masks
// will return a path masks for the given device
//
// add functions for path masks
// the existing path mask will be extended by the given path mask
//
// if the path is used
// it should not be in one of the negative lists
//
// set functions for path masks
// the existing path mask will be replaced by the given path mask
//
// if the path is used
// it should not be in one of the negative lists
//
// remove functions for path masks
// the existing path mask will be cleared with the given path mask
//
// add the newly available path to the to be verified pm and remove it from
// normal operation until it is verified
//
// remove all paths from normal operation
//
// end - path handling
