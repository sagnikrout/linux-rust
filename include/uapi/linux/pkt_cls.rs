//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/pkt_cls.h
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

pub const TC_COOKIE_MAX_SIZE: c_int = 16;
// Action attributes
// See other TCA_ACT_FLAGS_ * flags in include/net/act_api.h.

// actions stats.
//

// tca HW stats type
// When user does not pass the attribute, he does not care.
// It is the same as if he would pass the attribute with
// all supported bits set.
// In case no bits are set, user is not interested in getting any HW statistics.
//

// gets the current HW stats
// state from the device
// queried at the dump time.
//

// HW stats that might be out of date
// for some time, maybe couple of
// seconds. This is the case when
// driver polls stats updates
// periodically or when it gets async
// stats update from the device.
//

pub const TCA_ACT_MAX_PRIO: c_int = 32;
pub const TCA_ACT_BIND: c_int = 1;
pub const TCA_ACT_NOBIND: c_int = 0;
pub const TCA_ACT_UNBIND: c_int = 1;
pub const TCA_ACT_NOUNBIND: c_int = 0;
pub const TCA_ACT_REPLACE: c_int = 1;
pub const TCA_ACT_NOREPLACE: c_int = 0;

pub const TC_ACT_OK: c_int = 0;
pub const TC_ACT_RECLASSIFY: c_int = 1;
pub const TC_ACT_SHOT: c_int = 2;
pub const TC_ACT_PIPE: c_int = 3;
pub const TC_ACT_STOLEN: c_int = 4;
pub const TC_ACT_QUEUED: c_int = 5;
pub const TC_ACT_REPEAT: c_int = 6;
pub const TC_ACT_REDIRECT: c_int = 7;

// and don't further process the frame
// in hardware. For sw path, this is
// equivalent of TC_ACT_STOLEN - drop
// the skb and act like everything
// is alright.
//

// There is a special kind of actions called "extended actions",
// which need a value parameter. These have a local opcode located in
// the highest nibble, starting from 1. The rest of the bits
// are used to carry the value. These two parts together make
// a combined opcode.
//
pub const __TC_ACT_EXT_SHIFT: c_int = 28;

// These macros are put here for binary compatibility with userspace apps that
// make use of them. For kernel code and new userspace apps, use the TCA_ID_
// versions.
//
pub const TCA_ACT_GACT: c_int = 5;

pub const TCA_ACT_PEDIT: c_int = 7;
pub const TCA_ACT_MIRRED: c_int = 8;
pub const TCA_ACT_NAT: c_int = 9;
pub const TCA_ACT_XT: c_int = 10;
pub const TCA_ACT_SKBEDIT: c_int = 11;
pub const TCA_ACT_VLAN: c_int = 12;
pub const TCA_ACT_BPF: c_int = 13;
pub const TCA_ACT_CONNMARK: c_int = 14;
pub const TCA_ACT_SKBMOD: c_int = 15;
pub const TCA_ACT_CSUM: c_int = 16;
pub const TCA_ACT_TUNNEL_KEY: c_int = 17;
pub const TCA_ACT_SIMP: c_int = 22;
pub const TCA_ACT_IFE: c_int = 25;
pub const TCA_ACT_SAMPLE: c_int = 26;
// Action type identifiers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tca_id {
    TCA_ID_UNSPEC = 0,
    TCA_ID_POLICE = 1,
    TCA_ID_GACT = TCA_ACT_GACT,
    TCA_ID_IPT = TCA_ACT_IPT, /* Obsoleted, can be reused */
    TCA_ID_PEDIT = TCA_ACT_PEDIT,
    TCA_ID_MIRRED = TCA_ACT_MIRRED,
    TCA_ID_NAT = TCA_ACT_NAT,
    TCA_ID_XT = TCA_ACT_XT,
    TCA_ID_SKBEDIT = TCA_ACT_SKBEDIT,
    TCA_ID_VLAN = TCA_ACT_VLAN,
    TCA_ID_BPF = TCA_ACT_BPF,
    TCA_ID_CONNMARK = TCA_ACT_CONNMARK,
    TCA_ID_SKBMOD = TCA_ACT_SKBMOD,
    TCA_ID_CSUM = TCA_ACT_CSUM,
    TCA_ID_TUNNEL_KEY = TCA_ACT_TUNNEL_KEY,
    TCA_ID_SIMP = TCA_ACT_SIMP,
    TCA_ID_IFE = TCA_ACT_IFE,
    TCA_ID_SAMPLE = TCA_ACT_SAMPLE,
    TCA_ID_CTINFO,
    TCA_ID_MPLS,
    TCA_ID_CT,
    TCA_ID_GATE,
// other actions go here
    __TCA_ID_MAX = 255
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_police {
    pub index: __u32,
    pub action: c_int,

    pub limit: __u32,
    pub burst: __u32,
    pub mtu: __u32,
    pub rate: tc_ratespec,
    pub peakrate: tc_ratespec,
    pub refcnt: c_int,
    pub bindcnt: c_int,
    pub capab: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_t {
    pub install: __u64,
    pub lastuse: __u64,
    pub expires: __u64,
    pub firstuse: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_cnt {
    pub refcnt: c_int,
    pub bindcnt: c_int,
}

// tca flags definitions

// U32 filters

pub const TC_U32_UNSPEC: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_u32_key {
    pub mask: __be32,
    pub val: __be32,
    pub off: c_int,
    pub offmask: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_u32_sel {
// New members MUST be added within the __struct_group() macro below.
    pub flags: c_uchar,
    pub offshift: c_uchar,
    pub nkeys: c_uchar,
    pub offmask: __be16,
    pub off: __u16,
    pub offoff: c_short,
    pub hoff: c_short,
    pub hmask: __be32,
    pub keys: [tc_u32_key; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_u32_mark {
    pub val: __u32,
    pub mask: __u32,
    pub success: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_u32_pcnt {
    pub rcnt: __u64,
    pub rhit: __u64,
    pub kcnts: [__u64; ],
}

// Flags
pub const TC_U32_TERMINAL: c_int = 1;
pub const TC_U32_OFFSET: c_int = 2;
pub const TC_U32_VAROFFSET: c_int = 4;
pub const TC_U32_EAT: c_int = 8;
pub const TC_U32_MAXDEPTH: c_int = 8;
// ROUTE filter

// FW filter

// Flow filter

// Basic filter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_basic_pcnt {
    pub rcnt: __u64,
    pub rhit: __u64,
}

// Cgroup classifier

// BPF classifier

// Flower classifier

// TCA_FLOWER_KEY_ENC_OPT_GENEVE_
// attributes
//
// TCA_FLOWER_KEY_ENC_OPT_VXLAN_
// attributes
//
// TCA_FLOWER_KEY_ENC_OPT_ERSPAN_
// attributes
//
// TCA_FLOWER_KEY_ENC_OPT_GTP_
// attributes
//
// TCA_FLOWER_KEY_ENC_IPT_PFCP
// attributes
//

// Match-all classifier
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_matchall_pcnt {
    pub rhit: __u64,
}

// Extended Matches
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_ematch_tree_hdr {
    pub nmatches: __u16,
    pub progid: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_ematch_hdr {
    pub matchid: __u16,
    pub kind: __u16,
    pub flags: __u16,
    pub /: *mut *mut __u16 pad; / currently unused,
}

// 0                   1
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5
// +-----------------------+-+-+---+
// |         Unused        |S|I| R |
// +-----------------------+-+-+---+
//
// R(2) ::= relation to next ematch
// where: 0 0 END (last ematch)
// 0 1 AND
// 1 0 OR
// 1 1 Unused (invalid)
// I(1) ::= invert result
// S(1) ::= simple payload
//
pub const TCF_EM_REL_END: c_int = 0;

pub const TCF_EM_REL_MASK: c_int = 3;

// Ematch type assignments
// 1..32767		Reserved for ematches inside kernel tree
// 32768..65535	Free to use, not reliable
//
pub const TCF_EM_CONTAINER: c_int = 0;
pub const TCF_EM_CMP: c_int = 1;
pub const TCF_EM_NBYTE: c_int = 2;
pub const TCF_EM_U32: c_int = 3;
pub const TCF_EM_META: c_int = 4;
pub const TCF_EM_TEXT: c_int = 5;
pub const TCF_EM_VLAN: c_int = 6;
pub const TCF_EM_CANID: c_int = 7;
pub const TCF_EM_IPSET: c_int = 8;
pub const TCF_EM_IPT: c_int = 9;
pub const TCF_EM_MAX: c_int = 9;
