//! Automatically rewritten from C to Rust
//! Source: drivers/virt/nitro_enclaves/ne_misc_dev_test.c
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

pub const MAX_PHYS_REGIONS: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ne_phys_regions_test {
    pub paddr: u64,
    pub size: u64,
    pub expect_rc: c_int,
    pub expect_num: c_ulong,
    pub expect_last_paddr: u64,
    pub expect_last_size: u64,
    } phys_regions_test_cases[] = {
//
// Add the region from 0x1000 to (0x1000 + 0x200000 - 1):
// Expected result:
// Failed, start address is not 2M-aligned
//
// Now the instance of struct ne_phys_contig_mem_regions is:
// num = 0
// regions = {}
//
    {0x1000, 0x200000, -EINVAL, 0, INVALID_VALUE, INVALID_VALUE},
//
// Add the region from 0x200000 to (0x200000 + 0x1000 - 1):
// Expected result:
// Failed, size is not 2M-aligned
//
// Now the instance of struct ne_phys_contig_mem_regions is:
// num = 0
// regions = {}
//
    {0x200000, 0x1000, -EINVAL, 0, INVALID_VALUE, INVALID_VALUE},
//
// Add the region from 0x200000 to (0x200000 + 0x200000 - 1):
// Expected result:
// Successful
//
// Now the instance of struct ne_phys_contig_mem_regions is:
// num = 1
// regions = {
// {start=0x200000, end=0x3fffff}, // len=0x200000
// }
//
    {0x200000, 0x200000, 0, 1, 0x200000, 0x200000},
//
// Add the region from 0x0 to (0x0 + 0x200000 - 1):
// Expected result:
// Successful
//
// Now the instance of struct ne_phys_contig_mem_regions is:
// num = 2
// regions = {
// {start=0x200000, end=0x3fffff}, // len=0x200000
// {start=0x0,      end=0x1fffff}, // len=0x200000
// }
//
    {0x0, 0x200000, 0, 2, 0x0, 0x200000},
//
// Add the region from 0x600000 to (0x600000 + 0x400000 - 1):
// Expected result:
// Successful
//
// Now the instance of struct ne_phys_contig_mem_regions is:
// num = 3
// regions = {
// {start=0x200000, end=0x3fffff}, // len=0x200000
// {start=0x0,      end=0x1fffff}, // len=0x200000
// {start=0x600000, end=0x9fffff}, // len=0x400000
// }
//
    {0x600000, 0x400000, 0, 3, 0x600000, 0x400000},
//
// Add the region from 0xa00000 to (0xa00000 + 0x400000 - 1):
// Expected result:
// Successful, merging case!
//
// Now the instance of struct ne_phys_contig_mem_regions is:
// num = 3
// regions = {
// {start=0x200000, end=0x3fffff}, // len=0x200000
// {start=0x0,      end=0x1fffff}, // len=0x200000
// {start=0x600000, end=0xdfffff}, // len=0x800000
// }
//
    {0xa00000, 0x400000, 0, 3, 0x600000, 0x800000},
//
// Add the region from 0x1000 to (0x1000 + 0x200000 - 1):
// Expected result:
// Failed, start address is not 2M-aligned
//
// Now the instance of struct ne_phys_contig_mem_regions is:
// num = 3
// regions = {
// {start=0x200000, end=0x3fffff}, // len=0x200000
// {start=0x0,      end=0x1fffff}, // len=0x200000
// {start=0x600000, end=0xdfffff}, // len=0x800000
// }
//
    {0x1000, 0x200000, -EINVAL, 3, 0x600000, 0x800000},
}

#[no_mangle]
unsafe extern "C" fn ne_misc_dev_test_merge_phys_contig_memory_regions(test: *mut kunit) {
    static void ne_misc_dev_test_merge_phys_contig_memory_regions(struct kunit *test)
    {
    let mut phys_contig_mem_regions: ne_phys_contig_mem_regions = {};
    let mut rc: c_int = 0;
    let mut i: c_int = 0;
    phys_contig_mem_regions.regions = kunit_kcalloc(test, MAX_PHYS_REGIONS,
    sizeof(*phys_contig_mem_regions.regions),
    GFP_KERNEL);
    KUNIT_ASSERT_TRUE(test, phys_contig_mem_regions.regions);
    for (i = 0; i < ARRAY_SIZE(phys_regions_test_cases); i++) {
    struct ne_phys_regions_test *test_case = &phys_regions_test_cases[i];
    let mut num: c_ulong = 0;
    rc = ne_merge_phys_contig_memory_regions(&phys_contig_mem_regions,
    test_case.paddr, test_case.size);
    KUNIT_EXPECT_EQ(test, rc, test_case.expect_rc);
    KUNIT_EXPECT_EQ(test, phys_contig_mem_regions.num, test_case.expect_num);
    if (test_case.expect_last_paddr == INVALID_VALUE)
    continue;
    num = phys_contig_mem_regions.num;
    KUNIT_EXPECT_EQ(test, phys_contig_mem_regions.regions[num - 1].start,
    test_case.expect_last_paddr);
    KUNIT_EXPECT_EQ(test, range_len(&phys_contig_mem_regions.regions[num - 1]),
    test_case.expect_last_size);
    }
    kunit_kfree(test, phys_contig_mem_regions.regions);
    }
    static struct kunit_case ne_misc_dev_test_cases[] = {
    KUNIT_CASE(ne_misc_dev_test_merge_phys_contig_memory_regions),
    {}
    };
    static struct kunit_suite ne_misc_dev_test_suite = {
    .name = "ne_misc_dev_test",
    .test_cases = ne_misc_dev_test_cases,
    };
    kunit_test_suite(ne_misc_dev_test_suite);
