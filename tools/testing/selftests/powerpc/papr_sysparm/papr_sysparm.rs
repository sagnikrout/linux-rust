//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/papr_sysparm/papr_sysparm.c
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


// SPDX-License-Identifier: GPL-2.0-only

#[no_mangle]
unsafe extern "C" fn open_close() -> c_int {
    static int open_close(void)
    {
    let mut devfd: c_int = open(DEVPATH, O_RDONLY);
    SKIP_IF_MSG(devfd < 0 && errno == ENOENT,
    DEVPATH " not present");
    FAIL_IF(devfd < 0);
    FAIL_IF(close(devfd) != 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_splpar() -> c_int {
    static int get_splpar(void)
    {
    struct papr_sysparm_io_block sp = {
    .parameter = 20, // SPLPAR characteristics
    };
    let mut devfd: c_int = open(DEVPATH, O_RDONLY);
    SKIP_IF_MSG(devfd < 0 && errno == ENOENT,
    DEVPATH " not present");
    FAIL_IF(devfd < 0);
    FAIL_IF(ioctl(devfd, PAPR_SYSPARM_IOC_GET, &sp) != 0);
    FAIL_IF(sp.length == 0);
    FAIL_IF(sp.length > sizeof(sp.data));
    FAIL_IF(close(devfd) != 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_bad_parameter() -> c_int {
    static int get_bad_parameter(void)
    {
    struct papr_sysparm_io_block sp = {
    .parameter = UINT32_MAX, // there are only ~60 specified parameters
    };
    let mut devfd: c_int = open(DEVPATH, O_RDONLY);
    SKIP_IF_MSG(devfd < 0 && errno == ENOENT,
    DEVPATH " not present");
    FAIL_IF(devfd < 0);
// Ensure expected error
    FAIL_IF(ioctl(devfd, PAPR_SYSPARM_IOC_GET, &sp) != -1);
    FAIL_IF(errno != EOPNOTSUPP);
// Ensure the buffer is unchanged
    FAIL_IF(sp.length != 0);
    for (size_t i = 0; i < ARRAY_SIZE(sp.data); ++i)
    FAIL_IF(sp.data[i] != 0);
    FAIL_IF(close(devfd) != 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_efault_common(cmd: c_ulong) -> c_int {
    static int check_efault_common(unsigned long cmd)
    {
    let mut devfd: c_int = open(DEVPATH, O_RDWR);
    SKIP_IF_MSG(devfd < 0 && errno == ENOENT,
    DEVPATH " not present");
    FAIL_IF(devfd < 0);
// Ensure expected error
    FAIL_IF(ioctl(devfd, cmd, core::ptr::null_mut()) != -1);
    FAIL_IF(errno != EFAULT);
    FAIL_IF(close(devfd) != 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_efault_get() -> c_int {
    static int check_efault_get(void)
    {
    return check_efault_common(PAPR_SYSPARM_IOC_GET);
    }
#[no_mangle]
unsafe extern "C" fn check_efault_set() -> c_int {
    static int check_efault_set(void)
    {
    return check_efault_common(PAPR_SYSPARM_IOC_SET);
    }
#[no_mangle]
unsafe extern "C" fn set_hmc0() -> c_int {
    static int set_hmc0(void)
    {
    struct papr_sysparm_io_block sp = {
    .parameter = 0, // HMC0, not a settable parameter
    };
    let mut devfd: c_int = open(DEVPATH, O_RDWR);
    SKIP_IF_MSG(devfd < 0 && errno == ENOENT,
    DEVPATH " not present");
    FAIL_IF(devfd < 0);
// Ensure expected error
    FAIL_IF(ioctl(devfd, PAPR_SYSPARM_IOC_SET, &sp) != -1);
    SKIP_IF_MSG(errno == EOPNOTSUPP, "operation not supported");
    FAIL_IF(errno != EPERM);
    FAIL_IF(close(devfd) != 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_with_ro_fd() -> c_int {
    static int set_with_ro_fd(void)
    {
    struct papr_sysparm_io_block sp = {
    .parameter = 0, // HMC0, not a settable parameter.
    };
    let mut devfd: c_int = open(DEVPATH, O_RDONLY);
    SKIP_IF_MSG(devfd < 0 && errno == ENOENT,
    DEVPATH " not present");
    FAIL_IF(devfd < 0);
// Ensure expected error
    FAIL_IF(ioctl(devfd, PAPR_SYSPARM_IOC_SET, &sp) != -1);
    SKIP_IF_MSG(errno == EOPNOTSUPP, "operation not supported");
// HMC0 isn't a settable parameter and we would normally
// expect to get EPERM on attempts to modify it. However, when
// the file is open read-only, we expect the driver to prevent
// the attempt with a distinct error.
    FAIL_IF(errno != EBADF);
    FAIL_IF(close(devfd) != 0);
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysparm_test {
    pub (*function)(void): *mut c_int,
    pub description: *const c_char,
}

    static const struct sysparm_test sysparm_tests[] = {
    {
    .function = open_close,
    .description = "open and close " DEVPATH " without issuing commands",
    },
    {
    .function = get_splpar,
    .description = "retrieve SPLPAR characteristics",
    },
    {
    .function = get_bad_parameter,
    .description = "verify EOPNOTSUPP for known-bad parameter",
    },
    {
    .function = check_efault_get,
    .description = "PAPR_SYSPARM_IOC_GET returns EFAULT on bad address",
    },
    {
    .function = check_efault_set,
    .description = "PAPR_SYSPARM_IOC_SET returns EFAULT on bad address",
    },
    {
    .function = set_hmc0,
    .description = "ensure EPERM on attempt to update HMC0",
    },
    {
    .function = set_with_ro_fd,
    .description = "PAPR_IOC_SYSPARM_SET returns EACCES on read-only fd",
    },
    };
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    let mut fails: usize = 0;
    for (size_t i = 0; i < ARRAY_SIZE(sysparm_tests); ++i) {
    const struct sysparm_test *t = &sysparm_tests[i];
    if (test_harness(t.function, t.description))
    ++fails;
    }
    let mut fails: return = = 0 ? EXIT_SUCCESS : EXIT_FAILURE;
    }
