//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/find-map.c
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

#[no_mangle]
unsafe extern "C" fn find_map(start: *mut c_void, end: *mut c_void, name: *const c_char) -> c_int {
    static int find_map(void **start, void **end, const char *name)
    {
    FILE *maps;
    char *line = core::ptr::null_mut();
    let mut len: usize = 0;
    let mut found: c_int = 0;
    maps = fopen("/proc/self/maps", "r");
    if (!maps) {
    fprintf(stderr, "cannot open maps\n");
    return -1;
    }
    while (!found && getline(&line, &len, maps) != -1) {
    let mut m: c_int = -1;
// We care only about private r-x mappings.
    if (2 != sscanf(line, "%p-%p r-xp %*x %*x:%*x %*u %n",
    start, end, &m))
    continue;
    if (m < 0)
    continue;
    if (!strncmp(&line[m], name, strlen(name)))
    found = 1;
    }
    free(line);
    fclose(maps);
    return !found;
    }
