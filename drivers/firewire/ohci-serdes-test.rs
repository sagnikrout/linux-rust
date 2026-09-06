//! Automatically rewritten from C to Rust
//! Source: drivers/firewire/ohci-serdes-test.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// ohci-serdes-test.c - An application of Kunit to check serialization/deserialization of data in
// buffers and registers defined in 1394 OHCI specification.
//
// Copyright (c) 2024 Takashi Sakamoto

#[no_mangle]
unsafe extern "C" fn test_self_id_count_register_deserialization(test: *mut kunit) {
    static void test_self_id_count_register_deserialization(struct kunit *test)
    {
    let mut expected: u32 = 0x803d0594;
    let mut is_error: bool = ohci1394_self_id_count_is_error(expected);
    let mut generation: u8 = ohci1394_self_id_count_get_generation(expected);
    let mut size: u32 = ohci1394_self_id_count_get_size(expected);
    KUNIT_EXPECT_TRUE(test, is_error);
    KUNIT_EXPECT_EQ(test, 0x3d, generation);
    KUNIT_EXPECT_EQ(test, 0x165, size);
    }
#[no_mangle]
unsafe extern "C" fn test_self_id_receive_buffer_deserialization(test: *mut kunit) {
    static void test_self_id_receive_buffer_deserialization(struct kunit *test)
    {
    const u32 buffer[] = {
    0x0006f38b,
    0x807fcc56,
    0x7f8033a9,
    0x8145cc5e,
    0x7eba33a1,
    };
    let mut generation: u8 = ohci1394_self_id_receive_q0_get_generation(buffer[0]);
    let mut timestamp: u16 = ohci1394_self_id_receive_q0_get_timestamp(buffer[0]);
    KUNIT_EXPECT_EQ(test, 0x6, generation);
    KUNIT_EXPECT_EQ(test, 0xf38b, timestamp);
    }
#[no_mangle]
unsafe extern "C" fn test_at_data_serdes(test: *mut kunit) {
    static void test_at_data_serdes(struct kunit *test)
    {
    static const __le32 expected[] = {
    cpu_to_le32(0x00020e80),
    cpu_to_le32(0xffc2ffff),
    cpu_to_le32(0xe0000000),
    };
    __le32 quadlets[] = {0, 0, 0};
    let mut has_src_bus_id: bool = ohci1394_at_data_get_src_bus_id(expected);
    let mut speed: c_uint = ohci1394_at_data_get_speed(expected);
    let mut tlabel: c_uint = ohci1394_at_data_get_tlabel(expected);
    let mut retry: c_uint = ohci1394_at_data_get_retry(expected);
    let mut tcode: c_uint = ohci1394_at_data_get_tcode(expected);
    let mut destination_id: c_uint = ohci1394_at_data_get_destination_id(expected);
    let mut destination_offset: u64 = ohci1394_at_data_get_destination_offset(expected);
    KUNIT_EXPECT_FALSE(test, has_src_bus_id);
    KUNIT_EXPECT_EQ(test, 0x02, speed);
    KUNIT_EXPECT_EQ(test, 0x03, tlabel);
    KUNIT_EXPECT_EQ(test, 0x02, retry);
    KUNIT_EXPECT_EQ(test, 0x08, tcode);
    ohci1394_at_data_set_src_bus_id(quadlets, has_src_bus_id);
    ohci1394_at_data_set_speed(quadlets, speed);
    ohci1394_at_data_set_tlabel(quadlets, tlabel);
    ohci1394_at_data_set_retry(quadlets, retry);
    ohci1394_at_data_set_tcode(quadlets, tcode);
    ohci1394_at_data_set_destination_id(quadlets, destination_id);
    ohci1394_at_data_set_destination_offset(quadlets, destination_offset);
    KUNIT_EXPECT_MEMEQ(test, quadlets, expected, sizeof(expected));
    }
#[no_mangle]
unsafe extern "C" fn test_it_data_serdes(test: *mut kunit) {
    static void test_it_data_serdes(struct kunit *test)
    {
    static const __le32 expected[] = {
    cpu_to_le32(0x000349a7),
    cpu_to_le32(0x02300000),
    };
    __le32 quadlets[] = {0, 0};
    let mut scode: c_uint = ohci1394_it_data_get_speed(expected);
    let mut tag: c_uint = ohci1394_it_data_get_tag(expected);
    let mut channel: c_uint = ohci1394_it_data_get_channel(expected);
    let mut tcode: c_uint = ohci1394_it_data_get_tcode(expected);
    let mut sync: c_uint = ohci1394_it_data_get_sync(expected);
    let mut data_length: c_uint = ohci1394_it_data_get_data_length(expected);
    KUNIT_EXPECT_EQ(test, 0x03, scode);
    KUNIT_EXPECT_EQ(test, 0x01, tag);
    KUNIT_EXPECT_EQ(test, 0x09, channel);
    KUNIT_EXPECT_EQ(test, 0x0a, tcode);
    KUNIT_EXPECT_EQ(test, 0x7, sync);
    KUNIT_EXPECT_EQ(test, 0x0230, data_length);
    ohci1394_it_data_set_speed(quadlets, scode);
    ohci1394_it_data_set_tag(quadlets, tag);
    ohci1394_it_data_set_channel(quadlets, channel);
    ohci1394_it_data_set_tcode(quadlets, tcode);
    ohci1394_it_data_set_sync(quadlets, sync);
    ohci1394_it_data_set_data_length(quadlets, data_length);
    KUNIT_EXPECT_MEMEQ(test, quadlets, expected, sizeof(expected));
    }
    static struct kunit_case ohci_serdes_test_cases[] = {
    KUNIT_CASE(test_self_id_count_register_deserialization),
    KUNIT_CASE(test_self_id_receive_buffer_deserialization),
    KUNIT_CASE(test_at_data_serdes),
    KUNIT_CASE(test_it_data_serdes),
    {}
    };
    static struct kunit_suite ohci_serdes_test_suite = {
    .name = "firewire-ohci-serdes",
    .test_cases = ohci_serdes_test_cases,
    };
    kunit_test_suite(ohci_serdes_test_suite);
    MODULE_DESCRIPTION("FireWire buffers and registers serialization/deserialization unit test suite");
    MODULE_LICENSE("GPL");
