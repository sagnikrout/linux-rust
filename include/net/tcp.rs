//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tcp.h
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
// Definitions for the TCP module.
//
// Version:	@(#)tcp.h	1.0.5	05/23/93
//
// Authors:	Ross Biro
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//
pub const FASTRETRANS_DEBUG: c_int = 1;

extern "C" {
    pub fn tcp_orphan_count_sum() -> c_int;
}
extern "C" {
    pub fn tcp_time_wait(sk: *mut sock, state: c_int, timeo: c_int);
}

pub const MAX_TCP_OPTION_SPACE: c_int = 40;
pub const TCP_MIN_SND_MSS: c_int = 48;

//
// Never offer a window over 32767 without using window scaling. Some
// poor stacks do signed 16bit maths!
//

// Minimal accepted MSS. It is (60+60+8) - (20+20).

// The initial MTU to use for probing
pub const TCP_BASE_MSS: c_int = 1024;
// probing interval, default to 10 minutes as per RFC4821
pub const TCP_PROBE_INTERVAL: c_int = 600;
// Specify interval when tcp mtu probing will stop
pub const TCP_PROBE_THRESHOLD: c_int = 8;
// After receiving this amount of duplicate ACKs fast retransmit starts.
pub const TCP_FASTRETRANS_THRESH: c_int = 3;
// Maximal number of ACKs sent quickly to accelerate slow-start.

// Maximal number of window scale according to RFC1323

// Default sending frequency of accurate ECN option per RTT
pub const TCP_ACCECN_OPTION_BEACON: c_int = 3;
// urg_data states
pub const TCP_URG_VALID: c_uint = 0x0100;
pub const TCP_URG_NOTYET: c_uint = 0x0200;
pub const TCP_URG_READ: c_uint = 0x0400;

// This is how many retries it does before it
// tries to figure out if the gateway is
// down. Minimal RFC value is 3; it corresponds
// to ~3sec-8min depending on RTO.
//

// This should take at least
// 90 minutes to time out.
// RFC1122 says that the limit is 100 sec.
// 15 is ~13-30min depending on RTO.
//

// when active opening a connection.
// RFC1122 says the minimum retry MUST
// be at least 180secs.  Nevertheless
// this value is corresponding to
// 63secs of retransmission with the
// current initial RTO.
//

// when passive opening a connection.
// This is corresponding to 31secs of
// retransmission with the current
// initial RTO.
//

// state, about 60 seconds

// BSD style FIN_WAIT2 deadlock breaker.
// It used to be 3min, new value is 60sec,
// to combine FIN-WAIT-2 timeout with
// TIME-WAIT timer.
//

pub const TCP_RTO_MAX_SEC: c_int = 120;

// used as a fallback RTO for the
// initial data transmission if no
// valid RTT sample has been acquired,
// most likely due to retrans in 3WHS.
//

// for local resources.
//

pub const MAX_TCP_KEEPIDLE: c_int = 32767;
pub const MAX_TCP_KEEPINTVL: c_int = 32767;
pub const MAX_TCP_KEEPCNT: c_int = 127;
pub const MAX_TCP_SYNCNT: c_int = 127;
// Ensure that TCP PAWS checks are relaxed after ~2147 seconds
// to avoid overflows. This assumes a clock smaller than 1 Mhz.
// Default clock is 1 Khz, tcp_usec_ts uses 1 Mhz.
//

// after this time. It should be equal
// (or greater than) TCP_TIMEWAIT_LEN
// to provide reliability equal to one
// provided by timewait state.
//

// timestamps. It must be less than
// minimal timewait lifetime.
//
// TCP option
//

// Magic number to be after the option value for sharing TCP
// experimental options. See draft-ietf-tcpm-experimental-options-00.txt
//
pub const TCPOPT_FASTOPEN_MAGIC: c_uint = 0xF989;
pub const TCPOPT_SMC_MAGIC: c_uint = 0xE2D4C3D9;
//
// TCP option lengths
//
pub const TCPOLEN_MSS: c_int = 4;
pub const TCPOLEN_WINDOW: c_int = 3;
pub const TCPOLEN_SACK_PERM: c_int = 2;
pub const TCPOLEN_TIMESTAMP: c_int = 10;
pub const TCPOLEN_MD5SIG: c_int = 18;
pub const TCPOLEN_FASTOPEN_BASE: c_int = 2;
pub const TCPOLEN_ACCECN_BASE: c_int = 2;
pub const TCPOLEN_EXP_FASTOPEN_BASE: c_int = 4;
pub const TCPOLEN_EXP_SMC_BASE: c_int = 6;
// But this is what stacks really send out.
pub const TCPOLEN_TSTAMP_ALIGNED: c_int = 12;
pub const TCPOLEN_WSCALE_ALIGNED: c_int = 4;
pub const TCPOLEN_SACKPERM_ALIGNED: c_int = 4;
pub const TCPOLEN_SACK_BASE: c_int = 2;
pub const TCPOLEN_SACK_BASE_ALIGNED: c_int = 4;
pub const TCPOLEN_SACK_PERBLOCK: c_int = 8;
pub const TCPOLEN_MD5SIG_ALIGNED: c_int = 20;
pub const TCPOLEN_MSS_ALIGNED: c_int = 4;
pub const TCPOLEN_EXP_SMC_BASE_ALIGNED: c_int = 8;
pub const TCPOLEN_ACCECN_PERFIELD: c_int = 3;
// Maximum number of byte counters in AccECN option + size
pub const TCP_ACCECN_NUMFIELDS: c_int = 3;

// Flags in tp->nonagle

// TCP thin-stream limits

// TCP initial congestion window as per rfc6928
pub const TCP_INIT_CWND: c_int = 10;
// Bit Flags for sysctl_tcp_fastopen
pub const TFO_CLIENT_ENABLE: c_int = 1;
pub const TFO_SERVER_ENABLE: c_int = 2;

// Accept SYN data w/o any cookie option
pub const TFO_SERVER_COOKIE_NOT_REQD: c_uint = 0x200;
// Force enable TFO on all listeners, i.e., not requiring the
// TCP_FASTOPEN socket option.
//
pub const TFO_SERVER_WO_SOCKOPT1: c_uint = 0x400;
// sysctl variables for tcp
pub const TCP_RACK_LOSS_DETECTION: c_uint = 0x1 /* Use RACK to detect losses */;
pub const TCP_RACK_STATIC_REO_WND: c_uint = 0x2 /* Use static RACK reo wnd */;
pub const TCP_RACK_NO_DUPTHRESH: c_uint = 0x4 /* Do not use DUPACK threshold in RACK */;
// optimized version of sk_under_memory_pressure() for TCP sockets
extern "C" {
    pub fn READ_ONCE(_arg: tcp_memory_pressure) -> return;
}
//
// The next routines deal with comparing 32 bit unsigned ints
// and worry about wraparound (automatic with unsigned arithmetic).
//

// is s2<=s1<=s3 ?
extern "C" {
    pub fn sk_forced_mem_schedule(sk: *mut sock, size: c_int);
}
extern "C" {
    pub fn tcp_check_oom(sk: *const sock, shift: c_int) -> bool;
}

//
// TCP splice context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_splice_state {
    pub pipe: *mut pipe_inode_info,
    pub len: usize,
    pub flags: c_uint,
}

extern "C" {
    pub fn tcp_tsq_work_init();
}
extern "C" {
    pub fn tcp_v4_err(skb: *mut sk_buff, _arg: u32) -> c_int;
}
extern "C" {
    pub fn tcp_shutdown(sk: *mut sock, how: c_int);
}
extern "C" {
    pub fn tcp_v4_rcv(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn tcp_remove_empty_skb(sk: *mut sock);
}
extern "C" {
    pub fn tcp_sendmsg(sk: *mut sock, msg: *mut msghdr, size: usize) -> c_int;
}
extern "C" {
    pub fn tcp_sendmsg_locked(sk: *mut sock, msg: *mut msghdr, size: usize) -> c_int;
}
extern "C" {
    pub fn tcp_splice_eof(sock: *mut socket);
}
extern "C" {
    pub fn tcp_send_mss(sk: *mut sock, size_goal: *mut c_int, flags: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_wmem_schedule(sk: *mut sock, copy: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_release_cb(sk: *mut sock);
}

extern "C" {
    pub fn tcp_write_timer_handler(sk: *mut sock);
}
extern "C" {
    pub fn tcp_delack_timer_handler(sk: *mut sock);
}
extern "C" {
    pub fn tcp_ioctl(sk: *mut sock, cmd: c_int, karg: *mut c_int) -> c_int;
}
extern "C" {
    pub fn tcp_rcv_state_process(sk: *mut sock, skb: *mut sk_buff) -> skb_drop_reason;
}
extern "C" {
    pub fn tcp_rcv_established(sk: *mut sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn tcp_rcvbuf_grow(sk: *mut sock, newval: u32);
}
extern "C" {
    pub fn tcp_rcv_space_adjust(sk: *mut sock);
}
extern "C" {
    pub fn tcp_twsk_unique(sk: *mut sock, sktw: *mut sock, twp: *mut c_void) -> c_int;
}
extern "C" {
    pub fn tcp_twsk_destructor(sk: *mut sock);
}
extern "C" {
    pub fn tcp_twsk_purge(net_exit_list: *mut list_head);
}
// How many ACKs S/ACKing new data have we sent?
// Leaving quickack mode we deflate ATO.

pub const TCP_ECN_DISABLED: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_tw_status {
    TCP_TW_SUCCESS = 0,
    TCP_TW_RST = 1,
    TCP_TW_ACK = 2,
    TCP_TW_SYN = 3,
    TCP_TW_ACK_OOW = 4
}

extern "C" {
    pub fn tcp_enter_loss(sk: *mut sock);
}
extern "C" {
    pub fn tcp_cwnd_reduction(sk: *mut sock, newly_acked_sacked: c_int, newly_lost: c_int, flag: c_int);
}
extern "C" {
    pub fn tcp_clear_retrans(tp: *mut tcp_sock);
}
extern "C" {
    pub fn tcp_update_pacing_rate(sk: *mut sock);
}
extern "C" {
    pub fn tcp_set_rto(sk: *mut sock);
}
extern "C" {
    pub fn tcp_update_metrics(sk: *mut sock);
}
extern "C" {
    pub fn tcp_init_metrics(sk: *mut sock);
}
extern "C" {
    pub fn tcp_metrics_init();
}
extern "C" {
    pub fn tcp_peer_is_proven(req: *mut request_sock, dst: *mut dst_entry) -> bool;
}
extern "C" {
    pub fn __tcp_close(sk: *mut sock, timeout: c_long);
}
extern "C" {
    pub fn tcp_close(sk: *mut sock, timeout: c_long);
}
extern "C" {
    pub fn tcp_init_sock(sk: *mut sock);
}
extern "C" {
    pub fn tcp_init_transfer(sk: *mut sock, bpf_op: c_int, skb: *mut sk_buff);
}
extern "C" {
    pub fn tcp_bpf_bypass_getsockopt(level: c_int, optname: c_int) -> bool;
}
extern "C" {
    pub fn tcp_reset_keepalive_timer(sk: *mut sock, timeout: c_ulong);
}
extern "C" {
    pub fn tcp_set_keepalive(sk: *mut sock, val: c_int);
}
extern "C" {
    pub fn tcp_syn_ack_timeout(req: *const request_sock);
}
extern "C" {
    pub fn tcp_set_rcvlowat(sk: *mut sock, val: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_set_rcvbuf(sk: *mut sock, val: c_int);
}
extern "C" {
    pub fn tcp_set_window_clamp(sk: *mut sock, val: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_data_ready(sk: *mut sock);
}

//
// BPF SKB-less helpers
//
extern "C" {
    pub fn tcp_parse_mss_option(th: *const tcphdr, user_mss: u16) -> u16;
}
//
// TCP v4 functions exported for the inet6 API
//
extern "C" {
    pub fn tcp_v4_mtu_reduced(sk: *mut sock);
}
extern "C" {
    pub fn tcp_req_err(sk: *mut sock, seq: u32, abort: bool);
}
extern "C" {
    pub fn tcp_ld_RTO_revert(sk: *mut sock, seq: u32);
}
extern "C" {
    pub fn tcp_v4_conn_request(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn tcp_ca_openreq_child(sk: *mut sock, dst: *const dst_entry);
}
extern "C" {
    pub fn tcp_v4_do_rcv(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn tcp_v4_connect(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_connect(sk: *mut sock) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_synack_type {
    TCP_SYNACK_NORMAL,
    TCP_SYNACK_FASTOPEN,
    TCP_SYNACK_COOKIE,
    TCP_SYNACK_RETRANS,
}

extern "C" {
    pub fn tcp_disconnect(sk: *mut sock, flags: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_finish_connect(sk: *mut sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn tcp_send_rcvq(sk: *mut sock, msg: *mut msghdr, size: usize) -> c_int;
}
extern "C" {
    pub fn inet_sk_rx_dst_set(sk: *mut sock, skb: *const sk_buff);
}
// From syncookies.c
extern "C" {
    pub fn __cookie_v4_check(iph: *const iphdr, th: *const tcphdr) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tcp_req_attrs {
    pub rcv_tsval: u32,
    pub rcv_tsecr: u32,
    pub mss: u16,
    pub rcv_wscale: u8,
    pub snd_wscale: u8,
    pub ecn_ok: u8,
    pub wscale_ok: u8,
    pub sack_ok: u8,
    pub tstamp_ok: u8,
    pub usec_ts_ok: u8,
    pub reserved: [u8; 3],
}

// Syncookies use a monotonic timer which increments every 60 seconds.
// This counter is used both as a hash input and partially encoded into
// the cookie value.  A cookie is only validated further if the delta
// between the current counter value and the encoded one is less than this,
// i.e. a sent cookie is valid only at most for 2*60 seconds (or less if
// the counter advances immediately after a cookie is generated).
//
pub const MAX_SYNCOOKIE_AGE: c_int = 2;

// syncookies: remember time of last synqueue overflow
// But do not dirty this field too often (once per second is enough)
// It is racy as we do not hold a lock, but race is very minor.
//
// syncookies: no recent synqueue overflow on this listening socket?
// If last_overflow <= jiffies <= last_overflow + TCP_SYNCOOKIE_VALID,
// then we're under synflood. However, we have to use
// 'last_overflow - HZ' as lower bound. That's because a concurrent
// tcp_synq_overflow() could update .ts_recent_stamp after we read
// jiffies but before we store .ts_recent_stamp into last_overflow,
// which could lead to rejecting a valid syncookie.
//
// Convert one nsec 64bit timestamp to ts (ms or usec resolution)
extern "C" {
    pub fn div_u64(_arg: val, _arg: NSEC_PER_USEC) -> return;
}
extern "C" {
    pub fn div_u64(_arg: val, _arg: NSEC_PER_MSEC) -> return;
}
extern "C" {
    pub fn cookie_v4_init_sequence(skb: *const sk_buff, mss: *mut __u16) -> __u32;
}
extern "C" {
    pub fn cookie_init_timestamp(req: *mut request_sock, now: u64) -> u64;
}

// From net/ipv6/syncookies.c
extern "C" {
    pub fn __cookie_v6_check(iph: *const ipv6hdr, th: *const tcphdr) -> c_int;
}
extern "C" {
    pub fn cookie_v6_init_sequence(skb: *const sk_buff, mss: *mut __u16) -> __u32;
}

// tcp_output.c
extern "C" {
    pub fn tcp_skb_entail(sk: *mut sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn tcp_mark_push(tp: *mut tcp_sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn __tcp_retransmit_skb(sk: *mut sock, skb: *mut sk_buff, segs: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_retransmit_skb(sk: *mut sock, skb: *mut sk_buff, segs: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_retransmit_timer(sk: *mut sock);
}
extern "C" {
    pub fn tcp_xmit_retransmit_queue(: *mut sock);
}
extern "C" {
    pub fn tcp_simple_retransmit(: *mut sock);
}
extern "C" {
    pub fn tcp_enter_recovery(sk: *mut sock, ece_ack: bool);
}
extern "C" {
    pub fn tcp_trim_head(: *mut sock, : *mut sk_buff, _arg: u32) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_queue {
    TCP_FRAG_IN_WRITE_QUEUE,
    TCP_FRAG_IN_RTX_QUEUE,
}

extern "C" {
    pub fn tcp_send_probe0(: *mut sock);
}
extern "C" {
    pub fn tcp_write_wakeup(: *mut sock, mib: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_send_fin(sk: *mut sock);
}
extern "C" {
    pub fn tcp_send_active_reset(sk: *mut sock, reason: sk_rst_reason);
}
extern "C" {
    pub fn tcp_send_synack(: *mut sock) -> c_int;
}
extern "C" {
    pub fn tcp_push_one(: *mut sock, mss_now: c_uint);
}
extern "C" {
    pub fn __tcp_send_ack(sk: *mut sock, rcv_nxt: u32, flags: u16);
}
extern "C" {
    pub fn tcp_send_ack(sk: *mut sock);
}
extern "C" {
    pub fn tcp_send_delayed_ack(sk: *mut sock);
}
extern "C" {
    pub fn tcp_send_loss_probe(sk: *mut sock);
}
extern "C" {
    pub fn tcp_schedule_loss_probe(sk: *mut sock, advancing_rto: bool) -> bool;
}
// tcp_input.c
extern "C" {
    pub fn tcp_rearm_rto(sk: *mut sock);
}
extern "C" {
    pub fn tcp_synack_rtt_meas(sk: *mut sock, req: *mut request_sock);
}
extern "C" {
    pub fn tcp_done_with_error(sk: *mut sock, err: c_int);
}
extern "C" {
    pub fn tcp_reset(sk: *mut sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn tcp_fin(sk: *mut sock);
}
extern "C" {
    pub fn __tcp_check_space(sk: *mut sock);
}
// pairs with tcp_poll()
extern "C" {
    pub fn tcp_sack_compress_send_ack(sk: *mut sock);
}
// tcp_timer.c
extern "C" {
    pub fn tcp_init_xmit_timers(: *mut sock);
}
extern "C" {
    pub fn tcp_sync_mss(sk: *mut sock, pmtu: u32) -> c_uint;
}
extern "C" {
    pub fn tcp_current_mss(sk: *mut sock) -> c_uint;
}
extern "C" {
    pub fn tcp_clamp_probe0_to_user_timeout(sk: *const sock, when: u32) -> u32;
}
// Bound MSS / TSO packet size with the half of the window
// When peer uses tiny windows, there is no use in packetizing
// to sub-MSS pieces for the sake of SWS or making sure there
// are enough packets in the pipe for fast recovery.
//
// On the other hand, for extremely large MSS devices, handling
// smaller than MSS windows in this way does make sense.
//
extern "C" {
    pub fn max_t(_arg: c_int, _arg: cutoff, tp->tcp_header_len: 68U -) -> return;
}
// tcp.c
extern "C" {
    pub fn tcp_get_info(: *mut sock, : *mut tcp_info);
}
extern "C" {
    pub fn tcp_rate_check_app_limited(sk: *mut sock);
}
// Read 'sendfile()'-style from a TCP socket
extern "C" {
    pub fn tcp_read_skb(sk: *mut sock, recv_actor: skb_read_actor_t) -> c_int;
}
extern "C" {
    pub fn tcp_read_done(sk: *mut sock, len: usize);
}
extern "C" {
    pub fn tcp_initialize_rcv_mss(sk: *mut sock);
}
extern "C" {
    pub fn tcp_mtu_to_mss(sk: *mut sock, pmtu: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_mss_to_mtu(sk: *mut sock, mss: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_mtup_init(sk: *mut sock);
}
extern "C" {
    pub fn READ_ONCE(_arg: inet_csk(sk)->icsk_rto_max) -> return;
}
extern "C" {
    pub fn usecs_to_jiffies(tp->rttvar_us: (tp->srtt_us >> 3) +) -> return;
}
extern "C" {
    pub fn tcp_delack_max(sk: *const sock) -> u32;
}
// Compute the actual rto_min value
extern "C" {
    pub fn jiffies_to_usecs(_arg: tcp_rto_min(sk)) -> return;
}
extern "C" {
    pub fn dst_metric_locked(_arg: dst, _arg: RTAX_CC_ALGO) -> return;
}
// Minimum RTT in usec. ~0 means not available.
extern "C" {
    pub fn minmax_get(_arg: &tp->rtt_min) -> return;
}
// Compute the actual receive window we are currently advertising.
// Rcv_nxt can be after the window if our peer push more data
// than the offered window.
//
// Compute the maximum receive window we ever advertised.
// Rcv_nxt can be after the window if our peer push more data
// than the offered window.
//
// Check if we need to update the maximum receive window sequence number
// Choose a new window, without checks for shrinking, and without
// scaling applied to the result.  The caller does these things
// if necessary.  This is a "raw" window selection.
//
extern "C" {
    pub fn __tcp_select_window(sk: *mut sock) -> u32;
}
extern "C" {
    pub fn tcp_send_window_probe(sk: *mut sock);
}
// TCP uses 32bit jiffies to save some space.
// Note that this is different from tcp_time_stamp, which
// historically has been the same until linux-4.13.
//

//
// Deliver a 32bit value for TCP timestamp option (RFC 7323)
// It is no longer tied to jiffies, but to 1 ms clock.
// Note: double check if you want to use tcp_jiffies32 instead of this.
//
pub const TCP_TS_HZ: c_int = 1000;
extern "C" {
    pub fn ktime_get_ns() -> return;
}
extern "C" {
    pub fn div_u64(_arg: tcp_clock_ns(), _arg: NSEC_PER_USEC) -> return;
}
extern "C" {
    pub fn div_u64(_arg: tcp_clock_ns(), _arg: NSEC_PER_MSEC) -> return;
}
// TCP Timestamp included in TS option (RFC 1323) can either use ms
// or usec resolution. Each socket carries a flag to select one or other
// resolution, as the route attribute could change anytime.
// Each flow must stick to initial resolution.
//
extern "C" {
    pub fn div_u64(_arg: tp->tcp_mstamp, _arg: USEC_PER_MSEC) -> return;
}
extern "C" {
    pub fn tcp_time_stamp_ms(_arg: tp) -> return;
}
// Refresh clocks of a TCP socket,
// ensuring monotically increasing values.
//
extern "C" {
    pub fn tcp_mstamp_refresh(tp: *mut tcp_sock);
}
extern "C" {
    pub fn max_t(_arg: i64, t0: t1 -, _arg: 0) -> return;
}
// provide the departure time in us unit
extern "C" {
    pub fn div_u64(_arg: skb->skb_mstamp_ns, _arg: NSEC_PER_USEC) -> return;
}
// Provide skb TSval in usec or ms unit
extern "C" {
    pub fn tcp_skb_timestamp_us(_arg: skb) -> return;
}
extern "C" {
    pub fn div_u64(_arg: skb->skb_mstamp_ns, _arg: NSEC_PER_MSEC) -> return;
}

pub const TCP_ACCECN_CEP_ACE_MASK: c_uint = 0x7;
pub const TCP_ACCECN_ACE_MAX_DELTA: c_int = 6;
// To avoid/detect middlebox interference, not all counters start at 0.
// See draft-ietf-tcpm-accurate-ecn for the latest values.
//
pub const TCP_ACCECN_CEP_INIT_OFFSET: c_int = 5;
pub const TCP_ACCECN_E1B_INIT_OFFSET: c_int = 1;
pub const TCP_ACCECN_E0B_INIT_OFFSET: c_int = 1;
pub const TCP_ACCECN_CEB_INIT_OFFSET: c_int = 0;
// State flags for sacked in struct tcp_skb_cb
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_skb_cb_sacked_flags {
    TCPCB_SACKED_ACKED	= (1 << 0),	/* SKB ACK'd by a SACK block	*/
    TCPCB_SACKED_RETRANS	= (1 << 1),	/* SKB retransmitted		*/
    TCPCB_LOST		= (1 << 2),	/* SKB is lost			*/
    TCPCB_TAGBITS		= (TCPCB_SACKED_ACKED | TCPCB_SACKED_RETRANS |
    TCPCB_LOST),	/* All tag bits			*/
    TCPCB_REPAIRED		= (1 << 4),	/* SKB repaired (no skb_mstamp_ns)	*/
    TCPCB_EVER_RETRANS	= (1 << 7),	/* Ever retransmitted frame	*/
    TCPCB_RETRANS		= (TCPCB_SACKED_RETRANS | TCPCB_EVER_RETRANS |
    TCPCB_REPAIRED),
}

// This is what the send packet queuing engine uses to pass
// TCP per-packet control information to the transmission code.
// We also store the host-order sequence numbers in here too.
// This is 44 bytes if IPV6 is enabled.
// If this grows please adjust skbuff.h:skbuff->cb[xxx] size appropriately.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_skb_cb {
    pub /: *mut *mut __u32 seq; / Starting sequence number,
    pub /: *mut *mut __u32 end_seq; / SEQ + FIN + SYN + datalen,
// Notes :
// tcp_tw_isn is used in input path only
// (isn chosen by tcp_timewait_state_process())
// tcp_gso_segs/size are used in write queue only,
// cf tcp_skb_pcount()/tcp_skb_mss()
//
    pub tcp_tw_isn: u32,
    pub tcp_gso_segs: u16,
    pub tcp_gso_size: u16,
}

pub const TSTAMP_ACK_SK: c_uint = 0x1;
pub const TSTAMP_ACK_BPF: c_uint = 0x2;

// There is space for up to 24 bytes
// pkts S/ACKed so far upon tx of skb, incl retrans:
// start of send pipeline phase
// when we reached the "delivered" count

// This is the variant of inet6_iif() that must be used by TCP,
// as TCP moves IP6CB into a different location in skb->cb[]
//
// TCP_SKB_CB reference means this can not be used from early demux

// TCP_SKB_CB reference means this can not be used from early demux

// Due to TSO, an SKB can be composed of multiple actual
// packets.  To keep these tracked properly, we use this.
//
// This is valid iff skb is in write queue and tcp_skb_pcount() > 1.
extern "C" {
    pub fn likely(_arg: !TCP_SKB_CB(skb)->eor) -> return;
}
// skb_cmp_decrypted() not needed, use tcp_write_collapse_fence()
// Events passed to congestion control interface
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_ca_event {
    CA_EVENT_TX_START,	/* first transmit when no packets in flight */
    CA_EVENT_CWND_RESTART,	/* congestion window restart */
    CA_EVENT_COMPLETE_CWR,	/* end of congestion recovery */
    CA_EVENT_LOSS,		/* loss timeout */
    CA_EVENT_ECN_NO_CE,	/* ECT set, but not CE marked */
    CA_EVENT_ECN_IS_CE,	/* received CE marked IP packet */
}

// Information about inbound ACK, passed to cong_ops->in_ack_event()
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_ca_ack_event_flags {
    CA_ACK_SLOWPATH		= (1 << 0),	/* In slow path processing */
    CA_ACK_WIN_UPDATE	= (1 << 1),	/* ACK updated window */
    CA_ACK_ECE		= (1 << 2),	/* ECE bit is set on ack */
}

//
// Interface for adding new TCP congestion control handlers
//
pub const TCP_CA_NAME_MAX: c_int = 16;
pub const TCP_CA_MAX: c_int = 128;

pub const TCP_CA_UNSPEC: c_int = 0;
// Algorithm can be set on socket without CAP_NET_ADMIN privileges

// Requires ECN/ECT set on all packets

// Require successfully negotiated AccECN capability

// Use ECT(1) instead of ECT(0) while the CA is uninitialized

// Cannot fallback to RFC3168 during AccECN negotiation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ack_sample {
    pub pkts_acked: u32,
    pub rtt_us: i32,
    pub in_flight: u32,
}

// A rate sample measures the number of (original/retransmitted) data
// packets delivered "delivered" over an interval of time "interval_us".
// The tcp_rate.c code fills in the rate sample, and congestion
// control modules that define a cong_control function to run at the end
// of ACK processing can optionally chose to consult this sample when
// setting cwnd and pacing rate.
// A sample is invalid if "delivered" or "interval_us" is negative.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rate_sample {
    pub /: *mut *mut u64 prior_mstamp; / starting timestamp for interval,
    pub /: *mut *mut u32 prior_delivered; / tp->delivered at "prior_mstamp",
    pub /: *mut *mut u32 prior_delivered_ce;/ tp->delivered_ce at "prior_mstamp",
    pub /: *mut *mut s32 delivered; / number of packets delivered over interval,
    pub marks*/: *mut *mut s32 delivered_ce; / number of packets delivered w/ CE,
    pub /: *mut *mut long interval_us; / time for tp->delivered to incr "delivered",
    pub /: *mut *mut u32 snd_interval_us; / snd interval for delivered packets,
    pub /: *mut *mut u32 rcv_interval_us; / rcv interval for delivered packets,
    pub /: *mut *mut long rtt_us; / RTT of last (S)ACKed packet (or -1),
    pub /: *mut *mut int losses; / number of packets marked lost upon ACK,
    pub /: *mut *mut u32 acked_sacked; / number of packets newly (S)ACKed upon ACK,
    pub /: *mut *mut u32 prior_in_flight; / in flight before this ACK,
    pub /: *mut *mut u32 last_end_seq; / end_seq of most recently ACKed packet,
    pub /: *mut *mut bool is_app_limited; / is sample from packet with bubble in pipe?,
    pub /: *mut *mut bool is_retrans; / is sample from retransmission?,
    pub /: *mut *mut bool is_ack_delayed; / is this (likely) a delayed ACK?,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_congestion_ops {
// fast path fields are put first to fill one cache line
// A congestion control (CC) must provide one of either:
//
// (a) a cong_avoid function, if the CC wants to use the core TCP
// stack's default functionality to implement a "classic"
// (Reno/CUBIC-style) response to packet loss, RFC3168 ECN,
// idle periods, pacing rate computations, etc.
//
// (b) a cong_control function, if the CC wants custom behavior and
// complete control of all congestion control behaviors.
//
// (a) "classic" response: calculate new cwnd.
//
    pub acked): *mut *mut *mut void (cong_avoid)(struct sock sk, u32 ack, u32,
// (b) "custom" response: call when packets are delivered to update
// cwnd and pacing rate, after all the ca_state processing.
//
    pub rs): *const *const *const void (cong_control)(struct sock sk, u32 ack, int flag, struct rate_sample,
// return slow start threshold (required)
    pub sk): *mut *mut u32 (ssthresh)(struct sock,
// call before changing ca_state (optional)
    pub new_state): *mut *mut *mut void (set_state)(struct sock sk, u8,
// call when cwnd event occurs (optional)
    pub ev): *mut *mut *mut void (cwnd_event)(struct sock sk, enum tcp_ca_event,
// call when CA_EVENT_TX_START cwnd event occurs (optional)
    pub sk): *mut *mut void (cwnd_event_tx_start)(struct sock,
// call when ack arrives (optional)
    pub flags): *mut *mut *mut void (in_ack_event)(struct sock sk, u32,
// hook for packet ack accounting (optional)
    pub sample): *const *const *const void (pkts_acked)(struct sock sk, struct ack_sample,
// override sysctl_tcp_min_tso_segs (optional)
    pub sk): *mut *mut u32 (min_tso_segs)(struct sock,
// new value of cwnd after loss (required)
    pub sk): *mut *mut u32 (undo_cwnd)(struct sock,
// returns the multiplier used in tcp_sndbuf_expand (optional)
    pub sk): *mut *mut u32 (sndbuf_expand)(struct sock,
// control/slow paths put last
// get info for inet_diag (optional)
    pub info): *mut tcp_cc_info,
    pub name: [c_char; TCP_CA_NAME_MAX],
    pub owner: *mut module,
    pub list: list_head,
    pub key: u32,
    pub flags: u32,
// initialize private data (optional)
    pub sk): *mut *mut void (init)(struct sock,
// cleanup private data  (optional)
    pub sk): *mut *mut void (release)(struct sock,
    pub ____cacheline_aligned_in_smp: },
    pub type): *mut int tcp_register_congestion_control(struct tcp_congestion_ops,
    pub type): *mut void tcp_unregister_congestion_control(struct tcp_congestion_ops,
    pub old_type): *mut tcp_congestion_ops,
    pub ca): *mut int tcp_validate_congestion_control(struct tcp_congestion_ops,
    pub sk): *mut void tcp_assign_congestion_control(struct sock,
    pub sk): *mut void tcp_init_congestion_control(struct sock,
    pub sk): *mut void tcp_cleanup_congestion_control(struct sock,
    pub name): *const *const int tcp_set_default_congestion_control(struct net net, char,
    pub name): *mut *mut void tcp_get_default_congestion_control(struct net net, char,
    pub len): *mut *mut void tcp_get_available_congestion_control(char buf, size_t,
    pub len): *mut *mut void tcp_get_allowed_congestion_control(char buf, size_t,
    pub allowed): *mut int tcp_set_allowed_congestion_control(char,
    pub cap_net_admin): bool,
    pub acked): *mut *mut u32 tcp_slow_start(struct tcp_sock tp, u32,
    pub acked): *mut *mut void tcp_cong_avoid_ai(struct tcp_sock tp, u32 w, u32,
    pub sk): *mut u32 tcp_reno_ssthresh(struct sock,
    pub sk): *mut u32 tcp_reno_undo_cwnd(struct sock,
    pub acked): *mut *mut void tcp_reno_cong_avoid(struct sock sk, u32 ack, u32,
    pub tcp_reno: extern struct tcp_congestion_ops,
    pub name): *const *const tcp_congestion_ops tcp_ca_find(char,
    pub key): *mut *mut tcp_congestion_ops tcp_ca_find_key(u32,
    pub ecn_ca): *const *const u32 tcp_ca_get_key_by_name(char name, bool,

    pub buffer): *mut *mut char tcp_ca_get_name_by_key(u32 key, char,

    pub NULL: return,

    pub inet_csk(sk): *const *const inet_connection_sock icsk =,
    pub TCP_CONG_NEEDS_ECN: return icsk->icsk_ca_ops->flags &,
    pub inet_csk(sk): *const *const inet_connection_sock icsk =,
    pub TCP_CONG_NEEDS_ACCECN: return icsk->icsk_ca_ops->flags &,
    pub inet_csk(sk): *const *const inet_connection_sock icsk =,
    pub TCP_CONG_ECT_1_NEGOTIATION: return icsk->icsk_ca_ops->flags &,
    pub inet_csk(sk): *const *const inet_connection_sock icsk =,
    pub TCP_CONG_NO_FALLBACK_RFC3168: return icsk->icsk_ca_ops->flags &,
    pub inet_csk(sk): *const *const inet_connection_sock icsk =,
    pub event): icsk->icsk_ca_ops->cwnd_event(sk,,
// From tcp_cong.c
    pub ca_state): *const *const void tcp_set_ca_state(struct sock sk, u8,
    pub seq2)): return t1 > t2 || (t1 == t2 && after(seq1,,
// These functions determine how the current flow behaves in respect of SACK
// handling. SACK is negotiated with the peer, and therefore it can vary
// between different flows.
//
// tcp_is_sack - SACK enabled
// tcp_is_reno - No SACK
//
    pub likely(tp->rx_opt.sack_ok): return,
    pub !tcp_is_sack(tp): return,
    pub tp->lost_out: return tp->sacked_out +,
// This determines how many packets are "in the network" to the best
// of our knowledge.  In many cases it is conservative, but where
// detailed information is available from the receiver (via SACK
// blocks etc.) we can make more aggressive calculations.
//
// Use this for decisions involving congestion control, use just
// tp->packets_out to determine if the send queue is empty or not.
//
// Read this equation as:
//
// "Packets sent once on transmission queue" MINUS
// "Packets left network, but not honestly ACKed yet" PLUS
// "Packets fast retransmitted"
//
    pub tp->retrans_out: return tp->packets_out - tcp_left_out(tp) +,
pub const TCP_INFINITE_SSTHRESH: c_uint = 0x7fffffff;
    pub tp->snd_cwnd: return,
    pub 0): WARN_ON_ONCE((int)val <=,
    pub val): WRITE_ONCE(tp->snd_cwnd,,
    pub tp->snd_ssthresh: return tcp_snd_cwnd(tp) <,
    pub TCP_INFINITE_SSTHRESH: return tp->snd_ssthresh >=,
    pub inet_csk(sk)->icsk_ca_state): (1 <<,
// If cwnd > ssthresh, we may raise ssthresh to be half-way to cwnd.
// The exception is cwnd reduction phase, when cwnd is decreasing towards
// ssthresh.
//
    pub tcp_sk(sk): *const *const tcp_sock tp =,
    pub tp->snd_ssthresh: return,
    pub 2))): (tcp_snd_cwnd(tp) >>,
// Use define here intentionally to get WARN_ON location shown at the caller

    pub sk): *mut void tcp_enter_cwr(struct sock,
    pub dst): *const *const __u32 tcp_init_cwnd(struct tcp_sock tp, struct dst_entry,
// The maximum number of MSS of available cwnd for which TSO defers
// sending if not using sysctl_tcp_tso_win_divisor.
//
    pub 3: return,
// Returns end sequence number of the receiver's advertised window
    pub tp->snd_wnd: return tp->snd_una +,
// We follow the spirit of RFC2861 to validate cwnd but implement a more
// flexible approach. The RFC suggests cwnd should not be raised unless
// it was fully used previously. And that's exactly what we do in
// congestion avoidance mode. But in slow start we allow cwnd to grow
// as long as the application has used half the cwnd.
// Example :
// cwnd is 10 (IW10), but application sends 9 frames.
// We allow cwnd to reach 18 when all frames are ACKed.
// This check is safe because it's as aggressive as slow start which already
// risks 100% overshoot. The advantage is that we discourage application to
// either send more filler packets or data to artificially blow up the cwnd
// usage, and allow application-limited process to probe bw more aggressively.
//
    pub tcp_sk(sk): *const *const tcp_sock tp =,
    pub true: return,
// If in slow start, ensure cwnd grows to twice what was ACKed.
    pub tp->max_packets_out: *mut *mut return tcp_snd_cwnd(tp) < 2,
    pub false: return,
// BBR congestion control needs pacing.
// Same remark for SO_MAX_PACING_RATE.
// sch_fq packet scheduler is efficiently handling pacing,
// but is not always installed/used.
// Return true if TCP stack should pace packets itself.
//
    pub SK_PACING_NEEDED: return smp_load_acquire(&sk->sk_pacing_status) ==,
// Estimates in how many jiffies next packet for this flow can be sent.
// Scheduling a retransmit timer too early would be silly.
//
    pub tcp_sk(sk)->tcp_clock_cache: s64 delay = tcp_sk(sk)->tcp_wstamp_ns -,
    pub 0: return delay > 0 ? nsecs_to_jiffies(delay) :,
    pub tcp_pacing_delay(sk): when +=,
// Something is really bad, we could not queue an additional packet,
// because qdisc is full or receiver sent a 0 window, or we are paced.
// We do not want to add fuel to the fire, or abort too early,
// so make sure the timer we arm now is at least 200ms in the future,
// regardless of current icsk_rto value (as it could be ~2ms)
//
    pub TCP_RTO_MIN): return max_t(unsigned long, inet_csk(sk)->icsk_rto,,
// Variant of inet_csk_rto_backoff() used for zero window probes
    pub backoff: u64 when = (u64)tcp_probe0_base(sk) <<,
    pub max_when): return (unsigned long)min_t(u64, when,,
    pub true): tcp_probe0_base(sk),,
    pub seq: tp->snd_wl1 =,
    pub seq: tp->snd_wl1 =,
//
// Calculate(/check) TCP checksum
//
    pub base): return csum_tcpudp_magic(saddr, daddr, len, IPPROTO_TCP,,
    pub skb): *mut *mut skb_drop_reason tcp_add_backlog(struct sock sk, struct sk_buff,
    pub )skb->data: *const *const tcphdr th = (tcphdr,
    pub __tcp_hdrlen(th)): return sk_filter_trim_cap(sk, skb,,
    pub state): *mut *mut void tcp_set_state(struct sock sk, int,
    pub sk): *mut void tcp_done(struct sock,
    pub err): *mut *mut int tcp_abort(struct sock sk, int,
    pub 0: rx_opt->dsack =,
    pub 0: rx_opt->num_sacks =,
    pub delta): *mut *mut void tcp_cwnd_restart(struct sock sk, s32,
    pub inet_csk(sk)->icsk_ca_ops: *const *const tcp_congestion_ops ca_ops =,
    pub tcp_sk(sk): *mut *mut tcp_sock tp =,
    pub delta: i32,
    pub tp->lsndtime: delta = tcp_jiffies32 -,
    pub delta): tcp_cwnd_restart(sk,,
// Determine a window scaling and initial window to offer.
    pub init_rcv_wnd): *mut *mut __u8 rcv_wscale, __u32,
    pub scaling_ratio: *mut *mut s64 scaled_space = (s64)space,
    pub TCP_RMEM_TO_WIN_SCALE: return scaled_space >>,
    pub space): return __tcp_win_from_space(tcp_sk(sk)->scaling_ratio,,
// inverse of __tcp_win_from_space()
    pub TCP_RMEM_TO_WIN_SCALE: u64 val = (u64)win <<,
    pub scaling_ratio): do_div(val,,
    pub val: return,
    pub win): return __tcp_space_from_win(tcp_sk(sk)->scaling_ratio,,
// Assume a 50% default for skb->len/skb->truesize ratio.
// This may be adjusted later in tcp_measure_rcv_mss().
//

    pub TCP_DEFAULT_SCALING_RATIO: tcp_sk(sk)->scaling_ratio =,
// Note: caller must be prepared to deal with negative returns
    pub READ_ONCE(sk->sk_rcvbuf)): return tcp_win_from_space(sk,,
    pub TCP_MIN_MSS): return max_t(u32, dst_metric_advmss(dst),,
    pub sk_unused_reserved_mem(sk): int unused_mem =,
    pub tcp_sk(sk): *mut *mut tcp_sock tp =,
    pub new_ssthresh): tp->rcv_ssthresh = min(tp->rcv_ssthresh,,
    pub unused_mem)): tcp_win_from_space(sk,,
    pub tcp_sk(sk)->advmss): *mut *mut __tcp_adjust_rcv_ssthresh(sk, 4U,
    pub copied): *mut *mut void tcp_cleanup_rbuf(struct sock sk, int,
    pub copied): *mut *mut void __tcp_cleanup_rbuf(struct sock sk, int,
// We provision sk_rcvbuf around 200% of sk_rcvlowat.
// If 87.5 % (7/8) of the space has been consumed, we want to override
// SO_RCVLOWAT constraint, since we are receiving skbs with too small
// len/truesize ratio.
//
    pub threshold: int rcvbuf,,
    pub true: return,
    pub READ_ONCE(sk->sk_rcvbuf): rcvbuf =,
    pub 3): threshold = rcvbuf - (rcvbuf >>,
    pub threshold: return atomic_read(&sk->sk_rmem_alloc) >,
    pub tcp_sk(sk): *const *const tcp_sock tp =,
    pub READ_ONCE(tp->copied_seq): int avail = READ_ONCE(tp->rcv_nxt) -,
    pub false: return,
    pub inet_csk(sk)->icsk_ack.rcv_mss): (tcp_receive_window(tp) <=,
    pub dst): *const dst_entry,
    pub sk): *mut void tcp_enter_memory_pressure(struct sock,
    pub sk): *mut void tcp_leave_memory_pressure(struct sock,
    pub )tp): *mut *mut net net = sock_net((sock,
    pub val: c_int,
// Paired with WRITE_ONCE() in tcp_sock_set_keepintvl()
// and do_tcp_setsockopt().
//
    pub READ_ONCE(tp->keepalive_intvl): val =,
    pub READ_ONCE(net->ipv4.sysctl_tcp_keepalive_intvl): return val ? :,
    pub )tp): *mut *mut net net = sock_net((sock,
    pub val: c_int,
// Paired with WRITE_ONCE() in tcp_sock_set_keepidle_locked()
    pub READ_ONCE(tp->keepalive_time): val =,
    pub READ_ONCE(net->ipv4.sysctl_tcp_keepalive_time): return val ? :,
    pub )tp): *mut *mut net net = sock_net((sock,
    pub val: c_int,
// Paired with WRITE_ONCE() in tcp_sock_set_keepcnt()
// and do_tcp_setsockopt().
//
    pub READ_ONCE(tp->keepalive_probes): val =,
    pub READ_ONCE(net->ipv4.sysctl_tcp_keepalive_probes): return val ? :,
    pub &tp->inet_conn: *const *const inet_connection_sock icsk =,
    pub tp->rcv_tstamp): tcp_jiffies32 -,
    pub inet_csk(sk)->icsk_rto: int rto =,
    pub 1): fin_timeout = (rto << 2) - (rto >>,
    pub fin_timeout: return,
    pub true: return,
    pub true: return,
//
// Some OSes send SYN and SYNACK messages with tsval=0 tsecr=0,
// then following tcp messages have valid values. Ignore 0 value,
// or else 'negative' tsval might forbid us to accept their packets.
//
    pub true: return,
    pub false: return,
    pub false: return,
// RST segments are not recommended to carry timestamp,
//
    pub false: return,
    pub true: return,
    pub ace: u32,
// mptcp hooks are only on the slow path
    pub 0: TCP_ACCECN_CEP_ACE_MASK) :,
    pub tp->rx_opt.snd_wscale): __tcp_fast_path_on(tp, tp->snd_wnd >>,
    pub tcp_sk(sk): *mut *mut tcp_sock tp =,
    pub last_oow_ack_time): *mut int mib_idx, u32,
    pub req): *mut request_sock,
// See RFC 2012
    pub 1): TCP_ADD_STATS(net, TCP_MIB_RTOALGORITHM,,
    pub TCP_RTO_MIN*1000/HZ): *mut TCP_ADD_STATS(net, TCP_MIB_RTOMIN,,
    pub TCP_RTO_MAX*1000/HZ): *mut TCP_ADD_STATS(net, TCP_MIB_RTOMAX,,
    pub -1): TCP_ADD_STATS(net, TCP_MIB_MAXCONN,,
// from STCP
    pub NULL: tp->retransmit_skb_hint =,

// - key database
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_md5sig_key {
    pub node: hlist_node,
    pub keylen: u8,
    pub /: *mut *mut u8 family; / AF_INET or AF_INET6,
    pub prefixlen: u8,
    pub flags: u8,
    pub addr: tcp_md5_addr,
    pub /: *mut *mut int l3index; / set if key added with L3 scope,
    pub key: [u8; TCP_MD5SIG_MAXKEYLEN],
    pub rcu: rcu_head,
}

// - sock block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_md5sig_info {
    pub head: hlist_head,
    pub rcu: rcu_head,
}

// - pseudo header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp4_pseudohdr {
    pub saddr: __be32,
    pub daddr: __be32,
    pub pad: __u8,
    pub protocol: __u8,
    pub len: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp6_pseudohdr {
    pub saddr: in6_addr,
    pub daddr: in6_addr,
    pub len: __be32,
    pub /: *mut *mut __be32 protocol; / including padding,
}

extern "C" {
    pub fn tcp_clear_md5_list(sk: *mut sock);
}

extern "C" {
    pub fn __tcp_md5_do_lookup(_arg: sk, _arg: l3index, _arg: addr, _arg: family, _arg: false) -> return;
}
extern "C" {
    pub fn __tcp_md5_do_lookup(_arg: sk, _arg: 0, _arg: addr, _arg: family, _arg: true) -> return;
}

extern "C" {
    pub fn tcp_md5_destruct_sock(sk: *mut sock);
}

extern "C" {
    pub fn tcp_md5_hash_key(ctx: *mut md5_ctx, key: *const tcp_md5sig_key);
}
// From tcp_fastopen.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_fastopen_request {
// Fast Open cookie. Size 0 means a cookie request
    pub cookie: tcp_fastopen_cookie,
    pub /: *mut *mut *mut msghdr data; / data in MSG_FASTOPEN,
    pub size: usize,
    pub /: *mut *mut int copied; / queued in tcp_connect(),
    pub uarg: *mut ubuf_info,
}

extern "C" {
    pub fn tcp_free_fastopen_req(tp: *mut tcp_sock);
}
extern "C" {
    pub fn tcp_fastopen_destroy_cipher(sk: *mut sock);
}
extern "C" {
    pub fn tcp_fastopen_ctx_destroy(net: *mut net);
}
extern "C" {
    pub fn tcp_fastopen_add_skb(sk: *mut sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn tcp_fastopen_init_key_once(net: *mut net);
}
extern "C" {
    pub fn tcp_fastopen_defer_connect(sk: *mut sock, err: *mut c_int) -> bool;
}

pub const TCP_FASTOPEN_KEY_MAX: c_int = 2;

// Fastopen key context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_fastopen_context {
    pub key: [siphash_key_t; TCP_FASTOPEN_KEY_MAX],
    pub num: c_int,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn tcp_fastopen_active_disable(sk: *mut sock);
}
extern "C" {
    pub fn tcp_fastopen_active_should_disable(sk: *mut sock) -> bool;
}
extern "C" {
    pub fn tcp_fastopen_active_disable_ofo_check(sk: *mut sock);
}
extern "C" {
    pub fn tcp_fastopen_active_detect_blackhole(sk: *mut sock, expired: bool);
}
// Caller needs to wrap with rcu_read_(un)lock()
// Latencies incurred by various limits for a sender. They are
// chronograph-like stats that are mutually exclusive.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_chrono {
    TCP_CHRONO_UNSPEC,
    TCP_CHRONO_BUSY, /* Actively sending data (non-empty write queue) */
    TCP_CHRONO_RWND_LIMITED, /* Stalled by insufficient receive window */
    TCP_CHRONO_SNDBUF_LIMITED, /* Stalled by insufficient send buffer */
    __TCP_CHRONO_MAX,
}

// Following WRITE_ONCE()s pair with READ_ONCE()s in
// tcp_get_info_chrono_stats().
//
// If there are multiple conditions worthy of tracking in a
// chronograph then the highest priority enum takes precedence
// over the other conditions. So that if something "more interesting"
// starts happening, stop the previous chrono and start a new one.
//
extern "C" {
    pub fn tcp_chrono_stop(sk: *mut sock, type: tcp_chrono);
}
// This helper is needed, because skb->tcp_tsorted_anchor uses
// the same memory storage than skb->destructor/_skb_refdst
//

extern "C" {
    pub fn tcp_write_queue_purge(sk: *mut sock);
}
extern "C" {
    pub fn skb_rb_first(_arg: &sk->tcp_rtx_queue) -> return;
}
extern "C" {
    pub fn skb_rb_last(_arg: &sk->tcp_rtx_queue) -> return;
}
extern "C" {
    pub fn skb_peek_tail(_arg: &sk->sk_write_queue) -> return;
}

extern "C" {
    pub fn skb_peek(_arg: &sk->sk_write_queue) -> return;
}
extern "C" {
    pub fn skb_queue_is_last(_arg: &sk->sk_write_queue, _arg: skb) -> return;
}
//
// tcp_write_queue_empty - test if any payload (or FIN) is available in write queue
// @sk: socket
//
// Since the write queue can have a temporary empty skb in it,
// we must not use "return skb_queue_empty(&sk->sk_write_queue)"
//
extern "C" {
    pub fn RB_EMPTY_ROOT(_arg: &sk->tcp_rtx_queue) -> return;
}
extern "C" {
    pub fn tcp_rtx_queue_empty(tcp_write_queue_empty(sk: sk) &&) -> return;
}
// Queue it, remembering where we must start sending.
// Insert new before skb on the write queue of sk.
extern "C" {
    pub fn tcp_rbtree_insert(root: *mut rb_root, skb: *mut sk_buff);
}
// Start sequence of the skb just after the highest skb with SACKed
// bit, valid only if sacked_out > 0 or when the caller has ensured
// validity by itself.
//
// Called when old skb is about to be deleted and replaced by new skb
// This helper checks if socket has IP_TRANSPARENT set
extern "C" {
    pub fn inet_test_bit(_arg: TRANSPARENT, _arg: sk) -> return;
}
// Determines whether this is a thin stream (which may suffer from
// increased latency). Used to trigger latency-reducing mechanisms.
//
// /proc
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_seq_states {
    TCP_SEQ_STATE_LISTENING,
    TCP_SEQ_STATE_ESTABLISHED,
}

extern "C" {
    pub fn tcp_seq_stop(seq: *mut seq_file, v: *mut c_void);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_seq_afinfo {
    pub family: sa_family_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_iter_state {
    pub p: seq_net_private,
    pub state: tcp_seq_states,
    pub syn_wait_sk: *mut sock,
    pub num: int bucket, offset, sbucket,,
    pub last_pos: loff_t,
}

extern "C" {
    pub fn tcp_v4_destroy_sock(sk: *mut sock);
}

extern "C" {
    pub fn tcp_gro_complete(skb: *mut sk_buff);
}

extern "C" {
    pub fn tcp_stream_memory_free(sk: *const sock, wake: c_int) -> bool;
}

extern "C" {
    pub fn tcp4_proc_init() -> c_int;
}
extern "C" {
    pub fn tcp4_proc_exit();
}

extern "C" {
    pub fn tcp_rtx_synack(sk: *const sock, req: *mut request_sock) -> c_int;
}
// TCP af-specific functions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_sock_af_ops {

    pub addr_sk): *const sock,
    pub skb): *const sk_buff,
    pub optlen): c_int,

    pub optlen): *mut *mut *mut int (ao_parse)(struct sock sk, int optname, sockptr_t optval, int,
    pub rcvid): int sndid, int,
    pub send): __be32 sisn, __be32 disn, bool,
    pub sne): *const *const u8 tkey, int hash_offset, u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_request_sock_ops {
    pub mss_clamp: u16,

    pub addr_sk): *const sock,
    pub skb): *const sk_buff,

    pub rcvid): int sndid, int,
    pub sk): *mut *mut *mut *mut void (ao_calc_key)(struct tcp_ao_key mkt, u8 key, struct request_sock,
    pub sne): int hash_offset, u32,

    pub mss): *mut __u16,

    pub tw_isn): u32,
    pub skb): *const sk_buff,
    pub syn_skb): *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_key {
    pub ao_key: *mut tcp_ao_key,
    pub traffic_key: *mut c_char,
    pub sne: u32,
    pub rcv_next: u8,
}

extern "C" {
    pub fn tcpv4_offload_init() -> c_int;
}
extern "C" {
    pub fn tcp_v4_init();
}
extern "C" {
    pub fn tcp_init();
}
// tcp_recovery.c
extern "C" {
    pub fn tcp_mark_skb_lost(sk: *mut sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn tcp_newreno_mark_lost(sk: *mut sock, snd_una_advanced: bool);
}
extern "C" {
    pub fn tcp_rack_mark_lost(sk: *mut sock) -> bool;
}
extern "C" {
    pub fn tcp_rack_reo_timeout(sk: *mut sock);
}
// tcp_plb.c
//
// Scaling factor for fractions in PLB. For example, tcp_plb_update_state
// expects cong_ratio which represents fraction of traffic that experienced
// congestion over a single RTT. In order to avoid floating point operations,
// this fraction should be mapped to (1 << TCP_PLB_SCALE) and passed in.
//
pub const TCP_PLB_SCALE: c_int = 8;
// State for PLB (Protective Load Balancing) for a single TCP connection.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_plb_state {
    pub /: *mut *mut u32 pause_until; / jiffies32 when PLB can resume rerouting,
}

extern "C" {
    pub fn tcp_plb_check_rehash(sk: *mut sock, plb: *mut tcp_plb_state);
}
extern "C" {
    pub fn tcp_plb_update_state_upon_rto(sk: *mut sock, plb: *mut tcp_plb_state);
}
// At how many usecs into the future should the RTO fire?
extern "C" {
    pub fn jiffies_to_usecs(_arg: rto) -> return;
}
//
// Save and compile IPv4 options, return a pointer to it
//
// locally generated TCP pure ACKs have skb->truesize == 2
// (check tcp_send_ack() in net/ipv4/tcp_output.c )
// This is much faster than dissecting the packet to find out.
// (Think of GRE encapsulations, IPv4, IPv6, ...)
//
// Subtract 1, if FIN was received
extern "C" {
    pub fn tcp_peek_len(sock: *mut socket) -> c_int;
}
// We update these fields while other threads might
// read them from tcp_get_info()
//
// TCP listen path runs lockless.
// We forced "struct sock" to be const qualified to make sure
// we don't modify one of its field by mistake.
// Here, we increment sk_drops which is an atomic_t, so we can safely
// make sock writable again.
//
extern "C" {
    pub fn tcp_pace_kick(timer: *mut hrtimer) -> hrtimer_restart;
}
//
// Interface for adding Upper Level Protocols over TCP
//
pub const TCP_ULP_NAME_MAX: c_int = 16;
pub const TCP_ULP_MAX: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_ulp_ops {
    pub list: list_head,
// initialize ulp
    pub sk): *mut *mut int (init)(struct sock,
// update ulp
    pub sk)): *mut *mut void (write_space)(struct sock,
// cleanup ulp
    pub sk): *mut *mut void (release)(struct sock,
// diagnostic
    pub net_admin): *mut *mut *mut *mut int (get_info)(struct sock sk, struct sk_buff skb, bool,
    pub net_admin): *const *const *const size_t (get_info_size)(struct sock sk, bool,
// clone ulp
    pub priority): gfp_t,
    pub name: [c_char; TCP_ULP_NAME_MAX],
    pub owner: *mut module,
}

extern "C" {
    pub fn tcp_register_ulp(type: *mut tcp_ulp_ops) -> c_int;
}
extern "C" {
    pub fn tcp_unregister_ulp(type: *mut tcp_ulp_ops);
}
extern "C" {
    pub fn tcp_set_ulp(sk: *mut sock, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn tcp_get_available_ulp(buf: *mut c_char, len: usize);
}
extern "C" {
    pub fn tcp_cleanup_ulp(sk: *mut sock);
}

extern "C" {
    pub fn tcp_bpf_update_proto(sk: *mut sock, psock: *mut sk_psock, restore: bool) -> c_int;
}
extern "C" {
    pub fn tcp_bpf_clone(sk: *const sock, newsk: *mut sock);
}

extern "C" {
    pub fn tcp_eat_skb(sk: *mut sock, skb: *mut sk_buff);
}

// Call BPF_SOCK_OPS program that returns an int. If the return value
// is < 0, then the BPF op failed (for example if the loaded BPF
// program does not support the chosen operation or there is no BPF
// program loaded).
//

extern "C" {
    pub fn tcp_call_bpf(_arg: sk, _arg: op, _arg: 2, _arg: args) -> return;
}
extern "C" {
    pub fn tcp_call_bpf(_arg: sk, _arg: op, _arg: 3, _arg: args) -> return;
}

extern "C" {
    pub fn min_t(_arg: c_int, _arg: timeout, _arg: TCP_RTO_MAX) -> return;
}

extern "C" {
    pub fn clean_acked_data_disable(tp: *mut tcp_sock);
}
extern "C" {
    pub fn clean_acked_data_flush();
}

// Compute Earliest Departure Time for some control packets
// like ACK or RST for TIME_WAIT or non ESTABLISHED sockets.
//
// md5_hash = md5_tmp;
// aoh = NULL;
// aoh = (struct tcp_ao_hdr *)(ao_tmp - 2);

