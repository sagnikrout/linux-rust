//! Automatically rewritten from C to Rust
//! Source: samples/bpf/tracex1.bpf.c
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


// Copyright (c) 2013-2015 PLUMgrid, http://plumgrid.com
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

// kprobe is NOT a stable ABI
// kernel functions can be removed, renamed or completely change semantics.
// Number of arguments and their positions can change, etc.
// In such case this bpf+kprobe example will no longer be meaningful
//
    SEC("kprobe.multi/__netif_receive_skb_core*")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut pt_regs) -> c_int {
    int bpf_prog1(struct pt_regs *ctx)
    {
// attaches to kprobe __netif_receive_skb_core,
// looks for packets on loopback device and prints them
// (wildcard is used for avoiding symbol mismatch due to optimization)
//
    char devname[IFNAMSIZ];
    struct net_device *dev;
    struct sk_buff *skb;
    int len;
    bpf_core_read(&skb, sizeof(skb), (void *)PT_REGS_PARM1(ctx));
    dev = BPF_CORE_READ(skb, dev);
    len = BPF_CORE_READ(skb, len);
    BPF_CORE_READ_STR_INTO(&devname, dev, name);
    if (devname[0] == 'l' && devname[1] == 'o') {
    char fmt[] = "skb %p len %d\n";
// using bpf_trace_printk() for DEBUG ONLY
    bpf_trace_printk(fmt, sizeof(fmt), skb, len);
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
    u32 _version SEC("version") = LINUX_VERSION_CODE;
