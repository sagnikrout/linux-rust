//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/parsers/brcm_u-boot.c
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
//
// Copyright © 2022 Rafał Miłecki <rafal@milecki.pl>
//

pub const BRCM_U_BOOT_MAX_OFFSET: c_uint = 0x200000;
pub const BRCM_U_BOOT_STEP: c_uint = 0x1000;
pub const BRCM_U_BOOT_MAX_PARTS: c_int = 2;
pub const BRCM_U_BOOT_MAGIC: c_uint = 0x75456e76	/* uEnv */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcm_u_boot_header {
    pub magic: __le32,
    pub length: __le32,
    pub __packed: },
    static const char *names[BRCM_U_BOOT_MAX_PARTS] = {
    "u-boot-env",
    "u-boot-env-backup",
}

    static int brcm_u_boot_parse(struct mtd_info *mtd,
    const struct mtd_partition **pparts,
    struct mtd_part_parser_data *data)
    {
    struct brcm_u_boot_header header;
    struct mtd_partition *parts;
    size_t bytes_read;
    size_t offset;
    int err;
    let mut i: c_int = 0;
    parts = kzalloc_objs(*parts, BRCM_U_BOOT_MAX_PARTS);
    if (!parts)
    return -ENOMEM;
    for (offset = 0;
    offset < min_t(size_t, mtd.size, BRCM_U_BOOT_MAX_OFFSET);
    offset += BRCM_U_BOOT_STEP) {
    err = mtd_read(mtd, offset, sizeof(header), &bytes_read, (uint8_t *)&header);
    if (err && !mtd_is_bitflip(err)) {
    pr_err("Failed to read from %s at 0x%zx: %d\n", mtd.name, offset, err);
    continue;
    }
    if (le32_to_cpu(header.magic) != BRCM_U_BOOT_MAGIC)
    continue;
    parts[i].name = names[i];
    parts[i].offset = offset;
    parts[i].size = sizeof(header) + le32_to_cpu(header.length);
    i++;
    pr_info("offset:0x%zx magic:0x%08x BINGO\n", offset, header.magic);
    if (i == BRCM_U_BOOT_MAX_PARTS)
    break;
    }
// pparts = parts;
    return i;
    };
    static const struct of_device_id brcm_u_boot_of_match_table[] = {
    { .compatible = "brcm,u-boot" },
    {},
    };
    MODULE_DEVICE_TABLE(of, brcm_u_boot_of_match_table);
    static struct mtd_part_parser brcm_u_boot_mtd_parser = {
    .parse_fn = brcm_u_boot_parse,
    .name = "brcm_u-boot",
    .of_match_table = brcm_u_boot_of_match_table,
    };
    module_mtd_part_parser(brcm_u_boot_mtd_parser);
    MODULE_DESCRIPTION("Broadcom's U-Boot partition parser");
    MODULE_LICENSE("GPL");
