//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_ima.c
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
//
// Copyright (C) 2020 Google LLC.
//

pub const MAX_SAMPLES: c_int = 4;
    static int _run_measured_process(const char *measured_dir, u32 *monitored_pid,
    const char *cmd)
    {
    int child_pid, child_status;
    child_pid = fork();
    if (child_pid == 0) {
// monitored_pid = getpid();
    execlp("./ima_setup.sh", "./ima_setup.sh", cmd, measured_dir,
    core::ptr::null_mut());
    exit(errno);
    } else if (child_pid > 0) {
    waitpid(child_pid, &child_status, 0);
    return WEXITSTATUS(child_status);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn run_measured_process(measured_dir: *const c_char, monitored_pid: *mut u32) -> c_int {
    static int run_measured_process(const char *measured_dir, u32 *monitored_pid)
    {
    return _run_measured_process(measured_dir, monitored_pid, "run");
    }
    static u64 ima_hash_from_bpf[MAX_SAMPLES];
    static int ima_hash_from_bpf_idx;
#[no_mangle]
unsafe extern "C" fn process_sample(ctx: *mut c_void, data: *mut c_void, len: usize) -> c_int {
    static int process_sample(void *ctx, void *data, size_t len)
    {
    if (ima_hash_from_bpf_idx >= MAX_SAMPLES)
    return -ENOSPC;
    ima_hash_from_bpf[ima_hash_from_bpf_idx++] = *((u64 *)data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_init(bss: *mut ima__bss) {
    static void test_init(struct ima__bss *bss)
    {
    ima_hash_from_bpf_idx = 0;
    bss.use_ima_file_hash = false;
    bss.enable_bprm_creds_for_exec = false;
    bss.enable_kernel_read_file = false;
    bss.test_deny = false;
    }
#[no_mangle]
pub unsafe extern "C" fn test_test_ima() {
    void test_test_ima(void)
    {
    char measured_dir_template[] = "/tmp/ima_measuredXXXXXX";
    struct ring_buffer *ringbuf = core::ptr::null_mut();
    const char *measured_dir;
    u64 bin_true_sample;
    char cmd[256];
    int err, duration = 0, fresh_digest_idx = 0;
    struct ima *skel = core::ptr::null_mut();
    skel = ima__open_and_load();
    if (CHECK(!skel, "skel_load", "skeleton failed\n"))
    goto close_prog;
    ringbuf = ring_buffer__new(bpf_map__fd(skel.maps.ringbuf),
    process_sample, core::ptr::null_mut(), core::ptr::null_mut());
    if (!ASSERT_OK_PTR(ringbuf, "ringbuf"))
    goto close_prog;
    err = ima__attach(skel);
    if (CHECK(err, "attach", "attach failed: %d\n", err))
    goto close_prog;
    measured_dir = mkdtemp(measured_dir_template);
    if (CHECK(measured_dir == core::ptr::null_mut(), "mkdtemp", "err %d\n", errno))
    goto close_prog;
    snprintf(cmd, sizeof(cmd), "./ima_setup.sh setup %s", measured_dir);
    err = system(cmd);
    if (CHECK(err, "failed to run command", "%s, errno = %d\n", cmd, errno))
    goto close_clean;
//
// Test #1
// - Goal: obtain a sample with the bpf_ima_inode_hash() helper
// - Expected result:  1 sample (/bin/true)
//
    test_init(skel.bss);
    err = run_measured_process(measured_dir, &skel.bss.monitored_pid);
    if (CHECK(err, "run_measured_process #1", "err = %d\n", err))
    goto close_clean;
    err = ring_buffer__consume(ringbuf);
    ASSERT_EQ(err, 1, "num_samples_or_err");
    ASSERT_NEQ(ima_hash_from_bpf[0], 0, "ima_hash");
//
// Test #2
// - Goal: obtain samples with the bpf_ima_file_hash() helper
// - Expected result: 2 samples (./ima_setup.sh, /bin/true)
//
    test_init(skel.bss);
    skel.bss.use_ima_file_hash = true;
    err = run_measured_process(measured_dir, &skel.bss.monitored_pid);
    if (CHECK(err, "run_measured_process #2", "err = %d\n", err))
    goto close_clean;
    err = ring_buffer__consume(ringbuf);
    ASSERT_EQ(err, 2, "num_samples_or_err");
    ASSERT_NEQ(ima_hash_from_bpf[0], 0, "ima_hash");
    ASSERT_NEQ(ima_hash_from_bpf[1], 0, "ima_hash");
    bin_true_sample = ima_hash_from_bpf[1];
//
// Test #3
// - Goal: confirm that bpf_ima_inode_hash() returns a non-fresh digest
// - Expected result:
// 1 sample (/bin/true: fresh) if commit 62622dab0a28 applied
// 2 samples (/bin/true: non-fresh, fresh) if commit 62622dab0a28 is
// not applied
//
// If commit 62622dab0a28 ("ima: return IMA digest value only when
// IMA_COLLECTED flag is set") is applied, bpf_ima_inode_hash() refuses
// to give a non-fresh digest, hence the correct result is 1 instead of
// 2.
//
    test_init(skel.bss);
    err = _run_measured_process(measured_dir, &skel.bss.monitored_pid,
    "modify-bin");
    if (CHECK(err, "modify-bin #3", "err = %d\n", err))
    goto close_clean;
    skel.bss.enable_bprm_creds_for_exec = true;
    err = run_measured_process(measured_dir, &skel.bss.monitored_pid);
    if (CHECK(err, "run_measured_process #3", "err = %d\n", err))
    goto close_clean;
    err = ring_buffer__consume(ringbuf);
    ASSERT_GE(err, 1, "num_samples_or_err");
    if (err == 2) {
    ASSERT_NEQ(ima_hash_from_bpf[0], 0, "ima_hash");
    ASSERT_EQ(ima_hash_from_bpf[0], bin_true_sample,
    "sample_equal_or_err");
    fresh_digest_idx = 1;
    }
    ASSERT_NEQ(ima_hash_from_bpf[fresh_digest_idx], 0, "ima_hash");
// IMA refreshed the digest.
    ASSERT_NEQ(ima_hash_from_bpf[fresh_digest_idx], bin_true_sample,
    "sample_equal_or_err");
//
// Test #4
// - Goal: verify that bpf_ima_file_hash() returns a fresh digest
// - Expected result: 4 samples (./ima_setup.sh: fresh, fresh;
// /bin/true: fresh, fresh)
//
    test_init(skel.bss);
    skel.bss.use_ima_file_hash = true;
    skel.bss.enable_bprm_creds_for_exec = true;
    err = run_measured_process(measured_dir, &skel.bss.monitored_pid);
    if (CHECK(err, "run_measured_process #4", "err = %d\n", err))
    goto close_clean;
    err = ring_buffer__consume(ringbuf);
    ASSERT_EQ(err, 4, "num_samples_or_err");
    ASSERT_NEQ(ima_hash_from_bpf[0], 0, "ima_hash");
    ASSERT_NEQ(ima_hash_from_bpf[1], 0, "ima_hash");
    ASSERT_NEQ(ima_hash_from_bpf[2], 0, "ima_hash");
    ASSERT_NEQ(ima_hash_from_bpf[3], 0, "ima_hash");
    ASSERT_NEQ(ima_hash_from_bpf[2], bin_true_sample,
    "sample_different_or_err");
    ASSERT_EQ(ima_hash_from_bpf[3], ima_hash_from_bpf[2],
    "sample_equal_or_err");
    skel.bss.use_ima_file_hash = false;
    skel.bss.enable_bprm_creds_for_exec = false;
    err = _run_measured_process(measured_dir, &skel.bss.monitored_pid,
    "restore-bin");
    if (CHECK(err, "restore-bin #3", "err = %d\n", err))
    goto close_clean;
//
// Test #5
// - Goal: obtain a sample from the kernel_read_file hook
// - Expected result: 2 samples (./ima_setup.sh, policy_test)
//
    test_init(skel.bss);
    skel.bss.use_ima_file_hash = true;
    skel.bss.enable_kernel_read_file = true;
    err = _run_measured_process(measured_dir, &skel.bss.monitored_pid,
    "load-policy");
    if (CHECK(err, "run_measured_process #5", "err = %d\n", err))
    goto close_clean;
    err = ring_buffer__consume(ringbuf);
    ASSERT_EQ(err, 2, "num_samples_or_err");
    ASSERT_NEQ(ima_hash_from_bpf[0], 0, "ima_hash");
    ASSERT_NEQ(ima_hash_from_bpf[1], 0, "ima_hash");
//
// Test #6
// - Goal: ensure that the kernel_read_file hook denies an operation
// - Expected result: 0 samples
//
    test_init(skel.bss);
    skel.bss.enable_kernel_read_file = true;
    skel.bss.test_deny = true;
    err = _run_measured_process(measured_dir, &skel.bss.monitored_pid,
    "load-policy");
    if (CHECK(!err, "run_measured_process #6", "err = %d\n", err))
    goto close_clean;
    err = ring_buffer__consume(ringbuf);
    ASSERT_EQ(err, 0, "num_samples_or_err");
    close_clean:
    snprintf(cmd, sizeof(cmd), "./ima_setup.sh cleanup %s", measured_dir);
    err = system(cmd);
    CHECK(err, "failed to run command", "%s, errno = %d\n", cmd, errno);
    close_prog:
    ring_buffer__free(ringbuf);
    ima__destroy(skel);
    }
