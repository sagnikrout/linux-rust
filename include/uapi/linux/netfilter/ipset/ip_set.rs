//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/ipset/ip_set.h
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
// Copyright (C) 2000-2002 Joakim Axelsson <gozem@linux.nu>
// Patrick Schaaf <bof@bof.de>
// Martin Josefsson <gandalf@wlug.westbo.se>
// Copyright (C) 2003-2011 Jozsef Kadlecsik <kadlec@netfilter.org>
//

// The protocol versions
pub const IPSET_PROTOCOL: c_int = 7;
pub const IPSET_PROTOCOL_MIN: c_int = 6;
// The max length of strings including NUL: set and type identifiers
pub const IPSET_MAXNAMELEN: c_int = 32;
// The maximum permissible comment length we will accept over netlink
pub const IPSET_MAX_COMMENT_SIZE: c_int = 255;
// Message types and commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipset_cmd {
    IPSET_CMD_NONE,
    IPSET_CMD_PROTOCOL,	/* 1: Return protocol version */
    IPSET_CMD_CREATE,	/* 2: Create a new (empty) set */
    IPSET_CMD_DESTROY,	/* 3: Destroy a (empty) set */
    IPSET_CMD_FLUSH,	/* 4: Remove all elements from a set */
    IPSET_CMD_RENAME,	/* 5: Rename a set */
    IPSET_CMD_SWAP,		/* 6: Swap two sets */
    IPSET_CMD_LIST,		/* 7: List sets */
    IPSET_CMD_SAVE,		/* 8: Save sets */
    IPSET_CMD_ADD,		/* 9: Add an element to a set */
    IPSET_CMD_DEL,		/* 10: Delete an element from a set */
    IPSET_CMD_TEST,		/* 11: Test an element in a set */
    IPSET_CMD_HEADER,	/* 12: Get set header data only */
    IPSET_CMD_TYPE,		/* 13: Get set type */
    IPSET_CMD_GET_BYNAME,	/* 14: Get set index by name */
    IPSET_CMD_GET_BYINDEX,	/* 15: Get set name by index */
    IPSET_MSG_MAX,		/* Netlink message commands */

// Commands in userspace:
    IPSET_CMD_RESTORE = IPSET_MSG_MAX, /* 16: Enter restore mode */
    IPSET_CMD_HELP,		/* 17: Get help */
    IPSET_CMD_VERSION,	/* 18: Get program version */
    IPSET_CMD_QUIT,		/* 19: Quit from interactive mode */

    IPSET_CMD_MAX,

    IPSET_CMD_COMMIT = IPSET_CMD_MAX, /* 20: Commit buffered commands */
}

// Attributes at command level

// CADT specific attributes
// Reserve empty slots
// Create-only specific attributes
// Kernel-only

// ADT specific attributes

// IP specific attributes

// Error codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipset_errno {
    IPSET_ERR_PRIVATE = 4096,
    IPSET_ERR_PROTOCOL,
    IPSET_ERR_FIND_TYPE,
    IPSET_ERR_MAX_SETS,
    IPSET_ERR_BUSY,
    IPSET_ERR_EXIST_SETNAME2,
    IPSET_ERR_TYPE_MISMATCH,
    IPSET_ERR_EXIST,
    IPSET_ERR_INVALID_CIDR,
    IPSET_ERR_INVALID_NETMASK,
    IPSET_ERR_INVALID_FAMILY,
    IPSET_ERR_TIMEOUT,
    IPSET_ERR_REFERENCED,
    IPSET_ERR_IPADDR_IPV4,
    IPSET_ERR_IPADDR_IPV6,
    IPSET_ERR_COUNTER,
    IPSET_ERR_COMMENT,
    IPSET_ERR_INVALID_MARKMASK,
    IPSET_ERR_SKBINFO,
    IPSET_ERR_BITMASK_NETMASK_EXCL,

// Type specific error codes
    IPSET_ERR_TYPE_SPECIFIC = 4352,
}

// Flags at command level or match/target flags, lower half of cmdattrs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipset_cmd_flags {
    IPSET_FLAG_BIT_EXIST	= 0,
    IPSET_FLAG_EXIST	= (1 << IPSET_FLAG_BIT_EXIST),
    IPSET_FLAG_BIT_LIST_SETNAME = 1,
    IPSET_FLAG_LIST_SETNAME	= (1 << IPSET_FLAG_BIT_LIST_SETNAME),
    IPSET_FLAG_BIT_LIST_HEADER = 2,
    IPSET_FLAG_LIST_HEADER	= (1 << IPSET_FLAG_BIT_LIST_HEADER),
    IPSET_FLAG_BIT_SKIP_COUNTER_UPDATE = 3,
    IPSET_FLAG_SKIP_COUNTER_UPDATE =
    (1 << IPSET_FLAG_BIT_SKIP_COUNTER_UPDATE),
    IPSET_FLAG_BIT_SKIP_SUBCOUNTER_UPDATE = 4,
    IPSET_FLAG_SKIP_SUBCOUNTER_UPDATE =
    (1 << IPSET_FLAG_BIT_SKIP_SUBCOUNTER_UPDATE),
    IPSET_FLAG_BIT_MATCH_COUNTERS = 5,
    IPSET_FLAG_MATCH_COUNTERS = (1 << IPSET_FLAG_BIT_MATCH_COUNTERS),
    IPSET_FLAG_BIT_RETURN_NOMATCH = 7,
    IPSET_FLAG_RETURN_NOMATCH = (1 << IPSET_FLAG_BIT_RETURN_NOMATCH),
    IPSET_FLAG_BIT_MAP_SKBMARK = 8,
    IPSET_FLAG_MAP_SKBMARK = (1 << IPSET_FLAG_BIT_MAP_SKBMARK),
    IPSET_FLAG_BIT_MAP_SKBPRIO = 9,
    IPSET_FLAG_MAP_SKBPRIO = (1 << IPSET_FLAG_BIT_MAP_SKBPRIO),
    IPSET_FLAG_BIT_MAP_SKBQUEUE = 10,
    IPSET_FLAG_MAP_SKBQUEUE = (1 << IPSET_FLAG_BIT_MAP_SKBQUEUE),
    IPSET_FLAG_CMD_MAX = 15,
}

// Flags at CADT attribute level, upper half of cmdattrs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipset_cadt_flags {
    IPSET_FLAG_BIT_BEFORE	= 0,
    IPSET_FLAG_BEFORE	= (1 << IPSET_FLAG_BIT_BEFORE),
    IPSET_FLAG_BIT_PHYSDEV	= 1,
    IPSET_FLAG_PHYSDEV	= (1 << IPSET_FLAG_BIT_PHYSDEV),
    IPSET_FLAG_BIT_NOMATCH	= 2,
    IPSET_FLAG_NOMATCH	= (1 << IPSET_FLAG_BIT_NOMATCH),
    IPSET_FLAG_BIT_WITH_COUNTERS = 3,
    IPSET_FLAG_WITH_COUNTERS = (1 << IPSET_FLAG_BIT_WITH_COUNTERS),
    IPSET_FLAG_BIT_WITH_COMMENT = 4,
    IPSET_FLAG_WITH_COMMENT = (1 << IPSET_FLAG_BIT_WITH_COMMENT),
    IPSET_FLAG_BIT_WITH_FORCEADD = 5,
    IPSET_FLAG_WITH_FORCEADD = (1 << IPSET_FLAG_BIT_WITH_FORCEADD),
    IPSET_FLAG_BIT_WITH_SKBINFO = 6,
    IPSET_FLAG_WITH_SKBINFO = (1 << IPSET_FLAG_BIT_WITH_SKBINFO),
    IPSET_FLAG_BIT_IFACE_WILDCARD = 7,
    IPSET_FLAG_IFACE_WILDCARD = (1 << IPSET_FLAG_BIT_IFACE_WILDCARD),
    IPSET_FLAG_CADT_MAX	= 15,
}

// The flag bits which correspond to the non-extension create flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipset_create_flags {
    IPSET_CREATE_FLAG_BIT_FORCEADD = 0,
    IPSET_CREATE_FLAG_FORCEADD = (1 << IPSET_CREATE_FLAG_BIT_FORCEADD),
    IPSET_CREATE_FLAG_BIT_BUCKETSIZE = 1,
    IPSET_CREATE_FLAG_BUCKETSIZE = (1 << IPSET_CREATE_FLAG_BIT_BUCKETSIZE),
    IPSET_CREATE_FLAG_BIT_MAX = 7,
}

// Commands with settype-specific attributes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipset_adt {
    IPSET_ADD,
    IPSET_DEL,
    IPSET_TEST,
    IPSET_ADT_MAX,
    IPSET_CREATE = IPSET_ADT_MAX,
    IPSET_CADT_MAX,
}

// Sets are identified by an index in kernel space. Tweak with ip_set_id_t
// and IPSET_INVALID_ID if you want to increase the max number of sets.
// Also, IPSET_ATTR_INDEX must be changed.
//
pub type ip_set_id_t = __u16;
pub const IPSET_INVALID_ID: c_int = 65535;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ip_set_dim {
    IPSET_DIM_ZERO = 0,
    IPSET_DIM_ONE,
    IPSET_DIM_TWO,
    IPSET_DIM_THREE,
// Max dimension in elements.
// If changed, new revision of iptables match/target is required.
//
    IPSET_DIM_MAX = 6,
// Backward compatibility: set match revision 2
    IPSET_BIT_RETURN_NOMATCH = 7,
}

// Option flags for kernel operations
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ip_set_kopt {
    IPSET_INV_MATCH = (1 << IPSET_DIM_ZERO),
    IPSET_DIM_ONE_SRC = (1 << IPSET_DIM_ONE),
    IPSET_DIM_TWO_SRC = (1 << IPSET_DIM_TWO),
    IPSET_DIM_THREE_SRC = (1 << IPSET_DIM_THREE),
    IPSET_RETURN_NOMATCH = (1 << IPSET_BIT_RETURN_NOMATCH),
}

// Backward compatibility for set match v3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set_counter_match0 {
    pub op: __u8,
    pub value: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set_counter_match {
    pub value: __aligned_u64,
    pub op: __u8,
}

// Interface to iptables/ip6tables
pub const SO_IP_SET: c_int = 83;
#[repr(C)]
#[derive(Copy, Clone)]
pub union ip_set_name_index {
    pub name: [c_char; IPSET_MAXNAMELEN],
    pub index: ip_set_id_t,
}

pub const IP_SET_OP_GET_BYNAME: c_uint = 0x00000006	/* Get set index by name */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set_req_get_set {
    pub op: c_uint,
    pub version: c_uint,
    pub set: ip_set_name_index,
}

pub const IP_SET_OP_GET_BYINDEX: c_uint = 0x00000007	/* Get set name by index */;
// Uses ip_set_req_get_set
pub const IP_SET_OP_GET_FNAME: c_uint = 0x00000008	/* Get set index and family */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set_req_get_set_family {
    pub op: c_uint,
    pub version: c_uint,
    pub family: c_uint,
    pub set: ip_set_name_index,
}

pub const IP_SET_OP_VERSION: c_uint = 0x00000100	/* Ask kernel version */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set_req_version {
    pub op: c_uint,
    pub version: c_uint,
}
