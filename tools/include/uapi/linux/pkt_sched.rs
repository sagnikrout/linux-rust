//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/pkt_sched.h
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

// Logical priority bands not depending on specific packet scheduler.
//
pub const TC_PRIO_BESTEFFORT: c_int = 0;
pub const TC_PRIO_FILLER: c_int = 1;
pub const TC_PRIO_BULK: c_int = 2;
pub const TC_PRIO_INTERACTIVE_BULK: c_int = 4;
pub const TC_PRIO_INTERACTIVE: c_int = 6;
pub const TC_PRIO_CONTROL: c_int = 7;
pub const TC_PRIO_MAX: c_int = 15;
// Generic queue statistics, available for all the elements.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_stats {
    pub /: *mut *mut __u64 bytes; / Number of enqueued bytes,
    pub /: *mut *mut __u32 packets; / Number of enqueued packets,
    pub /: *mut *mut __u32 drops; / Packets dropped because of lack of resources,
    pub this: *mut *mut __u32 overlimits; / Number of throttle events when,
// flow goes out of allocated bandwidth
    pub /: *mut *mut __u32 bps; / Current flow byte rate,
    pub /: *mut *mut __u32 pps; / Current flow packet rate,
    pub qlen: __u32,
    pub backlog: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_estimator {
    pub interval: signed char,
    pub ewma_log: c_uchar,
}

// "Handles"
//

pub const TC_H_MIN_PRIORITY: c_uint = 0xFFE0U;
pub const TC_H_MIN_INGRESS: c_uint = 0xFFF2U;
pub const TC_H_MIN_EGRESS: c_uint = 0xFFF3U;
// Need to corrospond to iproute2 tc/tc_core.h "enum link_layer"
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_link_layer {
    TC_LINKLAYER_UNAWARE, /* Indicate unaware old iproute2 util */
    TC_LINKLAYER_ETHERNET,
    TC_LINKLAYER_ATM,
}

pub const TC_LINKLAYER_MASK: c_uint = 0x0F /* limit use to lower 4 bits */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_ratespec {
    pub cell_log: c_uchar,
    pub /: *mut *mut __u8 linklayer; / lower 4 bits,
    pub overhead: c_ushort,
    pub cell_align: c_short,
    pub mpu: c_ushort,
    pub rate: __u32,
}

pub const TC_RTAB_SIZE: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_sizespec {
    pub cell_log: c_uchar,
    pub size_log: c_uchar,
    pub cell_align: c_short,
    pub overhead: c_int,
    pub linklayer: c_uint,
    pub mpu: c_uint,
    pub mtu: c_uint,
    pub tsize: c_uint,
}

// FIFO section
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_fifo_qopt {
    pub /: *mut *mut __u32 limit; / Queue length: bytes for bfifo, packets for pfifo,
}

// SKBPRIO section
//
// Priorities go from zero to (SKBPRIO_MAX_PRIORITY - 1).
// SKBPRIO_MAX_PRIORITY should be at least 64 in order for skbprio to be able
// to map one to one the DS field of IPV4 and IPV6 headers.
// Memory allocation grows linearly with SKBPRIO_MAX_PRIORITY.
//
pub const SKBPRIO_MAX_PRIORITY: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_skbprio_qopt {
    pub /: *mut *mut __u32 limit; / Queue length in packets.,
}

// PRIO section
pub const TCQ_PRIO_BANDS: c_int = 16;
pub const TCQ_MIN_PRIO_BANDS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_prio_qopt {
    pub /: *mut *mut int bands; / Number of bands,
    pub /: *mut *mut __u8 priomap[TC_PRIO_MAX+1]; / Map: logical priority -> PRIO band,
}

// MULTIQ section
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_multiq_qopt {
    pub /: *mut *mut __u16 bands; / Number of bands,
    pub /: *mut *mut __u16 max_bands; / Maximum number of queues,
}

// PLUG section
pub const TCQ_PLUG_BUFFER: c_int = 0;
pub const TCQ_PLUG_RELEASE_ONE: c_int = 1;
pub const TCQ_PLUG_RELEASE_INDEFINITE: c_int = 2;
pub const TCQ_PLUG_LIMIT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_plug_qopt {
// TCQ_PLUG_BUFFER: Inset a plug into the queue and
// buffer any incoming packets
// TCQ_PLUG_RELEASE_ONE: Dequeue packets from queue head
// to beginning of the next plug.
// TCQ_PLUG_RELEASE_INDEFINITE: Dequeue all packets from queue.
// Stop buffering packets until the next TCQ_PLUG_BUFFER
// command is received (just act as a pass-thru queue).
// TCQ_PLUG_LIMIT: Increase/decrease queue size
//
    pub action: c_int,
    pub limit: __u32,
}

// TBF section
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_tbf_qopt {
    pub rate: tc_ratespec,
    pub peakrate: tc_ratespec,
    pub limit: __u32,
    pub buffer: __u32,
    pub mtu: __u32,
}

// TEQL section
// TEQL does not require any parameters
// SFQ section
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_sfq_qopt {
    pub /: *mut *mut unsigned quantum; / Bytes per round allocated to flow,
    pub /: *mut *mut int perturb_period; / Period of hash perturbation,
    pub /: *mut *mut __u32 limit; / Maximal packets in queue,
    pub /: *mut *mut unsigned divisor; / Hash divisor,
    pub /: *mut *mut unsigned flows; / Maximal number of flows,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_sfqred_stats {
    pub /: *mut *mut __u32 prob_drop; / Early drops, below max threshold,
    pub /: *mut *mut __u32 forced_drop; / Early drops, after max threshold,
    pub /: *mut *mut __u32 prob_mark; / Marked packets, below max threshold,
    pub /: *mut *mut __u32 forced_mark; / Marked packets, after max threshold,
    pub /: *mut *mut __u32 prob_mark_head; / Marked packets, below max threshold,
    pub /: *mut *mut __u32 forced_mark_head;/ Marked packets, after max threshold,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_sfq_qopt_v1 {
    pub v0: tc_sfq_qopt,
    pub /: *mut *mut unsigned int depth; / max number of packets per flow,
    pub headdrop: c_uint,
// SFQRED parameters
    pub /: *mut *mut __u32 limit; / HARD maximal flow queue length (bytes),
    pub /: *mut *mut __u32 qth_min; / Min average length threshold (bytes),
    pub /: *mut *mut __u32 qth_max; / Max average length threshold (bytes),
    pub /: *mut *mut unsigned char Wlog; / log(W),
    pub /: *mut *mut unsigned char Plog; / log(P_max/(qth_max-qth_min)),
    pub /: *mut *mut unsigned char Scell_log; / cell size for idle damping,
    pub flags: c_uchar,
    pub /: *mut *mut __u32 max_P; / probability, high resolution,
// SFQRED stats
    pub stats: tc_sfqred_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_sfq_xstats {
    pub allot: __s32,
}

// RED section

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_red_qopt {
    pub /: *mut *mut __u32 limit; / HARD maximal queue length (bytes),
    pub /: *mut *mut __u32 qth_min; / Min average length threshold (bytes),
    pub /: *mut *mut __u32 qth_max; / Max average length threshold (bytes),
    pub /: *mut *mut unsigned char Wlog; / log(W),
    pub /: *mut *mut unsigned char Plog; / log(P_max/(qth_max-qth_min)),
    pub /: *mut *mut unsigned char Scell_log; / cell size for idle damping,
    pub flags: c_uchar,
pub const TC_RED_ECN: c_int = 1;
pub const TC_RED_HARDDROP: c_int = 2;
pub const TC_RED_ADAPTATIVE: c_int = 4;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_red_xstats {
    pub /: *mut *mut __u32 early; / Early drops,
    pub /: *mut *mut __u32 pdrop; / Drops due to queue limits,
    pub /: *mut *mut __u32 other; / Drops due to drop() calls,
    pub /: *mut *mut __u32 marked; / Marked packets,
}

// GRED section
pub const MAX_DPs: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_gred_qopt {
    pub /: *mut *mut __u32 limit; / HARD maximal queue length (bytes),
    pub /: *mut *mut __u32 qth_min; / Min average length threshold (bytes),
    pub /: *mut *mut __u32 qth_max; / Max average length threshold (bytes),
    pub /: *mut *mut __u32 DP; / up to 2^32 DPs,
    pub backlog: __u32,
    pub qave: __u32,
    pub forced: __u32,
    pub early: __u32,
    pub other: __u32,
    pub pdrop: __u32,
    pub /: *mut *mut __u8 Wlog; / log(W),
    pub /: *mut *mut __u8 Plog; / log(P_max/(qth_max-qth_min)),
    pub /: *mut *mut __u8 Scell_log; / cell size for idle damping,
    pub /: *mut *mut __u8 prio; / prio of this VQ,
    pub packets: __u32,
    pub bytesin: __u32,
}

// gred setup
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_gred_sopt {
    pub DPs: __u32,
    pub def_DP: __u32,
    pub grio: __u8,
    pub flags: __u8,
    pub pad1: __u16,
}

// CHOKe section

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_choke_qopt {
    pub /: *mut *mut __u32 limit; / Hard queue length (packets),
    pub /: *mut *mut __u32 qth_min; / Min average threshold (packets),
    pub /: *mut *mut __u32 qth_max; / Max average threshold (packets),
    pub /: *mut *mut unsigned char Wlog; / log(W),
    pub /: *mut *mut unsigned char Plog; / log(P_max/(qth_max-qth_min)),
    pub /: *mut *mut unsigned char Scell_log; / cell size for idle damping,
    pub /: *mut *mut unsigned char flags; / see RED flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_choke_xstats {
    pub /: *mut *mut __u32 early; / Early drops,
    pub /: *mut *mut __u32 pdrop; / Drops due to queue limits,
    pub /: *mut *mut __u32 other; / Drops due to drop() calls,
    pub /: *mut *mut __u32 marked; / Marked packets,
    pub /: *mut *mut __u32 matched; / Drops due to flow match,
}

// HTB section
pub const TC_HTB_NUMPRIO: c_int = 8;
pub const TC_HTB_MAXDEPTH: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_htb_opt {
    pub rate: tc_ratespec,
    pub ceil: tc_ratespec,
    pub buffer: __u32,
    pub cbuffer: __u32,
    pub quantum: __u32,
    pub /: *mut *mut __u32 level; / out only,
    pub prio: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_htb_glob {
    pub /: *mut *mut __u32 version; / to match HTB/TC,
    pub /: *mut *mut __u32 rate2quantum; / bps->quantum divisor,
    pub /: *mut *mut __u32 defcls; / default class number,
    pub /: *mut *mut __u32 debug; / debug flags,
// stats
    pub /: *mut *mut __u32 direct_pkts; / count of non shaped packets,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_htb_xstats {
    pub lends: __u32,
    pub borrows: __u32,
    pub /: *mut *mut __u32 giants; / unused since 'Make HTB scheduler work with TSO.',
    pub tokens: __s32,
    pub ctokens: __s32,
}

// HFSC section
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_hfsc_qopt {
    pub /: *mut *mut __u16 defcls; / default class,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_service_curve {
    pub /: *mut *mut __u32 m1; / slope of the first segment in bps,
    pub /: *mut *mut __u32 d; / x-projection of the first segment in us,
    pub /: *mut *mut __u32 m2; / slope of the second segment in bps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_hfsc_stats {
    pub /: *mut *mut __u64 work; / total work done,
    pub /: *mut *mut __u64 rtwork; / work done by real-time criteria,
    pub /: *mut *mut __u32 period; / current period,
    pub /: *mut *mut __u32 level; / class level in hierarchy,
}

// Network emulator

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_netem_qopt {
    pub /: *mut *mut __u32 latency; / added delay (us),
    pub /: *mut *mut __u32 limit; / fifo limit (packets),
    pub /: *mut *mut __u32 loss; / random packet loss (0=none ~0=100%),
    pub /: *mut *mut __u32 gap; / re-ordering gap (0 for none),
    pub /: *mut *mut __u32 duplicate; / random packet dup (0=none ~0=100%),
    pub /: *mut *mut __u32 jitter; / random jitter in latency (us),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_netem_corr {
    pub /: *mut *mut __u32 delay_corr; / delay correlation,
    pub /: *mut *mut __u32 loss_corr; / packet loss correlation,
    pub /: *mut *mut __u32 dup_corr; / duplicate correlation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_netem_reorder {
    pub probability: __u32,
    pub correlation: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_netem_corrupt {
    pub probability: __u32,
    pub correlation: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_netem_rate {
    pub /: *mut *mut __u32 rate; / byte/s,
    pub packet_overhead: __s32,
    pub cell_size: __u32,
    pub cell_overhead: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_netem_slot {
    pub /: *mut *mut __s64 min_delay; / nsec,
    pub max_delay: __s64,
    pub max_packets: __s32,
    pub max_bytes: __s32,
    pub /: *mut *mut __s64 dist_delay; / nsec,
    pub /: *mut *mut __s64 dist_jitter; / nsec,
}

// State transition probabilities for 4 state model
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_netem_gimodel {
    pub p13: __u32,
    pub p31: __u32,
    pub p32: __u32,
    pub p14: __u32,
    pub p23: __u32,
}

// Gilbert-Elliot models
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_netem_gemodel {
    pub p: __u32,
    pub r: __u32,
    pub h: __u32,
    pub k1: __u32,
}

pub const NETEM_DIST_SCALE: c_int = 8192;
pub const NETEM_DIST_MAX: c_int = 16384;
// DRR

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_drr_stats {
    pub deficit: __u32,
}

// MQPRIO
pub const TC_QOPT_BITMASK: c_int = 15;
pub const TC_QOPT_MAX_QUEUE: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_mqprio_qopt {
    pub num_tc: __u8,
    pub 1]: __u8 prio_tc_map[TC_QOPT_BITMASK +,
    pub hw: __u8,
    pub count: [__u16; TC_QOPT_MAX_QUEUE],
    pub offset: [__u16; TC_QOPT_MAX_QUEUE],
}

pub const TC_MQPRIO_F_MODE: c_uint = 0x1;
pub const TC_MQPRIO_F_SHAPER: c_uint = 0x2;
pub const TC_MQPRIO_F_MIN_RATE: c_uint = 0x4;
pub const TC_MQPRIO_F_MAX_RATE: c_uint = 0x8;

// SFB

//
// Note: increment, decrement are Q0.16 fixed-point values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_sfb_qopt {
    pub /: *mut *mut __u32 rehash_interval; / delay between hash move, in ms,
    pub /: *mut *mut __u32 warmup_time; / double buffering warmup time in ms (warmup_time < rehash_interval),
    pub /: *mut *mut __u32 max; / max len of qlen_min,
    pub /: *mut *mut __u32 bin_size; / maximum queue length per bin,
    pub /: *mut *mut __u32 increment; / probability increment, (d1 in Blue),
    pub /: *mut *mut __u32 decrement; / probability decrement, (d2 in Blue),
    pub /: *mut *mut __u32 limit; / max SFB queue length,
    pub /: *mut *mut __u32 penalty_rate; / inelastic flows are rate limited to 'rate' pps,
    pub penalty_burst: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_sfb_xstats {
    pub earlydrop: __u32,
    pub penaltydrop: __u32,
    pub bucketdrop: __u32,
    pub queuedrop: __u32,
    pub /: *mut *mut __u32 childdrop; / drops in child qdisc,
    pub marked: __u32,
    pub maxqlen: __u32,
    pub maxprob: __u32,
    pub avgprob: __u32,
}

pub const SFB_MAX_PROB: c_uint = 0xFFFF;
// QFQ

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_qfq_stats {
    pub weight: __u32,
    pub lmax: __u32,
}

// CODEL

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_codel_xstats {
    pub /: *mut *mut __u32 maxpacket; / largest packet we've seen so far,
    pub we: *mut *mut __u32 count; / how many drops we've done since the last time,
// entered dropping state
//
    pub /: *mut *mut __u32 lastcount; / count at entry to dropping state,
    pub /: *mut *mut __u32 ldelay; / in-queue delay seen by most recently dequeued packet,
    pub /: *mut *mut __s32 drop_next; / time to drop next packet,
    pub /: *mut *mut __u32 drop_overlimit; / number of time max qdisc packet limit was hit,
    pub /: *mut *mut __u32 ecn_mark; / number of packets we ECN marked instead of dropped,
    pub /: *mut *mut __u32 dropping; / are we in dropping state ?,
    pub /: *mut *mut __u32 ce_mark; / number of CE marked packets because of ce_threshold,
}

// FQ_CODEL

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_fq_codel_qd_stats {
    pub /: *mut *mut __u32 maxpacket; / largest packet we've seen so far,
    pub qdisc: *mut *mut __u32 drop_overlimit; / number of time max,
// packet limit was hit
//
    pub marked: *mut *mut __u32 ecn_mark; / number of packets we ECN,
// instead of being dropped
//
    pub packets: *mut *mut __u32 new_flow_count; / number of time,
// created a 'new flow'
//
    pub /: *mut *mut __u32 new_flows_len; / count of flows in new list,
    pub /: *mut *mut __u32 old_flows_len; / count of flows in old list,
    pub /: *mut *mut __u32 ce_mark; / packets above ce_threshold,
    pub /: *mut *mut __u32 memory_usage; / in bytes,
    pub drop_overmemory: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_fq_codel_cl_stats {
    pub deficit: __s32,
    pub recently: *mut *mut __u32 ldelay; / in-queue delay seen by most,
// dequeued packet
//
    pub count: __u32,
    pub lastcount: __u32,
    pub dropping: __u32,
    pub drop_next: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_fq_codel_xstats {
    pub type: __u32,
    pub qdisc_stats: tc_fq_codel_qd_stats,
    pub class_stats: tc_fq_codel_cl_stats,
}

// FQ

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_fq_qd_stats {
    pub gc_flows: __u64,
    pub highprio_packets: __u64,
    pub tcp_retrans: __u64,
    pub throttled: __u64,
    pub flows_plimit: __u64,
    pub pkts_too_long: __u64,
    pub allocation_errors: __u64,
    pub time_next_delayed_flow: __s64,
    pub flows: __u32,
    pub inactive_flows: __u32,
    pub throttled_flows: __u32,
    pub unthrottle_latency_ns: __u32,
    pub /: *mut *mut __u64 ce_mark; / packets above ce_threshold,
}

// Heavy-Hitter Filter

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_hhf_xstats {
    pub limit: *mut *mut __u32 drop_overlimit; / number of times max qdisc packet,
// was hit
//
    pub /: *mut *mut __u32 hh_overlimit; / number of times max heavy-hitters was hit,
    pub /: *mut *mut __u32 hh_tot_count; / number of captured heavy-hitters so far,
    pub /: *mut *mut __u32 hh_cur_count; / number of current heavy-hitters,
}

// PIE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_pie_xstats {
    pub /: *mut *mut __u32 prob; / current probability,
    pub /: *mut *mut __u32 delay; / current delay in ms,
    pub /: *mut *mut __u32 avg_dq_rate; / current average dq_rate in bits/pie_time,
    pub /: *mut *mut __u32 packets_in; / total number of packets enqueued,
    pub /: *mut *mut __u32 dropped; / packets dropped due to pie_action,
    pub /: *mut *mut __u32 overlimit; / dropped due to lack of space in queue,
    pub /: *mut *mut __u32 maxq; / maximum queue size,
    pub ecn*/: *mut *mut __u32 ecn_mark; / packets marked with,
}

// CBS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_cbs_qopt {
    pub offload: __u8,
    pub _pad: [__u8; 3],
    pub hicredit: __s32,
    pub locredit: __s32,
    pub idleslope: __s32,
    pub sendslope: __s32,
}

// ETF
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_etf_qopt {
    pub delta: __s32,
    pub clockid: __s32,
    pub flags: __u32,

}

// CAKE

// TAPRIO

// The format for schedule entry list is:
// [TCA_TAPRIO_SCHED_ENTRY_LIST]
// [TCA_TAPRIO_SCHED_ENTRY]
// [TCA_TAPRIO_SCHED_ENTRY_CMD]
// [TCA_TAPRIO_SCHED_ENTRY_GATES]
// [TCA_TAPRIO_SCHED_ENTRY_INTERVAL]
//

