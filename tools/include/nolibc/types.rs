//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/types.h
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


// SPDX-License-Identifier: LGPL-2.1 OR MIT
//
// Special types used by various syscalls for NOLIBC
// Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
//
// make sure to include all global symbols

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: i64,
}

// Never use with system calls
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: i64,
}

// Only the generic macros and types may be defined here. The arch-specific
// ones such as the O_RDONLY and related macros used by fcntl() and open()
// must not be defined here.
//
// stat flags (WARNING, octal here). We need to check for an existing
// definition because linux/stat.h may omit to define those if it finds
// that any glibc header was already included.
//

pub const S_IFDIR: c_int = 0040000;
pub const S_IFCHR: c_int = 0020000;
pub const S_IFBLK: c_int = 0060000;
pub const S_IFREG: c_int = 0100000;
pub const S_IFIFO: c_int = 0010000;
pub const S_IFLNK: c_int = 0120000;
pub const S_IFSOCK: c_int = 0140000;
pub const S_IFMT: c_int = 0170000;

pub const S_IRWXU: c_int = 00700;
pub const S_IRUSR: c_int = 00400;
pub const S_IWUSR: c_int = 00200;
pub const S_IXUSR: c_int = 00100;
pub const S_IRWXG: c_int = 00070;
pub const S_IRGRP: c_int = 00040;
pub const S_IWGRP: c_int = 00020;
pub const S_IXGRP: c_int = 00010;
pub const S_IRWXO: c_int = 00007;
pub const S_IROTH: c_int = 00004;
pub const S_IWOTH: c_int = 00002;
pub const S_IXOTH: c_int = 00001;

// dirent types
pub const DT_UNKNOWN: c_uint = 0x0;
pub const DT_FIFO: c_uint = 0x1;
pub const DT_CHR: c_uint = 0x2;
pub const DT_DIR: c_uint = 0x4;
pub const DT_BLK: c_uint = 0x6;
pub const DT_REG: c_uint = 0x8;
pub const DT_LNK: c_uint = 0xa;
pub const DT_SOCK: c_uint = 0xc;
// PATH_MAX and MAXPATHLEN are often used and found with plenty of different
// values.
//

pub const PATH_MAX: c_int = 4096;

// flags for mmap

// whence values for lseek()
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;
// flags for reboot

// Macros used on waitpid()'s return status

// standard exit() codes
pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;
// for getdents64()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct linux_dirent64 {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: c_ushort,
    pub d_type: c_uchar,
    pub d_name: [c_char; ],
}

// The format of the struct as returned by the libc to the application, which
// significantly differs from the format returned by the stat() syscall flavours.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub /: *mut *mut dev_t st_dev; / ID of device containing file,
    pub /: *mut *mut ino_t st_ino; / inode number,
    pub /: *mut *mut mode_t st_mode; / protection,
    pub /: *mut *mut nlink_t st_nlink; / number of hard links,
    pub /: *mut *mut uid_t st_uid; / user ID of owner,
    pub /: *mut *mut gid_t st_gid; / group ID of owner,
    pub /: *mut *mut dev_t st_rdev; / device ID (if special file),
    pub /: *mut *mut off_t st_size; / total size, in bytes,
    pub /: *mut *mut blksize_t st_blksize; / blocksize for file system I/O,
    pub /: *mut *mut blkcnt_t st_blocks; / number of 512B blocks allocated,
    pub /: *mut *mut { time_t st_atime; struct timespec st_atim; }; / time of last access,
    pub /: *mut *mut { time_t st_mtime; struct timespec st_mtim; }; / time of last modification,
    pub /: *mut *mut { time_t st_ctime; struct timespec st_ctim; }; / time of last status change,
}

pub type clockid_t = __kernel_clockid_t;
pub type timer_t = c_int;

