//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/expr.c
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
unsafe extern "C" fn test_ids_union() -> c_int {
    static int test_ids_union(void)
    {
    struct hashmap *ids1, *ids2;
// Empty union.
    ids1 = ids__new();
    TEST_ASSERT_VAL("ids__new", ids1);
    ids2 = ids__new();
    TEST_ASSERT_VAL("ids__new", ids2);
    ids1 = ids__union(ids1, ids2);
    TEST_ASSERT_EQUAL("union", (int)hashmap__size(ids1), 0);
// Union {foo, bar} against {}.
    ids2 = ids__new();
    TEST_ASSERT_VAL("ids__new", ids2);
    TEST_ASSERT_EQUAL("ids__insert", ids__insert(ids1, strdup("foo")), 0);
    TEST_ASSERT_EQUAL("ids__insert", ids__insert(ids1, strdup("bar")), 0);
    ids1 = ids__union(ids1, ids2);
    TEST_ASSERT_EQUAL("union", (int)hashmap__size(ids1), 2);
// Union {foo, bar} against {foo}.
    ids2 = ids__new();
    TEST_ASSERT_VAL("ids__new", ids2);
    TEST_ASSERT_EQUAL("ids__insert", ids__insert(ids2, strdup("foo")), 0);
    ids1 = ids__union(ids1, ids2);
    TEST_ASSERT_EQUAL("union", (int)hashmap__size(ids1), 2);
// Union {foo, bar} against {bar,baz}.
    ids2 = ids__new();
    TEST_ASSERT_VAL("ids__new", ids2);
    TEST_ASSERT_EQUAL("ids__insert", ids__insert(ids2, strdup("bar")), 0);
    TEST_ASSERT_EQUAL("ids__insert", ids__insert(ids2, strdup("baz")), 0);
    ids1 = ids__union(ids1, ids2);
    TEST_ASSERT_EQUAL("union", (int)hashmap__size(ids1), 3);
    ids__free(ids1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test(ctx: *mut expr_parse_ctx, e: *const c_char, val2: double) -> c_int {
    static int test(struct expr_parse_ctx *ctx, const char *e, double val2)
    {
    double val;
    if (expr__parse(&val, ctx, e))
    TEST_ASSERT_VAL("parse test failed", 0);
    TEST_ASSERT_VAL("unexpected value", val == val2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test__expr(__maybe_unused: *mut *mut test_suite t, __maybe_unused: int subtest) -> c_int {
    static int test__expr(struct test_suite *t __maybe_unused, int subtest __maybe_unused)
    {
    struct expr_id_data *val_ptr;
    const char *p;
    double val, num_cpus_online, num_cpus, num_cores, num_dies, num_packages;
    int ret;
    struct expr_parse_ctx *ctx;
    char strcmp_cpuid_buf[256];
    let mut cpu: perf_cpu = {-1};
    char *cpuid = get_cpuid_allow_env_override(cpu);
    char *escaped_cpuid1, *escaped_cpuid2;
    TEST_ASSERT_VAL("get_cpuid", cpuid);
    TEST_ASSERT_EQUAL("ids_union", test_ids_union(), 0);
    ctx = expr__ctx_new();
    TEST_ASSERT_VAL("expr__ctx_new", ctx);
    expr__add_id_val(ctx, strdup("FOO"), 1);
    expr__add_id_val(ctx, strdup("BAR"), 2);
    ret = test(ctx, "1+1", 2);
    ret |= test(ctx, "FOO+BAR", 3);
    ret |= test(ctx, "(BAR/2)%2", 1);
    ret |= test(ctx, "1 - -4",  5);
    ret |= test(ctx, "(FOO-1)*2 + (BAR/2)%2 - -4",  5);
    ret |= test(ctx, "1-1 | 1", 1);
    ret |= test(ctx, "1-1 & 1", 0);
    ret |= test(ctx, "min(1,2) + 1", 2);
    ret |= test(ctx, "max(1,2) + 1", 3);
    ret |= test(ctx, "1+1 if 3*4 else 0", 2);
    ret |= test(ctx, "100 if 1 else 200 if 1 else 300", 100);
    ret |= test(ctx, "100 if 0 else 200 if 1 else 300", 200);
    ret |= test(ctx, "100 if 1 else 200 if 0 else 300", 100);
    ret |= test(ctx, "100 if 0 else 200 if 0 else 300", 300);
    ret |= test(ctx, "1.1 + 2.1", 3.2);
    ret |= test(ctx, ".1 + 2.", 2.1);
    ret |= test(ctx, "d_ratio(1, 2)", 0.5);
    ret |= test(ctx, "d_ratio(2.5, 0)", 0);
    ret |= test(ctx, "1.1 < 2.2", 1);
    ret |= test(ctx, "2.2 > 1.1", 1);
    ret |= test(ctx, "1.1 < 1.1", 0);
    ret |= test(ctx, "2.2 > 2.2", 0);
    ret |= test(ctx, "2.2 < 1.1", 0);
    ret |= test(ctx, "1.1 > 2.2", 0);
    ret |= test(ctx, "1.1e10 < 1.1e100", 1);
    ret |= test(ctx, "1.1e2 > 1.1e-2", 1);
    if (ret) {
    expr__ctx_free(ctx);
    return ret;
    }
    p = "FOO/0";
    ret = expr__parse(&val, ctx, p);
    TEST_ASSERT_VAL("division by zero", ret == 0);
    TEST_ASSERT_VAL("division by zero", isnan(val));
    p = "BAR/";
    ret = expr__parse(&val, ctx, p);
    TEST_ASSERT_VAL("missing operand", ret == -1);
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL("find ids",
    expr__find_ids("FOO + BAR + BAZ + BOZO", "FOO",
    ctx) == 0);
    TEST_ASSERT_VAL("find ids", hashmap__size(ctx.ids) == 3);
    TEST_ASSERT_VAL("find ids", hashmap__find(ctx.ids, "BAR", &val_ptr));
    TEST_ASSERT_VAL("find ids", hashmap__find(ctx.ids, "BAZ", &val_ptr));
    TEST_ASSERT_VAL("find ids", hashmap__find(ctx.ids, "BOZO", &val_ptr));
    expr__ctx_clear(ctx);
    ctx.sctx.runtime = 3;
    TEST_ASSERT_VAL("find ids",
    expr__find_ids("EVENT1\\,param\\=?@ + EVENT2\\,param\\=?@",
    core::ptr::null_mut(), ctx) == 0);
    TEST_ASSERT_VAL("find ids", hashmap__size(ctx.ids) == 2);
    TEST_ASSERT_VAL("find ids", hashmap__find(ctx.ids, "EVENT1,param=3@", &val_ptr));
    TEST_ASSERT_VAL("find ids", hashmap__find(ctx.ids, "EVENT2,param=3@", &val_ptr));
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL("find ids",
    expr__find_ids("dash\\-event1 - dash\\-event2",
    core::ptr::null_mut(), ctx) == 0);
    TEST_ASSERT_VAL("find ids", hashmap__size(ctx.ids) == 2);
    TEST_ASSERT_VAL("find ids", hashmap__find(ctx.ids, "dash-event1", &val_ptr));
    TEST_ASSERT_VAL("find ids", hashmap__find(ctx.ids, "dash-event2", &val_ptr));
// Only EVENT1 or EVENT2 need be measured depending on the value of smt_on.
    {
    let mut smton: bool = smt_on();
    bool corewide = core_wide(/*system_wide=*/false,
// user_requested_cpus=*/false);
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL("find ids",
    expr__find_ids("EVENT1 if #smt_on else EVENT2",
    core::ptr::null_mut(), ctx) == 0);
    TEST_ASSERT_VAL("find ids", hashmap__size(ctx.ids) == 1);
    TEST_ASSERT_VAL("find ids", hashmap__find(ctx.ids,
    smton ? "EVENT1" : "EVENT2",
    &val_ptr));
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL("find ids",
    expr__find_ids("EVENT1 if #core_wide else EVENT2",
    core::ptr::null_mut(), ctx) == 0);
    TEST_ASSERT_VAL("find ids", hashmap__size(ctx.ids) == 1);
    TEST_ASSERT_VAL("find ids", hashmap__find(ctx.ids,
    corewide ? "EVENT1" : "EVENT2",
    &val_ptr));
    }
// The expression is a constant 1.0 without needing to evaluate EVENT1.
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL("find ids",
    expr__find_ids("1.0 if EVENT1 > 100.0 else 1.0",
    core::ptr::null_mut(), ctx) == 0);
    TEST_ASSERT_VAL("find ids", hashmap__size(ctx.ids) == 0);
// The expression is a constant 0.0 without needing to evaluate EVENT1.
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL("find ids",
    expr__find_ids("0 & EVENT1 > 0", core::ptr::null_mut(), ctx) == 0);
    TEST_ASSERT_VAL("find ids", hashmap__size(ctx.ids) == 0);
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL("find ids",
    expr__find_ids("EVENT1 > 0 & 0", core::ptr::null_mut(), ctx) == 0);
    TEST_ASSERT_VAL("find ids", hashmap__size(ctx.ids) == 0);
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL("find ids",
    expr__find_ids("1 & EVENT1 > 0", core::ptr::null_mut(), ctx) == 0);
    TEST_ASSERT_VAL("find ids", hashmap__size(ctx.ids) == 1);
    TEST_ASSERT_VAL("find ids", hashmap__find(ctx.ids, "EVENT1", &val_ptr));
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL("find ids",
    expr__find_ids("EVENT1 > 0 & 1", core::ptr::null_mut(), ctx) == 0);
    TEST_ASSERT_VAL("find ids", hashmap__size(ctx.ids) == 1);
    TEST_ASSERT_VAL("find ids", hashmap__find(ctx.ids, "EVENT1", &val_ptr));
// The expression is a constant 1.0 without needing to evaluate EVENT1.
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL("find ids",
    expr__find_ids("1 | EVENT1 > 0", core::ptr::null_mut(), ctx) == 0);
    TEST_ASSERT_VAL("find ids", hashmap__size(ctx.ids) == 0);
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL("find ids",
    expr__find_ids("EVENT1 > 0 | 1", core::ptr::null_mut(), ctx) == 0);
    TEST_ASSERT_VAL("find ids", hashmap__size(ctx.ids) == 0);
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL("find ids",
    expr__find_ids("0 | EVENT1 > 0", core::ptr::null_mut(), ctx) == 0);
    TEST_ASSERT_VAL("find ids", hashmap__size(ctx.ids) == 1);
    TEST_ASSERT_VAL("find ids", hashmap__find(ctx.ids, "EVENT1", &val_ptr));
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL("find ids",
    expr__find_ids("EVENT1 > 0 | 0", core::ptr::null_mut(), ctx) == 0);
    TEST_ASSERT_VAL("find ids", hashmap__size(ctx.ids) == 1);
    TEST_ASSERT_VAL("find ids", hashmap__find(ctx.ids, "EVENT1", &val_ptr));
// Test toplogy constants appear well ordered.
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL("#num_cpus_online",
    expr__parse(&num_cpus_online, ctx, "#num_cpus_online") == 0);
    TEST_ASSERT_VAL("#num_cpus", expr__parse(&num_cpus, ctx, "#num_cpus") == 0);
    TEST_ASSERT_VAL("#num_cpus >= #num_cpus_online", num_cpus >= num_cpus_online);
    TEST_ASSERT_VAL("#num_cores", expr__parse(&num_cores, ctx, "#num_cores") == 0);
    TEST_ASSERT_VAL("#num_cpus >= #num_cores", num_cpus >= num_cores);
    TEST_ASSERT_VAL("#num_dies", expr__parse(&num_dies, ctx, "#num_dies") == 0);
    TEST_ASSERT_VAL("#num_cores >= #num_dies", num_cores >= num_dies);
    TEST_ASSERT_VAL("#num_packages", expr__parse(&num_packages, ctx, "#num_packages") == 0);
    if (num_dies) // Some platforms do not have CPU die support, for example s390
    TEST_ASSERT_VAL("#num_dies >= #num_packages", num_dies >= num_packages);
    if (expr__parse(&val, ctx, "#system_tsc_freq") == 0) {
    let mut is_intel: bool = strstr(cpuid, "Intel") != core::ptr::null_mut();
    if (is_intel)
    TEST_ASSERT_VAL("#system_tsc_freq > 0", val > 0);
    else
    TEST_ASSERT_VAL("#system_tsc_freq == 0", fpclassify(val) == FP_ZERO);
    } else {

    TEST_ASSERT_VAL("#system_tsc_freq unsupported", 0);

    }
//
// Source count returns the number of events aggregating in a leader
// event including the leader. Check parsing yields an id.
//
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL("source count",
    expr__find_ids("source_count(EVENT1)",
    core::ptr::null_mut(), ctx) == 0);
    TEST_ASSERT_VAL("source count", hashmap__size(ctx.ids) == 1);
    TEST_ASSERT_VAL("source count", hashmap__find(ctx.ids, "EVENT1", &val_ptr));
// Test no cpuid match
    ret = test(ctx, "strcmp_cpuid_str(0x0)", 0);
//
// Test cpuid match with current cpuid. Special chars have to be
// escaped.
//
    escaped_cpuid1 = strreplace_chars('-', cpuid, "\\-");
    free(cpuid);
    escaped_cpuid2 = strreplace_chars(',', escaped_cpuid1, "\\,");
    free(escaped_cpuid1);
    escaped_cpuid1 = strreplace_chars('=', escaped_cpuid2, "\\=");
    free(escaped_cpuid2);
    scnprintf(strcmp_cpuid_buf, sizeof(strcmp_cpuid_buf),
    "strcmp_cpuid_str(%s)", escaped_cpuid1);
    free(escaped_cpuid1);
    ret |= test(ctx, strcmp_cpuid_buf, 1);
// has_event returns 1 when an event exists.
    expr__add_id_val(ctx, strdup("cycles"), 2);
    ret |= test(ctx, "has_event(cycles)", 1);
    expr__ctx_free(ctx);
    return ret;
    }
    DEFINE_SUITE("Simple expression parser", expr);
