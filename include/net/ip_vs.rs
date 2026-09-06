//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ip_vs.h
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
// IP Virtual Server
// data structure and functionality definitions
//

pub const IP_VS_HDR_INVERSE: c_int = 1;
pub const IP_VS_HDR_ICMP: c_int = 2;
// Destination Server Flags
pub const IP_VS_DEST_F_OVERLOAD: c_uint = 0x0002		/* server is overloaded */;
// Destination Server Config Flags
pub const IP_VS_DEST_CF_AVAILABLE: c_uint = 0x0001		/* server is available */;
// conn_tab limits (as per Kconfig)
pub const IP_VS_CONN_TAB_MIN_BITS: c_int = 8;

pub const IP_VS_CONN_TAB_MAX_BITS: c_int = 27;

pub const IP_VS_CONN_TAB_MAX_BITS: c_int = 20;

// conn_max limits

// Limit of atomic_t but restricted by roundup_pow_of_two() in ip_vs_core.c

// svc_table limits
pub const IP_VS_SVC_TAB_MIN_BITS: c_int = 4;
pub const IP_VS_SVC_TAB_MAX_BITS: c_int = 20;
// Generic access of ipvs struct
// Connections' size value needed by ip_vs_ctl.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_iphdr {
    pub /: *mut *mut int hdr_flags; / ipvs flags,
    pub /: *mut *mut __u32 off; / Where IP or IPv4 header starts,
    pub starts: *mut *mut __u32 len; / IPv4 simply where L4,
// IPv6 where L4 Transport Header starts
    pub frag)*/: *mut *mut __u16 fragoffs; / IPv6 fragment offset, 0 if first frag (or not,
    pub protocol: __s16,
    pub flags: __s32,
    pub saddr: nf_inet_addr,
    pub daddr: nf_inet_addr,
}

extern "C" {
    pub fn skb_header_pointer(_arg: skb, _arg: offset, _arg: len, _arg: buffer) -> return;
}
// This function handles filling *ip_vs_iphdr, both for IPv4 and IPv6.
// IPv6 requires some extra work, as finding proper header position,
// depend on the IPv6 extension headers.
//

// ipv6_find_hdr() updates len, flags

extern "C" {
    pub fn ip_vs_fill_iph_skb_off(_arg: af, _arg: skb, _arg: offset, _arg: hdr_flags, _arg: iphdr) -> return;
}

extern "C" {
    pub fn ipv6_addr_equal(_arg: &a->in6, _arg: &b->in6) -> return;
}

extern "C" {
    pub fn ip_vs_get_debug_level() -> c_int;
}

// idx += len;

// Only use from within IP_VS_DBG_BUF() or IP_VS_ERR_BUF macros

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_aligned_lock {
    pub /: *mut *mut spinlock_t l; / Protect buckets,
    pub ____cacheline_aligned_in_smp: },
// For arrays per family
}

// work_flags
// The port number of FTP service (in network order).

// TCP State Values
// UDP State Values
// ICMP State Values
// SCTP State Values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ip_vs_sctp_states {
    IP_VS_SCTP_S_NONE,
    IP_VS_SCTP_S_INIT1,
    IP_VS_SCTP_S_INIT,
    IP_VS_SCTP_S_COOKIE_SENT,
    IP_VS_SCTP_S_COOKIE_REPLIED,
    IP_VS_SCTP_S_COOKIE_WAIT,
    IP_VS_SCTP_S_COOKIE,
    IP_VS_SCTP_S_COOKIE_ECHOED,
    IP_VS_SCTP_S_ESTABLISHED,
    IP_VS_SCTP_S_SHUTDOWN_SENT,
    IP_VS_SCTP_S_SHUTDOWN_RECEIVED,
    IP_VS_SCTP_S_SHUTDOWN_ACK_SENT,
    IP_VS_SCTP_S_REJECTED,
    IP_VS_SCTP_S_CLOSED,
    IP_VS_SCTP_S_LAST
}

// Connection templates use bits from state
pub const IP_VS_CTPL_S_NONE: c_uint = 0x0000;
pub const IP_VS_CTPL_S_ASSURED: c_uint = 0x0001;
pub const IP_VS_CTPL_S_LAST: c_uint = 0x0002;
// Delta sequence info structure
// Each ip_vs_conn has 2 (output AND input seq. changes).
// Only used in the VS/NAT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_seq {
    pub /: *mut *mut __u32 init_seq; / Add delta from this seq,
    pub /: *mut *mut __u32 delta; / Delta in sequence numbers,
    pub numbers: *mut *mut __u32 previous_delta; / Delta in sequence,
// before last resized pkt
}

// counters per cpu
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_counters {
    pub /: *mut *mut u64_stats_t conns; / connections scheduled,
    pub /: *mut *mut u64_stats_t inpkts; / incoming packets,
    pub /: *mut *mut u64_stats_t outpkts; / outgoing packets,
    pub /: *mut *mut u64_stats_t inbytes; / incoming bytes,
    pub /: *mut *mut u64_stats_t outbytes; / outgoing bytes,
}

// Stats per cpu
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_cpu_stats {
    pub cnt: ip_vs_counters,
    pub syncp: u64_stats_sync,
}

// Default nice for estimator kthreads
pub const IPVS_EST_NICE: c_int = 0;
// IPVS statistics objects
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_estimator {
    pub list: hlist_node,
    pub last_inbytes: u64,
    pub last_outbytes: u64,
    pub last_conns: u64,
    pub last_inpkts: u64,
    pub last_outpkts: u64,
    pub cps: u64,
    pub inpps: u64,
    pub outpps: u64,
    pub inbps: u64,
    pub outbps: u64,
    pub /: *mut *mut ktcid:8; / chain ID for kthread tick,
}

//
// IPVS statistics object, 64-bit kernel version of struct ip_vs_stats_user
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_kstats {
    pub /: *mut *mut u64 conns; / connections scheduled,
    pub /: *mut *mut u64 inpkts; / incoming packets,
    pub /: *mut *mut u64 outpkts; / outgoing packets,
    pub /: *mut *mut u64 inbytes; / incoming bytes,
    pub /: *mut *mut u64 outbytes; / outgoing bytes,
    pub /: *mut *mut u64 cps; / current connection rate,
    pub /: *mut *mut u64 inpps; / current in packet rate,
    pub /: *mut *mut u64 outpps; / current out packet rate,
    pub /: *mut *mut u64 inbps; / current in byte rate,
    pub /: *mut *mut u64 outbps; / current out byte rate,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_stats {
    pub /: *mut *mut ip_vs_kstats kstats; / kernel statistics,
    pub /: *mut *mut ip_vs_estimator est; / estimator,
    pub /: *mut *mut *mut ip_vs_cpu_stats __percpu cpustats; / per cpu counters,
    pub /: *mut *mut spinlock_t lock; / spin lock,
    pub /: *mut *mut ip_vs_kstats kstats0; / reset values,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_stats_rcu {
    pub s: ip_vs_stats,
    pub rcu_head: rcu_head,
}

extern "C" {
    pub fn ip_vs_stats_init_alloc(s: *mut ip_vs_stats) -> c_int;
}
extern "C" {
    pub fn ip_vs_stats_release(stats: *mut ip_vs_stats);
}
extern "C" {
    pub fn ip_vs_stats_free(stats: *mut ip_vs_stats);
}
// Process estimators in multiple timer ticks (20/50/100, see ktrow)
pub const IPVS_EST_NTICKS: c_int = 50;
// Estimation uses a 2-second period containing ticks (in jiffies)

// Limit of CPU load per kthread (8 for 12.5%), ratio of CPU capacity (1/C).
// Value of 4 and above ensures kthreads will take work without exceeding
// the CPU capacity under different circumstances.
//
pub const IPVS_EST_LOAD_DIVISOR: c_int = 8;
// Kthreads should not have work that exceeds the CPU load above 50%

// Desired number of chains per timer tick (chain load factor in 100us units),
// 48=4.8ms of 40ms tick (12% CPU usage):
// 2 sec * 1000 ms in sec * 10 (100us in ms) / 8 (12.5%) / 50
//

// Compiled number of chains per tick
// The defines should match cond_resched_rcu
//

pub const IPVS_EST_TICK_CHAINS: c_int = 1;

// Multiple chains processed in same tick
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_est_tick_data {
    pub rcu_head: rcu_head,
    pub chains: [hlist_head; IPVS_EST_TICK_CHAINS],
    pub IPVS_EST_TICK_CHAINS): DECLARE_BITMAP(present,,
    pub IPVS_EST_TICK_CHAINS): DECLARE_BITMAP(full,,
    pub chain_len: [c_int; IPVS_EST_TICK_CHAINS],
}

// Context for estimation kthread
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_est_kt_data {
    pub ipvs: *mut netns_ipvs,
    pub /: *mut *mut *mut task_task; / task if running,
    pub ticks: [*mut ip_vs_est_tick_data __rcu; IPVS_EST_NTICKS],
    pub /: *mut *mut DECLARE_BITMAP(avail, IPVS_EST_NTICKS); / tick has space for ests,
    pub /: *mut *mut unsigned long est_timer; / estimation timer (jiffies),
    pub /: *mut *mut *mut ip_vs_stats calc_stats; / Used for calculation,
    pub /: *mut *mut int needed; / task is needed,
    pub /: *mut *mut int tick_len[IPVS_EST_NTICKS]; / est count,
    pub /: *mut *mut int id; / ktid per netns,
    pub /: *mut *mut int chain_max; / max ests per tick chain,
    pub /: *mut *mut int tick_max; / max ests per tick,
    pub /: *mut *mut int est_count; / attached ests to kthread,
    pub /: *mut *mut int est_max_count; / max ests per kthread,
    pub /: *mut *mut int add_row; / row for new ests,
    pub /: *mut *mut int est_row; / estimated row,
}

// IPVS resizable hash tables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_rht {
    pub buckets: *mut hlist_bl_head,
    pub /: *mut *mut *mut ip_vs_rht __rcu new_tbl; / New/Same table,
    pub /: *mut *mut *mut seqcount_t seqc; / Protects moves,
    pub /: *mut *mut *mut ip_vs_aligned_lock lock; / Protect seqc,
    pub /: *mut *mut int mask; / Buckets mask,
    pub /: *mut *mut int size; / Buckets,
    pub /: *mut *mut int seqc_mask; / seqc mask,
    pub /: *mut *mut int lock_mask; / lock mask,
    pub table_id: u32,
    pub /: *mut *mut int u_thresh; / upper threshold,
    pub /: *mut *mut int l_thresh; / lower threshold,
    pub (shift)*/: *mut *mut int lfactor; / Load Factor,
    pub /: *mut *mut int bits; / size = 1 << bits,
    pub hash_key: siphash_key_t,
    pub rcu_head: rcu_head,
}

//
// ip_vs_rht_for_each_table() - Walk the hash tables
// @table:	struct ip_vs_rht __rcu *table
// @t:		current table, used as cursor, struct ip_vs_rht *var
// @p:		previous table, temp struct ip_vs_rht *var
//
// Walk tables assuming others can not change the installed tables
//

//
// ip_vs_rht_for_each_table_rcu() - Walk the hash tables under RCU reader lock
// @table:	struct ip_vs_rht __rcu *table
// @t:		current table, used as cursor, struct ip_vs_rht *var
// @p:		previous table, temp struct ip_vs_rht *var
//
// We usually search in one table and also in second table on resizing
//

//
// ip_vs_rht_for_each_bucket() - Walk all table buckets
// @t:		current table, used as cursor, struct ip_vs_rht *var
// @bucket:	bucket index, used as cursor, u32 var
// @head:	bucket address, used as cursor, struct hlist_bl_head *var
//

//
// ip_vs_rht_for_bucket_retry() - Retry bucket if entries are moved
// @t:		current table, used as cursor, struct ip_vs_rht *var
// @bucket:	index of current bucket or hash key
// @sc:		temp seqcount_t *var
// @seq:	temp unsigned int var for sequence count
// @retry:	temp int var
//

//
// DECLARE_IP_VS_RHT_WALK_BUCKETS_RCU() - Declare variables
//
// Variables for ip_vs_rht_walk_buckets_rcu
//

//
// ip_vs_rht_walk_buckets_rcu() - Walk all buckets under RCU read lock
// @table:	struct ip_vs_rht __rcu *table
// @head:	bucket address, used as cursor, struct hlist_bl_head *var
//
// Can be used while others add/delete/move entries
// Not suitable if duplicates are not desired
// Possible cases for reader that uses cond_resched_rcu() in the loop:
// - new table can not be installed, no need to repeat
// - new table can be installed => check and repeat if new table is
// installed, needed for !PREEMPT_RCU
//

//
// DECLARE_IP_VS_RHT_WALK_BUCKET_RCU() - Declare variables
//
// Variables for ip_vs_rht_walk_bucket_rcu
//

//
// ip_vs_rht_walk_bucket_rcu() - Walk bucket under RCU read lock
// @t:		current table, struct ip_vs_rht *var
// @bucket:	index of current bucket or hash key
// @head:	bucket address, used as cursor, struct hlist_bl_head *var
//
// Can be used while others add/delete/move entries
// Not suitable if duplicates are not desired
// Possible cases for reader that uses cond_resched_rcu() in the loop:
// - new table can not be installed, no need to repeat
// - new table can be installed => check and repeat if new table is
// installed, needed for !PREEMPT_RCU
//

//
// DECLARE_IP_VS_RHT_WALK_BUCKETS_SAFE_RCU() - Declare variables
//
// Variables for ip_vs_rht_walk_buckets_safe_rcu
//

//
// ip_vs_rht_walk_buckets_safe_rcu() - Walk all buckets under RCU read lock
// @table:	struct ip_vs_rht __rcu *table
// @head:	bucket address, used as cursor, struct hlist_bl_head *var
//
// Can be used while others add/delete entries but moving is disabled
// Using cond_resched_rcu() should be safe if tables do not change
//

//
// DECLARE_IP_VS_RHT_WALK_BUCKETS() - Declare variables
//
// Variables for ip_vs_rht_walk_buckets
//

//
// ip_vs_rht_walk_buckets() - Walk all buckets
// @table:	struct ip_vs_rht __rcu *table
// @head:	bucket address, used as cursor, struct hlist_bl_head *var
//
// Use if others can not add/delete/move entries
//

// Entries can be in one of two tables, so we flip bit when new table is
// created and store it as highest bit in hash keys
//

// Check if hash key is from this table
// Build per-table hash key from hash value
extern "C" {
    pub fn ip_vs_rht_free(t: *mut ip_vs_rht);
}
extern "C" {
    pub fn ip_vs_rht_rcu_free(head: *mut rcu_head);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_protocol {
    pub next: *mut ip_vs_protocol,
    pub name: *mut c_char,
    pub protocol: u16,
    pub num_states: u16,
    pub dont_defrag: c_int,
    pub pp): *mut *mut void (init)(struct ip_vs_protocol,
    pub pp): *mut *mut void (exit)(struct ip_vs_protocol,
    pub pd): *mut *mut *mut int (init_netns)(struct netns_ipvs ipvs, struct ip_vs_proto_data,
    pub pd): *mut *mut *mut void (exit_netns)(struct netns_ipvs ipvs, struct ip_vs_proto_data,
    pub iph): *mut ip_vs_iphdr,
    pub iph): *const ip_vs_iphdr,
    pub iph): *const ip_vs_iphdr,
    pub iph): *mut *mut ip_vs_conn cp, ip_vs_iphdr,
    pub iph): *mut *mut ip_vs_conn cp, ip_vs_iphdr,
    pub state): *const *const *const char (state_name)(int,
    pub iph_len): c_uint,
    pub inc): *mut *mut *mut int (register_app)(struct netns_ipvs ipvs, struct ip_vs_app,
    pub inc): *mut *mut *mut void (unregister_app)(struct netns_ipvs ipvs, struct ip_vs_app,
    pub cp): *mut *mut int (app_conn_bind)(struct ip_vs_conn,
    pub msg): *const c_char,
    pub flags): *mut *mut *mut void (timeout_change)(struct ip_vs_proto_data pd, int,
}

// protocol data per netns
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_proto_data {
    pub next: *mut ip_vs_proto_data,
    pub pp: *mut ip_vs_protocol,
    pub /: *mut *mut *mut int timeout_table; / protocol timeout table,
    pub /: *mut *mut atomic_t appcnt; / counter of proto app incs.,
    pub tcp_state_table: *mut tcp_states_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_conn_param {
    pub ipvs: *mut netns_ipvs,
    pub caddr: *const nf_inet_addr,
    pub vaddr: *const nf_inet_addr,
    pub cport: __be16,
    pub vport: __be16,
    pub protocol: __u16,
    pub af: u16,
    pub pe: *const ip_vs_pe,
    pub pe_data: *mut c_char,
    pub pe_data_len: __u8,
}

// Hash node in conn_tab
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_conn_hnode {
    pub /: *mut *mut hlist_bl_node node; / node in conn_tab,
    pub /: *mut *mut u32 hash_key; / Key for the hash table,
    pub /: *mut *mut u8 dir; / 0=out->in, 1=in->out,
    pub __packed: },
// IP_VS structure allocated for each dynamically scheduled connection
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_conn {
// Cacheline for hash table nodes - rarely modified
    pub /: *mut *mut ip_vs_conn_hnode hn0; / Original direction,
    pub /: *mut *mut u8 af; / address family,
    pub cport: __be16,
    pub /: *mut *mut ip_vs_conn_hnode hn1; / Reply direction,
    pub /: *mut *mut u8 daf; / Address family of the dest,
    pub dport: __be16,
    pub /: *mut *mut *mut ip_vs_dest dest; / real server,
    pub /: *mut *mut atomic_t n_control; / Number of controlled ones,
    pub /: *mut *mut volatile __u32 flags; / status flags,
// 44/64
    pub /: *mut *mut *mut ip_vs_conn control; / Master control connection,
    pub pe: *const ip_vs_pe,
    pub pe_data: *mut c_char,
    pub pe_data_len: __u8,
    pub /: *mut *mut volatile __u16 state; / state info,
    pub for: *mut *mut volatile __u16 old_state; / old state, to be used,
// state transition triggered
// synchronization
//
// 2-byte hole
// 64/96
    pub /: *mut *mut nf_inet_addr caddr; / client address,
    pub /: *mut *mut nf_inet_addr vaddr; / virtual address,
// 96/128
    pub /: *mut *mut nf_inet_addr daddr; / destination address,
    pub /: *mut *mut __u32 fwmark; / Fire wall mark from skb,
    pub vport: __be16,
    pub /: *mut *mut __u16 protocol; / Which protocol (TCP/UDP),
// Note: we can group the following members into a structure,
// in order to save more space, and the following members are
// only used in VS/NAT anyway
//
    pub /: *mut *mut *mut ip_vs_app app; / bound ip_vs_app object,
    pub /: *mut *mut *mut void app_data; / Application private data,
// 128/168
    pub /: *mut *mut ip_vs_seq in_seq; / incoming seq. struct,
    pub /: *mut *mut ip_vs_seq out_seq; / outgoing seq. struct,
// 152/192
    pub /: *mut *mut timer_list timer; / Expiration timer,
    pub /: *mut *mut volatile unsigned long timeout; / timeout,
    pub /: *mut *mut spinlock_t lock; / lock for state transition,
    pub /: *mut *mut refcount_t refcnt; / reference count,
    pub /: *mut *mut atomic_t in_pkts; / incoming packet counter,
// 64-bit: 4-byte gap
// 188/256
    pub /: *mut *mut unsigned long sync_endtime; / jiffies + sent_retries,
    pub ipvs: *mut netns_ipvs,
// Packet transmitter for different forwarding methods.  If it
// mangles the packet, it must return NF_DROP or better NF_STOLEN,
// otherwise this must be changed to a sk_buff **.
// NF_ACCEPT can be returned when destination is local.
//
    pub iph): *mut *mut ip_vs_protocol pp, ip_vs_iphdr,
    pub rcu_head: rcu_head,
}

// Extended internal versions of struct ip_vs_service_user and ip_vs_dest_user
// for IPv6 support.
//
// We need these to conveniently pass around service and destination
// options, but unfortunately, we also need to keep the old definitions to
// maintain userspace backwards compatibility for the setsockopt interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_service_user_kern {
// virtual service addresses
    pub af: u16,
    pub protocol: u16,
    pub /: *mut *mut nf_inet_addr addr; / virtual ip address,
    pub port: __be16,
    pub /: *mut *mut u32 fwmark; / firewall mark of service,
// virtual service options
    pub sched_name: *mut c_char,
    pub pe_name: *mut c_char,
    pub /: *mut *mut unsigned int flags; / virtual service flags,
    pub /: *mut *mut unsigned int timeout; / persistent timeout in sec,
    pub /: *mut *mut __be32 netmask; / persistent netmask or plen,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_dest_user_kern {
// destination server address
    pub addr: nf_inet_addr,
    pub port: __be16,
// real server options
    pub /: *mut *mut unsigned int conn_flags; / connection flags,
    pub /: *mut *mut int weight; / destination weight,
// thresholds for active connections
    pub /: *mut *mut u32 u_threshold; / upper threshold,
    pub /: *mut *mut u32 l_threshold; / lower threshold,
// Address family of addr
    pub af: u16,
    pub /: *mut *mut u16 tun_type; / tunnel type,
    pub /: *mut *mut __be16 tun_port; / tunnel port,
    pub /: *mut *mut u16 tun_flags; / tunnel flags,
}

//
// The information about the virtual service offered to the net and the
// forwarding entries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_service {
    pub /: *mut *mut hlist_bl_node s_list; / node in service table,
    pub /: *mut *mut u32 hash_key; / Key for the hash table,
    pub /: *mut *mut u16 af; / address family,
    pub /: *mut *mut __u16 protocol; / which protocol (TCP/UDP),
    pub /: *mut *mut nf_inet_addr addr; / IP address for virtual service,
    pub /: *mut *mut __u32 fwmark; / firewall mark of the service,
    pub /: *mut *mut atomic_t refcnt; / reference counter,
    pub /: *mut *mut __be16 port; / port number for the service,
    pub /: *mut *mut unsigned int flags; / service status flags,
    pub /: *mut *mut unsigned int timeout; / persistent timeout in ticks,
    pub /: *mut *mut __be32 netmask; / grouping granularity, mask/plen,
    pub ipvs: *mut netns_ipvs,
    pub /: *mut *mut list_head destinations; / real server d-linked list,
    pub /: *mut *mut __u32 num_dests; / number of servers,
    pub /: *mut *mut ip_vs_stats stats; / statistics for the service,
// for scheduling
    pub /: *mut *mut *mut ip_vs_scheduler __rcu scheduler; / bound scheduler object,
    pub /: *mut *mut spinlock_t sched_lock; / lock sched_data,
    pub /: *mut *mut *mut void sched_data; / scheduler application data,
// alternate persistence engine
    pub pe: *mut ip_vs_pe __rcu,
    pub conntrack_afmask: c_int,
    pub rcu_head: rcu_head,
}

// Information for cached dst
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_dest_dst {
    pub /: *mut *mut *mut dst_entry dst_cache; / destination cache entry,
    pub dst_cookie: u32,
    pub dst_saddr: nf_inet_addr,
    pub rcu_head: rcu_head,
}

// The real server destination forwarding entry with ip address, port number,
// and so on.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_dest {
    pub /: *mut *mut list_head n_list; / for the dests in the service,
    pub /: *mut *mut hlist_node d_list; / for table with all the dests,
    pub /: *mut *mut u16 af; / address family,
    pub /: *mut *mut __be16 port; / port number of the server,
    pub /: *mut *mut nf_inet_addr addr; / IP address of the server,
    pub /: *mut *mut volatile unsigned int flags; / dest status flags,
    pub /: *mut *mut atomic_t conn_flags; / flags to copy to conn,
    pub /: *mut *mut atomic_t weight; / server weight,
    pub /: *mut *mut unsigned long cflags; / config flags,
    pub /: *mut *mut atomic_t last_weight; / server latest weight,
    pub /: *mut *mut __u16 tun_type; / tunnel type,
    pub /: *mut *mut __be16 tun_port; / tunnel port,
    pub /: *mut *mut __u16 tun_flags; / tunnel flags,
    pub /: *mut *mut refcount_t refcnt; / reference counter,
    pub /: *mut *mut ip_vs_stats stats; / statistics,
    pub /: *mut *mut unsigned long idle_start; / start time, jiffies,
// connection counters and thresholds
    pub /: *mut *mut atomic_t activeconns; / active connections,
    pub /: *mut *mut atomic_t totalconns; / total connections,
    pub /: *mut *mut atomic_t persistconns; / persistent connections,
    pub /: *mut *mut __u32 u_threshold; / upper threshold,
    pub /: *mut *mut __u32 l_threshold; / lower threshold,
    pub /: *mut *mut __u32 l_threshold_val;/ used lower threshold,
// for destination cache
    pub /: *mut *mut spinlock_t dst_lock; / lock of dst_cache,
    pub /: *mut *mut *mut ip_vs_dest_dst __rcu dest_dst; / cached dst info,
// for virtual service
    pub /: *mut *mut *mut ip_vs_service __rcu svc; / service it belongs to,
    pub /: *mut *mut __u16 protocol; / which protocol (TCP/UDP),
    pub /: *mut *mut __be16 vport; / virtual port number,
    pub /: *mut *mut nf_inet_addr vaddr; / virtual IP address,
    pub /: *mut *mut __u32 vfwmark; / firewall mark of service,
    pub rcu_head: rcu_head,
    pub /: *mut *mut list_head t_list; / in dest_trash,
    pub /: *mut *mut unsigned int in_rs_table:1; / we are in rs_table,
}

// The scheduler object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_scheduler {
    pub /: *mut *mut list_head n_list; / d-linked list head,
    pub /: *mut *mut *mut char name; / scheduler name,
    pub /: *mut *mut atomic_t refcnt; / reference counter,
    pub /: *mut *mut *mut module module; / THIS_MODULE/NULL,
// scheduler initializing service
    pub svc): *mut *mut int (init_service)(struct ip_vs_service,
// scheduling service finish
    pub svc): *mut *mut void (done_service)(struct ip_vs_service,
// dest is linked
    pub dest): *mut *mut *mut int (add_dest)(struct ip_vs_service svc, struct ip_vs_dest,
// dest is unlinked
    pub dest): *mut *mut *mut int (del_dest)(struct ip_vs_service svc, struct ip_vs_dest,
// dest is updated
    pub dest): *mut *mut *mut int (upd_dest)(struct ip_vs_service svc, struct ip_vs_dest,
// selecting a server from the given service
    pub iph): *mut ip_vs_iphdr,
}

// The persistence engine object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_pe {
    pub /: *mut *mut list_head n_list; / d-linked list head,
    pub /: *mut *mut *mut char name; / scheduler name,
    pub /: *mut *mut atomic_t refcnt; / reference counter,
    pub /: *mut *mut *mut module module; / THIS_MODULE/NULL,
// get the connection template, if any
    pub skb): *mut *mut *mut int (fill_param)(struct ip_vs_conn_param p, struct sk_buff,
    pub ct): *mut ip_vs_conn,
    pub inverse): *mut *mut ip_vs_rht t, bool,
    pub buf): *const *const *const int (show_pe_data)(struct ip_vs_conn cp, char,
// create connections for real-server outgoing packets
    pub cport): __be16 dport, __be16,
}

// The application module object (a.k.a. app incarnation)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_app {
    pub /: *mut *mut list_head a_list; / member in app list,
    pub /: *mut *mut int type; / IP_VS_APP_TYPE_xxx,
    pub /: *mut *mut *mut char name; / application module name,
    pub protocol: __u16,
    pub /: *mut *mut *mut module module; / THIS_MODULE/NULL,
    pub /: *mut *mut list_head incs_list; / list of incarnations,
// members for application incarnations
    pub /: *mut *mut list_head p_list; / member in proto app list,
    pub /: *mut *mut *mut ip_vs_app app; / its real application,
    pub /: *mut *mut __be16 port; / port number in net order,
    pub /: *mut *mut atomic_t usecnt; / usage counter,
    pub rcu_head: rcu_head,
// output hook: Process packet in inout direction, diff set for TCP.
// Return: 0=Error, 1=Payload Not Mangled/Mangled but checksum is ok,
// 2=Mangled but checksum was not updated
//
    pub ipvsh): *mut *mut *mut sk_buff , int diff, ip_vs_iphdr,
// input hook: Process packet in outin direction, diff set for TCP.
// Return: 0=Error, 1=Payload Not Mangled/Mangled but checksum is ok,
// 2=Mangled but checksum was not updated
//
    pub ipvsh): *mut *mut *mut sk_buff , int diff, ip_vs_iphdr,
// ip_vs_app initializer
    pub ): *mut *mut *mut int (init_conn)(struct ip_vs_app , struct ip_vs_conn,
// ip_vs_app finish
    pub ): *mut *mut *mut int (done_conn)(struct ip_vs_app , struct ip_vs_conn,
// not used now
    pub ): *mut ip_vs_protocol,
    pub ): *mut *mut *mut void (unbind_conn)(struct ip_vs_app , struct ip_vs_conn,
    pub timeout_table: *mut *mut c_int,
    pub timeouts: *mut *mut c_int,
    pub timeouts_size: c_int,
    pub cpp): *mut *mut int verdict, struct ip_vs_conn,
    pub inverse): *const *const iphdr iph, int,
    pub inverse): *const *const iphdr iph, int,
    pub app): *mut ip_vs_app,
    pub flags): *mut *mut *mut void (timeout_change)(struct ip_vs_app app, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipvs_master_sync_state {
    pub sync_queue: list_head,
    pub sync_buff: *mut ip_vs_sync_buff,
    pub sync_queue_len: c_ulong,
    pub sync_queue_delay: c_uint,
    pub master_wakeup_work: delayed_work,
    pub ipvs: *mut netns_ipvs,
}

// How much time to keep dests in trash

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipvs_sync_daemon_cfg {
    pub mcast_group: nf_inet_addr,
    pub syncid: c_int,
    pub sync_maxlen: u16,
    pub mcast_port: u16,
    pub mcast_af: u8,
    pub mcast_ttl: u8,
// multicast interface name
    pub mcast_ifn: [c_char; IP_VS_IFNAME_MAXLEN],
}

// IPVS in network namespace
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netns_ipvs {
    pub /: *mut *mut int gen; / Generation,
    pub /: *mut *mut int enable; / enable like nf_hooks do,
// Hash table: for real service lookups
pub const IP_VS_RTAB_BITS: c_int = 4;
    pub rs_table: [hlist_head; IP_VS_RTAB_SIZE],
// ip_vs_app
    pub app_list: list_head,
// ip_vs_proto
    pub proto_data_table: [*mut ip_vs_proto_data; IP_VS_PROTO_TAB_SIZE],
// ip_vs_proto_tcp

pub const TCP_APP_TAB_BITS: c_int = 4;
    pub tcp_apps: [list_head; TCP_APP_TAB_SIZE],
// ip_vs_proto_udp

pub const UDP_APP_TAB_BITS: c_int = 4;
    pub udp_apps: [list_head; UDP_APP_TAB_SIZE],
// ip_vs_proto_sctp

pub const SCTP_APP_TAB_BITS: c_int = 4;

// Hash table for SCTP application incarnations
    pub sctp_apps: [list_head; SCTP_APP_TAB_SIZE],
// ip_vs_conn
    pub /: *mut *mut atomic_t conn_count; / connection counter,
    pub no_cport_conns: [core::sync::atomic::AtomicI32; IP_VS_AF_MAX],
    pub /: *mut *mut delayed_work conn_resize_work;/ resize conn_tab,
// ip_vs_ctl
    pub /: *mut *mut *mut ip_vs_stats_rcu tot_stats; / Statistics & est.,
// Trash for destinations
    pub dest_trash: list_head,
    pub dest_trash_lock: spinlock_t,
    pub /: *mut *mut timer_list dest_trash_timer; / expiration timer,
    pub /: *mut *mut mutex service_mutex; / service reconfig,
    pub /: *mut *mut rw_semaphore svc_resize_sem; / svc_table resizing,
    pub /: *mut *mut rw_semaphore svc_replace_sem; / svc_table replace,
    pub /: *mut *mut delayed_work svc_resize_work; / resize svc_table,
    pub /: *mut *mut atomic_t svc_table_changes;/ ++ on table changes,
// Service counters
    pub /: *mut *mut atomic_t num_services[IP_VS_AF_MAX]; / Services,
    pub /: *mut *mut atomic_t fwm_services[IP_VS_AF_MAX]; / Services,
    pub /: *mut *mut atomic_t nonfwm_services[IP_VS_AF_MAX];/ Services,
    pub /: *mut *mut atomic_t ftpsvc_counter[IP_VS_AF_MAX]; / FTPPORT,
    pub /: *mut *mut atomic_t nullsvc_counter[IP_VS_AF_MAX];/ Zero port,
    pub /: *mut *mut atomic_t conn_out_counter[IP_VS_AF_MAX];/ out conn,

// delayed work for expiring no dest connections
    pub expire_nodest_conn_work: delayed_work,
// 1/rate drop and drop-entry variables
    pub /: *mut *mut delayed_work defense_work; / Work handler,
    pub drop_rate: c_int,
    pub drop_counter: c_int,
    pub old_secure_tcp: c_int,
    pub dropentry: core::sync::atomic::AtomicI32,
    pub dropentry_counters: [i8; 8],
// locks in ctl.c
    pub /: *mut *mut spinlock_t dropentry_lock; / drop entry handling,
    pub /: *mut *mut spinlock_t droppacket_lock; / drop packet handling,
    pub /: *mut *mut spinlock_t securetcp_lock; / state and timeout tables,
// sys-ctl struct
    pub sysctl_hdr: *mut ctl_table_header,
    pub sysctl_tbl: *mut ctl_table,

// sysctl variables
    pub sysctl_amemthresh: c_int,
    pub sysctl_am_droprate: c_int,

    pub /: *mut *mut int sysctl_conn_max;/ soft limit for conns,
    pub /: *mut *mut int conn_max_limit; / hard limit for conn_max,

    pub sysctl_drop_entry: c_int,
    pub sysctl_drop_packet: c_int,
    pub sysctl_secure_tcp: c_int,

    pub sysctl_conntrack: c_int,

    pub sysctl_snat_reroute: c_int,
    pub sysctl_sync_ver: c_int,
    pub sysctl_sync_ports: c_int,
    pub sysctl_sync_persist_mode: c_int,
    pub sysctl_sync_qlen_max: c_ulong,
    pub sysctl_sync_sock_size: c_int,
    pub sysctl_cache_bypass: c_int,
    pub sysctl_expire_nodest_conn: c_int,
    pub sysctl_sloppy_tcp: c_int,
    pub sysctl_sloppy_sctp: c_int,
    pub sysctl_expire_quiescent_template: c_int,
    pub sysctl_sync_threshold: [c_int; 2],
    pub sysctl_sync_refresh_period: c_uint,
    pub sysctl_sync_retries: c_int,
    pub sysctl_nat_icmp_send: c_int,
    pub sysctl_pmtu_disc: c_int,
    pub sysctl_backup_only: c_int,
    pub sysctl_conn_reuse_mode: c_int,
    pub sysctl_schedule_icmp: c_int,
    pub sysctl_ignore_tunneled: c_int,
    pub sysctl_run_estimation: c_int,

    pub /: *mut *mut cpumask_var_t sysctl_est_cpulist; / kthread cpumask,
    pub /: *mut *mut int est_cpulist_valid; / cpulist set,
    pub /: *mut *mut int sysctl_est_nice; / kthread nice,
    pub /: *mut *mut int est_stopped; / stop tasks,

    pub sysctl_conn_lfactor: c_int,
    pub sysctl_svc_lfactor: c_int,
// ip_vs_lblc
    pub sysctl_lblc_expiration: c_int,
    pub lblc_ctl_header: *mut ctl_table_header,
    pub lblc_ctl_table: *mut ctl_table,
// ip_vs_lblcr
    pub sysctl_lblcr_expiration: c_int,
    pub lblcr_ctl_header: *mut ctl_table_header,
    pub lblcr_ctl_table: *mut ctl_table,
    pub /: *mut *mut *mut unsigned long work_flags; / IP_VS_WORK_ flags,
// ip_vs_est
    pub /: *mut *mut delayed_work est_reload_work;/ Reload kthread tasks,
    pub /: *mut *mut mutex est_mutex; / protect kthread tasks,
    pub /: *mut *mut hlist_head est_temp_list; / Ests during calc phase,
    pub /: *mut *mut *mut *mut ip_vs_est_kt_data est_kt_arr; / Array of kthread data ptrs,
    pub /: *mut *mut unsigned long est_max_threads;/ Hard limit of kthreads,
    pub /: *mut *mut int est_calc_phase; / Calculation phase,
    pub /: *mut *mut int est_chain_max; / Calculated chain_max,
    pub /: *mut *mut int est_kt_count; / Allocated ptrs,
    pub /: *mut *mut int est_add_ktid; / ktid where to add ests,
    pub /: *mut *mut atomic_t est_genid; / kthreads reload genid,
    pub /: *mut *mut atomic_t est_genid_done; / applied genid,
// ip_vs_sync
    pub sync_lock: spinlock_t,
    pub ms: *mut ipvs_master_sync_state,
    pub sync_buff_lock: spinlock_t,
    pub master_tinfo: *mut ip_vs_sync_thread_data,
    pub backup_tinfo: *mut ip_vs_sync_thread_data,
    pub threads_mask: c_int,
    pub sync_state: volatile int,
    pub sync_mutex: mutex,
    pub /: *mut *mut ipvs_sync_daemon_cfg mcfg; / Master Configuration,
    pub /: *mut *mut ipvs_sync_daemon_cfg bcfg; / Backup Configuration,
// net name space ptr
    pub /: *mut *mut *mut net net; / Needed by timer routines,
// Number of heterogeneous destinations, needed because heterogeneous
// are not supported when synchronization is enabled.
//
    pub mixed_address_family_dests: c_uint,
    pub /: *mut *mut unsigned int hooks_afmask; / &1=AF_INET, &2=AF_INET6,
    pub /: *mut *mut *mut ip_vs_rht __rcu svc_table; / Services,
    pub /: *mut *mut *mut ip_vs_rht __rcu conn_tab; / Connections,
    pub /: *mut *mut atomic_t conn_tab_changes;/ ++ on new table,
}

pub const DEFAULT_SYNC_THRESHOLD: c_int = 3;
pub const DEFAULT_SYNC_PERIOD: c_int = 50;
pub const DEFAULT_SYNC_VER: c_int = 1;
pub const DEFAULT_SLOPPY_TCP: c_int = 0;
pub const DEFAULT_SLOPPY_SCTP: c_int = 0;

pub const DEFAULT_SYNC_RETRIES: c_int = 0;
pub const IPVS_SYNC_WAKEUP_RATE: c_int = 8;

extern "C" {
    pub fn READ_ONCE(_arg: ipvs->sysctl_conn_max) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: ipvs->sysctl_sync_threshold[1]) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: ipvs->sysctl_sync_refresh_period) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: ipvs->sysctl_sync_ports) -> return;
}
extern "C" {
    pub fn housekeeping_cpumask(_arg: HK_TYPE_KTHREAD) -> return;
}

extern "C" {
    pub fn housekeeping_cpumask(_arg: HK_TYPE_KTHREAD) -> return;
}

// Get load factor to map conn_count/u_thresh to t->size
extern "C" {
    pub fn READ_ONCE(_arg: ipvs->sysctl_conn_lfactor) -> return;
}
// Get load factor to map num_services/u_thresh to t->size
// Smaller value decreases u_thresh to reduce collisions but increases
// the table size
// Returns factor where:
// - <0: u_thresh = size >> -factor, eg. lfactor -2 = 25% load
// - >=0: u_thresh = size << factor, eg. lfactor 1 = 200% load
//
extern "C" {
    pub fn READ_ONCE(_arg: ipvs->sysctl_svc_lfactor) -> return;
}
extern "C" {
    pub fn cpumask_empty(_arg: __sysctl_est_cpulist(ipvs)) -> return;
}
extern "C" {
    pub fn cpumask_weight(_arg: __sysctl_est_cpulist(ipvs)) -> return;
}
// IPVS core functions
// (from ip_vs_core.c)
//
extern "C" {
    pub fn ip_vs_init_hash_table(table: *mut list_head, rows: c_int);
}

pub const IP_VS_APP_TYPE_FTP: c_int = 1;
// ip_vs_conn handling functions
// (from ip_vs_conn.c)
//
// Get reference to gain full access to conn.
// By default, RCU read-side critical sections have access only to
// conn fields and its PE data, see ip_vs_conn_rcu_free() for reference.
//
extern "C" {
    pub fn refcount_inc_not_zero(_arg: &cp->refcnt) -> return;
}
// put back the conn without restarting its timer
extern "C" {
    pub fn ip_vs_conn_put(cp: *mut ip_vs_conn);
}
extern "C" {
    pub fn ip_vs_conn_fill_cport(cp: *mut ip_vs_conn, cport: __be16);
}
extern "C" {
    pub fn container_of(_arg: hn, ip_vs_conn: struct, _arg: hn0) -> return;
}
extern "C" {
    pub fn ip_vs_conn_expire_now(cp: *mut ip_vs_conn);
}
extern "C" {
    pub fn ip_vs_tcp_conn_listen(cp: *mut ip_vs_conn);
}
extern "C" {
    pub fn ip_vs_check_template(ct: *mut ip_vs_conn, cdest: *mut ip_vs_dest) -> c_int;
}
extern "C" {
    pub fn ip_vs_random_dropentry(ipvs: *mut netns_ipvs);
}
extern "C" {
    pub fn ip_vs_conn_init() -> c_int;
}
extern "C" {
    pub fn ip_vs_conn_cleanup();
}
// Mark our template as assured
// IPVS netns init & cleanup functions
extern "C" {
    pub fn ip_vs_estimator_net_init(ipvs: *mut netns_ipvs) -> c_int;
}
extern "C" {
    pub fn ip_vs_control_net_init(ipvs: *mut netns_ipvs) -> c_int;
}
extern "C" {
    pub fn ip_vs_protocol_net_init(ipvs: *mut netns_ipvs) -> c_int;
}
extern "C" {
    pub fn ip_vs_app_net_init(ipvs: *mut netns_ipvs) -> c_int;
}
extern "C" {
    pub fn ip_vs_conn_net_init(ipvs: *mut netns_ipvs) -> c_int;
}
extern "C" {
    pub fn ip_vs_sync_net_init(ipvs: *mut netns_ipvs) -> c_int;
}
extern "C" {
    pub fn ip_vs_conn_net_cleanup(ipvs: *mut netns_ipvs);
}
extern "C" {
    pub fn ip_vs_app_net_cleanup(ipvs: *mut netns_ipvs);
}
extern "C" {
    pub fn ip_vs_protocol_net_cleanup(ipvs: *mut netns_ipvs);
}
extern "C" {
    pub fn ip_vs_control_net_cleanup(ipvs: *mut netns_ipvs);
}
extern "C" {
    pub fn ip_vs_estimator_net_cleanup(ipvs: *mut netns_ipvs);
}
extern "C" {
    pub fn ip_vs_sync_net_cleanup(ipvs: *mut netns_ipvs);
}
extern "C" {
    pub fn ip_vs_service_nets_cleanup(net_list: *mut list_head);
}
// IPVS application functions
// (from ip_vs_app.c)
//
pub const IP_VS_APP_MAX_PORTS: c_int = 8;
extern "C" {
    pub fn unregister_ip_vs_app(ipvs: *mut netns_ipvs, app: *mut ip_vs_app);
}
extern "C" {
    pub fn ip_vs_bind_app(cp: *mut ip_vs_conn, pp: *mut ip_vs_protocol) -> c_int;
}
extern "C" {
    pub fn ip_vs_unbind_app(cp: *mut ip_vs_conn);
}
extern "C" {
    pub fn ip_vs_app_inc_get(inc: *mut ip_vs_app) -> c_int;
}
extern "C" {
    pub fn ip_vs_app_inc_put(inc: *mut ip_vs_app);
}
extern "C" {
    pub fn register_ip_vs_pe(pe: *mut ip_vs_pe) -> c_int;
}
extern "C" {
    pub fn unregister_ip_vs_pe(pe: *mut ip_vs_pe) -> c_int;
}
// Use a #define to avoid all of module.h just for these trivial ops

// IPVS protocol functions (from ip_vs_proto.c)
extern "C" {
    pub fn ip_vs_protocol_init() -> c_int;
}
extern "C" {
    pub fn ip_vs_protocol_cleanup();
}
extern "C" {
    pub fn ip_vs_protocol_timeout_change(ipvs: *mut netns_ipvs, flags: c_int);
}
// Registering/unregistering scheduler functions
// (from ip_vs_sched.c)
//
extern "C" {
    pub fn register_ip_vs_scheduler(scheduler: *mut ip_vs_scheduler) -> c_int;
}
extern "C" {
    pub fn unregister_ip_vs_scheduler(scheduler: *mut ip_vs_scheduler) -> c_int;
}
extern "C" {
    pub fn ip_vs_unbind_scheduler(svc: *mut ip_vs_service);
}
extern "C" {
    pub fn ip_vs_scheduler_put(scheduler: *mut ip_vs_scheduler);
}
extern "C" {
    pub fn ip_vs_scheduler_err(svc: *mut ip_vs_service, msg: *const c_char);
}
// IPVS control data and functions (from ip_vs_ctl.c)
extern "C" {
    pub fn ip_vs_use_count_inc() -> c_int;
}
extern "C" {
    pub fn ip_vs_use_count_dec();
}
extern "C" {
    pub fn ip_vs_register_nl_ioctl() -> c_int;
}
extern "C" {
    pub fn ip_vs_unregister_nl_ioctl();
}
extern "C" {
    pub fn ip_vs_control_init() -> c_int;
}
extern "C" {
    pub fn ip_vs_control_cleanup();
}
extern "C" {
    pub fn ip_vs_try_bind_dest(cp: *mut ip_vs_conn);
}
extern "C" {
    pub fn ip_vs_dest_update_overload(dest: *mut ip_vs_dest, mode: c_int);
}
// IPVS sync daemon data and function prototypes
// (from ip_vs_sync.c)
//
extern "C" {
    pub fn stop_sync_thread(ipvs: *mut netns_ipvs, state: c_int) -> c_int;
}
extern "C" {
    pub fn ip_vs_sync_conn(ipvs: *mut netns_ipvs, cp: *mut ip_vs_conn, pkts: c_int);
}
// IPVS rate estimator prototypes (from ip_vs_est.c)
extern "C" {
    pub fn ip_vs_start_estimator(ipvs: *mut netns_ipvs, stats: *mut ip_vs_stats) -> c_int;
}
extern "C" {
    pub fn ip_vs_stop_estimator(ipvs: *mut netns_ipvs, stats: *mut ip_vs_stats);
}
extern "C" {
    pub fn ip_vs_zero_estimator(stats: *mut ip_vs_stats);
}
extern "C" {
    pub fn ip_vs_read_estimator(dst: *mut ip_vs_kstats, stats: *mut ip_vs_stats);
}
extern "C" {
    pub fn ip_vs_est_reload_start(ipvs: *mut netns_ipvs, restart: bool);
}
extern "C" {
    pub fn ip_vs_est_kthread_stop(kd: *mut ip_vs_est_kt_data);
}

// Stop tasks while cpulist is empty or if disabled with flag

extern "C" {
    pub fn max(_arg: 1U, _arg: limit) -> return;
}
// Various IPVS packet transmitters (from ip_vs_xmit.c)
extern "C" {
    pub fn ip_vs_dest_dst_rcu_free(head: *mut rcu_head);
}

// This is a simple mechanism to ignore packets when
// we are loaded. Just set ip_vs_drop_rate to 'n' and
// we start to drop 1/rate of the packets
//

// Enqueue delayed work for expiring no dest connections
// Only run when sysctl_expire_nodest=1
//
extern "C" {
    pub fn ip_vs_expire_nodest_conn_flush(ipvs: *mut netns_ipvs);
}

// ip_vs_fwd_tag returns the forwarding tag of the connection

// Check if connection uses double hashing

extern "C" {
    pub fn csum_partial(_arg: diff, _arg: sizeof(diff), _arg: oldsum) -> return;
}

extern "C" {
    pub fn csum_partial(_arg: diff, _arg: sizeof(diff), _arg: oldsum) -> return;
}

extern "C" {
    pub fn csum_partial(_arg: diff, _arg: sizeof(diff), _arg: oldsum) -> return;
}
// Checksum unnecessary or already validated?
// Locally generated ?
// Validate csum even for FORWARD
// Forget current conntrack (unconfirmed) and attach notrack entry

// Netfilter connection tracking
// (from ip_vs_nfct.c)
//

extern "C" {
    pub fn ip_vs_confirm_conntrack(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ip_vs_conn_drop_conntrack(cp: *mut ip_vs_conn);
}

// Using old conntrack that can not be redirected to another real server?

extern "C" {
    pub fn ip_vs_register_hooks(ipvs: *mut netns_ipvs, af: c_uint) -> c_int;
}
extern "C" {
    pub fn ip_vs_unregister_hooks(ipvs: *mut netns_ipvs, af: c_uint);
}
// We think the overhead of processing active connections is 257
// times higher than that of inactive connections in average. (This
// 257 times might not be accurate, we will change it later) We
// use the following formula to estimate the overhead now:
// dest->activeconns*256 + dest->totalconns
//

