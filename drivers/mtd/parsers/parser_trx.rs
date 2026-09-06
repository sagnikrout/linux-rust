//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/parsers/parser_trx.c
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
// Parser for TRX format partitions
//
// Copyright (C) 2012 - 2017 Rafał Miłecki <rafal@milecki.pl>
//

pub const TRX_PARSER_MAX_PARTS: c_int = 4;
// Magics
pub const TRX_MAGIC: c_uint = 0x30524448;
pub const UBI_EC_MAGIC: c_uint = 0x23494255	/* UBI# */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trx_header {
    pub magic: u32,
    pub length: u32,
    pub crc32: u32,
    pub flags: u16,
    pub version: u16,
    pub offset: [u32; 3],
    pub __packed: },
    static const char *parser_trx_data_part_name(struct mtd_info *master,
    size_t offset)
    {
    pub buf: u32,
    pub bytes_read: usize,
    pub err: c_int,
    err  = mtd_read(master, offset, sizeof(buf), &bytes_read,
    pub )&buf): *mut (uint8_t,
    if (err && !mtd_is_bitflip(err)) {
    pr_err("mtd_read error while parsing (offset: 0x%zX): %d\n",
    pub err): offset,,
    pub out_default: goto,
    }
    if (buf == UBI_EC_MAGIC)
    pub "ubi": return,
    out_default:
    pub "rootfs": return,
    }
    static int parser_trx_parse(struct mtd_info *mtd,
    const struct mtd_partition **pparts,
    struct mtd_part_parser_data *data)
    {
    pub mtd_get_of_node(mtd): *mut *mut device_node np =,
    pub parts: *mut mtd_partition,
    pub part: *mut mtd_partition,
    pub trx: trx_header,
    pub bytes_read: usize,
    pub 0: uint8_t curr_part = 0, i =,
    pub TRX_MAGIC: uint32_t trx_magic =,
    pub err: c_int,
// Get different magic from device tree if specified
    pub &trx_magic): err = of_property_read_u32(np, "brcm,trx-magic",,
    if (err != 0 && err != -EINVAL)
    pub err): pr_err("failed to parse \"brcm,trx-magic\" DT attribute, using default: %d\n",,
    pub TRX_PARSER_MAX_PARTS): parts = kzalloc_objs(struct mtd_partition,,
    if (!parts)
    pub -ENOMEM: return,
    pub )&trx): *mut err = mtd_read(mtd, 0, sizeof(trx), &bytes_read, (uint8_t,
    if (err) {
    pub err): pr_err("MTD reading error: %d\n",,
    pub err: return,
    }
    if (trx.magic != trx_magic) {
    pub -ENOENT: return,
    }
// We have LZMA loader if there is address in offset[2]
    if (trx.offset[2]) {
    pub &parts[curr_part++]: part =,
    pub "loader": part->name =,
    pub trx.offset[i]: part->offset =,
    }
    if (trx.offset[i]) {
    pub &parts[curr_part++]: part =,
    pub "linux": part->name =,
    pub trx.offset[i]: part->offset =,
    }
    if (trx.offset[i]) {
    pub &parts[curr_part++]: part =,
    pub trx.offset[i]): part->name = parser_trx_data_part_name(mtd,,
    pub trx.offset[i]: part->offset =,
    }
//
// Assume that every partition ends at the beginning of the one it is
// followed by.
//
    pub {: for (i = 0; i < curr_part; i++),
    u64 next_part_offset = (i < curr_part - 1) ?
    pub mtd->size: parts[i + 1].offset :,
    pub parts[i].offset: parts[i].size = next_part_offset -,
    }
// pparts = parts;
    pub i: return,
}

    static const struct of_device_id mtd_parser_trx_of_match_table[] = {
    { .compatible = "brcm,trx" },
    {},
    };
    MODULE_DEVICE_TABLE(of, mtd_parser_trx_of_match_table);
    static struct mtd_part_parser mtd_parser_trx = {
    .parse_fn = parser_trx_parse,
    .name = "trx",
    .of_match_table = mtd_parser_trx_of_match_table,
    };
    module_mtd_part_parser(mtd_parser_trx);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Parser for TRX format partitions");
