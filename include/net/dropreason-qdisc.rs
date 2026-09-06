//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/dropreason-qdisc.h
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
// enum qdisc_drop_reason - reason why a qdisc dropped a packet
//
// Qdisc-specific drop reasons for packet drops that occur within the
// traffic control (TC) queueing discipline layer. These reasons provide
// detailed diagnostics about why packets were dropped by various qdisc
// algorithms, enabling fine-grained monitoring and troubleshooting of
// queue behavior.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qdisc_drop_reason {
//
// @QDISC_DROP_UNSPEC: unspecified/invalid qdisc drop reason.
// Value 0 serves as analogous to SKB_NOT_DROPPED_YET for enum skb_drop_reason.
// Used for catching zero-initialized drop_reason fields.
//
    QDISC_DROP_UNSPEC = 0,
//
// @__QDISC_DROP_REASON: subsystem base value for qdisc drop reasons
//
    __QDISC_DROP_REASON = SKB_DROP_REASON_SUBSYS_QDISC <<
    SKB_DROP_REASON_SUBSYS_SHIFT,
//
// @QDISC_DROP_GENERIC: generic/default qdisc drop, used when no
// more specific reason applies
//
    QDISC_DROP_GENERIC,
//
// @QDISC_DROP_OVERLIMIT: packet dropped because the qdisc queue
// length exceeded its configured limit (sch->limit). This typically
// indicates the queue is full and cannot accept more packets.
//
    QDISC_DROP_OVERLIMIT,
//
// @QDISC_DROP_CONGESTED: packet dropped due to active congestion
// control algorithms (e.g., CoDel, PIE, RED) detecting network
// congestion. The qdisc proactively dropped the packet to signal
// congestion to the sender and prevent bufferbloat.
//
    QDISC_DROP_CONGESTED,
//
// @QDISC_DROP_MAXFLOWS: packet dropped because the qdisc's flow
// tracking table is full and no free slots are available to allocate
// for a new flow. This indicates flow table exhaustion in flow-based
// qdiscs that maintain per-flow state (e.g., SFQ).
//
    QDISC_DROP_MAXFLOWS,
//
// @QDISC_DROP_FLOOD_PROTECTION: packet dropped by flood protection
// mechanism detecting unresponsive flows (potential DoS/flood).
// Used by qdiscs implementing probabilistic drop algorithms like
// BLUE (e.g., CAKE's Cobalt AQM).
//
    QDISC_DROP_FLOOD_PROTECTION,
//
// @QDISC_DROP_BAND_LIMIT: packet dropped because the priority band's
// limit was reached. Used by qdiscs with priority bands that have
// per-band packet limits (e.g., FQ).
//
    QDISC_DROP_BAND_LIMIT,
//
// @QDISC_DROP_HORIZON_LIMIT: packet dropped because its timestamp
// is too far in the future (beyond the configured horizon).
// Used by qdiscs with time-based scheduling (e.g., FQ).
//
    QDISC_DROP_HORIZON_LIMIT,
//
// @QDISC_DROP_FLOW_LIMIT: packet dropped because an individual flow
// exceeded its per-flow packet/depth limit. Used by FQ and SFQ qdiscs
// to enforce per-flow fairness and prevent a single flow from
// monopolizing queue resources.
//
    QDISC_DROP_FLOW_LIMIT,
//
// @QDISC_DROP_L4S_STEP_NON_ECN: DualPI2 qdisc dropped a non-ECN-capable
// packet because the L4S queue delay exceeded the step threshold.
// Since the packet cannot be ECN-marked, it must be dropped to signal
// congestion. See RFC 9332 for the DualQ Coupled AQM step mechanism.
//
    QDISC_DROP_L4S_STEP_NON_ECN,
//
// @QDISC_DROP_MAX: the maximum of qdisc drop reasons, which
// shouldn't be used as a real 'reason' - only for tracing code gen
//
    QDISC_DROP_MAX,
}

