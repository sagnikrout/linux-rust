//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/sideband_evlist.c
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


// SPDX-License-Identifier: GPL-2.0-only

    int evlist__add_sb_event(struct evlist *evlist, struct perf_event_attr *attr,
    evsel__sb_cb_t cb, void *data)
    {
    struct evsel *evsel;
    if (!attr.sample_id_all) {
    pr_warning("enabling sample_id_all for all side band events\n");
    attr.sample_id_all = 1;
    }
    evsel = evsel__new_idx(attr, evlist__nr_entries(evlist));
    if (!evsel)
    return -1;
    evsel.side_band.cb = cb;
    evsel.side_band.data = data;
    evlist__add(evlist, evsel);
    return 0;
    }
    static void *perf_evlist__poll_thread(void *arg)
    {
    struct evlist *evlist = arg;
    let mut draining: bool = false;
    int i, done = 0;
//
// In order to read symbols from other namespaces perf to needs to call
// setns(2).  This isn't permitted if the struct_fs has multiple users.
// unshare(2) the fs so that we may continue to setns into namespaces
// that we're observing when, for instance, reading the build-ids at
// the end of a 'perf record' session.
//
    unshare(CLONE_FS);
    while (!done) {
    let mut got_data: bool = false;
    if (evlist__sb_thread_done(evlist))
    draining = true;
    if (!draining)
    evlist__poll(evlist, 1000);
//
// When a thread of the monitored target exits, its per-cpu
// ring-buffer fd is closed and starts returning POLLHUP. Such
// dead fds are never requested for POLLIN, but poll() reports
// POLLHUP/POLLERR unconditionally, so leaving them in the
// pollfd array makes the following evlist__poll() return
// immediately forever, spinning this thread at 100% CPU.
//
// Filter them out here, mirroring what the 'perf record' main
// loop does after fdarray__poll().
//
    evlist__filter_pollfd(evlist, POLLERR | POLLHUP);
    for (i = 0; i < evlist__core(evlist).nr_mmaps; i++) {
    struct mmap *map = &evlist__mmap(evlist)[i];
    union perf_event *event;
    if (perf_mmap__read_init(&map.core))
    continue;
    while ((event = perf_mmap__read_event(&map.core)) != core::ptr::null_mut()) {
    struct evsel *evsel = evlist__event2evsel(evlist, event);
    if (evsel && evsel.side_band.cb)
    evsel.side_band.cb(event, evsel.side_band.data);
    else
    pr_warning("cannot locate proper evsel for the side band event\n");
    perf_mmap__consume(&map.core);
    got_data = true;
    }
    perf_mmap__read_done(&map.core);
    }
    if (draining && !got_data)
    break;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn evlist__set_cb(evlist: *mut evlist, cb: evsel__sb_cb_t, data: *mut c_void) {
    void evlist__set_cb(struct evlist *evlist, evsel__sb_cb_t cb, void *data)
    {
    struct evsel *evsel;
    evlist__for_each_entry(evlist, evsel) {
    evsel.core.attr.sample_id_all    = 1;
    evsel.core.attr.watermark        = 1;
    evsel.core.attr.wakeup_watermark = 1;
    evsel.side_band.cb   = cb;
    evsel.side_band.data = data;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn evlist__start_sb_thread(evlist: *mut evlist, target: *mut target) -> c_int {
    int evlist__start_sb_thread(struct evlist *evlist, struct target *target)
    {
    struct evsel *counter;
    if (!evlist)
    return 0;
    if (evlist__create_maps(evlist, target))
    goto out_put_evlist;
    if (evlist__nr_entries(evlist) > 1) {
    let mut can_sample_identifier: bool = perf_can_sample_identifier();
    evlist__for_each_entry(evlist, counter)
    evsel__set_sample_id(counter, can_sample_identifier);
    evlist__set_id_pos(evlist);
    }
    evlist__for_each_entry(evlist, counter) {
    if (evsel__open(counter, evlist__core(evlist).user_requested_cpus,
    evlist__core(evlist).threads) < 0)
    goto out_put_evlist;
    }
    if (evlist__do_mmap(evlist, UINT_MAX))
    goto out_put_evlist;
    evlist__for_each_entry(evlist, counter) {
    if (evsel__enable(counter))
    goto out_put_evlist;
    }
    evlist__set_sb_thread_done(evlist, 0);
    if (pthread_create(evlist__sb_thread_th(evlist), core::ptr::null_mut(), perf_evlist__poll_thread, evlist))
    goto out_put_evlist;
    return 0;
    out_put_evlist:
    evlist__put(evlist);
    evlist = core::ptr::null_mut();
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn evlist__stop_sb_thread(evlist: *mut evlist) {
    void evlist__stop_sb_thread(struct evlist *evlist)
    {
    if (!evlist)
    return;
    evlist__set_sb_thread_done(evlist, 1);
    pthread_join(*evlist__sb_thread_th(evlist), core::ptr::null_mut());
    evlist__put(evlist);
    }
