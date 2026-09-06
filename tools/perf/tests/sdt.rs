//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/sdt.c
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

// To test SDT event, we need libelf support to scan elf binary

#[no_mangle]
unsafe extern "C" fn target_function() -> c_int {
    static int target_function(void)
    {
    DTRACE_PROBE(perf, test_target);
    return TEST_OK;
    }
// Copied from builtin-buildid-cache.c
#[no_mangle]
unsafe extern "C" fn build_id_cache__add_file(filename: *const c_char) -> c_int {
    static int build_id_cache__add_file(const char *filename)
    {
    char sbuild_id[SBUILD_ID_SIZE];
    let mut bid: build_id = { .size = 0, };
    int err;
    err = filename__read_build_id(filename, &bid);
    if (err < 0) {
    pr_debug("Failed to read build id of %s\n", filename);
    return err;
    }
    build_id__snprintf(&bid, sbuild_id, sizeof(sbuild_id));
    err = build_id_cache__add_s(sbuild_id, filename, core::ptr::null_mut(), false, false);
    if (err < 0)
    pr_debug("Failed to add build id cache of %s\n", filename);
    return err;
    }
    static char *get_self_path(void)
    {
    char *buf = calloc(PATH_MAX, sizeof(char));
    if (buf && readlink("/proc/self/exe", buf, PATH_MAX - 1) < 0) {
    pr_debug("Failed to get correct path of perf\n");
    free(buf);
    return core::ptr::null_mut();
    }
    return buf;
    }
    static int search_cached_probe(const char *target,
    const char *group, const char *event)
    {
    struct probe_cache *cache = probe_cache__new(target, core::ptr::null_mut());
    let mut ret: c_int = 0;
    if (!cache) {
    pr_debug("Failed to open probe cache of %s\n", target);
    return -EINVAL;
    }
    if (!probe_cache__find_by_name(cache, group, event)) {
    pr_debug("Failed to find %s:%s in the cache\n", group, event);
    ret = -ENOENT;
    }
    probe_cache__delete(cache);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test__sdt_event(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtests) -> c_int {
    static int test__sdt_event(struct test_suite *test __maybe_unused, int subtests __maybe_unused)
    {
    let mut ret: c_int = TEST_FAIL;
    char __tempdir[] = "./test-buildid-XXXXXX";
    char *tempdir = core::ptr::null_mut(), *myself = get_self_path();
    if (myself == core::ptr::null_mut() || mkdtemp(__tempdir) == core::ptr::null_mut()) {
    pr_debug("Failed to make a tempdir for build-id cache\n");
    goto error;
    }
// Note that buildid_dir must be an absolute path
    tempdir = realpath(__tempdir, core::ptr::null_mut());
    if (tempdir == core::ptr::null_mut())
    goto error_rmdir;
// At first, scan itself
    set_buildid_dir(tempdir);
    if (build_id_cache__add_file(myself) < 0)
    goto error_rmdir;
// Open a cache and make sure the SDT is stored
    if (search_cached_probe(myself, "sdt_perf", "test_target") < 0)
    goto error_rmdir;
// TBD: probing on the SDT event and collect logs
// Call the target and get an event
    ret = target_function();
    error_rmdir:
// Cleanup temporary buildid dir
    rm_rf(__tempdir);
    error:
    free(tempdir);
    free(myself);
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn test__sdt_event(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtests) -> c_int {
    static int test__sdt_event(struct test_suite *test __maybe_unused, int subtests __maybe_unused)
    {
    pr_debug("Skip SDT event test because SDT support is not compiled\n");
    return TEST_SKIP;
    }

    DEFINE_SUITE("Probe SDT events", sdt_event);
