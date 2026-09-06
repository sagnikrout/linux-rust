//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/tc3589x-keypad.c
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
// Author: Jayeeta Banerjee <jayeeta.banerjee@stericsson.com>
// Author: Sundar Iyer <sundar.iyer@stericsson.com>
//
// TC35893 MFD Keypad Controller driver
//

// Maximum supported keypad matrix row/columns size
pub const TC3589x_MAX_KPROW: c_int = 8;
pub const TC3589x_MAX_KPCOL: c_int = 12;
// keypad related Constants
pub const TC3589x_MAX_DEBOUNCE_SETTLE: c_uint = 0xFF;
pub const DEDICATED_KEY_VAL: c_uint = 0xFF;
// Pull up/down masks
pub const TC3589x_NO_PULL_MASK: c_uint = 0x0;
pub const TC3589x_PULL_DOWN_MASK: c_uint = 0x1;
pub const TC3589x_PULL_UP_MASK: c_uint = 0x2;
pub const TC3589x_PULLUP_ALL_MASK: c_uint = 0xAA;

// Bit masks for IOCFG register
pub const IOCFG_BALLCFG: c_uint = 0x01;
pub const IOCFG_IG: c_uint = 0x08;
pub const KP_EVCODE_COL_MASK: c_uint = 0x0F;
pub const KP_EVCODE_ROW_MASK: c_uint = 0x70;
pub const KP_RELEASE_EVT_MASK: c_uint = 0x80;
pub const KP_ROW_SHIFT: c_int = 4;
pub const KP_NO_VALID_KEY_MASK: c_uint = 0x7F;
// bit masks for RESTCTRL register
pub const TC3589x_KBDRST: c_uint = 0x2;
pub const TC3589x_IRQRST: c_uint = 0x10;
pub const TC3589x_RESET_ALL: c_uint = 0x1B;
// KBDMFS register bit mask
pub const TC3589x_KBDMFS_EN: c_uint = 0x1;
// CLKEN register bitmask
pub const KPD_CLK_EN: c_uint = 0x1;
// RSTINTCLR register bit mask
pub const IRQ_CLEAR: c_uint = 0x1;
// bit masks for keyboard interrupts
pub const TC3589x_EVT_LOSS_INT: c_uint = 0x8;
pub const TC3589x_EVT_INT: c_uint = 0x4;
pub const TC3589x_KBD_LOSS_INT: c_uint = 0x2;
pub const TC3589x_KBD_INT: c_uint = 0x1;
// bit masks for keyboard interrupt clear
pub const TC3589x_EVT_INT_CLR: c_uint = 0x2;
pub const TC3589x_KBD_INT_CLR: c_uint = 0x1;
//
// struct tc3589x_keypad_platform_data - platform specific keypad data
// @keymap_data:        matrix scan code table for keycodes
// @krow:               mask for available rows, value is 0xFF
// @kcol:               mask for available columns, value is 0xFF
// @debounce_period:    platform specific debounce time
// @settle_time:        platform specific settle down time
// @irqtype:            type of interrupt, falling or rising edge
// @enable_wakeup:      specifies if keypad event can wake up system from sleep
// @no_autorepeat:      flag for auto repetition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc3589x_keypad_platform_data {
    pub keymap_data: *const matrix_keymap_data,
    pub krow: u8,
    pub kcol: u8,
    pub debounce_period: u8,
    pub settle_time: u8,
    pub irqtype: c_ulong,
    pub enable_wakeup: bool,
    pub no_autorepeat: bool,
}

//
// struct tc_keypad - data structure used by keypad driver
// @tc3589x:    pointer to tc35893
// @input:      pointer to input device object
// @board:      keypad platform device
// @krow:	number of rows
// @kcol:	number of columns
// @keymap:     matrix scan code table for keycodes
// @keypad_stopped: holds keypad status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_keypad {
    pub tc3589x: *mut tc3589x,
    pub input: *mut input_dev,
    pub board: *const tc3589x_keypad_platform_data,
    pub krow: c_uint,
    pub kcol: c_uint,
    pub keymap: *mut c_ushort,
    pub keypad_stopped: bool,
}

#[no_mangle]
unsafe extern "C" fn tc3589x_keypad_init_key_hardware(keypad: *mut tc_keypad) -> c_int {
    static int tc3589x_keypad_init_key_hardware(struct tc_keypad *keypad)
    {
    int ret;
    struct tc3589x *tc3589x = keypad.tc3589x;
    const struct tc3589x_keypad_platform_data *board = keypad.board;
// validate platform configuration
    if (board.kcol > TC3589x_MAX_KPCOL || board.krow > TC3589x_MAX_KPROW)
    return -EINVAL;
// configure KBDSIZE 4 LSbits for cols and 4 MSbits for rows
    ret = tc3589x_reg_write(tc3589x, TC3589x_KBDSIZE,
    (board.krow << KP_ROW_SHIFT) | board.kcol);
    if (ret < 0)
    return ret;
// configure dedicated key config, no dedicated key selected
    ret = tc3589x_reg_write(tc3589x, TC3589x_KBCFG_LSB, DEDICATED_KEY_VAL);
    if (ret < 0)
    return ret;
    ret = tc3589x_reg_write(tc3589x, TC3589x_KBCFG_MSB, DEDICATED_KEY_VAL);
    if (ret < 0)
    return ret;
// Configure settle time
    ret = tc3589x_reg_write(tc3589x, TC3589x_KBDSETTLE_REG,
    board.settle_time);
    if (ret < 0)
    return ret;
// Configure debounce time
    ret = tc3589x_reg_write(tc3589x, TC3589x_KBDBOUNCE,
    board.debounce_period);
    if (ret < 0)
    return ret;
// Start of initialise keypad GPIOs
    ret = tc3589x_set_bits(tc3589x, TC3589x_IOCFG, 0x0, IOCFG_IG);
    if (ret < 0)
    return ret;
// Configure pull-up resistors for all row GPIOs
    ret = tc3589x_reg_write(tc3589x, TC3589x_IOPULLCFG0_LSB,
    TC3589x_PULLUP_ALL_MASK);
    if (ret < 0)
    return ret;
    ret = tc3589x_reg_write(tc3589x, TC3589x_IOPULLCFG0_MSB,
    TC3589x_PULLUP_ALL_MASK);
    if (ret < 0)
    return ret;
// Configure pull-up resistors for all column GPIOs
    ret = tc3589x_reg_write(tc3589x, TC3589x_IOPULLCFG1_LSB,
    TC3589x_PULLUP_ALL_MASK);
    if (ret < 0)
    return ret;
    ret = tc3589x_reg_write(tc3589x, TC3589x_IOPULLCFG1_MSB,
    TC3589x_PULLUP_ALL_MASK);
    if (ret < 0)
    return ret;
    ret = tc3589x_reg_write(tc3589x, TC3589x_IOPULLCFG2_LSB,
    TC3589x_PULLUP_ALL_MASK);
    return ret;
    }
pub const TC35893_DATA_REGS: c_int = 4;
pub const TC35893_KEYCODE_FIFO_EMPTY: c_uint = 0x7f;
pub const TC35893_KEYCODE_FIFO_CLEAR: c_uint = 0xff;
pub const TC35893_KEYPAD_ROW_SHIFT: c_uint = 0x3;
#[no_mangle]
unsafe extern "C" fn tc3589x_keypad_irq(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t tc3589x_keypad_irq(int irq, void *dev)
    {
    struct tc_keypad *keypad = dev;
    struct tc3589x *tc3589x = keypad.tc3589x;
    u8 i, row_index, col_index, kbd_code, up;
    u8 code;
    for (i = 0; i < TC35893_DATA_REGS * 2; i++) {
    kbd_code = tc3589x_reg_read(tc3589x, TC3589x_EVTCODE_FIFO);
// loop till fifo is empty and no more keys are pressed
    if (kbd_code == TC35893_KEYCODE_FIFO_EMPTY ||
    kbd_code == TC35893_KEYCODE_FIFO_CLEAR)
    continue;
// valid key is found
    col_index = kbd_code & KP_EVCODE_COL_MASK;
    row_index = (kbd_code & KP_EVCODE_ROW_MASK) >> KP_ROW_SHIFT;
    code = MATRIX_SCAN_CODE(row_index, col_index,
    TC35893_KEYPAD_ROW_SHIFT);
    up = kbd_code & KP_RELEASE_EVT_MASK;
    input_event(keypad.input, EV_MSC, MSC_SCAN, code);
    input_report_key(keypad.input, keypad.keymap[code], !up);
    input_sync(keypad.input);
    }
// clear IRQ
    tc3589x_set_bits(tc3589x, TC3589x_KBDIC,
    0x0, TC3589x_EVT_INT_CLR | TC3589x_KBD_INT_CLR);
// enable IRQ
    tc3589x_set_bits(tc3589x, TC3589x_KBDMSK,
    0x0, TC3589x_EVT_LOSS_INT | TC3589x_EVT_INT);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tc3589x_keypad_enable(keypad: *mut tc_keypad) -> c_int {
    static int tc3589x_keypad_enable(struct tc_keypad *keypad)
    {
    struct tc3589x *tc3589x = keypad.tc3589x;
    int ret;
// pull the keypad module out of reset
    ret = tc3589x_set_bits(tc3589x, TC3589x_RSTCTRL, TC3589x_KBDRST, 0x0);
    if (ret < 0)
    return ret;
// configure KBDMFS
    ret = tc3589x_set_bits(tc3589x, TC3589x_KBDMFS, 0x0, TC3589x_KBDMFS_EN);
    if (ret < 0)
    return ret;
// enable the keypad clock
    ret = tc3589x_set_bits(tc3589x, TC3589x_CLKEN, 0x0, KPD_CLK_EN);
    if (ret < 0)
    return ret;
// clear pending IRQs
    ret =  tc3589x_set_bits(tc3589x, TC3589x_RSTINTCLR, 0x0, 0x1);
    if (ret < 0)
    return ret;
// enable the IRQs
    ret = tc3589x_set_bits(tc3589x, TC3589x_KBDMSK, 0x0,
    TC3589x_EVT_LOSS_INT | TC3589x_EVT_INT);
    if (ret < 0)
    return ret;
    keypad.keypad_stopped = false;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tc3589x_keypad_disable(keypad: *mut tc_keypad) -> c_int {
    static int tc3589x_keypad_disable(struct tc_keypad *keypad)
    {
    struct tc3589x *tc3589x = keypad.tc3589x;
    int ret;
// clear IRQ
    ret = tc3589x_set_bits(tc3589x, TC3589x_KBDIC,
    0x0, TC3589x_EVT_INT_CLR | TC3589x_KBD_INT_CLR);
    if (ret < 0)
    return ret;
// disable all interrupts
    ret = tc3589x_set_bits(tc3589x, TC3589x_KBDMSK,
    ~(TC3589x_EVT_LOSS_INT | TC3589x_EVT_INT), 0x0);
    if (ret < 0)
    return ret;
// disable the keypad module
    ret = tc3589x_set_bits(tc3589x, TC3589x_CLKEN, 0x1, 0x0);
    if (ret < 0)
    return ret;
// put the keypad module into reset
    ret = tc3589x_set_bits(tc3589x, TC3589x_RSTCTRL, TC3589x_KBDRST, 0x1);
    keypad.keypad_stopped = true;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tc3589x_keypad_open(input: *mut input_dev) -> c_int {
    static int tc3589x_keypad_open(struct input_dev *input)
    {
    int error;
    struct tc_keypad *keypad = input_get_drvdata(input);
// enable the keypad module
    error = tc3589x_keypad_enable(keypad);
    if (error < 0) {
    dev_err(&input.dev, "failed to enable keypad module\n");
    return error;
    }
    error = tc3589x_keypad_init_key_hardware(keypad);
    if (error < 0) {
    dev_err(&input.dev, "failed to configure keypad module\n");
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tc3589x_keypad_close(input: *mut input_dev) {
    static void tc3589x_keypad_close(struct input_dev *input)
    {
    struct tc_keypad *keypad = input_get_drvdata(input);
// disable the keypad module
    tc3589x_keypad_disable(keypad);
    }
    static const struct tc3589x_keypad_platform_data *
    tc3589x_keypad_of_probe(struct device *dev)
    {
    struct device_node *np = dev.of_node;
    struct tc3589x_keypad_platform_data *plat;
    u32 cols, rows;
    u32 debounce_ms;
    if (!np)
    return ERR_PTR(-ENODEV);
    plat = devm_kzalloc(dev, sizeof(*plat), GFP_KERNEL);
    if (!plat)
    return ERR_PTR(-ENOMEM);
    of_property_read_u32(np, "keypad,num-columns", &cols);
    of_property_read_u32(np, "keypad,num-rows", &rows);
    plat.kcol = (u8) cols;
    plat.krow = (u8) rows;
    if (!plat.krow || !plat.kcol ||
    plat.krow > TC_KPD_ROWS || plat.kcol > TC_KPD_COLUMNS) {
    dev_err(dev,
    "keypad columns/rows not properly specified (%ux%u)\n",
    plat.kcol, plat.krow);
    return ERR_PTR(-EINVAL);
    }
    if (!of_property_present(np, "linux,keymap")) {
    dev_err(dev, "property linux,keymap not found\n");
    return ERR_PTR(-ENOENT);
    }
    plat.no_autorepeat = of_property_read_bool(np, "linux,no-autorepeat");
    plat.enable_wakeup = of_property_read_bool(np, "wakeup-source") ||
// legacy name
    of_property_read_bool(np, "linux,wakeup");
// The custom delay format is ms/16
    of_property_read_u32(np, "debounce-delay-ms", &debounce_ms);
    if (debounce_ms)
    plat.debounce_period = debounce_ms * 16;
    else
    plat.debounce_period = TC_KPD_DEBOUNCE_PERIOD;
    plat.settle_time = TC_KPD_SETTLE_TIME;
// FIXME: should be property of the IRQ resource?
    plat.irqtype = IRQF_TRIGGER_FALLING;
    return plat;
    }
#[no_mangle]
unsafe extern "C" fn tc3589x_keypad_probe(pdev: *mut platform_device) -> c_int {
    static int tc3589x_keypad_probe(struct platform_device *pdev)
    {
    struct tc3589x *tc3589x = dev_get_drvdata(pdev.dev.parent);
    struct tc_keypad *keypad;
    struct input_dev *input;
    const struct tc3589x_keypad_platform_data *plat;
    int error, irq;
    plat = tc3589x_keypad_of_probe(&pdev.dev);
    if (IS_ERR(plat)) {
    dev_err(&pdev.dev, "invalid keypad platform data\n");
    return PTR_ERR(plat);
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    keypad = devm_kzalloc(&pdev.dev, sizeof(struct tc_keypad),
    GFP_KERNEL);
    if (!keypad)
    return -ENOMEM;
    input = devm_input_allocate_device(&pdev.dev);
    if (!input) {
    dev_err(&pdev.dev, "failed to allocate input device\n");
    return -ENOMEM;
    }
    keypad.board = plat;
    keypad.input = input;
    keypad.tc3589x = tc3589x;
    input.id.bustype = BUS_I2C;
    input.name = pdev.name;
    input.dev.parent = &pdev.dev;
    input.open = tc3589x_keypad_open;
    input.close = tc3589x_keypad_close;
    error = matrix_keypad_build_keymap(plat.keymap_data, core::ptr::null_mut(),
    TC3589x_MAX_KPROW, TC3589x_MAX_KPCOL,
    core::ptr::null_mut(), input);
    if (error) {
    dev_err(&pdev.dev, "Failed to build keymap\n");
    return error;
    }
    keypad.keymap = input.keycode;
    input_set_capability(input, EV_MSC, MSC_SCAN);
    if (!plat.no_autorepeat)
    __set_bit(EV_REP, input.evbit);
    input_set_drvdata(input, keypad);
    tc3589x_keypad_disable(keypad);
    error = devm_request_threaded_irq(&pdev.dev, irq,
    core::ptr::null_mut(), tc3589x_keypad_irq,
    plat.irqtype | IRQF_ONESHOT,
    "tc3589x-keypad", keypad);
    if (error) {
    dev_err(&pdev.dev,
    "Could not allocate irq %d,error %d\n",
    irq, error);
    return error;
    }
    error = input_register_device(input);
    if (error) {
    dev_err(&pdev.dev, "Could not register input device\n");
    return error;
    }
// let platform decide if keypad is a wakeup source or not
    device_init_wakeup(&pdev.dev, plat.enable_wakeup);
    device_set_wakeup_capable(&pdev.dev, plat.enable_wakeup);
    platform_set_drvdata(pdev, keypad);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tc3589x_keypad_suspend(dev: *mut device) -> c_int {
    static int tc3589x_keypad_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct tc_keypad *keypad = platform_get_drvdata(pdev);
    let mut irq: c_int = platform_get_irq(pdev, 0);
// keypad is already off; we do nothing
    if (keypad.keypad_stopped)
    return 0;
// if device is not a wakeup source, disable it for powersave
    if (!device_may_wakeup(&pdev.dev))
    tc3589x_keypad_disable(keypad);
    else
    enable_irq_wake(irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tc3589x_keypad_resume(dev: *mut device) -> c_int {
    static int tc3589x_keypad_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct tc_keypad *keypad = platform_get_drvdata(pdev);
    let mut irq: c_int = platform_get_irq(pdev, 0);
    if (!keypad.keypad_stopped)
    return 0;
// enable the device to resume normal operations
    if (!device_may_wakeup(&pdev.dev))
    tc3589x_keypad_enable(keypad);
    else
    disable_irq_wake(irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(tc3589x_keypad_dev_pm_ops,
    tc3589x_keypad_suspend, tc3589x_keypad_resume);
    static struct platform_driver tc3589x_keypad_driver = {
    .driver	= {
    .name	= "tc3589x-keypad",
    .pm	= pm_sleep_ptr(&tc3589x_keypad_dev_pm_ops),
    },
    .probe	= tc3589x_keypad_probe,
    };
    module_platform_driver(tc3589x_keypad_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Jayeeta Banerjee/Sundar Iyer");
    MODULE_DESCRIPTION("TC35893 Keypad Driver");
    MODULE_ALIAS("platform:tc3589x-keypad");
