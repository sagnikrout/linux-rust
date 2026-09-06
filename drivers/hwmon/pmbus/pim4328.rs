//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/pim4328.c
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
// Hardware monitoring driver for PIM4006, PIM4328 and PIM4820
//
// Copyright (c) 2021 Flextronics International Sweden AB
//

    enum chips { pim4006, pim4328, pim4820 };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pim4328_data {
    pub id: enum chips,
    pub info: pmbus_driver_info,
}

// PIM4006 and PIM4328
pub const PIM4328_MFR_READ_VINA: c_uint = 0xd3;
pub const PIM4328_MFR_READ_VINB: c_uint = 0xd4;
// PIM4006
pub const PIM4328_MFR_READ_IINA: c_uint = 0xd6;
pub const PIM4328_MFR_READ_IINB: c_uint = 0xd7;
pub const PIM4328_MFR_FET_CHECKSTATUS: c_uint = 0xd9;
// PIM4328
pub const PIM4328_MFR_STATUS_BITS: c_uint = 0xd5;
// PIM4820
pub const PIM4328_MFR_READ_STATUS: c_uint = 0xd0;
    static const struct i2c_device_id pim4328_id[] = {
    { .name = "bmr455", .driver_data = pim4328 },
    { .name = "pim4006", .driver_data = pim4006 },
    { .name = "pim4106", .driver_data = pim4006 },
    { .name = "pim4206", .driver_data = pim4006 },
    { .name = "pim4306", .driver_data = pim4006 },
    { .name = "pim4328", .driver_data = pim4328 },
    { .name = "pim4406", .driver_data = pim4006 },
    { .name = "pim4820", .driver_data = pim4820 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pim4328_id);
    static int pim4328_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    int ret;
    if (page > 0)
    return -ENXIO;
    if (phase == 0xff)
    return -ENODATA;
    switch (reg) {
    case PMBUS_READ_VIN:
    ret = pmbus_read_word_data(client, page, phase,
    phase == 0 ? PIM4328_MFR_READ_VINA
    : PIM4328_MFR_READ_VINB);
    break;
    case PMBUS_READ_IIN:
    ret = pmbus_read_word_data(client, page, phase,
    phase == 0 ? PIM4328_MFR_READ_IINA
    : PIM4328_MFR_READ_IINB);
    break;
    default:
    ret = -ENODATA;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pim4328_read_byte_data(client: *mut i2c_client, page: c_int, reg: c_int) -> c_int {
    static int pim4328_read_byte_data(struct i2c_client *client, int page, int reg)
    {
    const struct pmbus_driver_info *info = pmbus_get_driver_info(client);
    struct pim4328_data *data = to_pim4328_data(info);
    int ret, status;
    if (page > 0)
    return -ENXIO;
    switch (reg) {
    case PMBUS_STATUS_BYTE:
    ret = pmbus_read_byte_data(client, page, PMBUS_STATUS_BYTE);
    if (ret < 0)
    return ret;
    if (data.id == pim4006) {
    status = pmbus_read_word_data(client, page, 0xff,
    PIM4328_MFR_FET_CHECKSTATUS);
    if (status < 0)
    return status;
    if (status & 0x0630) /* Input UV */
    ret |= PB_STATUS_VIN_UV;
    } else if (data.id == pim4328) {
    status = pmbus_read_byte_data(client, page,
    PIM4328_MFR_STATUS_BITS);
    if (status < 0)
    return status;
    if (status & 0x04) /* Input UV */
    ret |= PB_STATUS_VIN_UV;
    if (status & 0x40) /* Output UV */
    ret |= PB_STATUS_NONE_ABOVE;
    } else if (data.id == pim4820) {
    status = pmbus_read_byte_data(client, page,
    PIM4328_MFR_READ_STATUS);
    if (status < 0)
    return status;
    if (status & 0x05) /* Input OV or OC */
    ret |= PB_STATUS_NONE_ABOVE;
    if (status & 0x1a) /* Input UV */
    ret |= PB_STATUS_VIN_UV;
    if (status & 0x40) /* OT */
    ret |= PB_STATUS_TEMPERATURE;
    }
    break;
    default:
    ret = -ENODATA;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pim4328_probe(client: *mut i2c_client) -> c_int {
    static int pim4328_probe(struct i2c_client *client)
    {
    int status;
    u8 device_id[I2C_SMBUS_BLOCK_MAX + 1];
    const struct i2c_device_id *mid;
    struct pim4328_data *data;
    struct pmbus_driver_info *info;
    struct pmbus_platform_data *pdata;
    struct device *dev = &client.dev;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_READ_BYTE_DATA
    | I2C_FUNC_SMBUS_BLOCK_DATA))
    return -ENODEV;
    data = devm_kzalloc(&client.dev, sizeof(struct pim4328_data),
    GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    status = i2c_smbus_read_block_data(client, PMBUS_MFR_MODEL, device_id);
    if (status < 0) {
    dev_err(&client.dev, "Failed to read Manufacturer Model\n");
    return status;
    }
    for (mid = pim4328_id; mid.name[0]; mid++) {
    if (!strncasecmp(mid.name, device_id, strlen(mid.name)))
    break;
    }
    if (!mid.name[0]) {
    dev_err(&client.dev, "Unsupported device\n");
    return -ENODEV;
    }
    if (strcmp(client.name, mid.name))
    dev_notice(&client.dev,
    "Device mismatch: Configured %s, detected %s\n",
    client.name, mid.name);
    data.id = mid.driver_data;
    info = &data.info;
    info.pages = 1;
    info.read_byte_data = pim4328_read_byte_data;
    info.read_word_data = pim4328_read_word_data;
    pdata = devm_kzalloc(dev, sizeof(struct pmbus_platform_data),
    GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    dev.platform_data = pdata;
    pdata.flags = PMBUS_NO_CAPABILITY | PMBUS_NO_WRITE_PROTECT;
    switch (data.id) {
    case pim4006:
    info.phases[0] = 2;
    info.func[0] = PMBUS_PHASE_VIRTUAL | PMBUS_HAVE_VIN
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_IOUT;
    info.pfunc[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_IIN;
    info.pfunc[1] = PMBUS_HAVE_VIN | PMBUS_HAVE_IIN;
    break;
    case pim4328:
    info.phases[0] = 2;
    info.func[0] = PMBUS_PHASE_VIRTUAL
    | PMBUS_HAVE_VCAP | PMBUS_HAVE_VIN
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_IOUT;
    info.pfunc[0] = PMBUS_HAVE_VIN;
    info.pfunc[1] = PMBUS_HAVE_VIN;
    info.format[PSC_VOLTAGE_IN] = direct;
    info.format[PSC_TEMPERATURE] = direct;
    info.format[PSC_CURRENT_OUT] = direct;
    pdata.flags |= PMBUS_USE_COEFFICIENTS_CMD;
    break;
    case pim4820:
    info.func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_TEMP
    | PMBUS_HAVE_IIN;
    info.format[PSC_VOLTAGE_IN] = direct;
    info.format[PSC_TEMPERATURE] = direct;
    info.format[PSC_CURRENT_IN] = direct;
    pdata.flags |= PMBUS_USE_COEFFICIENTS_CMD;
    break;
    default:
    return -ENODEV;
    }
    return pmbus_do_probe(client, info);
    }
    static struct i2c_driver pim4328_driver = {
    .driver = {
    .name = "pim4328",
    },
    .probe = pim4328_probe,
    .id_table = pim4328_id,
    };
    module_i2c_driver(pim4328_driver);
    MODULE_AUTHOR("Erik Rosen <erik.rosen@metormote.com>");
    MODULE_DESCRIPTION("PMBus driver for PIM4006, PIM4328, PIM4820 power interface modules");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
