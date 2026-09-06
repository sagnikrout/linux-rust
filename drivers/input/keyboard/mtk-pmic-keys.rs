//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/mtk-pmic-keys.c
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
// Copyright (C) 2017 MediaTek, Inc.
//
// Author: Chen Zhong <chen.zhong@mediatek.com>
//

pub const MTK_PMIC_PWRKEY_INDEX: c_int = 0;
pub const MTK_PMIC_HOMEKEY_INDEX: c_int = 1;
pub const MTK_PMIC_MAX_KEY_COUNT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pmic_keys_regs {
    pub deb_reg: u32,
    pub deb_mask: u32,
    pub intsel_reg: u32,
    pub intsel_mask: u32,
    pub rst_en_mask: u32,
}

    _intsel_reg, _intsel_mask, _rst_mask)		\
    {							\
    .deb_reg		= _deb_reg,		\
    .deb_mask		= _deb_mask,		\
    .intsel_reg		= _intsel_reg,		\
    .intsel_mask		= _intsel_mask,		\
    .rst_en_mask		= _rst_mask,		\
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pmic_regs {
    pub keys_regs: [mtk_pmic_keys_regs; MTK_PMIC_MAX_KEY_COUNT],
    pub pmic_rst_reg: u32,
    pub /: *mut *mut u32 rst_lprst_mask; / Long-press reset timeout bitmask,
    pub key_release_irq: bool,
}

    static const struct mtk_pmic_regs mt6397_regs = {
    .keys_regs[MTK_PMIC_PWRKEY_INDEX] =
    MTK_PMIC_KEYS_REGS(MT6397_CHRSTATUS,
    0x8, MT6397_INT_RSV, 0x10, MTK_PMIC_PWRKEY_RST),
    .keys_regs[MTK_PMIC_HOMEKEY_INDEX] =
    MTK_PMIC_KEYS_REGS(MT6397_OCSTATUS2,
    0x10, MT6397_INT_RSV, 0x8, MTK_PMIC_HOMEKEY_RST),
    .pmic_rst_reg = MT6397_TOP_RST_MISC,
    .rst_lprst_mask = MTK_PMIC_RST_DU_MASK,
    };
    static const struct mtk_pmic_regs mt6323_regs = {
    .keys_regs[MTK_PMIC_PWRKEY_INDEX] =
    MTK_PMIC_KEYS_REGS(MT6323_CHRSTATUS,
    0x2, MT6323_INT_MISC_CON, 0x10, MTK_PMIC_PWRKEY_RST),
    .keys_regs[MTK_PMIC_HOMEKEY_INDEX] =
    MTK_PMIC_KEYS_REGS(MT6323_CHRSTATUS,
    0x4, MT6323_INT_MISC_CON, 0x8, MTK_PMIC_HOMEKEY_RST),
    .pmic_rst_reg = MT6323_TOP_RST_MISC,
    .rst_lprst_mask = MTK_PMIC_RST_DU_MASK,
    };
    static const struct mtk_pmic_regs mt6331_regs = {
    .keys_regs[MTK_PMIC_PWRKEY_INDEX] =
    MTK_PMIC_KEYS_REGS(MT6331_TOPSTATUS, 0x2,
    MT6331_INT_MISC_CON, 0x4,
    MTK_PMIC_MT6331_PWRKEY_RST),
    .keys_regs[MTK_PMIC_HOMEKEY_INDEX] =
    MTK_PMIC_KEYS_REGS(MT6331_TOPSTATUS, 0x4,
    MT6331_INT_MISC_CON, 0x2,
    MTK_PMIC_MT6331_HOMEKEY_RST),
    .pmic_rst_reg = MT6331_TOP_RST_MISC,
    .rst_lprst_mask = MTK_PMIC_MT6331_RST_DU_MASK,
    };
    static const struct mtk_pmic_regs mt6357_regs = {
    .keys_regs[MTK_PMIC_PWRKEY_INDEX] =
    MTK_PMIC_KEYS_REGS(MT6357_TOPSTATUS,
    0x2, MT6357_PSC_TOP_INT_CON0, 0x5,
    MTK_PMIC_PWRKEY_RST),
    .keys_regs[MTK_PMIC_HOMEKEY_INDEX] =
    MTK_PMIC_KEYS_REGS(MT6357_TOPSTATUS,
    0x8, MT6357_PSC_TOP_INT_CON0, 0xa,
    MTK_PMIC_HOMEKEY_INDEX),
    .pmic_rst_reg = MT6357_TOP_RST_MISC,
    .rst_lprst_mask = MTK_PMIC_RST_DU_MASK,
    };
    static const struct mtk_pmic_regs mt6358_regs = {
    .keys_regs[MTK_PMIC_PWRKEY_INDEX] =
    MTK_PMIC_KEYS_REGS(MT6358_TOPSTATUS,
    0x2, MT6358_PSC_TOP_INT_CON0, 0x5,
    MTK_PMIC_PWRKEY_RST),
    .keys_regs[MTK_PMIC_HOMEKEY_INDEX] =
    MTK_PMIC_KEYS_REGS(MT6358_TOPSTATUS,
    0x8, MT6358_PSC_TOP_INT_CON0, 0xa,
    MTK_PMIC_HOMEKEY_RST),
    .pmic_rst_reg = MT6358_TOP_RST_MISC,
    .rst_lprst_mask = MTK_PMIC_RST_DU_MASK,
    .key_release_irq = true,
    };
    static const struct mtk_pmic_regs mt6359_regs = {
    .keys_regs[MTK_PMIC_PWRKEY_INDEX] =
    MTK_PMIC_KEYS_REGS(MT6359_TOPSTATUS,
    0x2, MT6359_PSC_TOP_INT_CON0, 0x5,
    MTK_PMIC_PWRKEY_RST),
    .keys_regs[MTK_PMIC_HOMEKEY_INDEX] =
    MTK_PMIC_KEYS_REGS(MT6359_TOPSTATUS,
    0x8, MT6359_PSC_TOP_INT_CON0, 0xa,
    MTK_PMIC_HOMEKEY_RST),
    .pmic_rst_reg = MT6359_TOP_RST_MISC,
    .rst_lprst_mask = MTK_PMIC_RST_DU_MASK,
    .key_release_irq = true,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pmic_keys_info {
    pub keys: *mut mtk_pmic_keys,
    pub regs: *const mtk_pmic_keys_regs,
    pub keycode: c_uint,
    pub irq: c_int,
    pub /: *mut *mut int irq_r; / optional: release irq if different,
    pub wakeup:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pmic_keys {
    pub input_dev: *mut input_dev,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub keys: [mtk_pmic_keys_info; MTK_PMIC_MAX_KEY_COUNT],
}

    enum mtk_pmic_keys_lp_mode {
    LP_DISABLE,
    LP_ONEKEY,
    LP_TWOKEY,
    };
    static void mtk_pmic_keys_lp_reset_setup(struct mtk_pmic_keys *keys,
    const struct mtk_pmic_regs *regs)
    {
    const struct mtk_pmic_keys_regs *kregs_home, *kregs_pwr;
    u32 long_press_mode, long_press_debounce;
    u32 value, mask;
    int error;
    kregs_home = &regs.keys_regs[MTK_PMIC_HOMEKEY_INDEX];
    kregs_pwr = &regs.keys_regs[MTK_PMIC_PWRKEY_INDEX];
    error = of_property_read_u32(keys.dev.of_node, "power-off-time-sec",
    &long_press_debounce);
    if (error)
    long_press_debounce = 0;
    mask = regs.rst_lprst_mask;
    value = long_press_debounce << (ffs(regs.rst_lprst_mask) - 1);
    error  = of_property_read_u32(keys.dev.of_node,
    "mediatek,long-press-mode",
    &long_press_mode);
    if (error)
    long_press_mode = LP_DISABLE;
    switch (long_press_mode) {
    case LP_TWOKEY:
    value |= kregs_home.rst_en_mask;
    fallthrough;
    case LP_ONEKEY:
    value |= kregs_pwr.rst_en_mask;
    fallthrough;
    case LP_DISABLE:
    mask |= kregs_home.rst_en_mask;
    mask |= kregs_pwr.rst_en_mask;
    break;
    default:
    break;
    }
    regmap_update_bits(keys.regmap, regs.pmic_rst_reg, mask, value);
    }
#[no_mangle]
unsafe extern "C" fn mtk_pmic_keys_irq_handler_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t mtk_pmic_keys_irq_handler_thread(int irq, void *data)
    {
    struct mtk_pmic_keys_info *info = data;
    u32 key_deb, pressed;
    regmap_read(info.keys.regmap, info.regs.deb_reg, &key_deb);
    key_deb &= info.regs.deb_mask;
    pressed = !key_deb;
    input_report_key(info.keys.input_dev, info.keycode, pressed);
    input_sync(info.keys.input_dev);
    dev_dbg(info.keys.dev, "(%s) key =%d using PMIC\n",
    pressed ? "pressed" : "released", info.keycode);
    return IRQ_HANDLED;
    }
    static int mtk_pmic_key_setup(struct mtk_pmic_keys *keys,
    struct mtk_pmic_keys_info *info)
    {
    int ret;
    info.keys = keys;
    ret = regmap_update_bits(keys.regmap, info.regs.intsel_reg,
    info.regs.intsel_mask,
    info.regs.intsel_mask);
    if (ret < 0)
    return ret;
    ret = devm_request_threaded_irq(keys.dev, info.irq, core::ptr::null_mut(),
    mtk_pmic_keys_irq_handler_thread,
    IRQF_ONESHOT | IRQF_TRIGGER_HIGH,
    "mtk-pmic-keys", info);
    if (ret) {
    dev_err(keys.dev, "Failed to request IRQ: %d: %d\n",
    info.irq, ret);
    return ret;
    }
    if (info.irq_r > 0) {
    ret = devm_request_threaded_irq(keys.dev, info.irq_r, core::ptr::null_mut(),
    mtk_pmic_keys_irq_handler_thread,
    IRQF_ONESHOT | IRQF_TRIGGER_HIGH,
    "mtk-pmic-keys", info);
    if (ret) {
    dev_err(keys.dev, "Failed to request IRQ_r: %d: %d\n",
    info.irq, ret);
    return ret;
    }
    }
    input_set_capability(keys.input_dev, EV_KEY, info.keycode);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_pmic_keys_suspend(dev: *mut device) -> c_int {
    static int mtk_pmic_keys_suspend(struct device *dev)
    {
    struct mtk_pmic_keys *keys = dev_get_drvdata(dev);
    int index;
    for (index = 0; index < MTK_PMIC_MAX_KEY_COUNT; index++) {
    if (keys.keys[index].wakeup) {
    enable_irq_wake(keys.keys[index].irq);
    if (keys.keys[index].irq_r > 0)
    enable_irq_wake(keys.keys[index].irq_r);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_pmic_keys_resume(dev: *mut device) -> c_int {
    static int mtk_pmic_keys_resume(struct device *dev)
    {
    struct mtk_pmic_keys *keys = dev_get_drvdata(dev);
    int index;
    for (index = 0; index < MTK_PMIC_MAX_KEY_COUNT; index++) {
    if (keys.keys[index].wakeup) {
    disable_irq_wake(keys.keys[index].irq);
    if (keys.keys[index].irq_r > 0)
    disable_irq_wake(keys.keys[index].irq_r);
    }
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(mtk_pmic_keys_pm_ops, mtk_pmic_keys_suspend,
    mtk_pmic_keys_resume);
    static const struct of_device_id of_mtk_pmic_keys_match_tbl[] = {
    {
    .compatible = "mediatek,mt6397-keys",
    .data = &mt6397_regs,
    }, {
    .compatible = "mediatek,mt6323-keys",
    .data = &mt6323_regs,
    }, {
    .compatible = "mediatek,mt6331-keys",
    .data = &mt6331_regs,
    }, {
    .compatible = "mediatek,mt6357-keys",
    .data = &mt6357_regs,
    }, {
    .compatible = "mediatek,mt6358-keys",
    .data = &mt6358_regs,
    }, {
    .compatible = "mediatek,mt6359-keys",
    .data = &mt6359_regs,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, of_mtk_pmic_keys_match_tbl);
#[no_mangle]
unsafe extern "C" fn mtk_pmic_keys_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_pmic_keys_probe(struct platform_device *pdev)
    {
    int error, index = 0;
    unsigned int keycount;
    struct mt6397_chip *pmic_chip = dev_get_drvdata(pdev.dev.parent);
    struct device_node *node = pdev.dev.of_node;
    static const char *const irqnames[] = { "powerkey", "homekey" };
    static const char *const irqnames_r[] = { "powerkey_r", "homekey_r" };
    struct mtk_pmic_keys *keys;
    const struct mtk_pmic_regs *mtk_pmic_regs;
    struct input_dev *input_dev;
    const struct of_device_id *of_id =
    of_match_device(of_mtk_pmic_keys_match_tbl, &pdev.dev);
    keys = devm_kzalloc(&pdev.dev, sizeof(*keys), GFP_KERNEL);
    if (!keys)
    return -ENOMEM;
    keys.dev = &pdev.dev;
    keys.regmap = pmic_chip.regmap;
    mtk_pmic_regs = of_id.data;
    keys.input_dev = input_dev = devm_input_allocate_device(keys.dev);
    if (!input_dev) {
    dev_err(keys.dev, "input allocate device fail.\n");
    return -ENOMEM;
    }
    input_dev.name = "mtk-pmic-keys";
    input_dev.id.bustype = BUS_HOST;
    input_dev.id.vendor = 0x0001;
    input_dev.id.product = 0x0001;
    input_dev.id.version = 0x0001;
    keycount = of_get_available_child_count(node);
    if (keycount > MTK_PMIC_MAX_KEY_COUNT ||
    keycount > ARRAY_SIZE(irqnames)) {
    dev_err(keys.dev, "too many keys defined (%d)\n", keycount);
    return -EINVAL;
    }
    for_each_child_of_node_scoped(node, child) {
    keys.keys[index].regs = &mtk_pmic_regs.keys_regs[index];
    keys.keys[index].irq =
    platform_get_irq_byname(pdev, irqnames[index]);
    if (keys.keys[index].irq < 0)
    return keys.keys[index].irq;
    if (mtk_pmic_regs.key_release_irq) {
    keys.keys[index].irq_r = platform_get_irq_byname(pdev,
    irqnames_r[index]);
    if (keys.keys[index].irq_r < 0)
    return keys.keys[index].irq_r;
    }
    error = of_property_read_u32(child,
    "linux,keycodes", &keys.keys[index].keycode);
    if (error) {
    dev_err(keys.dev,
    "failed to read key:%d linux,keycode property: %d\n",
    index, error);
    return error;
    }
    if (of_property_read_bool(child, "wakeup-source"))
    keys.keys[index].wakeup = true;
    error = mtk_pmic_key_setup(keys, &keys.keys[index]);
    if (error)
    return error;
    index++;
    }
    error = input_register_device(input_dev);
    if (error) {
    dev_err(&pdev.dev,
    "register input device failed (%d)\n", error);
    return error;
    }
    mtk_pmic_keys_lp_reset_setup(keys, mtk_pmic_regs);
    platform_set_drvdata(pdev, keys);
    return 0;
    }
    static struct platform_driver pmic_keys_pdrv = {
    .probe = mtk_pmic_keys_probe,
    .driver = {
    .name = "mtk-pmic-keys",
    .of_match_table = of_mtk_pmic_keys_match_tbl,
    .pm = pm_sleep_ptr(&mtk_pmic_keys_pm_ops),
    },
    };
    module_platform_driver(pmic_keys_pdrv);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Chen Zhong <chen.zhong@mediatek.com>");
    MODULE_DESCRIPTION("MTK pmic-keys driver v0.1");
