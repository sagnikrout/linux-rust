//! Automatically rewritten from C to Rust
//! Source: drivers/char/ipmi/kcs_bmc_npcm7xx.c
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
//
// Copyright (c) 2018, Nuvoton Corporation.
// Copyright (c) 2018, Intel Corporation.
//

pub const KCS_CHANNEL_MAX: c_int = 3;
pub const KCS1ST: c_uint = 0x0C;
pub const KCS2ST: c_uint = 0x1E;
pub const KCS3ST: c_uint = 0x30;
pub const KCS1DO: c_uint = 0x0E;
pub const KCS2DO: c_uint = 0x20;
pub const KCS3DO: c_uint = 0x32;
pub const KCS1DI: c_uint = 0x10;
pub const KCS2DI: c_uint = 0x22;
pub const KCS3DI: c_uint = 0x34;
pub const KCS1CTL: c_uint = 0x18;
pub const KCS2CTL: c_uint = 0x2A;
pub const KCS3CTL: c_uint = 0x3C;

pub const KCS1IE: c_uint = 0x1C;
pub const KCS2IE: c_uint = 0x2E;
pub const KCS3IE: c_uint = 0x40;

//
// 7.2.4 Core KCS Registers
// Registers in this module are 8 bits. An 8-bit register must be accessed
// by an 8-bit read or write.
//
// sts: KCS Channel n Status Register (KCSnST).
// dob: KCS Channel n Data Out Buffer Register (KCSnDO).
// dib: KCS Channel n Data In Buffer Register (KCSnDI).
// ctl: KCS Channel n Control Register (KCSnCTL).
// ie : KCS Channel n  Interrupt Enable Register (KCSnIE).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npcm7xx_kcs_reg {
    pub sts: u32,
    pub dob: u32,
    pub dib: u32,
    pub ctl: u32,
    pub ie: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npcm7xx_kcs_bmc {
    pub kcs_bmc: kcs_bmc_device,
    pub map: *mut regmap,
    pub reg: *const npcm7xx_kcs_reg,
}

    static const struct npcm7xx_kcs_reg npcm7xx_kcs_reg_tbl[KCS_CHANNEL_MAX] = {
    { .sts = KCS1ST, .dob = KCS1DO, .dib = KCS1DI, .ctl = KCS1CTL, .ie = KCS1IE },
    { .sts = KCS2ST, .dob = KCS2DO, .dib = KCS2DI, .ctl = KCS2CTL, .ie = KCS2IE },
    { .sts = KCS3ST, .dob = KCS3DO, .dib = KCS3DI, .ctl = KCS3CTL, .ie = KCS3IE },
    };
    static inline struct npcm7xx_kcs_bmc *to_npcm7xx_kcs_bmc(struct kcs_bmc_device *kcs_bmc)
    {
    return container_of(kcs_bmc, struct npcm7xx_kcs_bmc, kcs_bmc);
    }
#[no_mangle]
unsafe extern "C" fn npcm7xx_kcs_inb(kcs_bmc: *mut kcs_bmc_device, reg: u32) -> u8 {
    static u8 npcm7xx_kcs_inb(struct kcs_bmc_device *kcs_bmc, u32 reg)
    {
    struct npcm7xx_kcs_bmc *priv = to_npcm7xx_kcs_bmc(kcs_bmc);
    let mut val: u32 = 0;
    int rc;
    rc = regmap_read(priv.map, reg, &val);
    WARN(rc != 0, "regmap_read() failed: %d\n", rc);
    let mut rc: return = = 0 ? (u8)val : 0;
    }
#[no_mangle]
unsafe extern "C" fn npcm7xx_kcs_outb(kcs_bmc: *mut kcs_bmc_device, reg: u32, data: u8) {
    static void npcm7xx_kcs_outb(struct kcs_bmc_device *kcs_bmc, u32 reg, u8 data)
    {
    struct npcm7xx_kcs_bmc *priv = to_npcm7xx_kcs_bmc(kcs_bmc);
    int rc;
    rc = regmap_write(priv.map, reg, data);
    WARN(rc != 0, "regmap_write() failed: %d\n", rc);
    }
#[no_mangle]
unsafe extern "C" fn npcm7xx_kcs_updateb(kcs_bmc: *mut kcs_bmc_device, reg: u32, mask: u8, data: u8) {
    static void npcm7xx_kcs_updateb(struct kcs_bmc_device *kcs_bmc, u32 reg, u8 mask, u8 data)
    {
    struct npcm7xx_kcs_bmc *priv = to_npcm7xx_kcs_bmc(kcs_bmc);
    int rc;
    rc = regmap_update_bits(priv.map, reg, mask, data);
    WARN(rc != 0, "regmap_update_bits() failed: %d\n", rc);
    }
#[no_mangle]
unsafe extern "C" fn npcm7xx_kcs_enable_channel(kcs_bmc: *mut kcs_bmc_device, enable: bool) {
    static void npcm7xx_kcs_enable_channel(struct kcs_bmc_device *kcs_bmc, bool enable)
    {
    struct npcm7xx_kcs_bmc *priv = to_npcm7xx_kcs_bmc(kcs_bmc);
    regmap_update_bits(priv.map, priv.reg.ie, KCS_IE_IRQE | KCS_IE_HIRQE,
    enable ? KCS_IE_IRQE | KCS_IE_HIRQE : 0);
    }
#[no_mangle]
unsafe extern "C" fn npcm7xx_kcs_irq_mask_update(kcs_bmc: *mut kcs_bmc_device, mask: u8, state: u8) {
    static void npcm7xx_kcs_irq_mask_update(struct kcs_bmc_device *kcs_bmc, u8 mask, u8 state)
    {
    struct npcm7xx_kcs_bmc *priv = to_npcm7xx_kcs_bmc(kcs_bmc);
    if (mask & KCS_BMC_EVENT_TYPE_OBE)
    regmap_update_bits(priv.map, priv.reg.ctl, KCS_CTL_OBEIE,
    !!(state & KCS_BMC_EVENT_TYPE_OBE) * KCS_CTL_OBEIE);
    if (mask & KCS_BMC_EVENT_TYPE_IBF)
    regmap_update_bits(priv.map, priv.reg.ctl, KCS_CTL_IBFIE,
    !!(state & KCS_BMC_EVENT_TYPE_IBF) * KCS_CTL_IBFIE);
    }
#[no_mangle]
unsafe extern "C" fn npcm7xx_kcs_irq(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t npcm7xx_kcs_irq(int irq, void *arg)
    {
    struct kcs_bmc_device *kcs_bmc = arg;
    return kcs_bmc_handle_event(kcs_bmc);
    }
    static int npcm7xx_kcs_config_irq(struct kcs_bmc_device *kcs_bmc,
    struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int irq;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    return devm_request_irq(dev, irq, npcm7xx_kcs_irq, IRQF_SHARED,
    dev_name(dev), kcs_bmc);
    }
    static const struct kcs_bmc_device_ops npcm7xx_kcs_ops = {
    .irq_mask_update = npcm7xx_kcs_irq_mask_update,
    .io_inputb = npcm7xx_kcs_inb,
    .io_outputb = npcm7xx_kcs_outb,
    .io_updateb = npcm7xx_kcs_updateb,
    };
#[no_mangle]
unsafe extern "C" fn npcm7xx_kcs_probe(pdev: *mut platform_device) -> c_int {
    static int npcm7xx_kcs_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct npcm7xx_kcs_bmc *priv;
    struct kcs_bmc_device *kcs_bmc;
    u32 chan;
    int rc;
    rc = of_property_read_u32(dev.of_node, "kcs_chan", &chan);
    if (rc != 0 || chan == 0 || chan > KCS_CHANNEL_MAX) {
    dev_err(dev, "no valid 'kcs_chan' configured\n");
    return -ENODEV;
    }
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.map = syscon_node_to_regmap(dev.parent.of_node);
    if (IS_ERR(priv.map)) {
    dev_err(dev, "Couldn't get regmap\n");
    return -ENODEV;
    }
    priv.reg = &npcm7xx_kcs_reg_tbl[chan - 1];
    kcs_bmc = &priv.kcs_bmc;
    kcs_bmc.dev = &pdev.dev;
    kcs_bmc.channel = chan;
    kcs_bmc.ioreg.idr = priv.reg.dib;
    kcs_bmc.ioreg.odr = priv.reg.dob;
    kcs_bmc.ioreg.str = priv.reg.sts;
    kcs_bmc.ops = &npcm7xx_kcs_ops;
    platform_set_drvdata(pdev, priv);
    rc = npcm7xx_kcs_config_irq(kcs_bmc, pdev);
    if (rc)
    return rc;
    npcm7xx_kcs_irq_mask_update(kcs_bmc, (KCS_BMC_EVENT_TYPE_IBF | KCS_BMC_EVENT_TYPE_OBE), 0);
    npcm7xx_kcs_enable_channel(kcs_bmc, true);
    rc = kcs_bmc_add_device(kcs_bmc);
    if (rc) {
    dev_warn(&pdev.dev, "Failed to register channel %d: %d\n", kcs_bmc.channel, rc);
    return rc;
    }
    pr_info("channel=%u idr=0x%x odr=0x%x str=0x%x\n",
    chan,
    kcs_bmc.ioreg.idr, kcs_bmc.ioreg.odr, kcs_bmc.ioreg.str);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn npcm7xx_kcs_remove(pdev: *mut platform_device) {
    static void npcm7xx_kcs_remove(struct platform_device *pdev)
    {
    struct npcm7xx_kcs_bmc *priv = platform_get_drvdata(pdev);
    struct kcs_bmc_device *kcs_bmc = &priv.kcs_bmc;
    kcs_bmc_remove_device(kcs_bmc);
    npcm7xx_kcs_enable_channel(kcs_bmc, false);
    npcm7xx_kcs_irq_mask_update(kcs_bmc, (KCS_BMC_EVENT_TYPE_IBF | KCS_BMC_EVENT_TYPE_OBE), 0);
    }
    static const struct of_device_id npcm_kcs_bmc_match[] = {
    { .compatible = "nuvoton,npcm750-kcs-bmc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, npcm_kcs_bmc_match);
    static struct platform_driver npcm_kcs_bmc_driver = {
    .driver = {
    .name		= DEVICE_NAME,
    .of_match_table	= npcm_kcs_bmc_match,
    },
    .probe	= npcm7xx_kcs_probe,
    .remove = npcm7xx_kcs_remove,
    };
    module_platform_driver(npcm_kcs_bmc_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Avi Fishman <avifishman70@gmail.com>");
    MODULE_AUTHOR("Haiyue Wang <haiyue.wang@linux.intel.com>");
    MODULE_DESCRIPTION("NPCM7xx device interface to the KCS BMC device");
