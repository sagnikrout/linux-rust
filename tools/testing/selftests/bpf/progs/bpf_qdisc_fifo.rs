//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_qdisc_fifo.c
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

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skb_node {
    pub skb: *mut *mut sk_buff __kptr,
    pub node: bpf_list_node,
}

    private(A) struct bpf_spin_lock q_fifo_lock;
    private(A) struct bpf_list_head q_fifo __contains(skb_node, node);
    bool init_called;
    SEC("struct_ops/bpf_fifo_enqueue")
    int BPF_PROG(bpf_fifo_enqueue, struct sk_buff *skb, struct Qdisc *sch,
    struct bpf_sk_buff_ptr *to_free)
    {
    struct skb_node *skbn;
    u32 pkt_len;
    if (sch.q.qlen == sch.limit)
    goto drop;
    skbn = bpf_obj_new(typeof(*skbn));
    if (!skbn)
    goto drop;
    pkt_len = qdisc_pkt_len(skb);
    sch.q.qlen++;
    skb = bpf_kptr_xchg(&skbn.skb, skb);
    if (skb)
    bpf_qdisc_skb_drop(skb, to_free);
    bpf_spin_lock(&q_fifo_lock);
    bpf_list_push_back(&q_fifo, &skbn.node);
    bpf_spin_unlock(&q_fifo_lock);
    sch.qstats.backlog += pkt_len;
    return NET_XMIT_SUCCESS;
    drop:
    bpf_qdisc_skb_drop(skb, to_free);
    return NET_XMIT_DROP;
    }
    SEC("struct_ops/bpf_fifo_dequeue")
    struct sk_buff *BPF_PROG(bpf_fifo_dequeue, struct Qdisc *sch)
    {
    struct bpf_list_node *node;
    struct sk_buff *skb = core::ptr::null_mut();
    struct skb_node *skbn;
    bpf_spin_lock(&q_fifo_lock);
    node = bpf_list_pop_front(&q_fifo);
    bpf_spin_unlock(&q_fifo_lock);
    if (!node)
    return core::ptr::null_mut();
    skbn = container_of(node, struct skb_node, node);
    skb = bpf_kptr_xchg(&skbn.skb, skb);
    bpf_obj_drop(skbn);
    if (!skb)
    return core::ptr::null_mut();
    sch.qstats.backlog -= qdisc_pkt_len(skb);
    bpf_qdisc_bstats_update(sch, skb);
    sch.q.qlen--;
    return skb;
    }
    SEC("struct_ops/bpf_fifo_init")
    int BPF_PROG(bpf_fifo_init, struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    sch.limit = 1000;
    init_called = true;
    return 0;
    }
    SEC("struct_ops/bpf_fifo_reset")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_fifo_reset, sch: *mut Qdisc) {
    void BPF_PROG(bpf_fifo_reset, struct Qdisc *sch)
    {
    struct bpf_list_node *node;
    struct skb_node *skbn;
    int i;
    bpf_for(i, 0, sch.q.qlen) {
    struct sk_buff *skb = core::ptr::null_mut();
    bpf_spin_lock(&q_fifo_lock);
    node = bpf_list_pop_front(&q_fifo);
    bpf_spin_unlock(&q_fifo_lock);
    if (!node)
    break;
    skbn = container_of(node, struct skb_node, node);
    skb = bpf_kptr_xchg(&skbn.skb, skb);
    if (skb)
    bpf_kfree_skb(skb);
    bpf_obj_drop(skbn);
    }
    sch.q.qlen = 0;
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_fifo_destroy, sch: *mut Qdisc) {
    void BPF_PROG(bpf_fifo_destroy, struct Qdisc *sch)
    {
    }
    SEC(".struct_ops")
    struct Qdisc_ops fifo = {
    .enqueue   = (void *)bpf_fifo_enqueue,
    .dequeue   = (void *)bpf_fifo_dequeue,
    .init      = (void *)bpf_fifo_init,
    .reset     = (void *)bpf_fifo_reset,
    .destroy   = (void *)bpf_fifo_destroy,
    .id        = "bpf_fifo",
    };
