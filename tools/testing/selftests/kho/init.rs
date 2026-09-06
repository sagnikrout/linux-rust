//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/kho/init.c
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

// from arch/x86/include/asm/setup.h
pub const COMMAND_LINE_SIZE: c_int = 2048;

#[no_mangle]
unsafe extern "C" fn mount_filesystems() -> c_int {
    static int mount_filesystems(void)
    {
    if (mount("debugfs", "/debugfs", "debugfs", 0, core::ptr::null_mut()) < 0)
    return -1;
    return mount("proc", "/proc", "proc", 0, core::ptr::null_mut());
    }
    static long kexec_file_load(int kernel_fd, int initrd_fd,
    unsigned long cmdline_len, const char *cmdline,
    unsigned long flags)
    {
    return syscall(__NR_kexec_file_load, kernel_fd, initrd_fd, cmdline_len,
    cmdline, flags);
    }
#[no_mangle]
unsafe extern "C" fn kexec_load() -> c_int {
    static int kexec_load(void)
    {
    char cmdline[COMMAND_LINE_SIZE];
    ssize_t len;
    int fd, err;
    fd = open("/proc/cmdline", O_RDONLY);
    if (fd < 0)
    return -1;
    len = read(fd, cmdline, sizeof(cmdline));
    close(fd);
    if (len < 0)
    return -1;
// replace \n with \0
    cmdline[len - 1] = 0;
    fd = open(KERNEL_IMAGE, O_RDONLY);
    if (fd < 0)
    return -1;
    err = kexec_file_load(fd, -1, len, cmdline, KEXEC_FILE_NO_INITRAMFS);
    close(fd);
    return err ? : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    if (mount_filesystems())
    goto err_reboot;
    if (kexec_load())
    goto err_reboot;
    if (reboot(RB_KEXEC))
    goto err_reboot;
    return 0;
    err_reboot:
    reboot(RB_AUTOBOOT);
    return -1;
    }
