//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/psample.h
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
// sampled packets or scaled probability
// if PSAMPLE_ATTR_SAMPLE_PROBABILITY
// is set.
//
// PSAMPLE_ATTR_SAMPLE_RATE as a
// probability scaled 0 - U32_MAX.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psample_command {
    PSAMPLE_CMD_SAMPLE,
    PSAMPLE_CMD_GET_GROUP,
    PSAMPLE_CMD_NEW_GROUP,
    PSAMPLE_CMD_DEL_GROUP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psample_tunnel_key_attr {
    PSAMPLE_TUNNEL_KEY_ATTR_ID,                 /* be64 Tunnel ID */
    PSAMPLE_TUNNEL_KEY_ATTR_IPV4_SRC,           /* be32 src IP address. */
    PSAMPLE_TUNNEL_KEY_ATTR_IPV4_DST,           /* be32 dst IP address. */
    PSAMPLE_TUNNEL_KEY_ATTR_TOS,                /* u8 Tunnel IP ToS. */
    PSAMPLE_TUNNEL_KEY_ATTR_TTL,                /* u8 Tunnel IP TTL. */
    PSAMPLE_TUNNEL_KEY_ATTR_DONT_FRAGMENT,      /* No argument, set DF. */
    PSAMPLE_TUNNEL_KEY_ATTR_CSUM,               /* No argument. CSUM packet. */
    PSAMPLE_TUNNEL_KEY_ATTR_OAM,                /* No argument. OAM frame.  */
    PSAMPLE_TUNNEL_KEY_ATTR_GENEVE_OPTS,        /* Array of Geneve options. */
    PSAMPLE_TUNNEL_KEY_ATTR_TP_SRC,	            /* be16 src Transport Port. */
    PSAMPLE_TUNNEL_KEY_ATTR_TP_DST,		    /* be16 dst Transport Port. */
    PSAMPLE_TUNNEL_KEY_ATTR_VXLAN_OPTS,	    /* Nested VXLAN opts* */
    PSAMPLE_TUNNEL_KEY_ATTR_IPV6_SRC,           /* struct in6_addr src IPv6 address. */
    PSAMPLE_TUNNEL_KEY_ATTR_IPV6_DST,           /* struct in6_addr dst IPv6 address. */
    PSAMPLE_TUNNEL_KEY_ATTR_PAD,
    PSAMPLE_TUNNEL_KEY_ATTR_ERSPAN_OPTS,        /* struct erspan_metadata */
    PSAMPLE_TUNNEL_KEY_ATTR_IPV4_INFO_BRIDGE,   /* No argument. IPV4_INFO_BRIDGE mode.*/
    __PSAMPLE_TUNNEL_KEY_ATTR_MAX
}

// Can be overridden at runtime by module option

pub const PSAMPLE_GENL_VERSION: c_int = 1;
