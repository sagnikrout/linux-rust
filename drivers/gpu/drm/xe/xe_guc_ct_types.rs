//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_ct_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//

//
// struct guc_ctb_info - GuC command transport buffer (CTB) info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_ctb_info {
// @size: size of CTB commands (DW)
    pub size: u32,
// @resv_space: reserved space of CTB commands (DW)
    pub resv_space: u32,
// @head: head of CTB commands (DW)
    pub head: u32,
// @tail: tail of CTB commands (DW)
    pub tail: u32,
// @space: space in CTB commands (DW)
    pub space: u32,
// @broken: channel broken
    pub broken: bool,
}

//
// struct guc_ctb - GuC command transport buffer (CTB)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_ctb {
// @bo: Xe BO for CTB
    pub bo: *mut xe_bo,
// @desc: dma buffer map for CTB descriptor
    pub desc: iosys_map,
// @cmds: dma buffer map for CTB commands
    pub cmds: iosys_map,
// @info: CTB info
    pub info: guc_ctb_info,
}

//
// struct guc_ctb_snapshot - GuC command transport buffer (CTB) snapshot
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_ctb_snapshot {
// @desc: snapshot of the CTB descriptor
    pub desc: guc_ct_buffer_desc,
// @info: snapshot of the CTB info
    pub info: guc_ctb_info,
}

//
// struct xe_guc_ct_snapshot - GuC command transport (CT) snapshot
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_ct_snapshot {
// @ct_enabled: CT enabled info at capture time.
    pub ct_enabled: bool,
// @g2h_outstanding: G2H outstanding info at the capture time
    pub g2h_outstanding: u32,
// @g2h: G2H CTB snapshot
    pub g2h: guc_ctb_snapshot,
// @h2g: H2G CTB snapshot
    pub h2g: guc_ctb_snapshot,
// @ctb_size: size of the snapshot of the CTB
    pub ctb_size: usize,
// @ctb: snapshot of the entire CTB
    pub ctb: *mut c_void,
}

//
// enum xe_guc_ct_state - CT state
// @XE_GUC_CT_STATE_NOT_INITIALIZED: CT not initialized, messages not expected in this state
// @XE_GUC_CT_STATE_DISABLED: CT disabled, messages not expected in this state
// @XE_GUC_CT_STATE_STOPPED: CT stopped, drop messages without errors
// @XE_GUC_CT_STATE_ENABLED: CT enabled, messages sent / received in this state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_guc_ct_state {
    XE_GUC_CT_STATE_NOT_INITIALIZED = 0,
    XE_GUC_CT_STATE_DISABLED,
    XE_GUC_CT_STATE_STOPPED,
    XE_GUC_CT_STATE_ENABLED,
}

// struct xe_dead_ct - Information for debugging a dead CT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_dead_ct {
// @lock: protects memory allocation/free operations, and @reason updates
    pub lock: spinlock_t,
// @reason: bit mask of CT_DEAD_* reason codes
    pub reason: c_uint,
// @reported: for preventing multiple dumps per error sequence
    pub reported: bool,
// @worker: worker thread to get out of interrupt context before dumping
    pub worker: work_struct,
// @snapshot_ct: copy of CT state and CTB content at point of error
    pub snapshot_ct: *mut xe_guc_ct_snapshot,
// @snapshot_log: copy of GuC log at point of error
    pub snapshot_log: *mut xe_guc_log_snapshot,
}

// struct xe_fast_req_fence - Used to track FAST_REQ messages by fence to match error responses
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_fast_req_fence {
// @fence: sequence number sent in H2G and return in G2H error
    pub fence: u16,
// @action: H2G action code
    pub action: u16,

// @stack: call stack from when the H2G was sent
    pub stack: depot_stack_handle_t,

}

//
// struct xe_guc_ct - GuC command transport (CT) layer
//
// Includes a pair of CT buffers for bi-directional communication and tracking
// for the H2G and G2H requests sent and received through the buffers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_ct {
// @lock: protects everything in CT layer
    pub lock: mutex,
// @fast_lock: protects G2H channel and credits
    pub fast_lock: spinlock_t,
// @ctbs: buffers for sending and receiving commands
// @ctbs.h2g: Host to GuC (H2G, send) channel
    pub h2g: guc_ctb,
// @ctbs.g2h: GuC to Host (G2H, receive) channel
    pub g2h: guc_ctb,
    pub ctbs: },
// @g2h_outstanding: number of outstanding G2H
    pub g2h_outstanding: u32,
// @g2h_worker: worker to process G2H messages
    pub g2h_worker: work_struct,
// @safe_mode_worker: worker to check G2H messages with IRQ disabled
    pub safe_mode_worker: delayed_work,
// @state: CT state
    pub state: xe_guc_ct_state,
// @fence_seqno: G2H fence seqno - 16 bits used by CT
    pub fence_seqno: u32,
// @fence_lookup: G2H fence lookup
    pub fence_lookup: xarray,
// @wq: wait queue used for reliable CT sends and freeing G2H credits
    pub wq: wait_queue_head_t,
// @g2h_fence_wq: wait queue used for G2H fencing
    pub g2h_fence_wq: wait_queue_head_t,
// @g2h_wq: used to process G2H
    pub g2h_wq: *mut workqueue_struct,
// @msg: Message buffer
    pub msg: [u32; GUC_CTB_MSG_MAX_LEN],
// @fast_msg: Message buffer
    pub fast_msg: [u32; GUC_CTB_MSG_MAX_LEN],
// @dead: information for debugging dead CTs
    pub dead: xe_dead_ct,
// @fast_req: history of FAST_REQ messages for matching with G2H error responses
    pub fast_req: [xe_fast_req_fence; SZ_32],
}
