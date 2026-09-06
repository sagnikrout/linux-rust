//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netlink.h
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

pub const NETLINK_FIB_LOOKUP: c_int = 10;
pub const NETLINK_CONNECTOR: c_int = 11;

pub const NETLINK_IP6_FW: c_int = 13;

pub const NETLINK_GENERIC: c_int = 16;
// leave room for NETLINK_DM (DM Events)

pub const NETLINK_ECRYPTFS: c_int = 19;
pub const NETLINK_RDMA: c_int = 20;

pub const MAX_LINKS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_nl {
    pub /: *mut *mut __kernel_sa_family_t nl_family; / AF_NETLINK,
    pub /: *mut *mut unsigned short nl_pad; / zero,
    pub /: *mut *mut __u32 nl_pid; / port ID,
    pub /: *mut *mut __u32 nl_groups; / multicast groups mask,
}

//
// struct nlmsghdr - fixed format metadata header of Netlink messages
// @nlmsg_len:   Length of message including header
// @nlmsg_type:  Message content type
// @nlmsg_flags: Additional flags
// @nlmsg_seq:   Sequence number
// @nlmsg_pid:   Sending process port ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlmsghdr {
    pub nlmsg_len: __u32,
    pub nlmsg_type: __u16,
    pub nlmsg_flags: __u16,
    pub nlmsg_seq: __u32,
    pub nlmsg_pid: __u32,
}

// Flags values
pub const NLM_F_REQUEST: c_uint = 0x01	/* It is request message. 	*/;
pub const NLM_F_MULTI: c_uint = 0x02	/* Multipart message, terminated by NLMSG_DONE */;
pub const NLM_F_ACK: c_uint = 0x04	/* Reply with ack, with zero or error code */;
pub const NLM_F_ECHO: c_uint = 0x08	/* Receive resulting notifications */;
pub const NLM_F_DUMP_INTR: c_uint = 0x10	/* Dump was inconsistent due to sequence change */;
pub const NLM_F_DUMP_FILTERED: c_uint = 0x20	/* Dump was filtered as requested */;
// Modifiers to GET request
pub const NLM_F_ROOT: c_uint = 0x100	/* specify tree	root	*/;
pub const NLM_F_MATCH: c_uint = 0x200	/* return all matching	*/;
pub const NLM_F_ATOMIC: c_uint = 0x400	/* atomic GET		*/;

// Modifiers to NEW request
pub const NLM_F_REPLACE: c_uint = 0x100	/* Override existing		*/;
pub const NLM_F_EXCL: c_uint = 0x200	/* Do not touch, if it exists	*/;
pub const NLM_F_CREATE: c_uint = 0x400	/* Create, if it does not exist	*/;
pub const NLM_F_APPEND: c_uint = 0x800	/* Add to end of list		*/;
// Modifiers to DELETE request
pub const NLM_F_NONREC: c_uint = 0x100	/* Do not delete recursively	*/;
pub const NLM_F_BULK: c_uint = 0x200	/* Delete multiple objects	*/;
// Flags for ACK message
pub const NLM_F_CAPPED: c_uint = 0x100	/* request was capped */;
pub const NLM_F_ACK_TLVS: c_uint = 0x200	/* extended ACK TVLs were included */;
//

pub const NLMSG_NOOP: c_uint = 0x1	/* Nothing.		*/;
pub const NLMSG_ERROR: c_uint = 0x2	/* Error		*/;
pub const NLMSG_DONE: c_uint = 0x3	/* End of a dump	*/;
pub const NLMSG_OVERRUN: c_uint = 0x4	/* Data lost		*/;
pub const NLMSG_MIN_TYPE: c_uint = 0x10	/* < 0x10: reserved control messages */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlmsgerr {
    pub error: c_int,
    pub msg: nlmsghdr,
//
// followed by the message contents unless NETLINK_CAP_ACK was set
// or the ACK indicates success (error == 0)
// message length is aligned with NLMSG_ALIGN()
//
// followed by TLVs defined in enum nlmsgerr_attrs
// if NETLINK_EXT_ACK was set
//
}

//
// enum nlmsgerr_attrs - nlmsgerr attributes
// @NLMSGERR_ATTR_UNUSED: unused
// @NLMSGERR_ATTR_MSG: error message string (string)
// @NLMSGERR_ATTR_OFFS: offset of the invalid attribute in the original
// message, counting from the beginning of the header (u32)
// @NLMSGERR_ATTR_COOKIE: arbitrary subsystem specific cookie to
// be used - in the success case - to identify a created
// object or operation or similar (binary)
// @NLMSGERR_ATTR_POLICY: policy for a rejected attribute
// @NLMSGERR_ATTR_MISS_TYPE: type of a missing required attribute,
// %NLMSGERR_ATTR_MISS_NEST will not be present if the attribute was
// missing at the message level
// @NLMSGERR_ATTR_MISS_NEST: offset of the nest where attribute was missing
// @__NLMSGERR_ATTR_MAX: number of attributes
// @NLMSGERR_ATTR_MAX: highest attribute number
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nlmsgerr_attrs {
    NLMSGERR_ATTR_UNUSED,
    NLMSGERR_ATTR_MSG,
    NLMSGERR_ATTR_OFFS,
    NLMSGERR_ATTR_COOKIE,
    NLMSGERR_ATTR_POLICY,
    NLMSGERR_ATTR_MISS_TYPE,
    NLMSGERR_ATTR_MISS_NEST,

    __NLMSGERR_ATTR_MAX,
    NLMSGERR_ATTR_MAX = __NLMSGERR_ATTR_MAX - 1
}

pub const NETLINK_ADD_MEMBERSHIP: c_int = 1;
pub const NETLINK_DROP_MEMBERSHIP: c_int = 2;
pub const NETLINK_PKTINFO: c_int = 3;
pub const NETLINK_BROADCAST_ERROR: c_int = 4;
pub const NETLINK_NO_ENOBUFS: c_int = 5;
pub const NETLINK_RX_RING: c_int = 6;
pub const NETLINK_TX_RING: c_int = 7;

pub const NETLINK_LISTEN_ALL_NSID: c_int = 8;
pub const NETLINK_LIST_MEMBERSHIPS: c_int = 9;
pub const NETLINK_CAP_ACK: c_int = 10;
pub const NETLINK_EXT_ACK: c_int = 11;
pub const NETLINK_GET_STRICT_CHK: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nl_pktinfo {
    pub group: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nl_mmap_req {
    pub nm_block_size: c_uint,
    pub nm_block_nr: c_uint,
    pub nm_frame_size: c_uint,
    pub nm_frame_nr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nl_mmap_hdr {
    pub nm_status: c_uint,
    pub nm_len: c_uint,
    pub nm_group: __u32,
// credentials
    pub nm_pid: __u32,
    pub nm_uid: __u32,
    pub nm_gid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nl_mmap_status {
    NL_MMAP_STATUS_UNUSED,
    NL_MMAP_STATUS_RESERVED,
    NL_MMAP_STATUS_VALID,
    NL_MMAP_STATUS_COPY,
    NL_MMAP_STATUS_SKIP,
}

//
// <------- NLA_HDRLEN ------> <-- NLA_ALIGN(payload)-->
// +---------------------+- - -+- - - - - - - - - -+- - -+
// |        Header       | Pad |     Payload       | Pad |
// |   (struct nlattr)   | ing |                   | ing |
// +---------------------+- - -+- - - - - - - - - -+- - -+
// <-------------- nlattr->nla_len -------------->
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlattr {
    pub nla_len: __u16,
    pub nla_type: __u16,
}

//
// nla_type (16 bits)
// +---+---+-------------------------------+
// | N | O | Attribute Type                |
// +---+---+-------------------------------+
// N := Carries nested attributes
// O := Payload stored in network byte order
//
// Note: The N and O flag are mutually exclusive.
//

pub const NLA_ALIGNTO: c_int = 4;

// Generic 32 bitflags attribute content sent to the kernel.
//
// The value is a bitmap that defines the values being set
// The selector is a bitmask that defines which value is legit
//
// Examples:
// value = 0x0, and selector = 0x1
// implies we are selecting bit 1 and we want to set its value to 0.
//
// value = 0x2, and selector = 0x2
// implies we are selecting bit 2 and we want to set its value to 1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nla_bitfield32 {
    pub value: __u32,
    pub selector: __u32,
}

//
// policy descriptions - it's specific to each family how this is used
// Normally, it should be retrieved via a dump inside another attribute
// specifying where it applies.
//
// enum netlink_attribute_type - type of an attribute
// @NL_ATTR_TYPE_INVALID: unused
// @NL_ATTR_TYPE_FLAG: flag attribute (present/not present)
// @NL_ATTR_TYPE_U8: 8-bit unsigned attribute
// @NL_ATTR_TYPE_U16: 16-bit unsigned attribute
// @NL_ATTR_TYPE_U32: 32-bit unsigned attribute
// @NL_ATTR_TYPE_U64: 64-bit unsigned attribute
// @NL_ATTR_TYPE_S8: 8-bit signed attribute
// @NL_ATTR_TYPE_S16: 16-bit signed attribute
// @NL_ATTR_TYPE_S32: 32-bit signed attribute
// @NL_ATTR_TYPE_S64: 64-bit signed attribute
// @NL_ATTR_TYPE_BINARY: binary data, min/max length may be specified
// @NL_ATTR_TYPE_STRING: string, min/max length may be specified
// @NL_ATTR_TYPE_NUL_STRING: NUL-terminated string,
// min/max length may be specified
// @NL_ATTR_TYPE_NESTED: nested, i.e. the content of this attribute
// consists of sub-attributes. The nested policy and maxtype
// inside may be specified.
// @NL_ATTR_TYPE_NESTED_ARRAY: nested array, i.e. the content of this
// attribute contains sub-attributes whose type is irrelevant
// (just used to separate the array entries) and each such array
// entry has attributes again, the policy for those inner ones
// and the corresponding maxtype may be specified.
// @NL_ATTR_TYPE_BITFIELD32: &struct nla_bitfield32 attribute
// @NL_ATTR_TYPE_SINT: 32-bit or 64-bit signed attribute, aligned to 4B
// @NL_ATTR_TYPE_UINT: 32-bit or 64-bit unsigned attribute, aligned to 4B
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netlink_attribute_type {
    NL_ATTR_TYPE_INVALID,

    NL_ATTR_TYPE_FLAG,

    NL_ATTR_TYPE_U8,
    NL_ATTR_TYPE_U16,
    NL_ATTR_TYPE_U32,
    NL_ATTR_TYPE_U64,

    NL_ATTR_TYPE_S8,
    NL_ATTR_TYPE_S16,
    NL_ATTR_TYPE_S32,
    NL_ATTR_TYPE_S64,

    NL_ATTR_TYPE_BINARY,
    NL_ATTR_TYPE_STRING,
    NL_ATTR_TYPE_NUL_STRING,

    NL_ATTR_TYPE_NESTED,
    NL_ATTR_TYPE_NESTED_ARRAY,

    NL_ATTR_TYPE_BITFIELD32,

    NL_ATTR_TYPE_SINT,
    NL_ATTR_TYPE_UINT,
}

//
// enum netlink_policy_type_attr - policy type attributes
// @NL_POLICY_TYPE_ATTR_UNSPEC: unused
// @NL_POLICY_TYPE_ATTR_TYPE: type of the attribute,
// &enum netlink_attribute_type (U32)
// @NL_POLICY_TYPE_ATTR_MIN_VALUE_S: minimum value for signed
// integers (S64)
// @NL_POLICY_TYPE_ATTR_MAX_VALUE_S: maximum value for signed
// integers (S64)
// @NL_POLICY_TYPE_ATTR_MIN_VALUE_U: minimum value for unsigned
// integers (U64)
// @NL_POLICY_TYPE_ATTR_MAX_VALUE_U: maximum value for unsigned
// integers (U64)
// @NL_POLICY_TYPE_ATTR_MIN_LENGTH: minimum length for binary
// attributes, no minimum if not given (U32)
// @NL_POLICY_TYPE_ATTR_MAX_LENGTH: maximum length for binary
// attributes, no maximum if not given (U32)
// @NL_POLICY_TYPE_ATTR_POLICY_IDX: sub policy for nested and
// nested array types (U32)
// @NL_POLICY_TYPE_ATTR_POLICY_MAXTYPE: maximum sub policy
// attribute for nested and nested array types, this can
// in theory be < the size of the policy pointed to by
// the index, if limited inside the nesting (U32)
// @NL_POLICY_TYPE_ATTR_BITFIELD32_MASK: valid mask for the
// bitfield32 type (U32)
// @NL_POLICY_TYPE_ATTR_MASK: mask of valid bits for unsigned integers (U64)
// @NL_POLICY_TYPE_ATTR_PAD: pad attribute for 64-bit alignment
//
// @__NL_POLICY_TYPE_ATTR_MAX: number of attributes
// @NL_POLICY_TYPE_ATTR_MAX: highest attribute number
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netlink_policy_type_attr {
    NL_POLICY_TYPE_ATTR_UNSPEC,
    NL_POLICY_TYPE_ATTR_TYPE,
    NL_POLICY_TYPE_ATTR_MIN_VALUE_S,
    NL_POLICY_TYPE_ATTR_MAX_VALUE_S,
    NL_POLICY_TYPE_ATTR_MIN_VALUE_U,
    NL_POLICY_TYPE_ATTR_MAX_VALUE_U,
    NL_POLICY_TYPE_ATTR_MIN_LENGTH,
    NL_POLICY_TYPE_ATTR_MAX_LENGTH,
    NL_POLICY_TYPE_ATTR_POLICY_IDX,
    NL_POLICY_TYPE_ATTR_POLICY_MAXTYPE,
    NL_POLICY_TYPE_ATTR_BITFIELD32_MASK,
    NL_POLICY_TYPE_ATTR_PAD,
    NL_POLICY_TYPE_ATTR_MASK,

// keep last
    __NL_POLICY_TYPE_ATTR_MAX,
    NL_POLICY_TYPE_ATTR_MAX = __NL_POLICY_TYPE_ATTR_MAX - 1
}
