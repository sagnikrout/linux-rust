//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netns/ipv6.h
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
// ipv6 in net namespaces
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netns_sysctl_ipv6 {

    pub hdr: *mut ctl_table_header,
    pub route_hdr: *mut ctl_table_header,
    pub icmp_hdr: *mut ctl_table_header,
    pub frags_hdr: *mut ctl_table_header,
    pub xfrm6_hdr: *mut ctl_table_header,

    pub flush_delay: c_int,
    pub ip6_rt_max_size: c_int,
    pub ip6_rt_gc_min_interval: c_int,
    pub ip6_rt_gc_timeout: c_int,
    pub ip6_rt_gc_interval: c_int,
    pub ip6_rt_gc_elasticity: c_int,
    pub ip6_rt_mtu_expires: c_int,
    pub ip6_rt_min_advmss: c_int,
    pub multipath_hash_fields: u32,
    pub multipath_hash_policy: u8,
    pub flowlabel_consistency: u8,
    pub auto_flowlabels: u8,
    pub flowlabel_state_ranges: u8,
    pub icmpv6_echo_ignore_all: u8,
    pub icmpv6_echo_ignore_multicast: u8,
    pub icmpv6_echo_ignore_anycast: u8,
    pub icmpv6_time: c_int,
    pub 1): DECLARE_BITMAP(icmpv6_ratemask, ICMPV6_MSG_MAX +,
    pub icmpv6_ratemask_ptr: *mut c_ulong,
    pub anycast_src_echo_reply: u8,
    pub bindv6only: u8,
    pub ip_nonlocal_bind: u8,
    pub fwmark_reflect: u8,
    pub idgen_retries: c_int,
    pub idgen_delay: c_int,
    pub flowlabel_reflect: c_int,
    pub max_dst_opts_cnt: c_int,
    pub max_hbh_opts_cnt: c_int,
    pub max_dst_opts_len: c_int,
    pub max_hbh_opts_len: c_int,
    pub seg6_flowlabel: c_int,
    pub ioam6_id: u32,
    pub ioam6_id_wide: u64,
    pub skip_notify_on_dev_down: u8,
    pub fib_notify_on_flag_change: u8,
    pub icmpv6_error_anycast_as_unicast: u8,
    pub icmpv6_errors_extension_mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netns_ipv6 {
// Keep ip6_dst_ops at the beginning of netns_sysctl_ipv6
    pub ip6_dst_ops: dst_ops,
    pub sysctl: netns_sysctl_ipv6,
    pub devconf_all: *mut ipv6_devconf,
    pub devconf_dflt: *mut ipv6_devconf,
    pub peers: *mut inet_peer_base,
    pub fqdir: *mut fqdir,
    pub fib6_null_entry: *mut fib6_info,
    pub ip6_null_entry: *mut rt6_info,
    pub rt6_stats: *mut rt6_statistics,
    pub ip6_fib_timer: timer_list,
    pub fib_table_hash: *mut hlist_head,
    pub fib_table_hash_lock: spinlock_t,
    pub fib6_main_tbl: *mut fib6_table,
    pub fib6_walkers: list_head,
    pub fib6_walker_lock: rwlock_t,
    pub fib6_gc_lock: spinlock_t,
    pub ip6_rt_gc_expire: core::sync::atomic::AtomicI32,
    pub ip6_rt_last_gc: c_ulong,
    pub flowlabel_has_excl: c_uchar,

    pub fib6_has_custom_rules: bool,
    pub fib6_rules_require_fldissect: c_uint,

    pub fib6_routes_require_src: c_uint,

    pub ip6_prohibit_entry: *mut rt6_info,
    pub ip6_blk_hole_entry: *mut rt6_info,
    pub fib6_local_tbl: *mut fib6_table,
    pub fib6_rules_ops: *mut fib_rules_ops,

    pub ndisc_sk: *mut sock,
    pub tcp_sk: *mut sock,
    pub igmp_sk: *mut sock,
    pub mc_autojoin_sk: *mut sock,
    pub inet6_addr_lst: *mut hlist_head,
    pub addrconf_hash_lock: spinlock_t,
    pub addr_chk_work: delayed_work,

    pub mrt6: *mut mr_table,

    pub mr6_tables: list_head,
    pub mr6_rules_ops: *mut fib_rules_ops,

    pub ip6mr_notifier_ops: *mut fib_notifier_ops,
    pub ipmr_seq: core::sync::atomic::AtomicI32,
    pub mfc_mutex: mutex,

    pub dev_addr_genid: core::sync::atomic::AtomicI32,
    pub fib6_sernum: core::sync::atomic::AtomicI32,
    pub seg6_data: *mut seg6_pernet_data,
    pub notifier_ops: *mut fib_notifier_ops,
    pub flowlabel_count: c_int,
    pub head: hlist_head,
    pub lock: spinlock_t,
    pub seq: u32,
    pub ip6addrlbl_table: },
    pub ioam6_data: *mut ioam6_pernet_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netns_nf_frag {
    pub fqdir: *mut fqdir,
}

