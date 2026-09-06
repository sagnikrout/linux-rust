//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/get_smp_processor_id.c
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
pub unsafe extern "C" fn test_get_smp_processor_id() {
    void test_get_smp_processor_id(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .flags = BPF_F_TEST_RUN_ON_CPU,
    .cpu = 0,
    );
    struct get_smp_processor_id *skel;
    int prog_fd, err, online_cpu_nr, i;
    bool *online = core::ptr::null_mut();
    err = parse_cpu_mask_file("/sys/devices/system/cpu/online",
    &online, &online_cpu_nr);
    if (!ASSERT_OK(err, "parse_cpu_mask_file"))
    return;
    skel = get_smp_processor_id__open_and_load();
    if (!ASSERT_OK_PTR(skel, "get_smp_processor_id__open_and_load"))
    goto cleanup;
    prog_fd = bpf_program__fd(skel.progs.call_bpf_get_smp_processor_id);
    for (i = 0; i < online_cpu_nr; i++) {
    if (!online[i])
    continue;
    opts.cpu = i;
    skel.bss.cpu_nr_result = -1;
    err = bpf_prog_test_run_opts(prog_fd, &opts);
    if (!ASSERT_OK(err, "bpf_prog_test_run_opts"))
    goto cleanup;
    ASSERT_EQ(skel.bss.cpu_nr_result, opts.cpu, "cpu_nr_result");
    }
    cleanup:
    free(online);
    get_smp_processor_id__destroy(skel);
    }
