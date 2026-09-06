//! Automatically rewritten from C to Rust
//! Source: samples/bpf/hbm_out_kern.c
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
// Copyright (c) 2019 Facebook
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//
// Sample Host Bandwidth Manager (HBM) BPF program.
//
// A cgroup skb BPF egress program to limit cgroup output bandwidth.
// It uses a modified virtual token bucket queue to limit average
// egress bandwidth. The implementation uses credits instead of tokens.
// Negative credits imply that queueing would have happened (this is
// a virtual queue, so no queueing is done by it. However, queueing may
// occur at the actual qdisc (which is not used for rate limiting).
//
// This implementation uses 3 thresholds, one to start marking packets and
// the other two to drop packets:
// CREDIT
// - <--------------------------|------------------------> +
// |    |          |      0
// |  Large pkt    |
// |  drop thresh  |
// Small pkt drop             Mark threshold
// thresh
//
// The effect of marking depends on the type of packet:
// a) If the packet is ECN enabled and it is a TCP packet, then the packet
// is ECN marked.
// b) If the packet is a TCP packet, then we probabilistically call tcp_cwr
// to reduce the congestion window. The current implementation uses a linear
// distribution (0% probability at marking threshold, 100% probability
// at drop threshold).
// c) If the packet is not a TCP packet, then it is dropped.
//
// If the credit is below the drop threshold, the packet is dropped. If it
// is a TCP packet, then it also calls tcp_cwr since packets dropped by
// by a cgroup skb BPF program do not automatically trigger a call to
// tcp_cwr in the current kernel code.
//
// This BPF program actually uses 2 drop thresholds, one threshold
// for larger packets (>= 120 bytes) and another for smaller packets. This
// protects smaller packets such as SYNs, ACKs, etc.
//
// The default bandwidth limit is set at 1Gbps but this can be changed by
// a user program through a shared BPF map. In addition, by default this BPF
// program does not limit connections using loopback. This behavior can be
// overwritten by the user program. There is also an option to calculate
// some statistics, such as percent of packets marked or dropped, which
// the user program can access.
//
// A latter patch provides such a program (hbm.c)
//

    SEC("cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn _hbm_out_cg(skb: *mut __sk_buff) -> c_int {
    int _hbm_out_cg(struct __sk_buff *skb)
    {
    struct hbm_pkt_info pkti;
    let mut len: c_int = skb.len;
    let mut queue_index: c_uint = 0;
    unsigned long long curtime;
    int credit;
    let mut delta: signed long long = 0, new_credit;
    let mut max_credit: c_int = MAX_CREDIT;
    let mut congestion_flag: bool = false;
    let mut drop_flag: bool = false;
    let mut cwr_flag: bool = false;
    let mut ecn_ce_flag: bool = false;
    struct hbm_vqueue *qdp;
    struct hbm_queue_stats *qsp = core::ptr::null_mut();
    let mut rv: c_int = ALLOW_PKT;
    qsp = bpf_map_lookup_elem(&queue_stats, &queue_index);
    if (qsp != core::ptr::null_mut() && !qsp.loopback && (skb.ifindex == 1))
    return ALLOW_PKT;
    hbm_get_pkt_info(skb, &pkti);
// We may want to account for the length of headers in len
// calculation, like ETH header + overhead, specially if it
// is a gso packet. But I am not doing it right now.
    qdp = bpf_get_local_storage(&queue_state, 0);
    if (!qdp)
    return ALLOW_PKT;
#[no_mangle]
pub unsafe extern "C" fn if(0: qdp->lasttime ==) -> else {
    else if (qdp.lasttime == 0)
    hbm_init_vqueue(qdp, 1024);
    curtime = bpf_ktime_get_ns();
// Begin critical section
    bpf_spin_lock(&qdp.lock);
    credit = qdp.credit;
    delta = curtime - qdp.lasttime;
// delta < 0 implies that another process with a curtime greater
// than ours beat us to the critical section and already added
// the new credit, so we should not add it ourselves
//
    if (delta > 0) {
    qdp.lasttime = curtime;
    new_credit = credit + CREDIT_PER_NS(delta, qdp.rate);
    if (new_credit > MAX_CREDIT)
    credit = MAX_CREDIT;
    else
    credit = new_credit;
    }
    credit -= len;
    qdp.credit = credit;
    bpf_spin_unlock(&qdp.lock);
// End critical section
// Check if we should update rate
    if (qsp != core::ptr::null_mut() && (qsp.rate * 128) != qdp.rate) {
    qdp.rate = qsp.rate * 128;
    bpf_printk("Updating rate: %d (1sec:%llu bits)\n",
    (int)qdp.rate,
    CREDIT_PER_NS(1000000000, qdp.rate) * 8);
    }
// Set flags (drop, congestion, cwr)
// Dropping => we are congested, so ignore congestion flag
    if (credit < -DROP_THRESH ||
    (len > LARGE_PKT_THRESH && credit < -LARGE_PKT_DROP_THRESH)) {
// Very congested, set drop packet
    drop_flag = true;
    if (pkti.ecn)
    congestion_flag = true;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: pkti.is_tcp) -> else {
    else if (pkti.is_tcp)
    cwr_flag = true;
    } else if (credit < 0) {
// Congested, set congestion flag
    if (pkti.ecn || pkti.is_tcp) {
    if (credit < -MARK_THRESH)
    congestion_flag = true;
    else
    congestion_flag = false;
    } else {
    congestion_flag = true;
    }
    }
    if (congestion_flag) {
    if (bpf_skb_ecn_set_ce(skb)) {
    ecn_ce_flag = true;
    } else {
    if (pkti.is_tcp) {
    let mut rand: c_uint = bpf_get_prandom_u32();
    if (-credit >= MARK_THRESH +
    (rand % MARK_REGION_SIZE)) {
// Do congestion control
    cwr_flag = true;
    }
    } else if (len > LARGE_PKT_THRESH) {
// Problem if too many small packets?
    drop_flag = true;
    }
    }
    }
    if (qsp != core::ptr::null_mut())
    if (qsp.no_cn)
    cwr_flag = false;
    hbm_update_stats(qsp, len, curtime, congestion_flag, drop_flag,
    cwr_flag, ecn_ce_flag, &pkti, credit);
    if (drop_flag) {
    __sync_add_and_fetch(&(qdp.credit), len);
    rv = DROP_PKT;
    }
    if (cwr_flag)
    rv |= 2;
    return rv;
    }
    char _license[] SEC("license") = "GPL";
