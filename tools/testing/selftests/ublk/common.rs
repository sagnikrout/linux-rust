//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/ublk/common.c
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
pub unsafe extern "C" fn backing_file_tgt_deinit(dev: *mut ublk_dev) {
    void backing_file_tgt_deinit(struct ublk_dev *dev)
    {
    int i;
    for (i = 1; i < dev.nr_fds; i++) {
    fsync(dev.fds[i]);
    close(dev.fds[i]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn backing_file_tgt_init(dev: *mut ublk_dev, nr_direct: c_uint) -> c_int {
    int backing_file_tgt_init(struct ublk_dev *dev, unsigned int nr_direct)
    {
    int fd, i;
    ublk_assert(dev.nr_fds == 1);
    for (i = 0; i < dev.tgt.nr_backing_files; i++) {
    char *file = dev.tgt.backing_file[i];
    unsigned long bytes;
    struct stat st;
    ublk_dbg(UBLK_DBG_DEV, "%s: file %d: %s\n", __func__, i, file);
    fd = open(file, O_RDWR | (i < nr_direct ? O_DIRECT : 0));
    if (fd < 0) {
    ublk_err("%s: backing file %s can't be opened: %s\n",
    __func__, file, strerror(errno));
    return -EBADF;
    }
    if (fstat(fd, &st) < 0) {
    close(fd);
    return -EBADF;
    }
    if (S_ISREG(st.st_mode))
    bytes = st.st_size;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: S_ISBLK(st.st_mode)) -> else {
    if (ioctl(fd, BLKGETSIZE64, &bytes) != 0)
    return -1;
    } else {
    return -EINVAL;
    }
    dev.tgt.backing_file_size[i] = bytes;
    dev.fds[dev.nr_fds] = fd;
    dev.nr_fds += 1;
    }
    return 0;
    }
