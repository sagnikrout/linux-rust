//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/timer_crash.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_elem {
    pub timer: bpf_timer,
    pub lock: bpf_spin_lock,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct map_elem);
    } amap SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct map_elem);
    } hmap SEC(".maps");
    let mut pid: c_int = 0;
    int crash_map = 0; /* 0 for amap, 1 for hmap */
    SEC("fentry/do_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn sys_enter(ctx: *mut c_void) -> c_int {
    int sys_enter(void *ctx)
    {
    struct map_elem *e, value = {};
    void *map = crash_map ? (void *)&hmap : (void *)&amap;
    if (bpf_get_current_task_btf().tgid != pid)
    return 0;
// (void **)&value = (void *)0xdeadcaf3;
    bpf_map_update_elem(map, &(int){0}, &value, 0);
// For array map, doing bpf_map_update_elem will do a
// check_and_free_timer_in_array, which will trigger the crash if timer
// pointer was overwritten, for hmap we need to use bpf_timer_cancel.
//
    if (crash_map == 1) {
    e = bpf_map_lookup_elem(map, &(int){0});
    if (!e)
    return 0;
    bpf_timer_cancel(&e.timer);
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
