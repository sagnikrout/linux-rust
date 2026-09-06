//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/parsers/tplink_safeloader.c
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

pub const TPLINK_SAFELOADER_DATA_OFFSET: c_int = 4;
pub const TPLINK_SAFELOADER_MAX_PARTS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct safeloader_cmn_header {
    pub size: __be32,
    pub unused: u32,
    pub __packed: },
    static void *mtd_parser_tplink_safeloader_read_table(struct mtd_info *mtd)
    {
    pub hdr: safeloader_cmn_header,
    pub np: *mut device_node,
    pub bytes_read: usize,
    pub size: usize,
    pub offset: u32,
    pub buf: *mut c_char,
    pub err: c_int,
    pub mtd_get_of_node(mtd): np =,
    if (mtd_is_partition(mtd))
    else
    pub "partitions"): np = of_get_child_by_name(np,,
    if (of_property_read_u32(np, "partitions-table-offset", &offset)) {
    pub offset\n"): pr_err("Failed to get partitions table,
    pub err_put: goto,
    }
    pub )&hdr): *mut err = mtd_read(mtd, offset, sizeof(hdr), &bytes_read, (uint8_t,
    if (err && !mtd_is_bitflip(err)) {
    pub offset): pr_err("Failed to read from %s at 0x%x\n", mtd->name,,
    pub err_put: goto,
    }
    pub be32_to_cpu(hdr.size): size =,
    pub GFP_KERNEL): buf = kmalloc(size + 1,,
    if (!buf)
    pub err_put: goto,
    pub buf): err = mtd_read(mtd, offset + sizeof(hdr), size, &bytes_read,,
    if (err && !mtd_is_bitflip(err)) {
    pub sizeof(hdr)): pr_err("Failed to read from %s at 0x%zx\n", mtd->name, offset +,
    pub err_kfree: goto,
    }
    pub '\0': buf[size] =,
    pub buf: return,
    err_kfree:
    err_put:
    pub NULL: return,
    }
    static int mtd_parser_tplink_safeloader_parse(struct mtd_info *mtd,
    const struct mtd_partition **pparts,
    struct mtd_part_parser_data *data)
    {
    pub parts: *mut mtd_partition,
    pub name: [c_char; 65],
    pub offset: usize,
    pub bytes: usize,
    pub buf: *mut c_char,
    pub idx: c_int,
    pub err: c_int,
    pub TPLINK_SAFELOADER_MAX_PARTS): *mut *mut parts = kzalloc_objs(parts,,
    if (!parts) {
    pub -ENOMEM: err =,
    pub err_out: goto,
    }
    pub mtd_parser_tplink_safeloader_read_table(mtd): buf =,
    if (!buf) {
    pub -ENOENT: err =,
    pub err_free_parts: goto,
    }
    pub TPLINK_SAFELOADER_DATA_OFFSET: for (idx = 0, offset =,
    idx < TPLINK_SAFELOADER_MAX_PARTS &&
    sscanf(buf + offset, "partition %64s base 0x%llx size 0x%llx%zn\n",
    pub 3: name, &parts[idx].offset, &parts[idx].size, &bytes) ==,
    idx++, offset += bytes + 1) {
    pub GFP_KERNEL): parts[idx].name = kstrdup(name,,
    if (!parts[idx].name) {
    pub -ENOMEM: err =,
    pub err_free: goto,
    }
    }
    if (idx == TPLINK_SAFELOADER_MAX_PARTS)
    pub partitions!\n"): pr_warn("Reached maximum number of,
// pparts = parts;
    pub idx: return,
    err_free:
    pub idx--): for (idx -= 1; idx >= 0;,
    err_free_parts:
    err_out:
    pub err: return,
}

    static void mtd_parser_tplink_safeloader_cleanup(const struct mtd_partition *pparts,
    int nr_parts)
    {
    int i;
    for (i = 0; i < nr_parts; i++)
    kfree(pparts[i].name);
    kfree(pparts);
    }
    static const struct of_device_id mtd_parser_tplink_safeloader_of_match_table[] = {
    { .compatible = "tplink,safeloader-partitions" },
    {},
    };
    MODULE_DEVICE_TABLE(of, mtd_parser_tplink_safeloader_of_match_table);
    static struct mtd_part_parser mtd_parser_tplink_safeloader = {
    .parse_fn = mtd_parser_tplink_safeloader_parse,
    .cleanup = mtd_parser_tplink_safeloader_cleanup,
    .name = "tplink-safeloader",
    .of_match_table = mtd_parser_tplink_safeloader_of_match_table,
    };
    module_mtd_part_parser(mtd_parser_tplink_safeloader);
    MODULE_DESCRIPTION("TP-Link Safeloader partitions parser");
    MODULE_LICENSE("GPL");
