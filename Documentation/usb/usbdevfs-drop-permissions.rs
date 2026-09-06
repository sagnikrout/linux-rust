//! Automatically rewritten from C to Rust
//! Source: Documentation/usb/usbdevfs-drop-permissions.c
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


// For building without an updated set of headers

pub const USBDEVFS_CAP_DROP_PRIVILEGES: c_uint = 0x40;

#[no_mangle]
pub unsafe extern "C" fn drop_privileges(fd: c_int, mask: u32) {
    void drop_privileges(int fd, uint32_t mask)
    {
    int res;
    res = ioctl(fd, USBDEVFS_DROP_PRIVILEGES, &mask);
    if (res)
    printf("ERROR: USBDEVFS_DROP_PRIVILEGES returned %d\n", res);
    else
    printf("OK: privileges dropped!\n");
    }
#[no_mangle]
pub unsafe extern "C" fn reset_device(fd: c_int) {
    void reset_device(int fd)
    {
    int res;
    res = ioctl(fd, USBDEVFS_RESET);
    if (!res)
    printf("OK: USBDEVFS_RESET succeeded\n");
    else
    printf("ERROR: reset failed! (%d - %s)\n",
    -res, strerror(-res));
    }
#[no_mangle]
pub unsafe extern "C" fn claim_some_intf(fd: c_int) {
    void claim_some_intf(int fd)
    {
    int i, res;
    for (i = 0; i < 4; i++) {
    res = ioctl(fd, USBDEVFS_CLAIMINTERFACE, &i);
    if (!res)
    printf("OK: claimed if %d\n", i);
    else
    printf("ERROR claiming if %d (%d - %s)\n",
    i, -res, strerror(-res));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    uint32_t mask, caps;
    int c, fd;
    fd = open(argv[1], O_RDWR);
    if (fd < 0) {
    printf("Failed to open file\n");
    goto err_fd;
    }
//
// check if dropping privileges is supported,
// bail on systems where the capability is not present
//
    ioctl(fd, USBDEVFS_GET_CAPABILITIES, &caps);
    if (!(caps & USBDEVFS_CAP_DROP_PRIVILEGES)) {
    printf("DROP_PRIVILEGES not supported\n");
    goto err;
    }
//
// Drop privileges but keep the ability to claim all
// free interfaces (i.e., those not used by kernel drivers)
//
    drop_privileges(fd, -1U);
    printf("Available options:\n"
    "[0] Exit now\n"
    "[1] Reset device. Should fail if device is in use\n"
    "[2] Claim 4 interfaces. Should succeed where not in use\n"
    "[3] Narrow interface permission mask\n"
    "Which option shall I run?: ");
    while (scanf("%d", &c) == 1) {
    switch (c) {
    case 0:
    goto exit;
    case 1:
    reset_device(fd);
    break;
    case 2:
    claim_some_intf(fd);
    break;
    case 3:
    printf("Insert new mask: ");
    scanf("%x", &mask);
    drop_privileges(fd, mask);
    break;
    default:
    printf("I don't recognize that\n");
    }
    printf("Which test shall I run next?: ");
    }
    exit:
    close(fd);
    return 0;
    err:
    close(fd);
    err_fd:
    return 1;
    }
