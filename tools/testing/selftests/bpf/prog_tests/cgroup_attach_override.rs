//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgroup_attach_override.c
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

    static char bpf_log_buf[BPF_LOG_BUF_SIZE];
#[no_mangle]
unsafe extern "C" fn prog_load(verdict: c_int) -> c_int {
    static int prog_load(int verdict)
    {
    struct bpf_insn prog[] = {
    BPF_MOV64_IMM(BPF_REG_0, verdict), /* r0 = verdict */
    BPF_EXIT_INSN(),
    };
    let mut insns_cnt: usize = ARRAY_SIZE(prog);
    return bpf_test_load_program(BPF_PROG_TYPE_CGROUP_SKB,
    prog, insns_cnt, "GPL", 0,
    bpf_log_buf, BPF_LOG_BUF_SIZE);
    }
#[no_mangle]
pub unsafe extern "C" fn serial_test_cgroup_attach_override() {
    void serial_test_cgroup_attach_override(void)
    {
    let mut drop_prog: c_int = -1, allow_prog = -1, foo = -1, bar = -1;
    let mut duration: __u32 = 0;
    allow_prog = prog_load(1);
    if (CHECK(allow_prog < 0, "prog_load_allow",
    "verifier output:\n%s\n-------\n", bpf_log_buf))
    goto err;
    drop_prog = prog_load(0);
    if (CHECK(drop_prog < 0, "prog_load_drop",
    "verifier output:\n%s\n-------\n", bpf_log_buf))
    goto err;
    foo = test__join_cgroup(FOO);
    if (CHECK(foo < 0, "cgroup_join_foo", "cgroup setup failed\n"))
    goto err;
    if (CHECK(bpf_prog_attach(drop_prog, foo, BPF_CGROUP_INET_EGRESS,
    BPF_F_ALLOW_OVERRIDE),
    "prog_attach_drop_foo_override",
    "attach prog to %s failed, errno=%d\n", FOO, errno))
    goto err;
    if (CHECK(!system(PING_CMD), "ping_fail",
    "ping unexpectedly succeeded\n"))
    goto err;
    bar = test__join_cgroup(BAR);
    if (CHECK(bar < 0, "cgroup_join_bar", "cgroup setup failed\n"))
    goto err;
    if (CHECK(!system(PING_CMD), "ping_fail",
    "ping unexpectedly succeeded\n"))
    goto err;
    if (CHECK(bpf_prog_attach(allow_prog, bar, BPF_CGROUP_INET_EGRESS,
    BPF_F_ALLOW_OVERRIDE),
    "prog_attach_allow_bar_override",
    "attach prog to %s failed, errno=%d\n", BAR, errno))
    goto err;
    if (CHECK(system(PING_CMD), "ping_ok", "ping failed\n"))
    goto err;
    if (CHECK(bpf_prog_detach(bar, BPF_CGROUP_INET_EGRESS),
    "prog_detach_bar",
    "detach prog from %s failed, errno=%d\n", BAR, errno))
    goto err;
    if (CHECK(!system(PING_CMD), "ping_fail",
    "ping unexpectedly succeeded\n"))
    goto err;
    if (CHECK(bpf_prog_attach(allow_prog, bar, BPF_CGROUP_INET_EGRESS,
    BPF_F_ALLOW_OVERRIDE),
    "prog_attach_allow_bar_override",
    "attach prog to %s failed, errno=%d\n", BAR, errno))
    goto err;
    if (CHECK(bpf_prog_detach(foo, BPF_CGROUP_INET_EGRESS),
    "prog_detach_foo",
    "detach prog from %s failed, errno=%d\n", FOO, errno))
    goto err;
    if (CHECK(system(PING_CMD), "ping_ok", "ping failed\n"))
    goto err;
    if (CHECK(bpf_prog_attach(allow_prog, bar, BPF_CGROUP_INET_EGRESS,
    BPF_F_ALLOW_OVERRIDE),
    "prog_attach_allow_bar_override",
    "attach prog to %s failed, errno=%d\n", BAR, errno))
    goto err;
    if (CHECK(!bpf_prog_attach(allow_prog, bar, BPF_CGROUP_INET_EGRESS, 0),
    "fail_prog_attach_allow_bar_none",
    "attach prog to %s unexpectedly succeeded\n", BAR))
    goto err;
    if (CHECK(bpf_prog_detach(bar, BPF_CGROUP_INET_EGRESS),
    "prog_detach_bar",
    "detach prog from %s failed, errno=%d\n", BAR, errno))
    goto err;
    if (CHECK(!bpf_prog_detach(foo, BPF_CGROUP_INET_EGRESS),
    "fail_prog_detach_foo",
    "double detach from %s unexpectedly succeeded\n", FOO))
    goto err;
    if (CHECK(bpf_prog_attach(allow_prog, foo, BPF_CGROUP_INET_EGRESS, 0),
    "prog_attach_allow_foo_none",
    "attach prog to %s failed, errno=%d\n", FOO, errno))
    goto err;
    if (CHECK(!bpf_prog_attach(allow_prog, bar, BPF_CGROUP_INET_EGRESS, 0),
    "fail_prog_attach_allow_bar_none",
    "attach prog to %s unexpectedly succeeded\n", BAR))
    goto err;
    if (CHECK(!bpf_prog_attach(allow_prog, bar, BPF_CGROUP_INET_EGRESS,
    BPF_F_ALLOW_OVERRIDE),
    "fail_prog_attach_allow_bar_override",
    "attach prog to %s unexpectedly succeeded\n", BAR))
    goto err;
    if (CHECK(!bpf_prog_attach(allow_prog, foo, BPF_CGROUP_INET_EGRESS,
    BPF_F_ALLOW_OVERRIDE),
    "fail_prog_attach_allow_foo_override",
    "attach prog to %s unexpectedly succeeded\n", FOO))
    goto err;
    if (CHECK(bpf_prog_attach(drop_prog, foo, BPF_CGROUP_INET_EGRESS, 0),
    "prog_attach_drop_foo_none",
    "attach prog to %s failed, errno=%d\n", FOO, errno))
    goto err;
    err:
    close(foo);
    close(bar);
    close(allow_prog);
    close(drop_prog);
    }
