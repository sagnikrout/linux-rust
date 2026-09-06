//! Automatically rewritten from C to Rust
//! Source: net/sunrpc/timer.c
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
// linux/net/sunrpc/timer.c
//
// Estimate RPC request round trip time.
//
// Based on packet round-trip and variance estimator algorithms described
// in appendix A of "Congestion Avoidance and Control" by Van Jacobson
// and Michael J. Karels (ACM Computer Communication Review; Proceedings
// of the Sigcomm '88 Symposium in Stanford, CA, August, 1988).
//
// This RTT estimator is used only for RPC over datagram protocols.
//
// Copyright (C) 2002 Trond Myklebust <trond.myklebust@fys.uio.no>
//

//
// rpc_init_rtt - Initialize an RPC RTT estimator context
// @rt: context to initialize
// @timeo: initial timeout value, in jiffies
//
#[no_mangle]
pub unsafe extern "C" fn rpc_init_rtt(rt: *mut rpc_rtt, timeo: c_ulong) {
    void rpc_init_rtt(struct rpc_rtt *rt, unsigned long timeo)
    {
    let mut init: c_ulong = 0;
    unsigned int i;
    rt.timeo = timeo;
    if (timeo > RPC_RTO_INIT)
    init = (timeo - RPC_RTO_INIT) << 3;
    for (i = 0; i < 5; i++) {
    rt.srtt[i] = init;
    rt.sdrtt[i] = RPC_RTO_INIT;
    rt.ntimeouts[i] = 0;
    }
    }
    EXPORT_SYMBOL_GPL(rpc_init_rtt);
//
// rpc_update_rtt - Update an RPC RTT estimator context
// @rt: context to update
// @timer: timer array index (request type)
// @m: recent actual RTT, in jiffies
//
// NB: When computing the smoothed RTT and standard deviation,
// be careful not to produce negative intermediate results.
//
#[no_mangle]
pub unsafe extern "C" fn rpc_update_rtt(rt: *mut rpc_rtt, timer: c_uint, m: c_long) {
    void rpc_update_rtt(struct rpc_rtt *rt, unsigned int timer, long m)
    {
    long *srtt, *sdrtt;
    if (timer-- == 0)
    return;
// jiffies wrapped; ignore this one
    if (m < 0)
    return;
    if (m == 0)
    m = 1L;
    srtt = (long *)&rt.srtt[timer];
    m -= *srtt >> 3;
// srtt += m;
    if (m < 0)
    m = -m;
    sdrtt = (long *)&rt.sdrtt[timer];
    m -= *sdrtt >> 2;
// sdrtt += m;
// Set lower bound on the variance
    if (*sdrtt < RPC_RTO_MIN)
// sdrtt = RPC_RTO_MIN;
    }
    EXPORT_SYMBOL_GPL(rpc_update_rtt);
//
// rpc_calc_rto - Provide an estimated timeout value
// @rt: context to use for calculation
// @timer: timer array index (request type)
//
// Estimate RTO for an NFS RPC sent via an unreliable datagram.  Use
// the mean and mean deviation of RTT for the appropriate type of RPC
// for frequently issued RPCs, and a fixed default for the others.
//
// The justification for doing "other" this way is that these RPCs
// happen so infrequently that timer estimation would probably be
// stale.  Also, since many of these RPCs are non-idempotent, a
// conservative timeout is desired.
//
// getattr, lookup,
// read, write, commit     - A+4D
// other                   - timeo
//
#[no_mangle]
pub unsafe extern "C" fn rpc_calc_rto(rt: *mut rpc_rtt, timer: c_uint) -> c_ulong {
    unsigned long rpc_calc_rto(struct rpc_rtt *rt, unsigned int timer)
    {
    unsigned long res;
    if (timer-- == 0)
    return rt.timeo;
    res = ((rt.srtt[timer] + 7) >> 3) + rt.sdrtt[timer];
    if (res > RPC_RTO_MAX)
    res = RPC_RTO_MAX;
    return res;
    }
    EXPORT_SYMBOL_GPL(rpc_calc_rto);
