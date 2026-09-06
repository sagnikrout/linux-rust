//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/parsers/bcm63xxpart.c
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
// BCM63XX CFE image tag parser
//
// Copyright © 2006-2008  Florian Fainelli <florian@openwrt.org>
// Mike Albon <malbon@openwrt.org>
// Copyright © 2009-2010  Daniel Dickinson <openwrt@cshore.neomailbox.net>
// Copyright © 2011-2013  Jonas Gorski <jonas.gorski@gmail.com>
//

pub const BCM963XX_CFE_MAGIC_OFFSET: c_uint = 0x4e0;
pub const BCM963XX_CFE_VERSION_OFFSET: c_uint = 0x570;
pub const BCM963XX_NVRAM_OFFSET: c_uint = 0x580;
// Ensure strings read from flash structs are null terminated

    do { char *_str = (x); _str[sizeof(x) - 1] = 0; } while (0)
#[no_mangle]
pub unsafe extern "C" fn bcm63xx_detect_cfe() -> c_int {
    static inline int bcm63xx_detect_cfe(void)
    {
    let mut ret: c_int = 0;

    ret = (fw_arg3 == CFE_EPTSEAL);

    return ret;
    }
    static int bcm63xx_read_nvram(struct mtd_info *master,
    struct bcm963xx_nvram *nvram)
    {
    u32 actual_crc, expected_crc;
    size_t retlen;
    int ret;
// extract nvram data
    ret = mtd_read(master, BCM963XX_NVRAM_OFFSET, BCM963XX_NVRAM_V5_SIZE,
    &retlen, (void *)nvram);
    if (ret)
    return ret;
    ret = bcm963xx_nvram_checksum(nvram, &expected_crc, &actual_crc);
    if (ret)
    pr_warn("nvram checksum failed, contents may be invalid (expected %08x, got %08x)\n",
    expected_crc, actual_crc);
    if (!nvram.psi_size)
    nvram.psi_size = BCM963XX_DEFAULT_PSI_SIZE;
    return 0;
    }
    static const char * const bcm63xx_cfe_part_types[] = {
    "bcm963xx-imagetag",
    core::ptr::null_mut(),
    };
    static int bcm63xx_parse_cfe_nor_partitions(struct mtd_info *master,
    const struct mtd_partition **pparts, struct bcm963xx_nvram *nvram)
    {
    struct mtd_partition *parts;
    let mut nrparts: c_int = 3, curpart = 0;
    unsigned int cfelen, nvramlen;
    unsigned int cfe_erasesize;
    int i;
    cfe_erasesize = max_t(uint32_t, master.erasesize,
    BCM963XX_CFE_BLOCK_SIZE);
    cfelen = cfe_erasesize;
    nvramlen = nvram.psi_size * SZ_1K;
    nvramlen = roundup(nvramlen, cfe_erasesize);
    parts = kzalloc(sizeof(*parts) * nrparts + 10 * nrparts, GFP_KERNEL);
    if (!parts)
    return -ENOMEM;
// Start building partition list
    parts[curpart].name = "CFE";
    parts[curpart].offset = 0;
    parts[curpart].size = cfelen;
    curpart++;
    parts[curpart].name = "nvram";
    parts[curpart].offset = master.size - nvramlen;
    parts[curpart].size = nvramlen;
    curpart++;
// Global partition "linux" to make easy firmware upgrade
    parts[curpart].name = "linux";
    parts[curpart].offset = cfelen;
    parts[curpart].size = master.size - cfelen - nvramlen;
    parts[curpart].types = bcm63xx_cfe_part_types;
    for (i = 0; i < nrparts; i++)
    pr_info("Partition %d is %s offset %llx and length %llx\n", i,
    parts[i].name, parts[i].offset,	parts[i].size);
// pparts = parts;
    return nrparts;
    }
    static int bcm63xx_parse_cfe_partitions(struct mtd_info *master,
    const struct mtd_partition **pparts,
    struct mtd_part_parser_data *data)
    {
    struct bcm963xx_nvram *nvram = core::ptr::null_mut();
    int ret;
    if (!bcm63xx_detect_cfe())
    return -EINVAL;
    nvram = vzalloc(sizeof(*nvram));
    if (!nvram)
    return -ENOMEM;
    ret = bcm63xx_read_nvram(master, nvram);
    if (ret)
    goto out;
    if (!mtd_type_is_nand(master))
    ret = bcm63xx_parse_cfe_nor_partitions(master, pparts, nvram);
    else
    ret = -EINVAL;
    out:
    vfree(nvram);
    return ret;
    };
    static const struct of_device_id parse_bcm63xx_cfe_match_table[] = {
    { .compatible = "brcm,bcm963xx-cfe-nor-partitions" },
    {},
    };
    MODULE_DEVICE_TABLE(of, parse_bcm63xx_cfe_match_table);
    static struct mtd_part_parser bcm63xx_cfe_parser = {
    .parse_fn = bcm63xx_parse_cfe_partitions,
    .name = "bcm63xxpart",
    .of_match_table = parse_bcm63xx_cfe_match_table,
    };
    module_mtd_part_parser(bcm63xx_cfe_parser);
    MODULE_AUTHOR("Daniel Dickinson <openwrt@cshore.neomailbox.net>");
    MODULE_AUTHOR("Florian Fainelli <florian@openwrt.org>");
    MODULE_AUTHOR("Mike Albon <malbon@openwrt.org>");
    MODULE_AUTHOR("Jonas Gorski <jonas.gorski@gmail.com");
    MODULE_DESCRIPTION("MTD partitioning for BCM63XX CFE bootloaders");
