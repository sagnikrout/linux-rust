//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/cluster/tcp_internal.h
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
// Copyright (C) 2005 Oracle.  All rights reserved.
//

// we're delaying our quorum decision so that heartbeat will have timed
// out truly dead nodes by the time we come around to making decisions
// on their number

//
// This version number represents quite a lot, unfortunately.  It not
// only represents the raw network message protocol on the wire but also
// locking semantics of the file system using the protocol.  It should
// be somewhere else, I'm sure, but right now it isn't.
//
// With version 11, we separate out the filesystem locking portion.  The
// filesystem now has a major.minor version it negotiates.  Version 11
// introduces this negotiation to the o2dlm protocol, and as such the
// version here in tcp_internal.h should not need to be bumped for
// filesystem locking changes.
//
// New in version 11
// - Negotiation of filesystem locking in the dlm join.
//
// New in version 10:
// - Meta/data locks combined
//
// New in version 9:
// - All votes removed
//
// New in version 8:
// - Replace delete inode votes with a cluster lock
//
// New in version 7:
// - DLM join domain includes the live nodemap
//
// New in version 6:
// - DLM lockres remote refcount fixes.
//
// New in version 5:
// - Network timeout checking protocol
//
// New in version 4:
// - Remove i_generation from lock names for better stat performance.
//
// New in version 3:
// - Replace dentry votes with a cluster lock
//
// New in version 2:
// - full 64 bit i_size in the metadata lock lvbs
// - introduction of "rw" lock and pushing meta/data locking down
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct o2net_handshake {
    pub protocol_version: __be64,
    pub connector_id: __be64,
    pub o2hb_heartbeat_timeout_ms: __be32,
    pub o2net_idle_timeout_ms: __be32,
    pub o2net_keepalive_delay_ms: __be32,
    pub o2net_reconnect_delay_ms: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct o2net_node {
// this is never called from int/bh
    pub nn_lock: spinlock_t,
// set the moment an sc is allocated and a connect is started
    pub nn_sc: *mut o2net_sock_container,
// _valid is only set after the handshake passes and tx can happen
    pub nn_sc_valid:1: unsigned,
// if this is set tx just returns it
    pub nn_persistent_error: c_int,
// It is only set to 1 after the idle time out.
    pub nn_timeout: core::sync::atomic::AtomicI32,
// threads waiting for an sc to arrive wait on the wq for generation
// to increase.  it is increased when a connecting socket succeeds
// or fails or when an accepted socket is attached.
    pub nn_sc_wq: wait_queue_head_t,
    pub nn_status_idr: idr,
    pub nn_status_list: list_head,
// connects are attempted from when heartbeat comes up until either hb
// goes down, the node is unconfigured, or a connect succeeds.
// connect_work is queued from set_nn_state both from hb up and from
// itself if a connect attempt fails and so can be self-arming.
// shutdown is careful to first mark the nn such that no connects will
// be attempted before canceling delayed connect work and flushing the
// queue.
    pub nn_connect_work: delayed_work,
    pub nn_last_connect_attempt: c_ulong,
// this is queued as nodes come up and is canceled when a connection is
// established.  this expiring gives up on the node and errors out
// transmits
    pub nn_connect_expired: delayed_work,
// after we give up on a socket we wait a while before deciding
// that it is still heartbeating and that we should do some
// quorum work
    pub nn_still_up: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct o2net_sock_container {
    pub sc_kref: kref,
// the next two are valid for the life time of the sc
    pub sc_sock: *mut socket,
    pub sc_node: *mut o2nm_node,
// all of these sc work structs hold refs on the sc while they are
// queued.  they should not be able to ref a freed sc.  the teardown
// race is with o2net_wq destruction in o2net_stop_listening()
// rx and connect work are generated from socket callbacks.  sc
// shutdown removes the callbacks and then flushes the work queue
    pub sc_rx_work: work_struct,
    pub sc_connect_work: work_struct,
// shutdown work is triggered in two ways.  the simple way is
// for a code path calls ensure_shutdown which gets a lock, removes
// the sc from the nn, and queues the work.  in this case the
// work is single-shot.  the work is also queued from a sock
// callback, though, and in this case the work will find the sc
// still on the nn and will call ensure_shutdown itself.. this
// ends up triggering the shutdown work again, though nothing
// will be done in that second iteration.  so work queue teardown
// has to be careful to remove the sc from the nn before waiting
// on the work queue so that the shutdown work doesn't remove the
// sc and rearm itself.
//
    pub sc_shutdown_work: work_struct,
    pub sc_idle_timeout: timer_list,
    pub sc_keepalive_work: delayed_work,
    pub sc_handshake_ok:1: unsigned,
    pub sc_page: *mut page,
    pub sc_page_off: usize,
// original handlers for the sockets
    pub sk): *mut *mut void (sc_state_change)(struct sock,
    pub sk): *mut *mut void (sc_data_ready)(struct sock,
    pub sc_msg_key: u32,
    pub sc_msg_type: u16,

    pub sc_net_debug_item: list_head,
    pub sc_tv_timer: ktime_t,
    pub sc_tv_data_ready: ktime_t,
    pub sc_tv_advance_start: ktime_t,
    pub sc_tv_advance_stop: ktime_t,
    pub sc_tv_func_start: ktime_t,
    pub sc_tv_func_stop: ktime_t,

    pub sc_tv_acquiry_total: ktime_t,
    pub sc_tv_send_total: ktime_t,
    pub sc_tv_status_total: ktime_t,
    pub sc_send_count: u32,
    pub sc_recv_count: u32,
    pub sc_tv_process_total: ktime_t,

    pub sc_send_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct o2net_msg_handler {
    pub nh_node: rb_node,
    pub nh_max_len: u32,
    pub nh_msg_type: u32,
    pub nh_key: u32,
    pub nh_func: *mut o2net_msg_handler_func,
    pub nh_func_data: *mut c_void,
// nh_post_func;
    pub nh_kref: kref,
    pub nh_unregister_item: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum o2net_system_error {
    O2NET_ERR_NONE = 0,
    O2NET_ERR_NO_HNDLR,
    O2NET_ERR_OVERFLOW,
    O2NET_ERR_DIED,
    O2NET_ERR_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct o2net_status_wait {
    pub ns_sys_status: o2net_system_error,
    pub ns_status: i32,
    pub ns_id: c_int,
    pub ns_wq: wait_queue_head_t,
    pub ns_node_item: list_head,
}

// just for state dumps
#[repr(C)]
#[derive(Copy, Clone)]
pub struct o2net_send_tracking {
    pub st_net_debug_item: list_head,
    pub st_task: *mut task_struct,
    pub st_sc: *mut o2net_sock_container,
    pub st_id: u32,
    pub st_msg_type: u32,
    pub st_msg_key: u32,
    pub st_node: u8,
    pub st_sock_time: ktime_t,
    pub st_send_time: ktime_t,
    pub st_status_time: ktime_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct o2net_send_tracking {
    pub dummy: u32,
}

