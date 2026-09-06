//! Automatically rewritten from C to Rust
//! Source: samples/cgroup/memcg_event_listener.c
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
// memcg_event_listener.c - Simple listener of memcg memory.events
//
// Copyright (c) 2023, SaluteDevices. All Rights Reserved.
//
// Author: Dmitry Rokosov <ddrokosov@salutedevices.com>
//

// Size of buffer to use when reading inotify events
pub const INOTIFY_BUFFER_SIZE: c_int = 8192;

    (length) -= sizeof(*(event)) + (event).len; \
    (event)++;                                   \
    })

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memcg_counters {
    pub low: c_long,
    pub high: c_long,
    pub max: c_long,
    pub oom: c_long,
    pub oom_kill: c_long,
    pub oom_group_kill: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memcg_events {
    pub counters: memcg_counters,
    pub path: [c_char; PATH_MAX],
    pub inotify_fd: c_int,
    pub inotify_wd: c_int,
}

#[no_mangle]
unsafe extern "C" fn print_memcg_counters(counters: *const memcg_counters) {
    static void print_memcg_counters(const struct memcg_counters *counters)
    {
    printf("MEMCG events:\n");
    printf("\tlow: %ld\n", counters.low);
    printf("\thigh: %ld\n", counters.high);
    printf("\tmax: %ld\n", counters.max);
    printf("\toom: %ld\n", counters.oom);
    printf("\toom_kill: %ld\n", counters.oom_kill);
    printf("\toom_group_kill: %ld\n", counters.oom_group_kill);
    }
#[no_mangle]
unsafe extern "C" fn get_memcg_counter(line: *mut c_char, name: *const c_char, counter: *mut c_long) -> c_int {
    static int get_memcg_counter(char *line, const char *name, long *counter)
    {
    let mut len: usize = strlen(name);
    char *endptr;
    long tmp;
    if (memcmp(line, name, len)) {
    warnx("Counter line %s has wrong name, %s is expected",
    line, name);
    return -EINVAL;
    }
// skip the whitespace delimiter
    len += 1;
    errno = 0;
    tmp = strtol(&line[len], &endptr, 10);
    if (((tmp == LONG_MAX || tmp == LONG_MIN) && errno == ERANGE) ||
    (errno && !tmp)) {
    warnx("Failed to parse: %s", &line[len]);
    return -ERANGE;
    }
    if (endptr == &line[len]) {
    warnx("Not digits were found in line %s", &line[len]);
    return -EINVAL;
    }
    if (!(*endptr == '\0' || (*endptr == '\n' && *++endptr == '\0'))) {
    warnx("Further characters after number: %s", endptr);
    return -EINVAL;
    }
// counter = tmp;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn read_memcg_events(events: *mut memcg_events, show_diff: bool) -> c_int {
    static int read_memcg_events(struct memcg_events *events, bool show_diff)
    {
    FILE *fp = fopen(events.path, "re");
    size_t i;
    let mut ret: c_int = 0;
    let mut any_new_events: bool = false;
    char *line = core::ptr::null_mut();
    let mut len: usize = 0;
    struct memcg_counters new_counters;
    struct memcg_counters *counters = &events.counters;
    struct {
    const char *name;
    long *new;
    long *old;
    } map[] = {
    {
    .name = "low",
    .new = &new_counters.low,
    .old = &counters.low,
    },
    {
    .name = "high",
    .new = &new_counters.high,
    .old = &counters.high,
    },
    {
    .name = "max",
    .new = &new_counters.max,
    .old = &counters.max,
    },
    {
    .name = "oom",
    .new = &new_counters.oom,
    .old = &counters.oom,
    },
    {
    .name = "oom_kill",
    .new = &new_counters.oom_kill,
    .old = &counters.oom_kill,
    },
    {
    .name = "oom_group_kill",
    .new = &new_counters.oom_group_kill,
    .old = &counters.oom_group_kill,
    },
    };
    if (!fp) {
    warn("Failed to open memcg events file %s", events.path);
    return -EBADF;
    }
// Read new values for memcg counters
    for (i = 0; i < ARRAY_SIZE(map); ++i) {
    ssize_t nread;
    errno = 0;
    nread = getline(&line, &len, fp);
    if (nread == -1) {
    if (errno) {
    warn("Failed to read line for counter %s",
    map[i].name);
    ret = -EIO;
    goto exit;
    }
    break;
    }
    ret = get_memcg_counter(line, map[i].name, map[i].new);
    if (ret) {
    warnx("Failed to get counter value from line %s", line);
    goto exit;
    }
    }
    for (i = 0; i < ARRAY_SIZE(map); ++i) {
    long diff;
    if (*map[i].new > *map[i].old) {
    diff = *map[i].new - *map[i].old;
    if (show_diff)
    printf("*** %ld MEMCG %s event%s, "
    "change counter %ld => %ld\n",
    diff, map[i].name,
    (diff == 1) ? "" : "s",
// map[i].old, *map[i].new);
// map[i].old += diff;
    any_new_events = true;
    }
    }
    if (show_diff && !any_new_events)
    printf("*** No new untracked memcg events available\n");
    exit:
    free(line);
    fclose(fp);
    return ret;
    }
    static void process_memcg_events(struct memcg_events *events,
    struct inotify_event *event)
    {
    int ret;
    if (events.inotify_wd != event.wd) {
    warnx("Unknown inotify event %d, should be %d", event.wd,
    events.inotify_wd);
    return;
    }
    printf("Received event in %s:\n", events.path);
    if (!(event.mask & IN_MODIFY)) {
    warnx("No IN_MODIFY event, skip it");
    return;
    }
    ret = read_memcg_events(events, /* show_diff = */true);
    if (ret)
    warnx("Can't read memcg events");
    }
#[no_mangle]
unsafe extern "C" fn monitor_events(events: *mut memcg_events) {
    static void monitor_events(struct memcg_events *events)
    {
    struct pollfd fds[1];
    int ret;
    printf("Started monitoring memory events from '%s'...\n", events.path);
    fds[0].fd = events.inotify_fd;
    fds[0].events = POLLIN;
    for (;;) {
    ret = poll(fds, ARRAY_SIZE(fds), -1);
    if (ret < 0 && errno != EAGAIN)
    err(EXIT_FAILURE, "Can't poll memcg events (%d)", ret);
    if (fds[0].revents & POLLERR)
    err(EXIT_FAILURE, "Got POLLERR during monitor events");
    if (fds[0].revents & POLLIN) {
    struct inotify_event *event;
    char buffer[INOTIFY_BUFFER_SIZE];
    ssize_t length;
    length = read(fds[0].fd, buffer, INOTIFY_BUFFER_SIZE);
    if (length <= 0)
    continue;
    event = (struct inotify_event *)buffer;
    while (INOTIFY_EVENT_OK(event, length)) {
    process_memcg_events(events, event);
    event = INOTIFY_EVENT_NEXT(event, length);
    }
    }
    }
    }
    static int initialize_memcg_events(struct memcg_events *events,
    const char *cgroup)
    {
    int ret;
    memset(events, 0, sizeof(struct memcg_events));
    ret = snprintf(events.path, PATH_MAX,
    "/sys/fs/cgroup/%s/memory.events", cgroup);
    if (ret >= PATH_MAX) {
    warnx("Path to cgroup memory.events is too long");
    return -EMSGSIZE;
    } else if (ret < 0) {
    warn("Can't generate cgroup event full name");
    return ret;
    }
    ret = read_memcg_events(events, /* show_diff = */false);
    if (ret) {
    warnx("Failed to read initial memcg events state (%d)", ret);
    return ret;
    }
    events.inotify_fd = inotify_init();
    if (events.inotify_fd < 0) {
    warn("Failed to setup new inotify device");
    return -EMFILE;
    }
    events.inotify_wd = inotify_add_watch(events.inotify_fd,
    events.path, IN_MODIFY);
    if (events.inotify_wd < 0) {
    warn("Couldn't add monitor in dir %s", events.path);
    return -EIO;
    }
    printf("Initialized MEMCG events with counters:\n");
    print_memcg_counters(&events.counters);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_memcg_events(events: *mut memcg_events) {
    static void cleanup_memcg_events(struct memcg_events *events)
    {
    inotify_rm_watch(events.inotify_fd, events.inotify_wd);
    close(events.inotify_fd);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *const c_char) -> c_int {
    int main(int argc, const char **argv)
    {
    struct memcg_events events;
    ssize_t ret;
    if (argc != 2)
    errx(EXIT_FAILURE, "Usage: %s <cgroup>", argv[0]);
    ret = initialize_memcg_events(&events, argv[1]);
    if (ret)
    errx(EXIT_FAILURE, "Can't initialize memcg events (%zd)", ret);
    monitor_events(&events);
    cleanup_memcg_events(&events);
    printf("Exiting memcg event listener...\n");
    return EXIT_SUCCESS;
    }
