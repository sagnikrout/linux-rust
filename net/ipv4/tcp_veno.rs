//! Automatically rewritten from C to Rust
//! Source: net/ipv4/tcp_veno.c
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
// TCP Veno congestion control
//
// This is based on the congestion detection/avoidance scheme described in
// C. P. Fu, S. C. Liew.
// "TCP Veno: TCP Enhancement for Transmission over Wireless Access Networks."
// IEEE Journal on Selected Areas in Communication,
// Feb. 2003.
// See https://www.ie.cuhk.edu.hk/fileadmin/staff_upload/soung/Journal/J3.pdf
//

// Default values of the Veno variables, in fixed-point representation
// with V_PARAM_SHIFT bits to the right of the binary point.
//
pub const V_PARAM_SHIFT: c_int = 1;
    let mut beta: static int = 3 << V_PARAM_SHIFT;
// Veno variables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct veno {
    pub /: *mut *mut u8 doing_veno_now; / if true, do veno for this rtt,
    pub /: *mut *mut u16 cntrtt; / # of rtts measured within last rtt,
    pub /: *mut *mut u32 minrtt; / min of rtts measured within last rtt (in usec),
    pub /: *mut *mut u32 basertt; / the min of all Veno rtt measurements seen (in usec),
    pub /: *mut *mut u32 inc; / decide whether to increase cwnd,
    pub /: *mut *mut u32 diff; / calculate the diff rate,
}

// There are several situations when we must "re-start" Veno:
//
// o when a connection is established
// o after an RTO
// o after fast recovery
// o when we send a packet and there is no outstanding
// unacknowledged data (restarting an idle connection)
//
#[no_mangle]
pub unsafe extern "C" fn veno_enable(sk: *mut sock) {
    static inline void veno_enable(struct sock *sk)
    {
    struct veno *veno = inet_csk_ca(sk);
// turn on Veno
    veno.doing_veno_now = 1;
    veno.minrtt = 0x7fffffff;
    }
#[no_mangle]
pub unsafe extern "C" fn veno_disable(sk: *mut sock) {
    static inline void veno_disable(struct sock *sk)
    {
    struct veno *veno = inet_csk_ca(sk);
// turn off Veno
    veno.doing_veno_now = 0;
    }
#[no_mangle]
unsafe extern "C" fn tcp_veno_init(sk: *mut sock) {
    static void tcp_veno_init(struct sock *sk)
    {
    struct veno *veno = inet_csk_ca(sk);
    veno.basertt = 0x7fffffff;
    veno.inc = 1;
    veno_enable(sk);
    }
// Do rtt sampling needed for Veno.
    static void tcp_veno_pkts_acked(struct sock *sk,
    const struct ack_sample *sample)
    {
    struct veno *veno = inet_csk_ca(sk);
    u32 vrtt;
    if (sample.rtt_us < 0)
    return;
// Never allow zero rtt or baseRTT
    vrtt = sample.rtt_us + 1;
// Filter to find propagation delay:
    if (vrtt < veno.basertt)
    veno.basertt = vrtt;
// Find the min rtt during the last rtt to find
// the current prop. delay + queuing delay:
//
    veno.minrtt = min(veno.minrtt, vrtt);
    veno.cntrtt++;
    }
#[no_mangle]
unsafe extern "C" fn tcp_veno_state(sk: *mut sock, ca_state: u8) {
    static void tcp_veno_state(struct sock *sk, u8 ca_state)
    {
    if (ca_state == TCP_CA_Open)
    veno_enable(sk);
    else
    veno_disable(sk);
    }
//
// If the connection is idle and we are restarting,
// then we don't want to do any Veno calculations
// until we get fresh rtt samples.  So when we
// restart, we reset our Veno state to a clean
// state. After we get acks for this flight of
// packets, _then_ we can make Veno calculations
// again.
//
#[no_mangle]
unsafe extern "C" fn tcp_veno_cwnd_event(sk: *mut sock, event: enum tcp_ca_event) {
    static void tcp_veno_cwnd_event(struct sock *sk, enum tcp_ca_event event)
    {
    if (event == CA_EVENT_CWND_RESTART)
    tcp_veno_init(sk);
    }
#[no_mangle]
unsafe extern "C" fn tcp_veno_cwnd_event_tx_start(sk: *mut sock) {
    static void tcp_veno_cwnd_event_tx_start(struct sock *sk)
    {
    tcp_veno_init(sk);
    }
#[no_mangle]
unsafe extern "C" fn tcp_veno_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    static void tcp_veno_cong_avoid(struct sock *sk, u32 ack, u32 acked)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    struct veno *veno = inet_csk_ca(sk);
    if (!veno.doing_veno_now) {
    tcp_reno_cong_avoid(sk, ack, acked);
    return;
    }
// limited by applications
    if (!tcp_is_cwnd_limited(sk))
    return;
// We do the Veno calculations only if we got enough rtt samples
    if (veno.cntrtt <= 2) {
// We don't have enough rtt samples to do the Veno
// calculation, so we'll behave like Reno.
//
    tcp_reno_cong_avoid(sk, ack, acked);
    } else {
    u64 target_cwnd;
    u32 rtt;
// We have enough rtt samples, so, using the Veno
// algorithm, we determine the state of the network.
//
    rtt = veno.minrtt;
    target_cwnd = (u64)tcp_snd_cwnd(tp) * veno.basertt;
    target_cwnd <<= V_PARAM_SHIFT;
    do_div(target_cwnd, rtt);
    veno.diff = (tcp_snd_cwnd(tp) << V_PARAM_SHIFT) - target_cwnd;
    if (tcp_in_slow_start(tp)) {
// Slow start.
    acked = tcp_slow_start(tp, acked);
    if (!acked)
    goto done;
    }
// Congestion avoidance.
    if (veno.diff < beta) {
// In the "non-congestive state", increase cwnd
// every rtt.
//
    tcp_cong_avoid_ai(tp, tcp_snd_cwnd(tp), acked);
    } else {
// In the "congestive state", increase cwnd
// every other rtt.
//
    if (tp.snd_cwnd_cnt >= tcp_snd_cwnd(tp)) {
    if (veno.inc &&
    tcp_snd_cwnd(tp) < tp.snd_cwnd_clamp) {
    tcp_snd_cwnd_set(tp, tcp_snd_cwnd(tp) + 1);
    veno.inc = 0;
    } else
    veno.inc = 1;
    tp.snd_cwnd_cnt = 0;
    } else
    tp.snd_cwnd_cnt += acked;
    }
    done:
    if (tcp_snd_cwnd(tp) < 2)
    tcp_snd_cwnd_set(tp, 2);
#[no_mangle]
pub unsafe extern "C" fn if(tp->snd_cwnd_clamp: tcp_snd_cwnd(tp) >) -> else {
    else if (tcp_snd_cwnd(tp) > tp.snd_cwnd_clamp)
    tcp_snd_cwnd_set(tp, tp.snd_cwnd_clamp);
    }
// Wipe the slate clean for the next rtt.
// veno->cntrtt = 0;
    veno.minrtt = 0x7fffffff;
    }
// Veno MD phase
#[no_mangle]
unsafe extern "C" fn tcp_veno_ssthresh(sk: *mut sock) -> u32 {
    static u32 tcp_veno_ssthresh(struct sock *sk)
    {
    const struct tcp_sock *tp = tcp_sk(sk);
    struct veno *veno = inet_csk_ca(sk);
    if (veno.diff < beta)
// in "non-congestive state", cut cwnd by 1/5
    return max(tcp_snd_cwnd(tp) * 4 / 5, 2U);
    else
// in "congestive state", cut cwnd by 1/2
    return max(tcp_snd_cwnd(tp) >> 1U, 2U);
    }
    static struct tcp_congestion_ops tcp_veno __read_mostly = {
    .init		= tcp_veno_init,
    .ssthresh	= tcp_veno_ssthresh,
    .undo_cwnd	= tcp_reno_undo_cwnd,
    .cong_avoid	= tcp_veno_cong_avoid,
    .pkts_acked	= tcp_veno_pkts_acked,
    .set_state	= tcp_veno_state,
    .cwnd_event	= tcp_veno_cwnd_event,
    .cwnd_event_tx_start = tcp_veno_cwnd_event_tx_start,
    .owner		= THIS_MODULE,
    .name		= "veno",
    };
#[no_mangle]
unsafe extern "C" fn tcp_veno_register() -> int __init {
    static int __init tcp_veno_register(void)
    {
    BUILD_BUG_ON(sizeof(struct veno) > ICSK_CA_PRIV_SIZE);
    tcp_register_congestion_control(&tcp_veno);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tcp_veno_unregister() -> void __exit {
    static void __exit tcp_veno_unregister(void)
    {
    tcp_unregister_congestion_control(&tcp_veno);
    }
    module_init(tcp_veno_register);
    module_exit(tcp_veno_unregister);
    MODULE_AUTHOR("Bin Zhou, Cheng Peng Fu");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("TCP Veno");
