//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/xsk_xdp_progs.c
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
// Copyright (c) 2022 Intel

    struct {
    __uint(type, BPF_MAP_TYPE_XSKMAP);
    __uint(max_entries, 2);
    __uint(key_size, sizeof(int));
    __uint(value_size, sizeof(int));
    } xsk SEC(".maps");
    static unsigned int idx;
    let mut adjust_value: c_int = 0;
    let mut count: c_int = 0;
    SEC("xdp.frags") int xsk_def_prog(struct xdp_md *xdp)
    {
    return bpf_redirect_map(&xsk, 0, XDP_DROP);
    }
    SEC("xdp.frags") int xsk_xdp_drop(struct xdp_md *xdp)
    {
    static unsigned int drop_idx;
// Drop every other packet
    if (drop_idx++ % 2)
    return XDP_DROP;
    return bpf_redirect_map(&xsk, 0, XDP_DROP);
    }
    SEC("xdp.frags") int xsk_xdp_populate_metadata(struct xdp_md *xdp)
    {
    void *data, *data_meta;
    struct xdp_info *meta;
    int err;
// Reserve enough for all custom metadata.
    err = bpf_xdp_adjust_meta(xdp, -(int)sizeof(struct xdp_info));
    if (err)
    return XDP_DROP;
    data = (void *)(long)xdp.data;
    data_meta = (void *)(long)xdp.data_meta;
    if (data_meta + sizeof(struct xdp_info) > data)
    return XDP_DROP;
    meta = data_meta;
    meta.count = count++;
    return bpf_redirect_map(&xsk, 0, XDP_DROP);
    }
    SEC("xdp") int xsk_xdp_shared_umem(struct xdp_md *xdp)
    {
    void *data = (void *)(long)xdp.data;
    void *data_end = (void *)(long)xdp.data_end;
    struct ethhdr *eth = data;
    if (eth + 1 > data_end)
    return XDP_DROP;
// Redirecting packets based on the destination MAC address
    idx = ((unsigned int)(eth.h_dest[5])) / 2;
    if (idx > MAX_SOCKETS)
    return XDP_DROP;
    return bpf_redirect_map(&xsk, idx, XDP_DROP);
    }
    SEC("xdp.frags") int xsk_xdp_adjust_tail(struct xdp_md *xdp)
    {
    __u32 buff_len, curr_buff_len;
    int ret;
    buff_len = bpf_xdp_get_buff_len(xdp);
    if (buff_len == 0)
    return XDP_DROP;
    ret = bpf_xdp_adjust_tail(xdp, adjust_value);
    if (ret < 0) {
// Handle unsupported cases
    if (ret == -EOPNOTSUPP) {
// Set adjust_value to -EOPNOTSUPP to indicate to userspace that this case
// is unsupported
//
    adjust_value = -EOPNOTSUPP;
    return bpf_redirect_map(&xsk, 0, XDP_DROP);
    }
    return XDP_DROP;
    }
    curr_buff_len = bpf_xdp_get_buff_len(xdp);
    if (curr_buff_len != buff_len + adjust_value)
    return XDP_DROP;
    if (curr_buff_len > buff_len) {
    __u32 *pkt_data = (void *)(long)xdp.data;
    __u32 len, words_to_end, seq_num;
    len = curr_buff_len - PKT_HDR_ALIGN;
    words_to_end = len / sizeof(*pkt_data) - 1;
    seq_num = words_to_end;
// Convert sequence number to network byte order. Store this in the last 4 bytes of
// the packet. Use 'adjust_value' to determine the position at the end of the
// packet for storing the sequence number.
//
    seq_num = __constant_htonl(words_to_end);
    bpf_xdp_store_bytes(xdp, curr_buff_len - sizeof(seq_num), &seq_num,
    sizeof(seq_num));
    }
    return bpf_redirect_map(&xsk, 0, XDP_DROP);
    }
    char _license[] SEC("license") = "GPL";
