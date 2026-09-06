//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/nand_sandisk.c
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

    static int
    sdtnqgama_choose_interface_config(struct nand_chip *chip,
    struct nand_interface_config *iface)
    {
    onfi_fill_interface_config(chip, iface, NAND_SDR_IFACE, 0);
    return nand_choose_best_sdr_timings(chip, iface, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn sandisk_nand_init(chip: *mut nand_chip) -> c_int {
    static int sandisk_nand_init(struct nand_chip *chip)
    {
    if (!strncmp("SDTNQGAMA", chip.parameters.model,
    sizeof("SDTNQGAMA") - 1))
    chip.ops.choose_interface_config =
    &sdtnqgama_choose_interface_config;
    return 0;
    }
    const struct nand_manufacturer_ops sandisk_nand_manuf_ops = {
    .init = sandisk_nand_init,
    };
