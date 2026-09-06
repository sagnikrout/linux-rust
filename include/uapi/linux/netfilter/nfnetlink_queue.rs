//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/nfnetlink_queue.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfqnl_msg_types {
    NFQNL_MSG_PACKET,		/* packet from kernel to userspace */
    NFQNL_MSG_VERDICT,		/* verdict from userspace to kernel */
    NFQNL_MSG_CONFIG,		/* connect to a particular queue */
    NFQNL_MSG_VERDICT_BATCH,	/* batchv from userspace to kernel */

    NFQNL_MSG_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfqnl_msg_packet_hdr {
    pub /: *mut *mut __be32 packet_id; / unique ID of packet in queue,
    pub /: *mut *mut __be16 hw_protocol; / hw protocol (network order),
    pub /: *mut *mut __u8 hook; / netfilter hook,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfqnl_msg_packet_hw {
    pub hw_addrlen: __be16,
    pub _pad: __u16,
    pub hw_addr: [__u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfqnl_msg_packet_timestamp {
    pub sec: __aligned_be64,
    pub usec: __aligned_be64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfqnl_vlan_attr {
    NFQA_VLAN_UNSPEC,
    NFQA_VLAN_PROTO,		/* __be16 skb vlan_proto */
    NFQA_VLAN_TCI,			/* __be16 skb htons(vlan_tci) */
    __NFQA_VLAN_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfqnl_attr_type {
    NFQA_UNSPEC,
    NFQA_PACKET_HDR,
    NFQA_VERDICT_HDR,		/* nfqnl_msg_verdict_hrd */
    NFQA_MARK,			/* __u32 nfmark */
    NFQA_TIMESTAMP,			/* nfqnl_msg_packet_timestamp */
    NFQA_IFINDEX_INDEV,		/* __u32 ifindex */
    NFQA_IFINDEX_OUTDEV,		/* __u32 ifindex */
    NFQA_IFINDEX_PHYSINDEV,		/* __u32 ifindex */
    NFQA_IFINDEX_PHYSOUTDEV,	/* __u32 ifindex */
    NFQA_HWADDR,			/* nfqnl_msg_packet_hw */
    NFQA_PAYLOAD,			/* opaque data payload */
    NFQA_CT,			/* nfnetlink_conntrack.h */
    NFQA_CT_INFO,			/* enum ip_conntrack_info */
    NFQA_CAP_LEN,			/* __u32 length of captured packet */
    NFQA_SKB_INFO,			/* __u32 skb meta information */
    NFQA_EXP,			/* nfnetlink_conntrack.h */
    NFQA_UID,			/* __u32 sk uid */
    NFQA_GID,			/* __u32 sk gid */
    NFQA_SECCTX,			/* security context string */
    NFQA_VLAN,			/* nested attribute: packet vlan info */
    NFQA_L2HDR,			/* full L2 header */
    NFQA_PRIORITY,			/* skb->priority */
    NFQA_CGROUP_CLASSID,		/* __u32 cgroup classid */

    __NFQA_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfqnl_msg_verdict_hdr {
    pub verdict: __be32,
    pub id: __be32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfqnl_msg_config_cmds {
    NFQNL_CFG_CMD_NONE,
    NFQNL_CFG_CMD_BIND,
    NFQNL_CFG_CMD_UNBIND,
    NFQNL_CFG_CMD_PF_BIND,
    NFQNL_CFG_CMD_PF_UNBIND,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfqnl_msg_config_cmd {
    pub /: *mut *mut __u8 command; / nfqnl_msg_config_cmds,
    pub _pad: __u8,
    pub /: *mut *mut __be16 pf; / AF_xxx for PF_[UN]BIND,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfqnl_config_mode {
    NFQNL_COPY_NONE,
    NFQNL_COPY_META,
    NFQNL_COPY_PACKET,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfqnl_msg_config_params {
    pub copy_range: __be32,
    pub /: *mut *mut __u8 copy_mode; / enum nfqnl_config_mode,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfqnl_attr_config {
    NFQA_CFG_UNSPEC,
    NFQA_CFG_CMD,			/* nfqnl_msg_config_cmd */
    NFQA_CFG_PARAMS,		/* nfqnl_msg_config_params */
    NFQA_CFG_QUEUE_MAXLEN,		/* __u32 */
    NFQA_CFG_MASK,			/* identify which flags to change */
    NFQA_CFG_FLAGS,			/* value of these flags (__u32) */
    __NFQA_CFG_MAX
}

// Flags for NFQA_CFG_FLAGS

// flags for NFQA_SKB_INFO
// packet appears to have wrong checksums, but they are ok

// packet is GSO (i.e., exceeds device mtu)

// csum not validated (incoming device doesn't support hw checksum, etc.)

