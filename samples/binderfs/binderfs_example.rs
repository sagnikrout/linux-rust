//! Automatically rewritten from C to Rust
//! Source: samples/binderfs/binderfs_example.c
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
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int fd, ret, saved_errno;
    let mut device: binderfs_device = { 0 };
    ret = unshare(CLONE_NEWNS);
    if (ret < 0) {
    fprintf(stderr, "%s - Failed to unshare mount namespace\n",
    strerror(errno));
    exit(EXIT_FAILURE);
    }
    ret = mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_REC | MS_PRIVATE, 0);
    if (ret < 0) {
    fprintf(stderr, "%s - Failed to mount / as private\n",
    strerror(errno));
    exit(EXIT_FAILURE);
    }
    ret = mkdir("/dev/binderfs", 0755);
    if (ret < 0 && errno != EEXIST) {
    fprintf(stderr, "%s - Failed to create binderfs mountpoint\n",
    strerror(errno));
    exit(EXIT_FAILURE);
    }
    ret = mount(core::ptr::null_mut(), "/dev/binderfs", "binder", 0, 0);
    if (ret < 0) {
    fprintf(stderr, "%s - Failed to mount binderfs\n",
    strerror(errno));
    exit(EXIT_FAILURE);
    }
    memcpy(device.name, "my-binder", strlen("my-binder"));
    fd = open("/dev/binderfs/binder-control", O_RDONLY | O_CLOEXEC);
    if (fd < 0) {
    fprintf(stderr, "%s - Failed to open binder-control device\n",
    strerror(errno));
    exit(EXIT_FAILURE);
    }
    ret = ioctl(fd, BINDER_CTL_ADD, &device);
    saved_errno = errno;
    close(fd);
    errno = saved_errno;
    if (ret < 0) {
    fprintf(stderr, "%s - Failed to allocate new binder device\n",
    strerror(errno));
    exit(EXIT_FAILURE);
    }
    printf("Allocated new binder device with major %d, minor %d, and name %s\n",
    device.major, device.minor, device.name);
    ret = unlink("/dev/binderfs/my-binder");
    if (ret < 0) {
    fprintf(stderr, "%s - Failed to delete binder device\n",
    strerror(errno));
    exit(EXIT_FAILURE);
    }
// Cleanup happens when the mount namespace dies.
    exit(EXIT_SUCCESS);
    }
