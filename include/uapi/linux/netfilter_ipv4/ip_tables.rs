//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_ipv4/ip_tables.h
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
// 25-Jul-1998 Major changes to allow for ip chain table
//
// 3-Jan-2000 Named tables to allow packet selection for different uses.
//
// Format of an IP firewall descriptor
//
// src, dst, src_mask, dst_mask are always stored in network byte order.
// flags are stored in host byte order (of course).
// Port numbers are stored in HOST byte order.
//

// This group is older than old (iptables < v1.4.0-rc1~89)

// The argument to IPT_SO_ADD_COUNTERS.

// Standard return verdict, or do jump.

// Error verdict.

// fn returns 0 to continue iteration

// fn returns 0 to continue iteration

// Yes, Virginia, you have to zero the padding.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipt_ip {
// Source and destination IP addr
    pub dst: in_addr src,,
// Mask for src and dest IP addr
    pub dmsk: in_addr smsk,,
    pub outiface: [char iniface[IFNAMSIZ],; IFNAMSIZ],
    pub outiface_mask: [unsigned char iniface_mask[IFNAMSIZ],; IFNAMSIZ],
// Protocol, 0 = ANY
    pub proto: __u16,
// Flags word
    pub flags: __u8,
// Inverse flags
    pub invflags: __u8,
}

// Values for "flag" field in struct ipt_ip (general ip structure).
pub const IPT_F_FRAG: c_uint = 0x01	/* Set if rule is a fragment rule */;
pub const IPT_F_GOTO: c_uint = 0x02	/* Set if jump is a goto */;
pub const IPT_F_MASK: c_uint = 0x03	/* All possible flag bits mask. */;
// Values for "inv" field in struct ipt_ip.
pub const IPT_INV_VIA_IN: c_uint = 0x01	/* Invert the sense of IN IFACE. */;
pub const IPT_INV_VIA_OUT: c_uint = 0x02	/* Invert the sense of OUT IFACE */;
pub const IPT_INV_TOS: c_uint = 0x04	/* Invert the sense of TOS. */;
pub const IPT_INV_SRCIP: c_uint = 0x08	/* Invert the sense of SRC IP. */;
pub const IPT_INV_DSTIP: c_uint = 0x10	/* Invert the sense of DST OP. */;
pub const IPT_INV_FRAG: c_uint = 0x20	/* Invert the sense of FRAG. */;

pub const IPT_INV_MASK: c_uint = 0x7F	/* All possible flag bits mask. */;
// This structure defines each of the firewall rules.  Consists of 3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipt_entry {
    pub ip: ipt_ip,
// Mark with fields that we care about.
    pub nfcache: c_uint,
// Size of ipt_entry + matches
    pub target_offset: __u16,
// Size of ipt_entry + matches + target
    pub next_offset: __u16,
// Back pointer
    pub comefrom: c_uint,
// Packet and byte counters.
    pub counters: xt_counters,
// The matches (if any), then the target.
    pub elems: [c_uchar; ],
}

//
// New IP firewall options for [gs]etsockopt at the RAW IP level.
// Unlike BSD Linux inherits IP options so you don't have to use a raw
// socket for this. Instead we check rights in the calls.
//
// ATTENTION: check linux/in.h before adding new number here.
//
pub const IPT_BASE_CTL: c_int = 64;

// ICMP matching stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipt_icmp {
    pub /: *mut *mut __u8 type; / type to match,
    pub /: *mut *mut __u8 code[2]; / range of code,
    pub /: *mut *mut __u8 invflags; / Inverse flags,
}

// Values for "inv" field for struct ipt_icmp.
pub const IPT_ICMP_INV: c_uint = 0x01	/* Invert the sense of type/code test */;
// The argument to IPT_SO_GET_INFO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipt_getinfo {
// Which table: caller fills this in.
    pub name: [c_char; XT_TABLE_MAXNAMELEN],
// Kernel fills these in.
// Which hook entry points are valid: bitmask
    pub valid_hooks: c_uint,
// Hook entry points: one per netfilter hook.
    pub hook_entry: [c_uint; NF_INET_NUMHOOKS],
// Underflow points.
    pub underflow: [c_uint; NF_INET_NUMHOOKS],
// Number of entries
    pub num_entries: c_uint,
// Size of entries.
    pub size: c_uint,
}

// The argument to IPT_SO_SET_REPLACE.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipt_replace {
// Which table.
    pub name: [c_char; XT_TABLE_MAXNAMELEN],
// Which hook entry points are valid: bitmask.  You can't
    pub valid_hooks: c_uint,
// Number of entries
    pub num_entries: c_uint,
// Total size of new entries
    pub size: c_uint,
// Hook entry points.
    pub hook_entry: [c_uint; NF_INET_NUMHOOKS],
// Underflow points.
    pub underflow: [c_uint; NF_INET_NUMHOOKS],
// Information about old entries:
// Number of counters (must be equal to current number of entries).
    pub num_counters: c_uint,
// The old entries' counters.
    pub counters: *mut xt_counters __user,
// The entries (hang off end: not really an array).
    pub entries: [ipt_entry; ],
}

// The argument to IPT_SO_GET_ENTRIES.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipt_get_entries {
// Which table: user fills this in.
    pub name: [c_char; XT_TABLE_MAXNAMELEN],
// User fills this in: total entry size.
    pub size: c_uint,
// The entries.
    pub entrytable: [ipt_entry; ],
}

// Helper functions
//
// Main firewall chains definitions and global var's definitions.
//
