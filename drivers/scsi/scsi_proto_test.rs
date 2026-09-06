//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/scsi_proto_test.c
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
// Copyright 2023 Google LLC
//

#[no_mangle]
unsafe extern "C" fn test_scsi_proto(test: *mut kunit) {
    static void test_scsi_proto(struct kunit *test)
    {
    static const union {
    struct scsi_io_group_descriptor desc;
    u8 arr[sizeof(struct scsi_io_group_descriptor)];
    } d = { .arr = { 0x45, 0, 0, 0, 0xb0, 0xe4, 0xe3 } };
    KUNIT_EXPECT_EQ(test, d.desc.io_advice_hints_mode + 0, 1);
    KUNIT_EXPECT_EQ(test, d.desc.st_enble + 0, 1);
    KUNIT_EXPECT_EQ(test, d.desc.cs_enble + 0, 0);
    KUNIT_EXPECT_EQ(test, d.desc.ic_enable + 0, 1);
    KUNIT_EXPECT_EQ(test, d.desc.acdlu + 0, 1);
    KUNIT_EXPECT_EQ(test, d.desc.rlbsr + 0, 3);
    KUNIT_EXPECT_EQ(test, d.desc.lbm_descriptor_type + 0, 0);
    KUNIT_EXPECT_EQ(test, d.desc.params[0] + 0, 0xe4);
    KUNIT_EXPECT_EQ(test, d.desc.params[1] + 0, 0xe3);
    static const union {
    struct scsi_stream_status s;
    u8 arr[sizeof(struct scsi_stream_status)];
    } ss = { .arr = { 0x80, 0, 0x12, 0x34, 0x3f } };
    KUNIT_EXPECT_EQ(test, ss.s.perm + 0, 1);
    KUNIT_EXPECT_EQ(test, get_unaligned_be16(&ss.s.stream_identifier),
    0x1234);
    KUNIT_EXPECT_EQ(test, ss.s.rel_lifetime + 0, 0x3f);
    static const union {
    struct scsi_stream_status_header h;
    u8 arr[sizeof(struct scsi_stream_status_header)];
    } sh = { .arr = { 1, 2, 3, 4, 0, 0, 5, 6 } };
    KUNIT_EXPECT_EQ(test, get_unaligned_be32(&sh.h.len), 0x1020304);
    KUNIT_EXPECT_EQ(test, get_unaligned_be16(&sh.h.number_of_open_streams),
    0x506);
    }
    static struct kunit_case scsi_proto_test_cases[] = {
    KUNIT_CASE(test_scsi_proto),
    {}
    };
    static struct kunit_suite scsi_proto_test_suite = {
    .name = "scsi_proto",
    .test_cases = scsi_proto_test_cases,
    };
    kunit_test_suite(scsi_proto_test_suite);
    MODULE_DESCRIPTION("<scsi/scsi_proto.h> unit tests");
    MODULE_AUTHOR("Bart Van Assche");
    MODULE_LICENSE("GPL");
