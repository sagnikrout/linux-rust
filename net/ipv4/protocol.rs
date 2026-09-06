//! Automatically rewritten from C to Rust
//! Source: net/ipv4/protocol.c
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
// INET protocol dispatch tables.
//
// Authors:	Ross Biro
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//
// Fixes:
// Alan Cox	: Ahah! udp icmp errors don't work because
// udp_err is never called!
// Alan Cox	: Added new fields for init and ready for
// proper fragmentation (_NO_ 4K limits!)
// Richard Colella	: Hang on hash collision
// Vince Laviano	: Modified inet_del_protocol() to correctly
// maintain copy bit.
//

    struct net_protocol __rcu *inet_protos[MAX_INET_PROTOS] __read_mostly;
    EXPORT_SYMBOL(inet_protos);
    const struct net_offload __rcu *inet_offloads[MAX_INET_PROTOS] __read_mostly;
    EXPORT_SYMBOL(inet_offloads);
#[no_mangle]
pub unsafe extern "C" fn inet_add_protocol(prot: *const net_protocol, protocol: c_uchar) -> c_int {
    int inet_add_protocol(const struct net_protocol *prot, unsigned char protocol)
    {
    return !cmpxchg((const struct net_protocol **)&inet_protos[protocol],
    core::ptr::null_mut(), prot) ? 0 : -1;
    }
    EXPORT_SYMBOL(inet_add_protocol);
#[no_mangle]
pub unsafe extern "C" fn inet_add_offload(prot: *const net_offload, protocol: c_uchar) -> c_int {
    int inet_add_offload(const struct net_offload *prot, unsigned char protocol)
    {
    return !cmpxchg((const struct net_offload **)&inet_offloads[protocol],
    core::ptr::null_mut(), prot) ? 0 : -1;
    }
    EXPORT_SYMBOL(inet_add_offload);
#[no_mangle]
pub unsafe extern "C" fn inet_del_protocol(prot: *const net_protocol, protocol: c_uchar) -> c_int {
    int inet_del_protocol(const struct net_protocol *prot, unsigned char protocol)
    {
    int ret;
    ret = (cmpxchg((const struct net_protocol **)&inet_protos[protocol],
    prot, core::ptr::null_mut()) == prot) ? 0 : -1;
    synchronize_net();
    return ret;
    }
    EXPORT_SYMBOL(inet_del_protocol);
#[no_mangle]
pub unsafe extern "C" fn inet_del_offload(prot: *const net_offload, protocol: c_uchar) -> c_int {
    int inet_del_offload(const struct net_offload *prot, unsigned char protocol)
    {
    int ret;
    ret = (cmpxchg((const struct net_offload **)&inet_offloads[protocol],
    prot, core::ptr::null_mut()) == prot) ? 0 : -1;
    synchronize_net();
    return ret;
    }
    EXPORT_SYMBOL(inet_del_offload);
