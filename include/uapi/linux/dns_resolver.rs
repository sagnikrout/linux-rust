//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dns_resolver.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
// DNS resolver interface definitions.
//
// Copyright (C) 2018 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public Licence
// as published by the Free Software Foundation; either version
// 2 of the Licence, or (at your option) any later version.
//

//
// Type of payload.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dns_payload_content_type {
    DNS_PAYLOAD_IS_SERVER_LIST	= 0, /* List of servers, requested by srv=1 */
}

//
// Type of address that might be found in an address record.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dns_payload_address_type {
    DNS_ADDRESS_IS_IPV4		= 0, /* 4-byte AF_INET address */
    DNS_ADDRESS_IS_IPV6		= 1, /* 16-byte AF_INET6 address */
}

//
// Type of protocol used to access a server.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dns_payload_protocol_type {
    DNS_SERVER_PROTOCOL_UNSPECIFIED	= 0,
    DNS_SERVER_PROTOCOL_UDP		= 1, /* Use UDP to talk to the server */
    DNS_SERVER_PROTOCOL_TCP		= 2, /* Use TCP to talk to the server */
}

//
// Source of record included in DNS resolver payload.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dns_record_source {
    DNS_RECORD_UNAVAILABLE		= 0, /* No source available (empty record) */
    DNS_RECORD_FROM_CONFIG		= 1, /* From local configuration data */
    DNS_RECORD_FROM_DNS_A		= 2, /* From DNS A or AAAA record */
    DNS_RECORD_FROM_DNS_AFSDB	= 3, /* From DNS AFSDB record */
    DNS_RECORD_FROM_DNS_SRV		= 4, /* From DNS SRV record */
    DNS_RECORD_FROM_NSS		= 5, /* From NSS */
    NR__dns_record_source
}

//
// Status of record included in DNS resolver payload.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dns_lookup_status {
    DNS_LOOKUP_NOT_DONE		= 0, /* No lookup has been made */
    DNS_LOOKUP_GOOD			= 1, /* Good records obtained */
    DNS_LOOKUP_GOOD_WITH_BAD	= 2, /* Good records, some decoding errors */
    DNS_LOOKUP_BAD			= 3, /* Couldn't decode results */
    DNS_LOOKUP_GOT_NOT_FOUND	= 4, /* Got a "Not Found" result */
    DNS_LOOKUP_GOT_LOCAL_FAILURE	= 5, /* Local failure during lookup */
    DNS_LOOKUP_GOT_TEMP_FAILURE	= 6, /* Temporary failure during lookup */
    DNS_LOOKUP_GOT_NS_FAILURE	= 7, /* Name server failure */
    NR__dns_lookup_status
}

//
// Header at the beginning of binary format payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dns_payload_header {
    pub /: *mut *mut __u8 zero; / Zero byte: marks this as not being text,
    pub /: *mut *mut __u8 content; / enum dns_payload_content_type,
    pub /: *mut *mut __u8 version; / Encoding version,
    pub __packed: },
//
// Header at the beginning of a V1 server list.  This is followed directly by
// the server records.  Each server records begins with a struct of type
// dns_server_list_v1_server.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dns_server_list_v1_header {
    pub hdr: dns_payload_header,
    pub /: *mut *mut __u8 source; / enum dns_record_source,
    pub /: *mut *mut __u8 status; / enum dns_lookup_status,
    pub /: *mut *mut __u8 nr_servers; / Number of server records following this,
    pub __packed: },
//
// Header at the beginning of each V1 server record.  This is followed by the
// characters of the name with no NUL-terminator, followed by the address
// records for that server.  Each address record begins with a struct of type
// struct dns_server_list_v1_address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dns_server_list_v1_server {
    pub /: *mut *mut __u16 name_len; / Length of name (LE),
    pub /: *mut *mut __u16 priority; / Priority (as SRV record) (LE),
    pub /: *mut *mut __u16 weight; / Weight (as SRV record) (LE),
    pub /: *mut *mut __u16 port; / UDP/TCP port number (LE),
    pub /: *mut *mut __u8 source; / enum dns_record_source,
    pub /: *mut *mut __u8 status; / enum dns_lookup_status,
    pub /: *mut *mut __u8 protocol; / enum dns_payload_protocol_type,
    pub nr_addrs: __u8,
    pub __packed: },
//
// Header at the beginning of each V1 address record.  This is followed by the
// bytes of the address, 4 for IPV4 and 16 for IPV6.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dns_server_list_v1_address {
    pub /: *mut *mut __u8 address_type; / enum dns_payload_address_type,
    pub __packed: },
