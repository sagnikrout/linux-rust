//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/surface/aggregator/ssh_packet_layer.h
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
// SSH packet transport layer.
//
// Copyright (C) 2019-2022 Maximilian Luz <luzmaximilian@gmail.com>
//

//
// enum ssh_ptl_state_flags - State-flags for &struct ssh_ptl.
//
// @SSH_PTL_SF_SHUTDOWN_BIT:
// Indicates that the packet transport layer has been shut down or is
// being shut down and should not accept any new packets/data.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssh_ptl_state_flags {
    SSH_PTL_SF_SHUTDOWN_BIT,
}

//
// struct ssh_ptl_ops - Callback operations for packet transport layer.
// @data_received: Function called when a data-packet has been received. Both,
// the packet layer on which the packet has been received and
// the packet's payload data are provided to this function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssh_ptl_ops {
    pub data): *const *const *const void (data_received)(struct ssh_ptl p, struct ssam_span,
}

//
// struct ssh_ptl - SSH packet transport layer.
// @serdev:        Serial device providing the underlying data transport.
// @state:         State(-flags) of the transport layer.
// @queue:         Packet submission queue.
// @queue.lock:    Lock for modifying the packet submission queue.
// @queue.head:    List-head of the packet submission queue.
// @pending:       Set/list of pending packets.
// @pending.lock:  Lock for modifying the pending set.
// @pending.head:  List-head of the pending set/list.
// @pending.count: Number of currently pending packets.
// @tx:            Transmitter subsystem.
// @tx.running:    Flag indicating (desired) transmitter thread state.
// @tx.thread:     Transmitter thread.
// @tx.thread_cplt_tx:  Completion for transmitter thread waiting on transfer.
// @tx.thread_cplt_pkt: Completion for transmitter thread waiting on packets.
// @tx.packet_wq:  Waitqueue-head for packet transmit completion.
// @rx:            Receiver subsystem.
// @rx.thread:     Receiver thread.
// @rx.wq:         Waitqueue-head for receiver thread.
// @rx.fifo:       Buffer for receiving data/pushing data to receiver thread.
// @rx.buf:        Buffer for evaluating data on receiver thread.
// @rx.blocked:    List of recent/blocked sequence IDs to detect retransmission.
// @rx.blocked.seqs:   Array of blocked sequence IDs.
// @rx.blocked.offset: Offset indicating where a new ID should be inserted.
// @rtx_timeout:   Retransmission timeout subsystem.
// @rtx_timeout.lock:    Lock for modifying the retransmission timeout reaper.
// @rtx_timeout.timeout: Timeout interval for retransmission.
// @rtx_timeout.expires: Time specifying when the reaper work is next scheduled.
// @rtx_timeout.reaper:  Work performing timeout checks and subsequent actions.
// @ops:           Packet layer operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssh_ptl {
    pub serdev: *mut serdev_device,
    pub state: c_ulong,
    pub lock: spinlock_t,
    pub head: list_head,
    pub queue: },
    pub lock: spinlock_t,
    pub head: list_head,
    pub count: core::sync::atomic::AtomicI32,
    pub pending: },
    pub running: core::sync::atomic::AtomicI32,
    pub thread: *mut task_struct,
    pub thread_cplt_tx: completion,
    pub thread_cplt_pkt: completion,
    pub packet_wq: wait_queue_head,
    pub tx: },
    pub thread: *mut task_struct,
    pub wq: wait_queue_head,
    pub fifo: kfifo,
    pub buf: sshp_buf,
    pub seqs: [u16; 8],
    pub offset: u16,
    pub blocked: },
    pub rx: },
    pub lock: spinlock_t,
    pub timeout: ktime_t,
    pub expires: ktime_t,
    pub reaper: delayed_work,
    pub rtx_timeout: },
    pub ops: ssh_ptl_ops,
}

extern "C" {
    pub fn ssh_ptl_destroy(ptl: *mut ssh_ptl);
}
//
// ssh_ptl_get_device() - Get device associated with packet transport layer.
// @ptl: The packet transport layer.
//
// Return: Returns the device on which the given packet transport layer builds
// upon.
//
extern "C" {
    pub fn ssh_ptl_tx_start(ptl: *mut ssh_ptl) -> c_int;
}
extern "C" {
    pub fn ssh_ptl_tx_stop(ptl: *mut ssh_ptl) -> c_int;
}
extern "C" {
    pub fn ssh_ptl_rx_start(ptl: *mut ssh_ptl) -> c_int;
}
extern "C" {
    pub fn ssh_ptl_rx_stop(ptl: *mut ssh_ptl) -> c_int;
}
extern "C" {
    pub fn ssh_ptl_shutdown(ptl: *mut ssh_ptl);
}
extern "C" {
    pub fn ssh_ptl_submit(ptl: *mut ssh_ptl, p: *mut ssh_packet) -> c_int;
}
extern "C" {
    pub fn ssh_ptl_cancel(p: *mut ssh_packet);
}
extern "C" {
    pub fn ssh_ptl_rx_rcvbuf(ptl: *mut ssh_ptl, buf: *const u8, n: usize) -> isize;
}
//
// ssh_ptl_tx_wakeup_transfer() - Wake up packet transmitter thread for
// transfer.
// @ptl: The packet transport layer.
//
// Wakes up the packet transmitter thread, notifying it that the underlying
// transport has more space for data to be transmitted. If the packet
// transport layer has been shut down, calls to this function will be ignored.
//
extern "C" {
    pub fn ssh_ctrl_packet_cache_init() -> c_int;
}
extern "C" {
    pub fn ssh_ctrl_packet_cache_destroy();
}
