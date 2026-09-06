//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/fq.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2016 Qualcomm Atheros, Inc
//
// Based on net/sched/sch_fq_codel.c
//

//
// struct fq_flow - per traffic flow queue
//
// @tin: owner of this flow. Used to manage collisions, i.e. when a packet
// hashes to an index which points to a flow that is already owned by a
// different tin the packet is destined to. In such case the implementer
// must provide a fallback flow
// @flowchain: can be linked to fq_tin's new_flows or old_flows. Used for DRR++
// (deficit round robin) based round robin queuing similar to the one
// found in net/sched/sch_fq_codel.c
// @queue: sk_buff queue to hold packets
// @backlog: number of bytes pending in the queue. The number of packets can be
// found in @queue.qlen
// @deficit: used for DRR++
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fq_flow {
    pub tin: *mut fq_tin,
    pub flowchain: list_head,
    pub queue: sk_buff_head,
    pub backlog: u32,
    pub deficit: c_int,
}

//
// struct fq_tin - a logical container of fq_flows
//
// Used to group fq_flows into a logical aggregate. DRR++ scheme is used to
// pull interleaved packets out of the associated flows.
//
// @new_flows: linked list of fq_flow
// @old_flows: linked list of fq_flow
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fq_tin {
    pub new_flows: list_head,
    pub old_flows: list_head,
    pub tin_list: list_head,
    pub default_flow: fq_flow,
    pub backlog_bytes: u32,
    pub backlog_packets: u32,
    pub overlimit: u32,
    pub collisions: u32,
    pub flows: u32,
    pub tx_bytes: u32,
    pub tx_packets: u32,
}

//
// struct fq - main container for fair queuing purposes
//
// @limit: max number of packets that can be queued across all flows
// @backlog: number of packets queued across all flows
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fq {
    pub flows: *mut fq_flow,
    pub flows_bitmap: *mut c_ulong,
    pub tin_backlog: list_head,
    pub lock: spinlock_t,
    pub flows_cnt: u32,
    pub limit: u32,
    pub memory_limit: u32,
    pub memory_usage: u32,
    pub quantum: u32,
    pub backlog: u32,
    pub overlimit: u32,
    pub overmemory: u32,
    pub collisions: u32,
}

// Return %true to filter (drop) the frame.
