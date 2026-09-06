//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-npcm.c
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
// Copyright (c) 2019 Nuvoton Technology corporation.

// NPCM7xx GCR registers
pub const NPCM_MDLR_OFFSET: c_uint = 0x7C;

// NPCM8xx MDLR bits

pub const NPCM_USB1PHYCTL_OFFSET: c_uint = 0x140;
pub const NPCM_USB2PHYCTL_OFFSET: c_uint = 0x144;
pub const NPCM_USB3PHYCTL_OFFSET: c_uint = 0x148;

// NPCM7xx Reset registers
pub const NPCM_SWRSTR: c_uint = 0x14;

pub const NPCM_IPSRST1: c_uint = 0x20;

pub const NPCM_IPSRST2: c_uint = 0x24;

pub const NPCM_IPSRST3: c_uint = 0x34;

pub const NPCM_IPSRST4: c_uint = 0x74;

pub const NPCM_RC_RESETS_PER_REG: c_int = 32;

    enum {
    BMC_NPCM7XX = 0,
    BMC_NPCM8XX,
    };
    static const u32 npxm7xx_ipsrst[] = {NPCM_IPSRST1, NPCM_IPSRST2, NPCM_IPSRST3};
    static const u32 npxm8xx_ipsrst[] = {NPCM_IPSRST1, NPCM_IPSRST2, NPCM_IPSRST3,
    NPCM_IPSRST4};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npcm_reset_info {
    pub bmc_id: u32,
    pub num_ipsrst: u32,
    pub ipsrst: *const u32,
}

    static const struct npcm_reset_info npxm7xx_reset_info[] = {
    {.bmc_id = BMC_NPCM7XX, .num_ipsrst = 3, .ipsrst = npxm7xx_ipsrst}};
    static const struct npcm_reset_info npxm8xx_reset_info[] = {
    {.bmc_id = BMC_NPCM8XX, .num_ipsrst = 4, .ipsrst = npxm8xx_ipsrst}};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npcm_rc_data {
    pub rcdev: reset_controller_dev,
    pub info: *const npcm_reset_info,
    pub gcr_regmap: *mut regmap,
    pub sw_reset_number: u32,
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn npcm_rc_restart(data: *mut sys_off_data) -> c_int {
    static int npcm_rc_restart(struct sys_off_data *data)
    {
    struct npcm_rc_data *rc = data.cb_data;
    writel(NPCM_SWRST << rc.sw_reset_number, rc.base + NPCM_SWRSTR);
    mdelay(1000);
    pr_emerg("%s: unable to restart system\n", __func__);
    return NOTIFY_DONE;
    }
    static int npcm_rc_setclear_reset(struct reset_controller_dev *rcdev,
    unsigned long id, bool set)
    {
    struct npcm_rc_data *rc = to_rc_data(rcdev);
    let mut rst_bit: c_uint = BIT(id & NPCM_MASK_RESETS);
    let mut ctrl_offset: c_uint = id >> 8;
    unsigned long flags;
    u32 stat;
    spin_lock_irqsave(&rc.lock, flags);
    stat = readl(rc.base + ctrl_offset);
    if (set)
    writel(stat | rst_bit, rc.base + ctrl_offset);
    else
    writel(stat & ~rst_bit, rc.base + ctrl_offset);
    spin_unlock_irqrestore(&rc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn npcm_rc_assert(rcdev: *mut reset_controller_dev, id: c_ulong) -> c_int {
    static int npcm_rc_assert(struct reset_controller_dev *rcdev, unsigned long id)
    {
    return npcm_rc_setclear_reset(rcdev, id, true);
    }
    static int npcm_rc_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return npcm_rc_setclear_reset(rcdev, id, false);
    }
    static int npcm_rc_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct npcm_rc_data *rc = to_rc_data(rcdev);
    let mut rst_bit: c_uint = BIT(id & NPCM_MASK_RESETS);
    let mut ctrl_offset: c_uint = id >> 8;
    return (readl(rc.base + ctrl_offset) & rst_bit);
    }
    static int npcm_reset_xlate(struct reset_controller_dev *rcdev,
    const struct of_phandle_args *reset_spec)
    {
    struct npcm_rc_data *rc = to_rc_data(rcdev);
    unsigned int offset, bit;
    let mut offset_found: bool = false;
    int off_num;
    offset = reset_spec.args[0];
    for (off_num = 0 ; off_num < rc.info.num_ipsrst ; off_num++) {
    if (offset == rc.info.ipsrst[off_num]) {
    offset_found = true;
    break;
    }
    }
    if (!offset_found) {
    dev_err(rcdev.dev, "Error reset register (0x%x)\n", offset);
    return -EINVAL;
    }
    bit = reset_spec.args[1];
    if (bit >= NPCM_RC_RESETS_PER_REG) {
    dev_err(rcdev.dev, "Error reset number (%d)\n", bit);
    return -EINVAL;
    }
    return (offset << 8) | bit;
    }
    static const struct of_device_id npcm_rc_match[] = {
    { .compatible = "nuvoton,npcm750-reset", .data = &npxm7xx_reset_info},
    { .compatible = "nuvoton,npcm845-reset", .data = &npxm8xx_reset_info},
    { }
    };
#[no_mangle]
unsafe extern "C" fn npcm_usb_reset_npcm7xx(rc: *mut npcm_rc_data) {
    static void npcm_usb_reset_npcm7xx(struct npcm_rc_data *rc)
    {
    u32 mdlr, iprst1, iprst2, iprst3;
    let mut ipsrst1_bits: u32 = 0;
    let mut ipsrst2_bits: u32 = NPCM_IPSRST2_USB_HOST;
    let mut ipsrst3_bits: u32 = 0;
// checking which USB device is enabled
    regmap_read(rc.gcr_regmap, NPCM_MDLR_OFFSET, &mdlr);
    if (!(mdlr & NPCM7XX_MDLR_USBD0))
    ipsrst3_bits |= NPCM_IPSRST3_USBD0;
    if (!(mdlr & NPCM7XX_MDLR_USBD1))
    ipsrst1_bits |= NPCM_IPSRST1_USBD1;
    if (!(mdlr & NPCM7XX_MDLR_USBD2_4))
    ipsrst1_bits |= (NPCM_IPSRST1_USBD2 |
    NPCM_IPSRST1_USBD3 |
    NPCM_IPSRST1_USBD4);
    if (!(mdlr & NPCM7XX_MDLR_USBD0)) {
    ipsrst1_bits |= (NPCM_IPSRST1_USBD5 |
    NPCM_IPSRST1_USBD6);
    ipsrst3_bits |= (NPCM_IPSRST3_USBD7 |
    NPCM_IPSRST3_USBD8 |
    NPCM_IPSRST3_USBD9);
    }
// assert reset USB PHY and USB devices
    iprst1 = readl(rc.base + NPCM_IPSRST1);
    iprst2 = readl(rc.base + NPCM_IPSRST2);
    iprst3 = readl(rc.base + NPCM_IPSRST3);
    iprst1 |= ipsrst1_bits;
    iprst2 |= ipsrst2_bits;
    iprst3 |= (ipsrst3_bits | NPCM_IPSRST3_USBPHY1 |
    NPCM_IPSRST3_USBPHY2);
    writel(iprst1, rc.base + NPCM_IPSRST1);
    writel(iprst2, rc.base + NPCM_IPSRST2);
    writel(iprst3, rc.base + NPCM_IPSRST3);
// clear USB PHY RS bit
    regmap_update_bits(rc.gcr_regmap, NPCM_USB1PHYCTL_OFFSET,
    NPCM_USBXPHYCTL_RS, 0);
    regmap_update_bits(rc.gcr_regmap, NPCM_USB2PHYCTL_OFFSET,
    NPCM_USBXPHYCTL_RS, 0);
// deassert reset USB PHY
    iprst3 &= ~(NPCM_IPSRST3_USBPHY1 | NPCM_IPSRST3_USBPHY2);
    writel(iprst3, rc.base + NPCM_IPSRST3);
    udelay(50);
// set USB PHY RS bit
    regmap_update_bits(rc.gcr_regmap, NPCM_USB1PHYCTL_OFFSET,
    NPCM_USBXPHYCTL_RS, NPCM_USBXPHYCTL_RS);
    regmap_update_bits(rc.gcr_regmap, NPCM_USB2PHYCTL_OFFSET,
    NPCM_USBXPHYCTL_RS, NPCM_USBXPHYCTL_RS);
// deassert reset USB devices
    iprst1 &= ~ipsrst1_bits;
    iprst2 &= ~ipsrst2_bits;
    iprst3 &= ~ipsrst3_bits;
    writel(iprst1, rc.base + NPCM_IPSRST1);
    writel(iprst2, rc.base + NPCM_IPSRST2);
    writel(iprst3, rc.base + NPCM_IPSRST3);
    }
#[no_mangle]
unsafe extern "C" fn npcm_usb_reset_npcm8xx(rc: *mut npcm_rc_data) {
    static void npcm_usb_reset_npcm8xx(struct npcm_rc_data *rc)
    {
    u32 mdlr, iprst1, iprst2, iprst3, iprst4;
    let mut ipsrst1_bits: u32 = 0;
    let mut ipsrst2_bits: u32 = NPCM_IPSRST2_USB_HOST;
    let mut ipsrst3_bits: u32 = 0;
    let mut ipsrst4_bits: u32 = NPCM_IPSRST4_USB_HOST2 | NPCM_IPSRST4_USBPHY3;
// checking which USB device is enabled
    regmap_read(rc.gcr_regmap, NPCM_MDLR_OFFSET, &mdlr);
    if (!(mdlr & NPCM8XX_MDLR_USBD0_3)) {
    ipsrst3_bits |= NPCM_IPSRST3_USBD0;
    ipsrst1_bits |= (NPCM_IPSRST1_USBD1 |
    NPCM_IPSRST1_USBD2 |
    NPCM_IPSRST1_USBD3);
    }
    if (!(mdlr & NPCM8XX_MDLR_USBD4_7)) {
    ipsrst1_bits |= (NPCM_IPSRST1_USBD4 |
    NPCM_IPSRST1_USBD5 |
    NPCM_IPSRST1_USBD6);
    ipsrst3_bits |= NPCM_IPSRST3_USBD7;
    }
    if (!(mdlr & NPCM8XX_MDLR_USBD8))
    ipsrst3_bits |= NPCM_IPSRST3_USBD8;
    if (!(mdlr & NPCM8XX_MDLR_USBD9))
    ipsrst3_bits |= NPCM_IPSRST3_USBD9;
// assert reset USB PHY and USB devices
    iprst1 = readl(rc.base + NPCM_IPSRST1);
    iprst2 = readl(rc.base + NPCM_IPSRST2);
    iprst3 = readl(rc.base + NPCM_IPSRST3);
    iprst4 = readl(rc.base + NPCM_IPSRST4);
    iprst1 |= ipsrst1_bits;
    iprst2 |= ipsrst2_bits;
    iprst3 |= (ipsrst3_bits | NPCM_IPSRST3_USBPHY1 |
    NPCM_IPSRST3_USBPHY2);
    iprst4 |= ipsrst4_bits;
    writel(iprst1, rc.base + NPCM_IPSRST1);
    writel(iprst2, rc.base + NPCM_IPSRST2);
    writel(iprst3, rc.base + NPCM_IPSRST3);
    writel(iprst4, rc.base + NPCM_IPSRST4);
// clear USB PHY RS bit
    regmap_update_bits(rc.gcr_regmap, NPCM_USB1PHYCTL_OFFSET,
    NPCM_USBXPHYCTL_RS, 0);
    regmap_update_bits(rc.gcr_regmap, NPCM_USB2PHYCTL_OFFSET,
    NPCM_USBXPHYCTL_RS, 0);
    regmap_update_bits(rc.gcr_regmap, NPCM_USB3PHYCTL_OFFSET,
    NPCM_USBXPHYCTL_RS, 0);
// deassert reset USB PHY
    iprst3 &= ~(NPCM_IPSRST3_USBPHY1 | NPCM_IPSRST3_USBPHY2);
    writel(iprst3, rc.base + NPCM_IPSRST3);
    iprst4 &= ~NPCM_IPSRST4_USBPHY3;
    writel(iprst4, rc.base + NPCM_IPSRST4);
// set USB PHY RS bit
    regmap_update_bits(rc.gcr_regmap, NPCM_USB1PHYCTL_OFFSET,
    NPCM_USBXPHYCTL_RS, NPCM_USBXPHYCTL_RS);
    regmap_update_bits(rc.gcr_regmap, NPCM_USB2PHYCTL_OFFSET,
    NPCM_USBXPHYCTL_RS, NPCM_USBXPHYCTL_RS);
    regmap_update_bits(rc.gcr_regmap, NPCM_USB3PHYCTL_OFFSET,
    NPCM_USBXPHYCTL_RS, NPCM_USBXPHYCTL_RS);
// deassert reset USB devices
    iprst1 &= ~ipsrst1_bits;
    iprst2 &= ~ipsrst2_bits;
    iprst3 &= ~ipsrst3_bits;
    iprst4 &= ~ipsrst4_bits;
    writel(iprst1, rc.base + NPCM_IPSRST1);
    writel(iprst2, rc.base + NPCM_IPSRST2);
    writel(iprst3, rc.base + NPCM_IPSRST3);
    writel(iprst4, rc.base + NPCM_IPSRST4);
    }
//
// The following procedure should be observed in USB PHY, USB device and
// USB host initialization at BMC boot
//
#[no_mangle]
unsafe extern "C" fn npcm_usb_reset(pdev: *mut platform_device, rc: *mut npcm_rc_data) -> c_int {
    static int npcm_usb_reset(struct platform_device *pdev, struct npcm_rc_data *rc)
    {
    struct device *dev = &pdev.dev;
    rc.gcr_regmap = syscon_regmap_lookup_by_phandle(dev.of_node, "nuvoton,sysgcr");
    if (IS_ERR(rc.gcr_regmap)) {
    dev_warn(&pdev.dev, "Failed to find nuvoton,sysgcr property, please update the device tree\n");
    dev_info(&pdev.dev, "Using nuvoton,npcm750-gcr for Poleg backward compatibility\n");
    rc.gcr_regmap = syscon_regmap_lookup_by_compatible("nuvoton,npcm750-gcr");
    if (IS_ERR(rc.gcr_regmap)) {
    dev_err(&pdev.dev, "Failed to find nuvoton,npcm750-gcr");
    return PTR_ERR(rc.gcr_regmap);
    }
    }
    rc.info = device_get_match_data(dev);
    switch (rc.info.bmc_id) {
    case BMC_NPCM7XX:
    npcm_usb_reset_npcm7xx(rc);
    break;
    case BMC_NPCM8XX:
    npcm_usb_reset_npcm8xx(rc);
    break;
    default:
    return -ENODEV;
    }
    return 0;
    }
    static const struct reset_control_ops npcm_rc_ops = {
    .assert		= npcm_rc_assert,
    .deassert	= npcm_rc_deassert,
    .status		= npcm_rc_status,
    };
#[no_mangle]
unsafe extern "C" fn npcm_clock_unregister_adev(_adev: *mut c_void) {
    static void npcm_clock_unregister_adev(void *_adev)
    {
    struct auxiliary_device *adev = _adev;
    auxiliary_device_delete(adev);
    auxiliary_device_uninit(adev);
    }
#[no_mangle]
unsafe extern "C" fn npcm_clock_adev_release(dev: *mut device) {
    static void npcm_clock_adev_release(struct device *dev)
    {
    struct auxiliary_device *adev = to_auxiliary_dev(dev);
    struct npcm_clock_adev *rdev = to_npcm_clock_adev(adev);
    kfree(rdev);
    }
    static struct auxiliary_device *npcm_clock_adev_alloc(struct npcm_rc_data *rst_data, char *clk_name)
    {
    struct npcm_clock_adev *rdev;
    struct auxiliary_device *adev;
    int ret;
    rdev = kzalloc_obj(*rdev);
    if (!rdev)
    return ERR_PTR(-ENOMEM);
    rdev.base = rst_data.base;
    adev = &rdev.adev;
    adev.name = clk_name;
    adev.dev.parent = rst_data.dev;
    adev.dev.release = npcm_clock_adev_release;
    adev.id = 555u;
    ret = auxiliary_device_init(adev);
    if (ret) {
    kfree(rdev);
    return ERR_PTR(ret);
    }
    return adev;
    }
#[no_mangle]
unsafe extern "C" fn npcm8xx_clock_controller_register(rst_data: *mut npcm_rc_data, clk_name: *mut c_char) -> c_int {
    static int npcm8xx_clock_controller_register(struct npcm_rc_data *rst_data, char *clk_name)
    {
    struct auxiliary_device *adev;
    int ret;
    adev = npcm_clock_adev_alloc(rst_data, clk_name);
    if (IS_ERR(adev))
    return PTR_ERR(adev);
    ret = auxiliary_device_add(adev);
    if (ret) {
    auxiliary_device_uninit(adev);
    return ret;
    }
    return devm_add_action_or_reset(rst_data.dev, npcm_clock_unregister_adev, adev);
    }
#[no_mangle]
unsafe extern "C" fn npcm_rc_probe(pdev: *mut platform_device) -> c_int {
    static int npcm_rc_probe(struct platform_device *pdev)
    {
    struct npcm_rc_data *rc;
    int ret;
    rc = devm_kzalloc(&pdev.dev, sizeof(*rc), GFP_KERNEL);
    if (!rc)
    return -ENOMEM;
    rc.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rc.base))
    return PTR_ERR(rc.base);
    spin_lock_init(&rc.lock);
    rc.rcdev.owner = THIS_MODULE;
    rc.rcdev.ops = &npcm_rc_ops;
    rc.rcdev.of_node = pdev.dev.of_node;
    rc.rcdev.of_reset_n_cells = 2;
    rc.rcdev.of_xlate = npcm_reset_xlate;
    rc.dev = &pdev.dev;
    ret = devm_reset_controller_register(&pdev.dev, &rc.rcdev);
    if (ret) {
    dev_err(&pdev.dev, "unable to register device\n");
    return ret;
    }
    if (npcm_usb_reset(pdev, rc))
    dev_warn(&pdev.dev, "NPCM USB reset failed, can cause issues with UDC and USB host\n");
    if (!of_property_read_u32(pdev.dev.of_node, "nuvoton,sw-reset-number",
    &rc.sw_reset_number)) {
    if (rc.sw_reset_number && rc.sw_reset_number < 5) {
    ret = devm_register_sys_off_handler(&pdev.dev, SYS_OFF_MODE_RESTART, 192,
    npcm_rc_restart, rc);
    if (ret) {
    dev_warn(&pdev.dev, "failed to register restart handler\n");
    return ret;
    }
    }
    }
    switch (rc.info.bmc_id) {
    case BMC_NPCM8XX:
    return npcm8xx_clock_controller_register(rc, "clk-npcm8xx");
    default:
    return 0;
    }
    }
    static struct platform_driver npcm_rc_driver = {
    .probe	= npcm_rc_probe,
    .driver	= {
    .name			= "npcm-reset",
    .of_match_table		= npcm_rc_match,
    .suppress_bind_attrs	= true,
    },
    };
    builtin_platform_driver(npcm_rc_driver);
