//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/softing/softing_cs.c
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
// Copyright (C) 2008-2010
//
// - Kurt Van Dijck, EIA Electronics
//

    static int softingcs_index;
    static DEFINE_SPINLOCK(softingcs_index_lock);
    static int softingcs_reset(struct platform_device *pdev, int v);
    static int softingcs_enable_irq(struct platform_device *pdev, int v);
//
// platform_data descriptions
//

    static const struct softing_platform_data softingcs_platform_data[] = {
    {
    .name = "CANcard",
    .manf = 0x0168, .prod = 0x001,
    .generation = 1,
    .nbus = 2,
    .freq = 16 * MHZ, .max_brp = 32, .max_sjw = 4,
    .dpram_size = 0x0800,
    .boot = {0x0000, 0x000000, fw_dir "bcard.bin",},
    .load = {0x0120, 0x00f600, fw_dir "ldcard.bin",},
    .app = {0x0010, 0x0d0000, fw_dir "cancard.bin",},
    .reset = softingcs_reset,
    .enable_irq = softingcs_enable_irq,
    }, {
    .name = "CANcard-NEC",
    .manf = 0x0168, .prod = 0x002,
    .generation = 1,
    .nbus = 2,
    .freq = 16 * MHZ, .max_brp = 32, .max_sjw = 4,
    .dpram_size = 0x0800,
    .boot = {0x0000, 0x000000, fw_dir "bcard.bin",},
    .load = {0x0120, 0x00f600, fw_dir "ldcard.bin",},
    .app = {0x0010, 0x0d0000, fw_dir "cancard.bin",},
    .reset = softingcs_reset,
    .enable_irq = softingcs_enable_irq,
    }, {
    .name = "CANcard-SJA",
    .manf = 0x0168, .prod = 0x004,
    .generation = 1,
    .nbus = 2,
    .freq = 20 * MHZ, .max_brp = 32, .max_sjw = 4,
    .dpram_size = 0x0800,
    .boot = {0x0000, 0x000000, fw_dir "bcard.bin",},
    .load = {0x0120, 0x00f600, fw_dir "ldcard.bin",},
    .app = {0x0010, 0x0d0000, fw_dir "cansja.bin",},
    .reset = softingcs_reset,
    .enable_irq = softingcs_enable_irq,
    }, {
    .name = "CANcard-2",
    .manf = 0x0168, .prod = 0x005,
    .generation = 2,
    .nbus = 2,
    .freq = 24 * MHZ, .max_brp = 64, .max_sjw = 4,
    .dpram_size = 0x1000,
    .boot = {0x0000, 0x000000, fw_dir "bcard2.bin",},
    .load = {0x0120, 0x00f600, fw_dir "ldcard2.bin",},
    .app = {0x0010, 0x0d0000, fw_dir "cancrd2.bin",},
    .reset = softingcs_reset,
    .enable_irq = core::ptr::null_mut(),
    }, {
    .name = "Vector-CANcard",
    .manf = 0x0168, .prod = 0x081,
    .generation = 1,
    .nbus = 2,
    .freq = 16 * MHZ, .max_brp = 64, .max_sjw = 4,
    .dpram_size = 0x0800,
    .boot = {0x0000, 0x000000, fw_dir "bcard.bin",},
    .load = {0x0120, 0x00f600, fw_dir "ldcard.bin",},
    .app = {0x0010, 0x0d0000, fw_dir "cancard.bin",},
    .reset = softingcs_reset,
    .enable_irq = softingcs_enable_irq,
    }, {
    .name = "Vector-CANcard-SJA",
    .manf = 0x0168, .prod = 0x084,
    .generation = 1,
    .nbus = 2,
    .freq = 20 * MHZ, .max_brp = 32, .max_sjw = 4,
    .dpram_size = 0x0800,
    .boot = {0x0000, 0x000000, fw_dir "bcard.bin",},
    .load = {0x0120, 0x00f600, fw_dir "ldcard.bin",},
    .app = {0x0010, 0x0d0000, fw_dir "cansja.bin",},
    .reset = softingcs_reset,
    .enable_irq = softingcs_enable_irq,
    }, {
    .name = "Vector-CANcard-2",
    .manf = 0x0168, .prod = 0x085,
    .generation = 2,
    .nbus = 2,
    .freq = 24 * MHZ, .max_brp = 64, .max_sjw = 4,
    .dpram_size = 0x1000,
    .boot = {0x0000, 0x000000, fw_dir "bcard2.bin",},
    .load = {0x0120, 0x00f600, fw_dir "ldcard2.bin",},
    .app = {0x0010, 0x0d0000, fw_dir "cancrd2.bin",},
    .reset = softingcs_reset,
    .enable_irq = core::ptr::null_mut(),
    }, {
    .name = "EDICcard-NEC",
    .manf = 0x0168, .prod = 0x102,
    .generation = 1,
    .nbus = 2,
    .freq = 16 * MHZ, .max_brp = 64, .max_sjw = 4,
    .dpram_size = 0x0800,
    .boot = {0x0000, 0x000000, fw_dir "bcard.bin",},
    .load = {0x0120, 0x00f600, fw_dir "ldcard.bin",},
    .app = {0x0010, 0x0d0000, fw_dir "cancard.bin",},
    .reset = softingcs_reset,
    .enable_irq = softingcs_enable_irq,
    }, {
    .name = "EDICcard-2",
    .manf = 0x0168, .prod = 0x105,
    .generation = 2,
    .nbus = 2,
    .freq = 24 * MHZ, .max_brp = 64, .max_sjw = 4,
    .dpram_size = 0x1000,
    .boot = {0x0000, 0x000000, fw_dir "bcard2.bin",},
    .load = {0x0120, 0x00f600, fw_dir "ldcard2.bin",},
    .app = {0x0010, 0x0d0000, fw_dir "cancrd2.bin",},
    .reset = softingcs_reset,
    .enable_irq = core::ptr::null_mut(),
    }, {
    0, 0,
    },
    };
    MODULE_FIRMWARE(fw_dir "bcard.bin");
    MODULE_FIRMWARE(fw_dir "ldcard.bin");
    MODULE_FIRMWARE(fw_dir "cancard.bin");
    MODULE_FIRMWARE(fw_dir "cansja.bin");
    MODULE_FIRMWARE(fw_dir "bcard2.bin");
    MODULE_FIRMWARE(fw_dir "ldcard2.bin");
    MODULE_FIRMWARE(fw_dir "cancrd2.bin");
    static const struct softing_platform_data
// softingcs_find_platform_data(unsigned int manf, unsigned int prod)
    {
    const struct softing_platform_data *lp;
    for (lp = softingcs_platform_data; lp.manf; ++lp) {
    if ((lp.manf == manf) && (lp.prod == prod))
    return lp;
    }
    return core::ptr::null_mut();
    }
//
// platformdata callbacks
//
#[no_mangle]
unsafe extern "C" fn softingcs_reset(pdev: *mut platform_device, v: c_int) -> c_int {
    static int softingcs_reset(struct platform_device *pdev, int v)
    {
    struct pcmcia_device *pcmcia = to_pcmcia_dev(pdev.dev.parent);
    dev_dbg(&pdev.dev, "pcmcia config [2] %02x\n", v ? 0 : 0x20);
    return pcmcia_write_config_byte(pcmcia, 2, v ? 0 : 0x20);
    }
#[no_mangle]
unsafe extern "C" fn softingcs_enable_irq(pdev: *mut platform_device, v: c_int) -> c_int {
    static int softingcs_enable_irq(struct platform_device *pdev, int v)
    {
    struct pcmcia_device *pcmcia = to_pcmcia_dev(pdev.dev.parent);
    dev_dbg(&pdev.dev, "pcmcia config [0] %02x\n", v ? 0x60 : 0);
    return pcmcia_write_config_byte(pcmcia, 0, v ? 0x60 : 0);
    }
//
// pcmcia check
//
#[no_mangle]
unsafe extern "C" fn softingcs_probe_config(pcmcia: *mut pcmcia_device, priv_data: *mut c_void) -> c_int {
    static int softingcs_probe_config(struct pcmcia_device *pcmcia, void *priv_data)
    {
    struct softing_platform_data *pdat = priv_data;
    struct resource *pres;
    let mut memspeed: c_int = 0;
    WARN_ON(!pdat);
    pres = pcmcia.resource[PCMCIA_IOMEM_0];
    if (resource_size(pres) < 0x1000)
    return -ERANGE;
    pres.flags |= WIN_MEMORY_TYPE_CM | WIN_ENABLE;
    if (pdat.generation < 2) {
    pres.flags |= WIN_USE_WAIT | WIN_DATA_WIDTH_8;
    memspeed = 3;
    } else {
    pres.flags |= WIN_DATA_WIDTH_16;
    }
    return pcmcia_request_window(pcmcia, pres, memspeed);
    }
#[no_mangle]
unsafe extern "C" fn softingcs_remove(pcmcia: *mut pcmcia_device) {
    static void softingcs_remove(struct pcmcia_device *pcmcia)
    {
    struct platform_device *pdev = pcmcia.priv;
// free bits
    platform_device_unregister(pdev);
// release pcmcia stuff
    pcmcia_disable_device(pcmcia);
    }
//
// platform_device wrapper
// pdev->resource has 2 entries: io & irq
//
#[no_mangle]
unsafe extern "C" fn softingcs_pdev_release(dev: *mut device) {
    static void softingcs_pdev_release(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    kfree(pdev);
    }
#[no_mangle]
unsafe extern "C" fn softingcs_probe(pcmcia: *mut pcmcia_device) -> c_int {
    static int softingcs_probe(struct pcmcia_device *pcmcia)
    {
    int ret;
    struct platform_device *pdev;
    const struct softing_platform_data *pdat;
    struct resource *pres;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev {
    pub pdev: platform_device,
    pub res: [resource; 2],
    pub dev: *mut },
// find matching platform_data
    pub pcmcia->card_id): pdat = softingcs_find_platform_data(pcmcia->manf_id,,
    if (!pdat)
    pub -ENOTTY: return,
// setup pcmcia device
    pcmcia.config_flags |= CONF_ENABLE_IRQ | CONF_AUTO_SET_IOMEM |
    pub CONF_AUTO_CHECK_VCC: CONF_AUTO_SET_VPP |,
    pub )pdat): *mut ret = pcmcia_loop_config(pcmcia, softingcs_probe_config, (void,
    if (ret)
    pub pcmcia_failed: goto,
    pub pcmcia_enable_device(pcmcia): ret =,
    if (ret < 0)
    pub pcmcia_failed: goto,
    pub pcmcia->resource[PCMCIA_IOMEM_0]: pres =,
    if (!pres) {
    pub -EBADF: ret =,
    pub pcmcia_bad: goto,
    }
// create softing platform device
    pub kzalloc_obj(*dev): *mut dev =,
    if (!dev) {
    pub -ENOMEM: ret =,
    pub mem_failed: goto,
    }
    pub dev->res: dev->pdev.resource =,
    pub ARRAY_SIZE(dev->res): dev->pdev.num_resources =,
    pub softingcs_pdev_release: dev->pdev.dev.release =,
    pub &dev->pdev: pdev =,
    pub )pdat: *mut pdev->dev.platform_data = (void,
    pub &pcmcia->dev: pdev->dev.parent =,
    pub pdev: pcmcia->priv =,
// platform device resources
    pub IORESOURCE_MEM: pdev->resource[0].flags =,
    pub pres->start: pdev->resource[0].start =,
    pub pres->end: pdev->resource[0].end =,
    pub IORESOURCE_IRQ: pdev->resource[1].flags =,
    pub pcmcia->irq: pdev->resource[1].start =,
    pub pdev->resource[1].start: pdev->resource[1].end =,
// platform device setup
    pub softingcs_index++: pdev->id =,
    pub "softing": pdev->name =,
    pub pdev->id): dev_set_name(&pdev->dev, "softingcs.%i",,
    pub platform_device_register(pdev): ret =,
    if (ret < 0)
    pub platform_failed: goto,
    pub dev_name(&pdev->dev)): dev_info(&pcmcia->dev, "created %s\n",,
    pub 0: return,
    platform_failed:
    mem_failed:
    pcmcia_bad:
    pcmcia_failed:
    pub NULL: pcmcia->priv =,
    pub ret: return,
    }
    static const struct pcmcia_device_id softingcs_ids[] = {
// softing
    PCMCIA_DEVICE_MANF_CARD(0x0168, 0x0001),
    PCMCIA_DEVICE_MANF_CARD(0x0168, 0x0002),
    PCMCIA_DEVICE_MANF_CARD(0x0168, 0x0004),
    PCMCIA_DEVICE_MANF_CARD(0x0168, 0x0005),
// vector, manufacturer?
    PCMCIA_DEVICE_MANF_CARD(0x0168, 0x0081),
    PCMCIA_DEVICE_MANF_CARD(0x0168, 0x0084),
    PCMCIA_DEVICE_MANF_CARD(0x0168, 0x0085),
// EDIC
    PCMCIA_DEVICE_MANF_CARD(0x0168, 0x0102),
    PCMCIA_DEVICE_MANF_CARD(0x0168, 0x0105),
    PCMCIA_DEVICE_NULL,
}

    MODULE_DEVICE_TABLE(pcmcia, softingcs_ids);
    static struct pcmcia_driver softingcs_driver = {
    .owner		= THIS_MODULE,
    .name		= "softingcs",
    .id_table	= softingcs_ids,
    .probe		= softingcs_probe,
    .remove		= softingcs_remove,
    };
    module_pcmcia_driver(softingcs_driver);
    MODULE_DESCRIPTION("softing CANcard driver"
    ", links PCMCIA card to softing driver");
    MODULE_LICENSE("GPL v2");
