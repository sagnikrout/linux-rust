//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/xfrm.h
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

// All of the structures in this file may not change size as they are
// passed into the kernel from userspace via netlink sockets.
//
// Structure to encapsulate addresses. I do not want to use
// "standard" structure. My apologies.
//
// Ident of a specific xfrm_state. It is used on input to lookup
// the state by (spi,daddr,ah/esp) or to store information about
// spi, protocol and tunnel address on output.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_id {
    pub daddr: xfrm_address_t,
    pub spi: __be32,
    pub proto: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_sec_ctx {
    pub ctx_doi: __u8,
    pub ctx_alg: __u8,
    pub ctx_len: __u16,
    pub ctx_sid: __u32,
    pub __counted_by(ctx_len): char ctx_str[],
}

// Security Context Domains of Interpretation
pub const XFRM_SC_DOI_RESERVED: c_int = 0;
pub const XFRM_SC_DOI_LSM: c_int = 1;
// Security Context Algorithms
pub const XFRM_SC_ALG_RESERVED: c_int = 0;
pub const XFRM_SC_ALG_SELINUX: c_int = 1;
// Selector, used as selector both on policy rules (SPD) and SAs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_selector {
    pub daddr: xfrm_address_t,
    pub saddr: xfrm_address_t,
    pub dport: __be16,
    pub dport_mask: __be16,
    pub sport: __be16,
    pub sport_mask: __be16,
    pub family: __u16,
    pub prefixlen_d: __u8,
    pub prefixlen_s: __u8,
    pub proto: __u8,
    pub ifindex: c_int,
    pub user: __kernel_uid32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_lifetime_cfg {
    pub soft_byte_limit: __u64,
    pub hard_byte_limit: __u64,
    pub soft_packet_limit: __u64,
    pub hard_packet_limit: __u64,
    pub soft_add_expires_seconds: __u64,
    pub hard_add_expires_seconds: __u64,
    pub soft_use_expires_seconds: __u64,
    pub hard_use_expires_seconds: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_lifetime_cur {
    pub bytes: __u64,
    pub packets: __u64,
    pub add_time: __u64,
    pub use_time: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_replay_state {
    pub oseq: __u32,
    pub seq: __u32,
    pub bitmap: __u32,
}

pub const XFRMA_REPLAY_ESN_MAX: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_replay_state_esn {
    pub bmp_len: c_uint,
    pub oseq: __u32,
    pub seq: __u32,
    pub oseq_hi: __u32,
    pub seq_hi: __u32,
    pub replay_window: __u32,
    pub bmp: [__u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_algo {
    pub alg_name: [c_char; 64],
    pub /: *mut *mut unsigned int alg_key_len; / in bits,
    pub alg_key: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_algo_auth {
    pub alg_name: [c_char; 64],
    pub /: *mut *mut unsigned int alg_key_len; / in bits,
    pub /: *mut *mut unsigned int alg_trunc_len; / in bits,
    pub alg_key: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_algo_aead {
    pub alg_name: [c_char; 64],
    pub /: *mut *mut unsigned int alg_key_len; / in bits,
    pub /: *mut *mut unsigned int alg_icv_len; / in bits,
    pub alg_key: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_stats {
    pub replay_window: __u32,
    pub replay: __u32,
    pub integrity_failed: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfrm_sa_dir {
    XFRM_SA_DIR_IN	= 1,
    XFRM_SA_DIR_OUT = 2
}

pub const XFRM_MODE_TRANSPORT: c_int = 0;
pub const XFRM_MODE_TUNNEL: c_int = 1;
pub const XFRM_MODE_ROUTEOPTIMIZATION: c_int = 2;
pub const XFRM_MODE_IN_TRIGGER: c_int = 3;
pub const XFRM_MODE_BEET: c_int = 4;
pub const XFRM_MODE_IPTFS: c_int = 5;
pub const XFRM_MODE_MAX: c_int = 6;
// Netlink configuration messages.

//
// Generic LSM security context for communicating to user space
// NOTE: Same format as sadb_x_sec_ctx
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_user_sec_ctx {
    pub len: __u16,
    pub exttype: __u16,
    pub /: *mut *mut __u8 ctx_alg; / LSMs: e.g., selinux == 1,
    pub ctx_doi: __u8,
    pub ctx_len: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_user_tmpl {
    pub id: xfrm_id,
    pub family: __u16,
    pub saddr: xfrm_address_t,
    pub reqid: __u32,
    pub mode: __u8,
    pub share: __u8,
    pub optional: __u8,
    pub aalgos: __u32,
    pub ealgos: __u32,
    pub calgos: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_encap_tmpl {
    pub encap_type: __u16,
    pub encap_sport: __be16,
    pub encap_dport: __be16,
    pub encap_oa: xfrm_address_t,
}

// AEVENT flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfrm_ae_ftype_t {
    XFRM_AE_UNSPEC,
    XFRM_AE_RTHR=1,	/* replay threshold*/
    XFRM_AE_RVAL=2, /* replay value */
    XFRM_AE_LVAL=4, /* lifetime value */
    XFRM_AE_ETHR=8, /* expiry timer threshold */
    XFRM_AE_CR=16, /* Event cause is replay update */
    XFRM_AE_CE=32, /* Event cause is timer expiry */
    XFRM_AE_CU=64, /* Event cause is policy update */
    __XFRM_AE_MAX

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_userpolicy_type {
    pub type: __u8,
    pub reserved1: __u16,
    pub reserved2: __u8,
}

// Netlink message attributes.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfrm_attr_type_t {
    XFRMA_UNSPEC,
    XFRMA_ALG_AUTH,		/* struct xfrm_algo */
    XFRMA_ALG_CRYPT,	/* struct xfrm_algo */
    XFRMA_ALG_COMP,		/* struct xfrm_algo */
    XFRMA_ENCAP,		/* struct xfrm_algo + struct xfrm_encap_tmpl */
    XFRMA_TMPL,		/* 1 or more struct xfrm_user_tmpl */
    XFRMA_SA,		/* struct xfrm_usersa_info  */
    XFRMA_POLICY,		/*struct xfrm_userpolicy_info */
    XFRMA_SEC_CTX,		/* struct xfrm_sec_ctx */
    XFRMA_LTIME_VAL,
    XFRMA_REPLAY_VAL,
    XFRMA_REPLAY_THRESH,
    XFRMA_ETIMER_THRESH,
    XFRMA_SRCADDR,		/* xfrm_address_t */
    XFRMA_COADDR,		/* xfrm_address_t */
    XFRMA_LASTUSED,		/* __u64 */
    XFRMA_POLICY_TYPE,	/* struct xfrm_userpolicy_type */
    XFRMA_MIGRATE,
    XFRMA_ALG_AEAD,		/* struct xfrm_algo_aead */
    XFRMA_KMADDRESS,        /* struct xfrm_user_kmaddress */
    XFRMA_ALG_AUTH_TRUNC,	/* struct xfrm_algo_auth */
    XFRMA_MARK,		/* struct xfrm_mark */
    XFRMA_TFCPAD,		/* __u32 */
    XFRMA_REPLAY_ESN_VAL,	/* struct xfrm_replay_state_esn */
    XFRMA_SA_EXTRA_FLAGS,	/* __u32 */
    XFRMA_PROTO,		/* __u8 */
    XFRMA_ADDRESS_FILTER,	/* struct xfrm_address_filter */
    XFRMA_PAD,
    XFRMA_OFFLOAD_DEV,	/* struct xfrm_user_offload */
    XFRMA_SET_MARK,		/* __u32 */
    XFRMA_SET_MARK_MASK,	/* __u32 */
    XFRMA_IF_ID,		/* __u32 */
    XFRMA_MTIMER_THRESH,	/* __u32 in seconds for input SA */
    XFRMA_SA_DIR,		/* __u8 */
    XFRMA_NAT_KEEPALIVE_INTERVAL,	/* __u32 in seconds for NAT keepalive */
    XFRMA_SA_PCPU,		/* __u32 */
    XFRMA_IPTFS_DROP_TIME,	/* __u32 in: usec to wait for next seq */
    XFRMA_IPTFS_REORDER_WINDOW, /* __u16 in: reorder window size (pkts) */
    XFRMA_IPTFS_DONT_FRAG,	/* out: don't use fragmentation */
    XFRMA_IPTFS_INIT_DELAY,	/* __u32 out: initial packet wait delay (usec) */
    XFRMA_IPTFS_MAX_QSIZE,	/* __u32 out: max ingress queue size (octets) */
    XFRMA_IPTFS_PKT_SIZE,	/* __u32 out: size of outer packet, 0 for PMTU */
    __XFRMA_MAX

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_mark {
    pub /: *mut *mut __u32 v; / value,
    pub /: *mut *mut __u32 m; / mask,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfrm_sadattr_type_t {
    XFRMA_SAD_UNSPEC,
    XFRMA_SAD_CNT,
    XFRMA_SAD_HINFO,
    __XFRMA_SAD_MAX

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrmu_sadhinfo {
    pub /: *mut *mut __u32 sadhcnt; / current hash bkts,
    pub /: *mut *mut __u32 sadhmcnt; / max allowed hash bkts,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfrm_spdattr_type_t {
    XFRMA_SPD_UNSPEC,
    XFRMA_SPD_INFO,
    XFRMA_SPD_HINFO,
    XFRMA_SPD_IPV4_HTHRESH,
    XFRMA_SPD_IPV6_HTHRESH,
    __XFRMA_SPD_MAX

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrmu_spdinfo {
    pub incnt: __u32,
    pub outcnt: __u32,
    pub fwdcnt: __u32,
    pub inscnt: __u32,
    pub outscnt: __u32,
    pub fwdscnt: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrmu_spdhinfo {
    pub spdhcnt: __u32,
    pub spdhmcnt: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrmu_spdhthresh {
    pub lbits: __u8,
    pub rbits: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_usersa_info {
    pub sel: xfrm_selector,
    pub id: xfrm_id,
    pub saddr: xfrm_address_t,
    pub lft: xfrm_lifetime_cfg,
    pub curlft: xfrm_lifetime_cur,
    pub stats: xfrm_stats,
    pub seq: __u32,
    pub reqid: __u32,
    pub family: __u16,
    pub /: *mut *mut __u8 mode; / XFRM_MODE_xxx,
    pub replay_window: __u8,
    pub flags: __u8,
pub const XFRM_STATE_NOECN: c_int = 1;
pub const XFRM_STATE_DECAP_DSCP: c_int = 2;
pub const XFRM_STATE_NOPMTUDISC: c_int = 4;
pub const XFRM_STATE_WILDRECV: c_int = 8;
pub const XFRM_STATE_ICMP: c_int = 16;
pub const XFRM_STATE_AF_UNSPEC: c_int = 32;
pub const XFRM_STATE_ALIGN4: c_int = 64;
pub const XFRM_STATE_ESN: c_int = 128;
}

pub const XFRM_SA_XFLAG_DONT_ENCAP_DSCP: c_int = 1;
pub const XFRM_SA_XFLAG_OSEQ_MAY_WRAP: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_usersa_id {
    pub daddr: xfrm_address_t,
    pub spi: __be32,
    pub family: __u16,
    pub proto: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_aevent_id {
    pub sa_id: xfrm_usersa_id,
    pub saddr: xfrm_address_t,
    pub flags: __u32,
    pub reqid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_userspi_info {
    pub info: xfrm_usersa_info,
    pub min: __u32,
    pub max: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_userpolicy_info {
    pub sel: xfrm_selector,
    pub lft: xfrm_lifetime_cfg,
    pub curlft: xfrm_lifetime_cur,
    pub priority: __u32,
    pub index: __u32,
    pub dir: __u8,
    pub action: __u8,
pub const XFRM_POLICY_ALLOW: c_int = 0;
pub const XFRM_POLICY_BLOCK: c_int = 1;
    pub flags: __u8,

// Automatically expand selector to include matching ICMP payloads.
pub const XFRM_POLICY_ICMP: c_int = 2;
pub const XFRM_POLICY_CPU_ACQUIRE: c_int = 4;
    pub share: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_userpolicy_id {
    pub sel: xfrm_selector,
    pub index: __u32,
    pub dir: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_user_acquire {
    pub id: xfrm_id,
    pub saddr: xfrm_address_t,
    pub sel: xfrm_selector,
    pub policy: xfrm_userpolicy_info,
    pub aalgos: __u32,
    pub ealgos: __u32,
    pub calgos: __u32,
    pub seq: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_user_expire {
    pub state: xfrm_usersa_info,
    pub hard: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_user_polexpire {
    pub pol: xfrm_userpolicy_info,
    pub hard: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_usersa_flush {
    pub proto: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_user_report {
    pub proto: __u8,
    pub sel: xfrm_selector,
}

// Used by MIGRATE to pass addresses IKE should use to perform
// SA negotiation with the peer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_user_kmaddress {
    pub local: xfrm_address_t,
    pub remote: xfrm_address_t,
    pub reserved: __u32,
    pub family: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_user_migrate {
    pub old_daddr: xfrm_address_t,
    pub old_saddr: xfrm_address_t,
    pub new_daddr: xfrm_address_t,
    pub new_saddr: xfrm_address_t,
    pub proto: __u8,
    pub mode: __u8,
    pub reserved: __u16,
    pub reqid: __u32,
    pub old_family: __u16,
    pub new_family: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_user_migrate_state {
    pub id: xfrm_usersa_id,
    pub new_daddr: xfrm_address_t,
    pub new_saddr: xfrm_address_t,
    pub old_mark: xfrm_mark,
    pub new_sel: xfrm_selector,
    pub new_reqid: __u32,
    pub flags: __u32,
    pub new_family: __u16,
    pub reserved: __u16,
}

// Flags for xfrm_user_migrate_state.flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfrm_migrate_state_flags {
    XFRM_MIGRATE_STATE_CLEAR_OFFLOAD = 1, /* do not inherit offload from existing SA */
    XFRM_MIGRATE_STATE_UPDATE_H2H_SEL = 2, /* update H2H selector from new daddr/saddr */
}

// All flags defined as of this header version; unknown bits are rejected.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_user_mapping {
    pub id: xfrm_usersa_id,
    pub reqid: __u32,
    pub old_saddr: xfrm_address_t,
    pub new_saddr: xfrm_address_t,
    pub old_sport: __be16,
    pub new_sport: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_address_filter {
    pub saddr: xfrm_address_t,
    pub daddr: xfrm_address_t,
    pub family: __u16,
    pub splen: __u8,
    pub dplen: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_user_offload {
    pub ifindex: c_int,
    pub flags: __u8,
}

// This flag was exposed without any kernel code that supports it.
// Unfortunately, strongswan has the code that sets this flag,
// which makes it impossible to reuse this bit.
//
// So leave it here to make sure that it won't be reused by mistake.
//
pub const XFRM_OFFLOAD_IPV6: c_int = 1;
pub const XFRM_OFFLOAD_INBOUND: c_int = 2;
// Two bits above are relevant for state path only, while
// offload is used for both policy and state flows.
//
// In policy offload mode, they are free and can be safely reused.
//
pub const XFRM_OFFLOAD_PACKET: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_userpolicy_default {
pub const XFRM_USERPOLICY_UNSPEC: c_int = 0;
pub const XFRM_USERPOLICY_BLOCK: c_int = 1;
pub const XFRM_USERPOLICY_ACCEPT: c_int = 2;
    pub in: __u8,
    pub fwd: __u8,
    pub out: __u8,
}

// backwards compatibility for userspace
pub const XFRMGRP_ACQUIRE: c_int = 1;
pub const XFRMGRP_EXPIRE: c_int = 2;
pub const XFRMGRP_SA: c_int = 4;
pub const XFRMGRP_POLICY: c_int = 8;
pub const XFRMGRP_REPORT: c_uint = 0x20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfrm_nlgroups {
    XFRMNLGRP_NONE,

    XFRMNLGRP_ACQUIRE,

    XFRMNLGRP_EXPIRE,

    XFRMNLGRP_SA,

    XFRMNLGRP_POLICY,

    XFRMNLGRP_AEVENTS,

    XFRMNLGRP_REPORT,

    XFRMNLGRP_MIGRATE,

    XFRMNLGRP_MAPPING,

    __XFRMNLGRP_MAX
}

