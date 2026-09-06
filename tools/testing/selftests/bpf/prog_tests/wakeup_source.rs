//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/wakeup_source.c
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
// Copyright 2026 Google LLC

#[no_mangle]
unsafe extern "C" fn lock_ws(name: *const c_char) -> c_int {
    static int lock_ws(const char *name)
    {
    int fd;
    ssize_t bytes;
    fd = open("/sys/power/wake_lock", O_WRONLY);
    if (!ASSERT_OK_FD(fd, "open /sys/power/wake_lock"))
    return -1;
    bytes = write(fd, name, strlen(name));
    close(fd);
    if (!ASSERT_EQ(bytes, strlen(name), "write to wake_lock"))
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn unlock_ws(name: *const c_char) {
    static void unlock_ws(const char *name)
    {
    int fd;
    fd = open("/sys/power/wake_unlock", O_WRONLY);
    if (fd < 0)
    return;
    write(fd, name, strlen(name));
    close(fd);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_ctx {
    pub name: *const c_char,
    pub found: bool,
    pub active_time_ns: c_longlong,
    pub total_time_ns: c_longlong,
}

#[no_mangle]
unsafe extern "C" fn process_sample(ctx: *mut c_void, data: *mut c_void, len: usize) -> c_int {
    static int process_sample(void *ctx, void *data, size_t len)
    {
    struct rb_ctx *rb_ctx = ctx;
    struct wakeup_event_t *e = data;
    if (strcmp(e.name, rb_ctx.name) == 0) {
    rb_ctx.found = true;
    rb_ctx.active_time_ns = e.active_time_ns;
    rb_ctx.total_time_ns = e.total_time_ns;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_wakeup_source() {
    void test_wakeup_source(void)
    {
    struct btf *btf;
    int id;
    btf = btf__load_vmlinux_btf();
    if (!ASSERT_OK_PTR(btf, "btf_vmlinux"))
    return;
    id = btf__find_by_name_kind(btf, "bpf_wakeup_sources_get_head", BTF_KIND_FUNC);
    btf__free(btf);
    if (id < 0) {
    printf("%s:SKIP:bpf_wakeup_sources_get_head kfunc not found in BTF\n", __func__);
    test__skip();
    return;
    }
    if (test__start_subtest("iterate_and_verify_times")) {
    struct test_wakeup_source *skel;
    struct ring_buffer *rb = core::ptr::null_mut();
    struct rb_ctx rb_ctx = {
    .name = "bpf_selftest_ws_times",
    .found = false,
    };
    int err;
    skel = test_wakeup_source__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open_and_load"))
    return;
    rb = ring_buffer__new(bpf_map__fd(skel.maps.rb), process_sample, &rb_ctx, core::ptr::null_mut());
    if (!ASSERT_OK_PTR(rb, "ring_buffer__new"))
    goto destroy;
// Create a temporary wakeup source
    if (!ASSERT_OK(lock_ws(rb_ctx.name), "lock_ws"))
    goto unlock;
    err = bpf_prog_test_run_opts(bpf_program__fd(
    skel.progs.iterate_wakeupsources), core::ptr::null_mut());
    ASSERT_OK(err, "bpf_prog_test_run");
    ring_buffer__consume(rb);
    ASSERT_TRUE(rb_ctx.found, "found_test_ws_in_rb");
    ASSERT_GT(rb_ctx.active_time_ns, 0, "active_time_gt_0");
    ASSERT_GT(rb_ctx.total_time_ns, 0, "total_time_gt_0");
    unlock:
    unlock_ws(rb_ctx.name);
    destroy:
    if (rb)
    ring_buffer__free(rb);
    test_wakeup_source__destroy(skel);
    }
    RUN_TESTS(wakeup_source_fail);
    }
