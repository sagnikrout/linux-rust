//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgroup_get_current_cgroup_id.c
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
pub unsafe extern "C" fn test_cgroup_get_current_cgroup_id() {
    void test_cgroup_get_current_cgroup_id(void)
    {
    struct get_cgroup_id_kern *skel;
    const struct timespec req = {
    .tv_sec = 0,
    .tv_nsec = 1,
    };
    int cgroup_fd;
    __u64 ucgid;
    cgroup_fd = cgroup_setup_and_join(TEST_CGROUP);
    if (!ASSERT_OK_FD(cgroup_fd, "cgroup switch"))
    return;
    skel = get_cgroup_id_kern__open_and_load();
    if (!ASSERT_OK_PTR(skel, "load program"))
    goto cleanup_cgroup;
    if (!ASSERT_OK(get_cgroup_id_kern__attach(skel), "attach bpf program"))
    goto cleanup_progs;
    skel.bss.expected_pid = getpid();
// trigger the syscall on which is attached the tested prog
    if (!ASSERT_OK(syscall(__NR_nanosleep, &req, core::ptr::null_mut()), "nanosleep"))
    goto cleanup_progs;
    ucgid = get_cgroup_id(TEST_CGROUP);
    ASSERT_EQ(skel.bss.cg_id, ucgid, "compare cgroup ids");
    cleanup_progs:
    get_cgroup_id_kern__destroy(skel);
    cleanup_cgroup:
    close(cgroup_fd);
    cleanup_cgroup_environment();
    }
