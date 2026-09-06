//! Automatically rewritten from C to Rust
//! Source: drivers/iio/test/iio-test-gts.c
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
// Unit tests for IIO light sensor gain-time-scale helpers
//
// Copyright (c) 2023 Matti Vaittinen <mazziesaccount@gmail.com>
//

//
// Please, read the "rant" from the top of the lib/test_linear_ranges.c if
// you see a line of helper code which is not being tested.
//
// Then, please look at the line which is not being tested. Is this line
// somehow unusually complex? If answer is "no", then chances are that the
// "development inertia" caused by adding a test exceeds the benefits.
//
// If yes, then adding a test is probably a good idea but please stop for a
// moment and consider the effort of changing all the tests when code gets
// refactored. Eventually it needs to be.
//
pub const TEST_TSEL_50: c_int = 1;

pub const TEST_TSEL_100: c_int = 0;
pub const TEST_TSEL_200: c_int = 2;
pub const TEST_TSEL_400: c_int = 4;

pub const TEST_GSEL_1: c_uint = 0x00;

pub const TEST_GSEL_4: c_uint = 0x08;
pub const TEST_GSEL_16: c_uint = 0x0a;
pub const TEST_GSEL_32: c_uint = 0x0b;
pub const TEST_GSEL_64: c_uint = 0x0c;
pub const TEST_GSEL_256: c_uint = 0x18;
pub const TEST_GSEL_512: c_uint = 0x19;
pub const TEST_GSEL_1024: c_uint = 0x1a;
pub const TEST_GSEL_2048: c_uint = 0x1b;
pub const TEST_GSEL_4096: c_uint = 0x1c;

pub const TEST_SCALE_1X: c_int = 64;

pub const TEST_SCALE_2X: c_int = 32;
pub const TEST_SCALE_4X: c_int = 16;
pub const TEST_SCALE_8X: c_int = 8;
pub const TEST_SCALE_16X: c_int = 4;
pub const TEST_SCALE_32X: c_int = 2;
pub const TEST_SCALE_64X: c_int = 1;
pub const TEST_SCALE_NANO_128X: c_int = 500000000;
pub const TEST_SCALE_NANO_256X: c_int = 250000000;
pub const TEST_SCALE_NANO_512X: c_int = 125000000;
pub const TEST_SCALE_NANO_1024X: c_int = 62500000;
pub const TEST_SCALE_NANO_2048X: c_int = 31250000;
pub const TEST_SCALE_NANO_4096X: c_int = 15625000;
pub const TEST_SCALE_NANO_4096X2: c_int = 7812500;
pub const TEST_SCALE_NANO_4096X4: c_int = 3906250;
pub const TEST_SCALE_NANO_4096X8: c_int = 1953125;

//
// Can't have this allocated from stack because the kunit clean-up will
// happen only after the test function has already gone
//
    static struct iio_gts gts;
// Keep the gain and time tables unsorted to test the sorting
    static const struct iio_gain_sel_pair gts_test_gains[] = {
    GAIN_SCALE_GAIN(1, TEST_GSEL_1),
    GAIN_SCALE_GAIN(4, TEST_GSEL_4),
    GAIN_SCALE_GAIN(16, TEST_GSEL_16),
    GAIN_SCALE_GAIN(32, TEST_GSEL_32),
    GAIN_SCALE_GAIN(64, TEST_GSEL_64),
    GAIN_SCALE_GAIN(256, TEST_GSEL_256),
    GAIN_SCALE_GAIN(512, TEST_GSEL_512),
    GAIN_SCALE_GAIN(1024, TEST_GSEL_1024),
    GAIN_SCALE_GAIN(4096, TEST_GSEL_4096),
    GAIN_SCALE_GAIN(2048, TEST_GSEL_2048),
pub const HWGAIN_MAX: c_int = 4096;
    };
    static const struct iio_itime_sel_mul gts_test_itimes[] = {
    GAIN_SCALE_ITIME_US(100 * 1000, TEST_TSEL_100, 2),
    GAIN_SCALE_ITIME_US(400 * 1000, TEST_TSEL_400, 8),
    GAIN_SCALE_ITIME_US(400 * 1000, TEST_TSEL_400, 8),
    GAIN_SCALE_ITIME_US(50 * 1000, TEST_TSEL_50, 1),
    GAIN_SCALE_ITIME_US(200 * 1000, TEST_TSEL_200, 4),
pub const TIMEGAIN_MAX: c_int = 8;
    };

    static struct device *__test_init_iio_gain_scale(struct kunit *test,
    struct iio_gts *gts, const struct iio_gain_sel_pair *g_table,
    int num_g, const struct iio_itime_sel_mul *i_table, int num_i)
    {
    struct device *dev;
    int ret;
    dev = kunit_device_register(test, IIO_GTS_TEST_DEV);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, dev);
    if (IS_ERR_OR_NULL(dev))
    return core::ptr::null_mut();
    ret = devm_iio_init_iio_gts(dev, TEST_SCALE_1X, 0, g_table, num_g,
    i_table, num_i, gts);
    KUNIT_EXPECT_EQ(test, 0, ret);
    if (ret)
    return core::ptr::null_mut();
    return dev;
    }

    __test_init_iio_gain_scale(test, gts, gts_test_gains, \
    ARRAY_SIZE(gts_test_gains), gts_test_itimes, \
    ARRAY_SIZE(gts_test_itimes))
#[no_mangle]
unsafe extern "C" fn test_init_iio_gts_invalid(test: *mut kunit) {
    static void test_init_iio_gts_invalid(struct kunit *test)
    {
    struct device *dev;
    int ret;
    const struct iio_itime_sel_mul itimes_neg[] = {
    GAIN_SCALE_ITIME_US(-10, TEST_TSEL_400, 8),
    GAIN_SCALE_ITIME_US(200 * 1000, TEST_TSEL_200, 4),
    };
    const struct iio_gain_sel_pair gains_neg[] = {
    GAIN_SCALE_GAIN(1, TEST_GSEL_1),
    GAIN_SCALE_GAIN(2, TEST_GSEL_4),
    GAIN_SCALE_GAIN(-2, TEST_GSEL_16),
    };
// 55555 * 38656 = 2147534080 => overflows 32bit int
    const struct iio_itime_sel_mul itimes_overflow[] = {
    GAIN_SCALE_ITIME_US(400 * 1000, TEST_TSEL_400, 55555),
    GAIN_SCALE_ITIME_US(200 * 1000, TEST_TSEL_200, 4),
    };
    const struct iio_gain_sel_pair gains_overflow[] = {
    GAIN_SCALE_GAIN(1, TEST_GSEL_1),
    GAIN_SCALE_GAIN(2, TEST_GSEL_4),
    GAIN_SCALE_GAIN(38656, TEST_GSEL_16),
    };
    dev = kunit_device_register(test, IIO_GTS_TEST_DEV);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, dev);
    if (!dev)
    return;
// Ok gains, negative time
    ret = devm_iio_init_iio_gts(dev, TEST_SCALE_1X, 0, gts_test_gains,
    ARRAY_SIZE(gts_test_gains), itimes_neg,
    ARRAY_SIZE(itimes_neg), &gts);
    KUNIT_EXPECT_EQ(test, -EINVAL, ret);
// Ok times, negative gain
    ret = devm_iio_init_iio_gts(dev, TEST_SCALE_1X, 0, gains_neg,
    ARRAY_SIZE(gains_neg), gts_test_itimes,
    ARRAY_SIZE(gts_test_itimes), &gts);
    KUNIT_EXPECT_EQ(test, -EINVAL, ret);
// gain * time overflow int
    ret = devm_iio_init_iio_gts(dev, TEST_SCALE_1X, 0, gains_overflow,
    ARRAY_SIZE(gains_overflow), itimes_overflow,
    ARRAY_SIZE(itimes_overflow), &gts);
    KUNIT_EXPECT_EQ(test, -EOVERFLOW, ret);
    }
#[no_mangle]
unsafe extern "C" fn test_iio_gts_find_gain_for_scale_using_time(test: *mut kunit) {
    static void test_iio_gts_find_gain_for_scale_using_time(struct kunit *test)
    {
    struct device *dev;
    int ret, gain_sel;
    dev = test_init_iio_gain_scale(test, &gts);
    if (!dev)
    return;
    ret = iio_gts_find_gain_sel_for_scale_using_time(&gts, TEST_TSEL_100,
    TEST_SCALE_8X, 0, &gain_sel);
//
// Meas time 100 => gain by time 2x
// TEST_SCALE_8X matches total gain 8x
// => required HWGAIN 4x
//
    KUNIT_EXPECT_EQ(test, 0, ret);
    KUNIT_EXPECT_EQ(test, TEST_GSEL_4, gain_sel);
    ret = iio_gts_find_gain_sel_for_scale_using_time(&gts, TEST_TSEL_200, 0,
    TEST_SCALE_NANO_256X, &gain_sel);
//
// Meas time 200 => gain by time 4x
// TEST_SCALE_256X matches total gain 256x
// => required HWGAIN 256/4 => 64x
//
    KUNIT_EXPECT_EQ(test, 0, ret);
    KUNIT_EXPECT_EQ(test, TEST_GSEL_64, gain_sel);
// Min time, Min gain
    ret = iio_gts_find_gain_sel_for_scale_using_time(&gts, TEST_TSEL_X_MIN,
    TEST_SCALE_MIN_X, 0, &gain_sel);
    KUNIT_EXPECT_EQ(test, 0, ret);
    KUNIT_EXPECT_EQ(test, TEST_GSEL_1, gain_sel);
// Max time, Max gain
    ret = iio_gts_find_gain_sel_for_scale_using_time(&gts, TEST_TSEL_X_MAX,
    0, TEST_SCALE_NANO_MAX_X, &gain_sel);
    KUNIT_EXPECT_EQ(test, 0, ret);
    KUNIT_EXPECT_EQ(test, TEST_GSEL_4096, gain_sel);
    ret = iio_gts_find_gain_sel_for_scale_using_time(&gts, TEST_TSEL_100, 0,
    TEST_SCALE_NANO_256X, &gain_sel);
//
// Meas time 100 => gain by time 2x
// TEST_SCALE_256X matches total gain 256x
// => required HWGAIN 256/2 => 128x (not in gain-table - unsupported)
//
    KUNIT_EXPECT_NE(test, 0, ret);
    ret = iio_gts_find_gain_sel_for_scale_using_time(&gts, TEST_TSEL_200, 0,
    TEST_SCALE_NANO_MAX_X, &gain_sel);
// We can't reach the max gain with integration time smaller than MAX
    KUNIT_EXPECT_NE(test, 0, ret);
    ret = iio_gts_find_gain_sel_for_scale_using_time(&gts, TEST_TSEL_50, 0,
    TEST_SCALE_NANO_MAX_X, &gain_sel);
// We can't reach the max gain with integration time smaller than MAX
    KUNIT_EXPECT_NE(test, 0, ret);
    }
#[no_mangle]
unsafe extern "C" fn test_iio_gts_find_new_gain_sel_by_old_gain_time(test: *mut kunit) {
    static void test_iio_gts_find_new_gain_sel_by_old_gain_time(struct kunit *test)
    {
    struct device *dev;
    int ret, old_gain, new_gain, old_time_sel, new_time_sel;
    dev = test_init_iio_gain_scale(test, &gts);
    if (!dev)
    return;
    old_gain = 32;
    old_time_sel = TEST_TSEL_200;
    new_time_sel = TEST_TSEL_400;
    ret = iio_gts_find_new_gain_sel_by_old_gain_time(&gts, old_gain,
    old_time_sel, new_time_sel, &new_gain);
    KUNIT_EXPECT_EQ(test, 0, ret);
//
// Doubling the integration time doubles the total gain - so old
// (hw)gain must be divided by two to compensate. => 32 / 2 => 16
//
    KUNIT_EXPECT_EQ(test, 16, new_gain);
    old_gain = 4;
    old_time_sel = TEST_TSEL_50;
    new_time_sel = TEST_TSEL_200;
    ret = iio_gts_find_new_gain_sel_by_old_gain_time(&gts, old_gain,
    old_time_sel, new_time_sel, &new_gain);
    KUNIT_EXPECT_EQ(test, 0, ret);
//
// gain by time 1x => 4x - (hw)gain 4x => 1x
//
    KUNIT_EXPECT_EQ(test, 1, new_gain);
    old_gain = 512;
    old_time_sel = TEST_TSEL_400;
    new_time_sel = TEST_TSEL_50;
    ret = iio_gts_find_new_gain_sel_by_old_gain_time(&gts, old_gain,
    old_time_sel, new_time_sel, &new_gain);
    KUNIT_EXPECT_EQ(test, 0, ret);
//
// gain by time 8x => 1x - (hw)gain 512x => 4096x)
//
    KUNIT_EXPECT_EQ(test, 4096, new_gain);
// Unsupported gain 2x
    old_gain = 4;
    old_time_sel = TEST_TSEL_200;
    new_time_sel = TEST_TSEL_400;
    ret = iio_gts_find_new_gain_sel_by_old_gain_time(&gts, old_gain,
    old_time_sel, new_time_sel, &new_gain);
    KUNIT_EXPECT_NE(test, 0, ret);
// Too small gain
    old_gain = 4;
    old_time_sel = TEST_TSEL_50;
    new_time_sel = TEST_TSEL_400;
    ret = iio_gts_find_new_gain_sel_by_old_gain_time(&gts, old_gain,
    old_time_sel, new_time_sel, &new_gain);
    KUNIT_EXPECT_NE(test, 0, ret);
// Too big gain
    old_gain = 1024;
    old_time_sel = TEST_TSEL_400;
    new_time_sel = TEST_TSEL_50;
    ret = iio_gts_find_new_gain_sel_by_old_gain_time(&gts, old_gain,
    old_time_sel, new_time_sel, &new_gain);
    KUNIT_EXPECT_NE(test, 0, ret);
    }
#[no_mangle]
unsafe extern "C" fn test_iio_find_closest_gain_low(test: *mut kunit) {
    static void test_iio_find_closest_gain_low(struct kunit *test)
    {
    struct device *dev;
    bool in_range;
    int ret;
    const struct iio_gain_sel_pair gts_test_gains_gain_low[] = {
    GAIN_SCALE_GAIN(4, TEST_GSEL_4),
    GAIN_SCALE_GAIN(16, TEST_GSEL_16),
    GAIN_SCALE_GAIN(32, TEST_GSEL_32),
    };
    dev = test_init_iio_gain_scale(test, &gts);
    if (!dev)
    return;
    ret = iio_find_closest_gain_low(&gts, 2, &in_range);
    KUNIT_EXPECT_EQ(test, 1, ret);
    KUNIT_EXPECT_EQ(test, true, in_range);
    ret = iio_find_closest_gain_low(&gts, 1, &in_range);
    KUNIT_EXPECT_EQ(test, 1, ret);
    KUNIT_EXPECT_EQ(test, true, in_range);
    ret = iio_find_closest_gain_low(&gts, 4095, &in_range);
    KUNIT_EXPECT_EQ(test, 2048, ret);
    KUNIT_EXPECT_EQ(test, true, in_range);
    ret = iio_find_closest_gain_low(&gts, 4097, &in_range);
    KUNIT_EXPECT_EQ(test, 4096, ret);
    KUNIT_EXPECT_EQ(test, false, in_range);
    kunit_device_unregister(test, dev);
    dev = __test_init_iio_gain_scale(test, &gts, gts_test_gains_gain_low,
    ARRAY_SIZE(gts_test_gains_gain_low),
    gts_test_itimes, ARRAY_SIZE(gts_test_itimes));
    if (!dev)
    return;
    ret = iio_find_closest_gain_low(&gts, 3, &in_range);
    KUNIT_EXPECT_EQ(test, -EINVAL, ret);
    KUNIT_EXPECT_EQ(test, false, in_range);
    }
#[no_mangle]
unsafe extern "C" fn test_iio_gts_total_gain_to_scale(test: *mut kunit) {
    static void test_iio_gts_total_gain_to_scale(struct kunit *test)
    {
    struct device *dev;
    int ret, scale_int, scale_nano;
    dev = test_init_iio_gain_scale(test, &gts);
    if (!dev)
    return;
    ret = iio_gts_total_gain_to_scale(&gts, 1, &scale_int, &scale_nano);
    KUNIT_EXPECT_EQ(test, 0, ret);
    KUNIT_EXPECT_EQ(test, TEST_SCALE_1X, scale_int);
    KUNIT_EXPECT_EQ(test, 0, scale_nano);
    ret = iio_gts_total_gain_to_scale(&gts, 1, &scale_int, &scale_nano);
    KUNIT_EXPECT_EQ(test, 0, ret);
    KUNIT_EXPECT_EQ(test, TEST_SCALE_1X, scale_int);
    KUNIT_EXPECT_EQ(test, 0, scale_nano);
    ret = iio_gts_total_gain_to_scale(&gts, 4096 * 8, &scale_int,
    &scale_nano);
    KUNIT_EXPECT_EQ(test, 0, ret);
    KUNIT_EXPECT_EQ(test, 0, scale_int);
    KUNIT_EXPECT_EQ(test, TEST_SCALE_NANO_4096X8, scale_nano);
    }
#[no_mangle]
unsafe extern "C" fn test_iio_gts_chk_times(test: *mut kunit, vals: *const c_int) {
    static void test_iio_gts_chk_times(struct kunit *test, const int *vals)
    {
    static const int expected[] = {0, 50000, 0, 100000, 0, 200000, 0, 400000};
    int i;
    for (i = 0; i < ARRAY_SIZE(expected); i++)
    KUNIT_EXPECT_EQ(test, expected[i], vals[i]);
    }
    static void test_iio_gts_chk_scales_all(struct kunit *test, struct iio_gts *gts,
    const int *vals, int len)
    {
    static const int gains[] = {1, 2, 4, 8, 16, 32, 64, 128, 256, 512,
    1024, 2048, 4096, 4096 * 2, 4096 * 4,
    4096 * 8};
    int expected[ARRAY_SIZE(gains) * 2];
    int i, ret;
    let mut exp_len: c_int = ARRAY_SIZE(gains) * 2;
    KUNIT_EXPECT_EQ(test, exp_len, len);
    if (len != exp_len)
    return;
    for (i = 0; i < ARRAY_SIZE(gains); i++) {
    ret = iio_gts_total_gain_to_scale(gts, gains[i],
    &expected[2 * i],
    &expected[2 * i + 1]);
    KUNIT_EXPECT_EQ(test, 0, ret);
    if (ret)
    return;
    }
    for (i = 0; i < ARRAY_SIZE(expected); i++)
    KUNIT_EXPECT_EQ(test, expected[i], vals[i]);
    }
    static void test_iio_gts_chk_scales_t200(struct kunit *test, struct iio_gts *gts,
    const int *vals, int len)
    {
// The gain caused by time 200 is 4x
    static const int gains[] = {
    1 * 4,
    4 * 4,
    16 * 4,
    32 * 4,
    64 * 4,
    256 * 4,
    512 * 4,
    1024 * 4,
    2048 * 4,
    4096 * 4
    };
    int expected[ARRAY_SIZE(gains) * 2];
    int i, ret;
    KUNIT_EXPECT_EQ(test, 2 * ARRAY_SIZE(gains), len);
    if (len < 2 * ARRAY_SIZE(gains))
    return;
    for (i = 0; i < ARRAY_SIZE(gains); i++) {
    ret = iio_gts_total_gain_to_scale(gts, gains[i],
    &expected[2 * i],
    &expected[2 * i + 1]);
    KUNIT_EXPECT_EQ(test, 0, ret);
    if (ret)
    return;
    }
    for (i = 0; i < ARRAY_SIZE(expected); i++)
    KUNIT_EXPECT_EQ(test, expected[i], vals[i]);
    }
#[no_mangle]
unsafe extern "C" fn test_iio_gts_avail_test(test: *mut kunit) {
    static void test_iio_gts_avail_test(struct kunit *test)
    {
    struct device *dev;
    int ret;
    int type, len;
    const int *vals;
    dev = test_init_iio_gain_scale(test, &gts);
    if (!dev)
    return;
// test table building for times and iio_gts_avail_times()
    ret = iio_gts_avail_times(&gts, &vals, &type, &len);
    KUNIT_EXPECT_EQ(test, IIO_AVAIL_LIST, ret);
    if (ret)
    return;
    KUNIT_EXPECT_EQ(test, IIO_VAL_INT_PLUS_MICRO, type);
    KUNIT_EXPECT_EQ(test, 8, len);
    if (len < 8)
    return;
    test_iio_gts_chk_times(test, vals);
// Test table building for all scales and iio_gts_all_avail_scales()
    ret = iio_gts_all_avail_scales(&gts, &vals, &type, &len);
    KUNIT_EXPECT_EQ(test, IIO_AVAIL_LIST, ret);
    if (ret)
    return;
    KUNIT_EXPECT_EQ(test, IIO_VAL_INT_PLUS_NANO, type);
    test_iio_gts_chk_scales_all(test, &gts, vals, len);
//
// Test table building for scales/time and
// iio_gts_avail_scales_for_time()
//
    ret = iio_gts_avail_scales_for_time(&gts, 200000, &vals, &type, &len);
    KUNIT_EXPECT_EQ(test, IIO_AVAIL_LIST, ret);
    if (ret)
    return;
    KUNIT_EXPECT_EQ(test, IIO_VAL_INT_PLUS_NANO, type);
    test_iio_gts_chk_scales_t200(test, &gts, vals, len);
    }
    static struct kunit_case iio_gts_test_cases[] = {
    KUNIT_CASE(test_init_iio_gts_invalid),
    KUNIT_CASE(test_iio_gts_find_gain_for_scale_using_time),
    KUNIT_CASE(test_iio_gts_find_new_gain_sel_by_old_gain_time),
    KUNIT_CASE(test_iio_find_closest_gain_low),
    KUNIT_CASE(test_iio_gts_total_gain_to_scale),
    KUNIT_CASE(test_iio_gts_avail_test),
    { }
    };
    static struct kunit_suite iio_gts_test_suite = {
    .name = "iio-gain-time-scale",
    .test_cases = iio_gts_test_cases,
    };
    kunit_test_suite(iio_gts_test_suite);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Matti Vaittinen <mazziesaccount@gmail.com>");
    MODULE_DESCRIPTION("Test IIO light sensor gain-time-scale helpers");
    MODULE_IMPORT_NS("IIO_GTS_HELPER");
