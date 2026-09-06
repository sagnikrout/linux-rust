//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/tests/hash-test-template.h
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
// Test cases for hash functions, including a benchmark.  This is included by
// KUnit test suites that want to use it.  See sha512_kunit.c for an example.
//
// Copyright 2025 Google LLC
//

//
// Test the hash function against a list of test vectors.
//
// Note that it's only necessary to run each test vector in one way (e.g.,
// one-shot instead of incremental), since consistency between different ways of
// using the APIs is verified by other test cases.
//
// Test that the hash function produces correct results for *every* length up to
// 4096 bytes.  To do this, generate seeded random data, then calculate a hash
// value for each length 0..4096, then hash the hash values.  Verify just the
// final hash value, which should match only when all hash values were correct.
//
// Test that the hash function produces the same result with a one-shot
// computation as it does with an incremental computation.
//
// Compute the hash value in one shot.
//
// Compute the hash value incrementally, using a randomly
// selected sequence of update lengths that sum to total_len.
//
// Verify that the two hash values are the same.
//
// Test that the hash function does not overrun any buffers.  Uses a guard page
// to catch buffer overruns even if they occur in assembly code.
//
// Check for overruns of the data buffer.
// Check for overruns of the hash value buffer.
// Check for overruns of the hash context.
//
// Test that the caller is permitted to alias the output digest and source data
// buffer, and also modify the source data buffer after it has been used.
//
// Repeat the above test, but this time use init+update+final
// Test modifying the source data after it was used.
//
// Test that if the same data is hashed at different alignments in memory, the
// results are the same.
//
// Test that HASH_FINAL zeroizes the context.
pub const IRQ_TEST_DATA_LEN: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_irq_test1_state {
    pub data: *mut u8,
    pub expected_hashes: [u8; IRQ_TEST_NUM_BUFFERS][HASH_SIZE],
    pub seqno: core::sync::atomic::AtomicI32,
}

//
// Compute the hash of one of the test messages and verify that it matches the
// expected hash from @state->expected_hashes.  To increase the chance of
// detecting problems, cycle through multiple messages.
//
// Test that if hashes are computed in task, softirq, and hardirq context
// concurrently, then all results are as expected.
//
// Prepare some test messages and compute the expected hash of each.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_irq_test2_hash_ctx {
    pub hash_ctx: HASH_CTX,
    pub in_use: core::sync::atomic::AtomicI32,
    pub offset: c_int,
    pub step: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_irq_test2_state {
    pub data: *mut u8,
    pub data_len: usize,
    pub ctxs: [hash_irq_test2_hash_ctx; IRQ_TEST_NUM_BUFFERS],
    pub expected_hash: [u8; HASH_SIZE],
    pub update_lens: [u16; 32],
    pub num_steps: c_int,
}

//
// This should never happen, as the number of contexts is equal
// to the maximum concurrency level of kunit_run_irq_test().
//
// Init step
// Update step
// Final step
//
// Test that if hashes are computed in task, softirq, and hardirq context
// concurrently, *including doing different parts of the same incremental
// computation in different contexts*, then all results are as expected.
// Besides detecting bugs similar to those that test_hash_interrupt_context_1
// can detect, this test case can also detect bugs where hash function
// implementations don't correctly handle these mixed incremental computations.
//
// Generate a list of update lengths to use.  Ensure that it contains
// multiple entries but is limited to a maximum length.
//

// benchmark_hash is omitted so that the suites can put it last.

//
// Test the corresponding HMAC variant.
//
// This test case is fairly short, since HMAC is just a simple C wrapper around
// the underlying unkeyed hash function, which is already well-tested by the
// other test cases.  It's not useful to test things like data alignment or
// interrupt context again for HMAC, nor to have a long list of test vectors.
//
// Thus, just do a single consolidated test, which covers all data lengths up to
// 4096 bytes and all key lengths up to 292 bytes.  For each data length, select
// a key length, generate the inputs from a seed, and compute the HMAC value.
// Concatenate all these HMAC values together, and compute the HMAC of that.
// Verify that value.  If this fails, then the HMAC implementation is wrong.
// This won't show which specific input failed, but that should be fine.  Any
// failure would likely be non-input-specific or also show in the unkeyed tests.
//
// Cycle through key lengths as well.  Somewhat arbitrarily go
// up to 293, which is somewhat larger than the largest hash
// block size (which is the size at which the key starts being
// hashed down to one block); going higher would not be useful.
// To reduce correlation with data_len, use a prime number here.
//
// Verify that HMAC() is consistent with HMAC_USINGRAWKEY().

// Benchmark the hash function on various data lengths.
// Warm-up
// The '+ 128' tries to account for per-message overhead.
