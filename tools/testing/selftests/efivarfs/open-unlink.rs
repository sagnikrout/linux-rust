//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/efivarfs/open-unlink.c
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
unsafe extern "C" fn set_immutable(path: *const c_char, immutable: c_int) -> c_int {
    static int set_immutable(const char *path, int immutable)
    {
    unsigned int flags;
    int fd;
    int rc;
    int error;
    fd = open(path, O_RDONLY);
    if (fd < 0)
    return fd;
    rc = ioctl(fd, FS_IOC_GETFLAGS, &flags);
    if (rc < 0) {
    error = errno;
    close(fd);
    errno = error;
    return rc;
    }
    if (immutable)
    flags |= FS_IMMUTABLE_FL;
    else
    flags &= ~FS_IMMUTABLE_FL;
    rc = ioctl(fd, FS_IOC_SETFLAGS, &flags);
    error = errno;
    close(fd);
    errno = error;
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn get_immutable(path: *const c_char) -> c_int {
    static int get_immutable(const char *path)
    {
    unsigned int flags;
    int fd;
    int rc;
    int error;
    fd = open(path, O_RDONLY);
    if (fd < 0)
    return fd;
    rc = ioctl(fd, FS_IOC_GETFLAGS, &flags);
    if (rc < 0) {
    error = errno;
    close(fd);
    errno = error;
    return rc;
    }
    close(fd);
    if (flags & FS_IMMUTABLE_FL)
    return 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    const char *path;
    char buf[5];
    int fd, rc;
    if (argc < 2) {
    fprintf(stderr, "usage: %s <path>\n", argv[0]);
    return EXIT_FAILURE;
    }
    path = argv[1];
// attributes: EFI_VARIABLE_NON_VOLATILE |
// EFI_VARIABLE_BOOTSERVICE_ACCESS |
// EFI_VARIABLE_RUNTIME_ACCESS
//
// (uint32_t *)buf = 0x7;
    buf[4] = 0;
// create a test variable
    fd = open(path, O_WRONLY | O_CREAT, 0600);
    if (fd < 0) {
    perror("open(O_WRONLY)");
    return EXIT_FAILURE;
    }
    rc = write(fd, buf, sizeof(buf));
    if (rc != sizeof(buf)) {
    perror("write");
    return EXIT_FAILURE;
    }
    close(fd);
    rc = get_immutable(path);
    if (rc < 0) {
    perror("ioctl(FS_IOC_GETFLAGS)");
    return EXIT_FAILURE;
    } else if (rc) {
    rc = set_immutable(path, 0);
    if (rc < 0) {
    perror("ioctl(FS_IOC_SETFLAGS)");
    return EXIT_FAILURE;
    }
    }
    fd = open(path, O_RDONLY);
    if (fd < 0) {
    perror("open");
    return EXIT_FAILURE;
    }
    if (unlink(path) < 0) {
    perror("unlink");
    return EXIT_FAILURE;
    }
    rc = read(fd, buf, sizeof(buf));
    if (rc > 0) {
    fprintf(stderr, "reading from an unlinked variable "
    "shouldn't be possible\n");
    return EXIT_FAILURE;
    }
    return EXIT_SUCCESS;
    }
