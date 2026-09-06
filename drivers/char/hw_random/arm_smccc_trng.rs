//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/arm_smccc_trng.c
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
// Randomness driver for the ARM SMCCC TRNG Firmware Interface
// https://developer.arm.com/documentation/den0098/latest
//
// Copyright (C) 2020 Arm Ltd.
//
// The ARM TRNG firmware interface specifies a protocol to read entropy
// from a higher exception level, to abstract from any machine specific
// implemenations and allow easier use in hypervisors.
//
// The firmware interface is realised using the SMCCC specification.
//

// We don't want to allow the firmware to stall us forever.
pub const SMCCC_TRNG_MAX_TRIES: c_int = 20;

    static int copy_from_registers(char *buf, struct arm_smccc_res *res,
    size_t bytes)
    {
    unsigned int chunk, copied;
    if (bytes == 0)
    return 0;
    chunk = min(bytes, sizeof(long));
    memcpy(buf, &res.a3, chunk);
    copied = chunk;
    if (copied >= bytes)
    return copied;
    chunk = min((bytes - copied), sizeof(long));
    memcpy(&buf[copied], &res.a2, chunk);
    copied += chunk;
    if (copied >= bytes)
    return copied;
    chunk = min((bytes - copied), sizeof(long));
    memcpy(&buf[copied], &res.a1, chunk);
    return copied + chunk;
    }
#[no_mangle]
unsafe extern "C" fn smccc_trng_read(rng: *mut hwrng, data: *mut c_void, max: usize, wait: bool) -> c_int {
    static int smccc_trng_read(struct hwrng *rng, void *data, size_t max, bool wait)
    {
    struct arm_smccc_res res;
    u8 *buf = data;
    let mut copied: c_uint = 0;
    let mut tries: c_int = 0;
    while (copied < max) {
    size_t bits = min_t(size_t, (max - copied) * BITS_PER_BYTE,
    MAX_BITS_PER_CALL);
    arm_smccc_1_1_invoke(ARM_SMCCC_TRNG_RND, bits, &res);
    switch ((int)res.a0) {
    case SMCCC_RET_SUCCESS:
    copied += copy_from_registers(buf + copied, &res,
    bits / BITS_PER_BYTE);
    tries = 0;
    break;
    case SMCCC_RET_TRNG_NO_ENTROPY:
    if (!wait)
    return copied;
    tries++;
    if (tries >= SMCCC_TRNG_MAX_TRIES)
    return copied;
    cond_resched();
    break;
    default:
    return -EIO;
    }
    }
    return copied;
    }
#[no_mangle]
unsafe extern "C" fn smccc_trng_probe(pdev: *mut platform_device) -> c_int {
    static int smccc_trng_probe(struct platform_device *pdev)
    {
    struct hwrng *trng;
    trng = devm_kzalloc(&pdev.dev, sizeof(*trng), GFP_KERNEL);
    if (!trng)
    return -ENOMEM;
    trng.name = "smccc_trng";
    trng.read = smccc_trng_read;
    return devm_hwrng_register(&pdev.dev, trng);
    }
    static struct platform_driver smccc_trng_driver = {
    .driver = {
    .name		= "smccc_trng",
    },
    .probe		= smccc_trng_probe,
    };
    module_platform_driver(smccc_trng_driver);
    MODULE_ALIAS("platform:smccc_trng");
    MODULE_AUTHOR("Andre Przywara");
    MODULE_DESCRIPTION("Arm SMCCC TRNG firmware interface support");
    MODULE_LICENSE("GPL");
