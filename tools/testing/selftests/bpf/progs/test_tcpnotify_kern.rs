//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_tcpnotify_kern.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 4);
    __type(key, __u32);
    __type(value, struct tcpnotify_globals);
    } global_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __type(key, int);
    __type(value, __u32);
    } perf_event_map SEC(".maps");
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn bpf_testcb(skops: *mut bpf_sock_ops) -> c_int {
    int bpf_testcb(struct bpf_sock_ops *skops)
    {
    let mut rv: c_int = -1;
    int op;
    op = (int) skops.op;
    if (bpf_ntohl(skops.remote_port) != TESTPORT) {
    skops.reply = -1;
    return 0;
    }
    switch (op) {
    case BPF_SOCK_OPS_TIMEOUT_INIT:
    case BPF_SOCK_OPS_RWND_INIT:
    case BPF_SOCK_OPS_NEEDS_ECN:
    case BPF_SOCK_OPS_BASE_RTT:
    case BPF_SOCK_OPS_RTO_CB:
    rv = 1;
    break;
    case BPF_SOCK_OPS_TCP_CONNECT_CB:
    case BPF_SOCK_OPS_TCP_LISTEN_CB:
    case BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB:
    case BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB:
    bpf_sock_ops_cb_flags_set(skops, (BPF_SOCK_OPS_RETRANS_CB_FLAG|
    BPF_SOCK_OPS_RTO_CB_FLAG));
    rv = 1;
    break;
    case BPF_SOCK_OPS_RETRANS_CB: {
    let mut key: __u32 = 0;
    struct tcpnotify_globals g, *gp;
    struct tcp_notifier msg = {
    .type = 0xde,
    .subtype = 0xad,
    .source = 0xbe,
    .hash = 0xef,
    };
    rv = 1;
// Update results
    gp = bpf_map_lookup_elem(&global_map, &key);
    if (!gp)
    break;
    g = *gp;
    g.total_retrans = skops.total_retrans;
    g.ncalls++;
    bpf_map_update_elem(&global_map, &key, &g,
    BPF_ANY);
    bpf_perf_event_output(skops, &perf_event_map,
    BPF_F_CURRENT_CPU,
    &msg, sizeof(msg));
    }
    break;
    default:
    rv = -1;
    }
    skops.reply = rv;
    return 1;
    }
    char _license[] SEC("license") = "GPL";
