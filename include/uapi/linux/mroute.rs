//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mroute.h
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

// Based on the MROUTING 3.5 defines primarily to keep
// source compatibility with BSD.
//
// See the mrouted code for the original history.
//
// Protocol Independent Multicast (PIM) data structures included
// Carlos Picoto (cap@di.fc.ul.pt)
//
pub const MRT_BASE: c_int = 200;

// MRT_FLUSH optional flags

pub const MAXVIFS: c_int = 32;
pub type vifi_t = c_ushort;

// Same idea as select

// Passed by mrouted for an MRT_ADD_VIF - again we use the
// mrouted 3.6 structures for compatibility
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vifctl {
    pub /: *mut *mut vifi_t vifc_vifi; / Index of VIF,
    pub /: *mut *mut unsigned char vifc_flags; / VIFF_ flags,
    pub /: *mut *mut unsigned char vifc_threshold; / ttl limit,
    pub /: *mut *mut unsigned int vifc_rate_limit; / Rate limiter values (NI),
    pub /: *mut *mut in_addr vifc_lcl_addr; / Local interface address,
    pub /: *mut *mut int vifc_lcl_ifindex; / Local interface index,
}

pub const VIFF_TUNNEL: c_uint = 0x1	/* IPIP tunnel */;
pub const VIFF_SRCRT: c_uint = 0x2	/* NI */;
pub const VIFF_REGISTER: c_uint = 0x4	/* register vif	*/;
pub const VIFF_USE_IFINDEX: c_uint = 0x8	/* use vifc_lcl_ifindex instead of;
// Cache manipulation structures for mrouted and PIMd
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfcctl {
    pub /: *mut *mut in_addr mfcc_origin; / Origin of mcast,
    pub /: *mut *mut in_addr mfcc_mcastgrp; / Group in question,
    pub /: *mut *mut vifi_t mfcc_parent; / Where it arrived,
    pub /: *mut *mut unsigned char mfcc_ttls[MAXVIFS]; / Where it is going,
    pub /: *mut *mut unsigned int mfcc_pkt_cnt; / pkt count for src-grp,
    pub mfcc_byte_cnt: c_uint,
    pub mfcc_wrong_if: c_uint,
    pub mfcc_expire: c_int,
}

// Group count retrieval for mrouted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sioc_sg_req {
    pub src: in_addr,
    pub grp: in_addr,
    pub pktcnt: c_ulong,
    pub bytecnt: c_ulong,
    pub wrong_if: c_ulong,
}

// To get vif packet counts
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sioc_vif_req {
    pub /: *mut *mut vifi_t vifi; / Which iface,
    pub /: *mut *mut unsigned long icount; / In packets,
    pub /: *mut *mut unsigned long ocount; / Out packets,
    pub /: *mut *mut unsigned long ibytes; / In bytes,
    pub /: *mut *mut unsigned long obytes; / Out bytes,
}

// This is the format the mroute daemon expects to see IGMP control
// data. Magically happens to be like an IP packet as per the original
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igmpmsg {
    pub unused1,unused2: __u32,
    pub /: *mut *mut unsigned char im_msgtype; / What is this,
    pub /: *mut *mut unsigned char im_mbz; / Must be zero,
    pub /: *mut *mut unsigned char im_vif; / Low 8 bits of Interface,
    pub /: *mut *mut unsigned char im_vif_hi; / High 8 bits of Interface,
    pub im_src,im_dst: in_addr,
}

// ipmr netlink table attributes

// ipmr netlink vif attribute format
// [ IPMRA_TABLE_VIFS ] - nested attribute
// [ IPMRA_VIF ] - nested attribute
// [ IPMRA_VIFA_xxx ]
//

// vif-specific attributes

// ipmr netlink cache report attributes

// That's all usermode folks

// Pseudo messages used by mrouted

