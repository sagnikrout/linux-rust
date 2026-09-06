//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/fdarray.c
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
unsafe extern "C" fn fdarray__init_revents(fda: *mut fdarray, revents: c_short) {
    static void fdarray__init_revents(struct fdarray *fda, short revents)
    {
    int fd;
    fda.nr = fda.nr_alloc;
    for (fd = 0; fd < fda.nr; ++fd) {
    fda.entries[fd].fd	 = fda.nr - fd;
    fda.entries[fd].events  = revents;
    fda.entries[fd].revents = revents;
    }
    }
#[no_mangle]
unsafe extern "C" fn fdarray__fprintf_prefix(fda: *mut fdarray, prefix: *const c_char, fp: *mut FILE) -> c_int {
    static int fdarray__fprintf_prefix(struct fdarray *fda, const char *prefix, FILE *fp)
    {
    let mut printed: c_int = 0;
    if (verbose <= 0)
    return 0;
    printed += fprintf(fp, "\n%s: ", prefix);
    return printed + fdarray__fprintf(fda, fp);
    }
#[no_mangle]
unsafe extern "C" fn test__fdarray__filter(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__fdarray__filter(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    int nr_fds, err = TEST_FAIL;
    struct fdarray *fda = fdarray__new(5, 5);
    if (fda == core::ptr::null_mut()) {
    pr_debug("\nfdarray__new() failed!");
    goto out;
    }
    fdarray__init_revents(fda, POLLIN);
    nr_fds = fdarray__filter(fda, POLLHUP, core::ptr::null_mut(), core::ptr::null_mut());
    if (nr_fds != fda.nr_alloc) {
    pr_debug("\nfdarray__filter()=%d != %d shouldn't have filtered anything",
    nr_fds, fda.nr_alloc);
    goto out_delete;
    }
    fdarray__init_revents(fda, POLLHUP);
    nr_fds = fdarray__filter(fda, POLLHUP, core::ptr::null_mut(), core::ptr::null_mut());
    if (nr_fds != 0) {
    pr_debug("\nfdarray__filter()=%d != %d, should have filtered all fds",
    nr_fds, fda.nr_alloc);
    goto out_delete;
    }
    fdarray__init_revents(fda, POLLHUP);
    fda.entries[2].revents = POLLIN;
    pr_debug("\nfiltering all but fda.entries[2]:");
    fdarray__fprintf_prefix(fda, "before", stderr);
    nr_fds = fdarray__filter(fda, POLLHUP, core::ptr::null_mut(), core::ptr::null_mut());
    fdarray__fprintf_prefix(fda, " after", stderr);
    if (nr_fds != 1) {
    pr_debug("\nfdarray__filter()=%d != 1, should have left just one event", nr_fds);
    goto out_delete;
    }
    fdarray__init_revents(fda, POLLHUP);
    fda.entries[0].revents = POLLIN;
    fda.entries[3].revents = POLLIN;
    pr_debug("\nfiltering all but (fda.entries[0], fda.entries[3]):");
    fdarray__fprintf_prefix(fda, "before", stderr);
    nr_fds = fdarray__filter(fda, POLLHUP, core::ptr::null_mut(), core::ptr::null_mut());
    fdarray__fprintf_prefix(fda, " after", stderr);
    if (nr_fds != 2) {
    pr_debug("\nfdarray__filter()=%d != 2, should have left just two events",
    nr_fds);
    goto out_delete;
    }
    pr_debug("\n");
    err = 0;
    out_delete:
    fdarray__delete(fda);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn test__fdarray__add(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__fdarray__add(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    let mut err: c_int = TEST_FAIL;
    struct fdarray *fda = fdarray__new(2, 2);
    if (fda == core::ptr::null_mut()) {
    pr_debug("\nfdarray__new() failed!");
    goto out;
    }

    if (fda.entries[_idx].fd != _fd) {				   \
    pr_debug("\n%d: fda.entries[%d](%d) != %d!",		   \
    __LINE__, _idx, fda.entries[1].fd, _fd);	   \
    goto out_delete;					   \
    }								   \
    if (fda.entries[_idx].events != (_revents)) {			   \
    pr_debug("\n%d: fda.entries[%d].revents(%d) != %d!",	   \
    __LINE__, _idx, fda.entries[_idx].fd, _revents); \
    goto out_delete;					   \
    }

    if (fdarray__add(fda, _fd, _revents, fdarray_flag__default) < 0) { \
    pr_debug("\n%d: fdarray__add(fda, %d, %d) failed!",	   \
    __LINE__,_fd, _revents);			   \
    goto out_delete;					   \
    }								   \
    if (fda.nr != _nr) {						   \
    pr_debug("\n%d: fdarray__add(fda, %d, %d)=%d != %d",	   \
    __LINE__,_fd, _revents, fda.nr, _nr);		   \
    goto out_delete;					   \
    }								   \
    FDA_CHECK(_idx, _fd, _revents)
    FDA_ADD(0, 1, POLLIN, 1);
    FDA_ADD(1, 2, POLLERR, 2);
    fdarray__fprintf_prefix(fda, "before growing array", stderr);
    FDA_ADD(2, 35, POLLHUP, 3);
    if (fda.entries == core::ptr::null_mut()) {
    pr_debug("\nfdarray__add(fda, 35, POLLHUP) should have allocated fda.pollfd!");
    goto out_delete;
    }
    fdarray__fprintf_prefix(fda, "after 3rd add", stderr);
    FDA_ADD(3, 88, POLLIN | POLLOUT, 4);
    fdarray__fprintf_prefix(fda, "after 4th add", stderr);
    FDA_CHECK(0, 1, POLLIN);
    FDA_CHECK(1, 2, POLLERR);
    FDA_CHECK(2, 35, POLLHUP);
    FDA_CHECK(3, 88, POLLIN | POLLOUT);

    pr_debug("\n");
    err = 0;
    out_delete:
    fdarray__delete(fda);
    out:
    return err;
    }
    DEFINE_SUITE("Filter fds with revents mask in a fdarray", fdarray__filter);
    DEFINE_SUITE("Add fd to a fdarray, making it autogrow", fdarray__add);
