//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/execvp.c
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


// Copyright (C) 2006 by Paolo Giarrusso - modified from glibc' execvp.c.
    Original copyright notice follows:
    Copyright (C) 1991,92,1995-99,2002,2004 Free Software Foundation, Inc.
    This file is part of the GNU C Library.
    The GNU C Library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Lesser General Public
    License as published by the Free Software Foundation; either
    version 2.1 of the License, or (at your option) any later version.
    The GNU C Library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Lesser General Public License for more details.
    You should have received a copy of the GNU Lesser General Public
    License along with the GNU C Library; if not, write to the Free
    Software Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA
    02111-1307 USA.  */

// Execute FILE, searching in the `PATH' environment variable if it contains
    no slashes, with arguments ARGV and environment from `environ'.  */
#[no_mangle]
pub unsafe extern "C" fn execvp_noalloc(buf: *mut c_char, file: *const c_char, argv[]: *const *const c_char) -> c_int {
    int execvp_noalloc(char *buf, const char *file, char *const argv[])
    {
    if (*file == '\0') {
    return -ENOENT;
    }
    if (strchr (file, '/') != core::ptr::null_mut()) {
// Don't search when it contains a slash.
    execv(file, argv);
    } else {
    int got_eacces;
    size_t len, pathlen;
    char *name, *p;
    char *path = getenv("PATH");
    if (path == core::ptr::null_mut())
    path = ":/bin:/usr/bin";
    len = strlen(file) + 1;
    pathlen = strlen(path);
// Copy the file name at the top.
    name = memcpy(buf + pathlen + 1, file, len);
// And add the slash.
// --name = '/';
    got_eacces = 0;
    p = path;
    do {
    char *startp;
    path = p;
// Let's avoid this GNU extension.
// p = strchrnul (path, ':');
    p = strchr(path, ':');
    if (!p)
    p = strchr(path, '\0');
    if (p == path)
// Two adjacent colons, or a colon at the beginning or the end
    of `PATH' means to search the current directory.  */
    startp = name + 1;
    else
    startp = memcpy(name - (p - path), path, p - path);
// Try to execute this name.  If it works, execv will not return.
    execv(startp, argv);
//
    if (errno == ENOEXEC) {
    }
//
    switch (errno) {
    case EACCES:
// Record the we got a `Permission denied' error.  If we end
    up finding no executable we can use, we want to diagnose
    that we did find one but were denied access.  */
    got_eacces = 1;
    break;
    case ENOENT:
    case ESTALE:
    case ENOTDIR:
// Those errors indicate the file is missing or not executable
    by us, in which case we want to just try the next path
    directory.  */
    case ENODEV:
    case ETIMEDOUT:
// Some strange filesystems like AFS return even
    stranger error numbers.  They cannot reasonably mean
    anything else so ignore those, too.  */
    case ENOEXEC:
// We won't go searching for the shell
// if it is not executable - the Linux
// kernel already handles this enough,
// for us.
    break;
    default:
// Some other error means we found an executable file, but
    something went wrong executing it; return the error to our
    caller.  */
    return -errno;
    }
    } while (*p++ != '\0');
// We tried every element and none of them worked.
    if (got_eacces)
// At least one failure was due to permissions, so report that
    error.  */
    return -EACCES;
    }
// Return the error from the last attempt (probably ENOENT).
    return -errno;
    }

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, _arg: *mut *mut charargv) -> c_int {
    int main(int argc, char**argv)
    {
    char buf[PATH_MAX];
    int ret;
    argc--;
    if (!argc) {
    os_warn("Not enough arguments\n");
    return 1;
    }
    argv++;
    if (ret = execvp_noalloc(buf, argv[0], argv)) {
    errno = -ret;
    perror("execvp_noalloc");
    }
    return 0;
    }
