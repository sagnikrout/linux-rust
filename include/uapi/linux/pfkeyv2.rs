//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/pfkeyv2.h
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
// PF_KEY user interface, this is defined by rfc2367 so
// do not make arbitrary modifications or else this header
// file will not be compliant.
//

pub const PF_KEY_V2: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_msg {
    pub sadb_msg_version: __u8,
    pub sadb_msg_type: __u8,
    pub sadb_msg_errno: __u8,
    pub sadb_msg_satype: __u8,
    pub sadb_msg_len: __u16,
    pub sadb_msg_reserved: __u16,
    pub sadb_msg_seq: __u32,
    pub sadb_msg_pid: __u32,
    pub __attribute__((packed)): },
// sizeof(struct sadb_msg) == 16
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_ext {
    pub sadb_ext_len: __u16,
    pub sadb_ext_type: __u16,
    pub __attribute__((packed)): },
// sizeof(struct sadb_ext) == 4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_sa {
    pub sadb_sa_len: __u16,
    pub sadb_sa_exttype: __u16,
    pub sadb_sa_spi: __be32,
    pub sadb_sa_replay: __u8,
    pub sadb_sa_state: __u8,
    pub sadb_sa_auth: __u8,
    pub sadb_sa_encrypt: __u8,
    pub sadb_sa_flags: __u32,
    pub __attribute__((packed)): },
// sizeof(struct sadb_sa) == 16
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_lifetime {
    pub sadb_lifetime_len: __u16,
    pub sadb_lifetime_exttype: __u16,
    pub sadb_lifetime_allocations: __u32,
    pub sadb_lifetime_bytes: __u64,
    pub sadb_lifetime_addtime: __u64,
    pub sadb_lifetime_usetime: __u64,
    pub __attribute__((packed)): },
// sizeof(struct sadb_lifetime) == 32
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_address {
    pub sadb_address_len: __u16,
    pub sadb_address_exttype: __u16,
    pub sadb_address_proto: __u8,
    pub sadb_address_prefixlen: __u8,
    pub sadb_address_reserved: __u16,
    pub __attribute__((packed)): },
// sizeof(struct sadb_address) == 8
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_key {
    pub sadb_key_len: __u16,
    pub sadb_key_exttype: __u16,
    pub sadb_key_bits: __u16,
    pub sadb_key_reserved: __u16,
    pub __attribute__((packed)): },
// sizeof(struct sadb_key) == 8
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_ident {
    pub sadb_ident_len: __u16,
    pub sadb_ident_exttype: __u16,
    pub sadb_ident_type: __u16,
    pub sadb_ident_reserved: __u16,
    pub sadb_ident_id: __u64,
    pub __attribute__((packed)): },
// sizeof(struct sadb_ident) == 16
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_sens {
    pub sadb_sens_len: __u16,
    pub sadb_sens_exttype: __u16,
    pub sadb_sens_dpd: __u32,
    pub sadb_sens_sens_level: __u8,
    pub sadb_sens_sens_len: __u8,
    pub sadb_sens_integ_level: __u8,
    pub sadb_sens_integ_len: __u8,
    pub sadb_sens_reserved: __u32,
    pub __attribute__((packed)): },
// sizeof(struct sadb_sens) == 16
// followed by:
    pub sadb_sens_bitmap: [__u64; sens_len],
    pub /: *mut __u64 sadb_integ_bitmap[integ_len];,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_prop {
    pub sadb_prop_len: __u16,
    pub sadb_prop_exttype: __u16,
    pub sadb_prop_replay: __u8,
    pub sadb_prop_reserved: [__u8; 3],
    pub __attribute__((packed)): },
// sizeof(struct sadb_prop) == 8
// followed by:
    pub /: *mut sizeof(struct sadb_comb)];,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_comb {
    pub sadb_comb_auth: __u8,
    pub sadb_comb_encrypt: __u8,
    pub sadb_comb_flags: __u16,
    pub sadb_comb_auth_minbits: __u16,
    pub sadb_comb_auth_maxbits: __u16,
    pub sadb_comb_encrypt_minbits: __u16,
    pub sadb_comb_encrypt_maxbits: __u16,
    pub sadb_comb_reserved: __u32,
    pub sadb_comb_soft_allocations: __u32,
    pub sadb_comb_hard_allocations: __u32,
    pub sadb_comb_soft_bytes: __u64,
    pub sadb_comb_hard_bytes: __u64,
    pub sadb_comb_soft_addtime: __u64,
    pub sadb_comb_hard_addtime: __u64,
    pub sadb_comb_soft_usetime: __u64,
    pub sadb_comb_hard_usetime: __u64,
    pub __attribute__((packed)): },
// sizeof(struct sadb_comb) == 72
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_supported {
    pub sadb_supported_len: __u16,
    pub sadb_supported_exttype: __u16,
    pub sadb_supported_reserved: __u32,
    pub __attribute__((packed)): },
// sizeof(struct sadb_supported) == 8
// followed by:
    pub /: *mut sizeof(struct sadb_alg)];,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_alg {
    pub sadb_alg_id: __u8,
    pub sadb_alg_ivlen: __u8,
    pub sadb_alg_minbits: __u16,
    pub sadb_alg_maxbits: __u16,
    pub sadb_alg_reserved: __u16,
    pub __attribute__((packed)): },
// sizeof(struct sadb_alg) == 8
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_spirange {
    pub sadb_spirange_len: __u16,
    pub sadb_spirange_exttype: __u16,
    pub sadb_spirange_min: __u32,
    pub sadb_spirange_max: __u32,
    pub sadb_spirange_reserved: __u32,
    pub __attribute__((packed)): },
// sizeof(struct sadb_spirange) == 16
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_x_kmprivate {
    pub sadb_x_kmprivate_len: __u16,
    pub sadb_x_kmprivate_exttype: __u16,
    pub sadb_x_kmprivate_reserved: __u32,
    pub __attribute__((packed)): },
// sizeof(struct sadb_x_kmprivate) == 8
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_x_sa2 {
    pub sadb_x_sa2_len: __u16,
    pub sadb_x_sa2_exttype: __u16,
    pub sadb_x_sa2_mode: __u8,
    pub sadb_x_sa2_reserved1: __u8,
    pub sadb_x_sa2_reserved2: __u16,
    pub sadb_x_sa2_sequence: __u32,
    pub sadb_x_sa2_reqid: __u32,
    pub __attribute__((packed)): },
// sizeof(struct sadb_x_sa2) == 16
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_x_policy {
    pub sadb_x_policy_len: __u16,
    pub sadb_x_policy_exttype: __u16,
    pub sadb_x_policy_type: __u16,
    pub sadb_x_policy_dir: __u8,
    pub sadb_x_policy_reserved: __u8,
    pub sadb_x_policy_id: __u32,
    pub sadb_x_policy_priority: __u32,
    pub __attribute__((packed)): },
// sizeof(struct sadb_x_policy) == 16
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_x_ipsecrequest {
    pub sadb_x_ipsecrequest_len: __u16,
    pub sadb_x_ipsecrequest_proto: __u16,
    pub sadb_x_ipsecrequest_mode: __u8,
    pub sadb_x_ipsecrequest_level: __u8,
    pub sadb_x_ipsecrequest_reserved1: __u16,
    pub sadb_x_ipsecrequest_reqid: __u32,
    pub sadb_x_ipsecrequest_reserved2: __u32,
    pub __attribute__((packed)): },
// sizeof(struct sadb_x_ipsecrequest) == 16
// This defines the TYPE of Nat Traversal in use.  Currently only one
// type of NAT-T is supported, draft-ietf-ipsec-udp-encaps-06
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_x_nat_t_type {
    pub sadb_x_nat_t_type_len: __u16,
    pub sadb_x_nat_t_type_exttype: __u16,
    pub sadb_x_nat_t_type_type: __u8,
    pub sadb_x_nat_t_type_reserved: [__u8; 3],
    pub __attribute__((packed)): },
// sizeof(struct sadb_x_nat_t_type) == 8
// Pass a NAT Traversal port (Source or Dest port)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_x_nat_t_port {
    pub sadb_x_nat_t_port_len: __u16,
    pub sadb_x_nat_t_port_exttype: __u16,
    pub sadb_x_nat_t_port_port: __be16,
    pub sadb_x_nat_t_port_reserved: __u16,
    pub __attribute__((packed)): },
// sizeof(struct sadb_x_nat_t_port) == 8
// Generic LSM security context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_x_sec_ctx {
    pub sadb_x_sec_len: __u16,
    pub sadb_x_sec_exttype: __u16,
    pub /: *mut *mut __u8 sadb_x_ctx_alg; / LSMs: e.g., selinux == 1,
    pub sadb_x_ctx_doi: __u8,
    pub sadb_x_ctx_len: __u16,
    pub __attribute__((packed)): },
// sizeof(struct sadb_sec_ctx) = 8
// Used by MIGRATE to pass addresses IKE will use to perform
// negotiation with the peer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_x_kmaddress {
    pub sadb_x_kmaddress_len: __u16,
    pub sadb_x_kmaddress_exttype: __u16,
    pub sadb_x_kmaddress_reserved: __u32,
    pub __attribute__((packed)): },
// sizeof(struct sadb_x_kmaddress) == 8
// To specify the SA dump filter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sadb_x_filter {
    pub sadb_x_filter_len: __u16,
    pub sadb_x_filter_exttype: __u16,
    pub sadb_x_filter_saddr: [__u32; 4],
    pub sadb_x_filter_daddr: [__u32; 4],
    pub sadb_x_filter_family: __u16,
    pub sadb_x_filter_splen: __u8,
    pub sadb_x_filter_dplen: __u8,
    pub __attribute__((packed)): },
// sizeof(struct sadb_x_filter) == 40
// Message types
pub const SADB_RESERVED: c_int = 0;
pub const SADB_GETSPI: c_int = 1;
pub const SADB_UPDATE: c_int = 2;
pub const SADB_ADD: c_int = 3;
pub const SADB_DELETE: c_int = 4;
pub const SADB_GET: c_int = 5;
pub const SADB_ACQUIRE: c_int = 6;
pub const SADB_REGISTER: c_int = 7;
pub const SADB_EXPIRE: c_int = 8;
pub const SADB_FLUSH: c_int = 9;
pub const SADB_DUMP: c_int = 10;
pub const SADB_X_PROMISC: c_int = 11;
pub const SADB_X_PCHANGE: c_int = 12;
pub const SADB_X_SPDUPDATE: c_int = 13;
pub const SADB_X_SPDADD: c_int = 14;
pub const SADB_X_SPDDELETE: c_int = 15;
pub const SADB_X_SPDGET: c_int = 16;
pub const SADB_X_SPDACQUIRE: c_int = 17;
pub const SADB_X_SPDDUMP: c_int = 18;
pub const SADB_X_SPDFLUSH: c_int = 19;
pub const SADB_X_SPDSETIDX: c_int = 20;
pub const SADB_X_SPDEXPIRE: c_int = 21;
pub const SADB_X_SPDDELETE2: c_int = 22;
pub const SADB_X_NAT_T_NEW_MAPPING: c_int = 23;
pub const SADB_X_MIGRATE: c_int = 24;
pub const SADB_MAX: c_int = 24;
// Security Association flags
pub const SADB_SAFLAGS_PFS: c_int = 1;
pub const SADB_SAFLAGS_NOPMTUDISC: c_uint = 0x20000000;
pub const SADB_SAFLAGS_DECAP_DSCP: c_uint = 0x40000000;
pub const SADB_SAFLAGS_NOECN: c_uint = 0x80000000;
// Security Association states
pub const SADB_SASTATE_LARVAL: c_int = 0;
pub const SADB_SASTATE_MATURE: c_int = 1;
pub const SADB_SASTATE_DYING: c_int = 2;
pub const SADB_SASTATE_DEAD: c_int = 3;
pub const SADB_SASTATE_MAX: c_int = 3;
// Security Association types
pub const SADB_SATYPE_UNSPEC: c_int = 0;
pub const SADB_SATYPE_AH: c_int = 2;
pub const SADB_SATYPE_ESP: c_int = 3;
pub const SADB_SATYPE_RSVP: c_int = 5;
pub const SADB_SATYPE_OSPFV2: c_int = 6;
pub const SADB_SATYPE_RIPV2: c_int = 7;
pub const SADB_SATYPE_MIP: c_int = 8;
pub const SADB_X_SATYPE_IPCOMP: c_int = 9;
pub const SADB_SATYPE_MAX: c_int = 9;
// Authentication algorithms
pub const SADB_AALG_NONE: c_int = 0;
pub const SADB_AALG_MD5HMAC: c_int = 2;
pub const SADB_AALG_SHA1HMAC: c_int = 3;
pub const SADB_X_AALG_SHA2_256HMAC: c_int = 5;
pub const SADB_X_AALG_SHA2_384HMAC: c_int = 6;
pub const SADB_X_AALG_SHA2_512HMAC: c_int = 7;
pub const SADB_X_AALG_RIPEMD160HMAC: c_int = 8;
pub const SADB_X_AALG_AES_XCBC_MAC: c_int = 9;
pub const SADB_X_AALG_SM3_256HMAC: c_int = 10;

pub const SADB_AALG_MAX: c_int = 251;
// Encryption algorithms
pub const SADB_EALG_NONE: c_int = 0;
pub const SADB_EALG_DESCBC: c_int = 2;
pub const SADB_EALG_3DESCBC: c_int = 3;
pub const SADB_X_EALG_CASTCBC: c_int = 6;
pub const SADB_X_EALG_BLOWFISHCBC: c_int = 7;
pub const SADB_EALG_NULL: c_int = 11;
pub const SADB_X_EALG_AESCBC: c_int = 12;
pub const SADB_X_EALG_AESCTR: c_int = 13;
pub const SADB_X_EALG_AES_CCM_ICV8: c_int = 14;
pub const SADB_X_EALG_AES_CCM_ICV12: c_int = 15;
pub const SADB_X_EALG_AES_CCM_ICV16: c_int = 16;
pub const SADB_X_EALG_AES_GCM_ICV8: c_int = 18;
pub const SADB_X_EALG_AES_GCM_ICV12: c_int = 19;
pub const SADB_X_EALG_AES_GCM_ICV16: c_int = 20;
pub const SADB_X_EALG_CAMELLIACBC: c_int = 22;
pub const SADB_X_EALG_NULL_AES_GMAC: c_int = 23;
pub const SADB_X_EALG_SM4CBC: c_int = 24;

// private allocations should use 249-255 (RFC2407)

// Compression algorithms
pub const SADB_X_CALG_NONE: c_int = 0;
pub const SADB_X_CALG_OUI: c_int = 1;
pub const SADB_X_CALG_DEFLATE: c_int = 2;
pub const SADB_X_CALG_LZS: c_int = 3;
pub const SADB_X_CALG_LZJH: c_int = 4;
pub const SADB_X_CALG_MAX: c_int = 4;
// Extension Header values
pub const SADB_EXT_RESERVED: c_int = 0;
pub const SADB_EXT_SA: c_int = 1;
pub const SADB_EXT_LIFETIME_CURRENT: c_int = 2;
pub const SADB_EXT_LIFETIME_HARD: c_int = 3;
pub const SADB_EXT_LIFETIME_SOFT: c_int = 4;
pub const SADB_EXT_ADDRESS_SRC: c_int = 5;
pub const SADB_EXT_ADDRESS_DST: c_int = 6;
pub const SADB_EXT_ADDRESS_PROXY: c_int = 7;
pub const SADB_EXT_KEY_AUTH: c_int = 8;
pub const SADB_EXT_KEY_ENCRYPT: c_int = 9;
pub const SADB_EXT_IDENTITY_SRC: c_int = 10;
pub const SADB_EXT_IDENTITY_DST: c_int = 11;
pub const SADB_EXT_SENSITIVITY: c_int = 12;
pub const SADB_EXT_PROPOSAL: c_int = 13;
pub const SADB_EXT_SUPPORTED_AUTH: c_int = 14;
pub const SADB_EXT_SUPPORTED_ENCRYPT: c_int = 15;
pub const SADB_EXT_SPIRANGE: c_int = 16;
pub const SADB_X_EXT_KMPRIVATE: c_int = 17;
pub const SADB_X_EXT_POLICY: c_int = 18;
pub const SADB_X_EXT_SA2: c_int = 19;
// The next four entries are for setting up NAT Traversal
pub const SADB_X_EXT_NAT_T_TYPE: c_int = 20;
pub const SADB_X_EXT_NAT_T_SPORT: c_int = 21;
pub const SADB_X_EXT_NAT_T_DPORT: c_int = 22;
pub const SADB_X_EXT_NAT_T_OA: c_int = 23;
pub const SADB_X_EXT_SEC_CTX: c_int = 24;
// Used with MIGRATE to pass @ to IKE for negotiation
pub const SADB_X_EXT_KMADDRESS: c_int = 25;
pub const SADB_X_EXT_FILTER: c_int = 26;
pub const SADB_EXT_MAX: c_int = 26;
// Identity Extension values
pub const SADB_IDENTTYPE_RESERVED: c_int = 0;
pub const SADB_IDENTTYPE_PREFIX: c_int = 1;
pub const SADB_IDENTTYPE_FQDN: c_int = 2;
pub const SADB_IDENTTYPE_USERFQDN: c_int = 3;
pub const SADB_IDENTTYPE_MAX: c_int = 3;
