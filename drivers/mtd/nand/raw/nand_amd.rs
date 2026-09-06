//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/nand_amd.c
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
// Copyright (C) 2017 Free Electrons
// Copyright (C) 2017 NextThing Co
//
// Author: Boris Brezillon <boris.brezillon@free-electrons.com>
//

#[no_mangle]
unsafe extern "C" fn amd_nand_decode_id(chip: *mut nand_chip) {
    static void amd_nand_decode_id(struct nand_chip *chip)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    struct nand_memory_organization *memorg;
    memorg = nanddev_get_memorg(&chip.base);
    nand_decode_ext_id(chip);
//
// Check for Spansion/AMD ID + repeating 5th, 6th byte since
// some Spansion chips have erasesize that conflicts with size
// listed in nand_ids table.
// Data sheet (5 byte ID): Spansion S30ML-P ORNAND (p.39)
//
    if (chip.id.data[4] != 0x00 && chip.id.data[5] == 0x00 &&
    chip.id.data[6] == 0x00 && chip.id.data[7] == 0x00 &&
    memorg.pagesize == 512) {
    memorg.pages_per_eraseblock = 256;
    memorg.pages_per_eraseblock <<= ((chip.id.data[3] & 0x03) << 1);
    mtd.erasesize = memorg.pages_per_eraseblock *
    memorg.pagesize;
    }
    }
#[no_mangle]
unsafe extern "C" fn amd_nand_init(chip: *mut nand_chip) -> c_int {
    static int amd_nand_init(struct nand_chip *chip)
    {
    if (nand_is_slc(chip))
//
// According to the datasheet of some Cypress SLC NANDs,
// the bad block markers can be in the first, second or last
// page of a block. So let's check all three locations.
//
    chip.options |= NAND_BBM_FIRSTPAGE | NAND_BBM_SECONDPAGE |
    NAND_BBM_LASTPAGE;
    return 0;
    }
    const struct nand_manufacturer_ops amd_nand_manuf_ops = {
    .detect = amd_nand_decode_id,
    .init = amd_nand_init,
    };
