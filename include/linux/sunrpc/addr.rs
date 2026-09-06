//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/addr.h
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
//
// linux/include/linux/sunrpc/addr.h
//
// Various routines for copying and comparing sockaddrs and for
// converting them to and from presentation format.
//

extern "C" {
    pub fn rpc_ntop(: *const sockaddr, : *mut c_char, size_t: const) -> usize;
}
extern "C" {
    pub fn rpc_sockaddr2uaddr(: *const sockaddr, _arg: gfp_t) -> *mut c_char;
}
extern "C" {
    pub fn ntohs()sap)->sin_port: *mut ((struct sockaddr_in) -> return;
}
extern "C" {
    pub fn ntohs()sap)->sin6_port: *mut ((struct sockaddr_in6) -> return;
}

//
// rpc_cmp_addr - compare the address portion of two sockaddrs.
// @sap1: first sockaddr
// @sap2: second sockaddr
//
// Just compares the family and address portion. Ignores port, but
// compares the scope if it's a link-local address.
//
// Returns true if the addrs are equal, false if they aren't.
//
extern "C" {
    pub fn rpc_cmp_addr4(_arg: sap1, _arg: sap2) -> return;
}
extern "C" {
    pub fn rpc_cmp_addr6(_arg: sap1, _arg: sap2) -> return;
}
//
// rpc_cmp_addr_port - compare the address and port number of two sockaddrs.
// @sap1: first sockaddr
// @sap2: second sockaddr
//
extern "C" {
    pub fn rpc_get_port(rpc_get_port(sap2: sap1) ==) -> return;
}
//
// rpc_copy_addr - copy the address portion of one sockaddr to another
// @dst: destination sockaddr
// @src: source sockaddr
//
// Just copies the address portion and family. Ignores port, scope, etc.
// Caller is responsible for making certain that dst is large enough to hold
// the address in src. Returns true if address family is supported. Returns
// false otherwise.
//
extern "C" {
    pub fn __rpc_copy_addr4(_arg: dst, _arg: src) -> return;
}
extern "C" {
    pub fn __rpc_copy_addr6(_arg: dst, _arg: src) -> return;
}
//
// rpc_get_scope_id - return scopeid for a given sockaddr
// @sa: sockaddr to get scopeid from
//
// Returns the value of the sin6_scope_id for AF_INET6 addrs, or 0 if
// not an AF_INET6 address.
//
