//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/x86/util/event.c
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
pub struct perf_event__synthesize_extra_kmaps_cb_args {
    pub tool: *const perf_tool,
    pub process: perf_event__handler_t,
    pub machine: *mut machine,
    pub event: *mut union perf_event,
}

#[no_mangle]
unsafe extern "C" fn perf_event__synthesize_extra_kmaps_cb(map: *mut map, data: *mut c_void) -> c_int {
    static int perf_event__synthesize_extra_kmaps_cb(struct map *map, void *data)
    {
    struct perf_event__synthesize_extra_kmaps_cb_args *args = data;
    union perf_event *event = args.event;
    struct kmap *kmap;
    size_t size;
    if (!__map__is_extra_kernel_map(map))
    return 0;
    kmap = map__kmap(map);
    size = sizeof(event.mmap) - sizeof(event.mmap.filename) +
    PERF_ALIGN(strlen(kmap.name) + 1, sizeof(u64)) +
    args.machine.id_hdr_size;
    memset(event, 0, size);
    event.mmap.header.type = PERF_RECORD_MMAP;
//
// kernel uses 0 for user space maps, see kernel/perf_event.c
// __perf_event_mmap
//
    if (machine__is_host(args.machine))
    event.header.misc = PERF_RECORD_MISC_KERNEL;
    else
    event.header.misc = PERF_RECORD_MISC_GUEST_KERNEL;
    event.mmap.header.size = size;
    event.mmap.start = map__start(map);
    event.mmap.len   = map__size(map);
    event.mmap.pgoff = map__pgoff(map);
    event.mmap.pid   = args.machine.pid;
    strlcpy(event.mmap.filename, kmap.name, PATH_MAX);
    if (perf_tool__process_synth_event(args.tool, event, args.machine, args.process) != 0)
    return -1;
    return 0;
    }
    int perf_event__synthesize_extra_kmaps(const struct perf_tool *tool,
    perf_event__handler_t process,
    struct machine *machine)
    {
    int rc;
    struct maps *kmaps = machine__kernel_maps(machine);
    struct perf_event__synthesize_extra_kmaps_cb_args args = {
    .tool = tool,
    .process = process,
    .machine = machine,
    .event = zalloc(sizeof(args.event.mmap) + machine.id_hdr_size),
    };
    if (!args.event) {
    pr_debug("Not enough memory synthesizing mmap event "
    "for extra kernel maps\n");
    return -1;
    }
    rc = maps__for_each_map(kmaps, perf_event__synthesize_extra_kmaps_cb, &args);
    free(args.event);
    return rc;
    }
