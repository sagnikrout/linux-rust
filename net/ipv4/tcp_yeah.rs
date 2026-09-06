//! Automatically rewritten from C to Rust
//! Source: net/ipv4/tcp_yeah.c
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
// YeAH TCP
//
// For further details look at:
// https://web.archive.org/web/20080316215752/http://wil.cs.caltech.edu/pfldnet2007/paper/YeAH_TCP.pdf
//

// YeAH variables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct yeah {
    pub /: *mut *mut vegas vegas; / must be first,
// YeAH
    pub lastQ: u32,
    pub doing_reno_now: u32,
    pub reno_count: u32,
    pub fast_count: u32,
}

#[no_mangle]
unsafe extern "C" fn tcp_yeah_init(sk: *mut sock) {
    static void tcp_yeah_init(struct sock *sk)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    struct yeah *yeah = inet_csk_ca(sk);
    tcp_vegas_init(sk);
    yeah.doing_reno_now = 0;
    yeah.lastQ = 0;
    yeah.reno_count = 2;
// Ensure the MD arithmetic works.  This is somewhat pedantic,
// since I don't think we will see a cwnd this large. :)
    tp.snd_cwnd_clamp = min_t(u32, tp.snd_cwnd_clamp, 0xffffffff/128);
    }
#[no_mangle]
unsafe extern "C" fn tcp_yeah_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    static void tcp_yeah_cong_avoid(struct sock *sk, u32 ack, u32 acked)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    struct yeah *yeah = inet_csk_ca(sk);
    if (!tcp_is_cwnd_limited(sk))
    return;
    if (tcp_in_slow_start(tp)) {
    acked = tcp_slow_start(tp, acked);
    if (!acked)
    goto do_vegas;
    }
    if (!yeah.doing_reno_now) {
// Scalable
    tcp_cong_avoid_ai(tp, min(tcp_snd_cwnd(tp), TCP_SCALABLE_AI_CNT),
    acked);
    } else {
// Reno
    tcp_cong_avoid_ai(tp, tcp_snd_cwnd(tp), acked);
    }
// The key players are v_vegas.beg_snd_una and v_beg_snd_nxt.
//
// These are so named because they represent the approximate values
// of snd_una and snd_nxt at the beginning of the current RTT. More
// precisely, they represent the amount of data sent during the RTT.
// At the end of the RTT, when we receive an ACK for v_beg_snd_nxt,
// we will calculate that (v_beg_snd_nxt - v_vegas.beg_snd_una) outstanding
// bytes of data have been ACKed during the course of the RTT, giving
// an "actual" rate of:
//
// (v_beg_snd_nxt - v_vegas.beg_snd_una) / (rtt duration)
//
// Unfortunately, v_vegas.beg_snd_una is not exactly equal to snd_una,
// because delayed ACKs can cover more than one segment, so they
// don't line up yeahly with the boundaries of RTTs.
//
// Another unfortunate fact of life is that delayed ACKs delay the
// advance of the left edge of our send window, so that the number
// of bytes we send in an RTT is often less than our cwnd will allow.
// So we keep track of our cwnd separately, in v_beg_snd_cwnd.
//
    do_vegas:
    if (after(ack, yeah.vegas.beg_snd_nxt)) {
// We do the Vegas calculations only if we got enough RTT
// samples that we can be reasonably sure that we got
// at least one RTT sample that wasn't from a delayed ACK.
// If we only had 2 samples total,
// then that means we're getting only 1 ACK per RTT, which
// means they're almost certainly delayed ACKs.
// If  we have 3 samples, we should be OK.
//
    if (yeah.vegas.cntRTT > 2) {
    u32 rtt, queue;
    u64 bw;
// We have enough RTT samples, so, using the Vegas
// algorithm, we determine if we should increase or
// decrease cwnd, and by how much.
//
// Pluck out the RTT we are using for the Vegas
// calculations. This is the min RTT seen during the
// last RTT. Taking the min filters out the effects
// of delayed ACKs, at the cost of noticing congestion
// a bit later.
//
    rtt = yeah.vegas.minRTT;
// Compute excess number of packets above bandwidth
// Avoid doing full 64 bit divide.
//
    bw = tcp_snd_cwnd(tp);
    bw *= rtt - yeah.vegas.baseRTT;
    do_div(bw, rtt);
    queue = bw;
    if (queue > TCP_YEAH_ALPHA ||
    rtt - yeah.vegas.baseRTT > (yeah.vegas.baseRTT / TCP_YEAH_PHY)) {
    if (queue > TCP_YEAH_ALPHA &&
    tcp_snd_cwnd(tp) > yeah.reno_count) {
    u32 reduction = min(queue / TCP_YEAH_GAMMA ,
    tcp_snd_cwnd(tp) >> TCP_YEAH_EPSILON);
    tcp_snd_cwnd_set(tp, tcp_snd_cwnd(tp) - reduction);
    tcp_snd_cwnd_set(tp, max(tcp_snd_cwnd(tp),
    yeah.reno_count));
    WRITE_ONCE(tp.snd_ssthresh,
    tcp_snd_cwnd(tp));
    }
    if (yeah.reno_count <= 2)
    yeah.reno_count = max(tcp_snd_cwnd(tp)>>1, 2U);
    else
    yeah.reno_count++;
    yeah.doing_reno_now = min(yeah.doing_reno_now + 1,
    0xffffffU);
    } else {
    yeah.fast_count++;
    if (yeah.fast_count > TCP_YEAH_ZETA) {
    yeah.reno_count = 2;
    yeah.fast_count = 0;
    }
    yeah.doing_reno_now = 0;
    }
    yeah.lastQ = queue;
    }
// Save the extent of the current window so we can use this
// at the end of the next RTT.
//
    yeah.vegas.beg_snd_una  = yeah.vegas.beg_snd_nxt;
    yeah.vegas.beg_snd_nxt  = tp.snd_nxt;
    yeah.vegas.beg_snd_cwnd = tcp_snd_cwnd(tp);
// Wipe the slate clean for the next RTT.
    yeah.vegas.cntRTT = 0;
    yeah.vegas.minRTT = 0x7fffffff;
    }
    }
#[no_mangle]
unsafe extern "C" fn tcp_yeah_ssthresh(sk: *mut sock) -> u32 {
    static u32 tcp_yeah_ssthresh(struct sock *sk)
    {
    const struct tcp_sock *tp = tcp_sk(sk);
    struct yeah *yeah = inet_csk_ca(sk);
    u32 reduction;
    if (yeah.doing_reno_now < TCP_YEAH_RHO) {
    reduction = yeah.lastQ;
    reduction = min(reduction, max(tcp_snd_cwnd(tp)>>1, 2U));
    reduction = max(reduction, tcp_snd_cwnd(tp) >> TCP_YEAH_DELTA);
    } else
    reduction = max(tcp_snd_cwnd(tp)>>1, 2U);
    yeah.fast_count = 0;
    yeah.reno_count = max(yeah.reno_count>>1, 2U);
    return max_t(int, tcp_snd_cwnd(tp) - reduction, 2);
    }
    static struct tcp_congestion_ops tcp_yeah __read_mostly = {
    .init		= tcp_yeah_init,
    .ssthresh	= tcp_yeah_ssthresh,
    .undo_cwnd      = tcp_reno_undo_cwnd,
    .cong_avoid	= tcp_yeah_cong_avoid,
    .set_state	= tcp_vegas_state,
    .cwnd_event	= tcp_vegas_cwnd_event,
    .cwnd_event_tx_start = tcp_vegas_cwnd_event_tx_start,
    .get_info	= tcp_vegas_get_info,
    .pkts_acked	= tcp_vegas_pkts_acked,
    .owner		= THIS_MODULE,
    .name		= "yeah",
    };
#[no_mangle]
unsafe extern "C" fn tcp_yeah_register() -> int __init {
    static int __init tcp_yeah_register(void)
    {
    BUILD_BUG_ON(sizeof(struct yeah) > ICSK_CA_PRIV_SIZE);
    tcp_register_congestion_control(&tcp_yeah);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tcp_yeah_unregister() -> void __exit {
    static void __exit tcp_yeah_unregister(void)
    {
    tcp_unregister_congestion_control(&tcp_yeah);
    }
    module_init(tcp_yeah_register);
    module_exit(tcp_yeah_unregister);
    MODULE_AUTHOR("Angelo P. Castellani");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("YeAH TCP");
