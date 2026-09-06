//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/lantiq-flash.c
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
// Copyright (C) 2004 Liu Peng Infineon IFAP DC COM CPE
// Copyright (C) 2010 John Crispin <john@phrozen.org>
//

//
// The NOR flash is connected to the same external bus unit (EBU) as PCI.
// To make PCI work we need to enable the endianness swapping for the address
// written to the EBU. This endianness swapping works for PCI correctly but
// fails for attached NOR devices. To workaround this we need to use a complex
// map. The workaround involves swapping all addresses whilst probing the chip.
// Once probing is complete we stop swapping the addresses but swizzle the
// unlock addresses to ensure that access to the NOR device works correctly.
//
    enum {
    LTQ_NOR_PROBING,
    LTQ_NOR_NORMAL
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltq_mtd {
    pub res: *mut resource,
    pub mtd: *mut mtd_info,
    pub map: *mut map_info,
}

    static const char ltq_map_name[] = "ltq_nor";
    static map_word
    ltq_read16(struct map_info *map, unsigned long adr)
    {
    unsigned long flags;
    map_word temp;
    if (map.map_priv_1 == LTQ_NOR_PROBING)
    adr ^= 2;
    spin_lock_irqsave(&ebu_lock, flags);
    temp.x[0] = *(u16 *)(map.virt + adr);
    spin_unlock_irqrestore(&ebu_lock, flags);
    return temp;
    }
    static void
    ltq_write16(struct map_info *map, map_word d, unsigned long adr)
    {
    unsigned long flags;
    if (map.map_priv_1 == LTQ_NOR_PROBING)
    adr ^= 2;
    spin_lock_irqsave(&ebu_lock, flags);
// (u16 *)(map->virt + adr) = d.x[0];
    spin_unlock_irqrestore(&ebu_lock, flags);
    }
//
// The following 2 functions copy data between iomem and a cached memory
// section. As memcpy() makes use of pre-fetching we cannot use it here.
// The normal alternative of using memcpy_{to,from}io also makes use of
// memcpy() on MIPS so it is not applicable either. We are therefore stuck
// with having to use our own loop.
//
    static void
    ltq_copy_from(struct map_info *map, void *to,
    unsigned long from, ssize_t len)
    {
    unsigned char *f = (unsigned char *)map.virt + from;
    unsigned char *t = (unsigned char *)to;
    unsigned long flags;
    spin_lock_irqsave(&ebu_lock, flags);
    while (len--)
// t++ = *f++;
    spin_unlock_irqrestore(&ebu_lock, flags);
    }
    static void
    ltq_copy_to(struct map_info *map, unsigned long to,
    const void *from, ssize_t len)
    {
    unsigned char *f = (unsigned char *)from;
    unsigned char *t = (unsigned char *)map.virt + to;
    unsigned long flags;
    spin_lock_irqsave(&ebu_lock, flags);
    while (len--)
// t++ = *f++;
    spin_unlock_irqrestore(&ebu_lock, flags);
    }
    static int
    ltq_mtd_probe(struct platform_device *pdev)
    {
    struct ltq_mtd *ltq_mtd;
    struct cfi_private *cfi;
    int err;
    ltq_mtd = devm_kzalloc(&pdev.dev, sizeof(struct ltq_mtd), GFP_KERNEL);
    if (!ltq_mtd)
    return -ENOMEM;
    platform_set_drvdata(pdev, ltq_mtd);
    ltq_mtd.map.virt = devm_platform_get_and_ioremap_resource(pdev, 0, &ltq_mtd.res);
    if (IS_ERR(ltq_mtd.map.virt))
    return PTR_ERR(ltq_mtd.map.virt);
    ltq_mtd.map = devm_kzalloc(&pdev.dev, sizeof(struct map_info),
    GFP_KERNEL);
    if (!ltq_mtd.map)
    return -ENOMEM;
    ltq_mtd.map.phys = ltq_mtd.res.start;
    ltq_mtd.map.size = resource_size(ltq_mtd.res);
    ltq_mtd.map.name = ltq_map_name;
    ltq_mtd.map.bankwidth = 2;
    ltq_mtd.map.read = ltq_read16;
    ltq_mtd.map.write = ltq_write16;
    ltq_mtd.map.copy_from = ltq_copy_from;
    ltq_mtd.map.copy_to = ltq_copy_to;
    ltq_mtd.map.map_priv_1 = LTQ_NOR_PROBING;
    ltq_mtd.mtd = do_map_probe("cfi_probe", ltq_mtd.map);
    ltq_mtd.map.map_priv_1 = LTQ_NOR_NORMAL;
    if (!ltq_mtd.mtd) {
    dev_err(&pdev.dev, "probing failed\n");
    return -ENXIO;
    }
    ltq_mtd.mtd.dev.parent = &pdev.dev;
    mtd_set_of_node(ltq_mtd.mtd, pdev.dev.of_node);
    cfi = ltq_mtd.map.fldrv_priv;
    cfi.addr_unlock1 ^= 1;
    cfi.addr_unlock2 ^= 1;
    err = mtd_device_register(ltq_mtd.mtd, core::ptr::null_mut(), 0);
    if (err) {
    dev_err(&pdev.dev, "failed to add partitions\n");
    goto err_destroy;
    }
    return 0;
    err_destroy:
    map_destroy(ltq_mtd.mtd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ltq_mtd_remove(pdev: *mut platform_device) {
    static void ltq_mtd_remove(struct platform_device *pdev)
    {
    struct ltq_mtd *ltq_mtd = platform_get_drvdata(pdev);
    if (ltq_mtd && ltq_mtd.mtd) {
    mtd_device_unregister(ltq_mtd.mtd);
    map_destroy(ltq_mtd.mtd);
    }
    }
    static const struct of_device_id ltq_mtd_match[] = {
    { .compatible = "lantiq,nor" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ltq_mtd_match);
    static struct platform_driver ltq_mtd_driver = {
    .probe = ltq_mtd_probe,
    .remove = ltq_mtd_remove,
    .driver = {
    .name = "ltq-nor",
    .of_match_table = ltq_mtd_match,
    },
    };
    module_platform_driver(ltq_mtd_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("John Crispin <john@phrozen.org>");
    MODULE_DESCRIPTION("Lantiq SoC NOR");
