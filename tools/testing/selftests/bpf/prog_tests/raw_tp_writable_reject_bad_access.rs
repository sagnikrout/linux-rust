//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/raw_tp_writable_reject_bad_access.c
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
unsafe extern "C" fn check_attach_reject(program: *const bpf_insn, prog_len: usize) {
    static void check_attach_reject(const struct bpf_insn *program, size_t prog_len)
    {
    LIBBPF_OPTS(bpf_prog_load_opts, opts);
    char error[4096];
    int bpf_fd, tp_fd;
    opts.log_level = 2;
    opts.log_buf = error;
    opts.log_size = sizeof(error);
    bpf_fd = bpf_prog_load(BPF_PROG_TYPE_RAW_TRACEPOINT_WRITABLE, core::ptr::null_mut(), "GPL v2",
    program, prog_len, &opts);
    if (!ASSERT_GE(bpf_fd, 0, "prog_load"))
    return;
    tp_fd = bpf_raw_tracepoint_open("bpf_testmod_test_writable_bare_tp", bpf_fd);
    ASSERT_EQ(tp_fd, -EINVAL, "bpf_raw_tracepoint_open");
    if (tp_fd >= 0)
    close(tp_fd);
    close(bpf_fd);
    }
#[no_mangle]
pub unsafe extern "C" fn test_raw_tp_writable_reject_bad_access() {
    void test_raw_tp_writable_reject_bad_access(void)
    {
    const struct bpf_insn program[] = {
// r6 is our tp buffer
    BPF_LDX_MEM(BPF_DW, BPF_REG_6, BPF_REG_1, 0),
// one byte beyond the end of the writable context
    BPF_LDX_MEM(BPF_B, BPF_REG_0, BPF_REG_6,
    sizeof(struct bpf_testmod_test_writable_ctx)),
    BPF_EXIT_INSN(),
    };
    const struct bpf_insn negative_var_off_program[] = {
    BPF_LDX_MEM(BPF_DW, BPF_REG_6, BPF_REG_1, 0),
// make var_off negative, but keep the effective access offset non-negative
    BPF_ALU64_IMM(BPF_ADD, BPF_REG_6, -8),
// one byte beyond the end of the writable context
    BPF_LDX_MEM(BPF_B, BPF_REG_0, BPF_REG_6,
    sizeof(struct bpf_testmod_test_writable_ctx) + 8),
    BPF_EXIT_INSN(),
    };
    if (test__start_subtest("past_end"))
    check_attach_reject(program, ARRAY_SIZE(program));
    if (test__start_subtest("negative_var_off_past_end"))
    check_attach_reject(negative_var_off_program,
    ARRAY_SIZE(negative_var_off_program));
    }
