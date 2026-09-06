//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/fsl/dpio/qbman-portal.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright (C) 2014-2016 Freescale Semiconductor, Inc.
// Copyright 2016-2019 NXP
//

pub const QMAN_REV_4000: c_uint = 0x04000000;
pub const QMAN_REV_4100: c_uint = 0x04010000;
pub const QMAN_REV_4101: c_uint = 0x04010001;
pub const QMAN_REV_5000: c_uint = 0x05000000;
pub const QMAN_REV_MASK: c_uint = 0xffff0000;
// qbman software portal descriptor structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qbman_swp_desc {
    pub /: *mut *mut *mut void cena_bar; / Cache-enabled portal base address,
    pub /: *mut *mut *mut void __iomem cinh_bar; / Cache-inhibited portal base address,
    pub qman_version: u32,
    pub qman_clk: u32,
    pub qman_256_cycles_per_ns: u32,
}

pub const QBMAN_SWP_INTERRUPT_EQRI: c_uint = 0x01;
pub const QBMAN_SWP_INTERRUPT_EQDI: c_uint = 0x02;
pub const QBMAN_SWP_INTERRUPT_DQRI: c_uint = 0x04;
pub const QBMAN_SWP_INTERRUPT_RCRI: c_uint = 0x08;
pub const QBMAN_SWP_INTERRUPT_RCDI: c_uint = 0x10;
pub const QBMAN_SWP_INTERRUPT_VDCI: c_uint = 0x20;
// the structure for pull dequeue descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qbman_pull_desc {
    pub verb: u8,
    pub numf: u8,
    pub tok: u8,
    pub reserved: u8,
    pub dq_src: __le32,
    pub rsp_addr: __le64,
    pub rsp_addr_virt: u64,
    pub padding: [u8; 40],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qbman_pull_type_e {
// dequeue with priority precedence, respect intra-class scheduling
    qbman_pull_type_prio = 1,
// dequeue with active FQ precedence, respect ICS
    qbman_pull_type_active,
// dequeue with active FQ precedence, no ICS
    qbman_pull_type_active_noics
}

// Definitions for parsing dequeue entries
pub const QBMAN_RESULT_MASK: c_uint = 0x7f;
pub const QBMAN_RESULT_DQ: c_uint = 0x60;
pub const QBMAN_RESULT_FQRN: c_uint = 0x21;
pub const QBMAN_RESULT_FQRNI: c_uint = 0x22;
pub const QBMAN_RESULT_FQPN: c_uint = 0x24;
pub const QBMAN_RESULT_FQDAN: c_uint = 0x25;
pub const QBMAN_RESULT_CDAN: c_uint = 0x26;
pub const QBMAN_RESULT_CSCN_MEM: c_uint = 0x27;
pub const QBMAN_RESULT_CGCU: c_uint = 0x28;
pub const QBMAN_RESULT_BPSCN: c_uint = 0x29;
pub const QBMAN_RESULT_CSCN_WQ: c_uint = 0x2a;
// QBMan FQ management command codes
pub const QBMAN_FQ_SCHEDULE: c_uint = 0x48;
pub const QBMAN_FQ_FORCE: c_uint = 0x49;
pub const QBMAN_FQ_XON: c_uint = 0x4d;
pub const QBMAN_FQ_XOFF: c_uint = 0x4e;
// structure of enqueue descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qbman_eq_desc {
    pub verb: u8,
    pub dca: u8,
    pub seqnum: __le16,
    pub orpid: __le16,
    pub reserved1: __le16,
    pub tgtid: __le32,
    pub tag: __le32,
    pub qdbin: __le16,
    pub qpri: u8,
    pub reserved: [u8; 3],
    pub wae: u8,
    pub rspid: u8,
    pub rsp_addr: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qbman_eq_desc_with_fd {
    pub desc: qbman_eq_desc,
    pub fd: [u8; 32],
}

// buffer release descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qbman_release_desc {
    pub verb: u8,
    pub reserved: u8,
    pub bpid: __le16,
    pub reserved2: __le32,
    pub buf: [__le64; 7],
}

// Management command result codes
pub const QBMAN_MC_RSLT_OK: c_uint = 0xf0;
pub const CODE_CDAN_WE_EN: c_uint = 0x1;
pub const CODE_CDAN_WE_CTX: c_uint = 0x4;
// portal data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qbman_swp {
    pub desc: *const qbman_swp_desc,
    pub addr_cena: *mut c_void,
    pub addr_cinh: *mut void __iomem,
// Management commands
    pub /: *mut *mut u32 valid_bit; / 0x00 or 0x80,
    pub mc: },
// Management response
    pub /: *mut *mut u32 valid_bit; / 0x00 or 0x80,
    pub mr: },
// Push dequeues
    pub sdq: u32,
// Volatile dequeues
    pub /: *mut *mut atomic_t available; / indicates if a command can be sent,
    pub /: *mut *mut u32 valid_bit; / 0x00 or 0x80,
    pub /: *mut *mut *mut dpaa2_dq storage; / NULL if DQRR,
    pub vdq: },
// DQRR
    pub next_idx: u32,
    pub valid_bit: u32,
    pub dqrr_size: u8,
    pub /: *mut *mut int reset_bug; / indicates dqrr reset workaround is needed,
    pub dqrr: },
    pub pi: u32,
    pub pi_vb: u32,
    pub pi_ring_size: u32,
    pub pi_ci_mask: u32,
    pub ci: u32,
    pub available: c_int,
    pub pend: u32,
    pub no_pfdr: u32,
    pub eqcr: },
    pub access_spinlock: spinlock_t,
// Interrupt coalescing
    pub irq_threshold: u32,
    pub irq_holdoff: u32,
    pub use_adaptive_rx_coalesce: c_int,
}

// Function pointers
// Functions
extern "C" {
    pub fn qbman_swp_finish(p: *mut qbman_swp);
}
extern "C" {
    pub fn qbman_swp_interrupt_read_status(p: *mut qbman_swp) -> u32;
}
extern "C" {
    pub fn qbman_swp_interrupt_clear_status(p: *mut qbman_swp, mask: u32);
}
extern "C" {
    pub fn qbman_swp_interrupt_get_trigger(p: *mut qbman_swp) -> u32;
}
extern "C" {
    pub fn qbman_swp_interrupt_set_trigger(p: *mut qbman_swp, mask: u32);
}
extern "C" {
    pub fn qbman_swp_interrupt_get_inhibit(p: *mut qbman_swp) -> c_int;
}
extern "C" {
    pub fn qbman_swp_interrupt_set_inhibit(p: *mut qbman_swp, inhibit: c_int);
}
extern "C" {
    pub fn qbman_swp_push_get(p: *mut qbman_swp, channel_idx: u8, enabled: *mut c_int);
}
extern "C" {
    pub fn qbman_swp_push_set(p: *mut qbman_swp, channel_idx: u8, enable: c_int);
}
extern "C" {
    pub fn qbman_pull_desc_clear(d: *mut qbman_pull_desc);
}
extern "C" {
    pub fn qbman_pull_desc_set_numframes(d: *mut qbman_pull_desc, numframes: u8);
}
extern "C" {
    pub fn qbman_pull_desc_set_fq(d: *mut qbman_pull_desc, fqid: u32);
}
extern "C" {
    pub fn qbman_swp_dqrr_consume(s: *mut qbman_swp, dq: *const dpaa2_dq);
}
extern "C" {
    pub fn qbman_result_has_new_result(p: *mut qbman_swp, dq: *const dpaa2_dq) -> c_int;
}
extern "C" {
    pub fn qbman_eq_desc_clear(d: *mut qbman_eq_desc);
}
extern "C" {
    pub fn qbman_eq_desc_set_no_orp(d: *mut qbman_eq_desc, respond_success: c_int);
}
extern "C" {
    pub fn qbman_eq_desc_set_token(d: *mut qbman_eq_desc, token: u8);
}
extern "C" {
    pub fn qbman_eq_desc_set_fq(d: *mut qbman_eq_desc, fqid: u32);
}
extern "C" {
    pub fn qbman_release_desc_clear(d: *mut qbman_release_desc);
}
extern "C" {
    pub fn qbman_release_desc_set_bpid(d: *mut qbman_release_desc, bpid: u16);
}
extern "C" {
    pub fn qbman_release_desc_set_rcdi(d: *mut qbman_release_desc, enable: c_int);
}
extern "C" {
    pub fn qbman_swp_mc_submit(p: *mut qbman_swp, cmd: *mut c_void, cmd_verb: u8);
}
//
// qbman_swp_enqueue() - Issue an enqueue command
// @s:  the software portal used for enqueue
// @d:  the enqueue descriptor
// @fd: the frame descriptor to be enqueued
//
// Return 0 for successful enqueue, -EBUSY if the EQCR is not ready.
//
extern "C" {
    pub fn qbman_swp_enqueue_ptr(_arg: s, _arg: d, _arg: fd) -> return;
}
//
// qbman_swp_enqueue_multiple() - Issue a multi enqueue command
// using one enqueue descriptor
// @s:  the software portal used for enqueue
// @d:  the enqueue descriptor
// @fd: table pointer of frame descriptor table to be enqueued
// @flags: table pointer of QBMAN_ENQUEUE_FLAG_DCA flags, not used if NULL
// @num_frames: number of fd to be enqueued
//
// Return the number of fd enqueued, or a negative error number.
//
extern "C" {
    pub fn qbman_swp_enqueue_multiple_ptr(_arg: s, _arg: d, _arg: fd, _arg: flags, _arg: num_frames) -> return;
}
//
// qbman_swp_enqueue_multiple_desc() - Issue a multi enqueue command
// using multiple enqueue descriptor
// @s:  the software portal used for enqueue
// @d:  table of minimal enqueue descriptor
// @fd: table pointer of frame descriptor table to be enqueued
// @num_frames: number of fd to be enqueued
//
// Return the number of fd enqueued, or a negative error number.
//
extern "C" {
    pub fn qbman_swp_enqueue_multiple_desc_ptr(_arg: s, _arg: d, _arg: fd, _arg: num_frames) -> return;
}
//
// qbman_result_is_DQ() - check if the dequeue result is a dequeue response
// @dq: the dequeue result to be checked
//
// DQRR entries may contain non-dequeue results, ie. notifications
//
// qbman_result_is_SCN() - Check the dequeue result is notification or not
// @dq: the dequeue result to be checked
//
// FQ Data Availability
// Channel Data Availability
// Congestion State Change
// Buffer Pool State Change
// Congestion Group Count Update
// Retirement
// Retirement Immediate
// Park
//
// qbman_result_SCN_state() - Get the state field in State-change notification
//
pub const SCN_RID_MASK: c_uint = 0x00FFFFFF;
//
// qbman_result_SCN_rid() - Get the resource id in State-change notification
//
// qbman_result_SCN_ctx() - Get the context data in State-change notification
//
extern "C" {
    pub fn le64_to_cpu(_arg: scn->scn.ctx) -> return;
}
//
// qbman_swp_fq_schedule() - Move the fq to the scheduled state
// @s:    the software portal object
// @fqid: the index of frame queue to be scheduled
//
// There are a couple of different ways that a FQ can end up parked state,
// This schedules it.
//
// Return 0 for success, or negative error code for failure.
//
extern "C" {
    pub fn qbman_swp_alt_fq_state(_arg: s, _arg: fqid, _arg: QBMAN_FQ_SCHEDULE) -> return;
}
//
// qbman_swp_fq_force() - Force the FQ to fully scheduled state
// @s:    the software portal object
// @fqid: the index of frame queue to be forced
//
// Force eligible will force a tentatively-scheduled FQ to be fully-scheduled
// and thus be available for selection by any channel-dequeuing behaviour (push
// or pull). If the FQ is subsequently "dequeued" from the channel and is still
// empty at the time this happens, the resulting dq_entry will have no FD.
// (qbman_result_DQ_fd() will return NULL.)
//
// Return 0 for success, or negative error code for failure.
//
extern "C" {
    pub fn qbman_swp_alt_fq_state(_arg: s, _arg: fqid, _arg: QBMAN_FQ_FORCE) -> return;
}
//
// qbman_swp_fq_xon() - sets FQ flow-control to XON
// @s:    the software portal object
// @fqid: the index of frame queue
//
// This setting doesn't affect enqueues to the FQ, just dequeues.
//
// Return 0 for success, or negative error code for failure.
//
extern "C" {
    pub fn qbman_swp_alt_fq_state(_arg: s, _arg: fqid, _arg: QBMAN_FQ_XON) -> return;
}
//
// qbman_swp_fq_xoff() - sets FQ flow-control to XOFF
// @s:    the software portal object
// @fqid: the index of frame queue
//
// This setting doesn't affect enqueues to the FQ, just dequeues.
// XOFF FQs will remain in the tenatively-scheduled state, even when
// non-empty, meaning they won't be selected for scheduled dequeuing.
// If a FQ is changed to XOFF after it had already become truly-scheduled
// to a channel, and a pull dequeue of that channel occurs that selects
// that FQ for dequeuing, then the resulting dq_entry will have no FD.
// (qbman_result_DQ_fd() will return NULL.)
//
// Return 0 for success, or negative error code for failure.
//
extern "C" {
    pub fn qbman_swp_alt_fq_state(_arg: s, _arg: fqid, _arg: QBMAN_FQ_XOFF) -> return;
}
// If the user has been allocated a channel object that is going to generate
// CDANs to another channel, then the qbman_swp_CDAN* functions will be
// necessary.
//
// CDAN-enabled channels only generate a single CDAN notification, after which
// they need to be reenabled before they'll generate another. The idea is
// that pull dequeuing will occur in reaction to the CDAN, followed by a
// reenable step. Each function generates a distinct command to hardware, so a
// combination function is provided if the user wishes to modify the "context"
// (which shows up in each CDAN message) each time they reenable, as a single
// command to hardware.
//
// qbman_swp_CDAN_set_context() - Set CDAN context
// @s:         the software portal object
// @channelid: the channel index
// @ctx:       the context to be set in CDAN
//
// Return 0 for success, or negative error code for failure.
//
// qbman_swp_CDAN_enable() - Enable CDAN for the channel
// @s:         the software portal object
// @channelid: the index of the channel to generate CDAN
//
// Return 0 for success, or negative error code for failure.
//
// qbman_swp_CDAN_disable() - disable CDAN for the channel
// @s:         the software portal object
// @channelid: the index of the channel to generate CDAN
//
// Return 0 for success, or negative error code for failure.
//
// qbman_swp_CDAN_set_context_enable() - Set CDAN contest and enable CDAN
// @s:         the software portal object
// @channelid: the index of the channel to generate CDAN
// @ctx:i      the context set in CDAN
//
// Return 0 for success, or negative error code for failure.
//
// Wraps up submit + poll-for-result
// Query APIs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qbman_fq_query_np_rslt {
    pub verb: u8,
    pub rslt: u8,
    pub st1: u8,
    pub st2: u8,
    pub reserved: [u8; 2],
    pub od1_sfdr: __le16,
    pub od2_sfdr: __le16,
    pub od3_sfdr: __le16,
    pub ra1_sfdr: __le16,
    pub ra2_sfdr: __le16,
    pub pfdr_hptr: __le32,
    pub pfdr_tptr: __le32,
    pub frm_cnt: __le32,
    pub byte_cnt: __le32,
    pub ics_surp: __le16,
    pub is: u8,
    pub reserved2: [u8; 29],
}

extern "C" {
    pub fn qbman_fq_state_frame_count(r: *const qbman_fq_query_np_rslt) -> u32;
}
extern "C" {
    pub fn qbman_fq_state_byte_count(r: *const qbman_fq_query_np_rslt) -> u32;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qbman_bp_query_rslt {
    pub verb: u8,
    pub rslt: u8,
    pub reserved: [u8; 4],
    pub bdi: u8,
    pub state: u8,
    pub fill: __le32,
    pub hdotr: __le32,
    pub swdet: __le16,
    pub swdxt: __le16,
    pub hwdet: __le16,
    pub hwdxt: __le16,
    pub swset: __le16,
    pub swsxt: __le16,
    pub vbpid: __le16,
    pub icid: __le16,
    pub bpscn_addr: __le64,
    pub bpscn_ctx: __le64,
    pub hw_targ: __le16,
    pub dbe: u8,
    pub reserved2: u8,
    pub sdcnt: u8,
    pub hdcnt: u8,
    pub sscnt: u8,
    pub reserved3: [u8; 9],
}

extern "C" {
    pub fn qbman_bp_info_num_free_bufs(a: *mut qbman_bp_query_rslt) -> u32;
}
//
// qbman_swp_release() - Issue a buffer release command
// @s:           the software portal object
// @d:           the release descriptor
// @buffers:     a pointer pointing to the buffer address to be released
// @num_buffers: number of buffers to be released,  must be less than 8
//
// Return 0 for success, -EBUSY if the release command ring is not ready.
//
extern "C" {
    pub fn qbman_swp_release_ptr(_arg: s, _arg: d, _arg: buffers, _arg: num_buffers) -> return;
}
//
// qbman_swp_pull() - Issue the pull dequeue command
// @s: the software portal object
// @d: the software portal descriptor which has been configured with
// the set of qbman_pull_desc_set_*() calls
//
// Return 0 for success, and -EBUSY if the software portal is not ready
// to do pull dequeue.
//
extern "C" {
    pub fn qbman_swp_pull_ptr(_arg: s, _arg: d) -> return;
}
//
// qbman_swp_dqrr_next() - Get an valid DQRR entry
// @s: the software portal object
//
// Return NULL if there are no unconsumed DQRR entries. Return a DQRR entry
// only once, so repeated calls can return a sequence of DQRR entries, without
// requiring they be consumed immediately or in any particular order.
//
extern "C" {
    pub fn qbman_swp_dqrr_next_ptr(_arg: s) -> return;
}
