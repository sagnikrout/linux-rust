//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/spi/spi-test.h
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
// linux/drivers/spi/spi-test.h
//
// (c) Martin Sperl <kernel@martin.sperl.org>
//
// spi_test definitions
//

pub const SPI_TEST_MAX_TRANSFERS: c_int = 4;

pub const SPI_TEST_MAX_ITERATE: c_int = 32;
// the "dummy" start addresses used in spi_test
// these addresses get translated at a later stage
//

// some special defines for offsets

// detection pattern for unfinished reads...
// - 0x00 or 0xff could be valid levels for tx_buf = NULL,
// so we do not use either of them
//
pub const SPI_TEST_PATTERN_UNWRITTEN: c_uint = 0xAA;
pub const SPI_TEST_PATTERN_DO_NOT_WRITE: c_uint = 0x55;
pub const SPI_TEST_CHECK_DO_NOT_WRITE: c_int = 64;
//
// struct spi_test - describes a specific (set of) tests to execute
//
// @description:      description of the test
//
// @msg:              a template @spi_message usedfor the default settings
// @transfers:        array of @spi_transfers that are part of the
// resulting spi_message.
// @transfer_count:   number of transfers
//
// @run_test:         run a specific spi_test - this allows to override
// the default implementation of @spi_test_run_transfer
// either to add some custom filters for a specific test
// or to effectively run some very custom tests...
// @execute_msg:      run the spi_message for real - this allows to override
// @spi_test_execute_msg to apply final modifications
// on the spi_message
// @expected_return:  the expected return code - in some cases we want to
// test also for error conditions
//
// @iterate_len:      list of length to iterate on
// @iterate_tx_align: change the alignment of @spi_transfer.tx_buf
// for all values in the below range if set.
// the ranges are:
// [0 : @spi_master.dma_alignment[ if set
// [0 : iterate_tx_align[ if unset
// @iterate_rx_align: change the alignment of @spi_transfer.rx_buf
// see @iterate_tx_align for details
// @iterate_transfer_mask: the bitmask of transfers to which the iterations
// apply - if 0, then it applies to all transfer
//
// @fill_option:      define the way how tx_buf is filled
// @fill_pattern:     fill pattern to apply to the tx_buf
// (used in some of the @fill_options)
// @elapsed_time:     elapsed time in nanoseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_test {
    pub description: [c_char; 64],
    pub msg: spi_message,
    pub transfers: [spi_transfer; SPI_TEST_MAX_TRANSFERS],
    pub transfer_count: c_uint,
    pub rx): *mut *mut void tx, void,
    pub rx): *mut *mut void tx, void,
    pub expected_return: c_int,
// iterate over all values, terminated by a -1
    pub iterate_len: [c_int; SPI_TEST_MAX_ITERATE],
    pub iterate_tx_align: c_int,
    pub iterate_rx_align: c_int,
    pub iterate_transfer_mask: u32,
// the tx-fill operation
    pub fill_option: u32,

    pub fill_pattern: u32,
    pub elapsed_time: c_ulonglong,
}

// default implementation for @spi_test.run_test
// default implementation for @spi_test.execute_msg
// function to execute a set of tests

// some of the default @spi_transfer.len to test, terminated by a -1

// the default alignment to test
