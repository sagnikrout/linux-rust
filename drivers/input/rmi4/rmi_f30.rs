//! Automatically rewritten from C to Rust
//! Source: drivers/input/rmi4/rmi_f30.c
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
// Copyright (c) 2012-2016 Synaptics Incorporated
//

pub const RMI_F30_QUERY_SIZE: c_int = 2;
// Defs for Query 0
pub const RMI_F30_EXTENDED_PATTERNS: c_uint = 0x01;

// Defs for Query 1
pub const RMI_F30_GPIO_LED_COUNT: c_uint = 0x1F;
// Defs for Control Registers
pub const RMI_F30_CTRL_1_GPIO_DEBOUNCE: c_uint = 0x01;

pub const RMI_F30_CTRL_10_NUM_MECH_MOUSE_BTNS: c_uint = 0x03;
pub const RMI_F30_CTRL_MAX_REGS: c_int = 32;

pub const RMI_F30_CTRL_MAX_REG_BLOCKS: c_int = 11;

    + 1				\
    + RMI_F30_CTRL_MAX_BYTES	\
    + RMI_F30_CTRL_MAX_BYTES	\
    + RMI_F30_CTRL_MAX_BYTES	\
    + 6				\
    + RMI_F30_CTRL_MAX_REGS		\
    + RMI_F30_CTRL_MAX_REGS		\
    + RMI_F30_CTRL_MAX_BYTES	\
    + 1				\
    + 1)
pub const TRACKSTICK_RANGE_START: c_int = 3;
pub const TRACKSTICK_RANGE_END: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_f30_ctrl_data {
    pub address: c_int,
    pub length: c_int,
    pub regs: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f30_data {
// Query Data
    pub has_extended_pattern: bool,
    pub has_mappable_buttons: bool,
    pub has_led: bool,
    pub has_gpio: bool,
    pub has_haptic: bool,
    pub has_gpio_driver_control: bool,
    pub has_mech_mouse_btns: bool,
    pub gpioled_count: u8,
    pub register_count: u8,
// Control Register Data
    pub ctrl: [rmi_f30_ctrl_data; RMI_F30_CTRL_MAX_REG_BLOCKS],
    pub ctrl_regs: [u8; RMI_F30_CTRL_REGS_MAX_SIZE],
    pub ctrl_regs_size: u32,
    pub data_regs: [u8; RMI_F30_CTRL_MAX_BYTES],
    pub gpioled_key_map: *mut u16,
    pub input: *mut input_dev,
    pub f03: *mut rmi_function,
    pub trackstick_buttons: bool,
}

    static int rmi_f30_read_control_parameters(struct rmi_function *fn,
    struct f30_data *f30)
    {
    int error;
    error = rmi_read_block(fn.rmi_dev, fn.fd.control_base_addr,
    f30.ctrl_regs, f30.ctrl_regs_size);
    if (error) {
    dev_err(&fn.dev,
    "%s: Could not read control registers at 0x%x: %d\n",
    __func__, fn.fd.control_base_addr, error);
    return error;
    }
    return 0;
    }
    static void rmi_f30_report_button(struct rmi_function *fn,
    struct f30_data *f30, unsigned int button)
    {
    let mut reg_num: c_uint = button >> 3;
    let mut bit_num: c_uint = button & 0x07;
    let mut key_code: u16 = f30.gpioled_key_map[button];
    let mut key_down: bool = !(f30.data_regs[reg_num] & BIT(bit_num));
    if (f30.trackstick_buttons &&
    button >= TRACKSTICK_RANGE_START &&
    button <= TRACKSTICK_RANGE_END) {
    rmi_f03_overwrite_button(f30.f03, key_code, key_down);
    } else {
    rmi_dbg(RMI_DEBUG_FN, &fn.dev,
    "%s: call input report key (0x%04x) value (0x%02x)",
    __func__, key_code, key_down);
    input_report_key(f30.input, key_code, key_down);
    }
    }
#[no_mangle]
unsafe extern "C" fn rmi_f30_attention(irq: c_int, ctx: *mut c_void) -> irqreturn_t {
    static irqreturn_t rmi_f30_attention(int irq, void *ctx)
    {
    struct rmi_function *fn = ctx;
    struct f30_data *f30 = dev_get_drvdata(&fn.dev);
    struct rmi_driver_data *drvdata = dev_get_drvdata(&fn.rmi_dev.dev);
    int error;
    int i;
// Read the gpi led data.
    if (drvdata.attn_data.data) {
    if (drvdata.attn_data.size < f30.register_count) {
    dev_warn(&fn.dev,
    "F30 interrupted, but data is missing\n");
    return IRQ_HANDLED;
    }
    memcpy(f30.data_regs, drvdata.attn_data.data,
    f30.register_count);
    drvdata.attn_data.data += f30.register_count;
    drvdata.attn_data.size -= f30.register_count;
    } else {
    error = rmi_read_block(fn.rmi_dev, fn.fd.data_base_addr,
    f30.data_regs, f30.register_count);
    if (error) {
    dev_err(&fn.dev,
    "%s: Failed to read F30 data registers: %d\n",
    __func__, error);
    return IRQ_RETVAL(error);
    }
    }
    if (f30.has_gpio) {
    for (i = 0; i < f30.gpioled_count; i++)
    if (f30.gpioled_key_map[i] != KEY_RESERVED)
    rmi_f30_report_button(fn, f30, i);
    if (f30.trackstick_buttons)
    rmi_f03_commit_buttons(f30.f03);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f30_config(fn: *mut rmi_function) -> c_int {
    static int rmi_f30_config(struct rmi_function *fn)
    {
    struct f30_data *f30 = dev_get_drvdata(&fn.dev);
    struct rmi_driver *drv = fn.rmi_dev.driver;
    const struct rmi_device_platform_data *pdata =
    rmi_get_platform_data(fn.rmi_dev);
    int error;
// can happen if gpio_data.disable is set
    if (!f30)
    return 0;
    if (pdata.gpio_data.trackstick_buttons) {
// Try [re-]establish link to F03.
    f30.f03 = rmi_find_function(fn.rmi_dev, 0x03);
    f30.trackstick_buttons = f30.f03 != core::ptr::null_mut();
    }
    if (pdata.gpio_data.disable) {
    drv.clear_irq_bits(fn.rmi_dev, fn.irq_mask);
    } else {
// Write Control Register values back to device
    error = rmi_write_block(fn.rmi_dev, fn.fd.control_base_addr,
    f30.ctrl_regs, f30.ctrl_regs_size);
    if (error) {
    dev_err(&fn.dev,
    "%s: Could not write control registers at 0x%x: %d\n",
    __func__, fn.fd.control_base_addr, error);
    return error;
    }
    drv.set_irq_bits(fn.rmi_dev, fn.irq_mask);
    }
    return 0;
    }
    static void rmi_f30_set_ctrl_data(struct rmi_f30_ctrl_data *ctrl,
    int *ctrl_addr, int len, u8 **reg)
    {
    ctrl.address = *ctrl_addr;
    ctrl.length = len;
    ctrl.regs = *reg;
// ctrl_addr += len;
// reg += len;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f30_is_valid_button(button: c_int, ctrl: *mut rmi_f30_ctrl_data) -> bool {
    static bool rmi_f30_is_valid_button(int button, struct rmi_f30_ctrl_data *ctrl)
    {
    let mut byte_position: c_int = button >> 3;
    let mut bit_position: c_int = button & 0x07;
//
// ctrl2 -> dir == 0 -> input mode
// ctrl3 -> data == 1 -> actual button
//
    return !(ctrl[2].regs[byte_position] & BIT(bit_position)) &&
    (ctrl[3].regs[byte_position] & BIT(bit_position));
    }
    static int rmi_f30_map_gpios(struct rmi_function *fn,
    struct f30_data *f30)
    {
    const struct rmi_device_platform_data *pdata =
    rmi_get_platform_data(fn.rmi_dev);
    struct input_dev *input = f30.input;
    let mut button: c_uint = BTN_LEFT;
    let mut trackstick_button: c_uint = BTN_LEFT;
    let mut button_mapped: bool = false;
    int i;
    let mut button_count: c_int = min_t(u8, f30.gpioled_count, TRACKSTICK_RANGE_END);
    f30.gpioled_key_map = devm_kcalloc(&fn.dev,
    f30.gpioled_count,
    sizeof(f30.gpioled_key_map[0]),
    GFP_KERNEL);
    if (!f30.gpioled_key_map) {
    dev_err(&fn.dev, "Failed to allocate gpioled map memory.\n");
    return -ENOMEM;
    }
    for (i = 0; i < button_count; i++) {
    if (!rmi_f30_is_valid_button(i, f30.ctrl))
    continue;
    if (pdata.gpio_data.trackstick_buttons &&
    i >= TRACKSTICK_RANGE_START && i < TRACKSTICK_RANGE_END) {
    f30.gpioled_key_map[i] = trackstick_button++;
    } else if (!pdata.gpio_data.buttonpad || !button_mapped) {
    f30.gpioled_key_map[i] = button;
    input_set_capability(input, EV_KEY, button++);
    button_mapped = true;
    }
    }
    input.keycode = f30.gpioled_key_map;
    input.keycodesize = sizeof(f30.gpioled_key_map[0]);
    input.keycodemax = f30.gpioled_count;
//
// Buttonpad could be also inferred from f30->has_mech_mouse_btns,
// but I am not sure, so use only the pdata info and the number of
// mapped buttons.
//
    if (pdata.gpio_data.buttonpad || (button - BTN_LEFT == 1))
    __set_bit(INPUT_PROP_BUTTONPAD, input.propbit);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f30_initialize(fn: *mut rmi_function, f30: *mut f30_data) -> c_int {
    static int rmi_f30_initialize(struct rmi_function *fn, struct f30_data *f30)
    {
    u8 *ctrl_reg = f30.ctrl_regs;
    let mut control_address: c_int = fn.fd.control_base_addr;
    u8 buf[RMI_F30_QUERY_SIZE];
    int error;
    error = rmi_read_block(fn.rmi_dev, fn.fd.query_base_addr,
    buf, RMI_F30_QUERY_SIZE);
    if (error) {
    dev_err(&fn.dev, "Failed to read query register\n");
    return error;
    }
    f30.has_extended_pattern = buf[0] & RMI_F30_EXTENDED_PATTERNS;
    f30.has_mappable_buttons = buf[0] & RMI_F30_HAS_MAPPABLE_BUTTONS;
    f30.has_led = buf[0] & RMI_F30_HAS_LED;
    f30.has_gpio = buf[0] & RMI_F30_HAS_GPIO;
    f30.has_haptic = buf[0] & RMI_F30_HAS_HAPTIC;
    f30.has_gpio_driver_control = buf[0] & RMI_F30_HAS_GPIO_DRV_CTL;
    f30.has_mech_mouse_btns = buf[0] & RMI_F30_HAS_MECH_MOUSE_BTNS;
    f30.gpioled_count = buf[1] & RMI_F30_GPIO_LED_COUNT;
    f30.register_count = DIV_ROUND_UP(f30.gpioled_count, 8);
    if (f30.has_gpio && f30.has_led)
    rmi_f30_set_ctrl_data(&f30.ctrl[0], &control_address,
    f30.register_count, &ctrl_reg);
    rmi_f30_set_ctrl_data(&f30.ctrl[1], &control_address,
    sizeof(u8), &ctrl_reg);
    if (f30.has_gpio) {
    rmi_f30_set_ctrl_data(&f30.ctrl[2], &control_address,
    f30.register_count, &ctrl_reg);
    rmi_f30_set_ctrl_data(&f30.ctrl[3], &control_address,
    f30.register_count, &ctrl_reg);
    }
    if (f30.has_led) {
    rmi_f30_set_ctrl_data(&f30.ctrl[4], &control_address,
    f30.register_count, &ctrl_reg);
    rmi_f30_set_ctrl_data(&f30.ctrl[5], &control_address,
    f30.has_extended_pattern ? 6 : 2,
    &ctrl_reg);
    }
    if (f30.has_led || f30.has_gpio_driver_control) {
// control 6 uses a byte per gpio/led
    rmi_f30_set_ctrl_data(&f30.ctrl[6], &control_address,
    f30.gpioled_count, &ctrl_reg);
    }
    if (f30.has_mappable_buttons) {
// control 7 uses a byte per gpio/led
    rmi_f30_set_ctrl_data(&f30.ctrl[7], &control_address,
    f30.gpioled_count, &ctrl_reg);
    }
    if (f30.has_haptic) {
    rmi_f30_set_ctrl_data(&f30.ctrl[8], &control_address,
    f30.register_count, &ctrl_reg);
    rmi_f30_set_ctrl_data(&f30.ctrl[9], &control_address,
    sizeof(u8), &ctrl_reg);
    }
    if (f30.has_mech_mouse_btns)
    rmi_f30_set_ctrl_data(&f30.ctrl[10], &control_address,
    sizeof(u8), &ctrl_reg);
    f30.ctrl_regs_size = ctrl_reg -
    f30.ctrl_regs ?: RMI_F30_CTRL_REGS_MAX_SIZE;
    error = rmi_f30_read_control_parameters(fn, f30);
    if (error) {
    dev_err(&fn.dev,
    "Failed to initialize F30 control params: %d\n",
    error);
    return error;
    }
    if (f30.has_gpio) {
    error = rmi_f30_map_gpios(fn, f30);
    if (error)
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f30_probe(fn: *mut rmi_function) -> c_int {
    static int rmi_f30_probe(struct rmi_function *fn)
    {
    struct rmi_device *rmi_dev = fn.rmi_dev;
    const struct rmi_device_platform_data *pdata =
    rmi_get_platform_data(rmi_dev);
    struct rmi_driver_data *drv_data = dev_get_drvdata(&rmi_dev.dev);
    struct f30_data *f30;
    int error;
    if (pdata.gpio_data.disable)
    return 0;
    if (!drv_data.input) {
    dev_info(&fn.dev, "F30: no input device found, ignoring\n");
    return -ENXIO;
    }
    f30 = devm_kzalloc(&fn.dev, sizeof(*f30), GFP_KERNEL);
    if (!f30)
    return -ENOMEM;
    f30.input = drv_data.input;
    error = rmi_f30_initialize(fn, f30);
    if (error)
    return error;
    dev_set_drvdata(&fn.dev, f30);
    return 0;
    }
    struct rmi_function_handler rmi_f30_handler = {
    .driver = {
    .name = "rmi4_f30",
    },
    .func = 0x30,
    .probe = rmi_f30_probe,
    .config = rmi_f30_config,
    .attention = rmi_f30_attention,
    };
