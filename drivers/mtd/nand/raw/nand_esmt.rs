//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/nand_esmt.c
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
// Copyright (C) 2018 Toradex AG
//
// Author: Marcel Ziswiler <marcel.ziswiler@toradex.com>
//

#[no_mangle]
unsafe extern "C" fn esmt_nand_decode_id(chip: *mut nand_chip) {
    static void esmt_nand_decode_id(struct nand_chip *chip)
    {
    struct nand_device *base = &chip.base;
    let mut requirements: nand_ecc_props = {};
    nand_decode_ext_id(chip);
// Extract ECC requirements from 5th id byte.
    if (chip.id.len >= 5 && nand_is_slc(chip)) {
    requirements.step_size = 512;
    switch (chip.id.data[4] & 0x3) {
    case 0x0:
    requirements.strength = 4;
    break;
    case 0x1:
    requirements.strength = 2;
    break;
    case 0x2:
    requirements.strength = 1;
    break;
    default:
    WARN(1, "Could not get ECC info");
    requirements.step_size = 0;
    break;
    }
    }
    nanddev_set_ecc_requirements(base, &requirements);
    }
#[no_mangle]
unsafe extern "C" fn esmt_nand_init(chip: *mut nand_chip) -> c_int {
    static int esmt_nand_init(struct nand_chip *chip)
    {
    if (nand_is_slc(chip))
//
// It is known that some ESMT SLC NANDs have been shipped
// with the factory bad block markers in the first or last page
// of the block, instead of the first or second page. To be on
// the safe side, let's check all three locations.
//
    chip.options |= NAND_BBM_FIRSTPAGE | NAND_BBM_SECONDPAGE |
    NAND_BBM_LASTPAGE;
    return 0;
    }
    const struct nand_manufacturer_ops esmt_nand_manuf_ops = {
    .detect = esmt_nand_decode_id,
    .init = esmt_nand_init,
    };
