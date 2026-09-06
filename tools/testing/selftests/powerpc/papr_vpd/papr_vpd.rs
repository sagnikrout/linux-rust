//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/papr_vpd/papr_vpd.c
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
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
unsafe extern "C" fn dev_papr_vpd_open_close() -> c_int {
    static int dev_papr_vpd_open_close(void)
    {
    let mut devfd: c_int = open(DEVPATH, O_RDONLY);
    SKIP_IF_MSG(devfd < 0 && errno == ENOENT,
    DEVPATH " not present");
    FAIL_IF(devfd < 0);
    FAIL_IF(close(devfd) != 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dev_papr_vpd_get_handle_all() -> c_int {
    static int dev_papr_vpd_get_handle_all(void)
    {
    let mut devfd: c_int = open(DEVPATH, O_RDONLY);
    let mut lc: papr_location_code = { .str = "", };
    off_t size;
    int fd;
    SKIP_IF_MSG(devfd < 0 && errno == ENOENT,
    DEVPATH " not present");
    FAIL_IF(devfd < 0);
    errno = 0;
    fd = ioctl(devfd, PAPR_VPD_IOC_CREATE_HANDLE, &lc);
    FAIL_IF(errno != 0);
    FAIL_IF(fd < 0);
    FAIL_IF(close(devfd) != 0);
    size = lseek(fd, 0, SEEK_END);
    FAIL_IF(size <= 0);
    void *buf = malloc((size_t)size);
    FAIL_IF(!buf);
    let mut consumed: isize = pread(fd, buf, size, 0);
    FAIL_IF(consumed != size);
// Ensure EOF
    FAIL_IF(read(fd, buf, size) != 0);
    FAIL_IF(close(fd));
// Verify that the buffer looks like VPD
    static const char needle[] = "System VPD";
    FAIL_IF(!memmem(buf, size, needle, strlen(needle)));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dev_papr_vpd_get_handle_byte_at_a_time() -> c_int {
    static int dev_papr_vpd_get_handle_byte_at_a_time(void)
    {
    let mut devfd: c_int = open(DEVPATH, O_RDONLY);
    let mut lc: papr_location_code = { .str = "", };
    int fd;
    SKIP_IF_MSG(devfd < 0 && errno == ENOENT,
    DEVPATH " not present");
    FAIL_IF(devfd < 0);
    errno = 0;
    fd = ioctl(devfd, PAPR_VPD_IOC_CREATE_HANDLE, &lc);
    FAIL_IF(errno != 0);
    FAIL_IF(fd < 0);
    FAIL_IF(close(devfd) != 0);
    let mut consumed: usize = 0;
    while (1) {
    ssize_t res;
    char c;
    errno = 0;
    res = read(fd, &c, sizeof(c));
    FAIL_IF(res > sizeof(c));
    FAIL_IF(res < 0);
    FAIL_IF(errno != 0);
    consumed += res;
    if (res == 0)
    break;
    }
    FAIL_IF(consumed != lseek(fd, 0, SEEK_END));
    FAIL_IF(close(fd));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dev_papr_vpd_unterm_loc_code() -> c_int {
    static int dev_papr_vpd_unterm_loc_code(void)
    {
    let mut devfd: c_int = open(DEVPATH, O_RDONLY);
    let mut lc: papr_location_code = {};
    int fd;
    SKIP_IF_MSG(devfd < 0 && errno == ENOENT,
    DEVPATH " not present");
    FAIL_IF(devfd < 0);
//
// Place a non-null byte in every element of loc_code; the
// driver should reject this input.
//
    memset(lc.str, 'x', ARRAY_SIZE(lc.str));
    errno = 0;
    fd = ioctl(devfd, PAPR_VPD_IOC_CREATE_HANDLE, &lc);
    FAIL_IF(fd != -1);
    FAIL_IF(errno != EINVAL);
    FAIL_IF(close(devfd) != 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dev_papr_vpd_null_handle() -> c_int {
    static int dev_papr_vpd_null_handle(void)
    {
    let mut devfd: c_int = open(DEVPATH, O_RDONLY);
    int rc;
    SKIP_IF_MSG(devfd < 0 && errno == ENOENT,
    DEVPATH " not present");
    FAIL_IF(devfd < 0);
    errno = 0;
    rc = ioctl(devfd, PAPR_VPD_IOC_CREATE_HANDLE, core::ptr::null_mut());
    FAIL_IF(rc != -1);
    FAIL_IF(errno != EFAULT);
    FAIL_IF(close(devfd) != 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn papr_vpd_close_handle_without_reading() -> c_int {
    static int papr_vpd_close_handle_without_reading(void)
    {
    let mut devfd: c_int = open(DEVPATH, O_RDONLY);
    let mut lc: papr_location_code = { .str = "", };
    int fd;
    SKIP_IF_MSG(devfd < 0 && errno == ENOENT,
    DEVPATH " not present");
    FAIL_IF(devfd < 0);
    errno = 0;
    fd = ioctl(devfd, PAPR_VPD_IOC_CREATE_HANDLE, &lc);
    FAIL_IF(errno != 0);
    FAIL_IF(fd < 0);
// close the handle without reading it
    FAIL_IF(close(fd) != 0);
    FAIL_IF(close(devfd) != 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn papr_vpd_reread() -> c_int {
    static int papr_vpd_reread(void)
    {
    let mut devfd: c_int = open(DEVPATH, O_RDONLY);
    let mut lc: papr_location_code = { .str = "", };
    int fd;
    SKIP_IF_MSG(devfd < 0 && errno == ENOENT,
    DEVPATH " not present");
    FAIL_IF(devfd < 0);
    errno = 0;
    fd = ioctl(devfd, PAPR_VPD_IOC_CREATE_HANDLE, &lc);
    FAIL_IF(errno != 0);
    FAIL_IF(fd < 0);
    FAIL_IF(close(devfd) != 0);
    let mut size: off_t = lseek(fd, 0, SEEK_END);
    FAIL_IF(size <= 0);
    char *bufs[2];
    for (size_t i = 0; i < ARRAY_SIZE(bufs); ++i) {
    bufs[i] = malloc(size);
    FAIL_IF(!bufs[i]);
    let mut consumed: isize = pread(fd, bufs[i], size, 0);
    FAIL_IF(consumed != size);
    }
    FAIL_IF(memcmp(bufs[0], bufs[1], size));
    FAIL_IF(close(fd) != 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_system_loc_code(lc: *mut papr_location_code) -> c_int {
    static int get_system_loc_code(struct papr_location_code *lc)
    {
    static const char system_id_path[] = "/sys/firmware/devicetree/base/system-id";
    static const char model_path[] = "/sys/firmware/devicetree/base/model";
    char *system_id;
    char *model;
    let mut err: c_int = -1;
    if (read_file_alloc(model_path, &model, core::ptr::null_mut()))
    return err;
    if (read_file_alloc(system_id_path, &system_id, core::ptr::null_mut()))
    goto free_model;
    char *mtm;
    let mut sscanf_ret: c_int = sscanf(model, "IBM,%ms", &mtm);
    if (sscanf_ret != 1)
    goto free_system_id;
    char *plant_and_seq;
    if (sscanf(system_id, "IBM,%*c%*c%ms", &plant_and_seq) != 1)
    goto free_mtm;
//
// Replace - with . to build location code.
//
    char *sep = strchr(mtm, '-');
    if (!sep)
    goto free_mtm;
    else
// sep = '.';
    snprintf(lc.str, sizeof(lc.str),
    "U%s.%s", mtm, plant_and_seq);
    err = 0;
    free(plant_and_seq);
    free_mtm:
    free(mtm);
    free_system_id:
    free(system_id);
    free_model:
    free(model);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn papr_vpd_system_loc_code() -> c_int {
    static int papr_vpd_system_loc_code(void)
    {
    struct papr_location_code lc;
    let mut devfd: c_int = open(DEVPATH, O_RDONLY);
    off_t size;
    int fd;
    SKIP_IF_MSG(devfd < 0 && errno == ENOENT,
    DEVPATH " not present");
    SKIP_IF_MSG(get_system_loc_code(&lc),
    "Cannot determine system location code");
    FAIL_IF(devfd < 0);
    errno = 0;
    fd = ioctl(devfd, PAPR_VPD_IOC_CREATE_HANDLE, &lc);
    FAIL_IF(errno != 0);
    FAIL_IF(fd < 0);
    FAIL_IF(close(devfd) != 0);
    size = lseek(fd, 0, SEEK_END);
    FAIL_IF(size <= 0);
    void *buf = malloc((size_t)size);
    FAIL_IF(!buf);
    let mut consumed: isize = pread(fd, buf, size, 0);
    FAIL_IF(consumed != size);
// Ensure EOF
    FAIL_IF(read(fd, buf, size) != 0);
    FAIL_IF(close(fd));
// Verify that the buffer looks like VPD
    static const char needle[] = "System VPD";
    FAIL_IF(!memmem(buf, size, needle, strlen(needle)));
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpd_test {
    pub (*function)(void): *mut c_int,
    pub description: *const c_char,
}

    static const struct vpd_test vpd_tests[] = {
    {
    .function = dev_papr_vpd_open_close,
    .description = "open/close " DEVPATH,
    },
    {
    .function = dev_papr_vpd_unterm_loc_code,
    .description = "ensure EINVAL on unterminated location code",
    },
    {
    .function = dev_papr_vpd_null_handle,
    .description = "ensure EFAULT on bad handle addr",
    },
    {
    .function = dev_papr_vpd_get_handle_all,
    .description = "get handle for all VPD"
    },
    {
    .function = papr_vpd_close_handle_without_reading,
    .description = "close handle without consuming VPD"
    },
    {
    .function = dev_papr_vpd_get_handle_byte_at_a_time,
    .description = "read all VPD one byte at a time"
    },
    {
    .function = papr_vpd_reread,
    .description = "ensure re-read yields same results"
    },
    {
    .function = papr_vpd_system_loc_code,
    .description = "get handle for system VPD"
    },
    };
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    let mut fails: usize = 0;
    for (size_t i = 0; i < ARRAY_SIZE(vpd_tests); ++i) {
    const struct vpd_test *t = &vpd_tests[i];
    if (test_harness(t.function, t.description))
    ++fails;
    }
    let mut fails: return = = 0 ? EXIT_SUCCESS : EXIT_FAILURE;
    }
