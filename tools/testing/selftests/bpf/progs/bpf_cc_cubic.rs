//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_cc_cubic.c
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
// Highlights:
// 1. The major difference between this bpf program and tcp_cubic.c
// is that this bpf program relies on `cong_control` rather than
// `cong_avoid` in the struct tcp_congestion_ops.
// 2. Logic such as tcp_cwnd_reduction, tcp_cong_avoid, and
// tcp_update_pacing_rate is bypassed when `cong_control` is
// defined, so moving these logic to `cong_control`.
// 3. WARNING: This bpf program is NOT the same as tcp_cubic.c.
// The main purpose is to show use cases of the arguments in
// `cong_control`. For simplicity's sake, it reuses tcp cubic's
// kernel functions.
//

    extern void cubictcp_init(struct sock *sk) __ksym;
    extern void cubictcp_cwnd_event_tx_start(struct sock *sk) __ksym;
    extern __u32 cubictcp_recalc_ssthresh(struct sock *sk) __ksym;
    extern void cubictcp_state(struct sock *sk, __u8 new_state) __ksym;
    extern __u32 tcp_reno_undo_cwnd(struct sock *sk) __ksym;
    extern void cubictcp_acked(struct sock *sk, const struct ack_sample *sample) __ksym;
    extern void cubictcp_cong_avoid(struct sock *sk, __u32 ack, __u32 acked) __ksym;
#[no_mangle]
unsafe extern "C" fn div64_u64(dividend: __u64, divisor: __u64) -> __u64 {
    static __u64 div64_u64(__u64 dividend, __u64 divisor)
    {
    return dividend / divisor;
    }
#[no_mangle]
unsafe extern "C" fn tcp_update_pacing_rate(sk: *mut sock) {
    static void tcp_update_pacing_rate(struct sock *sk)
    {
    const struct tcp_sock *tp = tcp_sk(sk);
    __u64 rate;
// set sk_pacing_rate to 200 % of current rate (mss * cwnd / srtt)
    rate = (__u64)tp.mss_cache * ((USEC_PER_SEC / 100) << 3);
// current rate is (cwnd * mss) / srtt
// In Slow Start [1], set sk_pacing_rate to 200 % the current rate.
// In Congestion Avoidance phase, set it to 120 % the current rate.
//
// [1] : Normal Slow Start condition is (tp->snd_cwnd < tp->snd_ssthresh)
// If snd_cwnd >= (tp->snd_ssthresh / 2), we are approaching
// end of slow start and should slow down.
//
    if (tp.snd_cwnd < tp.snd_ssthresh / 2)
    rate *= TCP_PACING_SS_RATIO;
    else
    rate *= TCP_PACING_CA_RATIO;
    rate *= max(tp.snd_cwnd, tp.packets_out);
    if (tp.srtt_us)
    rate = div64_u64(rate, (__u64)tp.srtt_us);
    sk.sk_pacing_rate = min(rate, sk.sk_max_pacing_rate);
    }
    static void tcp_cwnd_reduction(struct sock *sk, int newly_acked_sacked,
    int newly_lost, int flag)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    let mut sndcnt: c_int = 0;
    let mut pkts_in_flight: __u32 = tp.packets_out - (tp.sacked_out + tp.lost_out) + tp.retrans_out;
    let mut delta: c_int = tp.snd_ssthresh - pkts_in_flight;
    if (newly_acked_sacked <= 0 || !tp.prior_cwnd)
    return;
    let mut prr_delivered: __u32 = tp.prr_delivered + newly_acked_sacked;
    if (delta < 0) {
    __u64 dividend =
    (__u64)tp.snd_ssthresh * prr_delivered + tp.prior_cwnd - 1;
    sndcnt = (__u32)div64_u64(dividend, (__u64)tp.prior_cwnd) - tp.prr_out;
    } else {
    sndcnt = max(prr_delivered - tp.prr_out, newly_acked_sacked);
    if (flag & FLAG_SND_UNA_ADVANCED && !newly_lost)
    sndcnt++;
    sndcnt = min(delta, sndcnt);
    }
// Force a fast retransmit upon entering fast recovery
    sndcnt = max(sndcnt, (tp.prr_out ? 0 : 1));
    tp.snd_cwnd = pkts_in_flight + sndcnt;
    }
// Decide whether to run the increase function of congestion control.
#[no_mangle]
unsafe extern "C" fn tcp_may_raise_cwnd(sk: *const sock, flag: c_int) -> bool {
    static bool tcp_may_raise_cwnd(const struct sock *sk, const int flag)
    {
    if (tcp_sk(sk).reordering > TCP_REORDERING)
    return flag & FLAG_FORWARD_PROGRESS;
    return flag & FLAG_DATA_ACKED;
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_cubic_init, sk: *mut sock) {
    void BPF_PROG(bpf_cubic_init, struct sock *sk)
    {
    cubictcp_init(sk);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_cubic_cwnd_event_tx_start, sk: *mut sock) {
    void BPF_PROG(bpf_cubic_cwnd_event_tx_start, struct sock *sk)
    {
    cubictcp_cwnd_event_tx_start(sk);
    }
    SEC("struct_ops")
    void BPF_PROG(bpf_cubic_cong_control, struct sock *sk, __u32 ack, int flag,
    const struct rate_sample *rs)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    if (((1<<TCP_CA_CWR) | (1<<TCP_CA_Recovery)) &
    (1 << inet_csk(sk).icsk_ca_state)) {
// Reduce cwnd if state mandates
    tcp_cwnd_reduction(sk, rs.acked_sacked, rs.losses, flag);
    if (!before(tp.snd_una, tp.high_seq)) {
// Reset cwnd to ssthresh in CWR or Recovery (unless it's undone)
    if (tp.snd_ssthresh < TCP_INFINITE_SSTHRESH &&
    inet_csk(sk).icsk_ca_state == TCP_CA_CWR) {
    tp.snd_cwnd = tp.snd_ssthresh;
    tp.snd_cwnd_stamp = tcp_jiffies32;
    }
    }
    } else if (tcp_may_raise_cwnd(sk, flag)) {
// Advance cwnd if state allows
    cubictcp_cong_avoid(sk, ack, rs.acked_sacked);
    tp.snd_cwnd_stamp = tcp_jiffies32;
    }
    tcp_update_pacing_rate(sk);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_cubic_recalc_ssthresh, sk: *mut sock) -> __u32 {
    __u32 BPF_PROG(bpf_cubic_recalc_ssthresh, struct sock *sk)
    {
    return cubictcp_recalc_ssthresh(sk);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_cubic_state, sk: *mut sock, new_state: __u8) {
    void BPF_PROG(bpf_cubic_state, struct sock *sk, __u8 new_state)
    {
    cubictcp_state(sk, new_state);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_cubic_acked, sk: *mut sock, sample: *const ack_sample) {
    void BPF_PROG(bpf_cubic_acked, struct sock *sk, const struct ack_sample *sample)
    {
    cubictcp_acked(sk, sample);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_cubic_undo_cwnd, sk: *mut sock) -> __u32 {
    __u32 BPF_PROG(bpf_cubic_undo_cwnd, struct sock *sk)
    {
    return tcp_reno_undo_cwnd(sk);
    }
    SEC(".struct_ops")
    struct tcp_congestion_ops cc_cubic = {
    .init		= (void *)bpf_cubic_init,
    .ssthresh	= (void *)bpf_cubic_recalc_ssthresh,
    .cong_control	= (void *)bpf_cubic_cong_control,
    .set_state	= (void *)bpf_cubic_state,
    .undo_cwnd	= (void *)bpf_cubic_undo_cwnd,
    .cwnd_event_tx_start	= (void *)bpf_cubic_cwnd_event_tx_start,
    .pkts_acked     = (void *)bpf_cubic_acked,
    .name		= "bpf_cc_cubic",
    };
    char _license[] SEC("license") = "GPL";
