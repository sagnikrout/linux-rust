//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_sock_fields.c
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

    enum bpf_linum_array_idx {
    EGRESS_LINUM_IDX,
    INGRESS_LINUM_IDX,
    READ_SK_DST_PORT_LINUM_IDX,
    __NR_BPF_LINUM_ARRAY_IDX,
    };
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, __NR_BPF_LINUM_ARRAY_IDX);
    __type(key, __u32);
    __type(value, __u32);
    } linum_map SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_spinlock_cnt {
    pub lock: bpf_spin_lock,
    pub cnt: __u32,
}

    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct bpf_spinlock_cnt);
    } sk_pkt_out_cnt SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct bpf_spinlock_cnt);
    } sk_pkt_out_cnt10 SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_sock {
    pub lsndtime: __u32,
    pub __attribute__((preserve_access_index)): },
    pub {}: bpf_tcp_sock listen_tp =,
    pub {}: sockaddr_in6 srv_sa6 =,
    pub {}: bpf_tcp_sock cli_tp =,
    pub {}: bpf_tcp_sock srv_tp =,
    pub {}: bpf_sock listen_sk =,
    pub {}: bpf_sock srv_sk =,
    pub {}: bpf_sock cli_sk =,
    pub 0: __u64 parent_cg_id =,
    pub 0: __u64 child_cg_id =,
    pub 0: __u64 lsndtime =,
#[no_mangle]
unsafe extern "C" fn is_loopback6(a6: *mut __u32) -> bool {
    static bool is_loopback6(__u32 *a6)
    {
    pub bpf_htonl(1): return !a6[0] && !a6[1] && !a6[2] && a6[3] ==,
    }
    static void skcpy(struct bpf_sock *dst,
    const struct bpf_sock *src)
    {
    pub src->bound_dev_if: dst->bound_dev_if =,
    pub src->family: dst->family =,
    pub src->type: dst->type =,
    pub src->protocol: dst->protocol =,
    pub src->mark: dst->mark =,
    pub src->priority: dst->priority =,
    pub src->src_ip4: dst->src_ip4 =,
    pub src->src_ip6[0]: dst->src_ip6[0] =,
    pub src->src_ip6[1]: dst->src_ip6[1] =,
    pub src->src_ip6[2]: dst->src_ip6[2] =,
    pub src->src_ip6[3]: dst->src_ip6[3] =,
    pub src->src_port: dst->src_port =,
    pub src->dst_ip4: dst->dst_ip4 =,
    pub src->dst_ip6[0]: dst->dst_ip6[0] =,
    pub src->dst_ip6[1]: dst->dst_ip6[1] =,
    pub src->dst_ip6[2]: dst->dst_ip6[2] =,
    pub src->dst_ip6[3]: dst->dst_ip6[3] =,
    pub src->dst_port: dst->dst_port =,
    pub src->state: dst->state =,
    }
    static void tpcpy(struct bpf_tcp_sock *dst,
    const struct bpf_tcp_sock *src)
    {
    pub src->snd_cwnd: dst->snd_cwnd =,
    pub src->srtt_us: dst->srtt_us =,
    pub src->rtt_min: dst->rtt_min =,
    pub src->snd_ssthresh: dst->snd_ssthresh =,
    pub src->rcv_nxt: dst->rcv_nxt =,
    pub src->snd_nxt: dst->snd_nxt =,
    pub src->snd_una: dst->snd_una =,
    pub src->mss_cache: dst->mss_cache =,
    pub src->ecn_flags: dst->ecn_flags =,
    pub src->rate_delivered: dst->rate_delivered =,
    pub src->rate_interval_us: dst->rate_interval_us =,
    pub src->packets_out: dst->packets_out =,
    pub src->retrans_out: dst->retrans_out =,
    pub src->total_retrans: dst->total_retrans =,
    pub src->segs_in: dst->segs_in =,
    pub src->data_segs_in: dst->data_segs_in =,
    pub src->segs_out: dst->segs_out =,
    pub src->data_segs_out: dst->data_segs_out =,
    pub src->lost_out: dst->lost_out =,
    pub src->sacked_out: dst->sacked_out =,
    pub src->bytes_received: dst->bytes_received =,
    pub src->bytes_acked: dst->bytes_acked =,
    }
// Always return CG_OK so that no pkt will be filtered out
pub const CG_OK: c_int = 1;

    pub \: linum = __LINE__;,
    pub \: bpf_map_update_elem(&linum_map, &linum_idx, &linum, BPF_ANY);,
    pub \: return CG_OK;,
    })
    SEC("cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn egress_read_sock_fields(skb: *mut __sk_buff) -> c_int {
    int egress_read_sock_fields(struct __sk_buff *skb)
    {
    pub }: bpf_spinlock_cnt cli_cnt_init = { .lock = {}, .cnt = 0xeB9F,
    pub pkt_out_cnt10: *mut *mut bpf_spinlock_cnt pkt_out_cnt,,
    pub tp_ret: *mut *mut bpf_tcp_sock tp,,
    pub sk_ret: *mut *mut bpf_sock sk,,
    pub linum_idx: __u32 linum,,
    pub ktp: *mut tcp_sock,
    pub EGRESS_LINUM_IDX: linum_idx =,
    pub skb->sk: sk =,
    if (!sk)
// Not testing the egress traffic or the listening socket,
// which are covered by the cgroup_skb/ingress test program.
//
    if (sk.family != AF_INET6 || !is_loopback6(sk.src_ip6) ||
    sk.state == BPF_TCP_LISTEN)
    pub CG_OK: return,
    if (sk.src_port == bpf_ntohs(srv_sa6.sin6_port)) {
// Server socket
    pub &srv_sk: sk_ret =,
    pub &srv_tp: tp_ret =,
    } else if (sk.dst_port == srv_sa6.sin6_port) {
// Client socket
    pub &cli_sk: sk_ret =,
    pub &cli_tp: tp_ret =,
    } else {
// Not the testing egress traffic
    pub CG_OK: return,
    }
// It must be a fullsock for cgroup_skb/egress prog
    pub bpf_sk_fullsock(sk): sk =,
    if (!sk)
// Not the testing egress traffic
    if (sk.protocol != IPPROTO_TCP)
    pub CG_OK: return,
    pub bpf_tcp_sock(sk): tp =,
    if (!tp)
    pub sk): skcpy(sk_ret,,
    pub tp): tpcpy(tp_ret,,
    if (sk_ret == &srv_sk) {
    pub bpf_skc_to_tcp_sock(sk): ktp =,
    if (!ktp)
    pub ktp->lsndtime: lsndtime =,
    pub bpf_sk_cgroup_id(ktp): child_cg_id =,
    if (!child_cg_id)
    pub 2): parent_cg_id = bpf_sk_ancestor_cgroup_id(ktp,,
    if (!parent_cg_id)
// The userspace has created it for srv sk
    pub 0): pkt_out_cnt = bpf_sk_storage_get(&sk_pkt_out_cnt, ktp, 0,,
    pkt_out_cnt10 = bpf_sk_storage_get(&sk_pkt_out_cnt10, ktp,
    pub 0): 0,,
    } else {
    pkt_out_cnt = bpf_sk_storage_get(&sk_pkt_out_cnt, sk,
    &cli_cnt_init,
    pkt_out_cnt10 = bpf_sk_storage_get(&sk_pkt_out_cnt10,
    sk, &cli_cnt_init,
    }
    if (!pkt_out_cnt || !pkt_out_cnt10)
// Even both cnt and cnt10 have lock defined in their BTF,
// intentionally one cnt takes lock while one does not
// as a test for the spinlock support in BPF_MAP_TYPE_SK_STORAGE.
//
    pub 1: pkt_out_cnt->cnt +=,
    pub 10: pkt_out_cnt10->cnt +=,
    pub CG_OK: return,
    }
    SEC("cgroup_skb/ingress")
#[no_mangle]
pub unsafe extern "C" fn ingress_read_sock_fields(skb: *mut __sk_buff) -> c_int {
    int ingress_read_sock_fields(struct __sk_buff *skb)
    {
    pub tp: *mut bpf_tcp_sock,
    pub linum_idx: __u32 linum,,
    pub sk: *mut bpf_sock,
    pub INGRESS_LINUM_IDX: linum_idx =,
    pub skb->sk: sk =,
    if (!sk)
// Not the testing ingress traffic to the server
    if (sk.family != AF_INET6 || !is_loopback6(sk.src_ip6) ||
    sk.src_port != bpf_ntohs(srv_sa6.sin6_port))
    pub CG_OK: return,
// Only interested in the listening socket
    if (sk.state != BPF_TCP_LISTEN)
    pub CG_OK: return,
// It must be a fullsock for cgroup_skb/ingress prog
    pub bpf_sk_fullsock(sk): sk =,
    if (!sk)
    pub bpf_tcp_sock(sk): tp =,
    if (!tp)
    pub sk): skcpy(&listen_sk,,
    pub tp): tpcpy(&listen_tp,,
    pub CG_OK: return,
    }
//
// NOTE: 4-byte load from bpf_sock at dst_port offset is quirky. It
// gets rewritten by the access converter to a 2-byte load for
// backward compatibility. Treating the load result as a be16 value
// makes the code portable across little- and big-endian platforms.
//
#[no_mangle]
unsafe extern "C" fn sk_dst_port__load_word(sk: *mut bpf_sock) -> __noinline bool {
    static __noinline bool sk_dst_port__load_word(struct bpf_sock *sk)
    {
    pub )&sk->dst_port: *mut *mut __u32 word = (__u32,
    pub bpf_htons(0xcafe): return word[0] ==,
    }
#[no_mangle]
unsafe extern "C" fn sk_dst_port__load_half(sk: *mut bpf_sock) -> __noinline bool {
    static __noinline bool sk_dst_port__load_half(struct bpf_sock *sk)
    {
    pub half: *mut __u16,
    pub (""): asm volatile,
    pub )&sk->dst_port: *mut half = (__u16,
    pub bpf_htons(0xcafe): return half[0] ==,
    }
#[no_mangle]
unsafe extern "C" fn sk_dst_port__load_byte(sk: *mut bpf_sock) -> __noinline bool {
    static __noinline bool sk_dst_port__load_byte(struct bpf_sock *sk)
    {
    pub )&sk->dst_port: *mut *mut __u8 byte = (__u8,
    pub 0xfe: return byte[0] == 0xca && byte[1] ==,
    }
    SEC("cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn read_sk_dst_port(skb: *mut __sk_buff) -> c_int {
    int read_sk_dst_port(struct __sk_buff *skb)
    {
    pub linum_idx: __u32 linum,,
    pub sk: *mut bpf_sock,
    pub READ_SK_DST_PORT_LINUM_IDX: linum_idx =,
    pub skb->sk: sk =,
    if (!sk)
// Ignore everything but the SYN from the client socket
    if (sk.state != BPF_TCP_SYN_SENT)
    pub CG_OK: return,
    if (!sk_dst_port__load_word(sk))
    if (!sk_dst_port__load_half(sk))
    if (!sk_dst_port__load_byte(sk))
    pub CG_OK: return,
    }
    pub "GPL": char _license[] SEC("license") =,
