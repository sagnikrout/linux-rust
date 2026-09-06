//! Automatically rewritten from C to Rust
//! Source: samples/cgroup/cgroup_event_listener.c
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
// cgroup_event_listener.c - Simple listener of cgroup events
//
// Copyright (C) Kirill A. Shutemov <kirill@shutemov.name>
//

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut efd: c_int = -1;
    let mut cfd: c_int = -1;
    let mut event_control: c_int = -1;
    char event_control_path[PATH_MAX];
    char line[LINE_MAX];
    int ret;
    if (argc != 3)
    errx(1, "%s", USAGE_STR);
    cfd = open(argv[1], O_RDONLY);
    if (cfd == -1)
    err(1, "Cannot open %s", argv[1]);
    ret = snprintf(event_control_path, PATH_MAX, "%s/cgroup.event_control",
    dirname(argv[1]));
    if (ret >= PATH_MAX)
    errx(1, "Path to cgroup.event_control is too long");
    event_control = open(event_control_path, O_WRONLY);
    if (event_control == -1)
    err(1, "Cannot open %s", event_control_path);
    efd = eventfd(0, 0);
    if (efd == -1)
    err(1, "eventfd() failed");
    ret = snprintf(line, LINE_MAX, "%d %d %s", efd, cfd, argv[2]);
    if (ret >= LINE_MAX)
    errx(1, "Arguments string is too long");
    ret = write(event_control, line, strlen(line) + 1);
    if (ret == -1)
    err(1, "Cannot write to cgroup.event_control");
    while (1) {
    uint64_t result;
    ret = read(efd, &result, sizeof(result));
    if (ret == -1) {
    if (errno == EINTR)
    continue;
    err(1, "Cannot read from eventfd");
    }
    assert(ret == sizeof(result));
    ret = access(event_control_path, W_OK);
    if ((ret == -1) && (errno == ENOENT)) {
    puts("The cgroup seems to have removed.");
    break;
    }
    if (ret == -1)
    err(1, "cgroup.event_control is not accessible any more");
    printf("%s %s: crossed\n", argv[1], argv[2]);
    }
    return 0;
    }
