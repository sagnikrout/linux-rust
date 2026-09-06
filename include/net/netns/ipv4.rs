//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netns/ipv4.h
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
// ipv4 in net namespaces
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct local_ports {
    pub /: *mut *mut u32 range; / high << 16 | low,
    pub warned: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ping_group_range {
    pub lock: seqlock_t,
    pub range: [kgid_t; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_timewait_death_row {
    pub tw_refcount: refcount_t,
// Padding to avoid false sharing, tw_refcount can be often written
    pub ____cacheline_aligned_in_smp: *mut *mut inet_hashinfo hashinfo,
    pub sysctl_max_tw_buckets: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysctl_fib_multipath_hash_seed {
    pub user_seed: u32,
    pub mp_seed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_tunnel_gro {
    pub sk: *mut sock __rcu,
    pub list: hlist_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netns_ipv4 {
// Cacheline organization can be found documented in
// Documentation/networking/net_cachelines/netns_ipv4_sysctl.rst.
// Please update the document when adding new fields.
//
// TX readonly hotpath cache lines
    pub sysctl_tcp_early_retrans: u8,
    pub sysctl_tcp_tso_win_divisor: u8,
    pub sysctl_tcp_tso_rtt_log: u8,
    pub sysctl_tcp_autocorking: u8,
    pub sysctl_tcp_min_snd_mss: c_int,
    pub sysctl_tcp_notsent_lowat: c_uint,
    pub sysctl_tcp_limit_output_bytes: c_int,
    pub sysctl_tcp_min_rtt_wlen: c_int,
    pub sysctl_tcp_wmem: [c_int; 3],
    pub sysctl_ip_fwd_use_pmtu: u8,
// TXRX readonly hotpath cache lines
    pub sysctl_tcp_shrink_window: u8,
// RX readonly hotpath cache line
    pub sysctl_tcp_moderate_rcvbuf: u8,
    pub sysctl_ip_early_demux: u8,
    pub sysctl_tcp_early_demux: u8,
    pub sysctl_tcp_l3mdev_accept: u8,
// 3 bytes hole, try to pack
    pub sysctl_tcp_reordering: c_int,
    pub sysctl_tcp_rmem: [c_int; 3],
    pub sysctl_tcp_rcvbuf_low_rtt: c_int,
// ICMP rate limiter hot cache line.
    pub icmp_global_credit: core::sync::atomic::AtomicI32,
    pub icmp_global_stamp: u32,
    pub tcp_death_row: inet_timewait_death_row,
    pub udp_table: *mut udp_table,

// Not in a pernet subsys because need to be available at GRO stage
    pub udp_tunnel_gro: [udp_tunnel_gro; 2],
    pub forw_hdr: *mut ctl_table_header,
    pub frags_hdr: *mut ctl_table_header,
    pub ipv4_hdr: *mut ctl_table_header,
    pub route_hdr: *mut ctl_table_header,
    pub xfrm4_hdr: *mut ctl_table_header,

    pub devconf_all: *mut ipv4_devconf,
    pub devconf_dflt: *mut ipv4_devconf,
    pub ra_chain: *mut ip_ra_chain __rcu,
    pub ra_mutex: mutex,

    pub rules_ops: *mut fib_rules_ops,
    pub fib_main: *mut fib_table __rcu,
    pub fib_default: *mut fib_table __rcu,
    pub fib_table_hash_lock: spinlock_t,
    pub fib_rules_require_fldissect: c_uint,
    pub fib_has_custom_rules: bool,

    pub fib_has_custom_local_routes: bool,
    pub fib_offload_disabled: bool,

    pub fib_num_tclassid_users: core::sync::atomic::AtomicI32,

    pub fib_table_hash: *mut hlist_head,
    pub fibnl: *mut sock,
    pub fib_info_hash: *mut hlist_head,
    pub fib_info_hash_bits: c_uint,
    pub fib_info_cnt: c_uint,
    pub mc_autojoin_sk: *mut sock,
    pub peers: *mut inet_peer_base,
    pub fqdir: *mut fqdir,
    pub sysctl_icmp_echo_ignore_all: u8,
    pub sysctl_icmp_echo_enable_probe: u8,
    pub sysctl_icmp_echo_ignore_broadcasts: u8,
    pub sysctl_icmp_ignore_bogus_error_responses: u8,
    pub sysctl_icmp_errors_use_inbound_ifaddr: u8,
    pub sysctl_icmp_errors_extension_mask: u8,
    pub sysctl_icmp_ratelimit: c_int,
    pub sysctl_icmp_ratemask: c_int,
    pub sysctl_icmp_msgs_per_sec: c_int,
    pub sysctl_icmp_msgs_burst: c_int,
    pub ip_rt_min_pmtu: u32,
    pub ip_rt_mtu_expires: c_int,
    pub ip_rt_min_advmss: c_int,
    pub ip_local_ports: local_ports,
    pub sysctl_tcp_ecn: u8,
    pub sysctl_tcp_ecn_option: u8,
    pub sysctl_tcp_ecn_option_beacon: u8,
    pub sysctl_tcp_ecn_fallback: u8,
    pub sysctl_ip_default_ttl: u8,
    pub sysctl_ip_no_pmtu_disc: u8,
    pub sysctl_ip_fwd_update_priority: u8,
    pub sysctl_ip_nonlocal_bind: u8,
    pub sysctl_ip_autobind_reuse: u8,
// Shall we try to damage output packets if routing dev changes?
    pub sysctl_ip_dynaddr: u8,
    pub sysctl_ip_local_port_step_width: u32,

    pub sysctl_raw_l3mdev_accept: u8,

    pub sysctl_udp_early_demux: u8,
    pub sysctl_nexthop_compat_mode: u8,
    pub sysctl_fwmark_reflect: u8,
    pub sysctl_tcp_fwmark_accept: u8,
    pub sysctl_tcp_mtu_probing: u8,
    pub sysctl_tcp_mtu_probe_floor: c_int,
    pub sysctl_tcp_base_mss: c_int,
    pub sysctl_tcp_probe_threshold: c_int,
    pub sysctl_tcp_probe_interval: u32,
    pub sysctl_tcp_keepalive_time: c_int,
    pub sysctl_tcp_keepalive_intvl: c_int,
    pub sysctl_tcp_keepalive_probes: u8,
    pub sysctl_tcp_syn_retries: u8,
    pub sysctl_tcp_synack_retries: u8,
    pub sysctl_tcp_syncookies: u8,
    pub sysctl_tcp_migrate_req: u8,
    pub sysctl_tcp_comp_sack_nr: u8,
    pub sysctl_tcp_backlog_ack_defer: u8,
    pub sysctl_tcp_pingpong_thresh: u8,
    pub sysctl_tcp_retries1: u8,
    pub sysctl_tcp_retries2: u8,
    pub sysctl_tcp_orphan_retries: u8,
    pub sysctl_tcp_tw_reuse: u8,
    pub sysctl_tcp_tw_reuse_delay: c_uint,
    pub sysctl_tcp_fin_timeout: c_int,
    pub sysctl_tcp_sack: u8,
    pub sysctl_tcp_window_scaling: u8,
    pub sysctl_tcp_timestamps: u8,
    pub sysctl_tcp_rto_min_us: c_int,
    pub sysctl_tcp_rto_max_ms: c_int,
    pub sysctl_tcp_recovery: u8,
    pub sysctl_tcp_thin_linear_timeouts: u8,
    pub sysctl_tcp_slow_start_after_idle: u8,
    pub sysctl_tcp_retrans_collapse: u8,
    pub sysctl_tcp_stdurg: u8,
    pub sysctl_tcp_rfc1337: u8,
    pub sysctl_tcp_abort_on_overflow: u8,
    pub /: *mut *mut u8 sysctl_tcp_fack; / obsolete,
    pub sysctl_tcp_max_reordering: c_int,
    pub /: *mut *mut int sysctl_tcp_adv_win_scale; / obsolete,
    pub sysctl_tcp_dsack: u8,
    pub sysctl_tcp_app_win: u8,
    pub sysctl_tcp_frto: u8,
    pub sysctl_tcp_nometrics_save: u8,
    pub sysctl_tcp_no_ssthresh_metrics_save: u8,
    pub sysctl_tcp_workaround_signed_windows: u8,
    pub sysctl_tcp_challenge_ack_limit: c_int,
    pub sysctl_tcp_min_tso_segs: u8,
    pub sysctl_tcp_reflect_tos: u8,
    pub sysctl_tcp_invalid_ratelimit: c_int,
    pub sysctl_tcp_pacing_ss_ratio: c_int,
    pub sysctl_tcp_pacing_ca_ratio: c_int,
    pub sysctl_tcp_child_ehash_entries: c_uint,
    pub sysctl_tcp_comp_sack_rtt_percent: c_int,
    pub sysctl_tcp_comp_sack_delay_ns: c_ulong,
    pub sysctl_tcp_comp_sack_slack_ns: c_ulong,
    pub sysctl_max_syn_backlog: c_int,
    pub sysctl_tcp_fastopen: c_int,
    pub tcp_congestion_control: *const tcp_congestion_ops __rcu,
    pub tcp_fastopen_ctx: *mut tcp_fastopen_context __rcu,
    pub sysctl_tcp_fastopen_blackhole_timeout: c_uint,
    pub tfo_active_disable_times: core::sync::atomic::AtomicI32,
    pub tfo_active_disable_stamp: c_ulong,
    pub tcp_challenge_timestamp: u32,
    pub tcp_challenge_count: u32,
    pub sysctl_tcp_plb_enabled: u8,
    pub sysctl_tcp_plb_idle_rehash_rounds: u8,
    pub sysctl_tcp_plb_rehash_rounds: u8,
    pub sysctl_tcp_plb_suspend_rto_sec: u8,
    pub sysctl_tcp_plb_cong_thresh: c_int,
    pub sysctl_udp_wmem_min: c_int,
    pub sysctl_udp_rmem_min: c_int,
    pub sysctl_fib_notify_on_flag_change: u8,
    pub sysctl_tcp_syn_linear_timeouts: u8,

    pub sysctl_udp_l3mdev_accept: u8,

    pub sysctl_igmp_llm_reports: u8,
    pub sysctl_igmp_max_memberships: c_int,
    pub sysctl_igmp_max_msf: c_int,
    pub sysctl_igmp_qrv: c_int,
    pub ping_group_range: ping_group_range,
    pub ping_port_rover: u16,
    pub dev_addr_genid: core::sync::atomic::AtomicI32,
    pub sysctl_udp_child_hash_entries: c_uint,

    pub sysctl_local_reserved_ports: *mut c_ulong,
    pub sysctl_ip_prot_sock: c_int,

    pub mrt: *mut mr_table __rcu,

    pub mr_tables: list_head,
    pub mr_rules_ops: *mut fib_rules_ops,

    pub ipmr_notifier_ops: *mut fib_notifier_ops,
    pub ipmr_seq: core::sync::atomic::AtomicI32,
    pub mfc_mutex: mutex,

    pub sysctl_fib_multipath_hash_seed: sysctl_fib_multipath_hash_seed,
    pub sysctl_fib_multipath_hash_fields: u32,
    pub sysctl_fib_multipath_use_neigh: u8,
    pub sysctl_fib_multipath_hash_policy: u8,

    pub notifier_ops: *mut fib_notifier_ops,
    pub /: *mut *mut unsigned int fib_seq; / writes protected by rtnl_mutex,
    pub rt_genid: core::sync::atomic::AtomicI32,
    pub ip_id_key: siphash_key_t,
    pub inet_addr_lst: *mut hlist_head,
    pub addr_chk_work: delayed_work,
}
