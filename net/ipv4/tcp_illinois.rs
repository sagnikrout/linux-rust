//! Automatically rewritten from C to Rust
//! Source: net/ipv4/tcp_illinois.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// TCP Illinois congestion control.
// Home page:
// http://www.ews.uiuc.edu/~shaoliu/tcpillinois/index.html
//
// The algorithm is described in:
// "TCP-Illinois: A Loss and Delay-Based Congestion Control Algorithm
// for High-Speed Networks"
// http://tamerbasar.csl.illinois.edu/LiuBasarSrikantPerfEvalArtJun2008.pdf
//
// Implemented from description in paper and ns-2 simulation.
// Copyright (C) 2007 Stephen Hemminger <shemminger@linux-foundation.org>
//

pub const ALPHA_SHIFT: c_int = 7;

pub const BETA_SHIFT: c_int = 6;

    let mut __read_mostly: static int win_thresh = 15;
    module_param(win_thresh, int, 0);
    MODULE_PARM_DESC(win_thresh, "Window threshold for starting adaptive sizing");
    let mut __read_mostly: static int theta = 5;
    module_param(theta, int, 0);
    MODULE_PARM_DESC(theta, "# of fast RTT's before full growth");
// TCP Illinois Parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct illinois {
    pub /: *mut *mut u64 sum_rtt; / sum of rtt's measured within last rtt,
    pub /: *mut *mut u16 cnt_rtt; / # of rtts measured within last rtt,
    pub /: *mut *mut u32 base_rtt; / min of all rtt in usec,
    pub /: *mut *mut u32 max_rtt; / max of all rtt in usec,
    pub /: *mut *mut u32 end_seq; / right edge of current RTT,
    pub /: *mut *mut u32 alpha; / Additive increase,
    pub /: *mut *mut u32 beta; / Muliplicative decrease,
    pub /: *mut *mut u16 acked; / # packets acked by current ACK,
    pub /: *mut *mut u8 rtt_above; / average rtt has gone above threshold,
    pub /: *mut *mut u8 rtt_low; / # of rtts measurements below threshold,
}

#[no_mangle]
unsafe extern "C" fn rtt_reset(sk: *mut sock) {
    static void rtt_reset(struct sock *sk)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    struct illinois *ca = inet_csk_ca(sk);
    ca.end_seq = tp.snd_nxt;
    ca.cnt_rtt = 0;
    ca.sum_rtt = 0;
// TODO: age max_rtt?
    }
#[no_mangle]
unsafe extern "C" fn tcp_illinois_init(sk: *mut sock) {
    static void tcp_illinois_init(struct sock *sk)
    {
    struct illinois *ca = inet_csk_ca(sk);
    ca.alpha = ALPHA_MAX;
    ca.beta = BETA_BASE;
    ca.base_rtt = 0x7fffffff;
    ca.max_rtt = 0;
    ca.acked = 0;
    ca.rtt_low = 0;
    ca.rtt_above = 0;
    rtt_reset(sk);
    }
// Measure RTT for each ack.
#[no_mangle]
unsafe extern "C" fn tcp_illinois_acked(sk: *mut sock, sample: *const ack_sample) {
    static void tcp_illinois_acked(struct sock *sk, const struct ack_sample *sample)
    {
    struct illinois *ca = inet_csk_ca(sk);
    let mut rtt_us: i32 = sample.rtt_us;
    ca.acked = sample.pkts_acked;
// dup ack, no rtt sample
    if (rtt_us < 0)
    return;
// ignore bogus values, this prevents wraparound in alpha math
    if (rtt_us > RTT_MAX)
    rtt_us = RTT_MAX;
// keep track of minimum RTT seen so far
    if (ca.base_rtt > rtt_us)
    ca.base_rtt = rtt_us;
// and max
    if (ca.max_rtt < rtt_us)
    ca.max_rtt = rtt_us;
    ++ca.cnt_rtt;
    ca.sum_rtt += rtt_us;
    }
// Maximum queuing delay
#[no_mangle]
pub unsafe extern "C" fn max_delay(ca: *const illinois) -> u32 {
    static inline u32 max_delay(const struct illinois *ca)
    {
    return ca.max_rtt - ca.base_rtt;
    }
// Average queuing delay
#[no_mangle]
pub unsafe extern "C" fn avg_delay(ca: *const illinois) -> u32 {
    static inline u32 avg_delay(const struct illinois *ca)
    {
    let mut t: u64 = ca.sum_rtt;
    do_div(t, ca.cnt_rtt);
    return t - ca.base_rtt;
    }
//
// Compute value of alpha used for additive increase.
// If small window then use 1.0, equivalent to Reno.
//
// For larger windows, adjust based on average delay.
// A. If average delay is at minimum (we are uncongested),
// then use large alpha (10.0) to increase faster.
// B. If average delay is at maximum (getting congested)
// then use small alpha (0.3)
//
// The result is a convex window growth curve.
//
#[no_mangle]
unsafe extern "C" fn alpha(ca: *mut illinois, da: u32, dm: u32) -> u32 {
    static u32 alpha(struct illinois *ca, u32 da, u32 dm)
    {
    u32 d1 = dm / 100;	/* Low threshold */
    if (da <= d1) {
// If never got out of low delay zone, then use max
    if (!ca.rtt_above)
    return ALPHA_MAX;
// Wait for 5 good RTT's before allowing alpha to go alpha max.
// This prevents one good RTT from causing sudden window increase.
//
    if (++ca.rtt_low < theta)
    return ca.alpha;
    ca.rtt_low = 0;
    ca.rtt_above = 0;
    return ALPHA_MAX;
    }
    ca.rtt_above = 1;
//
// Based on:
//
// (dm - d1) amin amax
// k1 = -------------------
// amax - amin
//
// (dm - d1) amin
// k2 = ----------------  - d1
// amax - amin
//
// k1
// alpha = ----------
// k2 + da
//
    dm -= d1;
    da -= d1;
    return (dm * ALPHA_MAX) /
    (dm + (da  * (ALPHA_MAX - ALPHA_MIN)) / ALPHA_MIN);
    }
//
// Beta used for multiplicative decrease.
// For small window sizes returns same value as Reno (0.5)
//
// If delay is small (10% of max) then beta = 1/8
// If delay is up to 80% of max then beta = 1/2
// In between is a linear function
//
#[no_mangle]
unsafe extern "C" fn beta(da: u32, dm: u32) -> u32 {
    static u32 beta(u32 da, u32 dm)
    {
    u32 d2, d3;
    d2 = dm / 10;
    if (da <= d2)
    return BETA_MIN;
    d3 = (8 * dm) / 10;
    if (da >= d3 || d3 <= d2)
    return BETA_MAX;
//
// Based on:
//
// bmin d3 - bmax d2
// k3 = -------------------
// d3 - d2
//
// bmax - bmin
// k4 = -------------
// d3 - d2
//
// b = k3 + k4 da
//
    return (BETA_MIN * d3 - BETA_MAX * d2 + (BETA_MAX - BETA_MIN) * da)
    / (d3 - d2);
    }
// Update alpha and beta values once per RTT
#[no_mangle]
unsafe extern "C" fn update_params(sk: *mut sock) {
    static void update_params(struct sock *sk)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    struct illinois *ca = inet_csk_ca(sk);
    if (tcp_snd_cwnd(tp) < win_thresh) {
    ca.alpha = ALPHA_BASE;
    ca.beta = BETA_BASE;
    } else if (ca.cnt_rtt > 0) {
    let mut dm: u32 = max_delay(ca);
    let mut da: u32 = avg_delay(ca);
    ca.alpha = alpha(ca, da, dm);
    ca.beta = beta(da, dm);
    }
    rtt_reset(sk);
    }
//
// In case of loss, reset to default values
//
#[no_mangle]
unsafe extern "C" fn tcp_illinois_state(sk: *mut sock, new_state: u8) {
    static void tcp_illinois_state(struct sock *sk, u8 new_state)
    {
    struct illinois *ca = inet_csk_ca(sk);
    if (new_state == TCP_CA_Loss) {
    ca.alpha = ALPHA_BASE;
    ca.beta = BETA_BASE;
    ca.rtt_low = 0;
    ca.rtt_above = 0;
    rtt_reset(sk);
    }
    }
//
// Increase window in response to successful acknowledgment.
//
#[no_mangle]
unsafe extern "C" fn tcp_illinois_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    static void tcp_illinois_cong_avoid(struct sock *sk, u32 ack, u32 acked)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    struct illinois *ca = inet_csk_ca(sk);
    if (after(ack, ca.end_seq))
    update_params(sk);
// RFC2861 only increase cwnd if fully utilized
    if (!tcp_is_cwnd_limited(sk))
    return;
// In slow start
    if (tcp_in_slow_start(tp))
    tcp_slow_start(tp, acked);
    else {
    u32 delta;
// snd_cwnd_cnt is # of packets since last cwnd increment
    tp.snd_cwnd_cnt += ca.acked;
    ca.acked = 1;
// This is close approximation of:
// tp->snd_cwnd += alpha/tp->snd_cwnd
//
    delta = (tp.snd_cwnd_cnt * ca.alpha) >> ALPHA_SHIFT;
    if (delta >= tcp_snd_cwnd(tp)) {
    tcp_snd_cwnd_set(tp, min(tcp_snd_cwnd(tp) + delta / tcp_snd_cwnd(tp),
    (u32)tp.snd_cwnd_clamp));
    tp.snd_cwnd_cnt = 0;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn tcp_illinois_ssthresh(sk: *mut sock) -> u32 {
    static u32 tcp_illinois_ssthresh(struct sock *sk)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    struct illinois *ca = inet_csk_ca(sk);
    u32 decr;
// Multiplicative decrease
    decr = (tcp_snd_cwnd(tp) * ca.beta) >> BETA_SHIFT;
    return max(tcp_snd_cwnd(tp) - decr, 2U);
    }
// Extract info for Tcp socket info provided via netlink.
    static size_t tcp_illinois_info(struct sock *sk, u32 ext, int *attr,
    union tcp_cc_info *info)
    {
    const struct illinois *ca = inet_csk_ca(sk);
    if (ext & (1 << (INET_DIAG_VEGASINFO - 1))) {
    info.vegas.tcpv_enabled = 1;
    info.vegas.tcpv_rttcnt = ca.cnt_rtt;
    info.vegas.tcpv_minrtt = ca.base_rtt;
    info.vegas.tcpv_rtt = 0;
    if (info.vegas.tcpv_rttcnt > 0) {
    let mut t: u64 = ca.sum_rtt;
    do_div(t, info.vegas.tcpv_rttcnt);
    info.vegas.tcpv_rtt = t;
    }
// attr = INET_DIAG_VEGASINFO;
    return sizeof(struct tcpvegas_info);
    }
    return 0;
    }
    static struct tcp_congestion_ops tcp_illinois __read_mostly = {
    .init		= tcp_illinois_init,
    .ssthresh	= tcp_illinois_ssthresh,
    .undo_cwnd	= tcp_reno_undo_cwnd,
    .cong_avoid	= tcp_illinois_cong_avoid,
    .set_state	= tcp_illinois_state,
    .get_info	= tcp_illinois_info,
    .pkts_acked	= tcp_illinois_acked,
    .owner		= THIS_MODULE,
    .name		= "illinois",
    };
#[no_mangle]
unsafe extern "C" fn tcp_illinois_register() -> int __init {
    static int __init tcp_illinois_register(void)
    {
    BUILD_BUG_ON(sizeof(struct illinois) > ICSK_CA_PRIV_SIZE);
    return tcp_register_congestion_control(&tcp_illinois);
    }
#[no_mangle]
unsafe extern "C" fn tcp_illinois_unregister() -> void __exit {
    static void __exit tcp_illinois_unregister(void)
    {
    tcp_unregister_congestion_control(&tcp_illinois);
    }
    module_init(tcp_illinois_register);
    module_exit(tcp_illinois_unregister);
    MODULE_AUTHOR("Stephen Hemminger, Shao Liu");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("TCP Illinois");
    MODULE_VERSION("1.0");
