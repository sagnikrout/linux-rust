//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tcp.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions for the TCP protocol.
//
// Version:	@(#)tcp.h	1.0.2	04/28/93
//
// Author:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//

extern "C" {
    pub fn __tcp_hdrlen(_arg: tcp_hdr(skb)) -> return;
}
//
// skb_tcp_all_headers - Returns size of all headers for a TCP packet
// @skb: buffer
//
// Used in TX path, for a packet known to be a TCP one.
//
// if (skb_is_gso(skb)) {
// int hlen = skb_tcp_all_headers(skb);
// ...
//
extern "C" {
    pub fn skb_transport_offset(tcp_hdrlen(skb: skb) +) -> return;
}
//
// skb_inner_tcp_all_headers - Returns size of all headers for an encap TCP packet
// @skb: buffer
//
// Used in TX path, for a packet known to be a TCP one.
//
// if (skb_is_gso(skb) && skb->encapsulation) {
// int hlen = skb_inner_tcp_all_headers(skb);
// ...
//
extern "C" {
    pub fn skb_inner_transport_offset(inner_tcp_hdrlen(skb: skb) +) -> return;
}
// TCP Fast Open

// TCP Fast Open Cookie as stored in memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_fastopen_cookie {
    pub sizeof(u64))]: __le64 val[DIV_ROUND_UP(TCP_FASTOPEN_COOKIE_MAX,,
    pub len: i8,
    pub /: *mut *mut bool exp; / In RFC6994 experimental option format,
}

// This defines a selective acknowledgement block.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_sack_block_wire {
    pub start_seq: __be32,
    pub end_seq: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_sack_block {
    pub start_seq: u32,
    pub end_seq: u32,
}

// These are used to set the sack_ok field in struct tcp_options_received

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_options_received {
// PAWS/RTTM data
    pub /: *mut *mut int ts_recent_stamp;/ Time we stored ts_recent (for aging),
    pub /: *mut *mut u32 ts_recent; / Time stamp to echo next,
    pub /: *mut *mut u32 rcv_tsval; / Time stamp value,
    pub /: *mut *mut u32 rcv_tsecr; / Time stamp echo reply,
    pub /: *mut *mut rcv_wscale : 4; / Window scaling to send to receiver,
    pub /: *mut *mut u8 num_sacks; / Number of SACK blocks,
    pub /: *mut *mut u16 user_mss; / mss requested by user in ioctl,
    pub /: *mut *mut u16 mss_clamp; / Maximal mss, negotiated at connection setup,
}

// This is the max number of SACKS that we'll generate and process. It's safe
// to increase this, although since:
// size = TCPOLEN_SACK_BASE_ALIGNED (4) + n * TCPOLEN_SACK_PERBLOCK (8)
// only four options will fit in a standard TCP header
pub const TCP_NUM_SACKS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_request_sock {
    pub req: inet_request_sock,
    pub af_specific: *const tcp_request_sock_ops,
    pub /: *mut *mut u64 snt_synack; / first SYNACK sent time,
    pub tfo_listener: bool,
    pub is_mptcp: bool,
    pub req_usec_ts: bool,

    pub drop_req: bool,

    pub txhash: u32,
    pub rcv_isn: u32,
    pub snt_isn: u32,
    pub ts_off: u32,
    pub snt_tsval_first: u32,
    pub snt_tsval_last: u32,
    pub /: *mut *mut u32 last_oow_ack_time; / last SYNACK,
    pub For: *mut *mut u32 rcv_nxt; / the ack # by SYNACK.,
// FastOpen it's the seq#
// after data-in-SYN.
//
    pub syn_tos: u8,
    pub accecn_ok: bool,
    pub :2: u8 saw_accecn_opt,

    pub ao_keyid: u8,
    pub ao_rcv_next: u8,
    pub used_tcp_ao: bool,

}

pub const TCP_RMEM_TO_WIN_SCALE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_sock {
// Cacheline organization can be found documented in
// Documentation/networking/net_cachelines/tcp_sock.rst.
// Please update the document when adding new fields.
//
// inet_connection_sock has to be the first member of tcp_sock
    pub inet_conn: inet_connection_sock,
// TX read-mostly hotpath cache lines
    pub /: *mut *mut u32 max_window; / Maximal window ever seen from peer,
    pub /: *mut *mut u32 rcv_ssthresh; / Current window clamp,
    pub /: *mut *mut u32 reordering; / Packet reordering metric.,
    pub /: *mut *mut u32 notsent_lowat; / TCP_NOTSENT_LOWAT,
    pub /: *mut *mut u16 gso_segs; / Max number of segs per GSO packet,
// from STCP, retrans queue hinting
    pub retransmit_skb_hint: *mut sk_buff,

    pub acked_seq): *mut *mut *mut void (tcp_clean_acked)(struct sock sk, u32,

// TXRX read-mostly hotpath cache lines
    pub /: *mut *mut u32 tsoffset; / timestamp offset,
    pub /: *mut *mut u32 snd_wnd; / The window we expect to receive,
    pub /: *mut *mut u32 mss_cache; / Cached effective mss, not including SACKS,
    pub /: *mut *mut u32 snd_cwnd; / Sending congestion window,
    pub /: *mut *mut u32 prr_out; / Total number of pkts sent during Recovery.,
    pub /: *mut *mut u32 lost_out; / Lost packets,
    pub /: *mut *mut u32 sacked_out; / SACK'd packets,
    pub /: *mut *mut u16 tcp_header_len; / Bytes of tcp header to send,
    pub /: *mut *mut u8 scaling_ratio; / see tcp_win_from_space(),
    pub /: *mut *mut recvmsg_inq : 1;/ Indicate # of bytes in queue upon recvmsg,
// RX read-mostly hotpath cache lines
    pub /: *mut *mut u32 copied_seq; / Head of yet unread data,
    pub /: *mut *mut u32 snd_wl1; / Sequence for window update,
    pub /: *mut *mut u32 tlp_high_seq; / snd_nxt at the time of TLP,
    pub /: *mut *mut u32 rttvar_us; / smoothed mdev_max,
    pub /: *mut *mut u32 retrans_out; / Retransmitted packets out,
    pub /: *mut *mut u16 advmss; / Advertised MSS,
    pub /: *mut *mut u16 urg_data; / Saved octet of OOB data and control flags,
    pub /: *mut *mut u32 lost; / Total data packets lost incl. rexmits,
    pub /: *mut *mut u32 snd_ssthresh; / Slow start size threshold,
    pub rtt_min: minmax,
// OOO segments go in this rbtree. Socket lock must be held.
    pub out_of_order_queue: rb_root,
// TX read-write hotpath cache lines
    pub ____cacheline_aligned: __cacheline_group_begin(tcp_sock_write_tx),
    pub /: *mut *mut u32 delivered; / Total data packets delivered incl. rexmits,
    pub /: *mut *mut u32 delivered_ce; / Like the above but only ECE marked packets,
    pub tcpEStatsAppHCThruOctetsAcked: *mut *mut u64 bytes_acked; / RFC4898,
// sum(delta(snd_una)), or how many bytes
// were acked.
//
    pub tcpEStatsPerfHCDataOctetsOut: *mut *mut u64 bytes_sent; / RFC4898,
// total number of data bytes sent.
//
    pub /: *mut *mut u64 first_tx_mstamp; / start of window send phase,
    pub /: *mut *mut u64 delivered_mstamp; / time we reached "delivered",
    pub tcpEStatsPerfDataSegsOut: *mut *mut u32 data_segs_out; / RFC4898,
// total number of data segments sent.
//
    pub /: *mut *mut u32 snd_sml; / Last byte of the most recently transmitted small packet,
    pub /: *mut *mut u8 chrono_type; / current chronograph type,
    pub /: *mut *mut u32 chrono_start; / Start time in jiffies of a TCP chrono,
    pub /: *mut *mut u32 chrono_stat[3]; / Time in jiffies for chrono_stat stats,
    pub /: *mut *mut u32 write_seq; / Tail(+1) of data held in tcp send buffer,
    pub /: *mut *mut u32 pushed_seq; / Last pushed seq, required to talk to windows,
    pub /: *mut *mut u32 lsndtime; / timestamp of last sent data packet (for restart window),
    pub /: *mut *mut u32 mdev_us; / mean deviation of RTT, scaled by 4 (<< 2) in usecs,
    pub /: *mut *mut u32 rtt_seq; / sequence number to update rttvar,
    pub /: *mut *mut u32 max_packets_out; / max packets_out in last window,
    pub /: *mut *mut u32 cwnd_usage_seq; / right edge of cwnd usage tracking flight,
    pub /: *mut *mut u32 rate_delivered; / saved rate sample: packets delivered,
    pub /: *mut *mut u32 rate_interval_us; / saved rate sample: time elapsed,
    pub /: *mut *mut u64 tcp_wstamp_ns; / departure time for next sent data packet,
    pub /: *mut *mut u64 accecn_opt_tstamp; / Last AccECN option sent timestamp,
    pub /: *mut *mut list_head tsorted_sent_queue; / time-sorted sent but un-SACKed skbs,
    pub highest: *mut *mut *mut sk_buff highest_sack; / skb just after the,
// skb with SACKed bit set
// (validity guaranteed only if
// sacked_out > 0)
//
    pub /: *mut *mut u8 ecn_flags; / ECN status bits.,
// TXRX read-write hotpath cache lines
//
// Header prediction flags
// 0x5?10 << 16 + snd_wnd in net byte order
//
    pub /: *mut *mut rate_app_limited:1; / rate_{delivered,interval_us} limited?,
    pub /: *mut *mut prev_ecnfield:2; / ECN bits from the previous segment,
    pub pred_flags: __be32,
    pub /: *mut *mut u64 tcp_clock_cache; / cache last tcp_clock_ns() (see tcp_mstamp_refresh()),
    pub /: *mut *mut u64 tcp_mstamp; / most recent packet received/sent,
    pub /: *mut *mut u32 rcv_nxt; / What we want to receive next,
    pub /: *mut *mut u32 snd_nxt; / Next sequence we send,
    pub /: *mut *mut u32 snd_una; / First byte we want an ack for,
    pub /: *mut *mut u32 window_clamp; / Maximal window to advertise,
    pub /: *mut *mut u32 srtt_us; / smoothed round trip time << 3 in usecs,
    pub /: *mut *mut u32 packets_out; / Packets which are "in flight",
    pub /: *mut *mut u32 snd_up; / Urgent pointer,
    pub /: *mut *mut u32 received_ce; / Like the above but for rcvd CE marked pkts,
    pub ECN: *mut *mut u32 received_ecn_bytes[3]; / received byte counters for three,
// types: INET_ECN_ECT_1, INET_ECN_ECT_0,
// and INET_ECN_CE
//
    pub /: *mut *mut u32 app_limited; / limited until "delivered" reaches this val,
    pub /: *mut *mut u32 rcv_wnd; / Current receiver window,
    pub 7323,: *mut *mut u32 rcv_mwnd_seq; / Maximum window sequence number (RFC,
// section 2.4, receiver requirements)
//
    pub /: *mut *mut u32 rcv_tstamp; / timestamp of last received ACK (for keepalives),
//
// Options received (usually on last packet, some only on SYN packets).
//
    pub rx_opt: tcp_options_received,
    pub tcpEStatsPerfSegsIn: *mut *mut u32 segs_in; / RFC4898,
// total number of segments in.
//
    pub tcpEStatsPerfSegsOut: *mut *mut u32 segs_out; / RFC4898,
// The total number of segments sent.
//
// RX read-write hotpath cache lines
    pub __aligned(8): __cacheline_group_begin(tcp_sock_write_rx),
    pub bytes_received: u64,
// RFC4898 tcpEStatsAppHCThruOctetsReceived
// sum(delta(rcv_nxt)), or how many bytes
// were acked.
//
    pub tcpEStatsPerfDataSegsIn: *mut *mut u32 data_segs_in; / RFC4898,
// total number of data segments in.
//
    pub /: *mut *mut u32 rcv_wup; / rcv_nxt on last window update sent,
    pub rcv_rtt_last_tsecr: u32,
    pub delivered_ecn_bytes: [u32; 3],
    pub /: *mut *mut u16 pkts_acked_ewma;/ Pkts acked EWMA for AccECN cep heuristic,
    pub rtt_us: u32,
    pub seq: u32,
    pub time: u64,
    pub rcv_rtt_est: },
// Receiver queue space
    pub space: c_int,
    pub seq: u32,
    pub time: u64,
    pub rcvq_space: },
// End of Hot Path
//
// RFC793 variables by their proper names. This means you can
// read the code and the spec side by side (and laugh ...)
// See RFC793 and RFC1122. The RFC writes these in capitals.
//
    pub tcpEStatsStackDSACKDups: *mut *mut u32 dsack_dups; / RFC4898,
// total number of DSACK blocks received
//
    pub compressed_ack_rcv_nxt: u32,
    pub /: *mut *mut list_head tsq_node; / anchor in tsq_tasklet.head list,
// Information of the most recently (s)acked skb
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_rack {
    pub /: *mut *mut u64 mstamp; / (Re)sent time of the skb,
    pub /: *mut *mut u32 rtt_us; / Associated RTT,
    pub /: *mut *mut u32 end_seq; / Ending TCP sequence of the skb,
    pub /: *mut *mut u32 last_delivered; / tp->delivered at last reo_wnd adj,
    pub /: *mut *mut u8 reo_wnd_steps; / Allowed reordering window,
pub const TCP_RACK_RECOVERY_THRESH: c_int = 16;
    pub /: *mut *mut advanced:1; / mstamp advanced since last lost marking,
    pub rack: },
    pub compressed_ack: u8,
    pub /: *mut *mut syn_ect_rcv:2; / ... needed during 3WHS + first seqno,
    pub /: *mut *mut frto : 1;/ F-RTO (RFC5682) activated in CA_Loss,
    pub repair_queue: u8,
    pub /: *mut *mut syn_fastopen_child:1; / created TFO passive child socket,
    pub /: *mut *mut u8 keepalive_probes; / num of allowed keep alive probes,
    pub /: *mut *mut saw_accecn_opt:2; / An AccECN option was seen,
    pub /: *mut *mut u32 tcp_tx_delay; / delay (in usec) added to TX packets,
// RTT measurement
    pub /: *mut *mut u32 mdev_max_us; / maximal mdev for the last rtt period,
    pub /: *mut *mut u32 reord_seen; / number of data packet reordering events,
//
// Slow start and congestion control (see also Nagle, and Karn & Partridge)
//
    pub /: *mut *mut u32 snd_cwnd_cnt; / Linear increase counter,
    pub /: *mut *mut u32 snd_cwnd_clamp; / Do not allow snd_cwnd to grow above this,
    pub snd_cwnd_used: u32,
    pub snd_cwnd_stamp: u32,
    pub /: *mut *mut u32 prior_cwnd; / cwnd right before starting loss recovery,
    pub to: *mut *mut u32 prr_delivered; / Number of newly delivered packets,
// receiver in Recovery.
    pub /: *mut *mut u32 last_oow_ack_time; / timestamp of last out-of-window ACK,
    pub pacing_timer: hrtimer,
    pub compressed_ack_timer: hrtimer,
    pub /: *mut *mut *mut sk_buff ooo_last_skb; / cache rb_last(out_of_order_queue),
// SACKs data, these 2 need to be together (see tcp_options_write)
    pub /: *mut *mut tcp_sack_block duplicate_sack[1]; / D-SACK block,
    pub themselves*/: *mut *mut tcp_sack_block selective_acks[4]; / The SACKS,
    pub recv_sack_cache: [tcp_sack_block; 4],
    pub /: *mut *mut u32 prior_ssthresh; / ssthresh saved at recovery start,
    pub /: *mut *mut u32 high_seq; / snd_nxt at onset of congestion,
    pub retransmit,: *mut *mut u32 retrans_stamp; / Timestamp of the last,
// also used in SYN-SENT to remember stamp of
// the first SYN.
    pub /: *mut *mut u32 undo_marker; / snd_una upon a new recovery episode.,
    pub /: *mut *mut int undo_retrans; / number of undoable retransmissions.,
    pub ICMPV6_PKT_TOOBIG: *mut *mut u32 mtu_info; / We received an ICMP_FRAG_NEEDED /,
// while socket was owned by user.
//
    pub tcpEStatsPerfOctetsRetrans: *mut *mut u64 bytes_retrans; / RFC4898,
// Total data bytes retransmitted
//
    pub /: *mut *mut u32 total_retrans; / Total retransmits for entire connection,
    pub /: *mut *mut u32 rto_stamp; / Start time (ms) of last CA_Loss recovery,
    pub including: *mut *mut u16 total_rto; / Total number of RTO timeouts,,
// SYN/SYN-ACK and recurring timeouts.
//
    pub recoveries,: *mut *mut u16 total_rto_recoveries; / Total number of RTO,
// including any unfinished recovery.
//
    pub /: *mut *mut u32 total_rto_time; / ms spent in (completed) RTO recoveries.,
    pub /: *mut *mut u32 urg_seq; / Seq of received urgent pointer,
    pub /: *mut *mut unsigned int keepalive_time; / time before keep alive takes place,
    pub /: *mut *mut unsigned int keepalive_intvl; / time interval between keep alive probes,
    pub linger2: c_int,
// Sock_ops bpf program related variables

    pub programs: *mut *mut u8 bpf_sock_ops_cb_flags; / Control calling BPF,
// values defined in uapi/linux/tcp.h
//
    pub of: *mut *mut u8 bpf_chg_cc_inprogress:1; / In the middle,
// bpf_setsockopt(TCP_CONGESTION),
// it is to avoid the bpf_tcp_cc->init()
// to recur itself by calling
// bpf_setsockopt(TCP_CONGESTION, "itself").
//

    pub /: *mut *mut u16 timeout_rehash; / Timeout-triggered rehash attempts,
    pub /: *mut *mut u32 rcv_ooopack; / Received out-of-order packets, for tcpinfo,
// TCP-specific MTU probe information.
    pub probe_seq_start: u32,
    pub probe_seq_end: u32,
    pub mtu_probe: },
    pub /: *mut *mut u32 plb_rehash; / PLB-triggered rehash attempts,

    pub is_mptcp: bool,

    pub /: *mut *mut bool syn_smc; / SYN includes SMC,
    pub sk): *const *const bool (smc_hs_congested)(struct sock,

// TCP AF-Specific parts; only used by TCP-AO/MD5 Signature support so far
    pub af_specific: *const tcp_sock_af_ops,

// TCP MD5 Signature Option information
    pub md5sig_info: *mut tcp_md5sig_info __rcu,

    pub ao_info: *mut tcp_ao_info __rcu,

// TCP fastopen related information
    pub fastopen_req: *mut tcp_fastopen_request,
// fastopen_rsk points to request_sock that resulted in this big
// socket. Used to retransmit SYNACKs etc.
//
    pub fastopen_rsk: *mut request_sock __rcu,
    pub saved_syn: *mut saved_syn,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tsq_enum {
    TSQ_THROTTLED,
    TSQ_QUEUED,
    TCP_TSQ_DEFERRED,	   /* tcp_tasklet_func() found socket was owned */
    TCP_WRITE_TIMER_DEFERRED,  /* tcp_write_timer() found socket was owned */
    TCP_DELACK_TIMER_DEFERRED, /* tcp_delack_timer() found socket was owned */
    TCP_MTU_REDUCED_DEFERRED,  /* tcp_v{4|6}_err() could not call
// tcp_v{4|6}_mtu_reduced()
//
    TCP_ACK_DEFERRED,	   /* TX pure ack is deferred */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tsq_flags {
    TSQF_THROTTLED			= BIT(TSQ_THROTTLED),
    TSQF_QUEUED			= BIT(TSQ_QUEUED),
    TCPF_TSQ_DEFERRED		= BIT(TCP_TSQ_DEFERRED),
    TCPF_WRITE_TIMER_DEFERRED	= BIT(TCP_WRITE_TIMER_DEFERRED),
    TCPF_DELACK_TIMER_DEFERRED	= BIT(TCP_DELACK_TIMER_DEFERRED),
    TCPF_MTU_REDUCED_DEFERRED	= BIT(TCP_MTU_REDUCED_DEFERRED),
    TCPF_ACK_DEFERRED		= BIT(TCP_ACK_DEFERRED),
}

// Flags of interest for tcp_release_cb()

// Variant of tcp_sk() upgrading a const sock to a read/write tcp socket.
// Used in context of (lockless) tcp listeners.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_timewait_sock {
    pub tw_sk: inet_timewait_sock,

    pub tw_rcv_wnd: u32,
    pub tw_ts_offset: u32,
    pub tw_ts_recent: u32,
// The time we sent the last out-of-window ACK:
    pub tw_last_oow_ack_time: u32,
    pub tw_ts_recent_stamp: c_int,
    pub tw_tx_delay: u32,

    pub tw_md5_key: *mut tcp_md5sig_key,

    pub ao_info: *mut tcp_ao_info __rcu,

}

// We use READ_ONCE() here because socket might not be locked.
// This happens for listeners.
//
extern "C" {
    pub fn __tcp_sock_set_cork(sk: *mut sock, on: bool);
}
extern "C" {
    pub fn tcp_sock_set_cork(sk: *mut sock, on: bool);
}
extern "C" {
    pub fn tcp_sock_set_keepcnt(sk: *mut sock, val: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_sock_set_keepidle_locked(sk: *mut sock, val: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_sock_set_keepidle(sk: *mut sock, val: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_sock_set_keepintvl(sk: *mut sock, val: c_int) -> c_int;
}
extern "C" {
    pub fn __tcp_sock_set_nodelay(sk: *mut sock, on: bool);
}
extern "C" {
    pub fn tcp_sock_set_nodelay(sk: *mut sock);
}
extern "C" {
    pub fn tcp_sock_set_quickack(sk: *mut sock, val: c_int);
}
extern "C" {
    pub fn tcp_sock_set_syncnt(sk: *mut sock, val: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_sock_set_user_timeout(sk: *mut sock, val: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_sock_set_maxseg(sk: *mut sock, val: c_int) -> c_int;
}
extern "C" {
    pub fn dst_feature(_arg: dst, _arg: RTAX_FEATURE_TCP_USEC_TS) -> return;
}
