//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_bpf_nf_fail.c
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

    struct nf_conn;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_ct_opts___local {
    pub netns_id: i32,
    pub error: i32,
    pub l4proto: u8,
    pub reserved: [u8; 3],
    pub __attribute__((preserve_access_index)): },
    struct nf_conn *bpf_skb_ct_alloc(struct __sk_buff *, struct bpf_sock_tuple *, u32,
    pub __ksym: *mut *mut bpf_ct_opts___local , u32),
    struct nf_conn *bpf_skb_ct_lookup(struct __sk_buff *, struct bpf_sock_tuple *, u32,
    pub __ksym: *mut *mut bpf_ct_opts___local , u32),
    struct nf_conn *bpf_xdp_ct_alloc(struct xdp_md *, struct bpf_sock_tuple *, u32,
    pub __ksym: *mut *mut bpf_ct_opts___local , u32),
    struct nf_conn *bpf_xdp_ct_lookup(struct xdp_md *, struct bpf_sock_tuple *, u32,
    pub __ksym: *mut *mut bpf_ct_opts___local , u32),
    pub __ksym: *mut *mut *mut nf_conn bpf_ct_insert_entry(nf_conn ),
    pub __ksym: *mut *mut void bpf_ct_release(struct nf_conn ),
    pub __ksym: *mut *mut void bpf_ct_set_timeout(struct nf_conn , u32),
    pub __ksym: *mut *mut int bpf_ct_change_timeout(struct nf_conn , u32),
    pub __ksym: *mut *mut int bpf_ct_set_status(struct nf_conn , u32),
    pub __ksym: *mut *mut int bpf_ct_change_status(struct nf_conn , u32),
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn alloc_release(ctx: *mut __sk_buff) -> c_int {
    int alloc_release(struct __sk_buff *ctx)
    {
    pub {}: bpf_ct_opts___local opts =,
    pub {}: bpf_sock_tuple tup =,
    pub ct: *mut nf_conn,
    pub sizeof(opts)): ct = bpf_skb_ct_alloc(ctx, &tup, sizeof(tup.ipv4), &opts,,
    if (!ct)
    pub 0: return,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn insert_insert(ctx: *mut __sk_buff) -> c_int {
    int insert_insert(struct __sk_buff *ctx)
    {
    pub {}: bpf_ct_opts___local opts =,
    pub {}: bpf_sock_tuple tup =,
    pub ct: *mut nf_conn,
    pub sizeof(opts)): ct = bpf_skb_ct_alloc(ctx, &tup, sizeof(tup.ipv4), &opts,,
    if (!ct)
    pub 0: return,
    pub bpf_ct_insert_entry(ct): ct =,
    if (!ct)
    pub 0: return,
    pub bpf_ct_insert_entry(ct): ct =,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn lookup_insert(ctx: *mut __sk_buff) -> c_int {
    int lookup_insert(struct __sk_buff *ctx)
    {
    pub {}: bpf_ct_opts___local opts =,
    pub {}: bpf_sock_tuple tup =,
    pub ct: *mut nf_conn,
    pub sizeof(opts)): ct = bpf_skb_ct_lookup(ctx, &tup, sizeof(tup.ipv4), &opts,,
    if (!ct)
    pub 0: return,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn write_not_allowlisted_field(ctx: *mut __sk_buff) -> c_int {
    int write_not_allowlisted_field(struct __sk_buff *ctx)
    {
    pub {}: bpf_ct_opts___local opts =,
    pub {}: bpf_sock_tuple tup =,
    pub ct: *mut nf_conn,
    pub sizeof(opts)): ct = bpf_skb_ct_lookup(ctx, &tup, sizeof(tup.ipv4), &opts,,
    if (!ct)
    pub 0: return,
    pub 0xF00: ct->status =,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn set_timeout_after_insert(ctx: *mut __sk_buff) -> c_int {
    int set_timeout_after_insert(struct __sk_buff *ctx)
    {
    pub {}: bpf_ct_opts___local opts =,
    pub {}: bpf_sock_tuple tup =,
    pub ct: *mut nf_conn,
    pub sizeof(opts)): ct = bpf_skb_ct_alloc(ctx, &tup, sizeof(tup.ipv4), &opts,,
    if (!ct)
    pub 0: return,
    pub bpf_ct_insert_entry(ct): ct =,
    if (!ct)
    pub 0: return,
    pub 0): bpf_ct_set_timeout(ct,,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn set_status_after_insert(ctx: *mut __sk_buff) -> c_int {
    int set_status_after_insert(struct __sk_buff *ctx)
    {
    pub {}: bpf_ct_opts___local opts =,
    pub {}: bpf_sock_tuple tup =,
    pub ct: *mut nf_conn,
    pub sizeof(opts)): ct = bpf_skb_ct_alloc(ctx, &tup, sizeof(tup.ipv4), &opts,,
    if (!ct)
    pub 0: return,
    pub bpf_ct_insert_entry(ct): ct =,
    if (!ct)
    pub 0: return,
    pub 0): bpf_ct_set_status(ct,,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn change_timeout_after_alloc(ctx: *mut __sk_buff) -> c_int {
    int change_timeout_after_alloc(struct __sk_buff *ctx)
    {
    pub {}: bpf_ct_opts___local opts =,
    pub {}: bpf_sock_tuple tup =,
    pub ct: *mut nf_conn,
    pub sizeof(opts)): ct = bpf_skb_ct_alloc(ctx, &tup, sizeof(tup.ipv4), &opts,,
    if (!ct)
    pub 0: return,
    pub 0): bpf_ct_change_timeout(ct,,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn change_status_after_alloc(ctx: *mut __sk_buff) -> c_int {
    int change_status_after_alloc(struct __sk_buff *ctx)
    {
    pub {}: bpf_ct_opts___local opts =,
    pub {}: bpf_sock_tuple tup =,
    pub ct: *mut nf_conn,
    pub sizeof(opts)): ct = bpf_skb_ct_alloc(ctx, &tup, sizeof(tup.ipv4), &opts,,
    if (!ct)
    pub 0: return,
    pub 0): bpf_ct_change_status(ct,,
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(R2": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R2")
#[no_mangle]
pub unsafe extern "C" fn lookup_null_bpf_tuple(ctx: *mut __sk_buff) -> c_int {
    int lookup_null_bpf_tuple(struct __sk_buff *ctx)
    {
    pub {}: bpf_ct_opts___local opts =,
    pub ct: *mut nf_conn,
    pub sizeof(opts)): ct = bpf_skb_ct_lookup(ctx, NULL, 0, &opts,,
    if (ct)
    pub 0: return,
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(R4": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R4")
#[no_mangle]
pub unsafe extern "C" fn lookup_null_bpf_opts(ctx: *mut __sk_buff) -> c_int {
    int lookup_null_bpf_opts(struct __sk_buff *ctx)
    {
    pub {}: bpf_sock_tuple tup =,
    pub ct: *mut nf_conn,
    pub bpf_ct_opts___local)): ct = bpf_skb_ct_lookup(ctx, &tup, sizeof(tup.ipv4), NULL, sizeof(struct,
    if (ct)
    pub 0: return,
    }
    SEC("?xdp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R2": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R2")
#[no_mangle]
pub unsafe extern "C" fn xdp_lookup_null_bpf_tuple(ctx: *mut xdp_md) -> c_int {
    int xdp_lookup_null_bpf_tuple(struct xdp_md *ctx)
    {
    pub {}: bpf_ct_opts___local opts =,
    pub ct: *mut nf_conn,
    pub sizeof(opts)): ct = bpf_xdp_ct_lookup(ctx, NULL, 0, &opts,,
    if (ct)
    pub 0: return,
    }
    SEC("?xdp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R4": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R4")
#[no_mangle]
pub unsafe extern "C" fn xdp_lookup_null_bpf_opts(ctx: *mut xdp_md) -> c_int {
    int xdp_lookup_null_bpf_opts(struct xdp_md *ctx)
    {
    pub {}: bpf_sock_tuple tup =,
    pub ct: *mut nf_conn,
    pub bpf_ct_opts___local)): ct = bpf_xdp_ct_lookup(ctx, &tup, sizeof(tup.ipv4), NULL, sizeof(struct,
    if (ct)
    pub 0: return,
    }
    pub "GPL": char _license[] SEC("license") =,
