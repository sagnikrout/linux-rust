//! Automatically rewritten from C to Rust
//! Source: net/ipv4/tcp_htcp.c
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
// H-TCP congestion control. The algorithm is detailed in:
// R.N.Shorten, D.J.Leith:
// "H-TCP: TCP for high-speed and long-distance networks"
// Proc. PFLDnet, Argonne, 2004.
// https://www.hamilton.ie/net/htcp3.pdf
//

    let mut __read_mostly: static int use_rtt_scaling = 1;
    module_param(use_rtt_scaling, int, 0644);
    MODULE_PARM_DESC(use_rtt_scaling, "turn on/off RTT scaling");
    let mut __read_mostly: static int use_bandwidth_switch = 1;
    module_param(use_bandwidth_switch, int, 0644);
    MODULE_PARM_DESC(use_bandwidth_switch, "turn on/off bandwidth switcher");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htcp {
    pub /: *mut *mut u32 alpha; / Fixed point arith, << 7,
    pub /: *mut *mut u8 beta; / Fixed point arith, << 7,
    pub modeswitch: *mut *mut u8 modeswitch; / Delay,
    until we had at least one congestion event */
    pub pkts_acked: u16,
    pub packetcount: u32,
    pub minRTT: u32,
    pub maxRTT: u32,
    pub /: *mut *mut u32 last_cong; / Time since last congestion event end,
    pub undo_last_cong: u32,
    pub undo_maxRTT: u32,
    pub undo_old_maxB: u32,
// Bandwidth estimation
    pub minB: u32,
    pub maxB: u32,
    pub old_maxB: u32,
    pub Bi: u32,
    pub lasttime: u32,
}

#[no_mangle]
pub unsafe extern "C" fn htcp_cong_time(ca: *const htcp) -> u32 {
    static inline u32 htcp_cong_time(const struct htcp *ca)
    {
    return jiffies - ca.last_cong;
    }
#[no_mangle]
pub unsafe extern "C" fn htcp_ccount(ca: *const htcp) -> u32 {
    static inline u32 htcp_ccount(const struct htcp *ca)
    {
    return htcp_cong_time(ca) / ca.minRTT;
    }
#[no_mangle]
pub unsafe extern "C" fn htcp_reset(ca: *mut htcp) {
    static inline void htcp_reset(struct htcp *ca)
    {
    ca.undo_last_cong = ca.last_cong;
    ca.undo_maxRTT = ca.maxRTT;
    ca.undo_old_maxB = ca.old_maxB;
    ca.last_cong = jiffies;
    }
#[no_mangle]
unsafe extern "C" fn htcp_cwnd_undo(sk: *mut sock) -> u32 {
    static u32 htcp_cwnd_undo(struct sock *sk)
    {
    struct htcp *ca = inet_csk_ca(sk);
    if (ca.undo_last_cong) {
    ca.last_cong = ca.undo_last_cong;
    ca.maxRTT = ca.undo_maxRTT;
    ca.old_maxB = ca.undo_old_maxB;
    ca.undo_last_cong = 0;
    }
    return tcp_reno_undo_cwnd(sk);
    }
#[no_mangle]
pub unsafe extern "C" fn measure_rtt(sk: *mut sock, srtt: u32) {
    static inline void measure_rtt(struct sock *sk, u32 srtt)
    {
    const struct inet_connection_sock *icsk = inet_csk(sk);
    struct htcp *ca = inet_csk_ca(sk);
// keep track of minimum RTT seen so far, minRTT is zero at first
    if (ca.minRTT > srtt || !ca.minRTT)
    ca.minRTT = srtt;
// max RTT
    if (icsk.icsk_ca_state == TCP_CA_Open) {
    if (ca.maxRTT < ca.minRTT)
    ca.maxRTT = ca.minRTT;
    if (ca.maxRTT < srtt &&
    srtt <= ca.maxRTT + msecs_to_jiffies(20))
    ca.maxRTT = srtt;
    }
    }
    static void measure_achieved_throughput(struct sock *sk,
    const struct ack_sample *sample)
    {
    const struct inet_connection_sock *icsk = inet_csk(sk);
    const struct tcp_sock *tp = tcp_sk(sk);
    struct htcp *ca = inet_csk_ca(sk);
    let mut now: u32 = tcp_jiffies32;
    if (icsk.icsk_ca_state == TCP_CA_Open)
    ca.pkts_acked = sample.pkts_acked;
    if (sample.rtt_us > 0)
    measure_rtt(sk, usecs_to_jiffies(sample.rtt_us));
    if (!use_bandwidth_switch)
    return;
// achieved throughput calculations
    if (!((1 << icsk.icsk_ca_state) & (TCPF_CA_Open | TCPF_CA_Disorder))) {
    ca.packetcount = 0;
    ca.lasttime = now;
    return;
    }
    ca.packetcount += sample.pkts_acked;
    if (ca.packetcount >= tcp_snd_cwnd(tp) - (ca.alpha >> 7 ? : 1) &&
    now - ca.lasttime >= ca.minRTT &&
    ca.minRTT > 0) {
    let mut cur_Bi: __u32 = ca.packetcount * HZ / (now - ca.lasttime);
    if (htcp_ccount(ca) <= 3) {
// just after backoff
    ca.minB = ca.maxB = ca.Bi = cur_Bi;
    } else {
    ca.Bi = (3 * ca.Bi + cur_Bi) / 4;
    if (ca.Bi > ca.maxB)
    ca.maxB = ca.Bi;
    if (ca.minB > ca.maxB)
    ca.minB = ca.maxB;
    }
    ca.packetcount = 0;
    ca.lasttime = now;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn htcp_beta_update(ca: *mut htcp, minRTT: u32, maxRTT: u32) {
    static inline void htcp_beta_update(struct htcp *ca, u32 minRTT, u32 maxRTT)
    {
    if (use_bandwidth_switch) {
    let mut maxB: u32 = ca.maxB;
    let mut old_maxB: u32 = ca.old_maxB;
    ca.old_maxB = ca.maxB;
    if (!between(5 * maxB, 4 * old_maxB, 6 * old_maxB)) {
    ca.beta = BETA_MIN;
    ca.modeswitch = 0;
    return;
    }
    }
    if (ca.modeswitch && minRTT > msecs_to_jiffies(10) && maxRTT) {
    ca.beta = (minRTT << 7) / maxRTT;
    if (ca.beta < BETA_MIN)
    ca.beta = BETA_MIN;
#[no_mangle]
pub unsafe extern "C" fn if(BETA_MAX: ca->beta >) -> else {
    else if (ca.beta > BETA_MAX)
    ca.beta = BETA_MAX;
    } else {
    ca.beta = BETA_MIN;
    ca.modeswitch = 1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn htcp_alpha_update(ca: *mut htcp) {
    static inline void htcp_alpha_update(struct htcp *ca)
    {
    let mut minRTT: u32 = ca.minRTT;
    let mut factor: u32 = 1;
    let mut diff: u32 = htcp_cong_time(ca);
    if (diff > HZ) {
    diff -= HZ;
    factor = 1 + (10 * diff + ((diff / 2) * (diff / 2) / HZ)) / HZ;
    }
    if (use_rtt_scaling && minRTT) {
    let mut scale: u32 = (HZ << 3) / (10 * minRTT);
// clamping ratio to interval [0.5,10]<<3
    scale = clamp(scale, 1U << 2, 10U << 3);
    factor = (factor << 3) / scale;
    if (!factor)
    factor = 1;
    }
    ca.alpha = 2 * factor * ((1 << 7) - ca.beta);
    if (!ca.alpha)
    ca.alpha = ALPHA_BASE;
    }
//
// After we have the rtt data to calculate beta, we'd still prefer to wait one
// rtt before we adjust our beta to ensure we are working from a consistent
// data.
//
// This function should be called when we hit a congestion event since only at
// that point do we really have a real sense of maxRTT (the queues en route
// were getting just too full now).
//
#[no_mangle]
unsafe extern "C" fn htcp_param_update(sk: *mut sock) {
    static void htcp_param_update(struct sock *sk)
    {
    struct htcp *ca = inet_csk_ca(sk);
    let mut minRTT: u32 = ca.minRTT;
    let mut maxRTT: u32 = ca.maxRTT;
    htcp_beta_update(ca, minRTT, maxRTT);
    htcp_alpha_update(ca);
// add slowly fading memory for maxRTT to accommodate routing changes
    if (minRTT > 0 && maxRTT > minRTT)
    ca.maxRTT = minRTT + ((maxRTT - minRTT) * 95) / 100;
    }
#[no_mangle]
unsafe extern "C" fn htcp_recalc_ssthresh(sk: *mut sock) -> u32 {
    static u32 htcp_recalc_ssthresh(struct sock *sk)
    {
    const struct tcp_sock *tp = tcp_sk(sk);
    const struct htcp *ca = inet_csk_ca(sk);
    htcp_param_update(sk);
    return max((tcp_snd_cwnd(tp) * ca.beta) >> 7, 2U);
    }
#[no_mangle]
unsafe extern "C" fn htcp_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    static void htcp_cong_avoid(struct sock *sk, u32 ack, u32 acked)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    struct htcp *ca = inet_csk_ca(sk);
    if (!tcp_is_cwnd_limited(sk))
    return;
    if (tcp_in_slow_start(tp))
    tcp_slow_start(tp, acked);
    else {
// In dangerous area, increase slowly.
// In theory this is tp->snd_cwnd += alpha / tp->snd_cwnd
//
    if ((tp.snd_cwnd_cnt * ca.alpha)>>7 >= tcp_snd_cwnd(tp)) {
    if (tcp_snd_cwnd(tp) < tp.snd_cwnd_clamp)
    tcp_snd_cwnd_set(tp, tcp_snd_cwnd(tp) + 1);
    tp.snd_cwnd_cnt = 0;
    htcp_alpha_update(ca);
    } else
    tp.snd_cwnd_cnt += ca.pkts_acked;
    ca.pkts_acked = 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn htcp_init(sk: *mut sock) {
    static void htcp_init(struct sock *sk)
    {
    struct htcp *ca = inet_csk_ca(sk);
    memset(ca, 0, sizeof(struct htcp));
    ca.alpha = ALPHA_BASE;
    ca.beta = BETA_MIN;
    ca.pkts_acked = 1;
    ca.last_cong = jiffies;
    }
#[no_mangle]
unsafe extern "C" fn htcp_state(sk: *mut sock, new_state: u8) {
    static void htcp_state(struct sock *sk, u8 new_state)
    {
    switch (new_state) {
    case TCP_CA_Open:
    {
    struct htcp *ca = inet_csk_ca(sk);
    if (ca.undo_last_cong) {
    ca.last_cong = jiffies;
    ca.undo_last_cong = 0;
    }
    }
    break;
    case TCP_CA_CWR:
    case TCP_CA_Recovery:
    case TCP_CA_Loss:
    htcp_reset(inet_csk_ca(sk));
    break;
    }
    }
    static struct tcp_congestion_ops htcp __read_mostly = {
    .init		= htcp_init,
    .ssthresh	= htcp_recalc_ssthresh,
    .cong_avoid	= htcp_cong_avoid,
    .set_state	= htcp_state,
    .undo_cwnd	= htcp_cwnd_undo,
    .pkts_acked	= measure_achieved_throughput,
    .owner		= THIS_MODULE,
    .name		= "htcp",
    };
#[no_mangle]
unsafe extern "C" fn htcp_register() -> int __init {
    static int __init htcp_register(void)
    {
    BUILD_BUG_ON(sizeof(struct htcp) > ICSK_CA_PRIV_SIZE);
    BUILD_BUG_ON(BETA_MIN >= BETA_MAX);
    return tcp_register_congestion_control(&htcp);
    }
#[no_mangle]
unsafe extern "C" fn htcp_unregister() -> void __exit {
    static void __exit htcp_unregister(void)
    {
    tcp_unregister_congestion_control(&htcp);
    }
    module_init(htcp_register);
    module_exit(htcp_unregister);
    MODULE_AUTHOR("Baruch Even");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("H-TCP");
