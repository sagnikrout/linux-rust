//! Automatically rewritten from C to Rust
//! Source: samples/vfs/test-fsmount.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// fd-based mount test.
//
// Copyright (C) 2017 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

#[no_mangle]
unsafe extern "C" fn check_messages(fd: c_int) {
    static void check_messages(int fd)
    {
    char buf[4096];
    int err, n;
    err = errno;
    for (;;) {
    n = read(fd, buf, sizeof(buf));
    if (n < 0)
    break;
    n -= 2;
    switch (buf[0]) {
    case 'e':
    fprintf(stderr, "Error: %*.*s\n", n, n, buf + 2);
    break;
    case 'w':
    fprintf(stderr, "Warning: %*.*s\n", n, n, buf + 2);
    break;
    case 'i':
    fprintf(stderr, "Info: %*.*s\n", n, n, buf + 2);
    break;
    }
    }
    errno = err;
    }
#[no_mangle]
pub unsafe extern "C" fn __attribute__(_arg: (noreturn)) -> static {
    static __attribute__((noreturn))
#[no_mangle]
pub unsafe extern "C" fn mount_error(fd: c_int, s: *const c_char) {
    void mount_error(int fd, const char *s)
    {
    check_messages(fd);
    fprintf(stderr, "%s: %m\n", s);
    exit(1);
    }
// Hope -1 isn't a syscall

#[no_mangle]
pub unsafe extern "C" fn fsopen(fs_name: *const c_char, flags: c_uint) -> c_int {
    static inline int fsopen(const char *fs_name, unsigned int flags)
    {
    return syscall(__NR_fsopen, fs_name, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn fsmount(fsfd: c_int, flags: c_uint, ms_flags: c_uint) -> c_int {
    static inline int fsmount(int fsfd, unsigned int flags, unsigned int ms_flags)
    {
    return syscall(__NR_fsmount, fsfd, flags, ms_flags);
    }
    static inline int fsconfig(int fsfd, unsigned int cmd,
    const char *key, const void *val, int aux)
    {
    return syscall(__NR_fsconfig, fsfd, cmd, key, val, aux);
    }
    static inline int move_mount(int from_dfd, const char *from_pathname,
    int to_dfd, const char *to_pathname,
    unsigned int flags)
    {
    return syscall(__NR_move_mount,
    from_dfd, from_pathname,
    to_dfd, to_pathname, flags);
    }

    do {								\
    if (fsconfig(fd, cmd, key, val, aux) == -1)		\
    mount_error(fd, key ?: "create");		\
    } while (0)
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int fsfd, mfd;
// Mount a publically available AFS filesystem
    fsfd = fsopen("afs", 0);
    if (fsfd == -1) {
    perror("fsopen");
    exit(1);
    }
    E_fsconfig(fsfd, FSCONFIG_SET_STRING, "source", "#grand.central.org:root.cell.", 0);
    E_fsconfig(fsfd, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0);
    mfd = fsmount(fsfd, 0, MOUNT_ATTR_RDONLY);
    if (mfd < 0)
    mount_error(fsfd, "fsmount");
    E(close(fsfd));
    if (move_mount(mfd, "", AT_FDCWD, "/mnt", MOVE_MOUNT_F_EMPTY_PATH) < 0) {
    perror("move_mount");
    exit(1);
    }
    E(close(mfd));
    exit(0);
    }
