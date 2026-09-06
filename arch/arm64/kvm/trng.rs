//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/trng.c
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
// Copyright (C) 2020 Arm Ltd.

pub const ARM_SMCCC_TRNG_VERSION_1_0: c_uint = 0x10000UL;
// Those values are deliberately separate from the generic SMCCC definitions.

pub const TRNG_MAX_BITS64: c_int = 192;
    static const uuid_t arm_smc_trng_uuid __aligned(4) = UUID_INIT(
    0x0d21e000, 0x4384, 0x11eb, 0x80, 0x70, 0x52, 0x44, 0x55, 0x4e, 0x5a, 0x4c);
#[no_mangle]
unsafe extern "C" fn kvm_trng_do_rnd(vcpu: *mut kvm_vcpu, size: c_int) -> c_int {
    static int kvm_trng_do_rnd(struct kvm_vcpu *vcpu, int size)
    {
    DECLARE_BITMAP(bits, TRNG_MAX_BITS64);
    let mut num_bits: u32 = smccc_get_arg1(vcpu);
    int i;
    if (num_bits > 3 * size) {
    smccc_set_retval(vcpu, TRNG_INVALID_PARAMETER, 0, 0, 0);
    return 1;
    }
// get as many bits as we need to fulfil the request
    for (i = 0; i < DIV_ROUND_UP(num_bits, BITS_PER_LONG); i++)
    bits[i] = get_random_long();
    bitmap_clear(bits, num_bits, TRNG_MAX_BITS64 - num_bits);
    if (size == 32)
    smccc_set_retval(vcpu, TRNG_SUCCESS, lower_32_bits(bits[1]),
    upper_32_bits(bits[0]), lower_32_bits(bits[0]));
    else
    smccc_set_retval(vcpu, TRNG_SUCCESS, bits[2], bits[1], bits[0]);
    memzero_explicit(bits, sizeof(bits));
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_trng_call(vcpu: *mut kvm_vcpu) -> c_int {
    int kvm_trng_call(struct kvm_vcpu *vcpu)
    {
    const __le32 *u = (__le32 *)arm_smc_trng_uuid.b;
    let mut func_id: u32 = smccc_get_function(vcpu);
    let mut val: c_ulong = TRNG_NOT_SUPPORTED;
    let mut size: c_int = 64;
    switch (func_id) {
    case ARM_SMCCC_TRNG_VERSION:
    val = ARM_SMCCC_TRNG_VERSION_1_0;
    break;
    case ARM_SMCCC_TRNG_FEATURES:
    switch (smccc_get_arg1(vcpu)) {
    case ARM_SMCCC_TRNG_VERSION:
    case ARM_SMCCC_TRNG_FEATURES:
    case ARM_SMCCC_TRNG_GET_UUID:
    case ARM_SMCCC_TRNG_RND32:
    case ARM_SMCCC_TRNG_RND64:
    val = TRNG_SUCCESS;
    }
    break;
    case ARM_SMCCC_TRNG_GET_UUID:
    smccc_set_retval(vcpu, le32_to_cpu(u[0]), le32_to_cpu(u[1]),
    le32_to_cpu(u[2]), le32_to_cpu(u[3]));
    return 1;
    case ARM_SMCCC_TRNG_RND32:
    size = 32;
    fallthrough;
    case ARM_SMCCC_TRNG_RND64:
    return kvm_trng_do_rnd(vcpu, size);
    }
    smccc_set_retval(vcpu, val, 0, 0, 0);
    return 1;
    }
