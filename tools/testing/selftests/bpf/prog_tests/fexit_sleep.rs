//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/fexit_sleep.c
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
// Copyright (c) 2021 Facebook
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
unsafe extern "C" fn do_sleep(skel: *mut c_void) -> c_int {
    static int do_sleep(void *skel)
    {
    struct fexit_sleep_lskel *fexit_skel = skel;
    let mut ts1: timespec = { .tv_nsec = 1 };
    let mut ts2: timespec = { .tv_sec = 10 };
    fexit_skel.bss.pid = getpid();
    (void)syscall(__NR_nanosleep, &ts1, core::ptr::null_mut());
    (void)syscall(__NR_nanosleep, &ts2, core::ptr::null_mut());
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn test_fexit_sleep() {
    void test_fexit_sleep(void)
    {
    struct fexit_sleep_lskel *fexit_skel = core::ptr::null_mut();
    int wstatus, duration = 0;
    pid_t cpid;
    char *child_stack = core::ptr::null_mut();
    int err, fexit_cnt;
    fexit_skel = fexit_sleep_lskel__open_and_load();
    if (CHECK(!fexit_skel, "fexit_skel_load", "fexit skeleton failed\n"))
    goto cleanup;
    err = fexit_sleep_lskel__attach(fexit_skel);
    if (CHECK(err, "fexit_attach", "fexit attach failed: %d\n", err))
    goto cleanup;
    child_stack = mmap(core::ptr::null_mut(), STACK_SIZE, PROT_READ | PROT_WRITE, MAP_PRIVATE |
    MAP_ANONYMOUS | MAP_STACK, -1, 0);
    if (!ASSERT_NEQ(child_stack, MAP_FAILED, "mmap"))
    goto cleanup;
    cpid = clone(do_sleep, child_stack + STACK_SIZE, CLONE_FILES | SIGCHLD, fexit_skel);
    if (CHECK(cpid == -1, "clone", "%s\n", strerror(errno)))
    goto cleanup;
// wait until first sys_nanosleep ends and second sys_nanosleep starts
    while (READ_ONCE(fexit_skel.bss.fentry_cnt) != 2);
    fexit_cnt = READ_ONCE(fexit_skel.bss.fexit_cnt);
    if (CHECK(fexit_cnt != 1, "fexit_cnt", "%d", fexit_cnt))
    goto cleanup;
// close progs and detach them. That will trigger two nop5->jmp5 rewrites
// in the trampolines to skip nanosleep_fexit prog.
// The nanosleep_fentry prog will get detached first.
// The nanosleep_fexit prog will get detached second.
// Detaching will trigger freeing of both progs JITed images.
// There will be two dying bpf_tramp_image-s, but only the initial
// bpf_tramp_image (with both _fentry and _fexit progs will be stuck
// waiting for percpu_ref_kill to confirm). The other one
// will be freed quickly.
//
    close(fexit_skel.progs.nanosleep_fentry.prog_fd);
    close(fexit_skel.progs.nanosleep_fexit.prog_fd);
    fexit_sleep_lskel__detach(fexit_skel);
// kill the thread to unwind sys_nanosleep stack through the trampoline
    kill(cpid, 9);
    if (CHECK(waitpid(cpid, &wstatus, 0) == -1, "waitpid", "%s\n", strerror(errno)))
    goto cleanup;
    if (CHECK(WEXITSTATUS(wstatus) != 0, "exitstatus", "failed"))
    goto cleanup;
// The bypassed nanosleep_fexit prog shouldn't have executed.
// Unlike progs the maps were not freed and directly accessible.
//
    fexit_cnt = READ_ONCE(fexit_skel.bss.fexit_cnt);
    if (CHECK(fexit_cnt != 1, "fexit_cnt", "%d", fexit_cnt))
    goto cleanup;
    cleanup:
    munmap(child_stack, STACK_SIZE);
    fexit_sleep_lskel__destroy(fexit_skel);
    }
