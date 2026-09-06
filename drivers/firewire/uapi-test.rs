//! Automatically rewritten from C to Rust
//! Source: drivers/firewire/uapi-test.c
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
//
// uapi_test.c - An application of Kunit to check layout of structures exposed to user space for
// FireWire subsystem.
//
// Copyright (c) 2023 Takashi Sakamoto

// Known issue added at v2.6.27 kernel.
#[no_mangle]
unsafe extern "C" fn structure_layout_event_response(test: *mut kunit) {
    static void structure_layout_event_response(struct kunit *test)
    {

// 4 bytes alignment for aggregate type including 8 bytes storage types.
    KUNIT_EXPECT_EQ(test, 20, sizeof(struct fw_cdev_event_response));

// 8 bytes alignment for aggregate type including 8 bytes storage types.
    KUNIT_EXPECT_EQ(test, 24, sizeof(struct fw_cdev_event_response));

    KUNIT_EXPECT_EQ(test, 0, offsetof(struct fw_cdev_event_response, closure));
    KUNIT_EXPECT_EQ(test, 8, offsetof(struct fw_cdev_event_response, type));
    KUNIT_EXPECT_EQ(test, 12, offsetof(struct fw_cdev_event_response, rcode));
    KUNIT_EXPECT_EQ(test, 16, offsetof(struct fw_cdev_event_response, length));
    KUNIT_EXPECT_EQ(test, 20, offsetof(struct fw_cdev_event_response, data));
    }
// Added at v6.5.
#[no_mangle]
unsafe extern "C" fn structure_layout_event_request3(test: *mut kunit) {
    static void structure_layout_event_request3(struct kunit *test)
    {
    KUNIT_EXPECT_EQ(test, 56, sizeof(struct fw_cdev_event_request3));
    KUNIT_EXPECT_EQ(test, 0, offsetof(struct fw_cdev_event_request3, closure));
    KUNIT_EXPECT_EQ(test, 8, offsetof(struct fw_cdev_event_request3, type));
    KUNIT_EXPECT_EQ(test, 12, offsetof(struct fw_cdev_event_request3, tcode));
    KUNIT_EXPECT_EQ(test, 16, offsetof(struct fw_cdev_event_request3, offset));
    KUNIT_EXPECT_EQ(test, 24, offsetof(struct fw_cdev_event_request3, source_node_id));
    KUNIT_EXPECT_EQ(test, 28, offsetof(struct fw_cdev_event_request3, destination_node_id));
    KUNIT_EXPECT_EQ(test, 32, offsetof(struct fw_cdev_event_request3, card));
    KUNIT_EXPECT_EQ(test, 36, offsetof(struct fw_cdev_event_request3, generation));
    KUNIT_EXPECT_EQ(test, 40, offsetof(struct fw_cdev_event_request3, handle));
    KUNIT_EXPECT_EQ(test, 44, offsetof(struct fw_cdev_event_request3, length));
    KUNIT_EXPECT_EQ(test, 48, offsetof(struct fw_cdev_event_request3, tstamp));
    KUNIT_EXPECT_EQ(test, 56, offsetof(struct fw_cdev_event_request3, data));
    }
// Added at v6.5.
#[no_mangle]
unsafe extern "C" fn structure_layout_event_response2(test: *mut kunit) {
    static void structure_layout_event_response2(struct kunit *test)
    {
    KUNIT_EXPECT_EQ(test, 32, sizeof(struct fw_cdev_event_response2));
    KUNIT_EXPECT_EQ(test, 0, offsetof(struct fw_cdev_event_response2, closure));
    KUNIT_EXPECT_EQ(test, 8, offsetof(struct fw_cdev_event_response2, type));
    KUNIT_EXPECT_EQ(test, 12, offsetof(struct fw_cdev_event_response2, rcode));
    KUNIT_EXPECT_EQ(test, 16, offsetof(struct fw_cdev_event_response2, length));
    KUNIT_EXPECT_EQ(test, 20, offsetof(struct fw_cdev_event_response2, request_tstamp));
    KUNIT_EXPECT_EQ(test, 24, offsetof(struct fw_cdev_event_response2, response_tstamp));
    KUNIT_EXPECT_EQ(test, 32, offsetof(struct fw_cdev_event_response2, data));
    }
// Added at v6.5.
#[no_mangle]
unsafe extern "C" fn structure_layout_event_phy_packet2(test: *mut kunit) {
    static void structure_layout_event_phy_packet2(struct kunit *test)
    {
    KUNIT_EXPECT_EQ(test, 24, sizeof(struct fw_cdev_event_phy_packet2));
    KUNIT_EXPECT_EQ(test, 0, offsetof(struct fw_cdev_event_phy_packet2, closure));
    KUNIT_EXPECT_EQ(test, 8, offsetof(struct fw_cdev_event_phy_packet2, type));
    KUNIT_EXPECT_EQ(test, 12, offsetof(struct fw_cdev_event_phy_packet2, rcode));
    KUNIT_EXPECT_EQ(test, 16, offsetof(struct fw_cdev_event_phy_packet2, length));
    KUNIT_EXPECT_EQ(test, 20, offsetof(struct fw_cdev_event_phy_packet2, tstamp));
    KUNIT_EXPECT_EQ(test, 24, offsetof(struct fw_cdev_event_phy_packet2, data));
    }
    static struct kunit_case structure_layout_test_cases[] = {
    KUNIT_CASE(structure_layout_event_response),
    KUNIT_CASE(structure_layout_event_request3),
    KUNIT_CASE(structure_layout_event_response2),
    KUNIT_CASE(structure_layout_event_phy_packet2),
    {}
    };
    static struct kunit_suite structure_layout_test_suite = {
    .name = "firewire-uapi-structure-layout",
    .test_cases = structure_layout_test_cases,
    };
    kunit_test_suite(structure_layout_test_suite);
    MODULE_DESCRIPTION("FireWire UAPI unit test suite");
    MODULE_LICENSE("GPL");
