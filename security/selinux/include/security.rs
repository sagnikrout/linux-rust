//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/include/security.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Security server interface.
//
// Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
//

pub const SECSID_NULL: c_uint = 0x00000000 /* unspecified SID */;
pub const SECSID_WILD: c_uint = 0xffffffff /* wildcard SID */;
pub const SECCLASS_NULL: c_uint = 0x0000 /* no class */;
// Identify specific policy version changes
pub const POLICYDB_VERSION_BASE: c_int = 15;
pub const POLICYDB_VERSION_BOOL: c_int = 16;
pub const POLICYDB_VERSION_IPV6: c_int = 17;
pub const POLICYDB_VERSION_NLCLASS: c_int = 18;
pub const POLICYDB_VERSION_VALIDATETRANS: c_int = 19;
pub const POLICYDB_VERSION_MLS: c_int = 19;
pub const POLICYDB_VERSION_AVTAB: c_int = 20;
pub const POLICYDB_VERSION_RANGETRANS: c_int = 21;
pub const POLICYDB_VERSION_POLCAP: c_int = 22;
pub const POLICYDB_VERSION_PERMISSIVE: c_int = 23;
pub const POLICYDB_VERSION_BOUNDARY: c_int = 24;
pub const POLICYDB_VERSION_FILENAME_TRANS: c_int = 25;
pub const POLICYDB_VERSION_ROLETRANS: c_int = 26;
pub const POLICYDB_VERSION_NEW_OBJECT_DEFAULTS: c_int = 27;
pub const POLICYDB_VERSION_DEFAULT_TYPE: c_int = 28;
pub const POLICYDB_VERSION_CONSTRAINT_NAMES: c_int = 29;
pub const POLICYDB_VERSION_XPERMS_IOCTL: c_int = 30;
pub const POLICYDB_VERSION_INFINIBAND: c_int = 31;
pub const POLICYDB_VERSION_GLBLUB: c_int = 32;

// Range of policy versions we understand

// Mask for just the mount related flags
pub const SE_MNTMASK: c_uint = 0x0f;
// Super block security struct flags for mount options
// BE CAREFUL, these need to be the low order bits for selinux_get_mnt_opts
pub const CONTEXT_MNT: c_uint = 0x01;
pub const FSCONTEXT_MNT: c_uint = 0x02;
pub const ROOTCONTEXT_MNT: c_uint = 0x04;
pub const DEFCONTEXT_MNT: c_uint = 0x08;
pub const SBLABEL_MNT: c_uint = 0x10;
// Non-mount related flags
pub const SE_SBINITIALIZED: c_uint = 0x0100;
pub const SE_SBPROC: c_uint = 0x0200;
pub const SE_SBGENFS: c_uint = 0x0400;
pub const SE_SBGENFS_XATTR: c_uint = 0x0800;
pub const SE_SBNATIVE: c_uint = 0x1000;

//
// type_datum properties
// available at the kernel policy version >= POLICYDB_VERSION_BOUNDARY
//
pub const TYPEDATUM_PROPERTY_PRIMARY: c_uint = 0x0001;
pub const TYPEDATUM_PROPERTY_ATTRIBUTE: c_uint = 0x0002;
// limitation of boundary depth
pub const POLICYDB_BOUNDS_MAXDEPTH: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct selinux_state {

    pub enforcing: bool,

    pub initialized: bool,
    pub policycap: [bool; __POLICYDB_CAP_MAX],
    pub status_page: *mut page,
    pub status_lock: mutex,
    pub policy: *mut selinux_policy __rcu,
    pub policy_mutex: mutex,
    pub __randomize_layout: },
    pub selinux_avc_init(void): c_void,
    pub selinux_state: extern struct selinux_state,
// do a synchronized load to avoid race conditions
    pub smp_load_acquire(&selinux_state.initialized): return,
// do a synchronized write to avoid race conditions
    pub true): smp_store_release(&selinux_state.initialized,,

    pub READ_ONCE(selinux_state.enforcing): return,
    pub value): WRITE_ONCE(selinux_state.enforcing,,

    pub true: return,

// non-zero/true checkreqprot values are no longer supported
    pub 0: return,
    pub READ_ONCE(selinux_state.policycap[POLICYDB_CAP_NETPEER]): return,
    pub READ_ONCE(selinux_state.policycap[POLICYDB_CAP_OPENPERM]): return,
    pub READ_ONCE(selinux_state.policycap[POLICYDB_CAP_EXTSOCKCLASS]): return,
    pub READ_ONCE(selinux_state.policycap[POLICYDB_CAP_ALWAYSNETWORK]): return,
    pub READ_ONCE(selinux_state.policycap[POLICYDB_CAP_CGROUPSECLABEL]): return,
    pub READ_ONCE(selinux_state.policycap[POLICYDB_CAP_MEMFD_CLASS]): return,
    pub selinux_policy_convert_data: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct selinux_load_state {
    pub policy: *mut selinux_policy,
    pub convert_data: *mut selinux_policy_convert_data,
}

extern "C" {
    pub fn security_mls_enabled() -> c_int;
}
extern "C" {
    pub fn selinux_policy_commit(load_state: *mut selinux_load_state);
}
extern "C" {
    pub fn selinux_policy_cancel(load_state: *mut selinux_load_state);
}
extern "C" {
    pub fn security_read_policy(data: *mut c_void, len: *mut usize) -> c_int;
}
extern "C" {
    pub fn security_read_state_kernel(data: *mut c_void, len: *mut usize) -> c_int;
}
extern "C" {
    pub fn security_policycap_supported(req_cap: c_uint) -> c_int;
}
// Maximum supported number of permissions per class
pub const SEL_VEC_MAX: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct av_decision {
    pub allowed: u32,
    pub auditallow: u32,
    pub auditdeny: u32,
    pub seqno: u32,
    pub flags: u32,
}

pub const XPERMS_ALLOWED: c_int = 1;
pub const XPERMS_AUDITALLOW: c_int = 2;
pub const XPERMS_DONTAUDIT: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct extended_perms_data {
    pub p: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct extended_perms_decision {
    pub used: u8,
    pub driver: u8,
    pub base_perm: u8,
    pub allowed: *mut extended_perms_data,
    pub auditallow: *mut extended_perms_data,
    pub dontaudit: *mut extended_perms_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct extended_perms {
    pub /: *mut *mut u16 len; / length associated decision chain,
    pub /: *mut *mut u8 base_perms; / which base permissions are covered,
    pub /: *mut *mut extended_perms_data drivers; / flag drivers that are used,
}

// definitions of av_decision.flags
pub const AVD_FLAGS_PERMISSIVE: c_uint = 0x0001;
pub const AVD_FLAGS_NEVERAUDIT: c_uint = 0x0002;
extern "C" {
    pub fn security_member_sid(ssid: u32, tsid: u32, tclass: u16, out_sid: *mut u32) -> c_int;
}
extern "C" {
    pub fn security_change_sid(ssid: u32, tsid: u32, tclass: u16, out_sid: *mut u32) -> c_int;
}
extern "C" {
    pub fn security_sid_to_context(sid: u32, scontext: *mut c_char, scontext_len: *mut u32) -> c_int;
}
extern "C" {
    pub fn security_sid_to_context_force(sid: u32, scontext: *mut c_char, scontext_len: *mut u32) -> c_int;
}
extern "C" {
    pub fn security_sid_to_context_inval(sid: u32, scontext: *mut c_char, scontext_len: *mut u32) -> c_int;
}
extern "C" {
    pub fn security_context_str_to_sid(scontext: *const c_char, out_sid: *mut u32, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn security_port_sid(protocol: u8, port: u16, out_sid: *mut u32) -> c_int;
}
extern "C" {
    pub fn security_ib_pkey_sid(subnet_prefix: u64, pkey_num: u16, out_sid: *mut u32) -> c_int;
}
extern "C" {
    pub fn security_ib_endport_sid(dev_name: *const c_char, port_num: u8, out_sid: *mut u32) -> c_int;
}
extern "C" {
    pub fn security_netif_sid(name: *const c_char, if_sid: *mut u32) -> c_int;
}
extern "C" {
    pub fn security_node_sid(domain: u16, addr: *const c_void, addrlen: u32, out_sid: *mut u32) -> c_int;
}
extern "C" {
    pub fn security_bounded_transition(old_sid: u32, new_sid: u32) -> c_int;
}
extern "C" {
    pub fn security_sid_mls_copy(sid: u32, mls_sid: u32, new_sid: *mut u32) -> c_int;
}
extern "C" {
    pub fn security_get_reject_unknown() -> c_int;
}
extern "C" {
    pub fn security_get_allow_unknown() -> c_int;
}

extern "C" {
    pub fn security_fs_use(sb: *mut super_block) -> c_int;
}

extern "C" {
    pub fn security_netlbl_sid_to_secattr(sid: u32, secattr: *mut netlbl_lsm_secattr) -> c_int;
}

//
// status notifier using mmap interface
//
pub const SELINUX_KERNEL_STATUS_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct selinux_kernel_status {
    pub /: *mut *mut u32 version; / version number of the structure,
    pub /: *mut *mut u32 sequence; / sequence number of seqlock logic,
    pub /: *mut *mut u32 enforcing; / current setting of enforcing mode,
    pub /: *mut *mut u32 policyload; / times of policy reloaded,
    pub /: *mut *mut u32 deny_unknown; / current setting of deny_unknown,
//
// The version > 0 supports above members.
//
    pub __packed: },
    pub enforcing): extern void selinux_status_update_setenforce(bool,
    pub seqno): extern void selinux_status_update_policyload(u32,
    pub selinux_complete_init(void): extern void,
    pub selinux_null: extern struct path,
    pub val): extern void selnl_notify_setenforce(int,
    pub seqno): extern void selnl_notify_policyload(u32,
    pub perm): *mut extern int selinux_nlmsg_lookup(u16 sclass, u16 nlmsg_type, u32,
    pub avtab_cache_init(void): extern void,
    pub ebitmap_cache_init(void): extern void,
    pub hashtab_cache_init(void): extern void,
    pub page): *mut extern int security_sidtab_hash_stats(char,
