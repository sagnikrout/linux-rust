//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/intel/ifs/ifs.h
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
// Copyright(c) 2022 Intel Corporation.
//
// DOC: In-Field Scan
//
// =============
// In-Field Scan
// =============
//
// Introduction
// ------------
//
// In Field Scan (IFS) is a hardware feature to run circuit level tests on
// a CPU core to detect problems that are not caught by parity or ECC checks.
// Future CPUs will support more than one type of test which will show up
// with a new platform-device instance-id.
//
// IFS Image
// ---------
//
// Intel provides firmware files containing the scan tests via the webpage [#f1]_.
// Look under "In-Field Scan Test Images Download" section towards the
// end of the page. Similar to microcode, there are separate files for each
// family-model-stepping. IFS Images are not applicable for some test types.
// Wherever applicable the sysfs directory would provide a "current_batch" file
// (see below) for loading the image.
//
// .. [#f1] https://intel.com/InFieldScan
//
// IFS Image Loading
// -----------------
//
// The driver loads the tests into memory reserved BIOS local to each CPU
// socket in a two step process using writes to MSRs to first load the
// SHA hashes for the test. Then the tests themselves. Status MSRs provide
// feedback on the success/failure of these steps.
//
// The test files are kept in a fixed location: /lib/firmware/intel/ifs_<n>
// For e.g if there are 3 test files, they would be named in the following
// fashion:
// ff-mm-ss-01.scan
// ff-mm-ss-02.scan
// ff-mm-ss-03.scan
// (where ff refers to family, mm indicates model and ss indicates stepping)
//
// A different test file can be loaded by writing the numerical portion
// (e.g 1, 2 or 3 in the above scenario) into the curent_batch file.
// To load ff-mm-ss-02.scan, the following command can be used::
//
// # echo 2 > /sys/devices/virtual/misc/intel_ifs_<n>/current_batch
//
// The above file can also be read to know the currently loaded image.
//
// Running tests
// -------------
//
// Tests are run by the driver synchronizing execution of all threads on a
// core and then writing to the ACTIVATE_SCAN MSR on all threads. Instruction
// execution continues when:
//
// 1) All tests have completed.
// 2) Execution was interrupted.
// 3) A test detected a problem.
//
// Note that ALL THREADS ON THE CORE ARE EFFECTIVELY OFFLINE FOR THE
// DURATION OF THE TEST. This can be up to 200 milliseconds. If the system
// is running latency sensitive applications that cannot tolerate an
// interruption of this magnitude, the system administrator must arrange
// to migrate those applications to other cores before running a core test.
// It may also be necessary to redirect interrupts to other CPUs.
//
// In all cases reading the corresponding test's STATUS MSR provides details on what
// happened. The driver makes the value of this MSR visible to applications
// via the "details" file (see below). Interrupted tests may be restarted.
//
// The IFS driver provides sysfs interfaces via /sys/devices/virtual/misc/intel_ifs_<n>
// to control execution:
//
// Test a specific core::
//
// # echo <cpu#> > /sys/devices/virtual/misc/intel_ifs_<n>/run_test
//
// when HT is enabled any of the sibling cpu# can be specified to test
// its corresponding physical core. Since the tests are per physical core,
// the result of testing any thread is same. All siblings must be online
// to run a core test. It is only necessary to test one thread.
//
// For e.g. to test core corresponding to cpu5
//
// # echo 5 > /sys/devices/virtual/misc/intel_ifs_<n>/run_test
//
// Results of the last test is provided in /sys::
//
// $ cat /sys/devices/virtual/misc/intel_ifs_<n>/status
// pass
//
// Status can be one of pass, fail, untested
//
// Additional details of the last test is provided by the details file::
//
// $ cat /sys/devices/virtual/misc/intel_ifs_<n>/details
// 0x8081
//
// The details file reports the hex value of the test specific status MSR.
// Hardware defined error codes are documented in volume 4 of the Intel
// Software Developer's Manual but the error_code field may contain one of
// the following driver defined software codes:
//
// +------+--------------------+
// | 0xFD | Software timeout   |
// +------+--------------------+
// | 0xFE | Partial completion |
// +------+--------------------+
//
// Driver design choices
// ---------------------
//
// 1) The ACTIVATE_SCAN MSR allows for running any consecutive subrange of
// available tests. But the driver always tries to run all tests and only
// uses the subrange feature to restart an interrupted test.
//
// 2) Hardware allows for some number of cores to be tested in parallel.
// The driver does not make use of this, it only tests one core at a time.
//
// Structural Based Functional Test at Field (SBAF):
// -------------------------------------------------
//
// SBAF is a new type of testing that provides comprehensive core test
// coverage complementing Scan at Field (SAF) testing. SBAF mimics the
// manufacturing screening environment and leverages the same test suite.
// It makes use of Design For Test (DFT) observation sites and features
// to maximize coverage in minimum time.
//
// Similar to the SAF test, SBAF isolates the core under test from the
// rest of the system during execution. Upon completion, the core
// seamlessly resets to its pre-test state and resumes normal operation.
// Any machine checks or hangs encountered during the test are confined to
// the isolated core, preventing disruption to the overall system.
//
// Like the SAF test, the SBAF test is also divided into multiple batches,
// and each batch test can take hundreds of milliseconds (100-200 ms) to
// complete. If such a lengthy interruption is undesirable, it is
// recommended to relocate the time-sensitive applications to other cores.
//

pub const MSR_ARRAY_BIST: c_uint = 0x00000105;
pub const MSR_COPY_SBAF_HASHES: c_uint = 0x000002b8;
pub const MSR_SBAF_HASHES_STATUS: c_uint = 0x000002b9;
pub const MSR_AUTHENTICATE_AND_COPY_SBAF_CHUNK: c_uint = 0x000002ba;
pub const MSR_SBAF_CHUNKS_AUTHENTICATION_STATUS: c_uint = 0x000002bb;
pub const MSR_ACTIVATE_SBAF: c_uint = 0x000002bc;
pub const MSR_SBAF_STATUS: c_uint = 0x000002bd;
pub const MSR_COPY_SCAN_HASHES: c_uint = 0x000002c2;
pub const MSR_SCAN_HASHES_STATUS: c_uint = 0x000002c3;
pub const MSR_AUTHENTICATE_AND_COPY_CHUNK: c_uint = 0x000002c4;
pub const MSR_CHUNKS_AUTHENTICATION_STATUS: c_uint = 0x000002c5;
pub const MSR_ACTIVATE_SCAN: c_uint = 0x000002c6;
pub const MSR_SCAN_STATUS: c_uint = 0x000002c7;
pub const MSR_ARRAY_TRIGGER: c_uint = 0x000002d6;
pub const MSR_ARRAY_STATUS: c_uint = 0x000002d7;
pub const MSR_SAF_CTRL: c_uint = 0x000004f0;
pub const MSR_SBAF_CTRL: c_uint = 0x000004f8;
pub const SCAN_NOT_TESTED: c_int = 0;
pub const SCAN_TEST_PASS: c_int = 1;
pub const SCAN_TEST_FAIL: c_int = 2;
pub const IFS_TYPE_SAF: c_int = 0;
pub const IFS_TYPE_ARRAY_BIST: c_int = 1;
pub const IFS_TYPE_SBAF: c_int = 2;
pub const ARRAY_GEN0: c_int = 0;
pub const ARRAY_GEN1: c_int = 1;
// MSR_SCAN_HASHES_STATUS bit fields
#[repr(C)]
#[derive(Copy, Clone)]
pub union ifs_scan_hashes_status {
    pub data: u64,
    pub :16: u32 chunk_size,
    pub :8: u32 num_chunks,
    pub :8: u32 rsvd1,
    pub :8: u32 error_code,
    pub :11: u32 rsvd2,
    pub :12: u32 max_core_limit,
    pub :1: u32 valid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ifs_scan_hashes_status_gen2 {
    pub data: u64,
    pub chunk_size: u16,
    pub num_chunks: u16,
    pub :8: u32 error_code,
    pub :9: u32 chunks_in_stride,
    pub :2: u32 rsvd,
    pub :12: u32 max_core_limit,
    pub :1: u32 valid,
}

// MSR_CHUNKS_AUTH_STATUS bit fields
#[repr(C)]
#[derive(Copy, Clone)]
pub union ifs_chunks_auth_status {
    pub data: u64,
    pub :8: u32 valid_chunks,
    pub :8: u32 total_chunks,
    pub :16: u32 rsvd1,
    pub :8: u32 error_code,
    pub :24: u32 rsvd2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ifs_chunks_auth_status_gen2 {
    pub data: u64,
    pub valid_chunks: u16,
    pub total_chunks: u16,
    pub :8: u32 error_code,
    pub :8: u32 rsvd2,
    pub :16: u32 max_bundle,
}

// MSR_ACTIVATE_SCAN bit fields
#[repr(C)]
#[derive(Copy, Clone)]
pub union ifs_scan {
    pub data: u64,
    pub start: u8,
    pub stop: u8,
    pub rsvd: u16,
    pub gen0: },
    pub start: u16,
    pub stop: u16,
    pub gen2: },
}

// MSR_SCAN_STATUS bit fields
#[repr(C)]
#[derive(Copy, Clone)]
pub union ifs_status {
    pub data: u64,
    pub chunk_num: u8,
    pub chunk_stop_index: u8,
    pub rsvd1: u16,
    pub gen0: },
    pub chunk_num: u16,
    pub chunk_stop_index: u16,
    pub gen2: },
}

// MSR_ARRAY_BIST bit fields
#[repr(C)]
#[derive(Copy, Clone)]
pub union ifs_array {
    pub data: u64,
    pub array_bitmask: u32,
    pub array_bank: u16,
    pub :15: u16 rsvd,
    pub :1: u16 ctrl_result,
}

// MSR_ACTIVATE_SBAF bit fields
#[repr(C)]
#[derive(Copy, Clone)]
pub union ifs_sbaf {
    pub data: u64,
    pub :9: u32 bundle_idx,
    pub :5: u32 rsvd1,
    pub :2: u32 pgm_idx,
    pub :16: u32 rsvd2,
    pub :31: u32 delay,
    pub :1: u32 sigmce,
}

// MSR_SBAF_STATUS bit fields
#[repr(C)]
#[derive(Copy, Clone)]
pub union ifs_sbaf_status {
    pub data: u64,
    pub :9: u32 bundle_idx,
    pub :5: u32 rsvd1,
    pub :2: u32 pgm_idx,
    pub :16: u32 rsvd2,
    pub :8: u32 error_code,
    pub :21: u32 rsvd3,
    pub :1: u32 test_fail,
    pub :2: u32 sbaf_status,
}

//
// Driver populated error-codes
// 0xFD: Test timed out before completing all the chunks.
// 0xFE: not all scan chunks were executed. Maximum forward progress retries exceeded.
//
pub const IFS_SW_TIMEOUT: c_uint = 0xFD;
pub const IFS_SW_PARTIAL_COMPLETION: c_uint = 0xFE;
pub const IFS_SUFFIX_SZ: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifs_test_caps {
    pub integrity_cap_bit: c_int,
    pub test_num: c_int,
    pub image_suffix: [c_char; IFS_SUFFIX_SZ],
}

//
// struct ifs_test_msrs - MSRs used in IFS tests
// @copy_hashes: Copy test hash data
// @copy_hashes_status: Status of copied test hash data
// @copy_chunks: Copy chunks of the test data
// @copy_chunks_status: Status of the copied test data chunks
// @test_ctrl: Control the test attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifs_test_msrs {
    pub copy_hashes: u32,
    pub copy_hashes_status: u32,
    pub copy_chunks: u32,
    pub copy_chunks_status: u32,
    pub test_ctrl: u32,
}

//
// struct ifs_data - attributes related to intel IFS driver
// @loaded_version: stores the currently loaded ifs image version.
// @loaded: If a valid test binary has been loaded into the memory
// @loading_error: Error occurred on another CPU while loading image
// @valid_chunks: number of chunks which could be validated.
// @status: it holds simple status pass/fail/untested
// @scan_details: opaque scan status code from h/w
// @cur_batch: number indicating the currently loaded test file
// @generation: IFS test generation enumerated by hardware
// @chunk_size: size of a test chunk
// @array_gen: test generation of array test
// @max_bundle: maximum bundle index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifs_data {
    pub loaded_version: c_int,
    pub loaded: bool,
    pub loading_error: bool,
    pub valid_chunks: c_int,
    pub status: c_int,
    pub scan_details: u64,
    pub cur_batch: u32,
    pub generation: u32,
    pub chunk_size: u32,
    pub array_gen: u32,
    pub max_bundle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifs_work {
    pub w: work_struct,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifs_device {
    pub test_caps: *const ifs_test_caps,
    pub test_msrs: *const ifs_test_msrs,
    pub rw_data: ifs_data,
    pub misc: miscdevice,
}

extern "C" {
    pub fn ifs_load_firmware(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn do_core_test(cpu: c_int, dev: *mut device) -> c_int;
}
