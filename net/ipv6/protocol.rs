//! Automatically rewritten from C to Rust
//! Source: net/ipv6/protocol.c
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
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// PF_INET6 protocol dispatch tables.
//
// Authors:	Pedro Roque	<roque@di.fc.ul.pt>
//
// Changes:
//
// Vince Laviano (vince@cs.stanford.edu)       16 May 2001
// - Removed unused variable 'inet6_protocol_base'
// - Modified inet6_del_protocol() to correctly maintain copy bit.
//

    struct inet6_protocol __rcu *inet6_protos[MAX_INET_PROTOS] __read_mostly;
    EXPORT_SYMBOL(inet6_protos);
#[no_mangle]
pub unsafe extern "C" fn inet6_add_protocol(prot: *const inet6_protocol, protocol: c_uchar) -> c_int {
    int inet6_add_protocol(const struct inet6_protocol *prot, unsigned char protocol)
    {
    return !cmpxchg((const struct inet6_protocol **)&inet6_protos[protocol],
    core::ptr::null_mut(), prot) ? 0 : -1;
    }
    EXPORT_SYMBOL(inet6_add_protocol);
#[no_mangle]
pub unsafe extern "C" fn inet6_del_protocol(prot: *const inet6_protocol, protocol: c_uchar) -> c_int {
    int inet6_del_protocol(const struct inet6_protocol *prot, unsigned char protocol)
    {
    int ret;
    ret = (cmpxchg((const struct inet6_protocol **)&inet6_protos[protocol],
    prot, core::ptr::null_mut()) == prot) ? 0 : -1;
    synchronize_net();
    return ret;
    }
    EXPORT_SYMBOL(inet6_del_protocol);

    const struct net_offload __rcu *inet6_offloads[MAX_INET_PROTOS] __read_mostly;
    EXPORT_SYMBOL(inet6_offloads);
#[no_mangle]
pub unsafe extern "C" fn inet6_add_offload(prot: *const net_offload, protocol: c_uchar) -> c_int {
    int inet6_add_offload(const struct net_offload *prot, unsigned char protocol)
    {
    return !cmpxchg((const struct net_offload **)&inet6_offloads[protocol],
    core::ptr::null_mut(), prot) ? 0 : -1;
    }
    EXPORT_SYMBOL(inet6_add_offload);
#[no_mangle]
pub unsafe extern "C" fn inet6_del_offload(prot: *const net_offload, protocol: c_uchar) -> c_int {
    int inet6_del_offload(const struct net_offload *prot, unsigned char protocol)
    {
    int ret;
    ret = (cmpxchg((const struct net_offload **)&inet6_offloads[protocol],
    prot, core::ptr::null_mut()) == prot) ? 0 : -1;
    synchronize_net();
    return ret;
    }
    EXPORT_SYMBOL(inet6_del_offload);
