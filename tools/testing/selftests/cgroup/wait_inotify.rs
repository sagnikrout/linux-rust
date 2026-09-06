//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/cgroup/wait_inotify.c
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
// Wait until an inotify event on the given cgroup file.
//

    static const char usage[] = "Usage: %s [-v] <cgroup_file>\n";
    static char *file;
    static int verbose;
#[no_mangle]
pub unsafe extern "C" fn fail_message(msg: *mut c_char) {
    static inline void fail_message(char *msg)
    {
    fprintf(stderr, msg, file);
    exit(1);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    char *cmd = argv[0];
    int c, fd;
    let mut fds: pollfd = { .events = POLLIN, };
    while ((c = getopt(argc, argv, "v")) != -1) {
    switch (c) {
    case 'v':
    verbose++;
    break;
    }
    argv++, argc--;
    }
    if (argc != 2) {
    fprintf(stderr, usage, cmd);
    return -1;
    }
    file = argv[1];
    fd = open(file, O_RDONLY);
    if (fd < 0)
    fail_message("Cgroup file %s not found!\n");
    close(fd);
    fd = inotify_init();
    if (fd < 0)
    fail_message("inotify_init() fails on %s!\n");
    if (inotify_add_watch(fd, file, IN_MODIFY) < 0)
    fail_message("inotify_add_watch() fails on %s!\n");
    fds.fd = fd;
//
// poll waiting loop
//
    for (;;) {
    let mut ret: c_int = poll(&fds, 1, 10000);
    if (ret < 0) {
    if (errno == EINTR)
    continue;
    perror("poll");
    exit(1);
    }
    if ((ret > 0) && (fds.revents & POLLIN))
    break;
    }
    if (verbose) {
    struct inotify_event events[10];
    long len;
    usleep(1000);
    len = read(fd, events, sizeof(events));
    printf("Number of events read = %ld\n",
    len/sizeof(struct inotify_event));
    }
    close(fd);
    return 0;
    }
