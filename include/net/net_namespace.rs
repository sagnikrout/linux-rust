//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/net_namespace.h
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
// Operations on the network namespace
//

pub const NETDEV_HASHBITS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net {
// First cache line can be often dirtied.
// Do not place here read-mostly fields.
//
    pub network: *mut *mut refcount_t passive; / To decide when the,
// namespace should be freed.
//
    pub rules_mod_lock: spinlock_t,
    pub /: *mut *mut unsigned int dev_base_seq; / protected by rtnl_mutex,
    pub ifindex: u32,
    pub nsid_lock: spinlock_t,
    pub fnhe_genid: core::sync::atomic::AtomicI32,
    pub /: *mut *mut list_head list; / list of network namespaces,
    pub exit: *mut *mut list_head exit_list; / To linked to call pernet,
// methods on dead net (
// pernet_ops_rwsem read locked),
// or to unregister pernet ops
// (pernet_ops_rwsem write locked).
//
    pub defer_free_list: llist_node,
    pub /: *mut *mut llist_node cleanup_list; / namespaces on death row,
    pub ptype_all: list_head,
    pub ptype_specific: list_head,

    pub /: *mut *mut *mut key_tag key_domain; / Key domain of operation tag,

    pub /: *mut *mut *mut user_namespace user_ns; / Owning user namespace,
    pub ucounts: *mut ucounts,
    pub netns_ids: idr,
    pub ns: ns_common,
    pub refcnt_tracker: ref_tracker_dir,
    pub not: *mut *mut ref_tracker_dir notrefcnt_tracker; / tracker for objects,
// refcounted against netns
//
    pub dev_base_head: list_head,
    pub proc_net: *mut proc_dir_entry,
    pub proc_net_stat: *mut proc_dir_entry,

    pub sysctls: ctl_table_set,

    pub /: *mut *mut *mut sock rtnl; / rtnetlink socket,
    pub genl_sock: *mut sock,
    pub /: *mut *mut *mut uevent_sock uevent_sock; / uevent socket,
    pub dev_name_head: *mut hlist_head,
    pub dev_index_head: *mut hlist_head,
    pub dev_by_index: xarray,
    pub netdev_chain: raw_notifier_head,
// Note that @hash_mix can be read millions times per second,
// it is critical that it is on a read_mostly cache line.
//
    pub hash_mix: u32,
    pub is_dying: bool,
    pub /: *mut *mut *mut net_device loopback_dev; / The loopback,
// core fib_rules
    pub rules_ops: list_head,
    pub core: netns_core,
    pub mib: netns_mib,
    pub packet: netns_packet,

    pub unx: netns_unix,

    pub nexthop: netns_nexthop,
    pub ipv4: netns_ipv4,

    pub ipv6: netns_ipv6,

    pub ieee802154_lowpan: netns_ieee802154_lowpan,

    pub sctp: netns_sctp,

    pub nf: netns_nf,

    pub ct: netns_ct,

    pub nft: netns_nftables,

    pub ft: netns_ft,

    pub wext_nlevents: sk_buff_head,

    pub gen: *mut net_generic __rcu,
// Used to store attached BPF programs
    pub bpf: netns_bpf,
// Note : following structs are cache line aligned

    pub xfrm: netns_xfrm,

    pub /: *mut *mut u64 net_cookie; / written once,

    pub ipvs: *mut netns_ipvs,

    pub mpls: netns_mpls,

    pub can: netns_can,

    pub xdp: netns_xdp,

    pub mctp: netns_mctp,

    pub crypto_nlsk: *mut sock,

    pub diag_nlsk: *mut sock,

    pub smc: netns_smc,

// Move to a better place when the config guard is removed.
    pub rtnl_mutex: mutex,
    pub rtnl_work: work_struct,
    pub dev_unreg_head: list_head,
    pub dev_unreg_lock: spinlock_t,

    pub vsock: netns_vsock,

    pub __randomize_layout: },

// Init's network namespace
    pub init_net: extern struct net,

    pub old_net): *mut net,
    pub gid): *const *const *const void net_ns_get_ownership(struct net net, kuid_t uid, kgid_t,
    pub net_ns_barrier(void): c_void,
    pub ns): *mut *mut ns_common get_net_ns(ns_common,
    pub fd): *mut *mut net get_net_ns_by_fd(int,
    pub cleanup_net_task: *mut extern struct task_struct,

    pub ERR_PTR(-EINVAL): return,
    pub old_net: return,
// uid = GLOBAL_ROOT_UID;
// gid = GLOBAL_ROOT_GID;
    pub ERR_PTR(-EINVAL): return,
    pub ERR_PTR(-EINVAL): return,

    pub net_namespace_list: extern struct list_head,
    pub pid): *mut *mut net get_net_ns_by_pid(pid_t,

    pub ipx_register_sysctl(void): c_void,
    pub ipx_unregister_sysctl(void): c_void,

// Macro flag: #define ipx_register_sysctl()
// Macro flag: #define ipx_unregister_sysctl()

    pub ns): return container_of(ns, struct net,,

    pub net): *mut void __put_net(struct net,
// Try using get_net_track() instead
    pub net: return,
// Used when we know struct net exists but we
// aren't guaranteed a previous reference count
// exists.  If the reference count is zero this
// function fails and returns NULL.
//
    pub NULL: net =,
    pub net: return,
// Try using put_net_track() instead
    pub net2: return net1 ==,
    pub 0: return ns_ref_read(net) !=,
    pub ): *mut void net_drop_ns(struct ns_common,
    pub net): *mut void net_passive_dec(struct net,

    pub net: return,
    pub net: return,
    pub 1: return,
    pub 1: return,

// Returns true if the netns initialization is completed successfully
    pub READ_ONCE(net->list.next): return,

    pub gfp): tracker,,

    pub gfp): __netns_tracker_alloc(net, tracker, true,,

    pub tracker): &net->notrefcnt_tracker,,

    pub gfp): netns_tracker_alloc(net, tracker,,
    pub net: return,
    pub true): __netns_tracker_free(net, tracker,,

    pub net: *mut net __rcu,

    pub possible_net_t: },

    pub net): rcu_assign_pointer(pnet->net,,

    pub true): return rcu_dereference_protected(pnet->net,,

    pub &init_net: return,

    pub rcu_dereference(pnet->net): return,

    pub &init_net: return,

// Protected by net_rwsem

// Macro flag: #define __net_init
// Macro flag: #define __net_exit
// Macro flag: #define __net_initdata
// Macro flag: #define __net_initconst

    pub gfp): *mut *mut *mut int peernet2id_alloc(struct net net, struct net peer, gfp_t,
    pub peer): *const *const int peernet2id(struct net net, struct net,
    pub peer): *const *const bool peernet_has_id(struct net net, struct net,
    pub id): *const *const *const net get_net_ns_by_id(net net, int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pernet_operations {
    pub list: list_head,
//
// Below methods are called without any exclusive locks.
// More than one net may be constructed and destructed
// in parallel on several cpus. Every pernet_operations
// have to keep in mind all other pernet_operations and
// to introduce a locking, if they share common resources.
//
// The only time they are called with exclusive lock is
// from register_pernet_subsys(), unregister_pernet_subsys()
// register_pernet_device() and unregister_pernet_device().
//
// Exit methods using blocking RCU primitives, such as
// synchronize_rcu(), should be implemented via exit_batch.
// Then, destruction of a group of net requires single
// synchronize_rcu() related to these pernet_operations,
// instead of separate synchronize_rcu() for every net.
// Please, avoid synchronize_rcu() at all, where it's possible.
//
// Note that a combination of pre_exit() and exit() can
// be used, since a synchronize_rcu() is guaranteed between
// the calls.
//
    pub net): *mut *mut int (init)(struct net,
    pub net): *mut *mut void (pre_exit)(struct net,
    pub net): *mut *mut void (exit)(struct net,
    pub net_exit_list): *mut *mut void (exit_batch)(struct list_head,
// Following method is called with RTNL held.
    pub dev_kill_list): *mut list_head,
    pub id: *const *const c_uint,
    pub size: usize,
}

//
// Use these carefully.  If you implement a network device and it
// needs per network namespace operations use device pernet operations,
// otherwise use pernet subsys operations.
//
// Network interfaces need to be removed from a dying netns _before_
// subsys notifiers can be called, as most of the network code cleanup
// (which is done from subsys notifiers) runs with the assumption that
// dev_remove_pack has been called so no new packets will arrive during
// and after the cleanup functions have been called.  dev_remove_pack
// is not per namespace so instead the guarantee of no more packets
// arriving in a network namespace is provided by ensuring that all
// network devices and all sockets have left the network namespace
// before the cleanup methods are called.
//
// For the longest time the ipv4 icmp code was registered as a pernet
// device which caused kernel oops, and panics during network
// namespace cleanup.   So please don't get this wrong.
//
extern "C" {
    pub fn register_pernet_subsys(: *mut pernet_operations) -> c_int;
}
extern "C" {
    pub fn unregister_pernet_subsys(: *mut pernet_operations);
}
extern "C" {
    pub fn register_pernet_device(: *mut pernet_operations) -> c_int;
}
extern "C" {
    pub fn unregister_pernet_device(: *mut pernet_operations);
}

extern "C" {
    pub fn net_sysctl_init() -> c_int;
}
extern "C" {
    pub fn unregister_net_sysctl_table(header: *mut ctl_table_header);
}

extern "C" {
    pub fn atomic_read(_arg: &net->ipv4.rt_genid) -> return;
}

extern "C" {
    pub fn atomic_read(_arg: &net->ipv6.fib6_sernum) -> return;
}

extern "C" {
    pub fn void(net: *mut *mut __fib6_flush_trees)(struct net) -> extern;
}

// For callers who don't really care about whether it's IPv4 or IPv6
extern "C" {
    pub fn atomic_read(_arg: &net->fnhe_genid) -> return;
}

extern "C" {
    pub fn net_ns_init();
}

