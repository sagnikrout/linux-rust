//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/bcm/bcm63xx-power.c
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
// BCM63xx Power Domain Controller Driver
//
// Copyright (C) 2020 Álvaro Fernández Rojas <noltari@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm63xx_power_dev {
    pub genpd: generic_pm_domain,
    pub power: *mut bcm63xx_power,
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm63xx_power {
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
    pub dev: *mut bcm63xx_power_dev,
    pub genpd_data: genpd_onecell_data,
    pub genpd: *mut generic_pm_domain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm63xx_power_data {
    pub name: *const *const c_char,
    pub bit: u8,
    pub flags: c_uint,
}

#[no_mangle]
unsafe extern "C" fn bcm63xx_power_get_state(pmd: *mut bcm63xx_power_dev, is_on: *mut bool) -> c_int {
    static int bcm63xx_power_get_state(struct bcm63xx_power_dev *pmd, bool *is_on)
    {
    struct bcm63xx_power *power = pmd.power;
    if (!pmd.mask) {
// is_on = false;
    return -EINVAL;
    }
// is_on = !(__raw_readl(power->base) & pmd->mask);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_power_set_state(pmd: *mut bcm63xx_power_dev, on: bool) -> c_int {
    static int bcm63xx_power_set_state(struct bcm63xx_power_dev *pmd, bool on)
    {
    struct bcm63xx_power *power = pmd.power;
    unsigned long flags;
    uint32_t val;
    if (!pmd.mask)
    return -EINVAL;
    spin_lock_irqsave(&power.lock, flags);
    val = __raw_readl(power.base);
    if (on)
    val &= ~pmd.mask;
    else
    val |= pmd.mask;
    __raw_writel(val, power.base);
    spin_unlock_irqrestore(&power.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_power_on(genpd: *mut generic_pm_domain) -> c_int {
    static int bcm63xx_power_on(struct generic_pm_domain *genpd)
    {
    struct bcm63xx_power_dev *pmd = container_of(genpd,
    struct bcm63xx_power_dev, genpd);
    return bcm63xx_power_set_state(pmd, true);
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_power_off(genpd: *mut generic_pm_domain) -> c_int {
    static int bcm63xx_power_off(struct generic_pm_domain *genpd)
    {
    struct bcm63xx_power_dev *pmd = container_of(genpd,
    struct bcm63xx_power_dev, genpd);
    return bcm63xx_power_set_state(pmd, false);
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_power_probe(pdev: *mut platform_device) -> c_int {
    static int bcm63xx_power_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    const struct bcm63xx_power_data *entry, *table;
    struct bcm63xx_power *power;
    unsigned int ndom;
    let mut max_bit: u8 = 0;
    int ret;
    power = devm_kzalloc(dev, sizeof(*power), GFP_KERNEL);
    if (!power)
    return -ENOMEM;
    power.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(power.base))
    return PTR_ERR(power.base);
    table = of_device_get_match_data(dev);
    if (!table)
    return -EINVAL;
    power.genpd_data.num_domains = 0;
    ndom = 0;
    for (entry = table; entry.name; entry++) {
    max_bit = max(max_bit, entry.bit);
    ndom++;
    }
    if (!ndom)
    return -ENODEV;
    power.genpd_data.num_domains = max_bit + 1;
    power.dev = devm_kcalloc(dev, power.genpd_data.num_domains,
    sizeof(struct bcm63xx_power_dev),
    GFP_KERNEL);
    if (!power.dev)
    return -ENOMEM;
    power.genpd = devm_kcalloc(dev, power.genpd_data.num_domains,
    sizeof(struct generic_pm_domain *),
    GFP_KERNEL);
    if (!power.genpd)
    return -ENOMEM;
    power.genpd_data.domains = power.genpd;
    ndom = 0;
    for (entry = table; entry.name; entry++) {
    struct bcm63xx_power_dev *pmd = &power.dev[ndom];
    bool is_on;
    pmd.power = power;
    pmd.mask = BIT(entry.bit);
    pmd.genpd.name = entry.name;
    pmd.genpd.flags = entry.flags;
    ret = bcm63xx_power_get_state(pmd, &is_on);
    if (ret)
    dev_warn(dev, "unable to get current state for %s\n",
    pmd.genpd.name);
    pmd.genpd.power_on = bcm63xx_power_on;
    pmd.genpd.power_off = bcm63xx_power_off;
    pm_genpd_init(&pmd.genpd, core::ptr::null_mut(), !is_on);
    power.genpd[entry.bit] = &pmd.genpd;
    ndom++;
    }
    spin_lock_init(&power.lock);
    ret = of_genpd_add_provider_onecell(np, &power.genpd_data);
    if (ret) {
    dev_err(dev, "failed to register genpd driver: %d\n", ret);
    return ret;
    }
    dev_info(dev, "registered %u power domains\n", ndom);
    return 0;
    }
    static const struct bcm63xx_power_data bcm6318_power_domains[] = {
    {
    .name = "pcie",
    .bit = BCM6318_POWER_DOMAIN_PCIE,
    }, {
    .name = "usb",
    .bit = BCM6318_POWER_DOMAIN_USB,
    }, {
    .name = "ephy0",
    .bit = BCM6318_POWER_DOMAIN_EPHY0,
    }, {
    .name = "ephy1",
    .bit = BCM6318_POWER_DOMAIN_EPHY1,
    }, {
    .name = "ephy2",
    .bit = BCM6318_POWER_DOMAIN_EPHY2,
    }, {
    .name = "ephy3",
    .bit = BCM6318_POWER_DOMAIN_EPHY3,
    }, {
    .name = "ldo2p5",
    .bit = BCM6318_POWER_DOMAIN_LDO2P5,
    .flags = GENPD_FLAG_ALWAYS_ON,
    }, {
    .name = "ldo2p9",
    .bit = BCM6318_POWER_DOMAIN_LDO2P9,
    .flags = GENPD_FLAG_ALWAYS_ON,
    }, {
    .name = "sw1p0",
    .bit = BCM6318_POWER_DOMAIN_SW1P0,
    .flags = GENPD_FLAG_ALWAYS_ON,
    }, {
    .name = "pad",
    .bit = BCM6318_POWER_DOMAIN_PAD,
    .flags = GENPD_FLAG_ALWAYS_ON,
    }, {
// sentinel
    },
    };
    static const struct bcm63xx_power_data bcm6328_power_domains[] = {
    {
    .name = "adsl2-mips",
    .bit = BCM6328_POWER_DOMAIN_ADSL2_MIPS,
    }, {
    .name = "adsl2-phy",
    .bit = BCM6328_POWER_DOMAIN_ADSL2_PHY,
    }, {
    .name = "adsl2-afe",
    .bit = BCM6328_POWER_DOMAIN_ADSL2_AFE,
    }, {
    .name = "sar",
    .bit = BCM6328_POWER_DOMAIN_SAR,
    }, {
    .name = "pcm",
    .bit = BCM6328_POWER_DOMAIN_PCM,
    }, {
    .name = "usbd",
    .bit = BCM6328_POWER_DOMAIN_USBD,
    }, {
    .name = "usbh",
    .bit = BCM6328_POWER_DOMAIN_USBH,
    }, {
    .name = "pcie",
    .bit = BCM6328_POWER_DOMAIN_PCIE,
    }, {
    .name = "robosw",
    .bit = BCM6328_POWER_DOMAIN_ROBOSW,
    }, {
    .name = "ephy",
    .bit = BCM6328_POWER_DOMAIN_EPHY,
    }, {
// sentinel
    },
    };
    static const struct bcm63xx_power_data bcm6362_power_domains[] = {
    {
    .name = "sar",
    .bit = BCM6362_POWER_DOMAIN_SAR,
    }, {
    .name = "ipsec",
    .bit = BCM6362_POWER_DOMAIN_IPSEC,
    }, {
    .name = "mips",
    .bit = BCM6362_POWER_DOMAIN_MIPS,
    .flags = GENPD_FLAG_ALWAYS_ON,
    }, {
    .name = "dect",
    .bit = BCM6362_POWER_DOMAIN_DECT,
    }, {
    .name = "usbh",
    .bit = BCM6362_POWER_DOMAIN_USBH,
    }, {
    .name = "usbd",
    .bit = BCM6362_POWER_DOMAIN_USBD,
    }, {
    .name = "robosw",
    .bit = BCM6362_POWER_DOMAIN_ROBOSW,
    }, {
    .name = "pcm",
    .bit = BCM6362_POWER_DOMAIN_PCM,
    }, {
    .name = "periph",
    .bit = BCM6362_POWER_DOMAIN_PERIPH,
    .flags = GENPD_FLAG_ALWAYS_ON,
    }, {
    .name = "adsl-phy",
    .bit = BCM6362_POWER_DOMAIN_ADSL_PHY,
    }, {
    .name = "gmii-pads",
    .bit = BCM6362_POWER_DOMAIN_GMII_PADS,
    }, {
    .name = "fap",
    .bit = BCM6362_POWER_DOMAIN_FAP,
    }, {
    .name = "pcie",
    .bit = BCM6362_POWER_DOMAIN_PCIE,
    }, {
    .name = "wlan-pads",
    .bit = BCM6362_POWER_DOMAIN_WLAN_PADS,
    }, {
// sentinel
    },
    };
    static const struct bcm63xx_power_data bcm63268_power_domains[] = {
    {
    .name = "sar",
    .bit = BCM63268_POWER_DOMAIN_SAR,
    }, {
    .name = "ipsec",
    .bit = BCM63268_POWER_DOMAIN_IPSEC,
    }, {
    .name = "mips",
    .bit = BCM63268_POWER_DOMAIN_MIPS,
    .flags = GENPD_FLAG_ALWAYS_ON,
    }, {
    .name = "dect",
    .bit = BCM63268_POWER_DOMAIN_DECT,
    }, {
    .name = "usbh",
    .bit = BCM63268_POWER_DOMAIN_USBH,
    }, {
    .name = "usbd",
    .bit = BCM63268_POWER_DOMAIN_USBD,
    }, {
    .name = "robosw",
    .bit = BCM63268_POWER_DOMAIN_ROBOSW,
    }, {
    .name = "pcm",
    .bit = BCM63268_POWER_DOMAIN_PCM,
    }, {
    .name = "periph",
    .bit = BCM63268_POWER_DOMAIN_PERIPH,
    .flags = GENPD_FLAG_ALWAYS_ON,
    }, {
    .name = "vdsl-phy",
    .bit = BCM63268_POWER_DOMAIN_VDSL_PHY,
    }, {
    .name = "vdsl-mips",
    .bit = BCM63268_POWER_DOMAIN_VDSL_MIPS,
    }, {
    .name = "fap",
    .bit = BCM63268_POWER_DOMAIN_FAP,
    }, {
    .name = "pcie",
    .bit = BCM63268_POWER_DOMAIN_PCIE,
    }, {
    .name = "wlan-pads",
    .bit = BCM63268_POWER_DOMAIN_WLAN_PADS,
    }, {
// sentinel
    },
    };
    static const struct of_device_id bcm63xx_power_of_match[] = {
    {
    .compatible = "brcm,bcm6318-power-controller",
    .data = &bcm6318_power_domains,
    }, {
    .compatible = "brcm,bcm6328-power-controller",
    .data = &bcm6328_power_domains,
    }, {
    .compatible = "brcm,bcm6362-power-controller",
    .data = &bcm6362_power_domains,
    }, {
    .compatible = "brcm,bcm63268-power-controller",
    .data = &bcm63268_power_domains,
    }, {
// sentinel
    }
    };
    static struct platform_driver bcm63xx_power_driver = {
    .driver = {
    .name = "bcm63xx-power-controller",
    .of_match_table = bcm63xx_power_of_match,
    },
    .probe  = bcm63xx_power_probe,
    };
    builtin_platform_driver(bcm63xx_power_driver);
