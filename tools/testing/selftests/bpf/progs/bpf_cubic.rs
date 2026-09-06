//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_cubic.c
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
// WARNING: This implementation is not necessarily the same
// as the tcp_cubic.c.  The purpose is mainly for testing
// the kernel BPF logic.
//
// Highlights:
// 1. CONFIG_HZ .kconfig map is used.
// 2. In bictcp_update(), calculation is changed to use usec
// resolution (i.e. USEC_PER_JIFFY) instead of using jiffies.
// Thus, usecs_to_jiffies() is not used in the bpf_cubic.c.
// 3. In bitctcp_update() [under tcp_friendliness], the original
// "while (ca->ack_cnt > delta)" loop is changed to the equivalent
// "ca->ack_cnt / delta" operation.
//

    char _license[] SEC("license") = "GPL";

    extern __u32 tcp_slow_start(struct tcp_sock *tp, __u32 acked) __ksym;
    extern void tcp_cong_avoid_ai(struct tcp_sock *tp, __u32 w, __u32 acked) __ksym;

// max_cwnd = snd_cwnd * beta
//

// Two methods of hybrid slow start
pub const HYSTART_ACK_TRAIN: c_uint = 0x1;
pub const HYSTART_DELAY: c_uint = 0x2;
// Number of delay samples for detecting the increase of delay
pub const HYSTART_MIN_SAMPLES: c_int = 8;

    let mut fast_convergence: static int = 1;
    static const int beta = 717;	/* = 717/1024 (BICTCP_BETA_SCALE) */
    static int initial_ssthresh;
    let mut bic_scale: static int = 41;
    let mut tcp_friendliness: static int = 1;
    let mut hystart: static int = 1;
    let mut hystart_detect: static int = HYSTART_ACK_TRAIN | HYSTART_DELAY;
    let mut hystart_low_window: static int = 16;
    let mut hystart_ack_delta_us: static int = 2000;
    static const __u32 cube_rtt_scale = (bic_scale * 10);	/* 1024*c/rtt */
    static const __u32 beta_scale = 8*(BICTCP_BETA_SCALE+beta) / 3
    / (BICTCP_BETA_SCALE - beta);
// calculate the "K" for (wmax-cwnd) = c/rtt * K^3
// so K = cubic_root( (wmax-cwnd)*rtt/c )
// the unit of K is bictcp_HZ=2^10, not HZ
//
// c = bic_scale >> 10
// rtt = 100ms
//
// the following code has been designed and tested for
// cwnd < 1 million packets
// RTT < 100 seconds
// HZ < 1,000,00  (corresponding to 10 nano-second)
//
// 1/c * 2^2*bictcp_HZ * srtt, 2^40
    static const __u64 cube_factor = (__u64)(1ull << (10+3*BICTCP_HZ))
    / (bic_scale * 10);
// BIC TCP Parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_bictcp {
    pub /: *mut *mut __u32 cnt; / increase cwnd by 1 after ACKs,
    pub /: *mut *mut __u32 last_max_cwnd; / last maximum snd_cwnd,
    pub /: *mut *mut __u32 last_cwnd; / the last snd_cwnd,
    pub /: *mut *mut __u32 last_time; / time when updated last_cwnd,
    pub /: *mut *mut __u32 bic_origin_point;/ origin point of bic function,
    pub point: *mut *mut __u32 bic_K; / time to origin,
    from the beginning of the current epoch */
    pub /: *mut *mut __u32 delay_min; / min delay (usec),
    pub /: *mut *mut __u32 epoch_start; / beginning of an epoch,
    pub /: *mut *mut __u32 ack_cnt; / number of acks,
    pub /: *mut *mut __u32 tcp_cwnd; / estimated tcp cwnd,
    pub unused: __u16,
    pub /: *mut *mut __u8 sample_cnt; / number of samples to decide curr_rtt,
    pub /: *mut *mut __u8 found; / the exit point is found?,
    pub /: *mut *mut __u32 round_start; / beginning of each round,
    pub /: *mut *mut __u32 end_seq; / end_seq of the round,
    pub /: *mut *mut __u32 last_ack; / last time when the ACK spacing is close,
    pub /: *mut *mut __u32 curr_rtt; / the minimum rtt of current round,
}

#[no_mangle]
unsafe extern "C" fn bictcp_reset(ca: *mut bpf_bictcp) {
    static void bictcp_reset(struct bpf_bictcp *ca)
    {
    ca.cnt = 0;
    ca.last_max_cwnd = 0;
    ca.last_cwnd = 0;
    ca.last_time = 0;
    ca.bic_origin_point = 0;
    ca.bic_K = 0;
    ca.delay_min = 0;
    ca.epoch_start = 0;
    ca.ack_cnt = 0;
    ca.tcp_cwnd = 0;
    ca.found = 0;
    }
    extern unsigned long CONFIG_HZ __kconfig;

#[no_mangle]
unsafe extern "C" fn div64_u64(dividend: __u64, divisor: __u64) -> __u64 {
    static __u64 div64_u64(__u64 dividend, __u64 divisor)
    {
    return dividend / divisor;
    }

#[no_mangle]
unsafe extern "C" fn fls64(x: __u64) -> c_int {
    static int fls64(__u64 x)
    {
    let mut num: c_int = BITS_PER_U64 - 1;
    if (x == 0)
    return 0;
    if (!(x & (~0ull << (BITS_PER_U64-32)))) {
    num -= 32;
    x <<= 32;
    }
    if (!(x & (~0ull << (BITS_PER_U64-16)))) {
    num -= 16;
    x <<= 16;
    }
    if (!(x & (~0ull << (BITS_PER_U64-8)))) {
    num -= 8;
    x <<= 8;
    }
    if (!(x & (~0ull << (BITS_PER_U64-4)))) {
    num -= 4;
    x <<= 4;
    }
    if (!(x & (~0ull << (BITS_PER_U64-2)))) {
    num -= 2;
    x <<= 2;
    }
    if (!(x & (~0ull << (BITS_PER_U64-1))))
    num -= 1;
    return num + 1;
    }
#[no_mangle]
unsafe extern "C" fn bictcp_clock_us(sk: *const sock) -> __u32 {
    static __u32 bictcp_clock_us(const struct sock *sk)
    {
    return tcp_sk(sk).tcp_mstamp;
    }
#[no_mangle]
unsafe extern "C" fn bictcp_hystart_reset(sk: *mut sock) {
    static void bictcp_hystart_reset(struct sock *sk)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    struct bpf_bictcp *ca = inet_csk_ca(sk);
    ca.round_start = ca.last_ack = bictcp_clock_us(sk);
    ca.end_seq = tp.snd_nxt;
    ca.curr_rtt = ~0U;
    ca.sample_cnt = 0;
    }
    let mut nodelay_init_reject: bool = false;
    let mut nodelay_cwnd_event_tx_start_reject: bool = false;
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_cubic_init, sk: *mut sock) {
    void BPF_PROG(bpf_cubic_init, struct sock *sk)
    {
    struct bpf_bictcp *ca = inet_csk_ca(sk);
    let mut true_val: c_int = 1, ret;
    ret = bpf_setsockopt(sk, SOL_TCP, TCP_NODELAY, &true_val, sizeof(true_val));
    if (ret == -EOPNOTSUPP)
    nodelay_init_reject = true;
    bictcp_reset(ca);
    if (hystart)
    bictcp_hystart_reset(sk);
    if (!hystart && initial_ssthresh)
    tcp_sk(sk).snd_ssthresh = initial_ssthresh;
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_cubic_cwnd_event_tx_start, sk: *mut sock) {
    void BPF_PROG(bpf_cubic_cwnd_event_tx_start, struct sock *sk)
    {
    struct bpf_bictcp *ca = inet_csk_ca(sk);
    let mut now: __u32 = tcp_jiffies32;
    let mut true_val: c_int = 1, ret;
    __s32 delta;
    ret = bpf_setsockopt(sk, SOL_TCP, TCP_NODELAY, &true_val, sizeof(true_val));
    if (ret == -EOPNOTSUPP)
    nodelay_cwnd_event_tx_start_reject = true;
    delta = now - tcp_sk(sk).lsndtime;
// We were application limited (idle) for a while.
// Shift epoch_start to keep cwnd growth to cubic curve.
//
    if (ca.epoch_start && delta > 0) {
    ca.epoch_start += delta;
    if (after(ca.epoch_start, now))
    ca.epoch_start = now;
    }
    }
//
// cbrt(x) MSB values for x MSB values in [0..63].
// Precomputed then refined by hand - Willy Tarreau
//
// For x in [0..63],
// v = cbrt(x << 18) - 1
// cbrt(x) = (v[x] + 10) >> 6
//
    static const __u8 v[] = {
// 0x00 */    0,   54,   54,   54,  118,  118,  118,  118,
// 0x08 */  123,  129,  134,  138,  143,  147,  151,  156,
// 0x10 */  157,  161,  164,  168,  170,  173,  176,  179,
// 0x18 */  181,  185,  187,  190,  192,  194,  197,  199,
// 0x20 */  200,  202,  204,  206,  209,  211,  213,  215,
// 0x28 */  217,  219,  221,  222,  224,  225,  227,  229,
// 0x30 */  231,  232,  234,  236,  237,  239,  240,  242,
// 0x38 */  244,  245,  246,  248,  250,  251,  252,  254,
    };
// calculate the cubic root of x using a table lookup followed by one
// Newton-Raphson iteration.
// Avg err ~= 0.195%
//
#[no_mangle]
unsafe extern "C" fn cubic_root(a: __u64) -> __u32 {
    static __u32 cubic_root(__u64 a)
    {
    __u32 x, b, shift;
    if (a < 64) {
// a in [0..63]
    return ((__u32)v[(__u32)a] + 35) >> 6;
    }
    b = fls64(a);
    b = ((b * 84) >> 8) - 1;
    shift = (a >> (b * 3));
// it is needed for verifier's bound check on v
    if (shift >= 64)
    return 0;
    x = ((__u32)(((__u32)v[shift] + 10) << b)) >> 6;
//
// Newton-Raphson iteration
// 2
// x    = ( 2 * x  +  a / x  ) / 3
// k+1          k         k
//
    x = (2 * x + (__u32)div64_u64(a, (__u64)x * (__u64)(x - 1)));
    x = ((x * 341) >> 10);
    return x;
    }
//
// Compute congestion window to use.
//
#[no_mangle]
unsafe extern "C" fn bictcp_update(ca: *mut bpf_bictcp, cwnd: __u32, acked: __u32) {
    static void bictcp_update(struct bpf_bictcp *ca, __u32 cwnd, __u32 acked)
    {
    __u32 delta, bic_target, max_cnt;
    __u64 offs, t;
    ca.ack_cnt += acked;	/* count the number of ACKed packets */
    if (ca.last_cwnd == cwnd &&
    (__s32)(tcp_jiffies32 - ca.last_time) <= HZ / 32)
    return;
// The CUBIC function can update ca->cnt at most once per jiffy.
// On all cwnd reduction events, ca->epoch_start is set to 0,
// which will force a recalculation of ca->cnt.
//
    if (ca.epoch_start && tcp_jiffies32 == ca.last_time)
    goto tcp_friendliness;
    ca.last_cwnd = cwnd;
    ca.last_time = tcp_jiffies32;
    if (ca.epoch_start == 0) {
    ca.epoch_start = tcp_jiffies32;	/* record beginning */
    ca.ack_cnt = acked;			/* start counting */
    ca.tcp_cwnd = cwnd;			/* syn with cubic */
    if (ca.last_max_cwnd <= cwnd) {
    ca.bic_K = 0;
    ca.bic_origin_point = cwnd;
    } else {
// Compute new K based on
// (wmax-cwnd) * (srtt>>3 / HZ) / c * 2^(3*bictcp_HZ)
//
    ca.bic_K = cubic_root(cube_factor
// (ca->last_max_cwnd - cwnd));
    ca.bic_origin_point = ca.last_max_cwnd;
    }
    }
// cubic function - calc
// calculate c * time^3 / rtt,
// while considering overflow in calculation of time^3
// (so time^3 is done by using 64 bit)
// and without the support of division of 64bit numbers
// (so all divisions are done by using 32 bit)
// also NOTE the unit of those variables
// time  = (t - K) / 2^bictcp_HZ
// c = bic_scale >> 10
// rtt  = (srtt >> 3) / HZ
// !!! The following code does not have overflow problems,
// if the cwnd < 1 million packets !!!
//
    t = (__s32)(tcp_jiffies32 - ca.epoch_start) * USEC_PER_JIFFY;
    t += ca.delay_min;
// change the unit from usec to bictcp_HZ
    t <<= BICTCP_HZ;
    t /= USEC_PER_SEC;
    if (t < ca.bic_K)		/* t - K */
    offs = ca.bic_K - t;
    else
    offs = t - ca.bic_K;
// c/rtt * (t-K)^3
    delta = (cube_rtt_scale * offs * offs * offs) >> (10+3*BICTCP_HZ);
    if (t < ca.bic_K)                            /* below origin*/
    bic_target = ca.bic_origin_point - delta;
    else                                          /* above origin*/
    bic_target = ca.bic_origin_point + delta;
// cubic function - calc bictcp_cnt
    if (bic_target > cwnd) {
    ca.cnt = cwnd / (bic_target - cwnd);
    } else {
    ca.cnt = 100 * cwnd;              /* very small increment*/
    }
//
// The initial growth of cubic function may be too conservative
// when the available bandwidth is still unknown.
//
    if (ca.last_max_cwnd == 0 && ca.cnt > 20)
    ca.cnt = 20;	/* increase cwnd 5% per RTT */
    tcp_friendliness:
// TCP Friendly
    if (tcp_friendliness) {
    let mut scale: __u32 = beta_scale;
    __u32 n;
// update tcp cwnd
    delta = (cwnd * scale) >> 3;
    if (ca.ack_cnt > delta && delta) {
    n = ca.ack_cnt / delta;
    ca.ack_cnt -= n * delta;
    ca.tcp_cwnd += n;
    }
    if (ca.tcp_cwnd > cwnd) {	/* if bic is slower than tcp */
    delta = ca.tcp_cwnd - cwnd;
    max_cnt = cwnd / delta;
    if (ca.cnt > max_cnt)
    ca.cnt = max_cnt;
    }
    }
// The maximum rate of cwnd increase CUBIC allows is 1 packet per
// 2 packets ACKed, meaning cwnd grows at 1.5x per RTT.
//
    ca.cnt = max(ca.cnt, 2U);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_cubic_cong_avoid, sk: *mut sock, ack: __u32, acked: __u32) {
    void BPF_PROG(bpf_cubic_cong_avoid, struct sock *sk, __u32 ack, __u32 acked)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    struct bpf_bictcp *ca = inet_csk_ca(sk);
    if (!tcp_is_cwnd_limited(sk))
    return;
    if (tcp_in_slow_start(tp)) {
    if (hystart && after(ack, ca.end_seq))
    bictcp_hystart_reset(sk);
    acked = tcp_slow_start(tp, acked);
    if (!acked)
    return;
    }
    bictcp_update(ca, tp.snd_cwnd, acked);
    tcp_cong_avoid_ai(tp, ca.cnt, acked);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_cubic_recalc_ssthresh, sk: *mut sock) -> __u32 {
    __u32 BPF_PROG(bpf_cubic_recalc_ssthresh, struct sock *sk)
    {
    const struct tcp_sock *tp = tcp_sk(sk);
    struct bpf_bictcp *ca = inet_csk_ca(sk);
    ca.epoch_start = 0;	/* end of epoch */
// Wmax and fast convergence
    if (tp.snd_cwnd < ca.last_max_cwnd && fast_convergence)
    ca.last_max_cwnd = (tp.snd_cwnd * (BICTCP_BETA_SCALE + beta))
    / (2 * BICTCP_BETA_SCALE);
    else
    ca.last_max_cwnd = tp.snd_cwnd;
    return max((tp.snd_cwnd * beta) / BICTCP_BETA_SCALE, 2U);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_cubic_state, sk: *mut sock, new_state: __u8) {
    void BPF_PROG(bpf_cubic_state, struct sock *sk, __u8 new_state)
    {
    if (new_state == TCP_CA_Loss) {
    bictcp_reset(inet_csk_ca(sk));
    bictcp_hystart_reset(sk);
    }
    }
pub const GSO_MAX_SIZE: c_int = 65536;
// Account for TSO/GRO delays.
// Otherwise short RTT flows could get too small ssthresh, since during
// slow start we begin with small TSO packets and ca->delay_min would
// not account for long aggregation delay when TSO packets get bigger.
// Ideally even with a very small RTT we would like to have at least one
// TSO packet being sent and received by GRO, and another one in qdisc layer.
// We apply another 100% factor because @rate is doubled at this point.
// We cap the cushion to 1ms.
//
#[no_mangle]
unsafe extern "C" fn hystart_ack_delay(sk: *mut sock) -> __u32 {
    static __u32 hystart_ack_delay(struct sock *sk)
    {
    unsigned long rate;
    rate = sk.sk_pacing_rate;
    if (!rate)
    return 0;
    return min((__u64)USEC_PER_MSEC,
    div64_ul((__u64)GSO_MAX_SIZE * 4 * USEC_PER_SEC, rate));
    }
#[no_mangle]
unsafe extern "C" fn hystart_update(sk: *mut sock, delay: __u32) {
    static void hystart_update(struct sock *sk, __u32 delay)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    struct bpf_bictcp *ca = inet_csk_ca(sk);
    __u32 threshold;
    if (hystart_detect & HYSTART_ACK_TRAIN) {
    let mut now: __u32 = bictcp_clock_us(sk);
// first detection parameter - ack-train detection
    if ((__s32)(now - ca.last_ack) <= hystart_ack_delta_us) {
    ca.last_ack = now;
    threshold = ca.delay_min + hystart_ack_delay(sk);
// Hystart ack train triggers if we get ack past
// ca->delay_min/2.
// Pacing might have delayed packets up to RTT/2
// during slow start.
//
    if (sk.sk_pacing_status == SK_PACING_NONE)
    threshold >>= 1;
    if ((__s32)(now - ca.round_start) > threshold) {
    ca.found = 1;
    tp.snd_ssthresh = tp.snd_cwnd;
    }
    }
    }
    if (hystart_detect & HYSTART_DELAY) {
// obtain the minimum delay of more than sampling packets
    if (ca.curr_rtt > delay)
    ca.curr_rtt = delay;
    if (ca.sample_cnt < HYSTART_MIN_SAMPLES) {
    ca.sample_cnt++;
    } else {
    if (ca.curr_rtt > ca.delay_min +
    HYSTART_DELAY_THRESH(ca.delay_min >> 3)) {
    ca.found = 1;
    tp.snd_ssthresh = tp.snd_cwnd;
    }
    }
    }
    }
    let mut bpf_cubic_acked_called: c_int = 0;
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_cubic_acked, sk: *mut sock, sample: *const ack_sample) {
    void BPF_PROG(bpf_cubic_acked, struct sock *sk, const struct ack_sample *sample)
    {
    const struct tcp_sock *tp = tcp_sk(sk);
    struct bpf_bictcp *ca = inet_csk_ca(sk);
    __u32 delay;
    bpf_cubic_acked_called = 1;
// Some calls are for duplicates without timestamps
    if (sample.rtt_us < 0)
    return;
// Discard delay samples right after fast recovery
    if (ca.epoch_start && (__s32)(tcp_jiffies32 - ca.epoch_start) < HZ)
    return;
    delay = sample.rtt_us;
    if (delay == 0)
    delay = 1;
// first time call or link delay decreases
    if (ca.delay_min == 0 || ca.delay_min > delay)
    ca.delay_min = delay;
// hystart triggers when cwnd is larger than some threshold
    if (!ca.found && tcp_in_slow_start(tp) && hystart &&
    tp.snd_cwnd >= hystart_low_window)
    hystart_update(sk, delay);
    }
    extern __u32 tcp_reno_undo_cwnd(struct sock *sk) __ksym;
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_cubic_undo_cwnd, sk: *mut sock) -> __u32 {
    __u32 BPF_PROG(bpf_cubic_undo_cwnd, struct sock *sk)
    {
    return tcp_reno_undo_cwnd(sk);
    }
    SEC(".struct_ops")
    struct tcp_congestion_ops cubic = {
    .init		= (void *)bpf_cubic_init,
    .ssthresh	= (void *)bpf_cubic_recalc_ssthresh,
    .cong_avoid	= (void *)bpf_cubic_cong_avoid,
    .set_state	= (void *)bpf_cubic_state,
    .undo_cwnd	= (void *)bpf_cubic_undo_cwnd,
    .cwnd_event_tx_start	= (void *)bpf_cubic_cwnd_event_tx_start,
    .pkts_acked     = (void *)bpf_cubic_acked,
    .name		= "bpf_cubic",
    };
