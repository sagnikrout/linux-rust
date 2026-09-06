//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/tdx/tdx_guest_test.c
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
// Test TDX guest features
//
// Copyright (C) 2022 Intel Corporation.
//
// Author: Kuppuswamy Sathyanarayanan <sathyanarayanan.kuppuswamy@linux.intel.com>
//

pub const HEX_DUMP_SIZE: c_int = 8;
pub const DEBUG: c_int = 0;
//
// struct tdreport_type - Type header of TDREPORT_STRUCT.
// @type: Type of the TDREPORT (0 - SGX, 81 - TDX, rest are reserved)
// @sub_type: Subtype of the TDREPORT (Default value is 0).
// @version: TDREPORT version (Default value is 0).
// @reserved: Added for future extension.
//
// More details can be found in TDX v1.0 module specification, sec
// titled "REPORTTYPE".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdreport_type {
    pub type: __u8,
    pub sub_type: __u8,
    pub version: __u8,
    pub reserved: __u8,
}

//
// struct reportmac - TDX guest report data, MAC and TEE hashes.
// @type: TDREPORT type header.
// @reserved1: Reserved for future extension.
// @cpu_svn: CPU security version.
// @tee_tcb_info_hash: SHA384 hash of TEE TCB INFO.
// @tee_td_info_hash: SHA384 hash of TDINFO_STRUCT.
// @reportdata: User defined unique data passed in TDG.MR.REPORT request.
// @reserved2: Reserved for future extension.
// @mac: CPU MAC ID.
//
// It is MAC-protected and contains hashes of the remainder of the
// report structure along with user provided report data. More details can
// be found in TDX v1.0 Module specification, sec titled "REPORTMACSTRUCT"
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reportmac {
    pub type: tdreport_type,
    pub reserved1: [__u8; 12],
    pub cpu_svn: [__u8; 16],
    pub tee_tcb_info_hash: [__u8; 48],
    pub tee_td_info_hash: [__u8; 48],
    pub reportdata: [__u8; 64],
    pub reserved2: [__u8; 32],
    pub mac: [__u8; 32],
}

//
// struct td_info - TDX guest measurements and configuration.
// @attr: TDX Guest attributes (like debug, spet_disable, etc).
// @xfam: Extended features allowed mask.
// @mrtd: Build time measurement register.
// @mrconfigid: Software-defined ID for non-owner-defined configuration
// of the guest - e.g., run-time or OS configuration.
// @mrowner: Software-defined ID for the guest owner.
// @mrownerconfig: Software-defined ID for owner-defined configuration of
// the guest - e.g., specific to the workload.
// @rtmr: Run time measurement registers.
// @reserved: Added for future extension.
//
// It contains the measurements and initial configuration of the TDX guest
// that was locked at initialization and a set of measurement registers
// that are run-time extendable. More details can be found in TDX v1.0
// Module specification, sec titled "TDINFO_STRUCT".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct td_info {
    pub attr: [__u8; 8],
    pub xfam: __u64,
    pub mrtd: [__u64; 6],
    pub mrconfigid: [__u64; 6],
    pub mrowner: [__u64; 6],
    pub mrownerconfig: [__u64; 6],
    pub rtmr: [__u64; 24],
    pub reserved: [__u64; 14],
}

//
// struct tdreport - Output of TDCALL[TDG.MR.REPORT].
// @reportmac: Mac protected header of size 256 bytes.
// @tee_tcb_info: Additional attestable elements in the TCB are not
// reflected in the reportmac.
// @reserved: Added for future extension.
// @tdinfo: Measurements and configuration data of size 512 bytes.
//
// More details can be found in TDX v1.0 Module specification, sec
// titled "TDREPORT_STRUCT".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdreport {
    pub reportmac: reportmac,
    pub tee_tcb_info: [__u8; 239],
    pub reserved: [__u8; 17],
    pub tdinfo: td_info,
}

    static void print_array_hex(const char *title, const char *prefix_str,
    const void *buf, int len)
    {
    int i, j, line_len, rowsize = HEX_DUMP_SIZE;
    const __u8 *ptr = buf;
    printf("\t\t%s", title);
    for (j = 0; j < len; j += rowsize) {
    line_len = rowsize < (len - j) ? rowsize : (len - j);
    printf("%s%.8x:", prefix_str, j);
    for (i = 0; i < line_len; i++)
    printf(" %.2x", ptr[j + i]);
    printf("\n");
    }
    printf("\n");
    }
    TEST(verify_report)
    {
    struct tdx_report_req req;
    struct tdreport *tdreport;
    int devfd, i;
    devfd = open(TDX_GUEST_DEVNAME, O_RDWR | O_SYNC);
    ASSERT_LT(0, devfd);
// Generate sample report data
    for (i = 0; i < TDX_REPORTDATA_LEN; i++)
    req.reportdata[i] = i;
// Get TDREPORT
    ASSERT_EQ(0, ioctl(devfd, TDX_CMD_GET_REPORT0, &req));
    if (DEBUG) {
    print_array_hex("\n\t\tTDX report data\n", "",
    req.reportdata, sizeof(req.reportdata));
    print_array_hex("\n\t\tTDX tdreport data\n", "",
    req.tdreport, sizeof(req.tdreport));
    }
// Make sure TDREPORT data includes the REPORTDATA passed
    tdreport = (struct tdreport *)req.tdreport;
    ASSERT_EQ(0, memcmp(&tdreport.reportmac.reportdata[0],
    req.reportdata, sizeof(req.reportdata)));
    ASSERT_EQ(0, close(devfd));
    }
    TEST_HARNESS_MAIN
