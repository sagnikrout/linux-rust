//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/iommu/iommufd_utils.h
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
// Copyright (c) 2021-2022, NVIDIA CORPORATION & AFFILIATES

// Hack to make assertions more readable

// Imported from include/asm-generic/bitops/generic-non-atomic.h
pub const BITS_PER_BYTE: c_int = 8;

// p |= mask;

// mfd_p = mfd;
//
// Have the kernel check the refcount on pages. I don't know why a freshly
// mmap'd anon non-compound page starts out with a ref of 3
//

// stdev_id = cmd.mock_domain.out_stdev_id;
// hwpt_id = cmd.mock_domain.out_hwpt_id;
// idev_id = cmd.mock_domain.out_idev_id;

// stdev_id = cmd.mock_domain_flags.out_stdev_id;
// hwpt_id = cmd.mock_domain_flags.out_hwpt_id;
// idev_id = cmd.mock_domain_flags.out_idev_id;

// hwpt_id = cmd.mock_domain_replace.pt_id;

// hwpt_id = cmd.out_hwpt_id;

// nreqs = cmd.entry_num;

// nreqs = cmd.entry_num;

// dirty = cmd.dirty.out_nr_dirty;

// Mark all even bits as dirty in the mock domain
// Expect all even bits as dirty in the user bitmap
// Beware ASSERT_EQ() is two statements -- braces are not redundant!
// It as read already -- expect all zeroes

// access_id = cmd.create_access.out_access_fd;

extern "C" {
    pub fn close(_arg: access_id) -> return;
}

extern "C" {
    pub fn ioctl(_arg: fd, _arg: IOMMU_TEST_CMD, _arg: &cmd) -> return;
}

// out_fd = ioctl(fd, IOMMU_TEST_CMD, &cmd);

extern "C" {
    pub fn ioctl(_arg: fd, _arg: IOMMU_DESTROY, _arg: &cmd) -> return;
}

// id = cmd.out_ioas_id;

// iova = cmd.iova;

// out_len = cmd.length;

// iova = cmd.iova;

// @data can be NULL
//
// The struct iommu_test_hw_info should be the one defined
// by the current kernel.
//
// Trailing bytes should be 0 if user buffer is larger than
// the data that kernel reports.
//
// max_pasid = cmd.out_max_pasid_log2;
// capabilities = cmd.out_capabilities;

// fault_id = cmd.out_fault_id;
// fault_fd = cmd.out_fault_fd;

// viommu_id = cmd.out_viommu_id;

// vdev_id = cmd.out_vdevice_id;

// hw_queue_id = cmd.out_hw_queue_id;

// veventq_id = cmd.out_veventq_id;
// veventq_fd = cmd.out_veventq_fd;

// prev_seq = hdr->sequence;

