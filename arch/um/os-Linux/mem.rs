//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/mem.c
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
// Copyright (C) 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

//
// kasan_map_memory - maps memory from @start with a size of @len.
// The allocated memory is filled with zeroes upon success.
// @start: the start address of the memory to be mapped
// @len: the length of the memory to be mapped
//
// This function is used to map shadow memory for KASAN in uml
//
#[no_mangle]
pub unsafe extern "C" fn kasan_map_memory(start: *mut c_void, len: usize) {
    void kasan_map_memory(void *start, size_t len)
    {
    if (mmap(start,
    len,
    PROT_READ|PROT_WRITE,
    MAP_FIXED|MAP_ANONYMOUS|MAP_PRIVATE|MAP_NORESERVE,
    -1,
    0) == MAP_FAILED) {
    os_info("Couldn't allocate shadow memory: %s\n.",
    strerror(errno));
    exit(1);
    }
    if (madvise(start, len, MADV_DONTDUMP)) {
    os_info("Couldn't set MAD_DONTDUMP on shadow memory: %s\n.",
    strerror(errno));
    exit(1);
    }
    if (madvise(start, len, MADV_DONTFORK)) {
    os_info("Couldn't set MADV_DONTFORK on shadow memory: %s\n.",
    strerror(errno));
    exit(1);
    }
    }
// Set by make_tempfile() during early boot.
    char *tempdir = core::ptr::null_mut();
// Check if dir is on tmpfs. Return 0 if yes, -1 if no or error.
#[no_mangle]
unsafe extern "C" fn check_tmpfs(dir: *const c_char) -> int __init {
    static int __init check_tmpfs(const char *dir)
    {
    struct statfs st;
    os_info("Checking if %s is on tmpfs...", dir);
    if (statfs(dir, &st) < 0) {
    os_info("%s\n", strerror(errno));
    } else if (st.f_type != TMPFS_MAGIC) {
    os_info("no\n");
    } else {
    os_info("OK\n");
    return 0;
    }
    return -1;
    }
//
// Choose the tempdir to use. We want something on tmpfs so that our memory is
// not subject to the host's vm.dirty_ratio. If a tempdir is specified in the
// environment, we use that even if it's not on tmpfs, but we warn the user.
// Otherwise, we try common tmpfs locations, and if no tmpfs directory is found
// then we fall back to /tmp.
//
#[no_mangle]
unsafe extern "C" fn choose_tempdir() -> *mut char  __init {
    static char * __init choose_tempdir(void)
    {
    static const char * const vars[] = {
    "TMPDIR",
    "TMP",
    "TEMP",
    core::ptr::null_mut()
    };
    static const char fallback_dir[] = "/tmp";
    static const char * const tmpfs_dirs[] = {
    "/dev/shm",
    fallback_dir,
    core::ptr::null_mut()
    };
    int i;
    const char *dir;
    os_info("Checking environment variables for a tempdir...");
    for (i = 0; vars[i]; i++) {
    dir = getenv(vars[i]);
    if ((dir != core::ptr::null_mut()) && (*dir != '\0')) {
    os_info("%s\n", dir);
    if (check_tmpfs(dir) >= 0)
    goto done;
    else
    goto warn;
    }
    }
    os_info("none found\n");
    for (i = 0; tmpfs_dirs[i]; i++) {
    dir = tmpfs_dirs[i];
    if (check_tmpfs(dir) >= 0)
    goto done;
    }
    dir = fallback_dir;
    warn:
    os_warn("Warning: tempdir %s is not on tmpfs\n", dir);
    done:
// Make a copy since getenv results may not remain valid forever.
    return strdup(dir);
    }
//
// Create an unlinked tempfile in a suitable tempdir. template must be the
// basename part of the template with a leading '/'.
//
#[no_mangle]
unsafe extern "C" fn make_tempfile(template: *const c_char) -> int __init {
    static int __init make_tempfile(const char *template)
    {
    char *tempname;
    int fd;
    if (tempdir == core::ptr::null_mut()) {
    tempdir = choose_tempdir();
    if (tempdir == core::ptr::null_mut()) {
    os_warn("Failed to choose tempdir: %s\n",
    strerror(errno));
    return -1;
    }
    }

    fd = open(tempdir, O_CLOEXEC | O_RDWR | O_EXCL | O_TMPFILE, 0700);
//
// If the running system does not support O_TMPFILE flag then retry
// without it.
//
    if (fd != -1 || (errno != EINVAL && errno != EISDIR &&
    errno != EOPNOTSUPP))
    return fd;

    tempname = malloc(strlen(tempdir) + strlen(template) + 1);
    if (tempname == core::ptr::null_mut())
    return -1;
    strcpy(tempname, tempdir);
    strcat(tempname, template);
    fd = mkstemp(tempname);
    if (fd < 0) {
    os_warn("open - cannot create %s: %s\n", tempname,
    strerror(errno));
    goto out;
    }
    if (unlink(tempname) < 0) {
    perror("unlink");
    goto close;
    }
    free(tempname);
    return fd;
    close:
    close(fd);
    out:
    free(tempname);
    return -1;
    }

#[no_mangle]
unsafe extern "C" fn create_tmp_file(len: c_ulonglong) -> int __init {
    static int __init create_tmp_file(unsigned long long len)
    {
    int fd, err;
    char zero;
    fd = make_tempfile(TEMPNAME_TEMPLATE);
    if (fd < 0)
    exit(1);
//
// Seek to len - 1 because writing a character there will
// increase the file size by one byte, to the desired length.
//
    if (lseek64(fd, len - 1, SEEK_SET) < 0) {
    perror("lseek64");
    exit(1);
    }
    zero = 0;
    err = write(fd, &zero, 1);
    if (err != 1) {
    perror("write");
    exit(1);
    }
    return fd;
    }
#[no_mangle]
pub unsafe extern "C" fn create_mem_file(len: c_ulonglong) -> int __init {
    int __init create_mem_file(unsigned long long len)
    {
    int err, fd;
    fd = create_tmp_file(len);
    err = os_set_exec_close(fd);
    if (err < 0) {
    errno = -err;
    perror("exec_close");
    }
    return fd;
    }
#[no_mangle]
pub unsafe extern "C" fn check_tmpexec() -> void __init {
    void __init check_tmpexec(void)
    {
    void *addr;
    int err, fd = create_tmp_file(UM_KERN_PAGE_SIZE);
    addr = mmap(core::ptr::null_mut(), UM_KERN_PAGE_SIZE,
    PROT_READ | PROT_WRITE | PROT_EXEC, MAP_PRIVATE, fd, 0);
    os_info("Checking PROT_EXEC mmap in %s...", tempdir);
    if (addr == MAP_FAILED) {
    err = errno;
    os_warn("%s\n", strerror(err));
    close(fd);
    if (err == EPERM)
    os_warn("%s must be not mounted noexec\n", tempdir);
    exit(1);
    }
    os_info("OK\n");
    munmap(addr, UM_KERN_PAGE_SIZE);
    close(fd);
    }
