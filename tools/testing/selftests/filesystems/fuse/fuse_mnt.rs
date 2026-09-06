//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/fuse/fuse_mnt.c
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
// fusectl test file-system
// Creates a simple FUSE filesystem with a single read-write file (/test)
//
pub const FUSE_USE_VERSION: c_int = 26;

    static char *content;
    let mut content_size: static size_t = 0;
    static const char test_path[] = "/test";
#[no_mangle]
unsafe extern "C" fn test_getattr(path: *const c_char, st: *mut stat) -> c_int {
    static int test_getattr(const char *path, struct stat *st)
    {
    memset(st, 0, sizeof(*st));
    if (!strcmp(path, "/")) {
    st.st_mode = S_IFDIR | 0755;
    st.st_nlink = 2;
    return 0;
    }
    if (!strcmp(path, test_path)) {
    st.st_mode = S_IFREG | 0664;
    st.st_nlink = 1;
    st.st_size = content_size;
    return 0;
    }
    return -ENOENT;
    }
    static int test_readdir(const char *path, void *buf, fuse_fill_dir_t filler,
    off_t offset, struct fuse_file_info *fi)
    {
    if (strcmp(path, "/"))
    return -ENOENT;
    filler(buf, ".", core::ptr::null_mut(), 0);
    filler(buf, "..", core::ptr::null_mut(), 0);
    filler(buf, test_path + 1, core::ptr::null_mut(), 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_open(path: *const c_char, fi: *mut fuse_file_info) -> c_int {
    static int test_open(const char *path, struct fuse_file_info *fi)
    {
    if (strcmp(path, test_path))
    return -ENOENT;
    return 0;
    }
    static int test_read(const char *path, char *buf, size_t size, off_t offset,
    struct fuse_file_info *fi)
    {
    if (strcmp(path, test_path) != 0)
    return -ENOENT;
    if (!content || content_size == 0)
    return 0;
    if (offset >= content_size)
    return 0;
    if (offset + size > content_size)
    size = content_size - offset;
    memcpy(buf, content + offset, size);
    return size;
    }
    static int test_write(const char *path, const char *buf, size_t size,
    off_t offset, struct fuse_file_info *fi)
    {
    size_t new_size;
    if (strcmp(path, test_path) != 0)
    return -ENOENT;
    if(offset > content_size)
    return -EINVAL;
    new_size = MAX(offset + size, content_size);
    if (new_size > content_size)
    content = realloc(content, new_size);
    content_size = new_size;
    if (!content)
    return -ENOMEM;
    memcpy(content + offset, buf, size);
    return size;
    }
#[no_mangle]
unsafe extern "C" fn test_truncate(path: *const c_char, size: off_t) -> c_int {
    static int test_truncate(const char *path, off_t size)
    {
    if (strcmp(path, test_path) != 0)
    return -ENOENT;
    if (size == 0) {
    free(content);
    content = core::ptr::null_mut();
    content_size = 0;
    return 0;
    }
    content = realloc(content, size);
    if (!content)
    return -ENOMEM;
    if (size > content_size)
    memset(content + content_size, 0, size - content_size);
    content_size = size;
    return 0;
    }
    static struct fuse_operations memfd_ops = {
    .getattr = test_getattr,
    .readdir = test_readdir,
    .open = test_open,
    .read = test_read,
    .write = test_write,
    .truncate = test_truncate,
    };
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    return fuse_main(argc, argv, &memfd_ops, core::ptr::null_mut());
    }
