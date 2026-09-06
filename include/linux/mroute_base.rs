//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mroute_base.h
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


//
// struct vif_device - interface representor for multicast routing
// @dev: network device being used
// @dev_tracker: refcount tracker for @dev reference
// @bytes_in: statistic; bytes ingressing
// @bytes_out: statistic; bytes egresing
// @pkt_in: statistic; packets ingressing
// @pkt_out: statistic; packets egressing
// @rate_limit: Traffic shaping (NI)
// @threshold: TTL threshold
// @flags: Control flags
// @link: Physical interface index
// @dev_parent_id: device parent id
// @local: Local address
// @remote: Remote address for tunnels
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vif_device {
    pub dev: *mut net_device __rcu,
    pub dev_tracker: netdevice_tracker,
    pub bytes_out: unsigned long bytes_in,,
    pub pkt_out: unsigned long pkt_in,,
    pub rate_limit: c_ulong,
    pub threshold: c_uchar,
    pub flags: c_ushort,
    pub link: c_int,
// Currently only used by ipmr
    pub dev_parent_id: netdev_phys_item_id,
    pub remote: __be32 local,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vif_entry_notifier_info {
    pub info: fib_notifier_info,
    pub dev: *mut net_device,
    pub vif_index: c_ushort,
    pub vif_flags: c_ushort,
    pub tb_id: u32,
}

extern "C" {
    pub fn call_fib_notifier(_arg: nb, _arg: event_type, _arg: &info.info) -> return;
}
extern "C" {
    pub fn call_fib_notifiers(_arg: net, _arg: event_type, _arg: &info.info) -> return;
}

// This one is nasty; value is defined in uapi using different symbols for
// mroute and morute6 but both map into same 32.
//
pub const MAXVIFS: c_int = 32;

// Note: This helper is deprecated.

// mfc_flags:
// MFC_STATIC - the entry was added statically (not by a routing daemon)
// MFC_OFFLOAD - the entry was offloaded to the hardware
//
// struct mr_mfc - common multicast routing entries
// @mnode: rhashtable list
// @mfc_parent: source interface (iif)
// @mfc_flags: entry flags
// @expires: unresolved entry expire time
// @unresolved: unresolved cached skbs
// @last_assert: time of last assert
// @minvif: minimum VIF id
// @maxvif: maximum VIF id
// @bytes: bytes that have passed for this entry
// @pkt: packets that have passed for this entry
// @wrong_if: number of wrong source interface hits
// @lastuse: time of last use of the group (traffic or update)
// @ttls: OIF TTL threshold array
// @refcount: reference count for this entry
// @list: global entry list
// @rcu: used for entry destruction
// @free: Operation used for freeing an entry under RCU
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mr_mfc {
    pub mnode: rhlist_head,
    pub mfc_parent: c_ushort,
    pub mfc_flags: c_int,
    pub expires: c_ulong,
    pub unresolved: sk_buff_head,
    pub unres: },
    pub last_assert: c_ulong,
    pub minvif: c_int,
    pub maxvif: c_int,
    pub bytes: atomic_long_t,
    pub pkt: atomic_long_t,
    pub wrong_if: atomic_long_t,
    pub lastuse: c_ulong,
    pub ttls: [c_uchar; MAXVIFS],
    pub refcount: refcount_t,
    pub res: },
    pub mfc_un: },
    pub list: list_head,
    pub rcu: rcu_head,
    pub head): *mut *mut void (free)(struct rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfc_entry_notifier_info {
    pub info: fib_notifier_info,
    pub mfc: *mut mr_mfc,
    pub tb_id: u32,
}

extern "C" {
    pub fn call_fib_notifier(_arg: nb, _arg: event_type, _arg: &info.info) -> return;
}
extern "C" {
    pub fn call_fib_notifiers(_arg: net, _arg: event_type, _arg: &info.info) -> return;
}
//
// struct mr_table_ops - callbacks and info for protocol-specific ops
// @rht_params: parameters for accessing the MFC hash
// @cmparg_any: a hash key to be used for matching on (*,*) routes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mr_table_ops {
    pub rht_params: *const rhashtable_params,
    pub cmparg_any: *mut c_void,
}

//
// struct mr_table - a multicast routing table
// @work: used for table destruction
// @list: entry within a list of multicast routing tables
// @net: net where this table belongs
// @ops: protocol specific operations
// @id: identifier of the table
// @mroute_sk: socket associated with the table
// @ipmr_expire_timer: timer for handling unresolved routes
// @mfc_unres_queue: list of unresolved MFC entries
// @vif_table: array containing all possible vifs
// @mfc_hash: Hash table of all resolved routes for easy lookup
// @mfc_cache_list: list of resovled routes for possible traversal
// @maxvif: Identifier of highest value vif currently in use
// @cache_resolve_queue_len: current size of unresolved queue
// @mroute_do_assert: Whether to inform userspace on wrong ingress
// @mroute_do_pim: Whether to receive IGMP PIMv1
// @mroute_reg_vif_num: PIM-device vif index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mr_table {
    pub work: rcu_work,
    pub list: list_head,
    pub net: possible_net_t,
    pub ops: mr_table_ops,
    pub id: u32,
    pub mroute_sk: *mut sock __rcu,
    pub ipmr_expire_timer: timer_list,
    pub mfc_unres_queue: list_head,
    pub vif_table: [vif_device; MAXVIFS],
    pub mfc_hash: rhltable,
    pub mfc_cache_list: list_head,
    pub maxvif: c_int,
    pub cache_resolve_queue_len: u32,
    pub mroute_do_assert: bool,
    pub mroute_do_pim: bool,
    pub mroute_do_wrvifwhole: bool,
    pub mroute_reg_vif_num: c_int,
}

extern "C" {
    pub fn mr_table_free(mrt: *mut mr_table);
}
// These actually return 'struct mr_mfc *', but to avoid need for explicit
// castings they simply return void.
//

extern "C" {
    pub fn mr_mfc_find_parent(_arg: mrt, _arg: hasharg, _arg: -1) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mr_vif_iter {
    pub p: seq_net_private,
    pub mrt: *mut mr_table,
    pub ct: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mr_mfc_iter {
    pub p: seq_net_private,
    pub mrt: *mut mr_table,
    pub cache: *mut list_head,
// Lock protecting the mr_table's unresolved queue
    pub lock: *mut spinlock_t,
}

// These actually return 'struct mr_mfc *', but to avoid need for explicit
// castings they simply return void.
//

