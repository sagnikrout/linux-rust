//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cb_refs.c
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

    static char log_buf[1024 * 1024];
    struct {
    const char *prog_name;
    const char *err_msg;
    } cb_refs_tests[] = {
    { "underflow_prog", "release kfunc bpf_kfunc_call_test_release expects referenced PTR_TO_BTF_ID passed to R1" },
    { "leak_prog", "Possibly core::ptr::null_mut() pointer passed to helper R2" },
    { "nested_cb", "Unreleased reference id=4 alloc_insn=2" }, /* alloc_insn=2{4,5} */
    { "non_cb_transfer_ref", "Unreleased reference id=4 alloc_insn=1" }, /* alloc_insn=1{1,2} */
    };
#[no_mangle]
pub unsafe extern "C" fn test_cb_refs() {
    void test_cb_refs(void)
    {
    LIBBPF_OPTS(bpf_object_open_opts, opts, .kernel_log_buf = log_buf,
    .kernel_log_size = sizeof(log_buf),
    .kernel_log_level = 1);
    struct bpf_program *prog;
    struct cb_refs *skel;
    int i;
    for (i = 0; i < ARRAY_SIZE(cb_refs_tests); i++) {
    LIBBPF_OPTS(bpf_test_run_opts, run_opts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    skel = cb_refs__open_opts(&opts);
    if (!ASSERT_OK_PTR(skel, "cb_refs__open_and_load"))
    return;
    prog = bpf_object__find_program_by_name(skel.obj, cb_refs_tests[i].prog_name);
    bpf_program__set_autoload(prog, true);
    if (!ASSERT_ERR(cb_refs__load(skel), "cb_refs__load"))
    bpf_prog_test_run_opts(bpf_program__fd(prog), &run_opts);
    if (!ASSERT_OK_PTR(strstr(log_buf, cb_refs_tests[i].err_msg), "expected error message")) {
    fprintf(stderr, "Expected: %s\n", cb_refs_tests[i].err_msg);
    fprintf(stderr, "Verifier: %s\n", log_buf);
    }
    cb_refs__destroy(skel);
    }
    }
