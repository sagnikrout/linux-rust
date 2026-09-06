//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/spi-nor/sysfs.c
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

    static ssize_t manufacturer_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct spi_mem *spimem = spi_get_drvdata(spi);
    struct spi_nor *nor = spi_mem_get_drvdata(spimem);
    return sysfs_emit(buf, "%s\n", nor.manufacturer.name);
    }
    static DEVICE_ATTR_RO(manufacturer);
    static ssize_t partname_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct spi_mem *spimem = spi_get_drvdata(spi);
    struct spi_nor *nor = spi_mem_get_drvdata(spimem);
    return sysfs_emit(buf, "%s\n", nor.info.name);
    }
    static DEVICE_ATTR_RO(partname);
    static ssize_t jedec_id_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct spi_mem *spimem = spi_get_drvdata(spi);
    struct spi_nor *nor = spi_mem_get_drvdata(spimem);
    const u8 *id = nor.info.id ? nor.info.id.bytes : nor.id;
    let mut id_len: u8 = nor.info.id ? nor.info.id.len : SPI_NOR_MAX_ID_LEN;
    return sysfs_emit(buf, "%*phN\n", id_len, id);
    }
    static DEVICE_ATTR_RO(jedec_id);
    static struct attribute *spi_nor_sysfs_entries[] = {
    &dev_attr_manufacturer.attr,
    &dev_attr_partname.attr,
    &dev_attr_jedec_id.attr,
    core::ptr::null_mut()
    };
    static ssize_t sfdp_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    struct spi_device *spi = to_spi_device(kobj_to_dev(kobj));
    struct spi_mem *spimem = spi_get_drvdata(spi);
    struct spi_nor *nor = spi_mem_get_drvdata(spimem);
    struct sfdp *sfdp = nor.sfdp;
    let mut sfdp_size: usize = sfdp.num_dwords * sizeof(*sfdp.dwords);
    return memory_read_from_buffer(buf, count, &off, nor.sfdp.dwords,
    sfdp_size);
    }
    static const BIN_ATTR_RO(sfdp, 0);
    static const struct bin_attribute *const spi_nor_sysfs_bin_entries[] = {
    &bin_attr_sfdp,
    core::ptr::null_mut()
    };
    static umode_t spi_nor_sysfs_is_visible(struct kobject *kobj,
    struct attribute *attr, int n)
    {
    struct spi_device *spi = to_spi_device(kobj_to_dev(kobj));
    struct spi_mem *spimem = spi_get_drvdata(spi);
    struct spi_nor *nor = spi_mem_get_drvdata(spimem);
    if (attr == &dev_attr_manufacturer.attr && !nor.manufacturer)
    return 0;
    if (attr == &dev_attr_partname.attr && !nor.info.name)
    return 0;
    if (attr == &dev_attr_jedec_id.attr && !nor.info.id && !nor.id)
    return 0;
    return 0444;
    }
    static umode_t spi_nor_sysfs_is_bin_visible(struct kobject *kobj,
    const struct bin_attribute *attr, int n)
    {
    struct spi_device *spi = to_spi_device(kobj_to_dev(kobj));
    struct spi_mem *spimem = spi_get_drvdata(spi);
    struct spi_nor *nor = spi_mem_get_drvdata(spimem);
    if (attr == &bin_attr_sfdp && nor.sfdp)
    return 0444;
    return 0;
    }
    static const struct attribute_group spi_nor_sysfs_group = {
    .name		= "spi-nor",
    .is_visible	= spi_nor_sysfs_is_visible,
    .is_bin_visible	= spi_nor_sysfs_is_bin_visible,
    .attrs		= spi_nor_sysfs_entries,
    .bin_attrs	= spi_nor_sysfs_bin_entries,
    };
    const struct attribute_group *spi_nor_sysfs_groups[] = {
    &spi_nor_sysfs_group,
    core::ptr::null_mut()
    };
