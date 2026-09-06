//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/mode_t.c
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


// SPDX-License-Identifier: LGPL-2.1

// From include/linux/stat.h

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_mode_t(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_mode_t(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut show_prefix: bool = arg.show_string_prefix;
    const char *prefix = "S_";
    let mut printed: c_int = 0, mode = arg.val;

    if ((mode & S_##n) == S_##n) { \
    printed += scnprintf(bf + printed, size - printed, "%s%s%s", printed ? "|" : "", show_prefix ? prefix : "", #n); \
    mode &= ~S_##n; \
    }
    P_MODE(IALLUGO);
    P_MODE(IRWXUGO);
    P_MODE(IRUGO);
    P_MODE(IWUGO);
    P_MODE(IXUGO);
    P_MODE(IFMT);
    P_MODE(IFSOCK);
    P_MODE(IFLNK);
    P_MODE(IFREG);
    P_MODE(IFBLK);
    P_MODE(IFDIR);
    P_MODE(IFCHR);
    P_MODE(IFIFO);
    P_MODE(ISUID);
    P_MODE(ISGID);
    P_MODE(ISVTX);
    P_MODE(IRWXU);
    P_MODE(IRUSR);
    P_MODE(IWUSR);
    P_MODE(IXUSR);
    P_MODE(IRWXG);
    P_MODE(IRGRP);
    P_MODE(IWGRP);
    P_MODE(IXGRP);
    P_MODE(IRWXO);
    P_MODE(IROTH);
    P_MODE(IWOTH);
    P_MODE(IXOTH);

    if (mode)
    printed += scnprintf(bf + printed, size - printed, "%s%#x", printed ? "|" : "", mode);
    return printed;
    }
