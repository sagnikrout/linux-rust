//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/mem_encrypt.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm64_mem_crypt_ops {
    pub numpages): *mut *mut int (encrypt)(unsigned long addr, int,
    pub numpages): *mut *mut int (decrypt)(unsigned long addr, int,
}

extern "C" {
    pub fn arm64_mem_crypt_ops_register(ops: *const arm64_mem_crypt_ops) -> c_int;
}
extern "C" {
    pub fn set_memory_encrypted(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn set_memory_decrypted(addr: c_ulong, numpages: c_int) -> c_int;
}
extern "C" {
    pub fn realm_register_memory_enc_ops() -> c_int;
}
extern "C" {
    pub fn is_realm_world(is_protected_kvm_guest(: ) ||) -> return;
}
//
// For Arm CCA guests, canonical addresses are "encrypted", so no changes
// required for dma_addr_encrypted().
// The unencrypted DMA buffers must be accessed via the unprotected IPA,
// "top IPA bit" set.
//

// Clear the "top" IPA bit while converting back

