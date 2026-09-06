//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_bridge/ebtables.h
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
// ebtables
//
// Authors:
// Bart De Schuymer		<bdschuym@pandora.be>
//
// ebtables.c,v 2.0, April, 2002
//
// This code is strongly inspired by the iptables code which is
// Copyright (C) 1999 Paul `Rusty' Russell & Michael J. Neuling
//

pub const EBT_TABLE_MAXNAMELEN: c_int = 32;

pub const EBT_EXTENSION_MAXNAMELEN: c_int = 31;
// verdicts >0 are "branches"

pub const NUM_STANDARD_TARGETS: c_int = 4;
// ebtables target modules store the verdict inside an int. We can
// reclaim a part of this int for backwards compatible extensions.
// The 4 lsb are more than enough to store the verdict.
pub const EBT_VERDICT_BITS: c_uint = 0x0000000F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_counter {
    pub pcnt: __u64,
    pub bcnt: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_replace {
    pub name: [c_char; EBT_TABLE_MAXNAMELEN],
    pub valid_hooks: c_uint,
// nr of rules in the table
    pub nentries: c_uint,
// total size of the entries
    pub entries_size: c_uint,
// start of the chains
    pub hook_entry: [*mut ebt_entries __user; NF_BR_NUMHOOKS],
// nr of counters userspace expects back
    pub num_counters: c_uint,
// where the kernel will put the old counters
    pub counters: *mut ebt_counter __user,
    pub entries: *mut char __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_replace_kernel {
    pub name: [c_char; EBT_TABLE_MAXNAMELEN],
    pub valid_hooks: c_uint,
// nr of rules in the table
    pub nentries: c_uint,
// total size of the entries
    pub entries_size: c_uint,
// start of the chains
    pub hook_entry: [*mut ebt_entries; NF_BR_NUMHOOKS],
// nr of counters userspace expects back
    pub num_counters: c_uint,
// where the kernel will put the old counters
    pub counters: *mut ebt_counter,
    pub entries: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_entries {
// this field is always set to zero
// See EBT_ENTRY_OR_ENTRIES.
// Must be same size as ebt_entry.bitmask
    pub distinguisher: c_uint,
// the chain name
    pub name: [c_char; EBT_CHAIN_MAXNAMELEN],
// counter offset for this chain
    pub counter_offset: c_uint,
// one standard (accept, drop, return) per hook
    pub policy: c_int,
// nr. of entries
    pub nentries: c_uint,
// entry list
// C attribute field omitted
}

// used for the bitmask of struct ebt_entry
// This is a hack to make a difference between an ebt_entry struct and an
// ebt_entries struct when traversing the entries from start to end.
// Using this simplifies the code a lot, while still being able to use
// ebt_entries.
// Contrary, iptables doesn't use something like ebt_entries and therefore uses
// different techniques for naming the policy and such. So, iptables doesn't
// need a hack like this.
//
pub const EBT_ENTRY_OR_ENTRIES: c_uint = 0x01;
// these are the normal masks
pub const EBT_NOPROTO: c_uint = 0x02;
pub const EBT_802_3: c_uint = 0x04;
pub const EBT_SOURCEMAC: c_uint = 0x08;
pub const EBT_DESTMAC: c_uint = 0x10;

pub const EBT_IPROTO: c_uint = 0x01;
pub const EBT_IIN: c_uint = 0x02;
pub const EBT_IOUT: c_uint = 0x04;
pub const EBT_ISOURCE: c_uint = 0x8;
pub const EBT_IDEST: c_uint = 0x10;
pub const EBT_ILOGICALIN: c_uint = 0x20;
pub const EBT_ILOGICALOUT: c_uint = 0x40;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_entry_match {
    pub name: [c_char; EBT_EXTENSION_MAXNAMELEN],
    pub revision: __u8,
}

// size of data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_entry_watcher {
    pub name: [c_char; EBT_EXTENSION_MAXNAMELEN],
    pub revision: __u8,
}

// size of data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_entry_target {
    pub name: [c_char; EBT_EXTENSION_MAXNAMELEN],
    pub revision: __u8,
}

// size of data

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_standard_target {
    pub target: ebt_entry_target,
    pub verdict: c_int,
}

// one entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_entry {
// this needs to be the first field
    pub bitmask: c_uint,
    pub invflags: c_uint,
    pub ethproto: __be16,
// the physical in-dev
    pub in: [c_char; IFNAMSIZ],
// the logical in-dev
    pub logical_in: [c_char; IFNAMSIZ],
// the physical out-dev
    pub out: [c_char; IFNAMSIZ],
// the logical out-dev
    pub logical_out: [c_char; IFNAMSIZ],
    pub sourcemac: [c_uchar; ETH_ALEN],
    pub sourcemsk: [c_uchar; ETH_ALEN],
    pub destmac: [c_uchar; ETH_ALEN],
    pub destmsk: [c_uchar; ETH_ALEN],
// sizeof ebt_entry + matches
    pub watchers_offset: c_uint,
// sizeof ebt_entry + matches + watchers
    pub target_offset: c_uint,
// sizeof ebt_entry + matches + watchers + target
    pub next_offset: c_uint,
// C attribute field omitted
}

// {g,s}etsockopt numbers
pub const EBT_BASE_CTL: c_int = 128;

// blatently stolen from ip_tables.h
// fn returns 0 to continue iteration

