//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/silead.c
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
// -------------------------------------------------------------------------
// Copyright (C) 2014-2015, Intel Corporation
//
// Derived from:
// gslX68X.c
// Copyright (C) 2010-2015, Shanghai Sileadinc Co.Ltd
//
// -------------------------------------------------------------------------
//

pub const SILEAD_REG_RESET: c_uint = 0xE0;
pub const SILEAD_REG_DATA: c_uint = 0x80;
pub const SILEAD_REG_TOUCH_NR: c_uint = 0x80;
pub const SILEAD_REG_POWER: c_uint = 0xBC;
pub const SILEAD_REG_CLOCK: c_uint = 0xE4;
pub const SILEAD_REG_STATUS: c_uint = 0xB0;
pub const SILEAD_REG_ID: c_uint = 0xFC;
pub const SILEAD_REG_MEM_CHECK: c_uint = 0xB0;
pub const SILEAD_STATUS_OK: c_uint = 0x5A5A5A5A;
pub const SILEAD_TS_DATA_LEN: c_int = 44;
pub const SILEAD_CLOCK: c_uint = 0x04;
pub const SILEAD_CMD_RESET: c_uint = 0x88;
pub const SILEAD_CMD_START: c_uint = 0x00;
pub const SILEAD_POINT_DATA_LEN: c_uint = 0x04;
pub const SILEAD_POINT_Y_OFF: c_uint = 0x00;
pub const SILEAD_POINT_Y_MSB_OFF: c_uint = 0x01;
pub const SILEAD_POINT_X_OFF: c_uint = 0x02;
pub const SILEAD_POINT_X_MSB_OFF: c_uint = 0x03;
pub const SILEAD_EXTRA_DATA_MASK: c_uint = 0xF0;
pub const SILEAD_CMD_SLEEP_MIN: c_int = 10000;
pub const SILEAD_CMD_SLEEP_MAX: c_int = 20000;
pub const SILEAD_POWER_SLEEP: c_int = 20;
pub const SILEAD_STARTUP_SLEEP: c_int = 30;
pub const SILEAD_MAX_FINGERS: c_int = 10;
    enum silead_ts_power {
    SILEAD_POWER_ON  = 1,
    SILEAD_POWER_OFF = 0
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct silead_ts_data {
    pub client: *mut i2c_client,
    pub gpio_power: *mut gpio_desc,
    pub input: *mut input_dev,
    pub pen_input: *mut input_dev,
    pub regulators: [regulator_bulk_data; 2],
    pub fw_name: [c_char; 64],
    pub prop: touchscreen_properties,
    pub chip_id: u32,
    pub pos: [input_mt_pos; SILEAD_MAX_FINGERS],
    pub slots: [c_int; SILEAD_MAX_FINGERS],
    pub id: [c_int; SILEAD_MAX_FINGERS],
    pub efi_fw_min_max: [u32; 4],
    pub efi_fw_min_max_set: bool,
    pub pen_supported: bool,
    pub pen_down: bool,
    pub pen_x_res: u32,
    pub pen_y_res: u32,
    pub pen_up_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct silead_fw_data {
    pub offset: u32,
    pub val: u32,
}

#[no_mangle]
unsafe extern "C" fn silead_apply_efi_fw_min_max(data: *mut silead_ts_data) {
    static void silead_apply_efi_fw_min_max(struct silead_ts_data *data)
    {
    struct input_absinfo *absinfo_x = &data.input.absinfo[ABS_MT_POSITION_X];
    struct input_absinfo *absinfo_y = &data.input.absinfo[ABS_MT_POSITION_Y];
    if (!data.efi_fw_min_max_set)
    return;
    absinfo_x.minimum = data.efi_fw_min_max[0];
    absinfo_x.maximum = data.efi_fw_min_max[1];
    absinfo_y.minimum = data.efi_fw_min_max[2];
    absinfo_y.maximum = data.efi_fw_min_max[3];
    if (data.prop.invert_x) {
    absinfo_x.maximum -= absinfo_x.minimum;
    absinfo_x.minimum = 0;
    }
    if (data.prop.invert_y) {
    absinfo_y.maximum -= absinfo_y.minimum;
    absinfo_y.minimum = 0;
    }
    if (data.prop.swap_x_y) {
    swap(absinfo_x.minimum, absinfo_y.minimum);
    swap(absinfo_x.maximum, absinfo_y.maximum);
    }
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_request_input_dev(data: *mut silead_ts_data) -> c_int {
    static int silead_ts_request_input_dev(struct silead_ts_data *data)
    {
    struct device *dev = &data.client.dev;
    int error;
    data.input = devm_input_allocate_device(dev);
    if (!data.input) {
    dev_err(dev,
    "Failed to allocate input device\n");
    return -ENOMEM;
    }
    input_set_abs_params(data.input, ABS_MT_POSITION_X, 0, 4095, 0, 0);
    input_set_abs_params(data.input, ABS_MT_POSITION_Y, 0, 4095, 0, 0);
    touchscreen_parse_properties(data.input, true, &data.prop);
    silead_apply_efi_fw_min_max(data);
    input_mt_init_slots(data.input, SILEAD_MAX_FINGERS,
    INPUT_MT_DIRECT | INPUT_MT_DROP_UNUSED |
    INPUT_MT_TRACK);
    if (device_property_read_bool(dev, "silead,home-button"))
    input_set_capability(data.input, EV_KEY, KEY_LEFTMETA);
    data.input.name = SILEAD_TS_NAME;
    data.input.phys = "input/ts";
    data.input.id.bustype = BUS_I2C;
    error = input_register_device(data.input);
    if (error) {
    dev_err(dev, "Failed to register input device: %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_request_pen_input_dev(data: *mut silead_ts_data) -> c_int {
    static int silead_ts_request_pen_input_dev(struct silead_ts_data *data)
    {
    struct device *dev = &data.client.dev;
    int error;
    if (!data.pen_supported)
    return 0;
    data.pen_input = devm_input_allocate_device(dev);
    if (!data.pen_input)
    return -ENOMEM;
    input_set_abs_params(data.pen_input, ABS_X, 0, 4095, 0, 0);
    input_set_abs_params(data.pen_input, ABS_Y, 0, 4095, 0, 0);
    input_set_capability(data.pen_input, EV_KEY, BTN_TOUCH);
    input_set_capability(data.pen_input, EV_KEY, BTN_TOOL_PEN);
    set_bit(INPUT_PROP_DIRECT, data.pen_input.propbit);
    touchscreen_parse_properties(data.pen_input, false, &data.prop);
    input_abs_set_res(data.pen_input, ABS_X, data.pen_x_res);
    input_abs_set_res(data.pen_input, ABS_Y, data.pen_y_res);
    data.pen_input.name = SILEAD_TS_NAME " pen";
    data.pen_input.phys = "input/pen";
    data.input.id.bustype = BUS_I2C;
    error = input_register_device(data.pen_input);
    if (error) {
    dev_err(dev, "Failed to register pen input device: %d\n", error);
    return error;
    }
    return 0;
    }
    static void silead_ts_set_power(struct i2c_client *client,
    enum silead_ts_power state)
    {
    struct silead_ts_data *data = i2c_get_clientdata(client);
    if (data.gpio_power) {
    gpiod_set_value_cansleep(data.gpio_power, state);
    msleep(SILEAD_POWER_SLEEP);
    }
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_handle_pen_data(data: *mut silead_ts_data, buf: *mut u8) -> bool {
    static bool silead_ts_handle_pen_data(struct silead_ts_data *data, u8 *buf)
    {
    u8 *coord = buf + SILEAD_POINT_DATA_LEN;
    struct input_mt_pos pos;
    if (!data.pen_supported || buf[2] != 0x00 || buf[3] != 0x00)
    return false;
    if (buf[0] == 0x00 && buf[1] == 0x00 && data.pen_down) {
    data.pen_up_count++;
    if (data.pen_up_count == 6) {
    data.pen_down = false;
    goto sync;
    }
    return true;
    }
    if (buf[0] == 0x01 && buf[1] == 0x08) {
    touchscreen_set_mt_pos(&pos, &data.prop,
    get_unaligned_le16(&coord[SILEAD_POINT_X_OFF]) & 0xfff,
    get_unaligned_le16(&coord[SILEAD_POINT_Y_OFF]) & 0xfff);
    input_report_abs(data.pen_input, ABS_X, pos.x);
    input_report_abs(data.pen_input, ABS_Y, pos.y);
    data.pen_up_count = 0;
    data.pen_down = true;
    goto sync;
    }
    return false;
    sync:
    input_report_key(data.pen_input, BTN_TOOL_PEN, data.pen_down);
    input_report_key(data.pen_input, BTN_TOUCH, data.pen_down);
    input_sync(data.pen_input);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_read_data(client: *mut i2c_client) {
    static void silead_ts_read_data(struct i2c_client *client)
    {
    struct silead_ts_data *data = i2c_get_clientdata(client);
    struct input_dev *input = data.input;
    struct device *dev = &client.dev;
    u8 *bufp, buf[SILEAD_TS_DATA_LEN];
    int touch_nr, softbutton, error, i;
    let mut softbutton_pressed: bool = false;
    error = i2c_smbus_read_i2c_block_data(client, SILEAD_REG_DATA,
    SILEAD_TS_DATA_LEN, buf);
    if (error < 0) {
    dev_err(dev, "Data read error %d\n", error);
    return;
    }
    if (buf[0] > SILEAD_MAX_FINGERS) {
    dev_warn(dev, "More touches reported then supported %d > %d\n",
    buf[0], SILEAD_MAX_FINGERS);
    buf[0] = SILEAD_MAX_FINGERS;
    }
    if (silead_ts_handle_pen_data(data, buf))
    goto sync; /* Pen is down, release all previous touches */
    touch_nr = 0;
    bufp = buf + SILEAD_POINT_DATA_LEN;
    for (i = 0; i < buf[0]; i++, bufp += SILEAD_POINT_DATA_LEN) {
    softbutton = (bufp[SILEAD_POINT_Y_MSB_OFF] &
    SILEAD_EXTRA_DATA_MASK) >> 4;
    if (softbutton) {
//
// For now only respond to softbutton == 0x01, some
// tablets *without* a capacative button send 0x04
// when crossing the edges of the screen.
//
    if (softbutton == 0x01)
    softbutton_pressed = true;
    continue;
    }
//
// Bits 4-7 are the touch id, note not all models have
// hardware touch ids so atm we don't use these.
//
    data.id[touch_nr] = (bufp[SILEAD_POINT_X_MSB_OFF] &
    SILEAD_EXTRA_DATA_MASK) >> 4;
    touchscreen_set_mt_pos(&data.pos[touch_nr], &data.prop,
    get_unaligned_le16(&bufp[SILEAD_POINT_X_OFF]) & 0xfff,
    get_unaligned_le16(&bufp[SILEAD_POINT_Y_OFF]) & 0xfff);
    touch_nr++;
    }
    input_mt_assign_slots(input, data.slots, data.pos, touch_nr, 0);
    for (i = 0; i < touch_nr; i++) {
    input_mt_slot(input, data.slots[i]);
    input_mt_report_slot_state(input, MT_TOOL_FINGER, true);
    input_report_abs(input, ABS_MT_POSITION_X, data.pos[i].x);
    input_report_abs(input, ABS_MT_POSITION_Y, data.pos[i].y);
    dev_dbg(dev, "x=%d y=%d hw_id=%d sw_id=%d\n", data.pos[i].x,
    data.pos[i].y, data.id[i], data.slots[i]);
    }
    sync:
    input_mt_sync_frame(input);
    input_report_key(input, KEY_LEFTMETA, softbutton_pressed);
    input_sync(input);
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_init(client: *mut i2c_client) -> c_int {
    static int silead_ts_init(struct i2c_client *client)
    {
    int error;
    error = i2c_smbus_write_byte_data(client, SILEAD_REG_RESET,
    SILEAD_CMD_RESET);
    if (error)
    goto i2c_write_err;
    usleep_range(SILEAD_CMD_SLEEP_MIN, SILEAD_CMD_SLEEP_MAX);
    error = i2c_smbus_write_byte_data(client, SILEAD_REG_TOUCH_NR,
    SILEAD_MAX_FINGERS);
    if (error)
    goto i2c_write_err;
    usleep_range(SILEAD_CMD_SLEEP_MIN, SILEAD_CMD_SLEEP_MAX);
    error = i2c_smbus_write_byte_data(client, SILEAD_REG_CLOCK,
    SILEAD_CLOCK);
    if (error)
    goto i2c_write_err;
    usleep_range(SILEAD_CMD_SLEEP_MIN, SILEAD_CMD_SLEEP_MAX);
    error = i2c_smbus_write_byte_data(client, SILEAD_REG_RESET,
    SILEAD_CMD_START);
    if (error)
    goto i2c_write_err;
    usleep_range(SILEAD_CMD_SLEEP_MIN, SILEAD_CMD_SLEEP_MAX);
    return 0;
    i2c_write_err:
    dev_err(&client.dev, "Registers clear error %d\n", error);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_reset(client: *mut i2c_client) -> c_int {
    static int silead_ts_reset(struct i2c_client *client)
    {
    int error;
    error = i2c_smbus_write_byte_data(client, SILEAD_REG_RESET,
    SILEAD_CMD_RESET);
    if (error)
    goto i2c_write_err;
    usleep_range(SILEAD_CMD_SLEEP_MIN, SILEAD_CMD_SLEEP_MAX);
    error = i2c_smbus_write_byte_data(client, SILEAD_REG_CLOCK,
    SILEAD_CLOCK);
    if (error)
    goto i2c_write_err;
    usleep_range(SILEAD_CMD_SLEEP_MIN, SILEAD_CMD_SLEEP_MAX);
    error = i2c_smbus_write_byte_data(client, SILEAD_REG_POWER,
    SILEAD_CMD_START);
    if (error)
    goto i2c_write_err;
    usleep_range(SILEAD_CMD_SLEEP_MIN, SILEAD_CMD_SLEEP_MAX);
    return 0;
    i2c_write_err:
    dev_err(&client.dev, "Chip reset error %d\n", error);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_startup(client: *mut i2c_client) -> c_int {
    static int silead_ts_startup(struct i2c_client *client)
    {
    int error;
    error = i2c_smbus_write_byte_data(client, SILEAD_REG_RESET, 0x00);
    if (error) {
    dev_err(&client.dev, "Startup error %d\n", error);
    return error;
    }
    msleep(SILEAD_STARTUP_SLEEP);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_load_fw(client: *mut i2c_client) -> c_int {
    static int silead_ts_load_fw(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct silead_ts_data *data = i2c_get_clientdata(client);
    const struct firmware *fw = core::ptr::null_mut();
    struct silead_fw_data *fw_data;
    unsigned int fw_size, i;
    int error;
    dev_dbg(dev, "Firmware file name: %s", data.fw_name);
//
// Unfortunately, at the time of writing this comment, we have been unable to
// get permission from Silead, or from device OEMs, to distribute the necessary
// Silead firmware files in linux-firmware.
//
// On a whole bunch of devices the UEFI BIOS code contains a touchscreen driver,
// which contains an embedded copy of the firmware. The fw-loader code has a
// "platform" fallback mechanism, which together with info on the firmware
// from drivers/platform/x86/touchscreen_dmi.c will use the firmware from the
// UEFI driver when the firmware is missing from /lib/firmware. This makes the
// touchscreen work OOTB without users needing to manually download the firmware.
//
// The firmware bundled with the original Windows/Android is usually newer then
// the firmware in the UEFI driver and it is better calibrated. This better
// calibration can lead to significant differences in the reported min/max
// coordinates.
//
// To deal with this we first try to load the firmware without "platform"
// fallback. If that fails we retry with "platform" fallback and if that
// succeeds we apply an (optional) set of alternative min/max values from the
// "silead,efi-fw-min-max" property.
//
    error = firmware_request_nowarn(&fw, data.fw_name, dev);
    if (error) {
    error = firmware_request_platform(&fw, data.fw_name, dev);
    if (error) {
    dev_err(dev, "Firmware request error %d\n", error);
    return error;
    }
    error = device_property_read_u32_array(dev, "silead,efi-fw-min-max",
    data.efi_fw_min_max,
    ARRAY_SIZE(data.efi_fw_min_max));
    if (!error)
    data.efi_fw_min_max_set = true;
// The EFI (platform) embedded fw does not have pen support
    if (data.pen_supported) {
    dev_warn(dev, "Warning loading '%s' from filesystem failed, using EFI embedded copy.\n",
    data.fw_name);
    dev_warn(dev, "Warning pen support is known to be broken in the EFI embedded fw version\n");
    data.pen_supported = false;
    }
    }
    fw_size = fw.size / sizeof(*fw_data);
    fw_data = (struct silead_fw_data *)fw.data;
    for (i = 0; i < fw_size; i++) {
    error = i2c_smbus_write_i2c_block_data(client,
    fw_data[i].offset,
    4,
    (u8 *)&fw_data[i].val);
    if (error) {
    dev_err(dev, "Firmware load error %d\n", error);
    break;
    }
    }
    release_firmware(fw);
    return error ?: 0;
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_get_status(client: *mut i2c_client) -> u32 {
    static u32 silead_ts_get_status(struct i2c_client *client)
    {
    int error;
    __le32 status;
    error = i2c_smbus_read_i2c_block_data(client, SILEAD_REG_STATUS,
    sizeof(status), (u8 *)&status);
    if (error < 0) {
    dev_err(&client.dev, "Status read error %d\n", error);
    return error;
    }
    return le32_to_cpu(status);
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_get_id(client: *mut i2c_client) -> c_int {
    static int silead_ts_get_id(struct i2c_client *client)
    {
    struct silead_ts_data *data = i2c_get_clientdata(client);
    __le32 chip_id;
    int error;
    error = i2c_smbus_read_i2c_block_data(client, SILEAD_REG_ID,
    sizeof(chip_id), (u8 *)&chip_id);
    if (error < 0)
    return error;
    data.chip_id = le32_to_cpu(chip_id);
    dev_info(&client.dev, "Silead chip ID: 0x%8X", data.chip_id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_setup(client: *mut i2c_client) -> c_int {
    static int silead_ts_setup(struct i2c_client *client)
    {
    int error;
    u32 status;
//
// Some buggy BIOS-es bring up the chip in a stuck state where it
// blocks the I2C bus. The following steps are necessary to
// unstuck the chip / bus:
// 1. Turn off the Silead chip.
// 2. Try to do an I2C transfer with the chip, this will fail in
// response to which the I2C-bus-driver will call:
// i2c_recover_bus() which will unstuck the I2C-bus. Note the
// unstuck-ing of the I2C bus only works if we first drop the
// chip off the bus by turning it off.
// 3. Turn the chip back on.
//
// On the x86/ACPI systems were this problem is seen, step 1. and
// 3. require making ACPI calls and dealing with ACPI Power
// Resources. The workaround below runtime-suspends the chip to
// turn it off, leaving it up to the ACPI subsystem to deal with
// this.
//
    if (device_property_read_bool(&client.dev,
    "silead,stuck-controller-bug")) {
    pm_runtime_set_active(&client.dev);
    pm_runtime_enable(&client.dev);
    pm_runtime_allow(&client.dev);
    pm_runtime_suspend(&client.dev);
    dev_warn(&client.dev, FW_BUG "Stuck I2C bus: please ignore the next 'controller timed out' error\n");
    silead_ts_get_id(client);
// The forbid will also resume the device
    pm_runtime_forbid(&client.dev);
    pm_runtime_disable(&client.dev);
    }
    silead_ts_set_power(client, SILEAD_POWER_OFF);
    silead_ts_set_power(client, SILEAD_POWER_ON);
    error = silead_ts_get_id(client);
    if (error) {
    dev_err(&client.dev, "Chip ID read error %d\n", error);
    return error;
    }
    error = silead_ts_init(client);
    if (error)
    return error;
    error = silead_ts_reset(client);
    if (error)
    return error;
    error = silead_ts_load_fw(client);
    if (error)
    return error;
    error = silead_ts_startup(client);
    if (error)
    return error;
    status = silead_ts_get_status(client);
    if (status != SILEAD_STATUS_OK) {
    dev_err(&client.dev,
    "Initialization error, status: 0x%X\n", status);
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_threaded_irq_handler(irq: c_int, id: *mut c_void) -> irqreturn_t {
    static irqreturn_t silead_ts_threaded_irq_handler(int irq, void *id)
    {
    struct silead_ts_data *data = id;
    struct i2c_client *client = data.client;
    silead_ts_read_data(client);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_read_props(client: *mut i2c_client) {
    static void silead_ts_read_props(struct i2c_client *client)
    {
    struct silead_ts_data *data = i2c_get_clientdata(client);
    struct device *dev = &client.dev;
    const char *str;
    int error;
    error = device_property_read_string(dev, "firmware-name", &str);
    if (!error)
    snprintf(data.fw_name, sizeof(data.fw_name),
    "silead/%s", str);
    else
    dev_dbg(dev, "Firmware file name read error. Using default.");
    data.pen_supported = device_property_read_bool(dev, "silead,pen-supported");
    device_property_read_u32(dev, "silead,pen-resolution-x", &data.pen_x_res);
    device_property_read_u32(dev, "silead,pen-resolution-y", &data.pen_y_res);
    }

    static int silead_ts_set_default_fw_name(struct silead_ts_data *data,
    const struct i2c_device_id *id)
    {
    const struct acpi_device_id *acpi_id;
    struct device *dev = &data.client.dev;
    int i;
    if (ACPI_HANDLE(dev)) {
    acpi_id = acpi_match_device(dev.driver.acpi_match_table, dev);
    if (!acpi_id)
    return -ENODEV;
    snprintf(data.fw_name, sizeof(data.fw_name),
    "silead/%s.fw", acpi_id.id);
    for (i = 0; i < strlen(data.fw_name); i++)
    data.fw_name[i] = tolower(data.fw_name[i]);
    } else {
    snprintf(data.fw_name, sizeof(data.fw_name),
    "silead/%s.fw", id.name);
    }
    return 0;
    }

    static int silead_ts_set_default_fw_name(struct silead_ts_data *data,
    const struct i2c_device_id *id)
    {
    snprintf(data.fw_name, sizeof(data.fw_name),
    "silead/%s.fw", id.name);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn silead_disable_regulator(arg: *mut c_void) {
    static void silead_disable_regulator(void *arg)
    {
    struct silead_ts_data *data = arg;
    regulator_bulk_disable(ARRAY_SIZE(data.regulators), data.regulators);
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_probe(client: *mut i2c_client) -> c_int {
    static int silead_ts_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct silead_ts_data *data;
    struct device *dev = &client.dev;
    int error;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_I2C |
    I2C_FUNC_SMBUS_READ_I2C_BLOCK |
    I2C_FUNC_SMBUS_WRITE_I2C_BLOCK)) {
    dev_err(dev, "I2C functionality check failed\n");
    return -ENXIO;
    }
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    i2c_set_clientdata(client, data);
    data.client = client;
    error = silead_ts_set_default_fw_name(data, id);
    if (error)
    return error;
    silead_ts_read_props(client);
// We must have the IRQ provided by DT or ACPI subsystem
    if (client.irq <= 0)
    return -ENODEV;
    data.regulators[0].supply = "vddio";
    data.regulators[1].supply = "avdd";
    error = devm_regulator_bulk_get(dev, ARRAY_SIZE(data.regulators),
    data.regulators);
    if (error)
    return error;
//
// Enable regulators at probe and disable them at remove, we need
// to keep the chip powered otherwise it forgets its firmware.
//
    error = regulator_bulk_enable(ARRAY_SIZE(data.regulators),
    data.regulators);
    if (error)
    return error;
    error = devm_add_action_or_reset(dev, silead_disable_regulator, data);
    if (error)
    return error;
// Power GPIO pin
    data.gpio_power = devm_gpiod_get_optional(dev, "power", GPIOD_OUT_LOW);
    if (IS_ERR(data.gpio_power))
    return dev_err_probe(dev, PTR_ERR(data.gpio_power),
    "Shutdown GPIO request failed\n");
    error = silead_ts_setup(client);
    if (error)
    return error;
    error = silead_ts_request_input_dev(data);
    if (error)
    return error;
    error = silead_ts_request_pen_input_dev(data);
    if (error)
    return error;
    error = devm_request_threaded_irq(dev, client.irq,
    core::ptr::null_mut(), silead_ts_threaded_irq_handler,
    IRQF_ONESHOT, client.name, data);
    if (error) {
    if (error != -EPROBE_DEFER)
    dev_err(dev, "IRQ request failed %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_suspend(dev: *mut device) -> c_int {
    static int silead_ts_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    disable_irq(client.irq);
    silead_ts_set_power(client, SILEAD_POWER_OFF);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn silead_ts_resume(dev: *mut device) -> c_int {
    static int silead_ts_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    let mut second_try: bool = false;
    int error, status;
    silead_ts_set_power(client, SILEAD_POWER_ON);
    retry:
    error = silead_ts_reset(client);
    if (error)
    return error;
    if (second_try) {
    error = silead_ts_load_fw(client);
    if (error)
    return error;
    }
    error = silead_ts_startup(client);
    if (error)
    return error;
    status = silead_ts_get_status(client);
    if (status != SILEAD_STATUS_OK) {
    if (!second_try) {
    second_try = true;
    dev_dbg(dev, "Reloading firmware after unsuccessful resume\n");
    goto retry;
    }
    dev_err(dev, "Resume error, status: 0x%02x\n", status);
    return -ENODEV;
    }
    enable_irq(client.irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(silead_ts_pm, silead_ts_suspend, silead_ts_resume);
    static const struct i2c_device_id silead_ts_id[] = {
    { .name = "gsl1680" },
    { .name = "gsl1688" },
    { .name = "gsl3670" },
    { .name = "gsl3675" },
    { .name = "gsl3692" },
    { .name = "mssl1680" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, silead_ts_id);

    static const struct acpi_device_id silead_ts_acpi_match[] = {
    { "GSL1680", 0 },
    { "GSL1688", 0 },
    { "GSL3670", 0 },
    { "GSL3675", 0 },
    { "GSL3692", 0 },
    { "MSSL1680", 0 },
    { "MSSL0001", 0 },
    { "MSSL0002", 0 },
    { "MSSL0017", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, silead_ts_acpi_match);

    static const struct of_device_id silead_ts_of_match[] = {
    { .compatible = "silead,gsl1680" },
    { .compatible = "silead,gsl1688" },
    { .compatible = "silead,gsl3670" },
    { .compatible = "silead,gsl3675" },
    { .compatible = "silead,gsl3692" },
    { },
    };
    MODULE_DEVICE_TABLE(of, silead_ts_of_match);

    static struct i2c_driver silead_ts_driver = {
    .probe = silead_ts_probe,
    .id_table = silead_ts_id,
    .driver = {
    .name = SILEAD_TS_NAME,
    .acpi_match_table = ACPI_PTR(silead_ts_acpi_match),
    .of_match_table = of_match_ptr(silead_ts_of_match),
    .pm = pm_sleep_ptr(&silead_ts_pm),
    },
    };
    module_i2c_driver(silead_ts_driver);
    MODULE_AUTHOR("Robert Dolca <robert.dolca@intel.com>");
    MODULE_DESCRIPTION("Silead I2C touchscreen driver");
    MODULE_LICENSE("GPL");
