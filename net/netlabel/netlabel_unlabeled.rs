//! Automatically rewritten from C Header to Rust Module
//! Source: net/netlabel/netlabel_unlabeled.h
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
// NetLabel Unlabeled Support
//
// This file defines functions for dealing with unlabeled packets for the
// NetLabel system.  The NetLabel system manages static and dynamic label
// mappings for network protocols such as CIPSO and RIPSO.
//
// Author: Paul Moore <paul@paul-moore.com>
//
// (c) Copyright Hewlett-Packard Development Company, L.P., 2006
//

//
// The following NetLabel payloads are supported by the Unlabeled subsystem.
//
// o STATICADD
// This message is sent from an application to add a new static label for
// incoming unlabeled connections.
//
// Required attributes:
//
// NLBL_UNLABEL_A_IFACE
// NLBL_UNLABEL_A_SECCTX
//
// If IPv4 is specified the following attributes are required:
//
// NLBL_UNLABEL_A_IPV4ADDR
// NLBL_UNLABEL_A_IPV4MASK
//
// If IPv6 is specified the following attributes are required:
//
// NLBL_UNLABEL_A_IPV6ADDR
// NLBL_UNLABEL_A_IPV6MASK
//
// o STATICREMOVE
// This message is sent from an application to remove an existing static
// label for incoming unlabeled connections.
//
// Required attributes:
//
// NLBL_UNLABEL_A_IFACE
//
// If IPv4 is specified the following attributes are required:
//
// NLBL_UNLABEL_A_IPV4ADDR
// NLBL_UNLABEL_A_IPV4MASK
//
// If IPv6 is specified the following attributes are required:
//
// NLBL_UNLABEL_A_IPV6ADDR
// NLBL_UNLABEL_A_IPV6MASK
//
// o STATICLIST
// This message can be sent either from an application or by the kernel in
// response to an application generated STATICLIST message.  When sent by an
// application there is no payload and the NLM_F_DUMP flag should be set.
// The kernel should response with a series of the following messages.
//
// Required attributes:
//
// NLBL_UNLABEL_A_IFACE
// NLBL_UNLABEL_A_SECCTX
//
// If IPv4 is specified the following attributes are required:
//
// NLBL_UNLABEL_A_IPV4ADDR
// NLBL_UNLABEL_A_IPV4MASK
//
// If IPv6 is specified the following attributes are required:
//
// NLBL_UNLABEL_A_IPV6ADDR
// NLBL_UNLABEL_A_IPV6MASK
//
// o STATICADDDEF
// This message is sent from an application to set the default static
// label for incoming unlabeled connections.
//
// Required attribute:
//
// NLBL_UNLABEL_A_SECCTX
//
// If IPv4 is specified the following attributes are required:
//
// NLBL_UNLABEL_A_IPV4ADDR
// NLBL_UNLABEL_A_IPV4MASK
//
// If IPv6 is specified the following attributes are required:
//
// NLBL_UNLABEL_A_IPV6ADDR
// NLBL_UNLABEL_A_IPV6MASK
//
// o STATICREMOVEDEF
// This message is sent from an application to remove the existing default
// static label for incoming unlabeled connections.
//
// If IPv4 is specified the following attributes are required:
//
// NLBL_UNLABEL_A_IPV4ADDR
// NLBL_UNLABEL_A_IPV4MASK
//
// If IPv6 is specified the following attributes are required:
//
// NLBL_UNLABEL_A_IPV6ADDR
// NLBL_UNLABEL_A_IPV6MASK
//
// o STATICLISTDEF
// This message can be sent either from an application or by the kernel in
// response to an application generated STATICLISTDEF message.  When sent by
// an application there is no payload and the NLM_F_DUMP flag should be set.
// The kernel should response with the following message.
//
// Required attribute:
//
// NLBL_UNLABEL_A_SECCTX
//
// If IPv4 is specified the following attributes are required:
//
// NLBL_UNLABEL_A_IPV4ADDR
// NLBL_UNLABEL_A_IPV4MASK
//
// If IPv6 is specified the following attributes are required:
//
// NLBL_UNLABEL_A_IPV6ADDR
// NLBL_UNLABEL_A_IPV6MASK
//
// o ACCEPT
// This message is sent from an application to specify if the kernel should
// allow unlabled packets to pass if they do not match any of the static
// mappings defined in the unlabeled module.
//
// Required attributes:
//
// NLBL_UNLABEL_A_ACPTFLG
//
// o LIST
// This message can be sent either from an application or by the kernel in
// response to an application generated LIST message.  When sent by an
// application there is no payload.  The kernel should respond to a LIST
// message with a LIST message on success.
//
// Required attributes:
//
// NLBL_UNLABEL_A_ACPTFLG
//
// NetLabel Unlabeled commands
// NetLabel Unlabeled attributes
// (NLA_U8)
// if true then unlabeled packets are allowed to pass, else unlabeled
// packets are rejected
// (NLA_BINARY, struct in6_addr)
// an IPv6 address
// (NLA_BINARY, struct in6_addr)
// an IPv6 address mask
// (NLA_BINARY, struct in_addr)
// an IPv4 address
// (NLA_BINARY, struct in_addr)
// and IPv4 address mask
// (NLA_NULL_STRING)
// network interface
// (NLA_BINARY)
// a LSM specific security context

// NetLabel protocol functions
extern "C" {
    pub fn netlbl_unlabel_genl_init() -> c_int;
}
// Unlabeled connection hash table size
// XXX - currently this number is an uneducated guess
pub const NETLBL_UNLHSH_BITSIZE: c_int = 7;
// General Unlabeled init function
extern "C" {
    pub fn netlbl_unlabel_init(size: u32) -> c_int;
}
// Static/Fallback label management functions
// Process Unlabeled incoming network packets
// Set the default configuration to allow Unlabeled packets
extern "C" {
    pub fn netlbl_unlabel_defconf() -> c_int;
}
