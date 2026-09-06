//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_bpf_nf.c
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
// Macro flag: #define BPF_NO_KFUNC_PROTOTYPES

pub const EAFNOSUPPORT: c_int = 97;
pub const EPROTO: c_int = 71;
pub const ENONET: c_int = 64;
pub const EINVAL: c_int = 22;
pub const ENOENT: c_int = 2;
pub const CT_OPTS_ERROR_GUARD: c_uint = 0x12345678;

    extern unsigned long CONFIG_HZ __kconfig;
    let mut test_einval_reserved: c_int = 0;
    let mut test_einval_reserved_new: c_int = 0;
    let mut test_einval_netns_id: c_int = 0;
    let mut test_einval_len_opts: c_int = 0;
    let mut test_einval_len_opts_small_lookup: c_int = 0;
    let mut test_einval_len_opts_small_alloc: c_int = 0;
    let mut test_eproto_l4proto: c_int = 0;
    let mut test_enonet_netns_id: c_int = 0;
    let mut test_enoent_lookup: c_int = 0;
    let mut test_eafnosupport: c_int = 0;
    let mut test_alloc_entry: c_int = -EINVAL;
    let mut test_insert_entry: c_int = -EAFNOSUPPORT;
    let mut test_succ_lookup: c_int = -ENOENT;
    let mut test_ct_zone_id_alloc_entry: c_int = -EINVAL;
    let mut test_ct_zone_id_insert_entry: c_int = -EAFNOSUPPORT;
    let mut test_ct_zone_id_succ_lookup: c_int = -ENOENT;
    let mut test_ct_zone_dir_enoent_lookup: c_int = 0;
    let mut test_ct_zone_id_enoent_lookup: c_int = 0;
    let mut test_delta_timeout: u32 = 0;
    let mut test_status: u32 = 0;
    let mut test_insert_lookup_mark: u32 = 0;
    let mut test_snat_addr: c_int = -EINVAL;
    let mut test_dnat_addr: c_int = -EINVAL;
    let mut saddr: __be32 = 0;
    let mut sport: __be16 = 0;
    let mut daddr: __be32 = 0;
    let mut dport: __be16 = 0;
    let mut test_exist_lookup: c_int = -ENOENT;
    let mut test_exist_lookup_mark: u32 = 0;
    enum nf_nat_manip_type___local {
    NF_NAT_MANIP_SRC___local,
    NF_NAT_MANIP_DST___local
    };
    struct nf_conn;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_ct_opts___local {
    pub netns_id: i32,
    pub error: i32,
    pub l4proto: u8,
    pub dir: u8,
    pub reserved: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_ct_opts___new {
    pub netns_id: i32,
    pub error: i32,
    pub l4proto: u8,
    pub dir: u8,
    pub ct_zone_id: u16,
    pub ct_zone_dir: u8,
    pub reserved: [u8; 3],
    pub __attribute__((preserve_access_index)): },
    struct nf_conn *bpf_xdp_ct_alloc(struct xdp_md *, struct bpf_sock_tuple *, u32,
    pub __ksym: *mut *mut bpf_ct_opts___local , u32),
    struct nf_conn *bpf_xdp_ct_lookup(struct xdp_md *, struct bpf_sock_tuple *, u32,
    pub __ksym: *mut *mut bpf_ct_opts___local , u32),
    struct nf_conn *bpf_skb_ct_alloc(struct __sk_buff *, struct bpf_sock_tuple *, u32,
    pub __ksym: *mut *mut bpf_ct_opts___local , u32),
    struct nf_conn *bpf_skb_ct_lookup(struct __sk_buff *, struct bpf_sock_tuple *, u32,
    pub __ksym: *mut *mut bpf_ct_opts___local , u32),
    pub __ksym: *mut *mut *mut nf_conn bpf_ct_insert_entry(nf_conn ),
    pub __ksym: *mut *mut void bpf_ct_release(struct nf_conn ),
    pub __ksym: *mut *mut void bpf_ct_set_timeout(struct nf_conn , u32),
    pub __ksym: *mut *mut int bpf_ct_change_timeout(struct nf_conn , u32),
    pub __ksym: *mut *mut int bpf_ct_set_status(struct nf_conn , u32),
    pub __ksym: *mut *mut int bpf_ct_change_status(struct nf_conn , u32),
    int bpf_ct_set_nat_info(struct nf_conn *, union nf_inet_addr *,
    pub __ksym: int port, enum nf_nat_manip_type___local),
    static __always_inline void
    nf_ct_test(struct nf_conn *(*lookup_fn)(void *, struct bpf_sock_tuple *, u32,
    struct bpf_ct_opts___local *, u32),
    struct nf_conn *(*alloc_fn)(void *, struct bpf_sock_tuple *, u32,
    struct bpf_ct_opts___local *, u32),
    void *ctx)
    {
    pub }: bpf_ct_opts___local opts_def = { .l4proto = IPPROTO_TCP, .netns_id = -1,
    pub bpf_tuple: bpf_sock_tuple,
    pub ct: *mut nf_conn,
    pub sizeof(bpf_tuple.ipv4)): __builtin_memset(&bpf_tuple, 0,,
    pub 1: opts_def.reserved[0] =,
    ct = lookup_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4), &opts_def,
    pub 0: opts_def.reserved[0] =,
    pub IPPROTO_TCP: opts_def.l4proto =,
    if (ct)
    else
    pub opts_def.error: test_einval_reserved =,
    pub -2: opts_def.netns_id =,
    ct = lookup_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4), &opts_def,
    pub -1: opts_def.netns_id =,
    if (ct)
    else
    pub opts_def.error: test_einval_netns_id =,
    ct = lookup_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4), &opts_def,
    pub 1): sizeof(opts_def) -,
    if (ct)
    else
    pub opts_def.error: test_einval_len_opts =,
    pub CT_OPTS_ERROR_GUARD: opts_def.error =,
    ct = lookup_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4), &opts_def,
    if (ct) {
    pub -EINVAL: test_einval_len_opts_small_lookup =,
    } else {
    pub opts_def.error: test_einval_len_opts_small_lookup =,
    }
    pub CT_OPTS_ERROR_GUARD: opts_def.error =,
    ct = alloc_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4), &opts_def,
    if (ct) {
    pub bpf_ct_insert_entry(ct): ct =,
    if (ct)
    pub -EINVAL: test_einval_len_opts_small_alloc =,
    } else {
    pub opts_def.error: test_einval_len_opts_small_alloc =,
    }
    pub IPPROTO_ICMP: opts_def.l4proto =,
    ct = lookup_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4), &opts_def,
    pub IPPROTO_TCP: opts_def.l4proto =,
    if (ct)
    else
    pub opts_def.error: test_eproto_l4proto =,
    pub 0xf00f: opts_def.netns_id =,
    ct = lookup_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4), &opts_def,
    pub -1: opts_def.netns_id =,
    if (ct)
    else
    pub opts_def.error: test_enonet_netns_id =,
    ct = lookup_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4), &opts_def,
    if (ct)
    else
    pub opts_def.error: test_enoent_lookup =,
    ct = lookup_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4) - 1, &opts_def,
    if (ct)
    else
    pub opts_def.error: test_eafnosupport =,
    pub /: *mut *mut bpf_tuple.ipv4.saddr = bpf_get_prandom_u32(); / src IP,
    pub /: *mut *mut bpf_tuple.ipv4.daddr = bpf_get_prandom_u32(); / dst IP,
    pub /: *mut *mut bpf_tuple.ipv4.sport = bpf_get_prandom_u32(); / src port,
    pub /: *mut *mut bpf_tuple.ipv4.dport = bpf_get_prandom_u32(); / dst port,
    ct = alloc_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4), &opts_def,
    if (ct) {
    pub bpf_get_prandom_u32(): __u16 sport =,
    pub bpf_get_prandom_u32(): __u16 dport =,
    pub {}: union nf_inet_addr saddr =,
    pub {}: union nf_inet_addr daddr =,
    pub ct_ins: *mut nf_conn,
    pub 10000): bpf_ct_set_timeout(ct,,
    pub 77: ct->mark =,
// snat
    pub bpf_get_prandom_u32(): saddr.ip =,
    pub NF_NAT_MANIP_SRC___local): bpf_ct_set_nat_info(ct, &saddr, sport,,
// dnat
    pub bpf_get_prandom_u32(): daddr.ip =,
    pub NF_NAT_MANIP_DST___local): bpf_ct_set_nat_info(ct, &daddr, dport,,
    pub bpf_ct_insert_entry(ct): ct_ins =,
    if (ct_ins) {
    pub ct_lk: *mut nf_conn,
    ct_lk = lookup_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4),
    pub sizeof(opts_def)): &opts_def,,
    if (ct_lk) {
    pub tuple: *mut nf_conntrack_tuple,
// check snat and dnat addresses
    pub &ct_lk->tuplehash[IP_CT_DIR_REPLY].tuple: tuple =,
    if (tuple.dst.u3.ip == saddr.ip &&
    tuple.dst.u.all == bpf_htons(sport))
    pub 0: test_snat_addr =,
    if (tuple.src.u3.ip == daddr.ip &&
    tuple.src.u.all == bpf_htons(dport))
    pub 0: test_dnat_addr =,
// update ct entry timeout
    pub 10000): bpf_ct_change_timeout(ct_lk,,
    pub bpf_jiffies64(): test_delta_timeout = ct_lk->timeout -,
    pub CONFIG_HZ: test_delta_timeout /=,
    pub ct_lk->mark: test_insert_lookup_mark =,
    bpf_ct_change_status(ct_lk,
    pub IPS_SEEN_REPLY): IPS_CONFIRMED |,
    pub ct_lk->status: test_status =,
    pub 0: test_succ_lookup =,
    }
    pub 0: test_insert_entry =,
    }
    pub 0: test_alloc_entry =,
    }
    pub saddr: bpf_tuple.ipv4.saddr =,
    pub daddr: bpf_tuple.ipv4.daddr =,
    pub sport: bpf_tuple.ipv4.sport =,
    pub dport: bpf_tuple.ipv4.dport =,
    ct = lookup_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4), &opts_def,
    if (ct) {
    pub 0: test_exist_lookup =,
    if (ct.mark == 42) {
    pub ct->mark: test_exist_lookup_mark =,
    }
    } else {
    pub opts_def.error: test_exist_lookup =,
    }
    }
    static __always_inline void
    nf_ct_opts_new_test(struct nf_conn *(*lookup_fn)(void *, struct bpf_sock_tuple *, u32,
    struct bpf_ct_opts___new *, u32),
    struct nf_conn *(*alloc_fn)(void *, struct bpf_sock_tuple *, u32,
    struct bpf_ct_opts___new *, u32),
    void *ctx)
    {
    pub }: bpf_ct_opts___new opts_def = { .l4proto = IPPROTO_TCP, .netns_id = -1,
    pub bpf_tuple: bpf_sock_tuple,
    pub ct: *mut nf_conn,
    pub sizeof(bpf_tuple.ipv4)): __builtin_memset(&bpf_tuple, 0,,
    pub 1: opts_def.reserved[0] =,
    ct = lookup_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4), &opts_def,
    pub 0: opts_def.reserved[0] =,
    if (ct)
    else
    pub opts_def.error: test_einval_reserved_new =,
    pub /: *mut *mut bpf_tuple.ipv4.saddr = bpf_get_prandom_u32(); / src IP,
    pub /: *mut *mut bpf_tuple.ipv4.daddr = bpf_get_prandom_u32(); / dst IP,
    pub /: *mut *mut bpf_tuple.ipv4.sport = bpf_get_prandom_u32(); / src port,
    pub /: *mut *mut bpf_tuple.ipv4.dport = bpf_get_prandom_u32(); / dst port,
// use non-default ct zone
    pub 10: opts_def.ct_zone_id =,
    pub NF_CT_ZONE_DIR_ORIG: opts_def.ct_zone_dir =,
    ct = alloc_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4), &opts_def,
    if (ct) {
    pub bpf_get_prandom_u32(): __u16 sport =,
    pub bpf_get_prandom_u32(): __u16 dport =,
    pub {}: union nf_inet_addr saddr =,
    pub {}: union nf_inet_addr daddr =,
    pub ct_ins: *mut nf_conn,
    pub 10000): bpf_ct_set_timeout(ct,,
// snat
    pub bpf_get_prandom_u32(): saddr.ip =,
    pub NF_NAT_MANIP_SRC___local): bpf_ct_set_nat_info(ct, &saddr, sport,,
// dnat
    pub bpf_get_prandom_u32(): daddr.ip =,
    pub NF_NAT_MANIP_DST___local): bpf_ct_set_nat_info(ct, &daddr, dport,,
    pub bpf_ct_insert_entry(ct): ct_ins =,
    if (ct_ins) {
    pub ct_lk: *mut nf_conn,
// entry should exist in same ct zone we inserted it
    ct_lk = lookup_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4),
    pub sizeof(opts_def)): &opts_def,,
    if (ct_lk) {
    pub 0: test_ct_zone_id_succ_lookup =,
    }
// entry should not exist with wrong direction
    pub NF_CT_ZONE_DIR_REPL: opts_def.ct_zone_dir =,
    ct_lk = lookup_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4),
    pub sizeof(opts_def)): &opts_def,,
    pub NF_CT_ZONE_DIR_ORIG: opts_def.ct_zone_dir =,
    if (ct_lk)
    else
    pub opts_def.error: test_ct_zone_dir_enoent_lookup =,
// entry should not exist in default ct zone
    pub 0: opts_def.ct_zone_id =,
    ct_lk = lookup_fn(ctx, &bpf_tuple, sizeof(bpf_tuple.ipv4),
    pub sizeof(opts_def)): &opts_def,,
    if (ct_lk)
    else
    pub opts_def.error: test_ct_zone_id_enoent_lookup =,
    pub 0: test_ct_zone_id_insert_entry =,
    }
    pub 0: test_ct_zone_id_alloc_entry =,
    }
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn nf_xdp_ct_test(ctx: *mut xdp_md) -> c_int {
    int nf_xdp_ct_test(struct xdp_md *ctx)
    {
    pub ctx): *mut *mut *mut nf_ct_test((void )bpf_xdp_ct_lookup, (void )bpf_xdp_ct_alloc,,
    pub ctx): *mut *mut *mut nf_ct_opts_new_test((void )bpf_xdp_ct_lookup, (void )bpf_xdp_ct_alloc,,
    pub 0: return,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn nf_skb_ct_test(ctx: *mut __sk_buff) -> c_int {
    int nf_skb_ct_test(struct __sk_buff *ctx)
    {
    pub ctx): *mut *mut *mut nf_ct_test((void )bpf_skb_ct_lookup, (void )bpf_skb_ct_alloc,,
    pub ctx): *mut *mut *mut nf_ct_opts_new_test((void )bpf_skb_ct_lookup, (void )bpf_skb_ct_alloc,,
    pub 0: return,
    }
    pub "GPL": char _license[] SEC("license") =,
