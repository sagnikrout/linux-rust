//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/stmpe-keypad.c
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
// Copyright (C) ST-Ericsson SA 2010
//
// Author: Rabin Vincent <rabin.vincent@stericsson.com> for ST-Ericsson
//

// These are at the same addresses in all STMPE variants
pub const STMPE_KPC_COL: c_uint = 0x60;
pub const STMPE_KPC_ROW_MSB: c_uint = 0x61;
pub const STMPE_KPC_ROW_LSB: c_uint = 0x62;
pub const STMPE_KPC_CTRL_MSB: c_uint = 0x63;
pub const STMPE_KPC_CTRL_LSB: c_uint = 0x64;
pub const STMPE_KPC_COMBI_KEY_0: c_uint = 0x65;
pub const STMPE_KPC_COMBI_KEY_1: c_uint = 0x66;
pub const STMPE_KPC_COMBI_KEY_2: c_uint = 0x67;
pub const STMPE_KPC_DATA_BYTE0: c_uint = 0x68;
pub const STMPE_KPC_DATA_BYTE1: c_uint = 0x69;
pub const STMPE_KPC_DATA_BYTE2: c_uint = 0x6a;
pub const STMPE_KPC_DATA_BYTE3: c_uint = 0x6b;
pub const STMPE_KPC_DATA_BYTE4: c_uint = 0x6c;

pub const STMPE_KPC_ROW_MSB_ROWS: c_uint = 0xff;

pub const STMPE_KPC_DATA_NOKEY_MASK: c_uint = 0x78;
pub const STMPE_KEYPAD_MAX_DEBOUNCE: c_int = 127;
pub const STMPE_KEYPAD_MAX_SCAN_COUNT: c_int = 15;
pub const STMPE_KEYPAD_MAX_ROWS: c_int = 8;
pub const STMPE_KEYPAD_MAX_COLS: c_int = 8;
pub const STMPE_KEYPAD_ROW_SHIFT: c_int = 3;

    (STMPE_KEYPAD_MAX_ROWS * STMPE_KEYPAD_MAX_COLS)
pub const STMPE1601_NUM_DATA: c_int = 5;
pub const STMPE2401_NUM_DATA: c_int = 3;
pub const STMPE2403_NUM_DATA: c_int = 5;
// Make sure it covers all cases above
pub const MAX_NUM_DATA: c_int = 5;
//
// struct stmpe_keypad_variant - model-specific attributes
// @auto_increment: whether the KPC_DATA_BYTE register address
// auto-increments on multiple read
// @set_pullup: whether the pins need to have their pull-ups set
// @num_data: number of data bytes
// @num_normal_data: number of normal keys' data bytes
// @max_cols: maximum number of columns supported
// @max_rows: maximum number of rows supported
// @col_gpios: bitmask of gpios which can be used for columns
// @row_gpios: bitmask of gpios which can be used for rows
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmpe_keypad_variant {
    pub auto_increment: bool,
    pub set_pullup: bool,
    pub num_data: c_int,
    pub num_normal_data: c_int,
    pub max_cols: c_int,
    pub max_rows: c_int,
    pub col_gpios: c_uint,
    pub row_gpios: c_uint,
}

    static const struct stmpe_keypad_variant stmpe_keypad_variants[] = {
    [STMPE1601] = {
    .auto_increment		= true,
    .num_data		= STMPE1601_NUM_DATA,
    .num_normal_data	= 3,
    .max_cols		= 8,
    .max_rows		= 8,
    .col_gpios		= 0x000ff,	/* GPIO 0 - 7 */
    .row_gpios		= 0x0ff00,	/* GPIO 8 - 15 */
    },
    [STMPE2401] = {
    .auto_increment		= false,
    .set_pullup		= true,
    .num_data		= STMPE2401_NUM_DATA,
    .num_normal_data	= 2,
    .max_cols		= 8,
    .max_rows		= 12,
    .col_gpios		= 0x0000ff,	/* GPIO 0 - 7*/
    .row_gpios		= 0x1f7f00,	/* GPIO 8-14, 16-20 */
    },
    [STMPE2403] = {
    .auto_increment		= true,
    .set_pullup		= true,
    .num_data		= STMPE2403_NUM_DATA,
    .num_normal_data	= 3,
    .max_cols		= 8,
    .max_rows		= 12,
    .col_gpios		= 0x0000ff,	/* GPIO 0 - 7*/
    .row_gpios		= 0x1fef00,	/* GPIO 8-14, 16-20 */
    },
    };
//
// struct stmpe_keypad - STMPE keypad state container
// @stmpe: pointer to parent STMPE device
// @input: spawned input device
// @variant: STMPE variant
// @debounce_ms: debounce interval, in ms.  Maximum is
// %STMPE_KEYPAD_MAX_DEBOUNCE.
// @scan_count: number of key scanning cycles to confirm key data.
// Maximum is %STMPE_KEYPAD_MAX_SCAN_COUNT.
// @no_autorepeat: disable key autorepeat
// @rows: bitmask for the rows
// @cols: bitmask for the columns
// @keymap: the keymap
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmpe_keypad {
    pub stmpe: *mut stmpe,
    pub input: *mut input_dev,
    pub variant: *const stmpe_keypad_variant,
    pub debounce_ms: c_uint,
    pub scan_count: c_uint,
    pub no_autorepeat: bool,
    pub rows: c_uint,
    pub cols: c_uint,
    pub keymap: [c_ushort; STMPE_KEYPAD_KEYMAP_MAX_SIZE],
}

#[no_mangle]
unsafe extern "C" fn stmpe_keypad_read_data(keypad: *mut stmpe_keypad, data: *mut u8) -> c_int {
    static int stmpe_keypad_read_data(struct stmpe_keypad *keypad, u8 *data)
    {
    const struct stmpe_keypad_variant *variant = keypad.variant;
    struct stmpe *stmpe = keypad.stmpe;
    int ret;
    int i;
    if (variant.auto_increment)
    return stmpe_block_read(stmpe, STMPE_KPC_DATA_BYTE0,
    variant.num_data, data);
    for (i = 0; i < variant.num_data; i++) {
    ret = stmpe_reg_read(stmpe, STMPE_KPC_DATA_BYTE0 + i);
    if (ret < 0)
    return ret;
    data[i] = ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmpe_keypad_irq(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t stmpe_keypad_irq(int irq, void *dev)
    {
    struct stmpe_keypad *keypad = dev;
    struct input_dev *input = keypad.input;
    const struct stmpe_keypad_variant *variant = keypad.variant;
    u8 fifo[MAX_NUM_DATA];
    int ret;
    int i;
    ret = stmpe_keypad_read_data(keypad, fifo);
    if (ret < 0)
    return IRQ_NONE;
    for (i = 0; i < variant.num_normal_data; i++) {
    let mut data: u8 = fifo[i];
    let mut row: c_int = (data & STMPE_KPC_DATA_ROW) >> 3;
    let mut col: c_int = data & STMPE_KPC_DATA_COL;
    let mut code: c_int = MATRIX_SCAN_CODE(row, col, STMPE_KEYPAD_ROW_SHIFT);
    let mut up: bool = data & STMPE_KPC_DATA_UP;
    if ((data & STMPE_KPC_DATA_NOKEY_MASK)
    == STMPE_KPC_DATA_NOKEY_MASK)
    continue;
    input_event(input, EV_MSC, MSC_SCAN, code);
    input_report_key(input, keypad.keymap[code], !up);
    input_sync(input);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn stmpe_keypad_altfunc_init(keypad: *mut stmpe_keypad) -> c_int {
    static int stmpe_keypad_altfunc_init(struct stmpe_keypad *keypad)
    {
    const struct stmpe_keypad_variant *variant = keypad.variant;
    let mut col_gpios: c_uint = variant.col_gpios;
    let mut row_gpios: c_uint = variant.row_gpios;
    struct stmpe *stmpe = keypad.stmpe;
    let mut pureg: u8 = stmpe.regs[STMPE_IDX_GPPUR_LSB];
    let mut pins: c_uint = 0;
    let mut pu_pins: c_uint = 0;
    int ret;
    int i;
//
// Figure out which pins need to be set to the keypad alternate
// function.
//
// {cols,rows}_gpios are bitmasks of which pins on the chip can be used
// for the keypad.
//
// keypad->{cols,rows} are a bitmask of which pins (of the ones useable
// for the keypad) are used on the board.
//
    for (i = 0; i < variant.max_cols; i++) {
    let mut num: c_int = __ffs(col_gpios);
    if (keypad.cols & (1 << i)) {
    pins |= 1 << num;
    pu_pins |= 1 << num;
    }
    col_gpios &= ~(1 << num);
    }
    for (i = 0; i < variant.max_rows; i++) {
    let mut num: c_int = __ffs(row_gpios);
    if (keypad.rows & (1 << i))
    pins |= 1 << num;
    row_gpios &= ~(1 << num);
    }
    ret = stmpe_set_altfunc(stmpe, pins, STMPE_BLOCK_KEYPAD);
    if (ret)
    return ret;
//
// On STMPE24xx, set pin bias to pull-up on all keypad input
// pins (columns), this incidentally happen to be maximum 8 pins
// and placed at GPIO0-7 so only the LSB of the pull up register
// ever needs to be written.
//
    if (variant.set_pullup) {
    u8 val;
    ret = stmpe_reg_read(stmpe, pureg);
    if (ret)
    return ret;
// Do not touch unused pins, may be used for GPIO
    val = ret & ~pu_pins;
    val |= pu_pins;
    ret = stmpe_reg_write(stmpe, pureg, val);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmpe_keypad_chip_init(keypad: *mut stmpe_keypad) -> c_int {
    static int stmpe_keypad_chip_init(struct stmpe_keypad *keypad)
    {
    const struct stmpe_keypad_variant *variant = keypad.variant;
    struct stmpe *stmpe = keypad.stmpe;
    int ret;
    if (keypad.debounce_ms > STMPE_KEYPAD_MAX_DEBOUNCE)
    return -EINVAL;
    if (keypad.scan_count > STMPE_KEYPAD_MAX_SCAN_COUNT)
    return -EINVAL;
    ret = stmpe_enable(stmpe, STMPE_BLOCK_KEYPAD);
    if (ret < 0)
    return ret;
    ret = stmpe_keypad_altfunc_init(keypad);
    if (ret < 0)
    return ret;
    ret = stmpe_reg_write(stmpe, STMPE_KPC_COL, keypad.cols);
    if (ret < 0)
    return ret;
    ret = stmpe_reg_write(stmpe, STMPE_KPC_ROW_LSB, keypad.rows);
    if (ret < 0)
    return ret;
    if (variant.max_rows > 8) {
    ret = stmpe_set_bits(stmpe, STMPE_KPC_ROW_MSB,
    STMPE_KPC_ROW_MSB_ROWS,
    keypad.rows >> 8);
    if (ret < 0)
    return ret;
    }
    ret = stmpe_set_bits(stmpe, STMPE_KPC_CTRL_MSB,
    STMPE_KPC_CTRL_MSB_SCAN_COUNT,
    keypad.scan_count << 4);
    if (ret < 0)
    return ret;
    return stmpe_set_bits(stmpe, STMPE_KPC_CTRL_LSB,
    STMPE_KPC_CTRL_LSB_SCAN |
    STMPE_KPC_CTRL_LSB_DEBOUNCE,
    STMPE_KPC_CTRL_LSB_SCAN |
    (keypad.debounce_ms << 1));
    }
    static void stmpe_keypad_fill_used_pins(struct stmpe_keypad *keypad,
    u32 used_rows, u32 used_cols)
    {
    int row, col;
    for (row = 0; row < used_rows; row++) {
    for (col = 0; col < used_cols; col++) {
    int code = MATRIX_SCAN_CODE(row, col,
    STMPE_KEYPAD_ROW_SHIFT);
    if (keypad.keymap[code] != KEY_RESERVED) {
    keypad.rows |= 1 << row;
    keypad.cols |= 1 << col;
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn stmpe_keypad_probe(pdev: *mut platform_device) -> c_int {
    static int stmpe_keypad_probe(struct platform_device *pdev)
    {
    struct stmpe *stmpe = dev_get_drvdata(pdev.dev.parent);
    struct device_node *np = pdev.dev.of_node;
    struct stmpe_keypad *keypad;
    struct input_dev *input;
    u32 rows;
    u32 cols;
    int error;
    int irq;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    keypad = devm_kzalloc(&pdev.dev, sizeof(struct stmpe_keypad),
    GFP_KERNEL);
    if (!keypad)
    return -ENOMEM;
    keypad.stmpe = stmpe;
    keypad.variant = &stmpe_keypad_variants[stmpe.partnum];
    of_property_read_u32(np, "debounce-interval", &keypad.debounce_ms);
    of_property_read_u32(np, "st,scan-count", &keypad.scan_count);
    keypad.no_autorepeat = of_property_read_bool(np, "st,no-autorepeat");
    input = devm_input_allocate_device(&pdev.dev);
    if (!input)
    return -ENOMEM;
    input.name = "STMPE keypad";
    input.id.bustype = BUS_I2C;
    input.dev.parent = &pdev.dev;
    error = matrix_keypad_parse_properties(&pdev.dev, &rows, &cols);
    if (error)
    return error;
    error = matrix_keypad_build_keymap(core::ptr::null_mut(), core::ptr::null_mut(), rows, cols,
    keypad.keymap, input);
    if (error)
    return error;
    input_set_capability(input, EV_MSC, MSC_SCAN);
    if (!keypad.no_autorepeat)
    __set_bit(EV_REP, input.evbit);
    stmpe_keypad_fill_used_pins(keypad, rows, cols);
    keypad.input = input;
    error = stmpe_keypad_chip_init(keypad);
    if (error < 0)
    return error;
    error = devm_request_threaded_irq(&pdev.dev, irq,
    core::ptr::null_mut(), stmpe_keypad_irq,
    IRQF_ONESHOT, "stmpe-keypad", keypad);
    if (error) {
    dev_err(&pdev.dev, "unable to get irq: %d\n", error);
    return error;
    }
    error = input_register_device(input);
    if (error) {
    dev_err(&pdev.dev,
    "unable to register input device: %d\n", error);
    return error;
    }
    platform_set_drvdata(pdev, keypad);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmpe_keypad_remove(pdev: *mut platform_device) {
    static void stmpe_keypad_remove(struct platform_device *pdev)
    {
    struct stmpe_keypad *keypad = platform_get_drvdata(pdev);
    stmpe_disable(keypad.stmpe, STMPE_BLOCK_KEYPAD);
    }
    static struct platform_driver stmpe_keypad_driver = {
    .driver.name	= "stmpe-keypad",
    .probe		= stmpe_keypad_probe,
    .remove		= stmpe_keypad_remove,
    };
    module_platform_driver(stmpe_keypad_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("STMPExxxx keypad driver");
    MODULE_AUTHOR("Rabin Vincent <rabin.vincent@stericsson.com>");
