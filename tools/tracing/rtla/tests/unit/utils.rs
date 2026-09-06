//! Automatically rewritten from C to Rust
//! Source: tools/tracing/rtla/tests/unit/utils.c
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

    extern int nr_cpus;
    START_TEST(test_strtoi)
    {
    int result;
    char buf[64];
    ck_assert_int_eq(strtoi("123", &result), 0);
    ck_assert_int_eq(result, 123);
    ck_assert_int_eq(strtoi(" -456", &result), 0);
    ck_assert_int_eq(result, -456);
    snprintf(buf, sizeof(buf), "%d", INT_MAX);
    ck_assert_int_eq(strtoi(buf, &result), 0);
    snprintf(buf, sizeof(buf), "%ld", (long)INT_MAX + 1);
    ck_assert_int_eq(strtoi(buf, &result), -1);
    ck_assert_int_eq(strtoi("", &result), -1);
    ck_assert_int_eq(strtoi("123abc", &result), -1);
    ck_assert_int_eq(strtoi("123 ", &result), -1);
    }
    END_TEST
    START_TEST(test_parse_cpu_set)
    {
    cpu_set_t set;
    nr_cpus = 8;
    ck_assert_int_eq(parse_cpu_set("0", &set), 0);
    ck_assert(CPU_ISSET(0, &set));
    ck_assert(!CPU_ISSET(1, &set));
    ck_assert_int_eq(parse_cpu_set("0,2", &set), 0);
    ck_assert(CPU_ISSET(0, &set));
    ck_assert(CPU_ISSET(2, &set));
    ck_assert_int_eq(parse_cpu_set("0-3", &set), 0);
    ck_assert(CPU_ISSET(0, &set));
    ck_assert(CPU_ISSET(1, &set));
    ck_assert(CPU_ISSET(2, &set));
    ck_assert(CPU_ISSET(3, &set));
    ck_assert_int_eq(parse_cpu_set("1-3,5", &set), 0);
    ck_assert(!CPU_ISSET(0, &set));
    ck_assert(CPU_ISSET(1, &set));
    ck_assert(CPU_ISSET(2, &set));
    ck_assert(CPU_ISSET(3, &set));
    ck_assert(!CPU_ISSET(4, &set));
    ck_assert(CPU_ISSET(5, &set));
    ck_assert_int_eq(parse_cpu_set("-1", &set), 1);
    ck_assert_int_eq(parse_cpu_set("abc", &set), 1);
    ck_assert_int_eq(parse_cpu_set("9999", &set), 1);
    }
    END_TEST
    START_TEST(test_parse_prio)
    {
    struct sched_attr attr;
    ck_assert_int_eq(parse_prio("f:50", &attr), 0);
    ck_assert_uint_eq(attr.sched_policy, SCHED_FIFO);
    ck_assert_uint_eq(attr.sched_priority, 50U);
    ck_assert_int_eq(parse_prio("r:30", &attr), 0);
    ck_assert_uint_eq(attr.sched_policy, SCHED_RR);
    ck_assert_int_eq(parse_prio("o:0", &attr), 0);
    ck_assert_uint_eq(attr.sched_policy, SCHED_OTHER);
    ck_assert_int_eq(attr.sched_nice, 0);
    ck_assert_int_eq(parse_prio("d:10ms:100ms", &attr), 0);
    ck_assert_uint_eq(attr.sched_policy, 6U);
    ck_assert_int_eq(parse_prio("f:999", &attr), -1);
    ck_assert_int_eq(parse_prio("o:-20", &attr), -1);
    ck_assert_int_eq(parse_prio("d:100ms:10ms", &attr), -1);
    ck_assert_int_eq(parse_prio("x:50", &attr), -1);
    }
    END_TEST
    Suite *utils_suite(void)
    {
    Suite *s = suite_create("utils");
    TCase *tc = tcase_create("core");
    tcase_add_test(tc, test_strtoi);
    tcase_add_test(tc, test_parse_cpu_set);
    tcase_add_test(tc, test_parse_prio);
    suite_add_tcase(s, tc);
    return s;
    }
