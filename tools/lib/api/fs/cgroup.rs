//! Automatically rewritten from C to Rust
//! Source: tools/lib/api/fs/cgroup.c
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
pub struct cgroupfs_cache_entry {
    pub subsys: [c_char; 32],
    pub mountpoint: [c_char; PATH_MAX],
}

// just cache last used one
    static struct cgroupfs_cache_entry *cached;
#[no_mangle]
pub unsafe extern "C" fn cgroupfs_find_mountpoint(buf: *mut c_char, maxlen: usize, subsys: *const c_char) -> c_int {
    int cgroupfs_find_mountpoint(char *buf, size_t maxlen, const char *subsys)
    {
    FILE *fp;
    char *line = core::ptr::null_mut();
    let mut len: usize = 0;
    char *p, *path;
    char mountpoint[PATH_MAX];
    if (cached && !strcmp(cached.subsys, subsys)) {
    if (strlen(cached.mountpoint) < maxlen) {
    strcpy(buf, cached.mountpoint);
    return 0;
    }
    return -1;
    }
    fp = fopen("/proc/mounts", "r");
    if (!fp)
    return -1;
//
// in order to handle split hierarchy, we need to scan /proc/mounts
// and inspect every cgroupfs mount point to find one that has
// the given subsystem.  If we found v1, just use it.  If not we can
// use v2 path as a fallback.
//
    mountpoint[0] = '\0';
//
// The /proc/mounts has the follow format:
//
// <devname> <mount point> <fs type> <options> ...
//
    while (getline(&line, &len, fp) != -1) {
// skip devname
    p = strchr(line, ' ');
    if (p == core::ptr::null_mut())
    continue;
// save the mount point
    path = ++p;
    p = strchr(p, ' ');
    if (p == core::ptr::null_mut())
    continue;
// p++ = '\0';
// check filesystem type
    if (strncmp(p, "cgroup", 6))
    continue;
    if (p[6] == '2') {
// save cgroup v2 path
    strcpy(mountpoint, path);
    continue;
    }
// now we have cgroup v1, check the options for subsystem
    p += 7;
    p = strstr(p, subsys);
    if (p == core::ptr::null_mut())
    continue;
// sanity check: it should be separated by a space or a comma
    if (!strchr(" ,", p[-1]) || !strchr(" ,", p[strlen(subsys)]))
    continue;
    strcpy(mountpoint, path);
    break;
    }
    free(line);
    fclose(fp);
    if (!cached)
    cached = calloc(1, sizeof(*cached));
    if (cached) {
    strncpy(cached.subsys, subsys, sizeof(cached.subsys) - 1);
    strcpy(cached.mountpoint, mountpoint);
    }
    if (mountpoint[0] && strlen(mountpoint) < maxlen) {
    strcpy(buf, mountpoint);
    return 0;
    }
    return -1;
    }
