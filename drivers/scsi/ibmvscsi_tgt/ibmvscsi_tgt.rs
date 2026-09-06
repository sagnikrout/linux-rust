//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/ibmvscsi_tgt/ibmvscsi_tgt.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// IBM Virtual SCSI Target Driver
// Copyright (C) 2003-2005 Dave Boutcher (boutcher@us.ibm.com) IBM Corp.
// Santiago Leon (santil@us.ibm.com) IBM Corp.
// Linda Xie (lxie@us.ibm.com) IBM Corp.
//
// Copyright (C) 2005-2011 FUJITA Tomonori <tomof@acm.org>
// Copyright (C) 2010 Nicholas A. Bellinger <nab@kernel.org>
// Copyright (C) 2016 Bryant G. Ly <bryantly@linux.vnet.ibm.com> IBM Corp.
//
// Authors: Bryant G. Ly <bryantly@linux.vnet.ibm.com>
// Authors: Michael Cyr <mikecyr@linux.vnet.ibm.com>
//

pub const SYS_ID_NAME_LEN: c_int = 64;
pub const PARTITION_NAMELEN: c_int = 96;
pub const IBMVSCSIS_NAMELEN: c_int = 32;
pub const MSG_HI: c_int = 0;
pub const MSG_LOW: c_int = 1;
pub const MAX_CMD_Q_PAGES: c_int = 4;

// in terms of number of elements

pub const SRP_VIOLATION: c_uint = 0x102  /* general error code */;
//
// SRP buffer formats defined as of 16.a supported by this driver.
//

pub const SCSI_LUN_ADDR_METHOD_FLAT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_window {
    pub /: *mut *mut u32 liobn; / Unique per vdevice,
    pub /: *mut *mut u64 tce_base; / Physical location of the TCE table,
    pub /: *mut *mut u64 tce_size; / Size of the TCE table in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_dds {
    pub /: *mut *mut u64 unit_id; / 64 bit will force alignment,
pub const NUM_DMA_WINDOWS: c_int = 2;
pub const LOCAL: c_int = 0;
pub const REMOTE: c_int = 1;
    pub window: [dma_window; NUM_DMA_WINDOWS],
// root node property "ibm,partition-no"
    pub partition_num: c_uint,
    pub partition_name: [c_char; PARTITION_NAMELEN],
}

pub const MAX_NUM_PORTS: c_int = 1;

pub const MAX_EYE: c_int = 64;
// Return codes

// choose error codes that do not conflict with PHYP

#[repr(C)]
#[derive(Copy, Clone)]
pub struct format_code {
    pub reserved: u8,
    pub buffers: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct client_info {
    pub srp_version: [c_char; 8],
// root node property ibm,partition-name
    pub partition_name: [c_char; PARTITION_NAMELEN],
// root node property ibm,partition-no
    pub partition_number: u32,
// initially 1
    pub mad_version: u32,
    pub os_type: u32,
}

//
// Changing this constant changes the number of seconds to wait before
// considering the client will never service its queue again.
//
pub const SECONDS_TO_CONSIDER_FAILED: c_int = 30;
//
// These constants set the polling period used to determine if the client
// has freed at least one element in the response queue.
//
pub const WAIT_SECONDS: c_int = 1;
pub const WAIT_NANO_SECONDS: c_int = 5000;

//
// general purpose timer control block
// which can be used for multiple functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer_cb {
    pub timer: hrtimer,
//
// how long has it been since the client
// serviced the queue. The variable is incrmented
// in the service_wait_q routine and cleared
// in send messages
//
    pub timer_pops: c_int,
// the timer is started
    pub started: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_queue {
// kva
    pub base_addr: *mut viosrp_crq,
    pub crq_token: dma_addr_t,
// used to maintain index
    pub mask: c_uint,
// current element
    pub index: c_uint,
    pub size: c_int,
}

pub const SCSOLNT_RESP_SHIFT: c_int = 1;
pub const UCSOLNT_RESP_SHIFT: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_type {
    SCSI_CDB	= 0x01,
    TASK_MANAGEMENT	= 0x02,
// MAD or addressed to port 0
    ADAPTER_MAD	= 0x04,
    UNSET_TYPE	= 0x08,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iu_rsp {
    pub format: u8,
    pub sol_not: u8,
    pub len: u16,
// tag is just to help client identify cmd, so don't translate be/le
    pub tag: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvscsis_cmd {
    pub list: list_head,
// Used for TCM Core operations
    pub se_cmd: se_cmd,
    pub iue: *mut iu_entry,
    pub rsp: iu_rsp,
    pub work: work_struct,
    pub adapter: *mut scsi_info,
    pub abort_cmd: *mut ibmvscsis_cmd,
// Sense buffer that will be mapped into outgoing status
    pub sense_buf: [c_uchar; TRANSPORT_SENSE_BUFFER],
    pub init_time: u64,

    pub flags: u32,
    pub type: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvscsis_nexus {
    pub se_sess: *mut se_session,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvscsis_tport {
// SCSI protocol the tport is providing
    pub tport_proto_id: u8,
// ASCII formatted WWPN for SRP Target port
    pub tport_name: [c_char; IBMVSCSIS_NAMELEN],
// Returned by ibmvscsis_make_tport()
    pub tport_wwn: se_wwn,
// Returned by ibmvscsis_make_tpg()
    pub se_tpg: se_portal_group,
// ibmvscsis port target portal group tag for TCM
    pub tport_tpgt: u16,
// Pointer to TCM session for I_T Nexus
    pub ibmv_nexus: *mut ibmvscsis_nexus,
    pub enabled: bool,
    pub releasing: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_info {
    pub list: list_head,
    pub eye: [c_char; MAX_EYE],
// commands waiting for space on repsonse queue
    pub waiting_rsp: list_head,
pub const NO_QUEUE: c_uint = 0x00;

pub const WAIT_CONNECTION: c_uint = 0x04;
// have established a connection
pub const CONNECTED: c_uint = 0x08;
// at least one port is processing SRP IU
pub const SRP_PROCESSING: c_uint = 0x10;
// remove request received
pub const UNCONFIGURING: c_uint = 0x20;
// disconnect by letting adapter go idle, no error
pub const WAIT_IDLE: c_uint = 0x40;
// disconnecting to clear an error
pub const ERR_DISCONNECT: c_uint = 0x80;
// disconnect to clear error state, then come back up
pub const ERR_DISCONNECT_RECONNECT: c_uint = 0x100;
// disconnected after clearing an error
pub const ERR_DISCONNECTED: c_uint = 0x200;
// A series of errors caused unexpected errors
pub const UNDEFINED: c_uint = 0x400;
    pub state: u16,
    pub fast_fail: c_int,
    pub dds: target_dds,
    pub cmd_pool: *mut c_char,
// list of free commands
    pub free_cmd: list_head,
// command elements ready for scheduler
    pub schedule_q: list_head,
// commands sent to TCM
    pub active_q: list_head,
    pub map_buf: *mut caddr_t,
// ioba of map buffer
    pub map_ioba: dma_addr_t,
// allowable number of outstanding SRP requests
    pub request_limit: c_int,
// extra credit
    pub credit: c_int,
// outstanding transactions against credit limit
    pub debit: c_int,
// allow only one outstanding mad request
pub const PROCESSING_MAD: c_uint = 0x00002;
// Waiting to go idle
pub const WAIT_FOR_IDLE: c_uint = 0x00004;
// H_REG_CRQ called
pub const CRQ_CLOSED: c_uint = 0x00010;
// detected that client has failed
pub const CLIENT_FAILED: c_uint = 0x00040;
// detected that transport event occurred
pub const TRANS_EVENT: c_uint = 0x00080;
// don't attempt to send anything to the client
pub const RESPONSE_Q_DOWN: c_uint = 0x00100;
// request made to schedule disconnect handler
pub const SCHEDULE_DISCONNECT: c_uint = 0x00400;
// disconnect handler is scheduled
pub const DISCONNECT_SCHEDULED: c_uint = 0x00800;
// remove function is sleeping
pub const CFG_SLEEPING: c_uint = 0x01000;
// Register for Prepare for Suspend Transport Events
pub const PREP_FOR_SUSPEND_ENABLED: c_uint = 0x02000;
// Prepare for Suspend event sent
pub const PREP_FOR_SUSPEND_PENDING: c_uint = 0x04000;
// Resume from Suspend event sent
pub const PREP_FOR_SUSPEND_ABORTED: c_uint = 0x08000;
// Prepare for Suspend event overwrote another CRQ entry
pub const PREP_FOR_SUSPEND_OVERWRITE: c_uint = 0x10000;
    pub flags: u32,
// adapter lock
    pub intr_lock: spinlock_t,
// information needed to manage command queue
    pub cmd_q: cmd_queue,
// used in hcall to copy response back into srp buffer
    pub empty_iu_id: u64,
// used in crq, to tag what iu the response is for
    pub empty_iu_tag: u64,
    pub new_state: c_uint,
    pub resume_state: c_uint,
// control block for the response queue timer
    pub rsp_q_timer: timer_cb,
// keep last client to enable proper accounting
    pub client_data: client_info,
// what can this client do
    pub client_cap: u32,
//
// The following two fields capture state and flag changes that
// can occur when the lock is given up.  In the orginal design,
// the lock was held during calls into phyp;
// however, phyp did not meet PAPR architecture.  This is
// a work around.
//
    pub phyp_acr_state: u16,
    pub phyp_acr_flags: u32,
    pub work_q: *mut workqueue_struct,
    pub wait_idle: completion,
    pub unconfig: completion,
    pub dev: device,
    pub dma_dev: *mut vio_dev,
    pub target: srp_target,
    pub tport: ibmvscsis_tport,
    pub work_task: tasklet_struct,
    pub proc_work: work_struct,
}

//
// Provide a constant that allows software to detect the adapter is
// disconnecting from the client from one of several states.
//

//
// Provide a constant that can be used with interrupt handling that
// essentially lets the interrupt handler know that all requests should
// be thrown out,
//

//
// If any of these flag bits are set then do not allow the interrupt
// handler to schedule the off level handler.
//

// State and transition events that stop the interrupt handler

// flag bit that are not reset during disconnect

pub const H_GET_PARTNER_INFO: c_uint = 0x0000000000000008LL;

pub const H_ENABLE_PREPARE_FOR_SUSPEND: c_uint = 0x000000000000001DLL;

pub const H_READY_FOR_SUSPEND: c_uint = 0x000000000000001ELL;

