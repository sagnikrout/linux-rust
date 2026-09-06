//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/rtnetlink.h
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

// rtnetlink families. Values up to 127 are reserved for real address
// families, values above 128 may be used arbitrarily.
//
pub const RTNL_FAMILY_IPMR: c_int = 128;
pub const RTNL_FAMILY_IP6MR: c_int = 129;
pub const RTNL_FAMILY_MAX: c_int = 129;
//
// Routing/neighbour discovery messages.
//
// Types of messages

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtattr {
    pub rta_len: c_ushort,
    pub rta_type: c_ushort,
}

// Macros to handle rtattributes

//
// Definitions used in routing table administration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtmsg {
    pub rtm_family: c_uchar,
    pub rtm_dst_len: c_uchar,
    pub rtm_src_len: c_uchar,
    pub rtm_tos: c_uchar,
    pub /: *mut *mut unsigned char rtm_table; / Routing table id,
    pub /: *mut *mut unsigned char rtm_protocol; / Routing protocol; see below,
    pub /: *mut *mut unsigned char rtm_scope; / See below,
    pub /: *mut *mut unsigned char rtm_type; / See below,
    pub rtm_flags: unsigned,
}

// rtm_type

// rtm_protocol
pub const RTPROT_UNSPEC: c_int = 0;

// Values of protocol >= RTPROT_STATIC are not interpreted by kernel;
//

// rtm_scope
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt_scope_t {
    RT_SCOPE_UNIVERSE=0,
// User defined values
    RT_SCOPE_SITE=200,
    RT_SCOPE_LINK=253,
    RT_SCOPE_HOST=254,
    RT_SCOPE_NOWHERE=255
}

// rtm_flags
pub const RTM_F_NOTIFY: c_uint = 0x100	/* Notify user of route change	*/;
pub const RTM_F_CLONED: c_uint = 0x200	/* This route is cloned		*/;
pub const RTM_F_EQUALIZE: c_uint = 0x400	/* Multipath equalizer: NI	*/;
pub const RTM_F_PREFIX: c_uint = 0x800	/* Prefix addresses		*/;
pub const RTM_F_LOOKUP_TABLE: c_uint = 0x1000	/* set rtm_table to FIB lookup result */;
pub const RTM_F_FIB_MATCH: c_uint = 0x2000	/* return full fib lookup match */;
pub const RTM_F_OFFLOAD: c_uint = 0x4000	/* route is offloaded */;
pub const RTM_F_TRAP: c_uint = 0x8000	/* route is trapping packets */;
pub const RTM_F_OFFLOAD_FAILED: c_uint = 0x20000000 /* route offload failed, this value;
// is chosen to avoid conflicts with
// other flags defined in
// include/uapi/linux/ipv6_route.h
//
// Reserved table identifiers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt_class_t {
    RT_TABLE_UNSPEC=0,
// User defined values
    RT_TABLE_COMPAT=252,
    RT_TABLE_DEFAULT=253,
    RT_TABLE_MAIN=254,
    RT_TABLE_LOCAL=255,
    RT_TABLE_MAX=0xFFFFFFFF
}

// Routing message attributes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtattr_type_t {
    RTA_UNSPEC,
    RTA_DST,
    RTA_SRC,
    RTA_IIF,
    RTA_OIF,
    RTA_GATEWAY,
    RTA_PRIORITY,
    RTA_PREFSRC,
    RTA_METRICS,
    RTA_MULTIPATH,
    RTA_PROTOINFO, /* no longer used */
    RTA_FLOW,
    RTA_CACHEINFO,
    RTA_SESSION, /* no longer used */
    RTA_MP_ALGO, /* no longer used */
    RTA_TABLE,
    RTA_MARK,
    RTA_MFC_STATS,
    RTA_VIA,
    RTA_NEWDST,
    RTA_PREF,
    RTA_ENCAP_TYPE,
    RTA_ENCAP,
    RTA_EXPIRES,
    RTA_PAD,
    RTA_UID,
    RTA_TTL_PROPAGATE,
    RTA_IP_PROTO,
    RTA_SPORT,
    RTA_DPORT,
    RTA_NH_ID,
    RTA_FLOWLABEL,
    RTA_DEL_REASON,
    __RTA_MAX
}

// RTA_DEL_REASON: why the kernel deleted the route. u32.
// Emitted only on RTM_DELROUTE notifications, and only when the deletion
// path records a cause. Absence means either an older kernel or a
// deletion path that does not (yet) record its cause - consumers must
// treat "absent" and "unspec" identically. New causes may be appended.
// Currently only IPv6 deletion paths record a cause.
//
// The attribute is notification-only: the kernel rejects it in
// requests, so a notification must not be echoed back verbatim.
//
// The value space is family-agnostic: a value must never be
// reinterpreted per address family. A cause that only one family can
// produce still gets its own value rather than reusing another
// family's.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt_del_reason {
    RT_DEL_REASON_UNSPEC,		/* cause not recorded */
    RT_DEL_REASON_EXPIRED,		/* RTF_EXPIRES lifetime ran out (GC) */
    RT_DEL_REASON_RA_WITHDRAWN,	/* zero-lifetime RA / PIO / RIO */
    __RT_DEL_REASON_MAX
}

// RTM_MULTIPATH --- array of struct rtnexthop.
//
// "struct rtnexthop" describes all necessary nexthop information,
// i.e. parameters of path to a destination via this nexthop.
//
// At the moment it is impossible to set different prefsrc, mtu, window
// and rtt for different paths from multipath.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtnexthop {
    pub rtnh_len: c_ushort,
    pub rtnh_flags: c_uchar,
    pub rtnh_hops: c_uchar,
    pub rtnh_ifindex: c_int,
}

// rtnh_flags

// Macros to handle hexthops
pub const RTNH_ALIGNTO: c_int = 4;

// RTA_VIA
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtvia {
    pub rtvia_family: __kernel_sa_family_t,
    pub rtvia_addr: [__u8; ],
}

// RTM_CACHEINFO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rta_cacheinfo {
    pub rta_clntref: __u32,
    pub rta_lastuse: __u32,
    pub rta_expires: __s32,
    pub rta_error: __u32,
    pub rta_used: __u32,
pub const RTNETLINK_HAVE_PEERINFO: c_int = 1;
    pub rta_id: __u32,
    pub rta_ts: __u32,
    pub rta_tsage: __u32,
}

// RTM_METRICS --- array of struct rtattr with types of RTAX_*

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rta_session {
    pub proto: __u8,
    pub pad1: __u8,
    pub pad2: __u16,
    pub sport: __u16,
    pub dport: __u16,
    pub ports: },
    pub type: __u8,
    pub code: __u8,
    pub ident: __u16,
    pub icmpt: },
    pub spi: __u32,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rta_mfc_stats {
    pub mfcs_packets: __u64,
    pub mfcs_bytes: __u64,
    pub mfcs_wrong_if: __u64,
}

//
// General form of address family dependent message.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtgenmsg {
    pub rtgen_family: c_uchar,
}

//
// Link layer specific messages.
//
// struct ifinfomsg
// passes link level specific information, not dependent
// on network protocol.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifinfomsg {
    pub ifi_family: c_uchar,
    pub __ifi_pad: c_uchar,
    pub /: *mut *mut *mut unsigned short ifi_type; / ARPHRD_,
    pub /: *mut *mut int ifi_index; / Link index,
    pub /: *mut *mut *mut unsigned ifi_flags; / IFF_ flags,
    pub /: *mut *mut *mut unsigned ifi_change; / IFF_ change mask,
}

//
// prefix information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prefixmsg {
    pub prefix_family: c_uchar,
    pub prefix_pad1: c_uchar,
    pub prefix_pad2: c_ushort,
    pub prefix_ifindex: c_int,
    pub prefix_type: c_uchar,
    pub prefix_len: c_uchar,
    pub prefix_flags: c_uchar,
    pub prefix_pad3: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prefix_cacheinfo {
    pub preferred_time: __u32,
    pub valid_time: __u32,
}

//
// Traffic control messages.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcmsg {
    pub tcm_family: c_uchar,
    pub tcm__pad1: c_uchar,
    pub tcm__pad2: c_ushort,
    pub tcm_ifindex: c_int,
    pub tcm_handle: __u32,
    pub tcm_parent: __u32,
// tcm_block_index is used instead of tcm_parent
// in case tcm_ifindex == TCM_IFINDEX_MAGIC_BLOCK
//

    pub tcm_info: __u32,
}

// For manipulation of filters in shared block, tcm_ifindex is set to
// TCM_IFINDEX_MAGIC_BLOCK, and tcm_parent is aliased to tcm_block_index
// which is the block index.
//

// data necessary to identify the objects
// (handle, cookie, etc.) and stats.
//

//
// Neighbor Discovery userland options
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nduseroptmsg {
    pub nduseropt_family: c_uchar,
    pub nduseropt_pad1: c_uchar,
    pub /: *mut *mut unsigned short nduseropt_opts_len; / Total length of options,
    pub nduseropt_ifindex: c_int,
    pub nduseropt_icmp_type: __u8,
    pub nduseropt_icmp_code: __u8,
    pub nduseropt_pad2: c_ushort,
    pub nduseropt_pad3: c_uint,
// Followed by one or more ND options
}

// RTnetlink multicast groups - backwards compatibility for userspace
pub const RTMGRP_LINK: c_int = 1;
pub const RTMGRP_NOTIFY: c_int = 2;
pub const RTMGRP_NEIGH: c_int = 4;
pub const RTMGRP_TC: c_int = 8;
pub const RTMGRP_IPV4_IFADDR: c_uint = 0x10;
pub const RTMGRP_IPV4_MROUTE: c_uint = 0x20;
pub const RTMGRP_IPV4_ROUTE: c_uint = 0x40;
pub const RTMGRP_IPV4_RULE: c_uint = 0x80;
pub const RTMGRP_IPV6_IFADDR: c_uint = 0x100;
pub const RTMGRP_IPV6_MROUTE: c_uint = 0x200;
pub const RTMGRP_IPV6_ROUTE: c_uint = 0x400;
pub const RTMGRP_IPV6_IFINFO: c_uint = 0x800;
pub const RTMGRP_DECnet_IFADDR: c_uint = 0x1000;
pub const RTMGRP_DECnet_ROUTE: c_uint = 0x4000;
pub const RTMGRP_IPV6_PREFIX: c_uint = 0x20000;

// RTnetlink multicast groups
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtnetlink_groups {
    RTNLGRP_NONE,

    RTNLGRP_LINK,

    RTNLGRP_NOTIFY,

    RTNLGRP_NEIGH,

    RTNLGRP_TC,

    RTNLGRP_IPV4_IFADDR,

    RTNLGRP_IPV4_MROUTE,

    RTNLGRP_IPV4_ROUTE,

    RTNLGRP_IPV4_RULE,

    RTNLGRP_IPV6_IFADDR,

    RTNLGRP_IPV6_MROUTE,

    RTNLGRP_IPV6_ROUTE,

    RTNLGRP_IPV6_IFINFO,

    RTNLGRP_DECnet_IFADDR,

    RTNLGRP_NOP2,
    RTNLGRP_DECnet_ROUTE,

    RTNLGRP_DECnet_RULE,

    RTNLGRP_NOP4,
    RTNLGRP_IPV6_PREFIX,

    RTNLGRP_IPV6_RULE,

    RTNLGRP_ND_USEROPT,

    RTNLGRP_PHONET_IFADDR,

    RTNLGRP_PHONET_ROUTE,

    RTNLGRP_DCB,

    RTNLGRP_IPV4_NETCONF,

    RTNLGRP_IPV6_NETCONF,

    RTNLGRP_MDB,

    RTNLGRP_MPLS_ROUTE,

    RTNLGRP_NSID,

    RTNLGRP_MPLS_NETCONF,

    RTNLGRP_IPV4_MROUTE_R,

    RTNLGRP_IPV6_MROUTE_R,

    RTNLGRP_NEXTHOP,

    RTNLGRP_BRVLAN,

    RTNLGRP_MCTP_IFADDR,

    RTNLGRP_TUNNEL,

    RTNLGRP_STATS,

    RTNLGRP_IPV4_MCADDR,

    RTNLGRP_IPV6_MCADDR,

    RTNLGRP_IPV6_ACADDR,

    __RTNLGRP_MAX
}

// TC action piece
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcamsg {
    pub tca_family: c_uchar,
    pub tca__pad1: c_uchar,
    pub tca__pad2: c_ushort,
}

// tcamsg flags stored in attribute TCA_ROOT_FLAGS
//
// TCA_ACT_FLAG_LARGE_DUMP_ON user->kernel to request for larger than
// TCA_ACT_MAX_PRIO actions in a dump. All dump responses will contain the
// number of actions being dumped stored in for user app's consumption in
// TCA_ROOT_COUNT
//
// TCA_ACT_FLAG_TERSE_DUMP user->kernel to request terse (brief) dump that only
// includes essential action info (kind, index, etc.)
//

// New extended info filters for IFLA_EXT_MASK

// End of information exported to user level
