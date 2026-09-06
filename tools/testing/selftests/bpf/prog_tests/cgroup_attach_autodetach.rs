//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgroup_attach_autodetach.c
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
unsafe extern "C" fn prog_load() -> c_int {
    static int prog_load(void)
    {
    struct bpf_insn prog[] = {
    BPF_MOV64_IMM(BPF_REG_0, 1), /* r0 = 1 */
    BPF_EXIT_INSN(),
    };
    let mut insns_cnt: usize = ARRAY_SIZE(prog);
    return bpf_test_load_program(BPF_PROG_TYPE_CGROUP_SKB,
    prog, insns_cnt, "GPL", 0,
    bpf_log_buf, BPF_LOG_BUF_SIZE);
    }
#[no_mangle]
pub unsafe extern "C" fn serial_test_cgroup_attach_autodetach() {
    void serial_test_cgroup_attach_autodetach(void)
    {
    let mut duration: __u32 = 0, prog_cnt = 4, attach_flags;
    int allow_prog[2] = {-1};
    __u32 prog_ids[2] = {0};
    void *ptr = core::ptr::null_mut();
    let mut cg: c_int = 0, i;
    int attempts;
    for (i = 0; i < ARRAY_SIZE(allow_prog); i++) {
    allow_prog[i] = prog_load();
    if (CHECK(allow_prog[i] < 0, "prog_load",
    "verifier output:\n%s\n-------\n", bpf_log_buf))
    goto err;
    }
    if (CHECK_FAIL(setup_cgroup_environment()))
    goto err;
// create a cgroup, attach two programs and remember their ids
    cg = create_and_get_cgroup("/cg_autodetach");
    if (CHECK_FAIL(cg < 0))
    goto err;
    if (CHECK_FAIL(join_cgroup("/cg_autodetach")))
    goto err;
    for (i = 0; i < ARRAY_SIZE(allow_prog); i++)
    if (CHECK(bpf_prog_attach(allow_prog[i], cg,
    BPF_CGROUP_INET_EGRESS,
    BPF_F_ALLOW_MULTI),
    "prog_attach", "prog[%d], errno=%d\n", i, errno))
    goto err;
// make sure that programs are attached and run some traffic
    if (CHECK(bpf_prog_query(cg, BPF_CGROUP_INET_EGRESS, 0, &attach_flags,
    prog_ids, &prog_cnt),
    "prog_query", "errno=%d\n", errno))
    goto err;
    if (CHECK_FAIL(system(PING_CMD)))
    goto err;
// allocate some memory (4Mb) to pin the original cgroup
    ptr = malloc(4 * (1 << 20));
    if (CHECK_FAIL(!ptr))
    goto err;
// close programs and cgroup fd
    for (i = 0; i < ARRAY_SIZE(allow_prog); i++) {
    close(allow_prog[i]);
    allow_prog[i] = -1;
    }
    close(cg);
    cg = 0;
// leave the cgroup and remove it. don't detach programs
    cleanup_cgroup_environment();
// wait for the asynchronous auto-detachment.
// wait for no more than 5 sec and give up.
//
    for (i = 0; i < ARRAY_SIZE(prog_ids); i++) {
    for (attempts = 5; attempts >= 0; attempts--) {
    let mut fd: c_int = bpf_prog_get_fd_by_id(prog_ids[i]);
    if (fd < 0)
    break;
// don't leave the fd open
    close(fd);
    if (CHECK_FAIL(!attempts))
    goto err;
    sleep(1);
    }
    }
    err:
    for (i = 0; i < ARRAY_SIZE(allow_prog); i++)
    if (allow_prog[i] >= 0)
    close(allow_prog[i]);
    if (cg)
    close(cg);
    free(ptr);
    cleanup_cgroup_environment();
    }
