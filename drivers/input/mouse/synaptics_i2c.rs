//! Automatically rewritten from C to Rust
//! Source: drivers/input/mouse/synaptics_i2c.c
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


//
// Synaptics touchpad with I2C interface
//
// Copyright (C) 2009 Compulab, Ltd.
// Mike Rapoport <mike@compulab.co.il>
// Igor Grinberg <grinberg@compulab.co.il>
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive for
// more details.
//

// maximum product id is 15 characters
pub const PRODUCT_ID_LENGTH: c_int = 15;
pub const REGISTER_LENGTH: c_int = 8;
//
// after soft reset, we should wait for 1 ms
// before the device becomes operational
//
pub const SOFT_RESET_DELAY_US: c_int = 3000;
// and after hard reset, we should wait for max 500ms
pub const HARD_RESET_DELAY_MS: c_int = 500;
// Registers by SMBus address
pub const PAGE_SEL_REG: c_uint = 0xff;
pub const DEVICE_STATUS_REG: c_uint = 0x09;
// Registers by RMI address
pub const DEV_CONTROL_REG: c_uint = 0x0000;
pub const INTERRUPT_EN_REG: c_uint = 0x0001;
pub const ERR_STAT_REG: c_uint = 0x0002;
pub const INT_REQ_STAT_REG: c_uint = 0x0003;
pub const DEV_COMMAND_REG: c_uint = 0x0004;
pub const RMI_PROT_VER_REG: c_uint = 0x0200;
pub const MANUFACT_ID_REG: c_uint = 0x0201;
pub const PHYS_INT_VER_REG: c_uint = 0x0202;
pub const PROD_PROPERTY_REG: c_uint = 0x0203;
pub const INFO_QUERY_REG0: c_uint = 0x0204;

pub const PRODUCT_ID_REG0: c_uint = 0x0210;

pub const DATA_REG0: c_uint = 0x0400;
pub const ABS_PRESSURE_REG: c_uint = 0x0401;
pub const ABS_MSB_X_REG: c_uint = 0x0402;

pub const ABS_MSB_Y_REG: c_uint = 0x0404;

pub const REL_X_REG: c_uint = 0x0406;
pub const REL_Y_REG: c_uint = 0x0407;
pub const DEV_QUERY_REG0: c_uint = 0x1000;

pub const GENERAL_2D_CONTROL_REG: c_uint = 0x1041;
pub const SENSOR_SENSITIVITY_REG: c_uint = 0x1044;
pub const SENS_MAX_POS_MSB_REG: c_uint = 0x1046;

// Register bits
// Device Control Register Bits
pub const REPORT_RATE_1ST_BIT: c_int = 6;
// Interrupt Enable Register Bits (INTERRUPT_EN_REG)
pub const F10_ABS_INT_ENA: c_int = 0;
pub const F10_REL_INT_ENA: c_int = 1;
pub const F20_INT_ENA: c_int = 2;
// Interrupt Request Register Bits (INT_REQ_STAT_REG | DEVICE_STATUS_REG)
pub const F10_ABS_INT_REQ: c_int = 0;
pub const F10_REL_INT_REQ: c_int = 1;
pub const F20_INT_REQ: c_int = 2;
// Device Status Register Bits (DEVICE_STATUS_REG)
pub const STAT_CONFIGURED: c_int = 6;
pub const STAT_ERROR: c_int = 7;
// Device Command Register Bits (DEV_COMMAND_REG)
pub const RESET_COMMAND: c_uint = 0x01;
pub const REZERO_COMMAND: c_uint = 0x02;
// Data Register 0 Bits (DATA_REG0)
pub const GESTURE: c_int = 3;
// Device Query Registers Bits
// DEV_QUERY_REG3
pub const HAS_PALM_DETECT: c_int = 1;
pub const HAS_MULTI_FING: c_int = 2;
pub const HAS_SCROLLER: c_int = 4;
pub const HAS_2D_SCROLL: c_int = 5;
// General 2D Control Register Bits (GENERAL_2D_CONTROL_REG)
pub const NO_DECELERATION: c_int = 1;
pub const REDUCE_REPORTING: c_int = 3;
pub const NO_FILTER: c_int = 5;
// Function Masks
// Device Control Register Masks (DEV_CONTROL_REG)
pub const REPORT_RATE_MSK: c_uint = 0xc0;
pub const SLEEP_MODE_MSK: c_uint = 0x07;
// Device Sleep Modes
pub const FULL_AWAKE: c_uint = 0x0;
pub const NORMAL_OP: c_uint = 0x1;
pub const LOW_PWR_OP: c_uint = 0x2;
pub const VERY_LOW_PWR_OP: c_uint = 0x3;
pub const SENS_SLEEP: c_uint = 0x4;
pub const SLEEP_MOD: c_uint = 0x5;
pub const DEEP_SLEEP: c_uint = 0x6;
pub const HIBERNATE: c_uint = 0x7;
// Interrupt Register Mask
// (INT_REQ_STAT_REG | DEVICE_STATUS_REG | INTERRUPT_EN_REG)
pub const INT_ENA_REQ_MSK: c_uint = 0x07;
pub const INT_ENA_ABS_MSK: c_uint = 0x01;
pub const INT_ENA_REL_MSK: c_uint = 0x02;
pub const INT_ENA_F20_MSK: c_uint = 0x04;
// Device Status Register Masks (DEVICE_STATUS_REG)
pub const CONFIGURED_MSK: c_uint = 0x40;
pub const ERROR_MSK: c_uint = 0x80;
// Data Register 0 Masks
pub const FINGER_WIDTH_MSK: c_uint = 0xf0;
pub const GESTURE_MSK: c_uint = 0x08;
pub const SENSOR_STATUS_MSK: c_uint = 0x07;
//
// MSB Position Register Masks
// ABS_MSB_X_REG | ABS_MSB_Y_REG | SENS_MAX_POS_MSB_REG |
// DEV_QUERY_REG3 | DEV_QUERY_REG5
//
pub const MSB_POSITION_MSK: c_uint = 0x1f;
// Device Query Registers Masks
// DEV_QUERY_REG2
pub const NUM_EXTRA_POS_MSK: c_uint = 0x07;
// When in IRQ mode read the device every THREAD_IRQ_SLEEP_SECS
pub const THREAD_IRQ_SLEEP_SECS: c_int = 2;

//
// When in Polling mode and no data received for NO_DATA_THRES msecs
// reduce the polling rate to NO_DATA_SLEEP_MSECS
//

// Control touchpad's No Deceleration option
    let mut no_decel: static bool = true;
    module_param(no_decel, bool, 0644);
    MODULE_PARM_DESC(no_decel, "No Deceleration. Default = 1 (on)");
// Control touchpad's Reduced Reporting option
    static bool reduce_report;
    module_param(reduce_report, bool, 0644);
    MODULE_PARM_DESC(reduce_report, "Reduced Reporting. Default = 0 (off)");
// Control touchpad's No Filter option
    static bool no_filter;
    module_param(no_filter, bool, 0644);
    MODULE_PARM_DESC(no_filter, "No Filter. Default = 0 (off)");
//
// touchpad Attention line is Active Low and Open Drain,
// therefore should be connected to pulled up line
// and the irq configuration should be set to Falling Edge Trigger
//
// Control IRQ / Polling option
    static bool polling_req;
    module_param(polling_req, bool, 0444);
    MODULE_PARM_DESC(polling_req, "Request Polling. Default = 0 (use irq)");
// Control Polling Rate
    let mut scan_rate: static int = 80;
    module_param(scan_rate, int, 0644);
    MODULE_PARM_DESC(scan_rate, "Polling rate in times/sec. Default = 80");
// The main device structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synaptics_i2c {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub dwork: delayed_work,
    pub no_data_count: c_int,
    pub no_decel_param: c_int,
    pub reduce_report_param: c_int,
    pub no_filter_param: c_int,
    pub scan_rate_param: c_int,
    pub scan_ms: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn set_scan_rate(touch: *mut synaptics_i2c, scan_rate: c_int) {
    static inline void set_scan_rate(struct synaptics_i2c *touch, int scan_rate)
    {
    touch.scan_ms = MSEC_PER_SEC / scan_rate;
    touch.scan_rate_param = scan_rate;
    }
//
// Driver's initial design makes no race condition possible on i2c bus,
// so there is no need in any locking.
// Keep it in mind, while playing with the code.
//
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_reg_get(client: *mut i2c_client, reg: u16) -> i32 {
    static s32 synaptics_i2c_reg_get(struct i2c_client *client, u16 reg)
    {
    int error;
    error = i2c_smbus_write_byte_data(client, PAGE_SEL_REG, reg >> 8);
    if (error)
    return error;
    return i2c_smbus_read_byte_data(client, reg & 0xff);
    }
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_reg_set(client: *mut i2c_client, reg: u16, val: u8) -> i32 {
    static s32 synaptics_i2c_reg_set(struct i2c_client *client, u16 reg, u8 val)
    {
    int error;
    error = i2c_smbus_write_byte_data(client, PAGE_SEL_REG, reg >> 8);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, reg & 0xff, val);
    if (error)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_word_get(client: *mut i2c_client, reg: u16) -> i32 {
    static s32 synaptics_i2c_word_get(struct i2c_client *client, u16 reg)
    {
    int error;
    error = i2c_smbus_write_byte_data(client, PAGE_SEL_REG, reg >> 8);
    if (error)
    return error;
    return i2c_smbus_read_word_data(client, reg & 0xff);
    }
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_config(client: *mut i2c_client) -> c_int {
    static int synaptics_i2c_config(struct i2c_client *client)
    {
    int control;
    int error;
    u8 int_en;
// set Report Rate to Device Highest (>=80) and Sleep to normal
    error = synaptics_i2c_reg_set(client, DEV_CONTROL_REG, 0xc1);
    if (error)
    return error;
// set Interrupt Disable to Func20 / Enable to Func10)
    int_en = (polling_req) ? 0 : INT_ENA_ABS_MSK | INT_ENA_REL_MSK;
    error = synaptics_i2c_reg_set(client, INTERRUPT_EN_REG, int_en);
    if (error)
    return error;
    control = synaptics_i2c_reg_get(client, GENERAL_2D_CONTROL_REG);
// No Deceleration
    control |= no_decel ? 1 << NO_DECELERATION : 0;
// Reduced Reporting
    control |= reduce_report ? 1 << REDUCE_REPORTING : 0;
// No Filter
    control |= no_filter ? 1 << NO_FILTER : 0;
    error = synaptics_i2c_reg_set(client, GENERAL_2D_CONTROL_REG, control);
    if (error)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_reset_config(client: *mut i2c_client) -> c_int {
    static int synaptics_i2c_reset_config(struct i2c_client *client)
    {
    int error;
// Reset the Touchpad
    error = synaptics_i2c_reg_set(client, DEV_COMMAND_REG, RESET_COMMAND);
    if (error) {
    dev_err(&client.dev, "Unable to reset device\n");
    return error;
    }
    usleep_range(SOFT_RESET_DELAY_US, SOFT_RESET_DELAY_US + 100);
    error = synaptics_i2c_config(client);
    if (error) {
    dev_err(&client.dev, "Unable to config device\n");
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_check_error(client: *mut i2c_client) -> c_int {
    static int synaptics_i2c_check_error(struct i2c_client *client)
    {
    int status;
    int error;
    status = i2c_smbus_read_byte_data(client, DEVICE_STATUS_REG) &
    (CONFIGURED_MSK | ERROR_MSK);
    if (status != CONFIGURED_MSK) {
    error = synaptics_i2c_reset_config(client);
    if (error)
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_get_input(touch: *mut synaptics_i2c) -> bool {
    static bool synaptics_i2c_get_input(struct synaptics_i2c *touch)
    {
    struct input_dev *input = touch.input;
    int xy_delta, gesture;
    s32 data;
    s8 x_delta, y_delta;
// Deal with spontaneous resets and errors
    if (synaptics_i2c_check_error(touch.client))
    return false;
// Get Gesture Bit
    data = synaptics_i2c_reg_get(touch.client, DATA_REG0);
    gesture = (data >> GESTURE) & 0x1;
//
// Get Relative axes. we have to get them in one shot,
// so we get 2 bytes starting from REL_X_REG.
//
    xy_delta = synaptics_i2c_word_get(touch.client, REL_X_REG) & 0xffff;
// Separate X from Y
    x_delta = xy_delta & 0xff;
    y_delta = (xy_delta >> REGISTER_LENGTH) & 0xff;
// Report the button event
    input_report_key(input, BTN_LEFT, gesture);
// Report the deltas
    input_report_rel(input, REL_X, x_delta);
    input_report_rel(input, REL_Y, -y_delta);
    input_sync(input);
    return xy_delta || gesture;
    }
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t synaptics_i2c_irq(int irq, void *dev_id)
    {
    struct synaptics_i2c *touch = dev_id;
    mod_delayed_work(system_dfl_wq, &touch.dwork, 0);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_check_params(touch: *mut synaptics_i2c) {
    static void synaptics_i2c_check_params(struct synaptics_i2c *touch)
    {
    let mut reset: bool = false;
    if (scan_rate != touch.scan_rate_param)
    set_scan_rate(touch, scan_rate);
    if (no_decel != touch.no_decel_param) {
    touch.no_decel_param = no_decel;
    reset = true;
    }
    if (no_filter != touch.no_filter_param) {
    touch.no_filter_param = no_filter;
    reset = true;
    }
    if (reduce_report != touch.reduce_report_param) {
    touch.reduce_report_param = reduce_report;
    reset = true;
    }
    if (reset)
    synaptics_i2c_reset_config(touch.client);
    }
// Control the Device polling rate / Work Handler sleep time
    static unsigned long synaptics_i2c_adjust_delay(struct synaptics_i2c *touch,
    bool have_data)
    {
    unsigned long delay, nodata_count_thres;
    if (polling_req) {
    delay = touch.scan_ms;
    if (have_data) {
    touch.no_data_count = 0;
    } else {
    nodata_count_thres = NO_DATA_THRES / touch.scan_ms;
    if (touch.no_data_count < nodata_count_thres)
    touch.no_data_count++;
    else
    delay = NO_DATA_SLEEP_MSECS;
    }
    return msecs_to_jiffies(delay);
    }
    delay = msecs_to_jiffies(THREAD_IRQ_SLEEP_MSECS);
    return round_jiffies_relative(delay);
    }
// Work Handler
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_work_handler(work: *mut work_struct) {
    static void synaptics_i2c_work_handler(struct work_struct *work)
    {
    bool have_data;
    struct synaptics_i2c *touch =
    container_of(work, struct synaptics_i2c, dwork.work);
    unsigned long delay;
    synaptics_i2c_check_params(touch);
    have_data = synaptics_i2c_get_input(touch);
    delay = synaptics_i2c_adjust_delay(touch, have_data);
//
// While interrupt driven, there is no real need to poll the device.
// But touchpads are very sensitive, so there could be errors
// related to physical environment and the attention line isn't
// necessarily asserted. In such case we can lose the touchpad.
// We poll the device once in THREAD_IRQ_SLEEP_SECS and
// if error is detected, we try to reset and reconfigure the touchpad.
//
    mod_delayed_work(system_dfl_wq, &touch.dwork, delay);
    }
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_open(input: *mut input_dev) -> c_int {
    static int synaptics_i2c_open(struct input_dev *input)
    {
    struct synaptics_i2c *touch = input_get_drvdata(input);
    int error;
    error = synaptics_i2c_reset_config(touch.client);
    if (error)
    return error;
    if (polling_req)
    mod_delayed_work(system_dfl_wq, &touch.dwork,
    msecs_to_jiffies(NO_DATA_SLEEP_MSECS));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_close(input: *mut input_dev) {
    static void synaptics_i2c_close(struct input_dev *input)
    {
    struct synaptics_i2c *touch = input_get_drvdata(input);
    if (!polling_req)
    synaptics_i2c_reg_set(touch.client, INTERRUPT_EN_REG, 0);
    cancel_delayed_work_sync(&touch.dwork);
// Save some power
    synaptics_i2c_reg_set(touch.client, DEV_CONTROL_REG, DEEP_SLEEP);
    }
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_set_input_params(touch: *mut synaptics_i2c) {
    static void synaptics_i2c_set_input_params(struct synaptics_i2c *touch)
    {
    struct input_dev *input = touch.input;
    input.name = touch.client.name;
    input.phys = touch.client.adapter.name;
    input.id.bustype = BUS_I2C;
    input.id.version = synaptics_i2c_word_get(touch.client,
    INFO_QUERY_REG0);
    input.open = synaptics_i2c_open;
    input.close = synaptics_i2c_close;
    input_set_drvdata(input, touch);
// Register the device as mouse
    input_set_capability(input, EV_REL, REL_X);
    input_set_capability(input, EV_REL, REL_Y);
// Register device's buttons and keys
    input_set_capability(input, EV_KEY, BTN_LEFT);
    }
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_probe(client: *mut i2c_client) -> c_int {
    static int synaptics_i2c_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct synaptics_i2c *touch;
    int error;
    touch = devm_kzalloc(dev, sizeof(*touch), GFP_KERNEL);
    if (!touch)
    return -ENOMEM;
    touch.client = client;
    touch.no_decel_param = no_decel;
    touch.scan_rate_param = scan_rate;
    set_scan_rate(touch, scan_rate);
    INIT_DELAYED_WORK(&touch.dwork, synaptics_i2c_work_handler);
    error = synaptics_i2c_reset_config(client);
    if (error)
    return error;
    if (client.irq <= 0)
    polling_req = true;
    touch.input = devm_input_allocate_device(dev);
    if (!touch.input)
    return -ENOMEM;
    synaptics_i2c_set_input_params(touch);
    if (!polling_req) {
    dev_dbg(dev, "Requesting IRQ: %d\n", client.irq);
    error = devm_request_irq(dev, client.irq, synaptics_i2c_irq,
    IRQ_TYPE_EDGE_FALLING,
    DRIVER_NAME, touch);
    if (error) {
    dev_warn(dev, "IRQ request failed: %d, falling back to polling\n",
    error);
    polling_req = true;
    synaptics_i2c_reg_set(client, INTERRUPT_EN_REG, 0);
    }
    }
    if (polling_req)
    dev_dbg(dev, "Using polling at rate: %d times/sec\n", scan_rate);
// Register the device in input subsystem
    error = input_register_device(touch.input);
    if (error) {
    dev_err(dev, "Input device register failed: %d\n", error);
    return error;
    }
    i2c_set_clientdata(client, touch);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_suspend(dev: *mut device) -> c_int {
    static int synaptics_i2c_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct synaptics_i2c *touch = i2c_get_clientdata(client);
    cancel_delayed_work_sync(&touch.dwork);
// Save some power
    synaptics_i2c_reg_set(touch.client, DEV_CONTROL_REG, DEEP_SLEEP);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn synaptics_i2c_resume(dev: *mut device) -> c_int {
    static int synaptics_i2c_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct synaptics_i2c *touch = i2c_get_clientdata(client);
    struct input_dev *input = touch.input;
    int error;
    error = synaptics_i2c_reset_config(client);
    if (error)
    return error;
    guard(mutex)(&input.mutex);
    if (input_device_enabled(input))
    mod_delayed_work(system_dfl_wq, &touch.dwork,
    msecs_to_jiffies(NO_DATA_SLEEP_MSECS));
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(synaptics_i2c_pm, synaptics_i2c_suspend,
    synaptics_i2c_resume);
    static const struct i2c_device_id synaptics_i2c_id_table[] = {
    { .name = "synaptics_i2c" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, synaptics_i2c_id_table);

    static const struct of_device_id synaptics_i2c_of_match[] = {
    { .compatible = "synaptics,synaptics_i2c", },
    { },
    };
    MODULE_DEVICE_TABLE(of, synaptics_i2c_of_match);

    static struct i2c_driver synaptics_i2c_driver = {
    .driver = {
    .name	= DRIVER_NAME,
    .of_match_table = of_match_ptr(synaptics_i2c_of_match),
    .pm	= pm_sleep_ptr(&synaptics_i2c_pm),
    },
    .probe		= synaptics_i2c_probe,
    .id_table	= synaptics_i2c_id_table,
    };
    module_i2c_driver(synaptics_i2c_driver);
    MODULE_DESCRIPTION("Synaptics I2C touchpad driver");
    MODULE_AUTHOR("Mike Rapoport, Igor Grinberg, Compulab");
    MODULE_LICENSE("GPL");
