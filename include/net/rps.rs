//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/rps.h
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
// This structure holds an RPS map which can be of variable length.  The
// map is an array of CPUs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rps_map {
    pub len: c_uint,
    pub rcu: rcu_head,
    pub cpus: [u16; ],
}

//
// The rps_dev_flow structure contains the mapping of a flow to a CPU, the
// tail pointer for that CPU's input queue at the time of last enqueue, a
// hardware filter index, and the hash of the flow if aRFS is enabled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rps_dev_flow {
    pub cpu: u16,
    pub filter: u16,
    pub last_qtail: c_uint,

    pub hash: u32,

}

pub const RPS_NO_FILTER: c_uint = 0xffff;
//
// The rps_sock_flow_table contains mappings of flows to the last CPU
// on which they were processed by the application (set in recvmsg).
// Each entry is a 32bit value. Upper part is the high-order bits
// of flow hash, lower part is CPU number.
// rps_cpu_mask is used to partition the space, depending on number of
// possible CPUs : rps_cpu_mask = roundup_pow_of_two(nr_cpu_ids) - 1
// For example, if 64 CPUs are possible, rps_cpu_mask = 0x3f,
// meaning we use 32-6=26 bits for the hash.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rps_sock_flow_table {
    pub ent: u32,
}

pub const RPS_NO_CPU: c_uint = 0xffff;
// We only give a hint, preemption can change CPU under us
// The following WRITE_ONCE() is paired with the READ_ONCE()
// here, and another one in get_rps_cpu().
//
// Reading sk->sk_rxhash might incur an expensive cache line
// miss.
//
// TCP_ESTABLISHED does cover almost all states where RFS
// might be useful, and is cheaper [1] than testing :
// IPv4: inet_sk(sk)->inet_daddr
// IPv6: ipv6_addr_any(&sk->sk_v6_daddr)
// OR	an additional socket flag
// [1] : sk_state and sk_prot are in the same cache line.
//
// This READ_ONCE() is paired with the WRITE_ONCE()
// from sock_rps_save_rxhash() and sock_rps_reset_rxhash().
//

extern "C" {
    pub fn static_branch_unlikely(_arg: &rfs_needed) -> return;
}

