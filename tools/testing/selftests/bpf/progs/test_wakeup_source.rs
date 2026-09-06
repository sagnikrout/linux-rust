//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_wakeup_source.c
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

pub const MAX_LOOP_ITER: c_int = 1000;

    struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, RB_SIZE);
    } rb SEC(".maps");
    struct bpf_ws_lock;
    struct bpf_ws_lock *bpf_wakeup_sources_read_lock(void) __ksym;
    void bpf_wakeup_sources_read_unlock(struct bpf_ws_lock *lock) __ksym;
    void *bpf_wakeup_sources_get_head(void) __ksym;
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn iterate_wakeupsources(ctx: *mut c_void) -> c_int {
    int iterate_wakeupsources(void *ctx)
    {
    struct list_head *head = bpf_wakeup_sources_get_head();
    struct list_head *pos = head;
    struct bpf_ws_lock *lock;
    int i;
    lock = bpf_wakeup_sources_read_lock();
    if (!lock)
    return 0;
    bpf_for(i, 0, MAX_LOOP_ITER) {
    if (bpf_core_read(&pos, sizeof(pos), &pos.next) || !pos || pos == head)
    break;
    struct wakeup_event_t *e = bpf_ringbuf_reserve(&rb, sizeof(*e), 0);
    if (!e)
    break;
    struct wakeup_source *ws = bpf_core_cast(
    (void *)pos - bpf_core_field_offset(struct wakeup_source, entry),
    struct wakeup_source);
    let mut active_time: i64 = 0;
    let mut active: bool = BPF_CORE_READ_BITFIELD(ws, active);
    let mut autosleep_enable: bool = BPF_CORE_READ_BITFIELD(ws, autosleep_enabled);
    let mut last_time: i64 = ws.last_time;
    let mut max_time: i64 = ws.max_time;
    let mut prevent_sleep_time: i64 = ws.prevent_sleep_time;
    let mut total_time: i64 = ws.total_time;
    if (active) {
    let mut curr_time: i64 = bpf_ktime_get_ns();
    let mut prevent_time: i64 = ws.start_prevent_time;
    if (curr_time > last_time)
    active_time = curr_time - last_time;
    total_time += active_time;
    if (active_time > max_time)
    max_time = active_time;
    if (autosleep_enable && curr_time > prevent_time)
    prevent_sleep_time += curr_time - prevent_time;
    }
    e.active_count = ws.active_count;
    e.active_time_ns = active_time;
    e.event_count = ws.event_count;
    e.expire_count = ws.expire_count;
    e.last_time_ns = last_time;
    e.max_time_ns = max_time;
    e.prevent_sleep_time_ns = prevent_sleep_time;
    e.total_time_ns = total_time;
    e.wakeup_count = ws.wakeup_count;
    if (bpf_probe_read_kernel_str(
    e.name, WAKEUP_NAME_LEN, ws.name) < 0)
    e.name[0] = '\0';
    bpf_ringbuf_submit(e, 0);
    }
    bpf_wakeup_sources_read_unlock(lock);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
