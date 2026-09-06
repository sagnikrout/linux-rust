//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/mt6779-keypad.c
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
// Copyright (C) 2022 MediaTek Inc.
// Author Fengping Yu <fengping.yu@mediatek.com>
//

pub const MTK_KPD_MEM: c_uint = 0x0004;
pub const MTK_KPD_DEBOUNCE: c_uint = 0x0018;

pub const MTK_KPD_DEBOUNCE_MAX_MS: c_int = 256;
pub const MTK_KPD_SEL: c_uint = 0x0020;

pub const MTK_KPD_NUM_MEMS: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6779_keypad {
    pub regmap: *mut regmap,
    pub input_dev: *mut input_dev,
    pub clk: *mut clk,
    pub n_rows: u32,
    pub n_cols: u32,
    void (*calc_row_col)(unsigned int key,
    pub col): *mut *mut unsigned int row, unsigned int,
    pub MTK_KPD_NUM_BITS): DECLARE_BITMAP(keymap_state,,
}

    static const struct regmap_config mt6779_keypad_regmap_cfg = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = sizeof(u32),
    .max_register = 36,
    };
#[no_mangle]
unsafe extern "C" fn mt6779_keypad_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mt6779_keypad_irq_handler(int irq, void *dev_id)
    {
    struct mt6779_keypad *keypad = dev_id;
    const unsigned short *keycode = keypad.input_dev.keycode;
    DECLARE_BITMAP(new_state, MTK_KPD_NUM_BITS);
    DECLARE_BITMAP(change, MTK_KPD_NUM_BITS);
    unsigned int bit_nr, key;
    unsigned int row, col;
    unsigned int scancode;
    let mut row_shift: c_uint = get_count_order(keypad.n_cols);
    bool pressed;
    regmap_bulk_read(keypad.regmap, MTK_KPD_MEM,
    new_state, MTK_KPD_NUM_MEMS);
    bitmap_xor(change, new_state, keypad.keymap_state, MTK_KPD_NUM_BITS);
    for_each_set_bit(bit_nr, change, MTK_KPD_NUM_BITS) {
//
// Registers are 32bits, but only bits [15:0] are used to
// indicate key status.
//
    if (bit_nr % 32 >= 16)
    continue;
    key = bit_nr / 32 * 16 + bit_nr % 32;
    keypad.calc_row_col(key, &row, &col);
    scancode = MATRIX_SCAN_CODE(row, col, row_shift);
// 1: not pressed, 0: pressed
    pressed = !test_bit(bit_nr, new_state);
    dev_dbg(&keypad.input_dev.dev, "%s",
    pressed ? "pressed" : "released");
    input_event(keypad.input_dev, EV_MSC, MSC_SCAN, scancode);
    input_report_key(keypad.input_dev, keycode[scancode], pressed);
    input_sync(keypad.input_dev);
    dev_dbg(&keypad.input_dev.dev,
    "report Linux keycode = %d\n", keycode[scancode]);
    }
    bitmap_copy(keypad.keymap_state, new_state, MTK_KPD_NUM_BITS);
    return IRQ_HANDLED;
    }
    static void mt6779_keypad_calc_row_col_single(unsigned int key,
    unsigned int *row,
    unsigned int *col)
    {
// row = key / 9;
// col = key % 9;
    }
    static void mt6779_keypad_calc_row_col_double(unsigned int key,
    unsigned int *row,
    unsigned int *col)
    {
// row = key / 13;
// col = (key % 13) / 2;
    }
#[no_mangle]
unsafe extern "C" fn mt6779_keypad_pdrv_probe(pdev: *mut platform_device) -> c_int {
    static int mt6779_keypad_pdrv_probe(struct platform_device *pdev)
    {
    struct mt6779_keypad *keypad;
    void __iomem *base;
    int irq;
    u32 debounce;
    u32 keys_per_group;
    bool wakeup;
    int error;
    keypad = devm_kzalloc(&pdev.dev, sizeof(*keypad), GFP_KERNEL);
    if (!keypad)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    keypad.regmap = devm_regmap_init_mmio(&pdev.dev, base,
    &mt6779_keypad_regmap_cfg);
    if (IS_ERR(keypad.regmap)) {
    dev_err(&pdev.dev,
    "regmap init failed:%pe\n", keypad.regmap);
    return PTR_ERR(keypad.regmap);
    }
    bitmap_fill(keypad.keymap_state, MTK_KPD_NUM_BITS);
    keypad.input_dev = devm_input_allocate_device(&pdev.dev);
    if (!keypad.input_dev) {
    dev_err(&pdev.dev, "Failed to allocate input dev\n");
    return -ENOMEM;
    }
    keypad.input_dev.name = MTK_KPD_NAME;
    keypad.input_dev.id.bustype = BUS_HOST;
    error = matrix_keypad_parse_properties(&pdev.dev, &keypad.n_rows,
    &keypad.n_cols);
    if (error) {
    dev_err(&pdev.dev, "Failed to parse keypad params\n");
    return error;
    }
    if (device_property_read_u32(&pdev.dev, "debounce-delay-ms",
    &debounce))
    debounce = 16;
    if (debounce > MTK_KPD_DEBOUNCE_MAX_MS) {
    dev_err(&pdev.dev,
    "Debounce time exceeds the maximum allowed time %dms\n",
    MTK_KPD_DEBOUNCE_MAX_MS);
    return -EINVAL;
    }
    if (device_property_read_u32(&pdev.dev, "mediatek,keys-per-group",
    &keys_per_group))
    keys_per_group = 1;
    switch (keys_per_group) {
    case 1:
    keypad.calc_row_col = mt6779_keypad_calc_row_col_single;
    break;
    case 2:
    keypad.calc_row_col = mt6779_keypad_calc_row_col_double;
    break;
    default:
    dev_err(&pdev.dev,
    "Invalid keys-per-group: %d\n", keys_per_group);
    return -EINVAL;
    }
    wakeup = device_property_read_bool(&pdev.dev, "wakeup-source");
    dev_dbg(&pdev.dev, "n_row=%d n_col=%d debounce=%d\n",
    keypad.n_rows, keypad.n_cols, debounce);
    error = matrix_keypad_build_keymap(core::ptr::null_mut(), core::ptr::null_mut(),
    keypad.n_rows, keypad.n_cols,
    core::ptr::null_mut(), keypad.input_dev);
    if (error) {
    dev_err(&pdev.dev, "Failed to build keymap\n");
    return error;
    }
    input_set_capability(keypad.input_dev, EV_MSC, MSC_SCAN);
    regmap_write(keypad.regmap, MTK_KPD_DEBOUNCE,
    (debounce * (1 << 5)) & MTK_KPD_DEBOUNCE_MASK);
    if (keys_per_group == 2)
    regmap_update_bits(keypad.regmap, MTK_KPD_SEL,
    MTK_KPD_SEL_DOUBLE_KP_MODE,
    MTK_KPD_SEL_DOUBLE_KP_MODE);
    regmap_update_bits(keypad.regmap, MTK_KPD_SEL, MTK_KPD_SEL_ROW,
    MTK_KPD_SEL_ROWMASK(keypad.n_rows));
    regmap_update_bits(keypad.regmap, MTK_KPD_SEL, MTK_KPD_SEL_COL,
    MTK_KPD_SEL_COLMASK(keypad.n_cols));
    keypad.clk = devm_clk_get_enabled(&pdev.dev, "kpd");
    if (IS_ERR(keypad.clk))
    return PTR_ERR(keypad.clk);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    error = devm_request_threaded_irq(&pdev.dev, irq,
    core::ptr::null_mut(), mt6779_keypad_irq_handler,
    IRQF_ONESHOT, MTK_KPD_NAME, keypad);
    if (error) {
    dev_err(&pdev.dev, "Failed to request IRQ#%d: %d\n",
    irq, error);
    return error;
    }
    error = input_register_device(keypad.input_dev);
    if (error) {
    dev_err(&pdev.dev, "Failed to register device\n");
    return error;
    }
    error = device_init_wakeup(&pdev.dev, wakeup);
    if (error)
    dev_warn(&pdev.dev, "device_init_wakeup() failed: %d\n",
    error);
    return 0;
    }
    static const struct of_device_id mt6779_keypad_of_match[] = {
    { .compatible = "mediatek,mt6779-keypad" },
    { .compatible = "mediatek,mt6873-keypad" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, mt6779_keypad_of_match);
    static struct platform_driver mt6779_keypad_pdrv = {
    .probe = mt6779_keypad_pdrv_probe,
    .driver = {
    .name = MTK_KPD_NAME,
    .of_match_table = mt6779_keypad_of_match,
    },
    };
    module_platform_driver(mt6779_keypad_pdrv);
    MODULE_AUTHOR("Mediatek Corporation");
    MODULE_DESCRIPTION("MTK Keypad (KPD) Driver");
    MODULE_LICENSE("GPL");
