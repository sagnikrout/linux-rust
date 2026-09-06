//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/ibmvmc.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// linux/drivers/misc/ibmvmc.h
//
// IBM Power Systems Virtual Management Channel Support.
//
// Copyright (c) 2004, 2018 IBM Corp.
// Dave Engebretsen engebret@us.ibm.com
// Steven Royer seroyer@linux.vnet.ibm.com
// Adam Reznechek adreznec@linux.vnet.ibm.com
// Bryant G. Ly <bryantly@linux.vnet.ibm.com>
//

pub const IBMVMC_PROTOCOL_VERSION: c_uint = 0x0101;
pub const MIN_BUF_POOL_SIZE: c_int = 16;
pub const MIN_HMCS: c_int = 1;
pub const MIN_MTU: c_int = 4096;
pub const MAX_BUF_POOL_SIZE: c_int = 64;
pub const MAX_HMCS: c_int = 2;

pub const DEFAULT_BUF_POOL_SIZE: c_int = 32;
pub const DEFAULT_HMCS: c_int = 1;
pub const DEFAULT_MTU: c_int = 4096;
pub const HMC_ID_LEN: c_int = 32;
pub const VMC_INVALID_BUFFER_ID: c_uint = 0xFFFF;
// ioctl numbers
pub const VMC_BASE: c_uint = 0xCC;

pub const VMC_MSG_CAP: c_uint = 0x01;
pub const VMC_MSG_CAP_RESP: c_uint = 0x81;
pub const VMC_MSG_OPEN: c_uint = 0x02;
pub const VMC_MSG_OPEN_RESP: c_uint = 0x82;
pub const VMC_MSG_CLOSE: c_uint = 0x03;
pub const VMC_MSG_CLOSE_RESP: c_uint = 0x83;
pub const VMC_MSG_ADD_BUF: c_uint = 0x04;
pub const VMC_MSG_ADD_BUF_RESP: c_uint = 0x84;
pub const VMC_MSG_REM_BUF: c_uint = 0x05;
pub const VMC_MSG_REM_BUF_RESP: c_uint = 0x85;
pub const VMC_MSG_SIGNAL: c_uint = 0x06;
pub const VMC_MSG_SUCCESS: c_int = 0;
pub const VMC_MSG_INVALID_HMC_INDEX: c_int = 1;
pub const VMC_MSG_INVALID_BUFFER_ID: c_int = 2;
pub const VMC_MSG_CLOSED_HMC: c_int = 3;
pub const VMC_MSG_INTERFACE_FAILURE: c_int = 4;
pub const VMC_MSG_NO_BUFFER: c_int = 5;
pub const VMC_BUF_OWNER_ALPHA: c_int = 0;
pub const VMC_BUF_OWNER_HV: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvmc_states {
    ibmvmc_state_sched_reset  = -1,
    ibmvmc_state_initial      = 0,
    ibmvmc_state_crqinit      = 1,
    ibmvmc_state_capabilities = 2,
    ibmvmc_state_ready        = 3,
    ibmvmc_state_failed       = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmhmc_states {
// HMC connection not established
    ibmhmc_state_free    = 0,

// HMC connection established (open called)
    ibmhmc_state_initial = 1,

// open msg sent to HV, due to ioctl(1) call
    ibmhmc_state_opening = 2,

// HMC connection ready, open resp msg from HV
    ibmhmc_state_ready   = 3,

// HMC connection failure
    ibmhmc_state_failed  = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvmc_buffer {
    pub /: *mut *mut u8 valid; / 1 when DMA storage allocated to buffer,
    pub /: *mut *mut u8 free; / 1 when buffer available for the Alpha Partition,
    pub owner: u8,
    pub id: u16,
    pub size: u32,
    pub msg_len: u32,
    pub dma_addr_local: dma_addr_t,
    pub dma_addr_remote: dma_addr_t,
    pub real_addr_local: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvmc_admin_crq_msg {
    pub /: *mut *mut u8 valid; / RPA Defined,
    pub /: *mut *mut u8 type; / ibmvmc msg type,
    pub failure,: *mut *mut u8 status; / Response msg status. Zero is success and on,
// either 1 - General Failure, or 2 - Invalid Version is
// returned.
//
    pub rsvd: [u8; 2],
    pub /: *mut *mut u8 max_hmc; / Max # of independent HMC connections supported,
    pub HMC: *mut *mut __be16 pool_size; / Maximum number of buffers supported per,
// connection
//
    pub /: *mut *mut __be32 max_mtu; / Maximum message size supported (bytes),
    pub the: *mut *mut __be16 crq_size; / # of entries available in the CRQ for,
// source partition. The target partition must
// limit the number of outstanding messages to
// one half or less.
//
    pub partition: *mut *mut __be16 version; / Indicates the code level of the management,
// or the hypervisor with the high-order byte
// indicating a major version and the low-order byte
// indicating a minor version.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvmc_crq_msg {
    pub /: *mut *mut u8 valid; / RPA Defined,
    pub /: *mut *mut u8 type; / ibmvmc msg type,
    pub /: *mut *mut u8 status; / Response msg status,
    pub /: *mut *mut u8 rsvd; / Reserved,
    pub owner: u8,
    pub var1: },
    pub /: *mut *mut u8 hmc_session; / Session Identifier for the current VMC connection,
    pub management: *mut *mut u8 hmc_index; / A unique HMC Idx would be used if multiple,
// applications running concurrently were desired
//
    pub rsvd: __be16,
    pub buffer_id: __be16,
    pub var2: },
    pub rsvd: __be32,
    pub rsvd: __be32,
    pub lioba: __be32,
    pub msg_len: __be32,
    pub var3: },
}

// an RPA command/response transport queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crq_queue {
    pub msgs: *mut ibmvmc_crq_msg,
    pub cur: int size,,
    pub msg_token: dma_addr_t,
    pub lock: spinlock_t,
}

// VMC server adapter settings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crq_server_adapter {
    pub dev: *mut device,
    pub queue: crq_queue,
    pub liobn: u32,
    pub riobn: u32,
    pub work_task: tasklet_struct,
    pub reset_wait_queue: wait_queue_head_t,
    pub reset_task: *mut task_struct,
}

// Driver wide settings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvmc_struct {
    pub state: u32,
    pub max_mtu: u32,
    pub max_buffer_pool_size: u32,
    pub max_hmc_index: u32,
    pub adapter: *mut crq_server_adapter,
    pub cdev: cdev,
    pub vmc_drc_index: u32,
}

// Connection specific settings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvmc_hmc {
    pub session: u8,
    pub index: u8,
    pub state: u32,
    pub adapter: *mut crq_server_adapter,
    pub lock: spinlock_t,
    pub hmc_id: [c_uchar; HMC_ID_LEN],
    pub buffer: [ibmvmc_buffer; MAX_BUF_POOL_SIZE],
    pub queue_outbound_msgs: [c_ushort; MAX_BUF_POOL_SIZE],
    pub queue_tail: int queue_head,,
    pub file_session: *mut ibmvmc_file_session,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvmc_file_session {
    pub file: *mut file,
    pub hmc: *mut ibmvmc_hmc,
    pub valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvmc_query_struct {
    pub have_vmc: c_int,
    pub state: c_int,
    pub vmc_drc_index: c_int,
}
