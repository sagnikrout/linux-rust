//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/xdp_cpumap_attach.c
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

pub const IFINDEX_LO: c_int = 1;

#[no_mangle]
unsafe extern "C" fn test_xdp_with_cpumap_helpers() {
    static void test_xdp_with_cpumap_helpers(void)
    {
    struct test_xdp_with_cpumap_helpers *skel = core::ptr::null_mut();
    let mut info: bpf_prog_info = {};
    let mut len: __u32 = sizeof(info);
    struct bpf_cpumap_val val = {
    .qsize = 192,
    };
    int err, prog_fd, prog_redir_fd, map_fd, bad_fd;
    struct nstoken *nstoken = core::ptr::null_mut();
    let mut idx: __u32 = 0;
    SYS(out_close, "ip netns add %s", TEST_NS);
    nstoken = open_netns(TEST_NS);
    if (!ASSERT_OK_PTR(nstoken, "open_netns"))
    goto out_close;
    SYS(out_close, "ip link set dev lo up");
    skel = test_xdp_with_cpumap_helpers__open_and_load();
    if (!ASSERT_OK_PTR(skel, "test_xdp_with_cpumap_helpers__open_and_load"))
    return;
    prog_redir_fd = bpf_program__fd(skel.progs.xdp_redir_prog);
    err = bpf_xdp_attach(IFINDEX_LO, prog_redir_fd, XDP_FLAGS_SKB_MODE, core::ptr::null_mut());
    if (!ASSERT_OK(err, "Generic attach of program with 8-byte CPUMAP"))
    goto out_close;
    prog_fd = bpf_program__fd(skel.progs.xdp_dummy_cm);
    map_fd = bpf_map__fd(skel.maps.cpu_map);
    err = bpf_prog_get_info_by_fd(prog_fd, &info, &len);
    if (!ASSERT_OK(err, "bpf_prog_get_info_by_fd"))
    goto out_close;
    val.bpf_prog.fd = prog_fd;
    err = bpf_map_update_elem(map_fd, &idx, &val, 0);
    ASSERT_OK(err, "Add program to cpumap entry");
    err = bpf_map_lookup_elem(map_fd, &idx, &val);
    ASSERT_OK(err, "Read cpumap entry");
    ASSERT_EQ(info.id, val.bpf_prog.id, "Match program id to cpumap entry prog_id");
// send a packet to trigger any potential bugs in there
    char data[ETH_HLEN] = {};
    DECLARE_LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &data,
    .data_size_in = sizeof(data),
    .flags = BPF_F_TEST_XDP_LIVE_FRAMES,
    .repeat = 1,
    );
    err = bpf_prog_test_run_opts(prog_redir_fd, &opts);
    ASSERT_OK(err, "XDP test run");
// wait for the packets to be flushed, then check that redirect has been
// performed
//
    kern_sync_rcu();
    ASSERT_NEQ(skel.bss.redirect_count, 0, "redirected packets");
    err = bpf_xdp_detach(IFINDEX_LO, XDP_FLAGS_SKB_MODE, core::ptr::null_mut());
    ASSERT_OK(err, "XDP program detach");
// can not attach BPF_XDP_CPUMAP program to a device
    err = bpf_xdp_attach(IFINDEX_LO, prog_fd, XDP_FLAGS_SKB_MODE, core::ptr::null_mut());
    if (!ASSERT_NEQ(err, 0, "Attach of BPF_XDP_CPUMAP program"))
    bpf_xdp_detach(IFINDEX_LO, XDP_FLAGS_SKB_MODE, core::ptr::null_mut());
    val.qsize = 192;
    val.bpf_prog.fd = bpf_program__fd(skel.progs.xdp_dummy_prog);
    err = bpf_map_update_elem(map_fd, &idx, &val, 0);
    ASSERT_EQ(err, -EINVAL, "Add non-BPF_XDP_CPUMAP program to cpumap entry");
// Try to attach non-BPF file descriptor
    bad_fd = open("/dev/null", O_RDONLY);
    ASSERT_GE(bad_fd, 0, "Open /dev/null for non-BPF fd");
    val.bpf_prog.fd = bad_fd;
    err = bpf_map_update_elem(map_fd, &idx, &val, 0);
    ASSERT_EQ(err, -EINVAL, "Add non-BPF fd to cpumap entry");
// Try to attach nonexistent file descriptor
    err = close(bad_fd);
    ASSERT_EQ(err, 0, "Close non-BPF fd for nonexistent fd");
    err = bpf_map_update_elem(map_fd, &idx, &val, 0);
    ASSERT_EQ(err, -EBADF, "Add nonexistent fd to cpumap entry");
// Try to attach BPF_XDP program with frags to cpumap when we have
// already loaded a BPF_XDP program on the map
//
    idx = 1;
    val.qsize = 192;
    val.bpf_prog.fd = bpf_program__fd(skel.progs.xdp_dummy_cm_frags);
    err = bpf_map_update_elem(map_fd, &idx, &val, 0);
    ASSERT_NEQ(err, 0, "Add BPF_XDP program with frags to cpumap entry");
    out_close:
    close_netns(nstoken);
    SYS_NOFAIL("ip netns del %s", TEST_NS);
    test_xdp_with_cpumap_helpers__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_xdp_with_cpumap_frags_helpers() {
    static void test_xdp_with_cpumap_frags_helpers(void)
    {
    struct test_xdp_with_cpumap_frags_helpers *skel;
    let mut info: bpf_prog_info = {};
    let mut len: __u32 = sizeof(info);
    struct bpf_cpumap_val val = {
    .qsize = 192,
    };
    int err, frags_prog_fd, map_fd;
    let mut idx: __u32 = 0;
    skel = test_xdp_with_cpumap_frags_helpers__open_and_load();
    if (!ASSERT_OK_PTR(skel, "test_xdp_with_cpumap_helpers__open_and_load"))
    return;
    frags_prog_fd = bpf_program__fd(skel.progs.xdp_dummy_cm_frags);
    map_fd = bpf_map__fd(skel.maps.cpu_map);
    err = bpf_prog_get_info_by_fd(frags_prog_fd, &info, &len);
    if (!ASSERT_OK(err, "bpf_prog_get_info_by_fd"))
    goto out_close;
    val.bpf_prog.fd = frags_prog_fd;
    err = bpf_map_update_elem(map_fd, &idx, &val, 0);
    ASSERT_OK(err, "Add program to cpumap entry");
    err = bpf_map_lookup_elem(map_fd, &idx, &val);
    ASSERT_OK(err, "Read cpumap entry");
    ASSERT_EQ(info.id, val.bpf_prog.id,
    "Match program id to cpumap entry prog_id");
// Try to attach BPF_XDP program to cpumap when we have
// already loaded a BPF_XDP program with frags on the map
//
    idx = 1;
    val.qsize = 192;
    val.bpf_prog.fd = bpf_program__fd(skel.progs.xdp_dummy_cm);
    err = bpf_map_update_elem(map_fd, &idx, &val, 0);
    ASSERT_NEQ(err, 0, "Add BPF_XDP program to cpumap entry");
    out_close:
    test_xdp_with_cpumap_frags_helpers__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_xdp_cpumap_attach() {
    void test_xdp_cpumap_attach(void)
    {
    if (test__start_subtest("CPUMAP with programs in entries"))
    test_xdp_with_cpumap_helpers();
    if (test__start_subtest("CPUMAP with frags programs in entries"))
    test_xdp_with_cpumap_frags_helpers();
    }
