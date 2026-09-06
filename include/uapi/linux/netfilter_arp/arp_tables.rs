//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_arp/arp_tables.h
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
// Format of an ARP firewall descriptor
//
// src, tgt, src_mask, tgt_mask, arpop, arpop_mask are always stored in
// network byte order.
// flags are stored in host byte order (of course).
//

pub const ARPT_DEV_ADDR_LEN_MAX: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpt_devaddr_info {
    pub addr: [c_char; ARPT_DEV_ADDR_LEN_MAX],
    pub mask: [c_char; ARPT_DEV_ADDR_LEN_MAX],
}

// Yes, Virginia, you have to zero the padding.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpt_arp {
// Source and target IP addr
    pub tgt: in_addr src,,
// Mask for src and target IP addr
    pub tmsk: in_addr smsk,,
// Device hw address length, src+target device addresses
    pub arhln_mask: __u8 arhln,,
    pub src_devaddr: arpt_devaddr_info,
    pub tgt_devaddr: arpt_devaddr_info,
// ARP operation code.
    pub arpop_mask: __be16 arpop,,
// ARP hardware address and protocol address format.
    pub arhrd_mask: __be16 arhrd,,
    pub arpro_mask: __be16 arpro,,
// The protocol address length is only accepted if it is 4
// so there is no use in offering a way to do filtering on it.
//
    pub outiface: [char iniface[IFNAMSIZ],; IFNAMSIZ],
    pub outiface_mask: [unsigned char iniface_mask[IFNAMSIZ],; IFNAMSIZ],
// Flags word
    pub flags: __u8,
// Inverse flags
    pub invflags: __u16,
}

// Values for "flag" field in struct arpt_ip (general arp structure).
// No flags defined yet.
//
pub const ARPT_F_MASK: c_uint = 0x00	/* All possible flag bits mask. */;
// Values for "inv" field in struct arpt_arp.
pub const ARPT_INV_VIA_IN: c_uint = 0x0001	/* Invert the sense of IN IFACE. */;
pub const ARPT_INV_VIA_OUT: c_uint = 0x0002	/* Invert the sense of OUT IFACE */;
pub const ARPT_INV_SRCIP: c_uint = 0x0004	/* Invert the sense of SRC IP. */;
pub const ARPT_INV_TGTIP: c_uint = 0x0008	/* Invert the sense of TGT IP. */;
pub const ARPT_INV_SRCDEVADDR: c_uint = 0x0010	/* Invert the sense of SRC DEV ADDR. */;
pub const ARPT_INV_TGTDEVADDR: c_uint = 0x0020	/* Invert the sense of TGT DEV ADDR. */;
pub const ARPT_INV_ARPOP: c_uint = 0x0040	/* Invert the sense of ARP OP. */;
pub const ARPT_INV_ARPHRD: c_uint = 0x0080	/* Invert the sense of ARP HRD. */;
pub const ARPT_INV_ARPPRO: c_uint = 0x0100	/* Invert the sense of ARP PRO. */;
pub const ARPT_INV_ARPHLN: c_uint = 0x0200	/* Invert the sense of ARP HLN. */;
pub const ARPT_INV_MASK: c_uint = 0x03FF	/* All possible flag bits mask. */;
// This structure defines each of the firewall rules.  Consists of 3
// Size of arpt_entry + matches
// Size of arpt_entry + matches + target
// Back pointer
// Packet and byte counters.
// The matches (if any), then the target.
//
// New IP firewall options for [gs]etsockopt at the RAW IP level.
// Unlike BSD Linux inherits IP options so you don't have to use a raw
// socket for this. Instead we check rights in the calls.
//
// ATTENTION: check linux/in.h before adding new number here.
//
pub const ARPT_BASE_CTL: c_int = 96;

// #define ARPT_SO_GET_REVISION_MATCH	(APRT_BASE_CTL + 2)

// The argument to ARPT_SO_GET_INFO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpt_getinfo {
// Which table: caller fills this in.
    pub name: [c_char; XT_TABLE_MAXNAMELEN],
// Kernel fills these in.
// Which hook entry points are valid: bitmask
    pub valid_hooks: c_uint,
// Hook entry points: one per netfilter hook.
    pub hook_entry: [c_uint; NF_ARP_NUMHOOKS],
// Underflow points.
    pub underflow: [c_uint; NF_ARP_NUMHOOKS],
// Number of entries
    pub num_entries: c_uint,
// Size of entries.
    pub size: c_uint,
}

// The argument to ARPT_SO_SET_REPLACE.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpt_replace {
// Which table.
    pub name: [c_char; XT_TABLE_MAXNAMELEN],
// Which hook entry points are valid: bitmask.  You can't
    pub valid_hooks: c_uint,
// Number of entries
    pub num_entries: c_uint,
// Total size of new entries
    pub size: c_uint,
// Hook entry points.
    pub hook_entry: [c_uint; NF_ARP_NUMHOOKS],
// Underflow points.
    pub underflow: [c_uint; NF_ARP_NUMHOOKS],
// Information about old entries:
// Number of counters (must be equal to current number of entries).
    pub num_counters: c_uint,
// The old entries' counters.
    pub counters: *mut xt_counters __user,
// The entries (hang off end: not really an array).
    pub entries: [arpt_entry; ],
}

// The argument to ARPT_SO_GET_ENTRIES.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpt_get_entries {
// Which table: user fills this in.
    pub name: [c_char; XT_TABLE_MAXNAMELEN],
// User fills this in: total entry size.
    pub size: c_uint,
// The entries.
    pub entrytable: [arpt_entry; ],
}

// Helper functions
//
// Main firewall chains definitions and global var's definitions.
//
