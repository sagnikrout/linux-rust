//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/xdp_dev_bound_only.c
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

#[no_mangle]
unsafe extern "C" fn load_dummy_prog(name: *mut c_char, ifindex: __u32, flags: __u32) -> c_int {
    static int load_dummy_prog(char *name, __u32 ifindex, __u32 flags)
    {
    struct bpf_insn insns[] = { BPF_MOV64_IMM(BPF_REG_0, 0), BPF_EXIT_INSN() };
    LIBBPF_OPTS(bpf_prog_load_opts, opts);
    opts.prog_flags = flags;
    opts.prog_ifindex = ifindex;
    return bpf_prog_load(BPF_PROG_TYPE_XDP, name, "GPL", insns, ARRAY_SIZE(insns), &opts);
    }
// A test case for bpf_offload_netdev->offload handling bug:
// - create a veth device (does not support offload);
// - create a device bound XDP program with BPF_F_XDP_DEV_BOUND_ONLY flag
// (such programs are not offloaded);
// - create a device bound XDP program without flags (such programs are offloaded).
// This might lead to 'BUG: kernel NULL pointer dereference'.
//
#[no_mangle]
pub unsafe extern "C" fn test_xdp_dev_bound_only_offdev() {
    void test_xdp_dev_bound_only_offdev(void)
    {
    struct nstoken *tok = core::ptr::null_mut();
    __u32 ifindex;
    let mut fd1: c_int = -1;
    let mut fd2: c_int = -1;
    SYS(out, "ip netns add " LOCAL_NETNS);
    tok = open_netns(LOCAL_NETNS);
    if (!ASSERT_OK_PTR(tok, "open_netns"))
    goto out;
    SYS(out, "ip link add eth42 type veth");
    ifindex = if_nametoindex("eth42");
    if (!ASSERT_NEQ(ifindex, 0, "if_nametoindex")) {
    perror("if_nametoindex");
    goto out;
    }
    fd1 = load_dummy_prog("dummy1", ifindex, BPF_F_XDP_DEV_BOUND_ONLY);
    if (!ASSERT_GE(fd1, 0, "load_dummy_prog #1")) {
    perror("load_dummy_prog #1");
    goto out;
    }
// Program with ifindex is considered offloaded, however veth
// does not support offload => error should be reported.
//
    fd2 = load_dummy_prog("dummy2", ifindex, 0);
    ASSERT_EQ(fd2, -EINVAL, "load_dummy_prog #2 (offloaded)");
    out:
    close(fd1);
    close(fd2);
    close_netns(tok);
// eth42 was added inside netns, removing the netns will
// also remove eth42 veth pair.
//
    SYS_NOFAIL("ip netns del " LOCAL_NETNS);
    }
