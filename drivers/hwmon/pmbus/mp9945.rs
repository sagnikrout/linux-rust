//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/mp9945.c
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
// Hardware monitoring driver for MPS Single-phase Digital VR Controllers(MP9945)
//

pub const MFR_VR_MULTI_CONFIG_R1: c_uint = 0x08;
pub const MFR_SVID_CFG_R1: c_uint = 0xBD;
// VOUT_MODE register values
pub const VOUT_MODE_LINEAR16: c_uint = 0x17;
pub const VOUT_MODE_VID: c_uint = 0x21;
pub const VOUT_MODE_DIRECT: c_uint = 0x40;
pub const MP9945_PAGE_NUM: c_int = 1;

    PMBUS_HAVE_IIN | PMBUS_HAVE_IOUT | \
    PMBUS_HAVE_PIN | PMBUS_HAVE_POUT | \
    PMBUS_HAVE_TEMP | \
    PMBUS_HAVE_STATUS_VOUT | \
    PMBUS_HAVE_STATUS_IOUT | \
    PMBUS_HAVE_STATUS_TEMP | \
    PMBUS_HAVE_STATUS_INPUT)
    enum mp9945_vout_mode {
    MP9945_VOUT_MODE_VID,
    MP9945_VOUT_MODE_DIRECT,
    MP9945_VOUT_MODE_LINEAR16,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mp9945_data {
    pub info: pmbus_driver_info,
    pub vout_mode: enum mp9945_vout_mode,
    pub vid_resolution: c_int,
    pub vid_offset: c_int,
}

#[no_mangle]
unsafe extern "C" fn mp9945_read_vout(client: *mut i2c_client, data: *mut mp9945_data) -> c_int {
    static int mp9945_read_vout(struct i2c_client *client, struct mp9945_data *data)
    {
    int ret;
    ret = i2c_smbus_read_word_data(client, PMBUS_READ_VOUT);
    if (ret < 0)
    return ret;
    ret &= GENMASK(11, 0);
    switch (data.vout_mode) {
    case MP9945_VOUT_MODE_VID:
    if (ret > 0)
    ret = (ret + data.vid_offset) * data.vid_resolution;
    break;
    case MP9945_VOUT_MODE_DIRECT:
    break;
    case MP9945_VOUT_MODE_LINEAR16:
// LSB: 1000 * 2^-9 (mV)
    ret = DIV_ROUND_CLOSEST(ret * 125, 64);
    break;
    default:
    return -ENODEV;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mp9945_read_byte_data(client: *mut i2c_client, page: c_int, reg: c_int) -> c_int {
    static int mp9945_read_byte_data(struct i2c_client *client, int page, int reg)
    {
    int ret;
    ret = i2c_smbus_write_byte_data(client, PMBUS_PAGE, 0);
    if (ret < 0)
    return ret;
    switch (reg) {
    case PMBUS_VOUT_MODE:
//
// Override VOUT_MODE to DIRECT as the driver handles custom
// VOUT format conversions internally.
//
    return PB_VOUT_MODE_DIRECT;
    default:
    return -ENODATA;
    }
    }
    static int mp9945_read_word_data(struct i2c_client *client, int page, int phase,
    int reg)
    {
    const struct pmbus_driver_info *info = pmbus_get_driver_info(client);
    struct mp9945_data *data = to_mp9945_data(info);
    int ret;
    ret = i2c_smbus_write_byte_data(client, PMBUS_PAGE, 0);
    if (ret < 0)
    return ret;
    switch (reg) {
    case PMBUS_READ_VOUT:
    ret = mp9945_read_vout(client, data);
    break;
    case PMBUS_VOUT_OV_FAULT_LIMIT:
    case PMBUS_VOUT_UV_FAULT_LIMIT:
    ret = i2c_smbus_read_word_data(client, reg);
    if (ret < 0)
    return ret;
// LSB: 1.95 (mV)
    ret = DIV_ROUND_CLOSEST((ret & GENMASK(11, 0)) * 39, 20);
    break;
    case PMBUS_VOUT_UV_WARN_LIMIT:
    ret = i2c_smbus_read_word_data(client, reg);
    if (ret < 0)
    return ret;
    ret &= GENMASK(9, 0);
    if (ret > 0)
    ret = (ret + data.vid_offset) * data.vid_resolution;
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static int mp9945_identify(struct i2c_client *client,
    struct pmbus_driver_info *info)
    {
    struct mp9945_data *data = to_mp9945_data(info);
    int ret;
    ret = i2c_smbus_read_byte_data(client, PMBUS_VOUT_MODE);
    if (ret < 0)
    return ret;
    switch (ret) {
    case VOUT_MODE_LINEAR16:
    data.vout_mode = MP9945_VOUT_MODE_LINEAR16;
    break;
    case VOUT_MODE_VID:
    data.vout_mode = MP9945_VOUT_MODE_VID;
    break;
    case VOUT_MODE_DIRECT:
    data.vout_mode = MP9945_VOUT_MODE_DIRECT;
    break;
    default:
    return -ENODEV;
    }
    ret = i2c_smbus_write_byte_data(client, PMBUS_PAGE, 3);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_read_word_data(client, MFR_VR_MULTI_CONFIG_R1);
    if (ret < 0)
    return ret;
    data.vid_resolution = (FIELD_GET(BIT(2), ret)) ? 5 : 10;
    ret = i2c_smbus_read_word_data(client, MFR_SVID_CFG_R1);
    if (ret < 0)
    return ret;
    data.vid_offset = (FIELD_GET(BIT(15), ret)) ? 19 : 49;
    return i2c_smbus_write_byte_data(client, PMBUS_PAGE, 0);
    }
    static struct pmbus_driver_info mp9945_info = {
    .pages = MP9945_PAGE_NUM,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = direct,
    .format[PSC_CURRENT_IN] = linear,
    .format[PSC_CURRENT_OUT] = linear,
    .format[PSC_POWER] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .m[PSC_VOLTAGE_OUT] = 1,
    .R[PSC_VOLTAGE_OUT] = 3,
    .b[PSC_VOLTAGE_OUT] = 0,
    .func[0] = MP9945_RAIL1_FUNC,
    .read_word_data = mp9945_read_word_data,
    .read_byte_data = mp9945_read_byte_data,
    .identify = mp9945_identify,
    };
#[no_mangle]
unsafe extern "C" fn mp9945_probe(client: *mut i2c_client) -> c_int {
    static int mp9945_probe(struct i2c_client *client)
    {
    struct mp9945_data *data;
    int ret;
    data = devm_kzalloc(&client.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    memcpy(&data.info, &mp9945_info, sizeof(mp9945_info));
//
// Set page 0 before probe. The core reads paged registers which are
// only on page 0 for this device.
//
    ret = i2c_smbus_write_byte_data(client, PMBUS_PAGE, 0);
    if (ret < 0)
    return ret;
    return pmbus_do_probe(client, &data.info);
    }
    static const struct i2c_device_id mp9945_id[] = {
    { .name = "mp9945" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mp9945_id);
    static const struct of_device_id __maybe_unused mp9945_of_match[] = {
    {.compatible = "mps,mp9945"},
    {}
    };
    MODULE_DEVICE_TABLE(of, mp9945_of_match);
    static struct i2c_driver mp9945_driver = {
    .driver = {
    .name = "mp9945",
    .of_match_table = of_match_ptr(mp9945_of_match),
    },
    .probe = mp9945_probe,
    .id_table = mp9945_id,
    };
    module_i2c_driver(mp9945_driver);
    MODULE_AUTHOR("Cosmo Chou <chou.cosmo@gmail.com>");
    MODULE_DESCRIPTION("PMBus driver for MPS MP9945");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
