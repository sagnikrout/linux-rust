//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/parsers/qcomsmempart.c
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
// Qualcomm SMEM NAND flash partition parser
//
// Copyright (C) 2020, Linaro Ltd.
//

pub const SMEM_AARM_PARTITION_TABLE: c_int = 9;
pub const SMEM_APPS: c_int = 0;
pub const SMEM_FLASH_PART_MAGIC1: c_uint = 0x55ee73aa;
pub const SMEM_FLASH_PART_MAGIC2: c_uint = 0xe35ebddb;
pub const SMEM_FLASH_PTABLE_V3: c_int = 3;
pub const SMEM_FLASH_PTABLE_V4: c_int = 4;
pub const SMEM_FLASH_PTABLE_MAX_PARTS_V3: c_int = 16;
pub const SMEM_FLASH_PTABLE_MAX_PARTS_V4: c_int = 48;

pub const SMEM_FLASH_PTABLE_NAME_SIZE: c_int = 16;
//
// struct smem_flash_pentry - SMEM Flash partition entry
// @name: Name of the partition
// @offset: Offset in blocks
// @length: Length of the partition in blocks
// @attr: Flags for this partition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smem_flash_pentry {
    pub name: [c_char; SMEM_FLASH_PTABLE_NAME_SIZE],
    pub offset: __le32,
    pub length: __le32,
    pub attr: u8,
    pub __aligned(4): } __packed,
//
// struct smem_flash_ptable - SMEM Flash partition table
// @magic1: Partition table Magic 1
// @magic2: Partition table Magic 2
// @version: Partition table version
// @numparts: Number of partitions in this ptable
// @pentry: Flash partition entries belonging to this ptable
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smem_flash_ptable {
    pub magic1: __le32,
    pub magic2: __le32,
    pub version: __le32,
    pub numparts: __le32,
    pub pentry: [smem_flash_pentry; SMEM_FLASH_PTABLE_MAX_PARTS_V4],
    pub __aligned(4): } __packed,
    static int parse_qcomsmem_part(struct mtd_info *mtd,
    const struct mtd_partition **pparts,
    struct mtd_part_parser_data *data)
    {
    pub SMEM_FLASH_PTABLE_HDR_LEN: size_t len =,
    pub 0: int ret, i, j, tmpparts, numparts =,
    pub pentry: *mut smem_flash_pentry,
    pub ptable: *mut smem_flash_ptable,
    pub parts: *mut mtd_partition,
    pub c: *mut *mut char name,,
    if (IS_ENABLED(CONFIG_MTD_SPI_NOR_USE_4K_SECTORS)
    && mtd.type == MTD_NORFLASH) {
    pr_err("%s: SMEM partition parser is incompatible with 4K sectors\n",
    pub -EINVAL: return,
    }
    pub SMEM\n"): pr_debug("Parsing partition table info from,
    pub &len): ptable = qcom_smem_get(SMEM_APPS, SMEM_AARM_PARTITION_TABLE,,
    if (IS_ERR(ptable)) {
    if (PTR_ERR(ptable) != -EPROBE_DEFER)
    pub header\n"): pr_err("Error reading partition table,
    pub PTR_ERR(ptable): return,
    }
// Verify ptable magic
    if (le32_to_cpu(ptable.magic1) != SMEM_FLASH_PART_MAGIC1 ||
    le32_to_cpu(ptable.magic2) != SMEM_FLASH_PART_MAGIC2) {
    pub failed\n"): pr_err("Partition table magic verification,
    pub -EINVAL: return,
    }
// Ensure that # of partitions is less than the max we have allocated
    pub le32_to_cpu(ptable->numparts): tmpparts =,
    if (tmpparts > SMEM_FLASH_PTABLE_MAX_PARTS_V4) {
    pub limit\n"): pr_err("Partition numbers exceed the max,
    pub -EINVAL: return,
    }
// Find out length of partition data based on table version
    if (le32_to_cpu(ptable.version) <= SMEM_FLASH_PTABLE_V3) {
    len = SMEM_FLASH_PTABLE_HDR_LEN + SMEM_FLASH_PTABLE_MAX_PARTS_V3 *
    pub smem_flash_pentry): sizeof(struct,
    } else if (le32_to_cpu(ptable.version) == SMEM_FLASH_PTABLE_V4) {
    len = SMEM_FLASH_PTABLE_HDR_LEN + SMEM_FLASH_PTABLE_MAX_PARTS_V4 *
    pub smem_flash_pentry): sizeof(struct,
    } else {
    pub le32_to_cpu(ptable->version)): pr_err("Unknown ptable version (%d)",,
    pub -EINVAL: return,
    }
//
// Now that the partition table header has been parsed, verified
// and the length of the partition table calculated, read the
// complete partition table
//
    pub &len): ptable = qcom_smem_get(SMEM_APPS, SMEM_AARM_PARTITION_TABLE,,
    if (IS_ERR(ptable)) {
    pub table\n"): pr_err("Error reading partition,
    pub PTR_ERR(ptable): return,
    }
    pub {: for (i = 0; i < tmpparts; i++),
    pub &ptable->pentry[i]: pentry =,
    if (pentry.name[0] != '\0')
    }
    pub numparts): *mut *mut parts = kzalloc_objs(parts,,
    if (!parts)
    pub -ENOMEM: return,
    pub {: for (i = 0, j = 0; i < tmpparts; i++),
    pub &ptable->pentry[i]: pentry =,
    if (pentry.name[0] == '\0')
    pub GFP_KERNEL): name = kstrdup(pentry->name,,
    if (!name) {
    pub -ENOMEM: ret =,
    pub out_free_parts: goto,
    }
// Convert name to lower case
    pub c++): *mut *mut for (c = name; c != '\0';,
// c = tolower(*c);
    pub name: parts[j].name =,
    pub mtd->erasesize: *mut *mut parts[j].offset = le32_to_cpu(pentry->offset),
    pub pentry->attr: parts[j].mask_flags =,
    pub mtd->erasesize: *mut *mut parts[j].size = le32_to_cpu(pentry->length),
    pr_debug("%d: %s offs=0x%08x size=0x%08x attr:0x%08x\n",
    i, pentry.name, le32_to_cpu(pentry.offset),
    pub pentry->attr): le32_to_cpu(pentry->length),,
    }
    pr_debug("SMEM partition table found: ver: %d len: %d\n",
    pub tmpparts): le32_to_cpu(ptable->version),,
// pparts = parts;
    pub numparts: return,
    out_free_parts:
    while (--j >= 0)
// pparts = NULL;
    pub ret: return,
    }
    static void parse_qcomsmem_cleanup(const struct mtd_partition *pparts,
    int nr_parts)
    {
    pub i: c_int,
    pub i++): for (i = 0; i < nr_parts;,
    }
    static const struct of_device_id qcomsmem_of_match_table[] = {
    { .compatible = "qcom,smem-part" },
    {},
}

    MODULE_DEVICE_TABLE(of, qcomsmem_of_match_table);
    static struct mtd_part_parser mtd_parser_qcomsmem = {
    .parse_fn = parse_qcomsmem_part,
    .cleanup = parse_qcomsmem_cleanup,
    .name = "qcomsmem",
    .of_match_table = qcomsmem_of_match_table,
    };
    module_mtd_part_parser(mtd_parser_qcomsmem);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>");
    MODULE_DESCRIPTION("Qualcomm SMEM NAND flash partition parser");
