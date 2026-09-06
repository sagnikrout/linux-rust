//! Automatically rewritten from C to Rust
//! Source: tools/tracing/rtla/tests/unit/cli_opt_callback.c
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
// Macro flag: #define _GNU_SOURCE

// Macro flag: #define RTLA_ALLOW_CLI_P_H

    RTLA_OPT_CALLBACK_DATA('t', "test", value, "test value", "test help", \
    opt_llong_callback, LLONG_RANGE(lo, hi))

    RTLA_OPT_CALLBACK_DATA('t', "test", value, "test value", "test help", \
    opt_int_callback, INT_RANGE(lo, hi))
    START_TEST(test_opt_llong_callback_simple)
    {
    let mut test_value: c_longlong = 0;
    let mut opt: option = TEST_CALLBACK(&test_value, opt_llong_callback);
    ck_assert_int_eq(opt_llong_callback(&opt, "1234567890", 0), 0);
    ck_assert_int_eq(test_value, 1234567890);
    }
    END_TEST
    START_TEST(test_opt_llong_callback_max)
    {
    let mut test_value: c_longlong = 0;
    let mut opt: option = TEST_CALLBACK(&test_value, opt_llong_callback);
    ck_assert_int_eq(opt_llong_callback(&opt, "9223372036854775807", 0), 0);
    ck_assert_int_eq(test_value, 9223372036854775807LL);
    }
    END_TEST
    START_TEST(test_opt_llong_callback_min)
    {
    let mut test_value: c_longlong = 0;
    let mut opt: option = TEST_CALLBACK(&test_value, opt_llong_callback);
    ck_assert_int_eq(opt_llong_callback(&opt, "-9223372036854775808", 0), 0);
    ck_assert_int_eq(test_value, ~9223372036854775807LL);
    }
    END_TEST
    START_TEST(test_opt_llong_callback_non_numeric)
    {
    let mut test_value: c_longlong = 0;
    let mut opt: option = TEST_CALLBACK(&test_value, opt_llong_callback);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_llong_callback(&opt, "abc", 0), -1);
    ck_assert_int_eq(test_value, 0);
    }
    END_TEST
    START_TEST(test_opt_llong_callback_non_numeric_suffix)
    {
    let mut test_value: c_longlong = 0;
    let mut opt: option = TEST_CALLBACK(&test_value, opt_llong_callback);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_llong_callback(&opt, "1234567890abc", 0), -1);
    ck_assert_int_eq(test_value, 0);
    }
    END_TEST
    START_TEST(test_opt_llong_callback_unset)
    {
    let mut test_value: c_longlong = 0;
    let mut opt: option = TEST_CALLBACK(&test_value, opt_llong_callback);
    ck_assert_int_eq(opt_llong_callback(&opt, "1234567890", 0), 0);
    ck_assert_int_eq(opt_llong_callback(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_int_eq(test_value, 0);
    }
    END_TEST
    START_TEST(test_opt_llong_callback_unset_defval)
    {
    let mut test_value: c_longlong = 0;
    let mut default_value: c_longlong = 42;
    const struct option opt = RTLA_OPT_LLONG_DEFVAL('t', "test", &test_value, "test value",
    "test help", &default_value);
    ck_assert_int_eq(opt_llong_callback(&opt, "1234567890", 0), 0);
    ck_assert_int_eq(opt_llong_callback(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_int_eq(test_value, default_value);
    }
    END_TEST
    START_TEST(test_opt_int_callback_simple)
    {
    let mut test_value: c_int = 0;
    let mut opt: option = TEST_CALLBACK(&test_value, opt_int_callback);
    ck_assert_int_eq(opt_int_callback(&opt, "1234567890", 0), 0);
    ck_assert_int_eq(test_value, 1234567890);
    }
    END_TEST
    START_TEST(test_opt_int_callback_max)
    {
    let mut test_value: c_int = 0;
    let mut opt: option = TEST_CALLBACK(&test_value, opt_int_callback);
    ck_assert_int_eq(opt_int_callback(&opt, "2147483647", 0), 0);
    ck_assert_int_eq(test_value, 2147483647);
    }
    END_TEST
    START_TEST(test_opt_int_callback_min)
    {
    let mut test_value: c_int = 0;
    let mut opt: option = TEST_CALLBACK(&test_value, opt_int_callback);
    ck_assert_int_eq(opt_int_callback(&opt, "-2147483648", 0), 0);
    ck_assert_int_eq(test_value, -2147483648);
    }
    END_TEST
    START_TEST(test_opt_int_callback_non_numeric)
    {
    let mut test_value: c_int = 0;
    let mut opt: option = TEST_CALLBACK(&test_value, opt_int_callback);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_int_callback(&opt, "abc", 0), -1);
    ck_assert_int_eq(test_value, 0);
    }
    END_TEST
    START_TEST(test_opt_int_callback_non_numeric_suffix)
    {
    let mut test_value: c_int = 0;
    let mut opt: option = TEST_CALLBACK(&test_value, opt_int_callback);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_int_callback(&opt, "1234567890abc", 0), -1);
    ck_assert_int_eq(test_value, 0);
    }
    END_TEST
    START_TEST(test_opt_int_callback_unset)
    {
    let mut test_value: c_int = 0;
    let mut opt: option = TEST_CALLBACK(&test_value, opt_int_callback);
    ck_assert_int_eq(opt_int_callback(&opt, "1234567890", 0), 0);
    ck_assert_int_eq(opt_int_callback(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_int_eq(test_value, 0);
    }
    END_TEST
    START_TEST(test_opt_int_callback_unset_defval)
    {
    let mut test_value: c_int = 0;
    const struct option opt = RTLA_OPT_INT_DEFVAL('t', "test", &test_value, "test value",
    "test help", 42);
    ck_assert_int_eq(opt_int_callback(&opt, "1234567890", 0), 0);
    ck_assert_int_eq(opt_int_callback(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_int_eq(test_value, 42);
    }
    END_TEST
    START_TEST(test_opt_llong_callback_range_in)
    {
    let mut test_value: c_longlong = 0;
    let mut opt: option = TEST_LLONG_RANGE(&test_value, 10, 100);
    ck_assert_int_eq(opt_llong_callback(&opt, "50", 0), 0);
    ck_assert_int_eq(test_value, 50);
    }
    END_TEST
    START_TEST(test_opt_llong_callback_range_below)
    {
    let mut test_value: c_longlong = 0;
    let mut opt: option = TEST_LLONG_RANGE(&test_value, 10, 100);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_llong_callback(&opt, "9", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_llong_callback_range_above)
    {
    let mut test_value: c_longlong = 0;
    let mut opt: option = TEST_LLONG_RANGE(&test_value, 10, 100);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_llong_callback(&opt, "101", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_llong_callback_range_boundary)
    {
    let mut test_value: c_longlong = 0;
    let mut opt: option = TEST_LLONG_RANGE(&test_value, 10, 100);
    ck_assert_int_eq(opt_llong_callback(&opt, "10", 0), 0);
    ck_assert_int_eq(test_value, 10);
    ck_assert_int_eq(opt_llong_callback(&opt, "100", 0), 0);
    ck_assert_int_eq(test_value, 100);
    }
    END_TEST
    START_TEST(test_opt_int_callback_range_in)
    {
    let mut test_value: c_int = 0;
    let mut opt: option = TEST_INT_RANGE(&test_value, 0, 10000);
    ck_assert_int_eq(opt_int_callback(&opt, "5000", 0), 0);
    ck_assert_int_eq(test_value, 5000);
    }
    END_TEST
    START_TEST(test_opt_int_callback_range_below)
    {
    let mut test_value: c_int = 0;
    let mut opt: option = TEST_INT_RANGE(&test_value, 0, 10000);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_int_callback(&opt, "-1", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_int_callback_range_above)
    {
    let mut test_value: c_int = 0;
    let mut opt: option = TEST_INT_RANGE(&test_value, 0, 10000);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_int_callback(&opt, "10001", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_int_callback_range_boundary)
    {
    let mut test_value: c_int = 0;
    let mut opt: option = TEST_INT_RANGE(&test_value, 0, 10000);
    ck_assert_int_eq(opt_int_callback(&opt, "0", 0), 0);
    ck_assert_int_eq(test_value, 0);
    ck_assert_int_eq(opt_int_callback(&opt, "10000", 0), 0);
    ck_assert_int_eq(test_value, 10000);
    }
    END_TEST
    START_TEST(test_opt_cpus_cb)
    {
    let mut params: common_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_cpus_cb);
    nr_cpus = 4;
    ck_assert_int_eq(opt_cpus_cb(&opt, "0-3", 0), 0);
    ck_assert_str_eq(params.cpus, "0-3");
    }
    END_TEST
    START_TEST(test_opt_cpus_cb_invalid)
    {
    let mut params: common_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_cpus_cb);
    nr_cpus = 4;
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_cpus_cb(&opt, "0-3,5", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_cgroup_cb)
    {
    let mut params: common_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_cgroup_cb);
    ck_assert_int_eq(opt_cgroup_cb(&opt, "cgroup", 0), 0);
    ck_assert_int_eq(params.cgroup, 1);
    ck_assert_str_eq(params.cgroup_name, "cgroup");
    }
    END_TEST
    START_TEST(test_opt_cgroup_cb_equals)
    {
    let mut params: common_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_cgroup_cb);
    ck_assert_int_eq(opt_cgroup_cb(&opt, "=cgroup", 0), 0);
    ck_assert_int_eq(params.cgroup, 1);
    ck_assert_str_eq(params.cgroup_name, "cgroup");
    }
    END_TEST
    START_TEST(test_opt_cgroup_cb_unset)
    {
    let mut params: common_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_cgroup_cb);
    ck_assert_int_eq(opt_cgroup_cb(&opt, "cgroup", 0), 0);
    ck_assert_int_eq(opt_cgroup_cb(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_int_eq(params.cgroup, 0);
    ck_assert_ptr_null(params.cgroup_name);
    }
    END_TEST
    START_TEST(test_opt_duration_cb)
    {
    let mut params: common_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_duration_cb);
    ck_assert_int_eq(opt_duration_cb(&opt, "1m", 0), 0);
    ck_assert_int_eq(params.duration, 60);
    }
    END_TEST
    START_TEST(test_opt_duration_cb_invalid)
    {
    let mut params: common_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_duration_cb);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_duration_cb(&opt, "abc", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_duration_cb_unset)
    {
    let mut params: common_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_duration_cb);
    ck_assert_int_eq(opt_duration_cb(&opt, "1m", 0), 0);
    ck_assert_int_eq(opt_duration_cb(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_int_eq(params.duration, 0);
    }
    END_TEST
    START_TEST(test_opt_event_cb)
    {
    struct trace_events *events = core::ptr::null_mut();
    let mut opt: option = TEST_CALLBACK(&events, opt_event_cb);
    ck_assert_int_eq(opt_event_cb(&opt, "sched:sched_switch", 0), 0);
    ck_assert_str_eq(events.system, "sched");
    ck_assert_str_eq(events.event, "sched_switch");
    ck_assert_ptr_eq(events.next, core::ptr::null_mut());
    }
    END_TEST
    START_TEST(test_opt_event_cb_multiple)
    {
    struct trace_events *events = core::ptr::null_mut();
    let mut opt: option = TEST_CALLBACK(&events, opt_event_cb);
    ck_assert_int_eq(opt_event_cb(&opt, "sched:sched_switch", 0), 0);
    ck_assert_int_eq(opt_event_cb(&opt, "sched:sched_wakeup", 0), 0);
    ck_assert_str_eq(events.system, "sched");
    ck_assert_str_eq(events.event, "sched_wakeup");
    ck_assert_str_eq(events.next.system, "sched");
    ck_assert_str_eq(events.next.event, "sched_switch");
    ck_assert_ptr_eq(events.next.next, core::ptr::null_mut());
    }
    END_TEST
    START_TEST(test_opt_housekeeping_cb)
    {
    let mut __params: common_params = {0};
    struct common_params *params = &__params;
    let mut opt: option = TEST_CALLBACK(params, opt_housekeeping_cb);
    nr_cpus = 4;
    ck_assert_int_eq(opt_housekeeping_cb(&opt, "0-3", 0), 0);
    ck_assert_int_eq(params.hk_cpus, 1);
    CLI_ASSERT_CPUSET(hk_cpu_set, 0, 1, 2, 3);
    }
    END_TEST
    START_TEST(test_opt_housekeeping_cb_invalid)
    {
    let mut params: common_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_housekeeping_cb);
    nr_cpus = 4;
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_housekeeping_cb(&opt, "0-3,5", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_housekeeping_cb_unset)
    {
    let mut params: common_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_housekeeping_cb);
    nr_cpus = 4;
    ck_assert_int_eq(opt_housekeeping_cb(&opt, "0-3", 0), 0);
    ck_assert_int_eq(opt_housekeeping_cb(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_int_eq(params.hk_cpus, 0);
    ck_assert_int_eq(CPU_COUNT(&params.hk_cpu_set), 0);
    }
    END_TEST
    START_TEST(test_opt_priority_cb)
    {
    let mut params: common_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_priority_cb);
    ck_assert_int_eq(opt_priority_cb(&opt, "f:95", 0), 0);
    ck_assert_int_eq(params.sched_param.sched_policy, SCHED_FIFO);
    ck_assert_int_eq(params.sched_param.sched_priority, 95);
    }
    END_TEST
    START_TEST(test_opt_priority_cb_invalid)
    {
    let mut params: common_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_priority_cb);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_priority_cb(&opt, "abc", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_priority_cb_unset)
    {
    let mut params: common_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_priority_cb);
    ck_assert_int_eq(opt_priority_cb(&opt, "f:95", 0), 0);
    ck_assert_int_eq(opt_priority_cb(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_int_eq(params.sched_param.sched_policy, 0);
    ck_assert_int_eq(params.sched_param.sched_priority, 0);
    }
    END_TEST
    START_TEST(test_opt_trigger_cb)
    {
    struct trace_events *events = trace_event_alloc("sched:sched_switch");
    let mut opt: option = TEST_CALLBACK(&events, opt_trigger_cb);
    ck_assert_int_eq(opt_trigger_cb(&opt, "stacktrace", 0), 0);
    ck_assert_str_eq(events.trigger, "stacktrace");
    }
    END_TEST
    START_TEST(test_opt_trigger_cb_no_event)
    {
    struct trace_events *events = core::ptr::null_mut();
    let mut opt: option = TEST_CALLBACK(&events, opt_trigger_cb);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_trigger_cb(&opt, "stacktrace", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_filter_cb)
    {
    struct trace_events *events = trace_event_alloc("sched:sched_switch");
    let mut opt: option = TEST_CALLBACK(&events, opt_filter_cb);
    ck_assert_int_eq(opt_filter_cb(&opt, "comm ~ \"rtla\"", 0), 0);
    ck_assert_str_eq(events.filter, "comm ~ \"rtla\"");
    }
    END_TEST
    START_TEST(test_opt_filter_cb_no_event)
    {
    struct trace_events *events = core::ptr::null_mut();
    let mut opt: option = TEST_CALLBACK(&events, opt_filter_cb);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_filter_cb(&opt, "comm ~ \"rtla\"", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_osnoise_auto_cb)
    {
    let mut params: osnoise_params = {0};
    let mut cb_data: osnoise_cb_data = {&params};
    let mut opt: option = TEST_CALLBACK(&cb_data, opt_osnoise_auto_cb);
    ck_assert_int_eq(opt_osnoise_auto_cb(&opt, "10", 0), 0);
    ck_assert_int_eq(params.common.stop_us, 10);
    ck_assert_int_eq(params.threshold, 1);
    ck_assert_str_eq(cb_data.trace_output, "osnoise_trace.txt");
    }
    END_TEST
    START_TEST(test_opt_osnoise_auto_cb_unset)
    {
    let mut params: osnoise_params = {0};
    let mut cb_data: osnoise_cb_data = {&params};
    let mut opt: option = TEST_CALLBACK(&cb_data, opt_osnoise_auto_cb);
    ck_assert_int_eq(opt_osnoise_auto_cb(&opt, "10", 0), 0);
    ck_assert_int_eq(opt_osnoise_auto_cb(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_int_eq(params.common.stop_us, 0);
    ck_assert_int_eq(params.threshold, 0);
    ck_assert_ptr_null(cb_data.trace_output);
    }
    END_TEST
    START_TEST(test_opt_osnoise_trace_output_cb)
    {
    const char *trace_output = core::ptr::null_mut();
    let mut opt: option = TEST_CALLBACK(&trace_output, opt_osnoise_trace_output_cb);
    ck_assert_int_eq(opt_osnoise_trace_output_cb(&opt, "trace.txt", 0), 0);
    ck_assert_str_eq(trace_output, "trace.txt");
    }
    END_TEST
    START_TEST(test_opt_osnoise_trace_output_cb_noarg)
    {
    const char *trace_output = core::ptr::null_mut();
    let mut opt: option = TEST_CALLBACK(&trace_output, opt_osnoise_trace_output_cb);
    ck_assert_int_eq(opt_osnoise_trace_output_cb(&opt, core::ptr::null_mut(), 0), 0);
    ck_assert_str_eq(trace_output, "osnoise_trace.txt");
    }
    END_TEST
    START_TEST(test_opt_osnoise_trace_output_cb_unset)
    {
    const char *trace_output = core::ptr::null_mut();
    let mut opt: option = TEST_CALLBACK(&trace_output, opt_osnoise_trace_output_cb);
    ck_assert_int_eq(opt_osnoise_trace_output_cb(&opt, "trace.txt", 0), 0);
    ck_assert_int_eq(opt_osnoise_trace_output_cb(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_ptr_null(trace_output);
    }
    END_TEST
    START_TEST(test_opt_osnoise_on_threshold_cb)
    {
    let mut actions: actions = {0};
    let mut opt: option = TEST_CALLBACK(&actions, opt_osnoise_on_threshold_cb);
    ck_assert_int_eq(opt_osnoise_on_threshold_cb(&opt, "trace", 0), 0);
    ck_assert_int_eq(actions.len, 1);
    ck_assert_int_eq(actions.list[0].type, ACTION_TRACE_OUTPUT);
    ck_assert_str_eq(actions.list[0].trace_output, "osnoise_trace.txt");
    }
    END_TEST
    START_TEST(test_opt_osnoise_on_threshold_cb_invalid)
    {
    let mut actions: actions = {0};
    let mut opt: option = TEST_CALLBACK(&actions, opt_osnoise_on_threshold_cb);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_osnoise_on_threshold_cb(&opt, "abc", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_osnoise_on_end_cb)
    {
    let mut actions: actions = {0};
    let mut opt: option = TEST_CALLBACK(&actions, opt_osnoise_on_end_cb);
    ck_assert_int_eq(opt_osnoise_on_end_cb(&opt, "trace", 0), 0);
    ck_assert_int_eq(actions.len, 1);
    ck_assert_int_eq(actions.list[0].type, ACTION_TRACE_OUTPUT);
    ck_assert_str_eq(actions.list[0].trace_output, "osnoise_trace.txt");
    }
    END_TEST
    START_TEST(test_opt_osnoise_on_end_cb_invalid)
    {
    let mut actions: actions = {0};
    let mut opt: option = TEST_CALLBACK(&actions, opt_osnoise_on_end_cb);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_osnoise_on_end_cb(&opt, "abc", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_timerlat_auto_cb)
    {
    let mut params: timerlat_params = {0};
    let mut cb_data: timerlat_cb_data = {&params};
    let mut opt: option = TEST_CALLBACK(&cb_data, opt_timerlat_auto_cb);
    ck_assert_int_eq(opt_timerlat_auto_cb(&opt, "10", 0), 0);
    ck_assert_int_eq(params.common.stop_us, 10);
    ck_assert_int_eq(params.common.stop_total_us, 10);
    ck_assert_int_eq(params.print_stack, 10);
    ck_assert_str_eq(cb_data.trace_output, "timerlat_trace.txt");
    }
    END_TEST
    START_TEST(test_opt_timerlat_auto_cb_unset)
    {
    let mut params: timerlat_params = {0};
    let mut cb_data: timerlat_cb_data = {&params};
    let mut opt: option = TEST_CALLBACK(&cb_data, opt_timerlat_auto_cb);
    ck_assert_int_eq(opt_timerlat_auto_cb(&opt, "10", 0), 0);
    ck_assert_int_eq(opt_timerlat_auto_cb(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_int_eq(params.common.stop_us, 0);
    ck_assert_int_eq(params.common.stop_total_us, 0);
    ck_assert_int_eq(params.print_stack, 0);
    ck_assert_ptr_null(cb_data.trace_output);
    }
    END_TEST
    START_TEST(test_opt_aa_only_cb)
    {
    let mut params: timerlat_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_aa_only_cb);
    ck_assert_int_eq(opt_aa_only_cb(&opt, "10", 0), 0);
    ck_assert_int_eq(params.common.stop_us, 10);
    ck_assert_int_eq(params.common.stop_total_us, 10);
    ck_assert_int_eq(params.print_stack, 10);
    ck_assert_int_eq(params.common.aa_only, 1);
    }
    END_TEST
    START_TEST(test_opt_aa_only_cb_unset)
    {
    let mut params: timerlat_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_aa_only_cb);
    ck_assert_int_eq(opt_aa_only_cb(&opt, "10", 0), 0);
    ck_assert_int_eq(opt_aa_only_cb(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_int_eq(params.common.stop_us, 0);
    ck_assert_int_eq(params.common.stop_total_us, 0);
    ck_assert_int_eq(params.print_stack, 0);
    ck_assert_int_eq(params.common.aa_only, 0);
    }
    END_TEST
    START_TEST(test_opt_timerlat_trace_output_cb)
    {
    const char *trace_output = core::ptr::null_mut();
    let mut opt: option = TEST_CALLBACK(&trace_output, opt_timerlat_trace_output_cb);
    ck_assert_int_eq(opt_timerlat_trace_output_cb(&opt, "trace.txt", 0), 0);
    ck_assert_str_eq(trace_output, "trace.txt");
    }
    END_TEST
    START_TEST(test_opt_timerlat_trace_output_cb_noarg)
    {
    const char *trace_output = core::ptr::null_mut();
    let mut opt: option = TEST_CALLBACK(&trace_output, opt_timerlat_trace_output_cb);
    ck_assert_int_eq(opt_timerlat_trace_output_cb(&opt, core::ptr::null_mut(), 0), 0);
    ck_assert_str_eq(trace_output, "timerlat_trace.txt");
    }
    END_TEST
    START_TEST(test_opt_timerlat_trace_output_cb_unset)
    {
    const char *trace_output = core::ptr::null_mut();
    let mut opt: option = TEST_CALLBACK(&trace_output, opt_timerlat_trace_output_cb);
    ck_assert_int_eq(opt_timerlat_trace_output_cb(&opt, "trace.txt", 0), 0);
    ck_assert_int_eq(opt_timerlat_trace_output_cb(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_ptr_null(trace_output);
    }
    END_TEST
    START_TEST(test_opt_timerlat_on_threshold_cb)
    {
    let mut actions: actions = {0};
    let mut opt: option = TEST_CALLBACK(&actions, opt_timerlat_on_threshold_cb);
    ck_assert_int_eq(opt_timerlat_on_threshold_cb(&opt, "trace", 0), 0);
    ck_assert_int_eq(actions.len, 1);
    ck_assert_int_eq(actions.list[0].type, ACTION_TRACE_OUTPUT);
    ck_assert_str_eq(actions.list[0].trace_output, "timerlat_trace.txt");
    }
    END_TEST
    START_TEST(test_opt_timerlat_on_threshold_cb_invalid)
    {
    let mut actions: actions = {0};
    let mut opt: option = TEST_CALLBACK(&actions, opt_timerlat_on_threshold_cb);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_timerlat_on_threshold_cb(&opt, "abc", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_timerlat_on_end_cb)
    {
    let mut actions: actions = {0};
    let mut opt: option = TEST_CALLBACK(&actions, opt_timerlat_on_end_cb);
    ck_assert_int_eq(opt_timerlat_on_end_cb(&opt, "trace", 0), 0);
    ck_assert_int_eq(actions.len, 1);
    ck_assert_int_eq(actions.list[0].type, ACTION_TRACE_OUTPUT);
    ck_assert_str_eq(actions.list[0].trace_output, "timerlat_trace.txt");
    }
    END_TEST
    START_TEST(test_opt_timerlat_on_end_cb_invalid)
    {
    let mut actions: actions = {0};
    let mut opt: option = TEST_CALLBACK(&actions, opt_timerlat_on_end_cb);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_timerlat_on_end_cb(&opt, "abc", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_user_threads_cb)
    {
    let mut params: timerlat_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_user_threads_cb);
    ck_assert_int_eq(opt_user_threads_cb(&opt, core::ptr::null_mut(), 0), 0);
    ck_assert_int_eq(params.common.user_workload, 1);
    ck_assert_int_eq(params.common.user_data, 1);
    }
    END_TEST
    START_TEST(test_opt_user_threads_cb_unset)
    {
    let mut params: timerlat_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_user_threads_cb);
    ck_assert_int_eq(opt_user_threads_cb(&opt, core::ptr::null_mut(), 0), 0);
    ck_assert_int_eq(opt_user_threads_cb(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_int_eq(params.common.user_workload, 0);
    ck_assert_int_eq(params.common.user_data, 0);
    }
    END_TEST
    START_TEST(test_opt_nano_cb)
    {
    let mut params: timerlat_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_nano_cb);
    ck_assert_int_eq(opt_nano_cb(&opt, core::ptr::null_mut(), 0), 0);
    ck_assert_int_eq(params.common.output_divisor, 1);
    }
    END_TEST
    START_TEST(test_opt_nano_cb_unset)
    {
    let mut params: timerlat_params = {0};
    let mut opt: option = TEST_CALLBACK(&params, opt_nano_cb);
    ck_assert_int_eq(opt_nano_cb(&opt, core::ptr::null_mut(), 0), 0);
    ck_assert_int_eq(opt_nano_cb(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_int_eq(params.common.output_divisor, default_output_divisor);
    }
    END_TEST
    START_TEST(test_opt_timerlat_align_cb)
    {
    let mut params: timerlat_params = {0};
    const struct option opt = RTLA_OPT_CALLBACK_DATA('A', "aligned", &params, "us",
    "test", opt_timerlat_align_cb, LLONG_RANGE(0, LLONG_MAX));
    ck_assert_int_eq(opt_timerlat_align_cb(&opt, "500", 0), 0);
    ck_assert(params.timerlat_align);
    ck_assert_int_eq(params.timerlat_align_us, 500);
    }
    END_TEST
    START_TEST(test_opt_timerlat_align_cb_invalid)
    {
    let mut params: timerlat_params = {0};
    const struct option opt = RTLA_OPT_CALLBACK_DATA('A', "aligned", &params, "us",
    "test", opt_timerlat_align_cb, LLONG_RANGE(0, LLONG_MAX));
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_timerlat_align_cb(&opt, "-1", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_timerlat_align_cb_unset)
    {
    let mut params: timerlat_params = {0};
    const struct option opt = RTLA_OPT_CALLBACK_DATA('A', "aligned", &params, "us",
    "test", opt_timerlat_align_cb, LLONG_RANGE(0, LLONG_MAX));
    ck_assert_int_eq(opt_timerlat_align_cb(&opt, "500", 0), 0);
    ck_assert_int_eq(opt_timerlat_align_cb(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_int_eq(params.timerlat_align, 0);
    ck_assert_int_eq(params.timerlat_align_us, 0);
    }
    END_TEST
    START_TEST(test_opt_stack_format_cb)
    {
    let mut stack_format: c_int = 0;
    let mut opt: option = TEST_CALLBACK(&stack_format, opt_stack_format_cb);
    ck_assert_int_eq(opt_stack_format_cb(&opt, "full", 0), 0);
    ck_assert_int_eq(stack_format, STACK_FORMAT_FULL);
    }
    END_TEST
    START_TEST(test_opt_stack_format_cb_invalid)
    {
    let mut stack_format: c_int = 0;
    let mut opt: option = TEST_CALLBACK(&stack_format, opt_stack_format_cb);
    assert(freopen("/dev/null", "w", stderr));
    ck_assert_int_eq(opt_stack_format_cb(&opt, "abc", 0), -1);
    }
    END_TEST
    START_TEST(test_opt_stack_format_cb_unset)
    {
    let mut stack_format: c_int = 0;
    let mut opt: option = TEST_CALLBACK(&stack_format, opt_stack_format_cb);
    ck_assert_int_eq(opt_stack_format_cb(&opt, "full", 0), 0);
    ck_assert_int_eq(opt_stack_format_cb(&opt, core::ptr::null_mut(), 1), 0);
    ck_assert_int_eq(stack_format, default_stack_format);
    }
    END_TEST
    Suite *cli_opt_callback_suite(void)
    {
    Suite *s = suite_create("cli_opt_callback");
    TCase *tc;
    tc = tcase_create("common");
    tcase_add_test(tc, test_opt_llong_callback_simple);
    tcase_add_test(tc, test_opt_llong_callback_max);
    tcase_add_test(tc, test_opt_llong_callback_min);
    tcase_add_test(tc, test_opt_llong_callback_non_numeric);
    tcase_add_test(tc, test_opt_llong_callback_non_numeric_suffix);
    tcase_add_test(tc, test_opt_llong_callback_unset);
    tcase_add_test(tc, test_opt_llong_callback_unset_defval);
    tcase_add_test(tc, test_opt_llong_callback_range_in);
    tcase_add_test(tc, test_opt_llong_callback_range_below);
    tcase_add_test(tc, test_opt_llong_callback_range_above);
    tcase_add_test(tc, test_opt_llong_callback_range_boundary);
    tcase_add_test(tc, test_opt_int_callback_simple);
    tcase_add_test(tc, test_opt_int_callback_max);
    tcase_add_test(tc, test_opt_int_callback_min);
    tcase_add_test(tc, test_opt_int_callback_non_numeric);
    tcase_add_test(tc, test_opt_int_callback_non_numeric_suffix);
    tcase_add_test(tc, test_opt_int_callback_unset);
    tcase_add_test(tc, test_opt_int_callback_unset_defval);
    tcase_add_test(tc, test_opt_int_callback_range_in);
    tcase_add_test(tc, test_opt_int_callback_range_below);
    tcase_add_test(tc, test_opt_int_callback_range_above);
    tcase_add_test(tc, test_opt_int_callback_range_boundary);
    tcase_add_test(tc, test_opt_cpus_cb);
    tcase_add_test(tc, test_opt_cpus_cb_invalid);
    tcase_add_test(tc, test_opt_cgroup_cb);
    tcase_add_test(tc, test_opt_cgroup_cb_equals);
    tcase_add_test(tc, test_opt_cgroup_cb_unset);
    tcase_add_test(tc, test_opt_duration_cb);
    tcase_add_test(tc, test_opt_duration_cb_unset);
    tcase_add_test(tc, test_opt_duration_cb_invalid);
    tcase_add_test(tc, test_opt_event_cb);
    tcase_add_test(tc, test_opt_event_cb_multiple);
    tcase_add_test(tc, test_opt_housekeeping_cb);
    tcase_add_test(tc, test_opt_housekeeping_cb_invalid);
    tcase_add_test(tc, test_opt_housekeeping_cb_unset);
    tcase_add_test(tc, test_opt_priority_cb);
    tcase_add_test(tc, test_opt_priority_cb_invalid);
    tcase_add_test(tc, test_opt_priority_cb_unset);
    tcase_add_test(tc, test_opt_trigger_cb);
    tcase_add_test(tc, test_opt_trigger_cb_no_event);
    tcase_add_test(tc, test_opt_filter_cb);
    tcase_add_test(tc, test_opt_filter_cb_no_event);
    suite_add_tcase(s, tc);
    tc = tcase_create("osnoise");
    tcase_add_test(tc, test_opt_osnoise_auto_cb);
    tcase_add_test(tc, test_opt_osnoise_auto_cb_unset);
    tcase_add_test(tc, test_opt_osnoise_trace_output_cb);
    tcase_add_test(tc, test_opt_osnoise_trace_output_cb_noarg);
    tcase_add_test(tc, test_opt_osnoise_trace_output_cb_unset);
    tcase_add_test(tc, test_opt_osnoise_on_threshold_cb);
    tcase_add_test(tc, test_opt_osnoise_on_threshold_cb_invalid);
    tcase_add_test(tc, test_opt_osnoise_on_end_cb);
    tcase_add_test(tc, test_opt_osnoise_on_end_cb_invalid);
    suite_add_tcase(s, tc);
    tc = tcase_create("timerlat");
    tcase_add_test(tc, test_opt_timerlat_auto_cb);
    tcase_add_test(tc, test_opt_timerlat_auto_cb_unset);
    tcase_add_test(tc, test_opt_aa_only_cb);
    tcase_add_test(tc, test_opt_aa_only_cb_unset);
    tcase_add_test(tc, test_opt_timerlat_trace_output_cb);
    tcase_add_test(tc, test_opt_timerlat_trace_output_cb_noarg);
    tcase_add_test(tc, test_opt_timerlat_trace_output_cb_unset);
    tcase_add_test(tc, test_opt_timerlat_on_threshold_cb);
    tcase_add_test(tc, test_opt_timerlat_on_threshold_cb_invalid);
    tcase_add_test(tc, test_opt_timerlat_on_end_cb);
    tcase_add_test(tc, test_opt_timerlat_on_end_cb_invalid);
    tcase_add_test(tc, test_opt_user_threads_cb);
    tcase_add_test(tc, test_opt_user_threads_cb_unset);
    tcase_add_test(tc, test_opt_nano_cb);
    tcase_add_test(tc, test_opt_nano_cb_unset);
    tcase_add_test(tc, test_opt_stack_format_cb);
    tcase_add_test(tc, test_opt_stack_format_cb_invalid);
    tcase_add_test(tc, test_opt_stack_format_cb_unset);
    tcase_add_test(tc, test_opt_timerlat_align_cb);
    tcase_add_test(tc, test_opt_timerlat_align_cb_invalid);
    tcase_add_test(tc, test_opt_timerlat_align_cb_unset);
    suite_add_tcase(s, tc);
    return s;
    }
