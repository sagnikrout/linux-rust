//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/trace-event-info.c
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
//
// Copyright (C) 2008,2009, Steven Rostedt <srostedt@redhat.com>
//

pub const MAX_EVENT_LENGTH: c_int = 512;
    static int output_fd;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracepoint_path {
    pub system: *mut c_char,
    pub name: *mut c_char,
    pub next: *mut tracepoint_path,
}

// unfortunately, you can not stat debugfs or proc files for size
#[no_mangle]
unsafe extern "C" fn record_file(file: *const c_char, hdr_sz: isize) -> c_int {
    static int record_file(const char *file, ssize_t hdr_sz)
    {
    let mut size: c_ulonglong = 0;
    char buf[BUFSIZ], *sizep;
    let mut hdr_pos: off_t = lseek(output_fd, 0, SEEK_CUR);
    int r, fd;
    let mut err: c_int = -EIO;
    fd = open(file, O_RDONLY);
    if (fd < 0) {
    pr_debug("Can't read '%s'", file);
    return -errno;
    }
// put in zeros for file size, then fill true size later
    if (hdr_sz) {
    if (write(output_fd, &size, hdr_sz) != hdr_sz)
    goto out;
    }
    do {
    r = read(fd, buf, BUFSIZ);
    if (r > 0) {
    size += r;
    if (write(output_fd, buf, r) != r)
    goto out;
    }
    } while (r > 0);
// ugh, handle big-endian hdr_size == 4
    sizep = (char*)&size;
    if (host_is_bigendian())
    sizep += sizeof(u64) - hdr_sz;
    if (hdr_sz && pwrite(output_fd, sizep, hdr_sz, hdr_pos) < 0) {
    pr_debug("writing file size failed\n");
    goto out;
    }
    err = 0;
    out:
    close(fd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn record_header_files() -> c_int {
    static int record_header_files(void)
    {
    char *path = get_events_file("header_page");
    struct stat st;
    let mut err: c_int = -EIO;
    if (!path) {
    pr_debug("can't get tracing/events/header_page");
    return -ENOMEM;
    }
    if (stat(path, &st) < 0) {
    pr_debug("can't read '%s'", path);
    goto out;
    }
    if (write(output_fd, "header_page", 12) != 12) {
    pr_debug("can't write header_page\n");
    goto out;
    }
    if (record_file(path, 8) < 0) {
    pr_debug("can't record header_page file\n");
    goto out;
    }
    put_events_file(path);
    path = get_events_file("header_event");
    if (!path) {
    pr_debug("can't get tracing/events/header_event");
    err = -ENOMEM;
    goto out;
    }
    if (stat(path, &st) < 0) {
    pr_debug("can't read '%s'", path);
    goto out;
    }
    if (write(output_fd, "header_event", 13) != 13) {
    pr_debug("can't write header_event\n");
    goto out;
    }
    if (record_file(path, 8) < 0) {
    pr_debug("can't record header_event file\n");
    goto out;
    }
    err = 0;
    out:
    put_events_file(path);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn name_in_tp_list(sys: *mut c_char, tps: *mut tracepoint_path) -> bool {
    static bool name_in_tp_list(char *sys, struct tracepoint_path *tps)
    {
    while (tps) {
    if (!strcmp(sys, tps.name))
    return true;
    tps = tps.next;
    }
    return false;
    }

    while ((dent = readdir(dir)))				\
    if (dent.d_type == DT_DIR &&			\
    (strcmp(dent.d_name, ".")) &&		\
    (strcmp(dent.d_name, "..")))		\
#[no_mangle]
unsafe extern "C" fn copy_event_system(sys: *const c_char, tps: *mut tracepoint_path) -> c_int {
    static int copy_event_system(const char *sys, struct tracepoint_path *tps)
    {
    struct dirent *dent;
    struct stat st;
    char *format;
    DIR *dir;
    let mut count: c_int = 0;
    int ret;
    int err;
    dir = opendir(sys);
    if (!dir) {
    pr_debug("can't read directory '%s'", sys);
    return -errno;
    }
    for_each_event_tps(dir, dent, tps) {
    if (!name_in_tp_list(dent.d_name, tps))
    continue;
    if (asprintf(&format, "%s/%s/format", sys, dent.d_name) < 0) {
    err = -ENOMEM;
    goto out;
    }
    ret = stat(format, &st);
    free(format);
    if (ret < 0)
    continue;
    count++;
    }
    if (write(output_fd, &count, 4) != 4) {
    err = -EIO;
    pr_debug("can't write count\n");
    goto out;
    }
    rewinddir(dir);
    for_each_event_tps(dir, dent, tps) {
    if (!name_in_tp_list(dent.d_name, tps))
    continue;
    if (asprintf(&format, "%s/%s/format", sys, dent.d_name) < 0) {
    err = -ENOMEM;
    goto out;
    }
    ret = stat(format, &st);
    if (ret >= 0) {
    err = record_file(format, 8);
    if (err) {
    free(format);
    goto out;
    }
    }
    free(format);
    }
    err = 0;
    out:
    closedir(dir);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn record_ftrace_files(tps: *mut tracepoint_path) -> c_int {
    static int record_ftrace_files(struct tracepoint_path *tps)
    {
    char *path;
    int ret;
    path = get_events_file("ftrace");
    if (!path) {
    pr_debug("can't get tracing/events/ftrace");
    return -ENOMEM;
    }
    ret = copy_event_system(path, tps);
    put_tracing_file(path);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn system_in_tp_list(sys: *mut c_char, tps: *mut tracepoint_path) -> bool {
    static bool system_in_tp_list(char *sys, struct tracepoint_path *tps)
    {
    while (tps) {
    if (!strcmp(sys, tps.system))
    return true;
    tps = tps.next;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn record_event_files(tps: *mut tracepoint_path) -> c_int {
    static int record_event_files(struct tracepoint_path *tps)
    {
    struct dirent *dent;
    struct stat st;
    char *path;
    char *sys;
    DIR *dir;
    let mut count: c_int = 0;
    int ret;
    int err;
    path = get_tracing_file("events");
    if (!path) {
    pr_debug("can't get tracing/events");
    return -ENOMEM;
    }
    dir = opendir(path);
    if (!dir) {
    err = -errno;
    pr_debug("can't read directory '%s'", path);
    goto out;
    }
    for_each_event_tps(dir, dent, tps) {
    if (strcmp(dent.d_name, "ftrace") == 0 ||
    !system_in_tp_list(dent.d_name, tps))
    continue;
    count++;
    }
    if (write(output_fd, &count, 4) != 4) {
    err = -EIO;
    pr_debug("can't write count\n");
    goto out;
    }
    rewinddir(dir);
    for_each_event_tps(dir, dent, tps) {
    if (strcmp(dent.d_name, "ftrace") == 0 ||
    !system_in_tp_list(dent.d_name, tps))
    continue;
    if (asprintf(&sys, "%s/%s", path, dent.d_name) < 0) {
    err = -ENOMEM;
    goto out;
    }
    ret = stat(sys, &st);
    if (ret >= 0) {
    let mut size: isize = strlen(dent.d_name) + 1;
    if (write(output_fd, dent.d_name, size) != size ||
    copy_event_system(sys, tps) < 0) {
    err = -EIO;
    free(sys);
    goto out;
    }
    }
    free(sys);
    }
    err = 0;
    out:
    if (dir)
    closedir(dir);
    put_tracing_file(path);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn record_proc_kallsyms() -> c_int {
    static int record_proc_kallsyms(void)
    {
    let mut size: c_ulonglong = 0;
//
// Just to keep older perf.data file parsers happy, record a zero
// sized kallsyms file, i.e. do the same thing that was done when
// /proc/kallsyms (or something specified via --kallsyms, in a
// different path) couldn't be read.
//
    return write(output_fd, &size, 4) != 4 ? -EIO : 0;
    }
#[no_mangle]
unsafe extern "C" fn record_ftrace_printk() -> c_int {
    static int record_ftrace_printk(void)
    {
    unsigned int size;
    char *path;
    struct stat st;
    int ret, err = 0;
    path = get_tracing_file("printk_formats");
    if (!path) {
    pr_debug("can't get tracing/printk_formats");
    return -ENOMEM;
    }
    ret = stat(path, &st);
    if (ret < 0) {
// not found
    size = 0;
    if (write(output_fd, &size, 4) != 4)
    err = -EIO;
    goto out;
    }
    err = record_file(path, 4);
    out:
    put_tracing_file(path);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn record_saved_cmdline() -> c_int {
    static int record_saved_cmdline(void)
    {
    unsigned long long size;
    char *path;
    struct stat st;
    int ret, err = 0;
    path = get_tracing_file("saved_cmdlines");
    if (!path) {
    pr_debug("can't get tracing/saved_cmdline");
    return -ENOMEM;
    }
    ret = stat(path, &st);
    if (ret < 0) {
// not found
    size = 0;
    if (write(output_fd, &size, 8) != 8)
    err = -EIO;
    goto out;
    }
    err = record_file(path, 8);
    out:
    put_tracing_file(path);
    return err;
    }
    static void
    put_tracepoints_path(struct tracepoint_path *tps)
    {
    while (tps) {
    struct tracepoint_path *t = tps;
    tps = tps.next;
    zfree(&t.name);
    zfree(&t.system);
    free(t);
    }
    }
    static struct tracepoint_path *tracepoint_id_to_path(u64 config)
    {
    struct tracepoint_path *path = core::ptr::null_mut();
    DIR *sys_dir, *evt_dir;
    struct dirent *sys_dirent, *evt_dirent;
    char id_buf[24];
    int fd;
    u64 id;
    char evt_path[MAXPATHLEN];
    char *dir_path;
    sys_dir = tracing_events__opendir();
    if (!sys_dir)
    return core::ptr::null_mut();
    for_each_subsystem(sys_dir, sys_dirent) {
    dir_path = get_events_file(sys_dirent.d_name);
    if (!dir_path)
    continue;
    evt_dir = opendir(dir_path);
    if (!evt_dir)
    goto next;
    for_each_event(dir_path, evt_dir, evt_dirent) {
    scnprintf(evt_path, MAXPATHLEN, "%s/%s/id", dir_path,
    evt_dirent.d_name);
    fd = open(evt_path, O_RDONLY);
    if (fd < 0)
    continue;
    if (read(fd, id_buf, sizeof(id_buf)) < 0) {
    close(fd);
    continue;
    }
    close(fd);
    id = atoll(id_buf);
    if (id == config) {
    put_events_file(dir_path);
    closedir(evt_dir);
    closedir(sys_dir);
    path = zalloc(sizeof(*path));
    if (!path)
    return core::ptr::null_mut();
    if (asprintf(&path.system, "%.*s",
    MAX_EVENT_LENGTH, sys_dirent.d_name) < 0) {
    free(path);
    return core::ptr::null_mut();
    }
    if (asprintf(&path.name, "%.*s",
    MAX_EVENT_LENGTH, evt_dirent.d_name) < 0) {
    zfree(&path.system);
    free(path);
    return core::ptr::null_mut();
    }
    return path;
    }
    }
    closedir(evt_dir);
    next:
    put_events_file(dir_path);
    }
    closedir(sys_dir);
    return core::ptr::null_mut();
    }
    char *tracepoint_id_to_name(u64 config)
    {
    struct tracepoint_path *path = tracepoint_id_to_path(config);
    char *buf = core::ptr::null_mut();
    if (path && asprintf(&buf, "%s:%s", path.system, path.name) < 0)
    buf = core::ptr::null_mut();
    put_tracepoints_path(path);
    return buf;
    }
    static struct tracepoint_path *tracepoint_name_to_path(const char *name)
    {
    struct tracepoint_path *path = zalloc(sizeof(*path));
    const char *str = strchr(name, ':');
    if (path == core::ptr::null_mut() || str == core::ptr::null_mut()) {
    free(path);
    return core::ptr::null_mut();
    }
    path.system = strndup(name, str - name);
    path.name = strdup(str+1);
    if (path.system == core::ptr::null_mut() || path.name == core::ptr::null_mut()) {
    zfree(&path.system);
    zfree(&path.name);
    zfree(&path);
    }
    return path;
    }
    static struct tracepoint_path *
    get_tracepoints_path(struct list_head *pattrs)
    {
    struct tracepoint_path path, *ppath = &path;
    struct evsel *pos;
    let mut nr_tracepoints: c_int = 0;
    list_for_each_entry(pos, pattrs, core.node) {
    if (pos.core.attr.type != PERF_TYPE_TRACEPOINT)
    continue;
    ++nr_tracepoints;
    if (pos.name) {
    ppath.next = tracepoint_name_to_path(pos.name);
    if (ppath.next)
    goto next;
    if (strchr(pos.name, ':') == core::ptr::null_mut())
    goto try_id;
    goto error;
    }
    try_id:
    ppath.next = tracepoint_id_to_path(pos.core.attr.config);
    if (!ppath.next) {
    error:
    pr_debug("No memory to alloc tracepoints list\n");
    put_tracepoints_path(path.next);
    return core::ptr::null_mut();
    }
    next:
    ppath = ppath.next;
    }
    return nr_tracepoints > 0 ? path.next : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn have_tracepoints(pattrs: *mut list_head) -> bool {
    bool have_tracepoints(struct list_head *pattrs)
    {
    struct evsel *pos;
    list_for_each_entry(pos, pattrs, core.node)
    if (pos.core.attr.type == PERF_TYPE_TRACEPOINT)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn tracing_data_header() -> c_int {
    static int tracing_data_header(void)
    {
    char buf[20];
    ssize_t size;
// just guessing this is someone's birthday.. ;)
    buf[0] = 23;
    buf[1] = 8;
    buf[2] = 68;
    memcpy(buf + 3, "tracing", 7);
    if (write(output_fd, buf, 10) != 10)
    return -1;
    size = strlen(VERSION) + 1;
    if (write(output_fd, VERSION, size) != size)
    return -1;
// save endian
    if (host_is_bigendian())
    buf[0] = 1;
    else
    buf[0] = 0;
    if (write(output_fd, buf, 1) != 1)
    return -1;
// save size of long
    buf[0] = sizeof(long);
    if (write(output_fd, buf, 1) != 1)
    return -1;
// save page_size
    if (write(output_fd, &page_size, 4) != 4)
    return -1;
    return 0;
    }
    struct tracing_data *tracing_data_get(struct list_head *pattrs,
    int fd, bool temp)
    {
    struct tracepoint_path *tps;
    struct tracing_data *tdata;
    int err;
    output_fd = fd;
    tps = get_tracepoints_path(pattrs);
    if (!tps)
    return core::ptr::null_mut();
    tdata = malloc(sizeof(*tdata));
    if (!tdata)
    return core::ptr::null_mut();
    tdata.temp = temp;
    tdata.size = 0;
    if (temp) {
    int temp_fd;
    snprintf(tdata.temp_file, sizeof(tdata.temp_file),
    "/tmp/perf-XXXXXX");
    if (!mkstemp(tdata.temp_file)) {
    pr_debug("Can't make temp file");
    free(tdata);
    return core::ptr::null_mut();
    }
    temp_fd = open(tdata.temp_file, O_RDWR);
    if (temp_fd < 0) {
    pr_debug("Can't read '%s'", tdata.temp_file);
    free(tdata);
    return core::ptr::null_mut();
    }
//
// Set the temp file the default output, so all the
// tracing data are stored into it.
//
    output_fd = temp_fd;
    }
    err = tracing_data_header();
    if (err)
    goto out;
    err = record_header_files();
    if (err)
    goto out;
    err = record_ftrace_files(tps);
    if (err)
    goto out;
    err = record_event_files(tps);
    if (err)
    goto out;
    err = record_proc_kallsyms();
    if (err)
    goto out;
    err = record_ftrace_printk();
    if (err)
    goto out;
    err = record_saved_cmdline();
    out:
//
// All tracing data are stored by now, we can restore
// the default output file in case we used temp file.
//
    if (temp) {
    tdata.size = lseek(output_fd, 0, SEEK_CUR);
    close(output_fd);
    output_fd = fd;
    }
    if (err)
    zfree(&tdata);
    put_tracepoints_path(tps);
    return tdata;
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_data_put(tdata: *mut tracing_data) -> c_int {
    int tracing_data_put(struct tracing_data *tdata)
    {
    let mut err: c_int = 0;
    if (tdata.temp) {
    err = record_file(tdata.temp_file, 0);
    unlink(tdata.temp_file);
    }
    free(tdata);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn read_tracing_data(fd: c_int, pattrs: *mut list_head) -> c_int {
    int read_tracing_data(int fd, struct list_head *pattrs)
    {
    int err;
    struct tracing_data *tdata;
//
// We work over the real file, so we can write data
// directly, no temp file is needed.
//
    tdata = tracing_data_get(pattrs, fd, false);
    if (!tdata)
    return -ENOMEM;
    err = tracing_data_put(tdata);
    return err;
    }
