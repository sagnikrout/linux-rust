//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/mem.c
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

    static int check(union perf_mem_data_src data_src,
    const char *string)
    {
    char out[100];
    char failure[100];
    struct mem_info *mi = mem_info__new();
    int n;
    TEST_ASSERT_VAL("Memory allocation failed", mi);
// mem_info__data_src(mi) = data_src;
    n = perf_mem__snp_scnprintf(out, sizeof out, mi);
    n += perf_mem__lvl_scnprintf(out + n, sizeof out - n, mi);
    mem_info__put(mi);
    scnprintf(failure, sizeof failure, "unexpected %s", out);
    TEST_ASSERT_VAL(failure, !strcmp(string, out));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test__mem(__maybe_unused: *mut *mut test_suite text, __maybe_unused: int subtest) -> c_int {
    static int test__mem(struct test_suite *text __maybe_unused, int subtest __maybe_unused)
    {
    let mut ret: c_int = 0;
    union perf_mem_data_src src;
    memset(&src, 0, sizeof(src));
    src.mem_lvl = PERF_MEM_LVL_HIT;
    src.mem_lvl_num = 4;
    ret |= check(src, "N/AL4 hit");
    src.mem_remote = 1;
    ret |= check(src, "N/ARemote L4 hit");
    src.mem_lvl = PERF_MEM_LVL_MISS;
    src.mem_lvl_num = PERF_MEM_LVLNUM_PMEM;
    src.mem_remote = 0;
    ret |= check(src, "N/APMEM miss");
    src.mem_remote = 1;
    ret |= check(src, "N/ARemote PMEM miss");
    src.mem_snoopx = PERF_MEM_SNOOPX_FWD;
    src.mem_lvl_num = PERF_MEM_LVLNUM_RAM;
    ret |= check(src , "FwdRemote RAM miss");
    return ret;
    }
    DEFINE_SUITE("Test data source output", mem);
