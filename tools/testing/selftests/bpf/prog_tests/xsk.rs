//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/xsk.c
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

pub const MTU: c_int = 1500;
#[no_mangle]
pub unsafe extern "C" fn setup_veth(busy_poll: bool) -> c_int {
    int setup_veth(bool busy_poll)
    {
    SYS(fail,
    "ip link add %s numtxqueues 4 numrxqueues 4 type veth peer name %s numtxqueues 4 numrxqueues 4",
    VETH_RX, VETH_TX);
    SYS(fail, "sysctl -wq net.ipv6.conf.%s.disable_ipv6=1", VETH_RX);
    SYS(fail, "sysctl -wq net.ipv6.conf.%s.disable_ipv6=1", VETH_TX);
    if (busy_poll) {
    SYS(fail, "echo 2 > /sys/class/net/%s/napi_defer_hard_irqs", VETH_RX);
    SYS(fail, "echo 200000 > /sys/class/net/%s/gro_flush_timeout", VETH_RX);
    SYS(fail, "echo 2 > /sys/class/net/%s/napi_defer_hard_irqs", VETH_TX);
    SYS(fail, "echo 200000 > /sys/class/net/%s/gro_flush_timeout", VETH_TX);
    }
    SYS(fail, "ip link set %s mtu %d", VETH_RX, MTU);
    SYS(fail, "ip link set %s mtu %d", VETH_TX, MTU);
    SYS(fail, "ip link set %s up", VETH_RX);
    SYS(fail, "ip link set %s up", VETH_TX);
    return 0;
    fail:
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn delete_veth() {
    void delete_veth(void)
    {
    SYS_NOFAIL("ip link del %s", VETH_RX);
    SYS_NOFAIL("ip link del %s", VETH_TX);
    }
#[no_mangle]
pub unsafe extern "C" fn configure_ifobj(tx: *mut ifobject, rx: *mut ifobject) -> c_int {
    int configure_ifobj(struct ifobject *tx, struct ifobject *rx)
    {
    rx.ifindex = if_nametoindex(VETH_RX);
    if (!ASSERT_OK_FD(rx.ifindex, "get RX ifindex"))
    return -1;
    tx.ifindex = if_nametoindex(VETH_TX);
    if (!ASSERT_OK_FD(tx.ifindex, "get TX ifindex"))
    return -1;
    tx.shared_umem = false;
    rx.shared_umem = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_xsk(test_to_run: *const test_spec, mode: enum test_mode) {
    static void test_xsk(const struct test_spec *test_to_run, enum test_mode mode)
    {
    u32 max_frags, umem_tailroom, cache_line_size;
    struct ifobject *ifobj_tx, *ifobj_rx;
    struct test_spec test;
    int ret;
    ifobj_tx = ifobject_create();
    if (!ASSERT_OK_PTR(ifobj_tx, "create ifobj_tx"))
    return;
    ifobj_rx = ifobject_create();
    if (!ASSERT_OK_PTR(ifobj_rx, "create ifobj_rx"))
    goto delete_tx;
    if (!ASSERT_OK(configure_ifobj(ifobj_tx, ifobj_rx), "conigure ifobj"))
    goto delete_rx;
    ret = get_hw_ring_size(ifobj_tx.ifname, &ifobj_tx.ring);
    if (!ret) {
    ifobj_tx.hw_ring_size_supp = true;
    ifobj_tx.set_ring.default_tx = ifobj_tx.ring.tx_pending;
    ifobj_tx.set_ring.default_rx = ifobj_tx.ring.rx_pending;
    }
    cache_line_size = read_procfs_val(SMP_CACHE_BYTES_PATH);
    if (!cache_line_size)
    cache_line_size = 64;
    max_frags = read_procfs_val(MAX_SKB_FRAGS_PATH);
    if (!max_frags)
    max_frags = 17;
    ifobj_tx.max_skb_frags = max_frags;
    ifobj_rx.max_skb_frags = max_frags;
// 48 bytes is a part of skb_shared_info w/o frags array;
// 16 bytes is sizeof(skb_frag_t)
//
    umem_tailroom = ALIGN(48 + (max_frags * 16), cache_line_size);
    ifobj_tx.umem_tailroom = umem_tailroom;
    ifobj_rx.umem_tailroom = umem_tailroom;
    if (!ASSERT_OK(init_iface(ifobj_rx, worker_testapp_validate_rx), "init RX"))
    goto delete_rx;
    if (!ASSERT_OK(init_iface(ifobj_tx, worker_testapp_validate_tx), "init TX"))
    goto delete_rx;
    test_init(&test, ifobj_tx, ifobj_rx, 0, &tests[0]);
    test.tx_pkt_stream_default = pkt_stream_generate(DEFAULT_PKT_CNT, MIN_PKT_SIZE);
    if (!ASSERT_OK_PTR(test.tx_pkt_stream_default, "TX pkt generation"))
    goto delete_rx;
    test.rx_pkt_stream_default = pkt_stream_generate(DEFAULT_PKT_CNT, MIN_PKT_SIZE);
    if (!ASSERT_OK_PTR(test.rx_pkt_stream_default, "RX pkt generation"))
    goto delete_rx;
    test_init(&test, ifobj_tx, ifobj_rx, mode, test_to_run);
    ret = test.test_func(&test);
    if (ret != TEST_SKIP)
    ASSERT_OK(ret, "Run test");
    pkt_stream_restore_default(&test);
    if (ifobj_tx.hw_ring_size_supp)
    hw_ring_size_reset(ifobj_tx);
    pkt_stream_delete(test.tx_pkt_stream_default);
    pkt_stream_delete(test.rx_pkt_stream_default);
    xsk_xdp_progs__destroy(ifobj_tx.xdp_progs);
    xsk_xdp_progs__destroy(ifobj_rx.xdp_progs);
    delete_rx:
    ifobject_delete(ifobj_rx);
    delete_tx:
    ifobject_delete(ifobj_tx);
    }
#[no_mangle]
pub unsafe extern "C" fn test_ns_xsk_skb() {
    void test_ns_xsk_skb(void)
    {
    int i;
    if (!ASSERT_OK(setup_veth(false), "setup veth"))
    return;
    for (i = 0; i < ARRAY_SIZE(tests); i++) {
    if (test__start_subtest(tests[i].name))
    test_xsk(&tests[i], TEST_MODE_SKB);
    }
    delete_veth();
    }
#[no_mangle]
pub unsafe extern "C" fn test_ns_xsk_drv() {
    void test_ns_xsk_drv(void)
    {
    int i;
    if (!ASSERT_OK(setup_veth(false), "setup veth"))
    return;
    for (i = 0; i < ARRAY_SIZE(tests); i++) {
    if (test__start_subtest(tests[i].name))
    test_xsk(&tests[i], TEST_MODE_DRV);
    }
    delete_veth();
    }
