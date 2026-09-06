//! Automatically rewritten from C to Rust
//! Source: tools/perf/dlfilters/dlfilter-test-api-v2.c
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
// Test v2 API for perf --dlfilter shared object
// Copyright (c) 2023, Intel Corporation.
//

//
// Copy v2 API instead of including current API
//

//
// The following macro can be used to determine if this header defines
// perf_dlfilter_sample machine_pid and vcpu.
//
// Macro flag: #define PERF_DLFILTER_HAS_MACHINE_PID
// Definitions for perf_dlfilter_sample flags
    enum {
    PERF_DLFILTER_FLAG_BRANCH	= 1ULL << 0,
    PERF_DLFILTER_FLAG_CALL		= 1ULL << 1,
    PERF_DLFILTER_FLAG_RETURN	= 1ULL << 2,
    PERF_DLFILTER_FLAG_CONDITIONAL	= 1ULL << 3,
    PERF_DLFILTER_FLAG_SYSCALLRET	= 1ULL << 4,
    PERF_DLFILTER_FLAG_ASYNC	= 1ULL << 5,
    PERF_DLFILTER_FLAG_INTERRUPT	= 1ULL << 6,
    PERF_DLFILTER_FLAG_TX_ABORT	= 1ULL << 7,
    PERF_DLFILTER_FLAG_TRACE_BEGIN	= 1ULL << 8,
    PERF_DLFILTER_FLAG_TRACE_END	= 1ULL << 9,
    PERF_DLFILTER_FLAG_IN_TX	= 1ULL << 10,
    PERF_DLFILTER_FLAG_VMENTRY	= 1ULL << 11,
    PERF_DLFILTER_FLAG_VMEXIT	= 1ULL << 12,
    };
//
// perf sample event information (as per perf script and <linux/perf_event.h>)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_dlfilter_sample {
    pub /: *mut *mut __u32 size; / Size of this structure (for compatibility checking),
    pub /: *mut *mut __u16 ins_lat; / Refer PERF_SAMPLE_WEIGHT_TYPE in <linux/perf_event.h>,
    pub /: *mut *mut __u16 p_stage_cyc; / Refer PERF_SAMPLE_WEIGHT_TYPE in <linux/perf_event.h>,
    pub ip: __u64,
    pub pid: __s32,
    pub tid: __s32,
    pub time: __u64,
    pub addr: __u64,
    pub id: __u64,
    pub stream_id: __u64,
    pub period: __u64,
    pub /: *mut *mut __u64 weight; / Refer PERF_SAMPLE_WEIGHT_TYPE in <linux/perf_event.h>,
    pub /: *mut *mut __u64 transaction; / Refer PERF_SAMPLE_TRANSACTION in <linux/perf_event.h>,
    pub /: *mut *mut __u64 insn_cnt; / For instructions-per-cycle (IPC),
    pub /: *mut *mut __u64 cyc_cnt; / For instructions-per-cycle (IPC),
    pub cpu: __s32,
    pub /: *mut *mut *mut __u32 flags; / Refer PERF_DLFILTER_FLAG_ above,
    pub /: *mut *mut __u64 data_src; / Refer PERF_SAMPLE_DATA_SRC in <linux/perf_event.h>,
    pub /: *mut *mut __u64 phys_addr; / Refer PERF_SAMPLE_PHYS_ADDR in <linux/perf_event.h>,
    pub /: *mut *mut __u64 data_page_size; / Refer PERF_SAMPLE_DATA_PAGE_SIZE in <linux/perf_event.h>,
    pub /: *mut *mut __u64 code_page_size; / Refer PERF_SAMPLE_CODE_PAGE_SIZE in <linux/perf_event.h>,
    pub /: *mut *mut __u64 cgroup; / Refer PERF_SAMPLE_CGROUP in <linux/perf_event.h>,
    pub /: *mut *mut __u8 cpumode; / Refer CPUMODE_MASK etc in <linux/perf_event.h>,
    pub /: *mut *mut __u8 addr_correlates_sym; / True => resolve_addr() can be called,
    pub /: *mut *mut __u16 misc; / Refer perf_event_header in <linux/perf_event.h>,
    pub /: *mut *mut __u32 raw_size; / Refer PERF_SAMPLE_RAW in <linux/perf_event.h>,
    pub /: *const *const *const void raw_data; / Refer PERF_SAMPLE_RAW in <linux/perf_event.h>,
    pub /: *mut *mut __u64 brstack_nr; / Number of brstack entries,
    pub /: *const *const *const perf_branch_entry brstack; / Refer <linux/perf_event.h>,
    pub /: *mut *mut __u64 raw_callchain_nr; / Number of raw_callchain entries,
    pub /: *const *const *const __u64 raw_callchain; / Refer <linux/perf_event.h>,
    pub event: *const c_char,
    pub machine_pid: __s32,
    pub vcpu: __s32,
}

//
// Address location (as per perf script)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_dlfilter_al {
    pub /: *mut *mut __u32 size; / Size of this structure (for compatibility checking),
    pub symoff: __u32,
    pub sym: *const c_char,
    pub /: *mut *mut __u64 addr; / Mapped address (from dso),
    pub sym_start: __u64,
    pub sym_end: __u64,
    pub dso: *const c_char,
    pub /: *mut *mut __u8 sym_binding; / STB_LOCAL, STB_GLOBAL or STB_WEAK, refer <elf.h>,
    pub /: *mut *mut __u8 is_64_bit; / Only valid if dso is not NULL,
    pub /: *mut *mut __u8 is_kernel_ip; / True if in kernel space,
    pub buildid_size: __u32,
    pub buildid: *mut __u8,
// Below members are only populated by resolve_ip()
    pub /: *mut *mut __u8 filtered; / True if this sample event will be filtered out,
    pub comm: *const c_char,
    pub /: *mut *mut *mut void priv; / Private data (v2 API),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_dlfilter_fns {
// Return information about ip
    pub ctx): *const *const *const perf_dlfilter_al (resolve_ip)(void,
// Return information about addr (if addr_correlates_sym)
    pub ctx): *const *const *const perf_dlfilter_al (resolve_addr)(void,
// Return arguments from --dlarg option
    pub dlargc): *mut *mut *mut *mut *mut char (args)(void ctx, int,
//
// Return information about address (al->size must be set before
// calling). Returns 0 on success, -1 otherwise. Call al_cleanup()
// when 'al' data is no longer needed.
//
    pub al): *mut *mut *mut __s32 (resolve_address)(void ctx, __u64 address, struct perf_dlfilter_al,
// Return instruction bytes and length
    pub length): *const *const *const *const __u8 (insn)(void ctx, __u32,
// Return source file name and line number
    pub line_number): *const *const *const *const char (srcline)(void ctx, __u32,
// Return perf_event_attr, refer <linux/perf_event.h>
    pub ctx): *mut *mut *mut perf_event_attr (attr)(void,
// Read object code, return numbers of bytes read
    pub len): *mut *mut *mut *mut __s32 (object_code)(void ctx, __u64 ip, void buf, __u32,
//
// If present (i.e. must check al_cleanup != NULL), call after
// resolve_address() to free any associated resources. (v2 API)
//
    pub al): *mut *mut *mut void (al_cleanup)(void ctx, struct perf_dlfilter_al,
// Reserved
    pub ): *mut *mut *mut void (reserved[119])(void,
}

    struct perf_dlfilter_fns perf_dlfilter_fns;
    static int verbose;

    if (verbose > 0) \
    fprintf(stderr, fmt, ##__VA_ARGS__); \
    } while (0)
#[no_mangle]
unsafe extern "C" fn test_fail(msg: *const c_char) -> c_int {
    static int test_fail(const char *msg)
    {
    pr_debug("%s\n", msg);
    return -1;
    }

    if (!(x)) \
    return test_fail("Check '" #x "' failed\n"); \
    } while (0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_data {
    pub ip: __u64,
    pub addr: __u64,
    pub do_early: c_int,
    pub early_filter_cnt: c_int,
    pub filter_cnt: c_int,
}

    static struct filter_data *filt_dat;
#[no_mangle]
pub unsafe extern "C" fn start(data: *mut c_void, ctx: *mut c_void) -> c_int {
    int start(void **data, void *ctx)
    {
    int dlargc;
    char **dlargv;
    struct filter_data *d;
    static bool called;
    verbose = 1;
    CHECK(!filt_dat && !called);
    called = true;
    d = calloc(1, sizeof(*d));
    if (!d)
    test_fail("Failed to allocate memory");
    filt_dat = d;
// data = d;
    dlargv = perf_dlfilter_fns.args(ctx, &dlargc);
    CHECK(dlargc == 6);
    CHECK(!strcmp(dlargv[0], "first"));
    verbose = strtol(dlargv[1], core::ptr::null_mut(), 0);
    d.ip = strtoull(dlargv[2], core::ptr::null_mut(), 0);
    d.addr = strtoull(dlargv[3], core::ptr::null_mut(), 0);
    d.do_early = strtol(dlargv[4], core::ptr::null_mut(), 0);
    CHECK(!strcmp(dlargv[5], "last"));
    pr_debug("%s API\n", __func__);
    return 0;
    }

    if (sample.x != expected.x) \
    return test_fail("'" #x "' not expected value\n"); \
    } while (0)
#[no_mangle]
unsafe extern "C" fn check_sample(d: *mut filter_data, sample: *const perf_dlfilter_sample) -> c_int {
    static int check_sample(struct filter_data *d, const struct perf_dlfilter_sample *sample)
    {
    struct perf_dlfilter_sample expected = {
    .ip		= d.ip,
    .pid		= 12345,
    .tid		= 12346,
    .time		= 1234567890,
    .addr		= d.addr,
    .id		= 99,
    .stream_id	= 101,
    .period		= 543212345,
    .cpu		= 31,
    .cpumode	= PERF_RECORD_MISC_USER,
    .addr_correlates_sym = 1,
    .misc		= PERF_RECORD_MISC_USER,
    };
    CHECK(sample.size >= sizeof(struct perf_dlfilter_sample));
    CHECK_SAMPLE(ip);
    CHECK_SAMPLE(pid);
    CHECK_SAMPLE(tid);
    CHECK_SAMPLE(time);
    CHECK_SAMPLE(addr);
    CHECK_SAMPLE(id);
    CHECK_SAMPLE(stream_id);
    CHECK_SAMPLE(period);
    CHECK_SAMPLE(cpu);
    CHECK_SAMPLE(cpumode);
    CHECK_SAMPLE(addr_correlates_sym);
    CHECK_SAMPLE(misc);
    CHECK(!sample.raw_data);
    CHECK_SAMPLE(brstack_nr);
    CHECK(!sample.brstack);
    CHECK_SAMPLE(raw_callchain_nr);
    CHECK(!sample.raw_callchain);

    CHECK(!strncmp(sample.event, EVENT_NAME, strlen(EVENT_NAME)));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_al(ctx: *mut c_void) -> c_int {
    static int check_al(void *ctx)
    {
    const struct perf_dlfilter_al *al;
    al = perf_dlfilter_fns.resolve_ip(ctx);
    if (!al)
    return test_fail("resolve_ip() failed");
    CHECK(al.sym && !strcmp("foo", al.sym));
    CHECK(!al.symoff);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_addr_al(ctx: *mut c_void) -> c_int {
    static int check_addr_al(void *ctx)
    {
    const struct perf_dlfilter_al *addr_al;
    addr_al = perf_dlfilter_fns.resolve_addr(ctx);
    if (!addr_al)
    return test_fail("resolve_addr() failed");
    CHECK(addr_al.sym && !strcmp("bar", addr_al.sym));
    CHECK(!addr_al.symoff);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_address_al(ctx: *mut c_void, sample: *const perf_dlfilter_sample) -> c_int {
    static int check_address_al(void *ctx, const struct perf_dlfilter_sample *sample)
    {
    struct perf_dlfilter_al address_al;
    const struct perf_dlfilter_al *al;
    al = perf_dlfilter_fns.resolve_ip(ctx);
    if (!al)
    return test_fail("resolve_ip() failed");
    address_al.size = sizeof(address_al);
    if (perf_dlfilter_fns.resolve_address(ctx, sample.ip, &address_al))
    return test_fail("resolve_address() failed");
    CHECK(address_al.sym && al.sym);
    CHECK(!strcmp(address_al.sym, al.sym));
    CHECK(address_al.addr == al.addr);
    CHECK(address_al.sym_start == al.sym_start);
    CHECK(address_al.sym_end == al.sym_end);
    CHECK(address_al.dso && al.dso);
    CHECK(!strcmp(address_al.dso, al.dso));
// al_cleanup() is v2 API so may not be present
    if (perf_dlfilter_fns.al_cleanup)
    perf_dlfilter_fns.al_cleanup(ctx, &address_al);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_attr(ctx: *mut c_void) -> c_int {
    static int check_attr(void *ctx)
    {
    struct perf_event_attr *attr = perf_dlfilter_fns.attr(ctx);
    CHECK(attr);
    CHECK(attr.type == PERF_TYPE_HARDWARE);
    CHECK(attr.config == PERF_COUNT_HW_BRANCH_INSTRUCTIONS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_object_code(ctx: *mut c_void, sample: *const perf_dlfilter_sample) -> c_int {
    static int check_object_code(void *ctx, const struct perf_dlfilter_sample *sample)
    {
    __u8 buf[15];
    CHECK(perf_dlfilter_fns.object_code(ctx, sample.ip, buf, sizeof(buf)) > 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_checks(data: *mut c_void, sample: *const perf_dlfilter_sample, ctx: *mut c_void, early: bool) -> c_int {
    static int do_checks(void *data, const struct perf_dlfilter_sample *sample, void *ctx, bool early)
    {
    struct filter_data *d = data;
    CHECK(data && filt_dat == data);
    if (early) {
    CHECK(!d.early_filter_cnt);
    d.early_filter_cnt += 1;
    } else {
    CHECK(!d.filter_cnt);
    CHECK(d.early_filter_cnt);
    CHECK(d.do_early != 2);
    d.filter_cnt += 1;
    }
    if (check_sample(data, sample))
    return -1;
    if (check_attr(ctx))
    return -1;
    if (early && !d.do_early)
    return 0;
    if (check_al(ctx) || check_addr_al(ctx) || check_address_al(ctx, sample) ||
    check_object_code(ctx, sample))
    return -1;
    if (early)
    return d.do_early == 2;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn filter_event_early(data: *mut c_void, sample: *const perf_dlfilter_sample, ctx: *mut c_void) -> c_int {
    int filter_event_early(void *data, const struct perf_dlfilter_sample *sample, void *ctx)
    {
    pr_debug("%s API\n", __func__);
    return do_checks(data, sample, ctx, true);
    }
#[no_mangle]
pub unsafe extern "C" fn filter_event(data: *mut c_void, sample: *const perf_dlfilter_sample, ctx: *mut c_void) -> c_int {
    int filter_event(void *data, const struct perf_dlfilter_sample *sample, void *ctx)
    {
    pr_debug("%s API\n", __func__);
    return do_checks(data, sample, ctx, false);
    }
#[no_mangle]
pub unsafe extern "C" fn stop(data: *mut c_void, ctx: *mut c_void) -> c_int {
    int stop(void *data, void *ctx)
    {
    static bool called;
    pr_debug("%s API\n", __func__);
    CHECK(data && filt_dat == data && !called);
    called = true;
    free(data);
    filt_dat = core::ptr::null_mut();
    return 0;
    }
    const char *filter_description(const char **long_description)
    {
// long_description = "Filter used by the 'dlfilter C API' perf test";
    return "dlfilter to test v2 C API";
    }
