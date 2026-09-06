//! Automatically rewritten from C to Rust
//! Source: fs/ext4/inode-test.c
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
// KUnit test of ext4 inode that verify the seconds part of [a/c/m]
// timestamps in ext4 inode structs are decoded correctly.
//

//
// For constructing the nonnegative timestamp lower bound value.
// binary: 00000000 00000000 00000000 00000000
//

//
// For constructing the nonnegative timestamp upper bound value.
// binary: 01111111 11111111 11111111 11111111
//
pub const UPPER_MSB_0: c_uint = 0x7fffffffL;
//
// For constructing the negative timestamp lower bound value.
// binary: 10000000 00000000 00000000 00000000
//

//
// For constructing the negative timestamp upper bound value.
// binary: 11111111 11111111 11111111 11111111
//

//
// Upper bound for nanoseconds value supported by the encoding.
// binary: 00111111 11111111 11111111 11111111
//

// Macro flag: #define LOWER_BOUND_NEG_NO_EXTRA_BITS_CASE\
    "1901-12-13 Lower bound of 32bit < 0 timestamp, no extra bits"
// Macro flag: #define UPPER_BOUND_NEG_NO_EXTRA_BITS_CASE\
    "1969-12-31 Upper bound of 32bit < 0 timestamp, no extra bits"
// Macro flag: #define LOWER_BOUND_NONNEG_NO_EXTRA_BITS_CASE\
    "1970-01-01 Lower bound of 32bit >=0 timestamp, no extra bits"
// Macro flag: #define UPPER_BOUND_NONNEG_NO_EXTRA_BITS_CASE\
    "2038-01-19 Upper bound of 32bit >=0 timestamp, no extra bits"
// Macro flag: #define LOWER_BOUND_NEG_LO_1_CASE\
    "2038-01-19 Lower bound of 32bit <0 timestamp, lo extra sec bit on"
// Macro flag: #define UPPER_BOUND_NEG_LO_1_CASE\
    "2106-02-07 Upper bound of 32bit <0 timestamp, lo extra sec bit on"
// Macro flag: #define LOWER_BOUND_NONNEG_LO_1_CASE\
    "2106-02-07 Lower bound of 32bit >=0 timestamp, lo extra sec bit on"
// Macro flag: #define UPPER_BOUND_NONNEG_LO_1_CASE\
    "2174-02-25 Upper bound of 32bit >=0 timestamp, lo extra sec bit on"
// Macro flag: #define LOWER_BOUND_NEG_HI_1_CASE\
    "2174-02-25 Lower bound of 32bit <0 timestamp, hi extra sec bit on"
// Macro flag: #define UPPER_BOUND_NEG_HI_1_CASE\
    "2242-03-16 Upper bound of 32bit <0 timestamp, hi extra sec bit on"
// Macro flag: #define LOWER_BOUND_NONNEG_HI_1_CASE\
    "2242-03-16 Lower bound of 32bit >=0 timestamp, hi extra sec bit on"
// Macro flag: #define UPPER_BOUND_NONNEG_HI_1_CASE\
    "2310-04-04 Upper bound of 32bit >=0 timestamp, hi extra sec bit on"
// Macro flag: #define UPPER_BOUND_NONNEG_HI_1_NS_1_CASE\
    "2310-04-04 Upper bound of 32bit>=0 timestamp, hi extra sec bit 1. 1 ns"
// Macro flag: #define LOWER_BOUND_NONNEG_HI_1_NS_MAX_CASE\
    "2378-04-22 Lower bound of 32bit>= timestamp. Extra sec bits 1. Max ns"
// Macro flag: #define LOWER_BOUND_NONNEG_EXTRA_BITS_1_CASE\
    "2378-04-22 Lower bound of 32bit >=0 timestamp. All extra sec bits on"
// Macro flag: #define UPPER_BOUND_NONNEG_EXTRA_BITS_1_CASE\
    "2446-05-10 Upper bound of 32bit >=0 timestamp. All extra sec bits on"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timestamp_expectation {
    pub test_case_name: *const c_char,
    pub expected: timespec64,
    pub extra_bits: u32,
    pub msb_set: bool,
    pub lower_bound: bool,
}

    static const struct timestamp_expectation test_data[] = {
    {
    .test_case_name = LOWER_BOUND_NEG_NO_EXTRA_BITS_CASE,
    .msb_set = true,
    .lower_bound = true,
    .extra_bits = 0,
    .expected = {.tv_sec = -0x80000000LL, .tv_nsec = 0L},
    },
    {
    .test_case_name = UPPER_BOUND_NEG_NO_EXTRA_BITS_CASE,
    .msb_set = true,
    .lower_bound = false,
    .extra_bits = 0,
    .expected = {.tv_sec = -1LL, .tv_nsec = 0L},
    },
    {
    .test_case_name = LOWER_BOUND_NONNEG_NO_EXTRA_BITS_CASE,
    .msb_set = false,
    .lower_bound = true,
    .extra_bits = 0,
    .expected = {0LL, 0L},
    },
    {
    .test_case_name = UPPER_BOUND_NONNEG_NO_EXTRA_BITS_CASE,
    .msb_set = false,
    .lower_bound = false,
    .extra_bits = 0,
    .expected = {.tv_sec = 0x7fffffffLL, .tv_nsec = 0L},
    },
    {
    .test_case_name = LOWER_BOUND_NEG_LO_1_CASE,
    .msb_set = true,
    .lower_bound = true,
    .extra_bits = 1,
    .expected = {.tv_sec = 0x80000000LL, .tv_nsec = 0L},
    },
    {
    .test_case_name = UPPER_BOUND_NEG_LO_1_CASE,
    .msb_set = true,
    .lower_bound = false,
    .extra_bits = 1,
    .expected = {.tv_sec = 0xffffffffLL, .tv_nsec = 0L},
    },
    {
    .test_case_name = LOWER_BOUND_NONNEG_LO_1_CASE,
    .msb_set = false,
    .lower_bound = true,
    .extra_bits = 1,
    .expected = {.tv_sec = 0x100000000LL, .tv_nsec = 0L},
    },
    {
    .test_case_name = UPPER_BOUND_NONNEG_LO_1_CASE,
    .msb_set = false,
    .lower_bound = false,
    .extra_bits = 1,
    .expected = {.tv_sec = 0x17fffffffLL, .tv_nsec = 0L},
    },
    {
    .test_case_name = LOWER_BOUND_NEG_HI_1_CASE,
    .msb_set = true,
    .lower_bound = true,
    .extra_bits =  2,
    .expected = {.tv_sec = 0x180000000LL, .tv_nsec = 0L},
    },
    {
    .test_case_name = UPPER_BOUND_NEG_HI_1_CASE,
    .msb_set = true,
    .lower_bound = false,
    .extra_bits = 2,
    .expected = {.tv_sec = 0x1ffffffffLL, .tv_nsec = 0L},
    },
    {
    .test_case_name = LOWER_BOUND_NONNEG_HI_1_CASE,
    .msb_set = false,
    .lower_bound = true,
    .extra_bits = 2,
    .expected = {.tv_sec = 0x200000000LL, .tv_nsec = 0L},
    },
    {
    .test_case_name = UPPER_BOUND_NONNEG_HI_1_CASE,
    .msb_set = false,
    .lower_bound = false,
    .extra_bits = 2,
    .expected = {.tv_sec = 0x27fffffffLL, .tv_nsec = 0L},
    },
    {
    .test_case_name = UPPER_BOUND_NONNEG_HI_1_NS_1_CASE,
    .msb_set = false,
    .lower_bound = false,
    .extra_bits = 6,
    .expected = {.tv_sec = 0x27fffffffLL, .tv_nsec = 1L},
    },
    {
    .test_case_name = LOWER_BOUND_NONNEG_HI_1_NS_MAX_CASE,
    .msb_set = false,
    .lower_bound = true,
    .extra_bits = 0xFFFFFFFF,
    .expected = {.tv_sec = 0x300000000LL,
    .tv_nsec = MAX_NANOSECONDS},
    },
    {
    .test_case_name = LOWER_BOUND_NONNEG_EXTRA_BITS_1_CASE,
    .msb_set = false,
    .lower_bound = true,
    .extra_bits = 3,
    .expected = {.tv_sec = 0x300000000LL, .tv_nsec = 0L},
    },
    {
    .test_case_name = UPPER_BOUND_NONNEG_EXTRA_BITS_1_CASE,
    .msb_set = false,
    .lower_bound = false,
    .extra_bits = 3,
    .expected = {.tv_sec = 0x37fffffffLL, .tv_nsec = 0L},
    }
    };
    static void timestamp_expectation_to_desc(const struct timestamp_expectation *t,
    char *desc)
    {
    strscpy(desc, t.test_case_name, KUNIT_PARAM_DESC_SIZE);
    }
    KUNIT_ARRAY_PARAM(ext4_inode, test_data, timestamp_expectation_to_desc);
#[no_mangle]
unsafe extern "C" fn get_32bit_time(test: *const *const timestamp_expectation) -> time64_t {
    static time64_t get_32bit_time(const struct timestamp_expectation * const test)
    {
    if (test.msb_set) {
    if (test.lower_bound)
    return LOWER_MSB_1;
    return UPPER_MSB_1;
    }
    if (test.lower_bound)
    return LOWER_MSB_0;
    return UPPER_MSB_0;
    }
//
// Test data is derived from the table in the Inode Timestamps section of
// Documentation/filesystems/ext4/inodes.rst.
//
#[no_mangle]
unsafe extern "C" fn inode_test_xtimestamp_decoding(test: *mut kunit) {
    static void inode_test_xtimestamp_decoding(struct kunit *test)
    {
    struct timespec64 timestamp;
    struct timestamp_expectation *test_param =
    (struct timestamp_expectation *)(test.param_value);
    timestamp = ext4_decode_extra_time(
    cpu_to_le32(get_32bit_time(test_param)),
    cpu_to_le32(test_param.extra_bits));
    KUNIT_EXPECT_EQ_MSG(test,
    test_param.expected.tv_sec,
    timestamp.tv_sec,
    CASE_NAME_FORMAT,
    test_param.test_case_name,
    test_param.msb_set,
    test_param.lower_bound,
    test_param.extra_bits);
    KUNIT_EXPECT_EQ_MSG(test,
    test_param.expected.tv_nsec,
    timestamp.tv_nsec,
    CASE_NAME_FORMAT,
    test_param.test_case_name,
    test_param.msb_set,
    test_param.lower_bound,
    test_param.extra_bits);
    }
    static struct kunit_case ext4_inode_test_cases[] = {
    KUNIT_CASE_PARAM(inode_test_xtimestamp_decoding, ext4_inode_gen_params),
    {}
    };
    static struct kunit_suite ext4_inode_test_suite = {
    .name = "ext4_inode_test",
    .test_cases = ext4_inode_test_cases,
    };
    kunit_test_suites(&ext4_inode_test_suite);
    MODULE_DESCRIPTION("KUnit test of ext4 inode timestamp decoding");
    MODULE_LICENSE("GPL v2");
