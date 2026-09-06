//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/ia64/aliasing-test.c
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
//
// Exercise /dev/mem mmap cases that have been troublesome in the past
//
// (c) Copyright 2007 Hewlett-Packard Development Company, L.P.
// Bjorn Helgaas <bjorn.helgaas@hp.com>
//

    int sum;
#[no_mangle]
unsafe extern "C" fn map_mem(path: *mut c_char, offset: off_t, length: usize, touch: c_int) -> c_int {
    static int map_mem(char *path, off_t offset, size_t length, int touch)
    {
    int fd, rc;
    void *addr;
    int *c;
    fd = open(path, O_RDWR);
    if (fd == -1) {
    perror(path);
    return -1;
    }
    if (fnmatch("/proc/bus/pci/*", path, 0) == 0) {
    rc = ioctl(fd, PCIIOC_MMAP_IS_MEM);
    if (rc == -1)
    perror("PCIIOC_MMAP_IS_MEM ioctl");
    }
    addr = mmap(core::ptr::null_mut(), length, PROT_READ|PROT_WRITE, MAP_SHARED, fd, offset);
    if (addr == MAP_FAILED)
    return 1;
    if (touch) {
    c = (int *) addr;
    while (c < (int *) (addr + length))
    sum += *c++;
    }
    rc = munmap(addr, length);
    if (rc == -1) {
    perror("munmap");
    return -1;
    }
    close(fd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn scan_tree(path: *mut c_char, file: *mut c_char, offset: off_t, length: usize, touch: c_int) -> c_int {
    static int scan_tree(char *path, char *file, off_t offset, size_t length, int touch)
    {
    struct dirent **namelist;
    char *name, *path2;
    int i, n, r, rc = 0, result = 0;
    struct stat buf;
    n = scandir(path, &namelist, 0, alphasort);
    if (n < 0) {
    perror("scandir");
    return -1;
    }
    for (i = 0; i < n; i++) {
    name = namelist[i].d_name;
    if (fnmatch(".", name, 0) == 0)
    goto skip;
    if (fnmatch("..", name, 0) == 0)
    goto skip;
    path2 = malloc(strlen(path) + strlen(name) + 3);
    strcpy(path2, path);
    strcat(path2, "/");
    strcat(path2, name);
    if (fnmatch(file, name, 0) == 0) {
    rc = map_mem(path2, offset, length, touch);
    if (rc == 0)
    fprintf(stderr, "PASS: %s 0x%lx-0x%lx is %s\n", path2, offset, offset + length, touch ? "readable" : "mappable");
#[no_mangle]
pub unsafe extern "C" fn if(0: rc >) -> else {
    else if (rc > 0)
    fprintf(stderr, "PASS: %s 0x%lx-0x%lx not mappable\n", path2, offset, offset + length);
    else {
    fprintf(stderr, "FAIL: %s 0x%lx-0x%lx not accessible\n", path2, offset, offset + length);
    return rc;
    }
    } else {
    r = lstat(path2, &buf);
    if (r == 0 && S_ISDIR(buf.st_mode)) {
    rc = scan_tree(path2, file, offset, length, touch);
    if (rc < 0)
    return rc;
    }
    }
    result |= rc;
    free(path2);
    skip:
    free(namelist[i]);
    }
    free(namelist);
    return result;
    }
    char buf[1024];
#[no_mangle]
unsafe extern "C" fn read_rom(path: *mut c_char) -> c_int {
    static int read_rom(char *path)
    {
    int fd, rc;
    let mut size: usize = 0;
    fd = open(path, O_RDWR);
    if (fd == -1) {
    perror(path);
    return -1;
    }
    rc = write(fd, "1", 2);
    if (rc <= 0) {
    close(fd);
    perror("write");
    return -1;
    }
    do {
    rc = read(fd, buf, sizeof(buf));
    if (rc > 0)
    size += rc;
    } while (rc > 0);
    close(fd);
    return size;
    }
#[no_mangle]
unsafe extern "C" fn scan_rom(path: *mut c_char, file: *mut c_char) -> c_int {
    static int scan_rom(char *path, char *file)
    {
    struct dirent **namelist;
    char *name, *path2;
    int i, n, r, rc = 0, result = 0;
    struct stat buf;
    n = scandir(path, &namelist, 0, alphasort);
    if (n < 0) {
    perror("scandir");
    return -1;
    }
    for (i = 0; i < n; i++) {
    name = namelist[i].d_name;
    if (fnmatch(".", name, 0) == 0)
    goto skip;
    if (fnmatch("..", name, 0) == 0)
    goto skip;
    path2 = malloc(strlen(path) + strlen(name) + 3);
    strcpy(path2, path);
    strcat(path2, "/");
    strcat(path2, name);
    if (fnmatch(file, name, 0) == 0) {
    rc = read_rom(path2);
//
// It's OK if the ROM is unreadable.  Maybe there
// is no ROM, or some other error occurred.  The
// important thing is that no MCA happened.
//
    if (rc > 0)
    fprintf(stderr, "PASS: %s read %d bytes\n", path2, rc);
    else {
    fprintf(stderr, "PASS: %s not readable\n", path2);
    return rc;
    }
    } else {
    r = lstat(path2, &buf);
    if (r == 0 && S_ISDIR(buf.st_mode)) {
    rc = scan_rom(path2, file);
    if (rc < 0)
    return rc;
    }
    }
    result |= rc;
    free(path2);
    skip:
    free(namelist[i]);
    }
    free(namelist);
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    int rc;
    if (map_mem("/dev/mem", 0, 0xA0000, 1) == 0)
    fprintf(stderr, "PASS: /dev/mem 0x0-0xa0000 is readable\n");
    else
    fprintf(stderr, "FAIL: /dev/mem 0x0-0xa0000 not accessible\n");
//
// It's not safe to blindly read the VGA frame buffer.  If you know
// how to poke the card the right way, it should respond, but it's
// not safe in general.  Many machines, e.g., Intel chipsets, cover
// up a non-responding card by just returning -1, but others will
// report the failure as a machine check.
//
    if (map_mem("/dev/mem", 0xA0000, 0x20000, 0) == 0)
    fprintf(stderr, "PASS: /dev/mem 0xa0000-0xc0000 is mappable\n");
    else
    fprintf(stderr, "FAIL: /dev/mem 0xa0000-0xc0000 not accessible\n");
    if (map_mem("/dev/mem", 0xC0000, 0x40000, 1) == 0)
    fprintf(stderr, "PASS: /dev/mem 0xc0000-0x100000 is readable\n");
    else
    fprintf(stderr, "FAIL: /dev/mem 0xc0000-0x100000 not accessible\n");
//
// Often you can map all the individual pieces above (0-0xA0000,
// 0xA0000-0xC0000, and 0xC0000-0x100000), but can't map the whole
// thing at once.  This is because the individual pieces use different
// attributes, and there's no single attribute supported over the
// whole region.
//
    rc = map_mem("/dev/mem", 0, 1024*1024, 0);
    if (rc == 0)
    fprintf(stderr, "PASS: /dev/mem 0x0-0x100000 is mappable\n");
#[no_mangle]
pub unsafe extern "C" fn if(0: rc >) -> else {
    else if (rc > 0)
    fprintf(stderr, "PASS: /dev/mem 0x0-0x100000 not mappable\n");
    else
    fprintf(stderr, "FAIL: /dev/mem 0x0-0x100000 not accessible\n");
    scan_tree("/sys/class/pci_bus", "legacy_mem", 0, 0xA0000, 1);
    scan_tree("/sys/class/pci_bus", "legacy_mem", 0xA0000, 0x20000, 0);
    scan_tree("/sys/class/pci_bus", "legacy_mem", 0xC0000, 0x40000, 1);
    scan_tree("/sys/class/pci_bus", "legacy_mem", 0, 1024*1024, 0);
    scan_rom("/sys/devices", "rom");
    scan_tree("/proc/bus/pci", "??.?", 0, 0xA0000, 1);
    scan_tree("/proc/bus/pci", "??.?", 0xA0000, 0x20000, 0);
    scan_tree("/proc/bus/pci", "??.?", 0xC0000, 0x40000, 1);
    scan_tree("/proc/bus/pci", "??.?", 0, 1024*1024, 0);
    return rc;
    }
