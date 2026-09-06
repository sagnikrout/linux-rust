//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/iowait.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2015 - 2018 Intel Corporation.
//

//
// typedef (*restart_t)() - restart callback
// @work: pointer to work structure
//
extern "C" {
    pub fn void(work: *mut *mut restart_t)(struct work_struct) -> typedef;
}
pub const IOWAIT_PENDING_IB: c_uint = 0x0;
pub const IOWAIT_PENDING_TID: c_uint = 0x1;
//
// A QP can have multiple Send Engines (SEs).
//
// The current use case is for supporting a TID RDMA
// packet build/xmit mechanism independent from verbs.
//
pub const IOWAIT_SES: c_int = 2;
pub const IOWAIT_IB_SE: c_int = 0;
pub const IOWAIT_TID_SE: c_int = 1;
//
// @iowork: the work struct
// @tx_head: list of prebuilt packets
// @iow: the parent iowait structure
//
// This structure is the work item (process) specific
// details associated with the each of the two SEs of the
// QP.
//
// The workstruct and the queued TXs are unique to each
// SE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iowait_work {
    pub iowork: work_struct,
    pub tx_head: list_head,
    pub iow: *mut iowait,
}

//
// @list: used to add/insert into QP/PQ wait lists
// @tx_head: overflow list of sdma_txreq's
// @sleep: no space callback
// @wakeup: space callback wakeup
// @sdma_drained: sdma count drained
// @init_priority: callback to manipulate priority
// @lock: lock protected head of wait queue
// @iowork: workqueue overhead
// @wait_dma: wait for sdma_busy == 0
// @wait_pio: wait for pio_busy == 0
// @sdma_busy: # of packets in flight
// @count: total number of descriptors in tx_head'ed list
// @tx_limit: limit for overflow queuing
// @tx_count: number of tx entry's in tx_head'ed list
// @flags: wait flags (one per QP)
// @wait: SE array for multiple legs
//
// This is to be embedded in user's state structure
// (QP or PQ).
//
// The sleep and wakeup members are a
// bit misnamed.   They do not strictly
// speaking sleep or wake up, but they
// are callbacks for the ULP to implement
// what ever queuing/dequeuing of
// the embedded iowait and its containing struct
// when a resource shortage like SDMA ring space
// or PIO credit space is seen.
//
// Both potentially have locks help
// so sleeping is not allowed and it is not
// supported to submit txreqs from the wakeup
// call directly because of lock conflicts.
//
// The wait_dma member along with the iow
//
// The lock field is used by waiters to record
// the seqlock_t that guards the list head.
// Waiters explicitly know that, but the destroy
// code that unwaits QPs does not.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iowait {
    pub list: list_head,
    pub reason): *mut *mut *mut void (wakeup)(struct iowait wait, int,
    pub wait): *mut *mut void (sdma_drained)(struct iowait,
    pub wait): *mut *mut void (init_priority)(struct iowait,
    pub lock: *mut seqlock_t,
    pub wait_dma: wait_queue_head_t,
    pub wait_pio: wait_queue_head_t,
    pub sdma_busy: core::sync::atomic::AtomicI32,
    pub pio_busy: core::sync::atomic::AtomicI32,
    pub count: u32,
    pub tx_limit: u32,
    pub tx_count: u32,
    pub starved_cnt: u8,
    pub priority: u8,
    pub flags: c_ulong,
    pub wait: [iowait_work; IOWAIT_SES],
}

pub const SDMA_AVAIL_REASON: c_int = 0;
extern "C" {
    pub fn iowait_set_flag(wait: *mut iowait, flag: u32);
}
extern "C" {
    pub fn iowait_flag_set(wait: *mut iowait, flag: u32) -> bool;
}
extern "C" {
    pub fn iowait_clear_flag(wait: *mut iowait, flag: u32);
}
//
// iowait_schedule() - schedule the default send engine work
// @wait: wait struct to schedule
// @wq: workqueue for schedule
// @cpu: cpu
//
// iowait_tid_schedule - schedule the tid SE
// @wait: the iowait structure
// @wq: the work queue
// @cpu: the cpu
//
// iowait_sdma_drain() - wait for DMAs to drain
//
// @wait: iowait structure
//
// This will delay until the iowait sdmas have
// completed.
//
// iowait_sdma_pending() - return sdma pending count
//
// @wait: iowait structure
//
extern "C" {
    pub fn atomic_read(_arg: &wait->sdma_busy) -> return;
}
//
// iowait_sdma_inc - note sdma io pending
// @wait: iowait structure
//
// iowait_sdma_add - add count to pending
// @wait: iowait structure
//
// iowait_sdma_dec - note sdma complete
// @wait: iowait structure
//
extern "C" {
    pub fn atomic_dec_and_test(_arg: &wait->sdma_busy) -> return;
}
//
// iowait_pio_drain() - wait for pios to drain
//
// @wait: iowait structure
//
// This will delay until the iowait pios have
// completed.
//
// iowait_pio_pending() - return pio pending count
//
// @wait: iowait structure
//
extern "C" {
    pub fn atomic_read(_arg: &wait->pio_busy) -> return;
}
//
// iowait_pio_inc - note pio pending
// @wait: iowait structure
//
// iowait_pio_dec - note pio complete
// @wait: iowait structure
//
extern "C" {
    pub fn atomic_dec_and_test(_arg: &wait->pio_busy) -> return;
}
//
// iowait_drain_wakeup() - trigger iowait_drain() waiter
//
// @wait: iowait structure
//
// This will trigger any waiters.
//
// iowait_get_txhead() - get packet off of iowait list
//
// @wait: iowait_work structure
//
// iowait_queue - Put the iowait on a wait queue
// @pkts_sent: have some packets been sent before queuing?
// @w: the iowait struct
// @wait_head: the wait queue
//
// This function is called to insert an iowait struct into a
// wait queue after a resource (eg, sdma descriptor or pio
// buffer) is run out.
//
// To play fair, insert the iowait at the tail of the wait queue if it
// has already sent some packets; Otherwise, put it at the head.
// However, if it has priority packets to send, also put it at the
// head.
//
// iowait_starve_clear - clear the wait queue's starve count
// @pkts_sent: have some packets been sent?
// @w: the iowait struct
//
// This function is called to clear the starve count. If no
// packets have been sent, the starve count will not be cleared.
//
// Update the top priority index
//
// iowait_packet_queued() - determine if a packet is queued
// @wait: the iowait_work structure
//
// inc_wait_count - increment wait counts
// @w: the log work struct
// @n: the count
//
// iowait_get_tid_work - return iowait_work for tid SE
// @w: the iowait struct
//
// iowait_get_ib_work - return iowait_work for ib SE
// @w: the iowait struct
//
// iowait_ioww_to_iow - return iowait given iowait_work
// @w: the iowait_work struct
//
extern "C" {
    pub fn iowait_cancel_work(w: *mut iowait);
}
extern "C" {
    pub fn iowait_set_work_flag(w: *mut iowait_work) -> c_int;
}
