//! Automatically rewritten from C to Rust
//! Source: net/ipv4/tcp_bic.c
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
// Binary Increase Congestion control for TCP
// Home page:
// http://netsrv.csc.ncsu.edu/twiki/bin/view/Main/BIC
// This is from the implementation of BICTCP in
// Lison-Xu, Kahaled Harfoush, and Injong Rhee.
// "Binary Increase Congestion Control for Fast, Long Distance
// Networks" in InfoComm 2004
// Available from:
// http://netsrv.csc.ncsu.edu/export/bitcp.pdf
//
// Unless BIC is enabled and congestion window is large
// this behaves the same as the original Reno.
//

// max_cwnd = snd_cwnd * beta
//

// In binary search,
// go to point (max+min)/N
//
    let mut fast_convergence: static int = 1;
    let mut max_increment: static int = 16;
    let mut low_window: static int = 14;
    static int beta = 819;		/* = 819/1024 (BICTCP_BETA_SCALE) */
    static int initial_ssthresh;
    let mut smooth_part: static int = 20;
    module_param(fast_convergence, int, 0644);
    MODULE_PARM_DESC(fast_convergence, "turn on/off fast convergence");
    module_param(max_increment, int, 0644);
    MODULE_PARM_DESC(max_increment, "Limit on increment allowed during binary search");
    module_param(low_window, int, 0644);
    MODULE_PARM_DESC(low_window, "lower bound on congestion window (for TCP friendliness)");
    module_param(beta, int, 0644);
    MODULE_PARM_DESC(beta, "beta for multiplicative increase");
    module_param(initial_ssthresh, int, 0644);
    MODULE_PARM_DESC(initial_ssthresh, "initial value of slow start threshold");
    module_param(smooth_part, int, 0644);
    MODULE_PARM_DESC(smooth_part, "log(B/(B*Smin))/log(B/(B-1))+B, # of RTT from Wmax-B to Wmax");
// BIC TCP Parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bictcp {
    pub /: *mut *mut u32 cnt; / increase cwnd by 1 after ACKs,
    pub /: *mut *mut u32 last_max_cwnd; / last maximum snd_cwnd,
    pub /: *mut *mut u32 last_cwnd; / the last snd_cwnd,
    pub /: *mut *mut u32 last_time; / time when updated last_cwnd,
    pub /: *mut *mut u32 epoch_start; / beginning of an epoch,
pub const ACK_RATIO_SHIFT: c_int = 4;
    pub /: *mut *mut u32 delayed_ack; / estimate the ratio of Packets/ACKs << 4,
}

#[no_mangle]
pub unsafe extern "C" fn bictcp_reset(ca: *mut bictcp) {
    static inline void bictcp_reset(struct bictcp *ca)
    {
    ca.cnt = 0;
    ca.last_max_cwnd = 0;
    ca.last_cwnd = 0;
    ca.last_time = 0;
    ca.epoch_start = 0;
    ca.delayed_ack = 2 << ACK_RATIO_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn bictcp_init(sk: *mut sock) {
    static void bictcp_init(struct sock *sk)
    {
    struct bictcp *ca = inet_csk_ca(sk);
    bictcp_reset(ca);
    if (initial_ssthresh)
    WRITE_ONCE(tcp_sk(sk).snd_ssthresh, initial_ssthresh);
    }
//
// Compute congestion window to use.
//
#[no_mangle]
pub unsafe extern "C" fn bictcp_update(ca: *mut bictcp, cwnd: u32) {
    static inline void bictcp_update(struct bictcp *ca, u32 cwnd)
    {
    if (ca.last_cwnd == cwnd &&
    (s32)(tcp_jiffies32 - ca.last_time) <= HZ / 32)
    return;
    ca.last_cwnd = cwnd;
    ca.last_time = tcp_jiffies32;
    if (ca.epoch_start == 0) /* record the beginning of an epoch */
    ca.epoch_start = tcp_jiffies32;
// start off normal
    if (cwnd <= low_window) {
    ca.cnt = cwnd;
    return;
    }
// binary increase
    if (cwnd < ca.last_max_cwnd) {
    __u32	dist = (ca.last_max_cwnd - cwnd)
    / BICTCP_B;
    if (dist > max_increment)
// linear increase
    ca.cnt = cwnd / max_increment;
#[no_mangle]
pub unsafe extern "C" fn if(1U: dist <=) -> else {
    else if (dist <= 1U)
// binary search increase
    ca.cnt = (cwnd * smooth_part) / BICTCP_B;
    else
// binary search increase
    ca.cnt = cwnd / dist;
    } else {
// slow start AMD linear increase
    if (cwnd < ca.last_max_cwnd + BICTCP_B)
// slow start
    ca.cnt = (cwnd * smooth_part) / BICTCP_B;
#[no_mangle]
pub unsafe extern "C" fn if(max_increment*(BICTCP_B-1): *mut cwnd < ca->last_max_cwnd +) -> else {
    else if (cwnd < ca.last_max_cwnd + max_increment*(BICTCP_B-1))
// slow start
    ca.cnt = (cwnd * (BICTCP_B-1))
    / (cwnd - ca.last_max_cwnd);
    else
// linear increase
    ca.cnt = cwnd / max_increment;
    }
// if in slow start or link utilization is very low
    if (ca.last_max_cwnd == 0) {
    if (ca.cnt > 20) /* increase cwnd 5% per RTT */
    ca.cnt = 20;
    }
    ca.cnt = (ca.cnt << ACK_RATIO_SHIFT) / ca.delayed_ack;
    if (ca.cnt == 0)			/* cannot be zero */
    ca.cnt = 1;
    }
#[no_mangle]
unsafe extern "C" fn bictcp_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    static void bictcp_cong_avoid(struct sock *sk, u32 ack, u32 acked)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    struct bictcp *ca = inet_csk_ca(sk);
    if (!tcp_is_cwnd_limited(sk))
    return;
    if (tcp_in_slow_start(tp)) {
    acked = tcp_slow_start(tp, acked);
    if (!acked)
    return;
    }
    bictcp_update(ca, tcp_snd_cwnd(tp));
    tcp_cong_avoid_ai(tp, ca.cnt, acked);
    }
//
// behave like Reno until low_window is reached,
// then increase congestion window slowly
//
#[no_mangle]
unsafe extern "C" fn bictcp_recalc_ssthresh(sk: *mut sock) -> u32 {
    static u32 bictcp_recalc_ssthresh(struct sock *sk)
    {
    const struct tcp_sock *tp = tcp_sk(sk);
    struct bictcp *ca = inet_csk_ca(sk);
    ca.epoch_start = 0;	/* end of epoch */
// Wmax and fast convergence
    if (tcp_snd_cwnd(tp) < ca.last_max_cwnd && fast_convergence)
    ca.last_max_cwnd = (tcp_snd_cwnd(tp) * (BICTCP_BETA_SCALE + beta))
    / (2 * BICTCP_BETA_SCALE);
    else
    ca.last_max_cwnd = tcp_snd_cwnd(tp);
    if (tcp_snd_cwnd(tp) <= low_window)
    return max(tcp_snd_cwnd(tp) >> 1U, 2U);
    else
    return max((tcp_snd_cwnd(tp) * beta) / BICTCP_BETA_SCALE, 2U);
    }
#[no_mangle]
unsafe extern "C" fn bictcp_state(sk: *mut sock, new_state: u8) {
    static void bictcp_state(struct sock *sk, u8 new_state)
    {
    if (new_state == TCP_CA_Loss)
    bictcp_reset(inet_csk_ca(sk));
    }
// Track delayed acknowledgment ratio using sliding window
// ratio = (15*ratio + sample) / 16
//
#[no_mangle]
unsafe extern "C" fn bictcp_acked(sk: *mut sock, sample: *const ack_sample) {
    static void bictcp_acked(struct sock *sk, const struct ack_sample *sample)
    {
    const struct inet_connection_sock *icsk = inet_csk(sk);
    if (icsk.icsk_ca_state == TCP_CA_Open) {
    struct bictcp *ca = inet_csk_ca(sk);
    ca.delayed_ack += sample.pkts_acked -
    (ca.delayed_ack >> ACK_RATIO_SHIFT);
    }
    }
    static struct tcp_congestion_ops bictcp __read_mostly = {
    .init		= bictcp_init,
    .ssthresh	= bictcp_recalc_ssthresh,
    .cong_avoid	= bictcp_cong_avoid,
    .set_state	= bictcp_state,
    .undo_cwnd	= tcp_reno_undo_cwnd,
    .pkts_acked     = bictcp_acked,
    .owner		= THIS_MODULE,
    .name		= "bic",
    };
#[no_mangle]
unsafe extern "C" fn bictcp_register() -> int __init {
    static int __init bictcp_register(void)
    {
    BUILD_BUG_ON(sizeof(struct bictcp) > ICSK_CA_PRIV_SIZE);
    return tcp_register_congestion_control(&bictcp);
    }
#[no_mangle]
unsafe extern "C" fn bictcp_unregister() -> void __exit {
    static void __exit bictcp_unregister(void)
    {
    tcp_unregister_congestion_control(&bictcp);
    }
    module_init(bictcp_register);
    module_exit(bictcp_unregister);
    MODULE_AUTHOR("Stephen Hemminger");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("BIC TCP");
