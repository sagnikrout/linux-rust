//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/ucd9000.c
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
// Hardware monitoring driver for UCD90xxx Sequencer and System Health
// Controller series
//
// Copyright (C) 2011 Ericsson AB.
//

    enum chips { ucd9000, ucd90120, ucd90124, ucd90160, ucd90320, ucd9090,
    ucd90910 };
pub const UCD9000_MONITOR_CONFIG: c_uint = 0xd5;
pub const UCD9000_NUM_PAGES: c_uint = 0xd6;
pub const UCD9000_FAN_CONFIG_INDEX: c_uint = 0xe7;
pub const UCD9000_FAN_CONFIG: c_uint = 0xe8;
pub const UCD9000_MFR_STATUS: c_uint = 0xf3;
pub const UCD9000_GPIO_SELECT: c_uint = 0xfa;
pub const UCD9000_GPIO_CONFIG: c_uint = 0xfb;
pub const UCD9000_DEVICE_ID: c_uint = 0xfd;
// GPIO CONFIG bits

pub const UCD9000_GPIO_INPUT: c_int = 0;
pub const UCD9000_GPIO_OUTPUT: c_int = 1;

pub const UCD9000_MON_VOLTAGE: c_int = 1;
pub const UCD9000_MON_TEMPERATURE: c_int = 2;
pub const UCD9000_MON_CURRENT: c_int = 3;
pub const UCD9000_MON_VOLTAGE_HW: c_int = 4;
pub const UCD9000_NUM_FAN: c_int = 4;
pub const UCD9000_GPIO_NAME_LEN: c_int = 16;
pub const UCD9090_NUM_GPIOS: c_int = 23;
pub const UCD901XX_NUM_GPIOS: c_int = 26;
pub const UCD90320_NUM_GPIOS: c_int = 84;
pub const UCD90910_NUM_GPIOS: c_int = 26;
pub const UCD9000_DEBUGFS_NAME_LEN: c_int = 24;
pub const UCD9000_GPI_COUNT: c_int = 8;
pub const UCD90320_GPI_COUNT: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucd9000_data {
    pub fan_data: [u8; UCD9000_NUM_FAN][I2C_SMBUS_BLOCK_MAX],
    pub info: pmbus_driver_info,

    pub gpio: gpio_chip,

    pub debugfs: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucd9000_debugfs_entry {
    pub client: *mut i2c_client,
    pub index: u8,
}

//
// It has been observed that the UCD90320 randomly fails register access when
// doing another access right on the back of a register write. To mitigate this
// make sure that there is a minimum delay between a write access and the
// following access. The 500 is based on experimental data. At a delay of
// 350us the issue seems to go away. Add a bit of extra margin to allow for
// system to system differences.
//
pub const UCD90320_WAIT_DELAY_US: c_int = 500;
#[no_mangle]
unsafe extern "C" fn ucd9000_get_fan_config(client: *mut i2c_client, fan: c_int) -> c_int {
    static int ucd9000_get_fan_config(struct i2c_client *client, int fan)
    {
    let mut fan_config: c_int = 0;
    struct ucd9000_data *data
    = to_ucd9000_data(pmbus_get_driver_info(client));
    if (data.fan_data[fan][3] & 1)
    fan_config |= PB_FAN_2_INSTALLED;   /* Use lower bit position */
// Pulses/revolution
    fan_config |= (data.fan_data[fan][3] & 0x06) >> 1;
    return fan_config;
    }
#[no_mangle]
unsafe extern "C" fn ucd9000_read_byte_data(client: *mut i2c_client, page: c_int, reg: c_int) -> c_int {
    static int ucd9000_read_byte_data(struct i2c_client *client, int page, int reg)
    {
    let mut ret: c_int = 0;
    int fan_config;
    switch (reg) {
    case PMBUS_FAN_CONFIG_12:
    if (page > 0)
    return -ENXIO;
    ret = ucd9000_get_fan_config(client, 0);
    if (ret < 0)
    return ret;
    fan_config = ret << 4;
    ret = ucd9000_get_fan_config(client, 1);
    if (ret < 0)
    return ret;
    fan_config |= ret;
    ret = fan_config;
    break;
    case PMBUS_FAN_CONFIG_34:
    if (page > 0)
    return -ENXIO;
    ret = ucd9000_get_fan_config(client, 2);
    if (ret < 0)
    return ret;
    fan_config = ret << 4;
    ret = ucd9000_get_fan_config(client, 3);
    if (ret < 0)
    return ret;
    fan_config |= ret;
    ret = fan_config;
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static const struct i2c_device_id ucd9000_id[] = {
    { .name = "ucd9000", .driver_data = ucd9000 },
    { .name = "ucd90120", .driver_data = ucd90120 },
    { .name = "ucd90124", .driver_data = ucd90124 },
    { .name = "ucd90160", .driver_data = ucd90160 },
    { .name = "ucd90320", .driver_data = ucd90320 },
    { .name = "ucd9090", .driver_data = ucd9090 },
    { .name = "ucd90910", .driver_data = ucd90910 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ucd9000_id);
    static const struct of_device_id __maybe_unused ucd9000_of_match[] = {
    {
    .compatible = "ti,ucd9000",
    .data = (void *)ucd9000
    },
    {
    .compatible = "ti,ucd90120",
    .data = (void *)ucd90120
    },
    {
    .compatible = "ti,ucd90124",
    .data = (void *)ucd90124
    },
    {
    .compatible = "ti,ucd90160",
    .data = (void *)ucd90160
    },
    {
    .compatible = "ti,ucd90320",
    .data = (void *)ucd90320
    },
    {
    .compatible = "ti,ucd9090",
    .data = (void *)ucd9090
    },
    {
    .compatible = "ti,ucd90910",
    .data = (void *)ucd90910
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, ucd9000_of_match);

    static int ucd9000_gpio_read_config(struct i2c_client *client,
    unsigned int offset)
    {
    int ret;
// No page set required
    ret = i2c_smbus_write_byte_data(client, UCD9000_GPIO_SELECT, offset);
    if (ret < 0)
    return ret;
    return i2c_smbus_read_byte_data(client, UCD9000_GPIO_CONFIG);
    }
#[no_mangle]
unsafe extern "C" fn ucd9000_gpio_get(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int ucd9000_gpio_get(struct gpio_chip *gc, unsigned int offset)
    {
    struct i2c_client *client  = gpiochip_get_data(gc);
    int ret;
    ret = ucd9000_gpio_read_config(client, offset);
    if (ret < 0)
    return ret;
    return !!(ret & UCD9000_GPIO_CONFIG_STATUS);
    }
    static int ucd9000_gpio_set(struct gpio_chip *gc, unsigned int offset,
    int value)
    {
    struct i2c_client *client = gpiochip_get_data(gc);
    int ret;
    ret = ucd9000_gpio_read_config(client, offset);
    if (ret < 0) {
    dev_dbg(&client.dev, "failed to read GPIO %d config: %d\n",
    offset, ret);
    return ret;
    }
    if (value) {
    if (ret & UCD9000_GPIO_CONFIG_OUT_VALUE)
    return 0;
    ret |= UCD9000_GPIO_CONFIG_OUT_VALUE;
    } else {
    if (!(ret & UCD9000_GPIO_CONFIG_OUT_VALUE))
    return 0;
    ret &= ~UCD9000_GPIO_CONFIG_OUT_VALUE;
    }
    ret |= UCD9000_GPIO_CONFIG_ENABLE;
// Page set not required
    ret = i2c_smbus_write_byte_data(client, UCD9000_GPIO_CONFIG, ret);
    if (ret < 0) {
    dev_dbg(&client.dev, "Failed to write GPIO %d config: %d\n",
    offset, ret);
    return ret;
    }
    ret &= ~UCD9000_GPIO_CONFIG_ENABLE;
    ret = i2c_smbus_write_byte_data(client, UCD9000_GPIO_CONFIG, ret);
    if (ret < 0)
    dev_dbg(&client.dev, "Failed to write GPIO %d config: %d\n",
    offset, ret);
    return ret;
    }
    static int ucd9000_gpio_get_direction(struct gpio_chip *gc,
    unsigned int offset)
    {
    struct i2c_client *client = gpiochip_get_data(gc);
    int ret;
    ret = ucd9000_gpio_read_config(client, offset);
    if (ret < 0)
    return ret;
    return !(ret & UCD9000_GPIO_CONFIG_OUT_ENABLE);
    }
    static int ucd9000_gpio_set_direction(struct gpio_chip *gc,
    unsigned int offset, bool direction_out,
    int requested_out)
    {
    struct i2c_client *client = gpiochip_get_data(gc);
    int ret, config, out_val;
    ret = ucd9000_gpio_read_config(client, offset);
    if (ret < 0)
    return ret;
    if (direction_out) {
    out_val = requested_out ? UCD9000_GPIO_CONFIG_OUT_VALUE : 0;
    if (ret & UCD9000_GPIO_CONFIG_OUT_ENABLE) {
    if ((ret & UCD9000_GPIO_CONFIG_OUT_VALUE) == out_val)
    return 0;
    } else {
    ret |= UCD9000_GPIO_CONFIG_OUT_ENABLE;
    }
    if (out_val)
    ret |= UCD9000_GPIO_CONFIG_OUT_VALUE;
    else
    ret &= ~UCD9000_GPIO_CONFIG_OUT_VALUE;
    } else {
    if (!(ret & UCD9000_GPIO_CONFIG_OUT_ENABLE))
    return 0;
    ret &= ~UCD9000_GPIO_CONFIG_OUT_ENABLE;
    }
    ret |= UCD9000_GPIO_CONFIG_ENABLE;
    config = ret;
// Page set not required
    ret = i2c_smbus_write_byte_data(client, UCD9000_GPIO_CONFIG, config);
    if (ret < 0)
    return ret;
    config &= ~UCD9000_GPIO_CONFIG_ENABLE;
    return i2c_smbus_write_byte_data(client, UCD9000_GPIO_CONFIG, config);
    }
    static int ucd9000_gpio_direction_input(struct gpio_chip *gc,
    unsigned int offset)
    {
    return ucd9000_gpio_set_direction(gc, offset, UCD9000_GPIO_INPUT, 0);
    }
    static int ucd9000_gpio_direction_output(struct gpio_chip *gc,
    unsigned int offset, int val)
    {
    return ucd9000_gpio_set_direction(gc, offset, UCD9000_GPIO_OUTPUT,
    val);
    }
    static void ucd9000_probe_gpio(struct i2c_client *client,
    const struct i2c_device_id *mid,
    struct ucd9000_data *data)
    {
    int rc;
    switch (mid.driver_data) {
    case ucd9090:
    data.gpio.ngpio = UCD9090_NUM_GPIOS;
    break;
    case ucd90120:
    case ucd90124:
    case ucd90160:
    data.gpio.ngpio = UCD901XX_NUM_GPIOS;
    break;
    case ucd90320:
    data.gpio.ngpio = UCD90320_NUM_GPIOS;
    break;
    case ucd90910:
    data.gpio.ngpio = UCD90910_NUM_GPIOS;
    break;
    default:
    return; /* GPIO support is optional. */
    }
//
// Pinmux support has not been added to the new gpio_chip.
// This support should be added when possible given the mux
// behavior of these IO devices.
//
    data.gpio.label = client.name;
    data.gpio.get_direction = ucd9000_gpio_get_direction;
    data.gpio.direction_input = ucd9000_gpio_direction_input;
    data.gpio.direction_output = ucd9000_gpio_direction_output;
    data.gpio.get = ucd9000_gpio_get;
    data.gpio.set = ucd9000_gpio_set;
    data.gpio.can_sleep = true;
    data.gpio.base = -1;
    data.gpio.parent = &client.dev;
    rc = devm_gpiochip_add_data(&client.dev, &data.gpio, client);
    if (rc)
    dev_warn(&client.dev, "Could not add gpiochip: %d\n", rc);
    }

    static void ucd9000_probe_gpio(struct i2c_client *client,
    const struct i2c_device_id *mid,
    struct ucd9000_data *data)
    {
    }

#[no_mangle]
unsafe extern "C" fn ucd9000_get_mfr_status(client: *mut i2c_client, buffer: *mut u8) -> c_int {
    static int ucd9000_get_mfr_status(struct i2c_client *client, u8 *buffer)
    {
    let mut ret: c_int = pmbus_set_page(client, 0, 0xff);
    if (ret < 0)
    return ret;
    return i2c_smbus_read_block_data(client, UCD9000_MFR_STATUS, buffer);
    }
#[no_mangle]
unsafe extern "C" fn ucd9000_debugfs_show_mfr_status_bit(data: *mut c_void, val: *mut u64) -> c_int {
    static int ucd9000_debugfs_show_mfr_status_bit(void *data, u64 *val)
    {
    struct ucd9000_debugfs_entry *entry = data;
    struct i2c_client *client = entry.client;
    u8 buffer[I2C_SMBUS_BLOCK_MAX];
    int ret, i;
    ret = ucd9000_get_mfr_status(client, buffer);
    if (ret < 0)
    return ret;
//
// GPI fault bits are in sets of 8, two bytes from end of response.
//
    i = ret - 3 - entry.index / 8;
    if (i >= 0)
// val = !!(buffer[i] & BIT(entry->index % 8));
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(ucd9000_debugfs_mfr_status_bit,
    ucd9000_debugfs_show_mfr_status_bit, core::ptr::null_mut(), "%1lld\n");
    static ssize_t ucd9000_debugfs_read_mfr_status(struct file *file,
    char __user *buf, size_t count,
    loff_t *ppos)
    {
    struct i2c_client *client = file.private_data;
    u8 buffer[I2C_SMBUS_BLOCK_MAX];
    char str[(I2C_SMBUS_BLOCK_MAX * 2) + 2];
    char *res;
    int rc;
    rc = ucd9000_get_mfr_status(client, buffer);
    if (rc < 0)
    return rc;
    res = bin2hex(str, buffer, min(rc, I2C_SMBUS_BLOCK_MAX));
// res++ = '\n';
// res = 0;
    return simple_read_from_buffer(buf, count, ppos, str, res - str);
    }
    static const struct file_operations ucd9000_debugfs_show_mfr_status_fops = {
    .llseek = noop_llseek,
    .read = ucd9000_debugfs_read_mfr_status,
    .open = simple_open,
    };
    static int ucd9000_init_debugfs(struct i2c_client *client,
    const struct i2c_device_id *mid,
    struct ucd9000_data *data)
    {
    struct dentry *debugfs;
    struct ucd9000_debugfs_entry *entries;
    int i, gpi_count;
    char name[UCD9000_DEBUGFS_NAME_LEN];
    debugfs = pmbus_get_debugfs_dir(client);
    if (!debugfs)
    return -ENOENT;
    data.debugfs = debugfs_create_dir(client.name, debugfs);
//
// Of the chips this driver supports, only the UCD9090, UCD90160,
// UCD90320, and UCD90910 report GPI faults in their MFR_STATUS
// register, so only create the GPI fault debugfs attributes for those
// chips.
//
    if (mid.driver_data == ucd9090 || mid.driver_data == ucd90160 ||
    mid.driver_data == ucd90320 || mid.driver_data == ucd90910) {
    gpi_count = mid.driver_data == ucd90320 ? UCD90320_GPI_COUNT
    : UCD9000_GPI_COUNT;
    entries = devm_kcalloc(&client.dev,
    gpi_count, sizeof(*entries),
    GFP_KERNEL);
    if (!entries)
    return -ENOMEM;
    for (i = 0; i < gpi_count; i++) {
    entries[i].client = client;
    entries[i].index = i;
    scnprintf(name, UCD9000_DEBUGFS_NAME_LEN,
    "gpi%d_alarm", i + 1);
    debugfs_create_file(name, 0444, data.debugfs,
    &entries[i],
    &ucd9000_debugfs_mfr_status_bit);
    }
    }
    scnprintf(name, UCD9000_DEBUGFS_NAME_LEN, "mfr_status");
    debugfs_create_file(name, 0444, data.debugfs, client,
    &ucd9000_debugfs_show_mfr_status_fops);
    return 0;
    }

    static int ucd9000_init_debugfs(struct i2c_client *client,
    const struct i2c_device_id *mid,
    struct ucd9000_data *data)
    {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ucd9000_probe(client: *mut i2c_client) -> c_int {
    static int ucd9000_probe(struct i2c_client *client)
    {
    u8 block_buffer[I2C_SMBUS_BLOCK_MAX + 1];
    struct ucd9000_data *data;
    struct pmbus_driver_info *info;
    const struct i2c_device_id *mid;
    enum chips chip;
    int i, ret;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE_DATA |
    I2C_FUNC_SMBUS_BLOCK_DATA))
    return -ENODEV;
    ret = i2c_smbus_read_block_data(client, UCD9000_DEVICE_ID,
    block_buffer);
    if (ret < 0) {
    dev_err(&client.dev, "Failed to read device ID\n");
    return ret;
    }
    block_buffer[ret] = '\0';
    dev_info(&client.dev, "Device ID %s\n", block_buffer);
    for (mid = ucd9000_id; mid.name[0]; mid++) {
    if (!strncasecmp(mid.name, block_buffer, strlen(mid.name)))
    break;
    }
    if (!mid.name[0]) {
    dev_err(&client.dev, "Unsupported device\n");
    return -ENODEV;
    }
    if (client.dev.of_node)
    chip = (uintptr_t)of_device_get_match_data(&client.dev);
    else
    chip = mid.driver_data;
    if (chip != ucd9000 && strcmp(client.name, mid.name) != 0)
    dev_notice(&client.dev,
    "Device mismatch: Configured %s, detected %s\n",
    client.name, mid.name);
    data = devm_kzalloc(&client.dev, sizeof(struct ucd9000_data),
    GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    info = &data.info;
    ret = i2c_smbus_read_byte_data(client, UCD9000_NUM_PAGES);
    if (ret < 0) {
    dev_err(&client.dev,
    "Failed to read number of active pages\n");
    return ret;
    }
    info.pages = ret;
    if (!info.pages) {
    dev_err(&client.dev, "No pages configured\n");
    return -ENODEV;
    }
// The internal temperature sensor is always active
    info.func[0] = PMBUS_HAVE_TEMP;
// Everything else is configurable
    ret = i2c_smbus_read_block_data(client, UCD9000_MONITOR_CONFIG,
    block_buffer);
    if (ret <= 0) {
    dev_err(&client.dev, "Failed to read configuration data\n");
    return -ENODEV;
    }
    for (i = 0; i < ret; i++) {
    let mut page: c_int = UCD9000_MON_PAGE(block_buffer[i]);
    if (page >= info.pages)
    continue;
    switch (UCD9000_MON_TYPE(block_buffer[i])) {
    case UCD9000_MON_VOLTAGE:
    case UCD9000_MON_VOLTAGE_HW:
    info.func[page] |= PMBUS_HAVE_VOUT
    | PMBUS_HAVE_STATUS_VOUT;
    break;
    case UCD9000_MON_TEMPERATURE:
    info.func[page] |= PMBUS_HAVE_TEMP2
    | PMBUS_HAVE_STATUS_TEMP;
    break;
    case UCD9000_MON_CURRENT:
    info.func[page] |= PMBUS_HAVE_IOUT
    | PMBUS_HAVE_STATUS_IOUT;
    break;
    default:
    break;
    }
    }
// Fan configuration
    if (mid.driver_data == ucd90124) {
    for (i = 0; i < UCD9000_NUM_FAN; i++) {
    i2c_smbus_write_byte_data(client,
    UCD9000_FAN_CONFIG_INDEX, i);
    ret = i2c_smbus_read_block_data(client,
    UCD9000_FAN_CONFIG,
    data.fan_data[i]);
    if (ret < 0)
    return ret;
    }
    i2c_smbus_write_byte_data(client, UCD9000_FAN_CONFIG_INDEX, 0);
    info.read_byte_data = ucd9000_read_byte_data;
    info.func[0] |= PMBUS_HAVE_FAN12 | PMBUS_HAVE_STATUS_FAN12
    | PMBUS_HAVE_FAN34 | PMBUS_HAVE_STATUS_FAN34;
    } else if (mid.driver_data == ucd90320) {
// Delay SMBus operations after a write
    info.write_delay = UCD90320_WAIT_DELAY_US;
    }
    ucd9000_probe_gpio(client, mid, data);
    ret = pmbus_do_probe(client, info);
    if (ret)
    return ret;
    ret = ucd9000_init_debugfs(client, mid, data);
    if (ret)
    dev_warn(&client.dev, "Failed to register debugfs: %d\n",
    ret);
    return 0;
    }
// This is the driver that will be inserted
    static struct i2c_driver ucd9000_driver = {
    .driver = {
    .name = "ucd9000",
    .of_match_table = of_match_ptr(ucd9000_of_match),
    },
    .probe = ucd9000_probe,
    .id_table = ucd9000_id,
    };
    module_i2c_driver(ucd9000_driver);
    MODULE_AUTHOR("Guenter Roeck");
    MODULE_DESCRIPTION("PMBus driver for TI UCD90xxx");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
