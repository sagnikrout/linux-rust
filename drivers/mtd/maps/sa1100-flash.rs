//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/sa1100-flash.c
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
// Flash memory access on SA11x0 based devices
//
// (C) 2000 Nicolas Pitre <nico@fluxnic.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_subdev_info {
    pub name: [c_char; 16],
    pub map: map_info,
    pub mtd: *mut mtd_info,
    pub plat: *mut flash_platform_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_info {
    pub mtd: *mut mtd_info,
    pub num_subdev: c_int,
    pub subdev: [sa_subdev_info; ],
}

    static DEFINE_SPINLOCK(sa1100_vpp_lock);
    static int sa1100_vpp_refcnt;
#[no_mangle]
unsafe extern "C" fn sa1100_set_vpp(map: *mut map_info, on: c_int) {
    static void sa1100_set_vpp(struct map_info *map, int on)
    {
    struct sa_subdev_info *subdev = container_of(map, struct sa_subdev_info, map);
    unsigned long flags;
    spin_lock_irqsave(&sa1100_vpp_lock, flags);
    if (on) {
    if (++sa1100_vpp_refcnt == 1)   /* first nested 'on' */
    subdev.plat.set_vpp(1);
    } else {
    if (--sa1100_vpp_refcnt == 0)   /* last nested 'off' */
    subdev.plat.set_vpp(0);
    }
    spin_unlock_irqrestore(&sa1100_vpp_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sa1100_destroy_subdev(subdev: *mut sa_subdev_info) {
    static void sa1100_destroy_subdev(struct sa_subdev_info *subdev)
    {
    if (subdev.mtd)
    map_destroy(subdev.mtd);
    if (subdev.map.virt)
    iounmap(subdev.map.virt);
    release_mem_region(subdev.map.phys, subdev.map.size);
    }
#[no_mangle]
unsafe extern "C" fn sa1100_probe_subdev(subdev: *mut sa_subdev_info, res: *mut resource) -> c_int {
    static int sa1100_probe_subdev(struct sa_subdev_info *subdev, struct resource *res)
    {
    unsigned long phys;
    unsigned int size;
    int ret;
    phys = res.start;
    size = res.end - phys + 1;
//
// Retrieve the bankwidth from the MSC registers.
// We currently only implement CS0 and CS1 here.
//
    switch (phys) {
    default:
    printk(KERN_WARNING "SA1100 flash: unknown base address "
    "0x%08lx, assuming CS0\n", phys);
    fallthrough;
    case SA1100_CS0_PHYS:
    subdev.map.bankwidth = (MSC0 & MSC_RBW) ? 2 : 4;
    break;
    case SA1100_CS1_PHYS:
    subdev.map.bankwidth = ((MSC0 >> 16) & MSC_RBW) ? 2 : 4;
    break;
    }
    if (!request_mem_region(phys, size, subdev.name)) {
    ret = -EBUSY;
    goto out;
    }
    if (subdev.plat.set_vpp)
    subdev.map.set_vpp = sa1100_set_vpp;
    subdev.map.phys = phys;
    subdev.map.size = size;
    subdev.map.virt = ioremap(phys, size);
    if (!subdev.map.virt) {
    ret = -ENOMEM;
    goto err;
    }
    simple_map_init(&subdev.map);
//
// Now let's probe for the actual flash.  Do it here since
// specific machine settings might have been set above.
//
    subdev.mtd = do_map_probe(subdev.plat.map_name, &subdev.map);
    if (subdev.mtd == core::ptr::null_mut()) {
    ret = -ENXIO;
    goto err;
    }
    printk(KERN_INFO "SA1100 flash: CFI device at 0x%08lx, %uMiB, %d-bit\n",
    phys, (unsigned)(subdev.mtd.size >> 20),
    subdev.map.bankwidth * 8);
    return 0;
    err:
    sa1100_destroy_subdev(subdev);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sa1100_destroy(info: *mut sa_info, plat: *mut flash_platform_data) {
    static void sa1100_destroy(struct sa_info *info, struct flash_platform_data *plat)
    {
    int i;
    if (info.mtd) {
    mtd_device_unregister(info.mtd);
    if (info.mtd != info.subdev[0].mtd)
    mtd_concat_destroy(info.mtd);
    }
    for (i = info.num_subdev - 1; i >= 0; i--)
    sa1100_destroy_subdev(&info.subdev[i]);
    kfree(info);
    if (plat.exit)
    plat.exit();
    }
    static struct sa_info *sa1100_setup_mtd(struct platform_device *pdev,
    struct flash_platform_data *plat)
    {
    struct sa_info *info;
    int nr, i, ret = 0;
//
// Count number of devices.
//
    for (nr = 0; ; nr++)
    if (!platform_get_resource(pdev, IORESOURCE_MEM, nr))
    break;
    if (nr == 0) {
    ret = -ENODEV;
    goto out;
    }
//
// Allocate the map_info structs in one go.
//
    info = kzalloc_flex(*info, subdev, nr);
    if (!info) {
    ret = -ENOMEM;
    goto out;
    }
    if (plat.init) {
    ret = plat.init();
    if (ret)
    goto err;
    }
//
// Claim and then map the memory regions.
//
    for (i = 0; i < nr; i++) {
    struct sa_subdev_info *subdev = &info.subdev[i];
    struct resource *res;
    res = platform_get_resource(pdev, IORESOURCE_MEM, i);
    if (!res)
    break;
    subdev.map.name = subdev.name;
    sprintf(subdev.name, "%s-%d", plat.name, i);
    subdev.plat = plat;
    ret = sa1100_probe_subdev(subdev, res);
    if (ret)
    break;
    }
    info.num_subdev = i;
//
// ENXIO is special.  It means we didn't find a chip when we probed.
//
    if (ret != 0 && !(ret == -ENXIO && info.num_subdev > 0))
    goto err;
//
// If we found one device, don't bother with concat support.  If
// we found multiple devices, use concat if we have it available,
// otherwise fail.  Either way, it'll be called "sa1100".
//
    if (info.num_subdev == 1) {
    strcpy(info.subdev[0].name, plat.name);
    info.mtd = info.subdev[0].mtd;
    ret = 0;
    } else if (info.num_subdev > 1) {
    struct mtd_info **cdev;
    cdev = kmalloc_objs(*cdev, nr);
    if (!cdev) {
    ret = -ENOMEM;
    goto err;
    }
//
// We detected multiple devices.  Concatenate them together.
//
    for (i = 0; i < info.num_subdev; i++)
    cdev[i] = info.subdev[i].mtd;
    info.mtd = mtd_concat_create(cdev, info.num_subdev,
    plat.name);
    kfree(cdev);
    if (info.mtd == core::ptr::null_mut()) {
    ret = -ENXIO;
    goto err;
    }
    }
    info.mtd.dev.parent = &pdev.dev;
    if (ret == 0)
    return info;
    err:
    sa1100_destroy(info, plat);
    out:
    return ERR_PTR(ret);
    }
    static const char * const part_probes[] = { "cmdlinepart", "RedBoot", core::ptr::null_mut() };
#[no_mangle]
unsafe extern "C" fn sa1100_mtd_probe(pdev: *mut platform_device) -> c_int {
    static int sa1100_mtd_probe(struct platform_device *pdev)
    {
    struct flash_platform_data *plat = dev_get_platdata(&pdev.dev);
    struct sa_info *info;
    int err;
    if (!plat)
    return -ENODEV;
    info = sa1100_setup_mtd(pdev, plat);
    if (IS_ERR(info)) {
    err = PTR_ERR(info);
    goto out;
    }
//
// Partition selection stuff.
//
    mtd_device_parse_register(info.mtd, part_probes, core::ptr::null_mut(), plat.parts,
    plat.nr_parts);
    platform_set_drvdata(pdev, info);
    err = 0;
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sa1100_mtd_remove(pdev: *mut platform_device) {
    static void sa1100_mtd_remove(struct platform_device *pdev)
    {
    struct sa_info *info = platform_get_drvdata(pdev);
    struct flash_platform_data *plat = dev_get_platdata(&pdev.dev);
    sa1100_destroy(info, plat);
    }
    static struct platform_driver sa1100_mtd_driver = {
    .probe		= sa1100_mtd_probe,
    .remove		= sa1100_mtd_remove,
    .driver		= {
    .name	= "sa1100-mtd",
    },
    };
    module_platform_driver(sa1100_mtd_driver);
    MODULE_AUTHOR("Nicolas Pitre");
    MODULE_DESCRIPTION("SA1100 CFI map driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:sa1100-mtd");
