//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/qeth.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// ioctl definitions for qeth driver
//
// Copyright IBM Corp. 2004
//
// Author(s):	Thomas Spatzier <tspat@de.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_arp_cache_entry {
    pub macaddr: [__u8; 6],
    pub reserved1: [__u8; 2],
    pub /: *mut *mut __u8 ipaddr[16]; / for both IPv4 and IPv6,
    pub reserved2: [__u8; 32],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_arp_ipaddrtype {
    QETHARP_IP_ADDR_V4 = 1,
    QETHARP_IP_ADDR_V6 = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_arp_entrytype {
    pub mac: __u8,
    pub ip: __u8,
    pub __attribute__((packed)): },
pub const QETH_QARP_MEDIASPECIFIC_BYTES: c_int = 32;
pub const QETH_QARP_MACADDRTYPE_BYTES: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_arp_qi_entry7 {
    pub media_specific: [__u8; QETH_QARP_MEDIASPECIFIC_BYTES],
    pub type: qeth_arp_entrytype,
    pub macaddr: [__u8; 6],
    pub ipaddr: [__u8; 4],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_arp_qi_entry7_ipv6 {
    pub media_specific: [__u8; QETH_QARP_MEDIASPECIFIC_BYTES],
    pub type: qeth_arp_entrytype,
    pub macaddr: [__u8; 6],
    pub ipaddr: [__u8; 16],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_arp_qi_entry7_short {
    pub type: qeth_arp_entrytype,
    pub macaddr: [__u8; 6],
    pub ipaddr: [__u8; 4],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_arp_qi_entry7_short_ipv6 {
    pub type: qeth_arp_entrytype,
    pub macaddr: [__u8; 6],
    pub ipaddr: [__u8; 16],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_arp_qi_entry5 {
    pub media_specific: [__u8; QETH_QARP_MEDIASPECIFIC_BYTES],
    pub type: qeth_arp_entrytype,
    pub ipaddr: [__u8; 4],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_arp_qi_entry5_ipv6 {
    pub media_specific: [__u8; QETH_QARP_MEDIASPECIFIC_BYTES],
    pub type: qeth_arp_entrytype,
    pub ipaddr: [__u8; 16],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_arp_qi_entry5_short {
    pub type: qeth_arp_entrytype,
    pub ipaddr: [__u8; 4],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_arp_qi_entry5_short_ipv6 {
    pub type: qeth_arp_entrytype,
    pub ipaddr: [__u8; 16],
    pub __attribute__((packed)): },
//
// can be set by user if no "media specific information" is wanted
// -> saves a lot of space in user space buffer
//
pub const QETH_QARP_STRIP_ENTRIES: c_uint = 0x8000;
pub const QETH_QARP_WITH_IPV6: c_uint = 0x4000;
pub const QETH_QARP_REQUEST_MASK: c_uint = 0x00ff;
// data sent to user space as result of query arp ioctl
pub const QETH_QARP_USER_DATA_SIZE: c_int = 20000;
pub const QETH_QARP_MASK_OFFSET: c_int = 4;
pub const QETH_QARP_ENTRIES_OFFSET: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_arp_query_user_data {
    pub /: *mut *mut __u32 data_len; / set by user space program,
    pub /: *mut *mut __u32 no_entries; / set by kernel,
    pub u: },
    pub mask_bits: __u16,
    pub entries: *mut c_char,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_query_oat_data {
    pub command: __u32,
    pub buffer_len: __u32,
    pub response_len: __u32,
    pub ptr: __u64,
}
