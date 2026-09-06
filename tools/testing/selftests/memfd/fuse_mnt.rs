//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/memfd/fuse_mnt.c
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
// memfd test file-system
// This file uses FUSE to create a dummy file-system with only one file /memfd.
// This file is read-only and takes 1s per read.
//
// This file-system is used by the memfd test-cases to force the kernel to pin
// pages during reads(). Due to the 1s delay of this file-system, this is a
// nice way to test race-conditions against get_user_pages() in the kernel.
//
// We use direct_io==1 to force the kernel to use direct-IO for this
// file-system.
//
pub const FUSE_USE_VERSION: c_int = 26;

    static const char memfd_content[] = "memfd-example-content";
    static const char memfd_path[] = "/memfd";
#[no_mangle]
unsafe extern "C" fn memfd_getattr(path: *const c_char, st: *mut stat) -> c_int {
    static int memfd_getattr(const char *path, struct stat *st)
    {
    memset(st, 0, sizeof(*st));
    if (!strcmp(path, "/")) {
    st.st_mode = S_IFDIR | 0755;
    st.st_nlink = 2;
    } else if (!strcmp(path, memfd_path)) {
    st.st_mode = S_IFREG | 0444;
    st.st_nlink = 1;
    st.st_size = strlen(memfd_content);
    } else {
    return -ENOENT;
    }
    return 0;
    }
    static int memfd_readdir(const char *path,
    void *buf,
    fuse_fill_dir_t filler,
    off_t offset,
    struct fuse_file_info *fi)
    {
    if (strcmp(path, "/"))
    return -ENOENT;
    filler(buf, ".", core::ptr::null_mut(), 0);
    filler(buf, "..", core::ptr::null_mut(), 0);
    filler(buf, memfd_path + 1, core::ptr::null_mut(), 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn memfd_open(path: *const c_char, fi: *mut fuse_file_info) -> c_int {
    static int memfd_open(const char *path, struct fuse_file_info *fi)
    {
    if (strcmp(path, memfd_path))
    return -ENOENT;
    if ((fi.flags & 3) != O_RDONLY)
    return -EACCES;
// force direct-IO
    fi.direct_io = 1;
    return 0;
    }
    static int memfd_read(const char *path,
    char *buf,
    size_t size,
    off_t offset,
    struct fuse_file_info *fi)
    {
    size_t len;
    if (strcmp(path, memfd_path) != 0)
    return -ENOENT;
    sleep(1);
    len = strlen(memfd_content);
    if (offset < len) {
    if (offset + size > len)
    size = len - offset;
    memcpy(buf, memfd_content + offset, size);
    } else {
    size = 0;
    }
    return size;
    }
    static struct fuse_operations memfd_ops = {
    .getattr	= memfd_getattr,
    .readdir	= memfd_readdir,
    .open		= memfd_open,
    .read		= memfd_read,
    };
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    return fuse_main(argc, argv, &memfd_ops, core::ptr::null_mut());
    }
