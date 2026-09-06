//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/ublk/metadata_size.c
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
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut cap: logical_block_metadata_cap = {};
    const char *filename;
    int fd;
    int result;
    if (argc != 2) {
    fprintf(stderr, "Usage: %s BLOCK_DEVICE\n", argv[0]);
    return 1;
    }
    filename = argv[1];
    fd = open(filename, O_RDONLY);
    if (fd < 0) {
    perror(filename);
    return 1;
    }
    result = ioctl(fd, FS_IOC_GETLBMD_CAP, &cap);
    if (result < 0) {
    perror("ioctl");
    return 1;
    }
    printf("metadata_size: %u\n", cap.lbmd_size);
    printf("pi_offset: %u\n", cap.lbmd_pi_offset);
    printf("pi_tuple_size: %u\n", cap.lbmd_pi_size);
    return 0;
    }
