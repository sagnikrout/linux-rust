//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_xdp_do_redirect.c
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

pub const ETH_ALEN: c_int = 6;

//
// enum frame_mark - magics to distinguish page/packet paths
// @MARK_XMIT: page was recycled due to the frame being "xmitted" by the NIC.
// @MARK_IN: frame is being processed by the input XDP prog.
// @MARK_SKB: frame did hit the TC ingress hook as an skb.
//
    enum frame_mark {
    MARK_XMIT	= 0U,
    MARK_IN		= 0x42,
    MARK_SKB	= 0x45,
    };
    const volatile int ifindex_out;
    const volatile int ifindex_in;
    const volatile __u8 expect_dst[ETH_ALEN];
    let mut pkts_seen_xdp: volatile int = 0;
    let mut pkts_seen_zero: volatile int = 0;
    let mut pkts_seen_tc: volatile int = 0;
    let mut retcode: volatile int = XDP_REDIRECT;
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_redirect(xdp: *mut xdp_md) -> c_int {
    int xdp_redirect(struct xdp_md *xdp)
    {
    __u32 *metadata = (void *)(long)xdp.data_meta;
    void *data_end = (void *)(long)xdp.data_end;
    void *data = (void *)(long)xdp.data;
    __u8 *payload = data + HDR_SZ;
    let mut ret: c_int = retcode;
    if (payload + 1 > data_end)
    return XDP_ABORTED;
    if (xdp.ingress_ifindex != (__u32)ifindex_in)
    return XDP_ABORTED;
    if (metadata + 1 > data)
    return XDP_ABORTED;
    if (*metadata != 0x42)
    return XDP_ABORTED;
    if (*payload == MARK_XMIT)
    pkts_seen_zero++;
// payload = MARK_IN;
    if (bpf_xdp_adjust_meta(xdp, sizeof(__u64)))
    return XDP_ABORTED;
    if (retcode > XDP_PASS)
    retcode--;
    if (ret == XDP_REDIRECT)
    return bpf_redirect(ifindex_out, 0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn check_pkt(data: *mut c_void, data_end: *mut c_void, mark: __u32) -> bool {
    static bool check_pkt(void *data, void *data_end, const __u32 mark)
    {
    struct ipv6hdr *iph = data + sizeof(struct ethhdr);
    __u8 *payload = data + HDR_SZ;
    if (payload + 1 > data_end)
    return false;
    if (iph.nexthdr != IPPROTO_UDP || *payload != MARK_IN)
    return false;
// reset the payload so the same packet doesn't get counted twice when
// it cycles back through the kernel path and out the dst veth
//
// payload = mark;
    return true;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_count_pkts(xdp: *mut xdp_md) -> c_int {
    int xdp_count_pkts(struct xdp_md *xdp)
    {
    void *data = (void *)(long)xdp.data;
    void *data_end = (void *)(long)xdp.data_end;
    if (check_pkt(data, data_end, MARK_XMIT))
    pkts_seen_xdp++;
// Return %XDP_DROP to recycle the data page with %MARK_XMIT, like
// it exited a physical NIC. Those pages will be counted in the
// pkts_seen_zero counter above.
//
    return XDP_DROP;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_redirect_to_111(xdp: *mut xdp_md) -> c_int {
    int xdp_redirect_to_111(struct xdp_md *xdp)
    {
    return bpf_redirect(111, 0);
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_redirect_to_222(xdp: *mut xdp_md) -> c_int {
    int xdp_redirect_to_222(struct xdp_md *xdp)
    {
    return bpf_redirect(222, 0);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_count_pkts(skb: *mut __sk_buff) -> c_int {
    int tc_count_pkts(struct __sk_buff *skb)
    {
    void *data = (void *)(long)skb.data;
    void *data_end = (void *)(long)skb.data_end;
    if (check_pkt(data, data_end, MARK_SKB))
    pkts_seen_tc++;
// Will be either recycled or freed, %MARK_SKB makes sure it won't
// hit any of the counters above.
//
    return 0;
    }
    char _license[] SEC("license") = "GPL";
