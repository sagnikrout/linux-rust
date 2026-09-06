//! Automatically rewritten from C to Rust
//! Source: samples/bpf/xdp2skb_meta_kern.c
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
// Copyright (c) 2018 Jesper Dangaard Brouer, Red Hat Inc.
//
// Example howto transfer info from XDP to SKB, e.g. skb->mark
// -----------------------------------------------------------
// This uses the XDP data_meta infrastructure, and is a cooperation
// between two bpf-programs (1) XDP and (2) clsact at TC-ingress hook.
//
// Notice: This example does not use the BPF C-loader,
// but instead rely on the iproute2 TC tool for loading BPF-objects.
//

//
// This struct is stored in the XDP 'data_meta' area, which is located
// just in-front-of the raw packet payload data.  The meaning is
// specific to these two BPF programs that use it as a communication
// channel.  XDP adjust/increase the area via a bpf-helper, and TC use
// boundary checks to see if data have been provided.
//
// The struct must be 4 byte aligned, which here is enforced by the
// struct __attribute__((aligned(4))).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meta_info {
    pub mark: __u32,
    pub __attribute__((aligned(4))): },
    SEC("xdp_mark")
#[no_mangle]
pub unsafe extern "C" fn _xdp_mark(ctx: *mut xdp_md) -> c_int {
    int _xdp_mark(struct xdp_md *ctx)
    {
    pub meta: *mut meta_info,
    pub data: *mut c_void,
    pub ret: c_int,
// Reserve space in-front of data pointer for our meta info.
// (Notice drivers not supporting data_meta will fail here!)
//
    pub -(int)sizeof(*meta)): *mut ret = bpf_xdp_adjust_meta(ctx,,
    if (ret < 0)
    pub XDP_ABORTED: return,
// Notice: Kernel-side verifier requires that loading of
// ctx->data MUST happen _after_ helper bpf_xdp_adjust_meta(),
// as pkt-data pointers are invalidated.  Helpers that require
// this are determined/marked by bpf_helper_changes_pkt_data()
//
    pub long)ctx->data: *mut *mut data = (void )(unsigned,
// Check data_meta have room for meta_info struct
    pub long)ctx->data_meta: *mut *mut meta = (void )(unsigned,
    if (meta + 1 > data)
    pub XDP_ABORTED: return,
    pub 42: meta->mark =,
    pub XDP_PASS: return,
    }
    SEC("tc_mark")
#[no_mangle]
pub unsafe extern "C" fn _tc_mark(ctx: *mut __sk_buff) -> c_int {
    int _tc_mark(struct __sk_buff *ctx)
    {
    pub long)ctx->data: *mut *mut *mut void data = (void )(unsigned,
    pub long)ctx->data_meta: *mut *mut *mut void data_meta = (void )(unsigned,
    pub data_meta: *mut *mut meta_info meta =,
// Check XDP gave us some data_meta
    if (meta + 1 > data) {
    pub 41: ctx->mark =,
// Skip "accept" if no data_meta is avail
    pub TC_ACT_OK: return,
    }
// Hint: See func tc_cls_act_is_valid_access() for BPF_WRITE access
    pub /: *mut *mut ctx->mark = meta->mark; / Transfer XDP-mark to SKB-mark,
    pub TC_ACT_OK: return,
    }
// Manually attaching these programs:
    export DEV=ixgbe2
    export FILE=xdp2skb_meta_kern.o

    tc qdisc del dev $DEV clsact 2> /dev/null
    tc qdisc add dev $DEV clsact
    tc filter  add dev $DEV ingress prio 1 handle 1 bpf da obj $FILE sec tc_mark
    tc filter show dev $DEV ingress

    ip link set dev $DEV xdp off
    ip link set dev $DEV xdp obj $FILE sec xdp_mark

    iptables -I INPUT -p icmp -m mark --mark 41  # == 0x29
    iptables -I INPUT -p icmp -m mark --mark 42  # == 0x2a

    perf record -e xdp:*
    perf script
//
