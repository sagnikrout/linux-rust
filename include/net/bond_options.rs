//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bond_options.h
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
// drivers/net/bond/bond_options.h - bonding options
// Copyright (c) 2013 Nikolay Aleksandrov <nikolay@redhat.com>
//

pub const BOND_OPT_MAX_NAMELEN: c_int = 32;

// Option flags:
// BOND_OPTFLAG_NOSLAVES - check if the bond device is empty before setting
// BOND_OPTFLAG_IFDOWN - check if the bond device is down before setting
// BOND_OPTFLAG_RAWVAL - the option parses the value itself
//
// Value type flags:
// BOND_VALFLAG_DEFAULT - mark the value as default
// BOND_VALFLAG_(MIN|MAX) - mark the value as min/max
//
// Option IDs, their bit positions correspond to their IDs
// This structure is used for storing option values and for passing option
// values when changing an option. The logic when used as an arg is as follows:
// - if value != ULLONG_MAX -> parse value
// - if string != NULL -> parse string
// - if the opt is RAW data and length less than maxlen,
// copy the data to extra storage
//
pub const BOND_OPT_EXTRA_MAXLEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bond_opt_value {
    pub string: *mut c_char,
    pub value: u64,
    pub flags: u32,
    pub extra: [c_char; BOND_OPT_EXTRA_MAXLEN],
    pub slave_dev: *mut net_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bond_option {
    pub id: c_int,
    pub name: *const c_char,
    pub desc: *const c_char,
    pub flags: u32,
// unsuppmodes is used to denote modes in which the option isn't
// supported.
//
    pub unsuppmodes: c_ulong,
// supported values which this option can have, can be a subset of
// BOND_OPTVAL_RANGE's value range
//
    pub values: *const bond_opt_value,
    pub val): *const *const *const int (set)(struct bonding bond, struct bond_opt_value,
}

extern "C" {
    pub fn bond_opt_tryset_rtnl(bond: *mut bonding, option: c_uint, buf: *mut c_char) -> c_int;
}
// This helper is used to initialize a bond_opt_value structure for parameter
// passing. There should be either a valid string or value, but not both.
// When value is ULLONG_MAX then string will be used.
//

extern "C" {
    pub fn bond_option_arp_ip_targets_clear(bond: *mut bonding);
}

extern "C" {
    pub fn bond_option_ns_ip6_targets_clear(bond: *mut bonding);
}

extern "C" {
    pub fn bond_slave_ns_maddrs_add(bond: *mut bonding, slave: *mut slave);
}
extern "C" {
    pub fn bond_slave_ns_maddrs_del(bond: *mut bonding, slave: *mut slave);
}
