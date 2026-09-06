//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_xdp_adjust_tail_grow.c
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

    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn _xdp_adjust_tail_grow(xdp: *mut xdp_md) -> c_int {
    int _xdp_adjust_tail_grow(struct xdp_md *xdp)
    {
    let mut data_len: c_int = bpf_xdp_get_buff_len(xdp);
    let mut offset: c_int = 0;
// SKB_DATA_ALIGN(sizeof(struct skb_shared_info))

    let mut tailroom: c_int = 512;

    let mut tailroom: c_int = 384;

    let mut tailroom: c_int = 320;

// Data length determine test case
    if (data_len == 54) { /* sizeof(pkt_v4) */
    offset = 4096; /* test too large offset, 4k page size */
    } else if (data_len == 53) { /* sizeof(pkt_v4) - 1 */
    offset = 65536; /* test too large offset, 64k page size */
    } else if (data_len == 74) { /* sizeof(pkt_v6) */
    offset = 40;
    } else if (data_len == 64) {
    offset = 128;
    } else if (data_len == 128) {
// Max tail grow 3520
    offset = 4096 - 256 - tailroom - data_len;
    } else if (data_len == 9000) {
    offset = 10;
    } else if (data_len == 9001) {
    offset = 4096;
    } else if (data_len == 90000) {
    offset = 10; /* test a small offset, 64k page size */
    } else if (data_len == 90001) {
    offset = 65536; /* test too large offset, 64k page size */
    } else {
    return XDP_ABORTED; /* No matching test */
    }
    if (bpf_xdp_adjust_tail(xdp, offset))
    return XDP_DROP;
    return XDP_TX;
    }
    char _license[] SEC("license") = "GPL";
