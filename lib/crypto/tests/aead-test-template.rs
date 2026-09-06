//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/tests/aead-test-template.h
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
// Shared KUnit test cases for AEAD algorithms, including a benchmark
//
// Copyright 2026 Google LLC
//
// This file implements KUnit test cases shared by the different KUnit test
// suites for Authenticated Encryption with Associated Data (AEAD) algorithms.
//
// Test suites including this file must #define the following:
//
// Data structs:
// - AEAD_KEY: name of key struct
// - AEAD_CTX: name of context for incremental computation
//
// Constants:
// - AEAD_VALID_KEY_LENS: array of all valid key lengths in bytes
// - AEAD_VALID_NONCE_LENS: array of all valid nonce lengths in bytes
// - AEAD_VALID_TAG_LENS: array of all valid authtag lengths in bytes
// - AEAD_MAX_KEY_LEN: max key length in bytes (assumed to fit on stack)
// - AEAD_MAX_NONCE_LEN: max nonce length in bytes (assumed to fit on stack)
// - AEAD_MAX_TAG_LEN: max authtag length in bytes (assumed to fit on stack)
// - AEAD_MONTE_CARLO_CHECKSUM: checksum of a deterministically generated series
// of (ciphertext, authtag) pairs (see test_aead_monte_carlo())
//
// Functions:
// - AEAD_PREPAREKEY: key preparation
// - AEAD_ENCRYPT and AEAD_DECRYPT: one-shot encryption and decryption
// - AEAD_INIT, AEAD_AUTH_UPDATE, AEAD_ENCRYPT_UPDATE, AEAD_ENCRYPT_FINAL,
// AEAD_DECRYPT_UPDATE, AEAD_DECRYPT_FINAL: functions for incremental
// encryption and decryption
//
// Function prototypes and their behavior must match the AES-CCM API.
//

//
// Allocate a KUnit-managed struct AEAD_KEY and prepare it with a random key,
// using a random key length and random authentication tag length.
//
// tag_len_ret = tag_len;
//
// Allocate a KUnit-managed slab buffer of length @len bytes and initialize it
// with random data.
//
// Allocate a KUnit-managed guarded buffer of length @len bytes and initialize
// it with random data.
//
// Process the given associated data using a random incremental strategy.
// Process the given en/decrypted data using a random incremental strategy.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_incremental_info {
    pub num_data_parts: usize,
    pub num_ad_parts: usize,
}

//
// Encrypt data using a random incremental strategy.
// Return information about the incremental strategy used.
//
// enc= */ true);
//
// Decrypt authentic data using a random incremental strategy.
// Return information about the incremental strategy used.
//
// enc= */ false);
// Return true if key_len is declared to be a valid key length.
// Return true if nonce_len is declared to be a valid nonce length.
// Return true if tag_len is declared to be a valid tag length.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_basic_validation_test_ctx {
    pub key: AEAD_KEY,
    pub ctx: AEAD_CTX,
    pub raw_key_buf_end: *mut u8,
    pub nonce_buf_end: *mut u8,
    pub tag_buf_end: *mut u8,
    pub /: *mut *mut u8 pt[64]; / plaintext,
    pub /: *mut *mut u8 ct[64]; / ciphertext,
    pub decrypted: [u8; 64],
    pub /: *mut *mut u8 ad[16]; / associated data,
    pub unused_buf: *mut u8,
    pub data_len: usize,
    pub ad_len: usize,
}

//
// A pointer to this buffer is passed when passing a length that is
// expected to be invalid.  It should never actually be accessed.
//
// Given an expected-valid key_len, nonce_len, and tag_len, verify round-trip
// encryption and decryption with them.  Use guarded buffers for each of the raw
// key, nonce, and tag to detect any buffer overruns in them.  Also, verify that
// every byte of the tag is actually checked.
//
// Set up exact-size guarded buffers for (raw_key, nonce, tag).
// Key preparation should succeed.
// Encryption should succeed.
// Decryption should succeed and give the original data.
//
// Every byte of the tag should actually be checked.
// And on authentication failure, the dst buffer should be cleared.
//
// Verify that the given expected-invalid key_len is actually rejected.
//
// The preparekey function should reject the key_len.  It should do so
// before writing to the key struct.
//
// Test that every valid key length is accepted and basic checks pass with it,
// and test that invalid key lengths are rejected.
//
// Verify that the given expected-invalid nonce_len is actually rejected.
// Key preparation should succeed, as nonce_len isn't given yet.
// The init function should reject the nonce_len.
// The encrypt function should reject the nonce_len.
// The decrypt function should reject the nonce_len.
//
// Test that every valid nonce length is accepted and basic checks pass with it,
// and test that invalid nonce lengths are rejected.
//
// Verify that the given expected-invalid tag_len is actually rejected.
//
// The preparekey function should reject the tag_len.  It should do so
// before writing to the key struct.
//
// Test that every valid authentication tag length is accepted and basic checks
// pass with it, and test that invalid authentication tag lengths are rejected.
//
// Test that one-shot encryption and decryption are consistent with each other
// and with incremental encryption and decryption.
//
// Select the lengths to test.
// Try one-shot encryption and decryption.
// Try incremental encryption and decryption.
//
// Test using guarded buffers for the plaintext, ciphertext, and associated
// data.  This detects out-of-bounds accesses, even in assembly code.
//
// Note: other test cases cover overrun of raw_key, nonce, and tag.
//
// Select the lengths to test.
// Set up exact-size guarded buffers.
// Encrypt and decrypt.
//
// Test that encryption and decryption produce the same results regardless of
// how the buffers are aligned in memory.
//
// Generate lengths.
// Generate two sets of alignments.
//
// Generate inputs in the first set of buffers using the first
// set of alignments.
//
// Copy the inputs to the second set of buffers using the second
// set of alignments.
//
// Verify encryption consistency.
// Verify decryption consistency.
// Encrypt out-of-place.
// Encrypt in-place.
// Compare the results.
// Decrypt out-of-place.
// Decrypt in-place.
// Compare the results.
//
// Monte-Carlo test for AEAD algorithms.  This deterministically generates
// random AEAD inputs, encrypts them, verifies decryption, and computes and
// verifies the checksum of all computed (ciphertext, tag) pairs.
//
pub const IRQ_TEST_DATA_LEN: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_irq_test_slot {
// Fields written only at test case initialization time
    pub raw_key: [u8; AEAD_MAX_KEY_LEN],
    pub nonce: [u8; AEAD_MAX_NONCE_LEN],
    pub pt: [u8; IRQ_TEST_DATA_LEN],
    pub AEAD_MAX_TAG_LEN]: u8 ct[IRQ_TEST_DATA_LEN +,
    pub ad: [u8; IRQ_TEST_DATA_LEN],
// Fields written throughout the test case
    pub key: AEAD_KEY,
    pub AEAD_MAX_TAG_LEN]: u8 scratch_buf[IRQ_TEST_DATA_LEN +,
    pub phase: c_int,
    pub in_use: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_irq_test_state {
    pub slots: [aead_irq_test_slot; IRQ_TEST_NUM_BUFFERS],
}

//
// Find a free slot.  This should always succeed, since the number of
// slots is equal to the max concurrency level of kunit_run_irq_test().
//
// This execution context now has exclusive access to 'slot'.
// Next, execute the next operation that the slot is set to perform.
//
// Phase 0: Prepare slot's key in current context.
//
// Phase 1: Encrypt plaintext using key that may have been
// prepared in a different context.
//
// Verify the ciphertext (with concatenated auth tag) matches
//
// Phase 2: Decrypt ciphertext using key that may have been
// prepared in a different context.
//
// Verify the plaintext matches.
//
// Test that encryption and decryption produce the correct results in task,
// softirq, and hardirq contexts running concurrently -- including with keys
// prepared in other contexts.  This is needed to cover fallback code paths that
// execute in contexts where FPU or vector registers cannot be used.
//
// For each slot, generate a set of AEAD inputs: a key, a nonce, a
// plaintext, and some associated data.  Then generate the corresponding
// ciphertext with concatenated auth tag.
//
// Benchmark AEAD encryption and decryption on various data lengths.
// Warm-up
// clang-format off
