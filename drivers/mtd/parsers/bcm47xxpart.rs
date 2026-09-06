//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/parsers/bcm47xxpart.c
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
// BCM47XX MTD partitioning
//
// Copyright © 2012 Rafał Miłecki <zajec5@gmail.com>
//

//
// NAND flash on Netgear R6250 was verified to contain 15 partitions.
// This will result in allocating too big array for some old devices, but the
// memory will be freed soon anyway (see mtd_device_parse_register).
//
pub const BCM47XXPART_MAX_PARTS: c_int = 20;
//
// Amount of bytes we read when analyzing each block of flash memory.
// Set it big enough to allow detecting partition and reading important data.
//
pub const BCM47XXPART_BYTES_TO_READ: c_uint = 0x4e8;
// Magics
pub const BOARD_DATA_MAGIC: c_uint = 0x5246504D	/* MPFR */;
pub const BOARD_DATA_MAGIC2: c_uint = 0xBD0D0BBD;
pub const CFE_MAGIC: c_uint = 0x43464531	/* 1EFC */;
pub const FACTORY_MAGIC: c_uint = 0x59544346	/* FCTY */;
pub const NVRAM_HEADER: c_uint = 0x48534C46	/* FLSH */;
pub const POT_MAGIC1: c_uint = 0x54544f50	/* POTT */;
pub const POT_MAGIC2: c_uint = 0x504f		/* OP */;
pub const ML_MAGIC1: c_uint = 0x39685a42;
pub const ML_MAGIC2: c_uint = 0x26594131;
pub const TRX_MAGIC: c_uint = 0x30524448;
pub const SHSQ_MAGIC: c_uint = 0x71736873	/* shsq (weird ZTE H218N endianness) */;
    static const char * const trx_types[] = { "trx", core::ptr::null_mut() };
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
    static void bcm47xxpart_add_part(struct mtd_partition *part, const char *name,
    u64 offset, uint32_t mask_flags)
    {
    pub name: part->name =,
    pub offset: part->offset =,
    pub mask_flags: part->mask_flags =,
    }
//
// bcm47xxpart_bootpartition - gets index of TRX partition used by bootloader
//
// Some devices may have more than one TRX partition. In such case one of them
// is the main one and another a failsafe one. Bootloader may fallback to the
// failsafe firmware if it detects corruption of the main image.
//
// This function provides info about currently used TRX partition. It's the one
// containing kernel started by the bootloader.
//
#[no_mangle]
unsafe extern "C" fn bcm47xxpart_bootpartition() -> c_int {
    static int bcm47xxpart_bootpartition(void)
    {
    pub buf: [c_char; 4],
    pub bootpartition: c_int,
// Check CFE environment variable
    if (bcm47xx_nvram_getenv("bootpartition", buf, sizeof(buf)) > 0) {
    if (!kstrtoint(buf, 0, &bootpartition))
    pub bootpartition: return,
    }
    pub 0: return,
    }
    static int bcm47xxpart_parse(struct mtd_info *master,
    const struct mtd_partition **pparts,
    struct mtd_part_parser_data *data)
    {
    pub parts: *mut mtd_partition,
    pub 0: uint8_t i, curr_part =,
    pub buf: *mut u32,
    pub bytes_read: usize,
    pub offset: u32,
    pub master->erasesize: uint32_t blocksize =,
    pub /: *mut *mut int trx_parts[2]; / Array with indexes of TRX partitions,
    pub /: *mut *mut int trx_num = 0; / Number of found TRX partitions,
    pub }: static int possible_nvram_sizes[] = { 0x8000, 0xF000, 0x10000,,
    pub err: c_int,
//
// Some really old flashes (like AT45DB*) had smaller erasesize-s, but
// partitions were aligned to at least 0x1000 anyway.
//
    if (blocksize < 0x1000)
    pub 0x1000: blocksize =,
// Alloc
    pub BCM47XXPART_MAX_PARTS): parts = kzalloc_objs(struct mtd_partition,,
    if (!parts)
    pub -ENOMEM: return,
    pub GFP_KERNEL): buf = kzalloc(BCM47XXPART_BYTES_TO_READ,,
    if (!buf) {
    pub -ENOMEM: return,
    }
// Parse block by block looking for magics
    pub blocksize: for (offset = 0; offset <= master->size -,
    offset += blocksize) {
// Nothing more in higher memory on BCM47XX (MIPS)
    if (IS_ENABLED(CONFIG_BCM47XX) && offset >= 0x2000000)
    if (curr_part >= BCM47XXPART_MAX_PARTS) {
    pub stopped!\n"): pr_warn("Reached maximum number of partitions, scanning,
    }
// Read beginning of the block
    err = mtd_read(master, offset, BCM47XXPART_BYTES_TO_READ,
    pub )buf): *mut &bytes_read, (uint8_t,
    if (err && !mtd_is_bitflip(err)) {
    pr_err("mtd_read error while parsing (offset: 0x%X): %d\n",
    pub err): offset,,
    }
// Magic or small NVRAM at 0x400
    if ((buf[0x4e0 / 4] == CFE_MAGIC && buf[0x4e4 / 4] == CFE_MAGIC) ||
    (buf[0x400 / 4] == NVRAM_HEADER)) {
    bcm47xxpart_add_part(&parts[curr_part++], "boot",
    pub MTD_WRITEABLE): offset,,
    }
//
// board_data starts with board_id which differs across boards,
// but we can use 'MPFR' (hopefully) magic at 0x100
//
    if (buf[0x100 / 4] == BOARD_DATA_MAGIC) {
    bcm47xxpart_add_part(&parts[curr_part++], "board_data",
    pub MTD_WRITEABLE): offset,,
    }
// Found on Huawei E970
    if (buf[0x000 / 4] == FACTORY_MAGIC) {
    bcm47xxpart_add_part(&parts[curr_part++], "factory",
    pub MTD_WRITEABLE): offset,,
    }
// POT(TOP)
    if (buf[0x000 / 4] == POT_MAGIC1 &&
    (buf[0x004 / 4] & 0xFFFF) == POT_MAGIC2) {
    bcm47xxpart_add_part(&parts[curr_part++], "POT", offset,
    }
// ML
    if (buf[0x010 / 4] == ML_MAGIC1 &&
    buf[0x014 / 4] == ML_MAGIC2) {
    bcm47xxpart_add_part(&parts[curr_part++], "ML", offset,
    }
// TRX
    if (buf[0x000 / 4] == TRX_MAGIC) {
    pub trx: *mut trx_header,
    pub last_subpart: u32,
    pub trx_size: u32,
    if (trx_num >= ARRAY_SIZE(trx_parts))
    pr_warn("No enough space to store another TRX found at 0x%X\n",
    else
    pub curr_part: trx_parts[trx_num++] =,
    bcm47xxpart_add_part(&parts[curr_part++], "firmware",
    pub 0): offset,,
//
// Try to find TRX size. The "length" field isn't fully
// reliable as it could be decreased to make CRC32 cover
// only part of TRX data. It's commonly used as checksum
// can't cover e.g. ever-changing rootfs partition.
// Use offsets as helpers for assuming min TRX size.
//
    pub )buf: *mut trx = (struct trx_header,
    last_subpart = max3(trx.offset[0], trx.offset[1],
    pub blocksize): trx_size = max(trx->length, last_subpart +,
//
// Skip the TRX data. Decrease offset by block size as
// the next loop iteration will increase it.
//
    pub blocksize: offset += roundup(trx_size, blocksize) -,
    }
// Squashfs on devices not using TRX
    if (le32_to_cpu(buf[0x000 / 4]) == SQUASHFS_MAGIC ||
    buf[0x000 / 4] == SHSQ_MAGIC) {
    bcm47xxpart_add_part(&parts[curr_part++], "rootfs",
    pub 0): offset,,
    }
//
// New (ARM?) devices may have NVRAM in some middle block. Last
// block will be checked later, so skip it.
//
    if (offset != master.size - blocksize &&
    buf[0x000 / 4] == NVRAM_HEADER) {
    bcm47xxpart_add_part(&parts[curr_part++], "nvram",
    pub 0): offset,,
    }
// Read middle of the block
    err = mtd_read(master, offset + (blocksize / 2), 0x4, &bytes_read,
    pub )buf): *mut (uint8_t,
    if (err && !mtd_is_bitflip(err)) {
    pr_err("mtd_read error while parsing (offset: 0x%X): %d\n",
    pub err): offset + (blocksize / 2),,
    }
// Some devices (ex. WNDR3700v3) don't have a standard 'MPFR'
    if (buf[0x000 / 4] == BOARD_DATA_MAGIC2) {
    bcm47xxpart_add_part(&parts[curr_part++], "board_data",
    pub MTD_WRITEABLE): offset,,
    }
    }
// Look for NVRAM at the end of the last block.
    pub {: for (i = 0; i < ARRAY_SIZE(possible_nvram_sizes); i++),
    if (curr_part >= BCM47XXPART_MAX_PARTS) {
    pub stopped!\n"): pr_warn("Reached maximum number of partitions, scanning,
    }
    pub possible_nvram_sizes: [offset = master->size -; i],
    err = mtd_read(master, offset, 0x4, &bytes_read,
    pub )buf): *mut (uint8_t,
    if (err && !mtd_is_bitflip(err)) {
    pr_err("mtd_read error while reading (offset 0x%X): %d\n",
    pub err): offset,,
    }
// Standard NVRAM
    if (buf[0] == NVRAM_HEADER) {
    bcm47xxpart_add_part(&parts[curr_part++], "nvram",
    pub 0): master->size - blocksize,,
    }
    }
//
// Assume that partitions end at the beginning of the one they are
// followed by.
//
    pub {: for (i = 0; i < curr_part; i++),
    u64 next_part_offset = (i < curr_part - 1) ?
    pub master->size: parts[i + 1].offset :,
    pub parts[i].offset: parts[i].size = next_part_offset -,
    }
// If there was TRX parse it now
    pub {: for (i = 0; i < trx_num; i++),
    pub &parts[trx_parts[i]]: *mut *mut mtd_partition trx =,
    if (i == bcm47xxpart_bootpartition())
    pub trx_types: trx->types =,
    else
    pub "failsafe": trx->name =,
    }
// pparts = parts;
    pub curr_part: return,
}

    static const struct of_device_id bcm47xxpart_of_match_table[] = {
    { .compatible = "brcm,bcm947xx-cfe-partitions" },
    {},
    };
    MODULE_DEVICE_TABLE(of, bcm47xxpart_of_match_table);
    static struct mtd_part_parser bcm47xxpart_mtd_parser = {
    .parse_fn = bcm47xxpart_parse,
    .name = "bcm47xxpart",
    .of_match_table = bcm47xxpart_of_match_table,
    };
    module_mtd_part_parser(bcm47xxpart_mtd_parser);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("MTD partitioning for BCM47XX flash memories");
