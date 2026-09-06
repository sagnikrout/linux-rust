//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/lx1308.c
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

pub const LX1308_MFR_IOUT_OCP3_FAULT: c_uint = 0xBE;
pub const LX1308_MFR_IOUT_OCP3_WARN: c_uint = 0xBF;
//
// Decode a Linear11-encoded word to an integer value.
// Linear11 format: bits[15:11] = signed 5-bit exponent,
// bits[10:0] = signed 11-bit mantissa. Result = mant * 2^exp.
//
#[no_mangle]
pub unsafe extern "C" fn linear11_to_int(word: u16) -> c_int {
    static inline int linear11_to_int(u16 word)
    {
    let mut exp: i16 = ((s16)word) >> 11;
    let mut mant: i16 = ((s16)((word & 0x7ff) << 5)) >> 5;
    return (exp >= 0) ? (int)((u32)mant << exp) : (mant >> -exp);
    }
    static int lx1308_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    int ret;
    if (page > 0)
    return -ENXIO;
    switch (reg) {
//
// The LX1308 OCP3 registers (slow OCP, 60ms delay) use a
// manufacturer-specific U8.0 format. Read the byte value N and present
// it as a Linear11 word with exponent 0.
//
    case PMBUS_IOUT_OC_FAULT_LIMIT:
    ret = i2c_smbus_read_byte_data(client, LX1308_MFR_IOUT_OCP3_FAULT);
    if (ret < 0)
    break;
    ret &= 0x7FF;
    break;
    case PMBUS_IOUT_OC_WARN_LIMIT:
    ret = i2c_smbus_read_byte_data(client, LX1308_MFR_IOUT_OCP3_WARN);
    if (ret < 0)
    break;
    ret &= 0x7FF;
    break;
//
// The following registers are not implemented by the LX1308. Return
// -ENXIO to suppress the corresponding sysfs attributes.
//
    case PMBUS_IIN_OC_WARN_LIMIT:
    case PMBUS_IIN_OC_FAULT_LIMIT:
    case PMBUS_IOUT_UC_FAULT_LIMIT:
    case PMBUS_PIN_OP_WARN_LIMIT:
    case PMBUS_POUT_OP_WARN_LIMIT:
    case PMBUS_UT_WARN_LIMIT:
    case PMBUS_UT_FAULT_LIMIT:
    case PMBUS_MFR_IIN_MAX:
    case PMBUS_MFR_IOUT_MAX:
    case PMBUS_MFR_VIN_MIN:
    case PMBUS_MFR_VIN_MAX:
    case PMBUS_MFR_VOUT_MIN:
    case PMBUS_MFR_VOUT_MAX:
    case PMBUS_MFR_PIN_MAX:
    case PMBUS_MFR_POUT_MAX:
    case PMBUS_MFR_MAX_TEMP_1:
    ret = -ENXIO;
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static int lx1308_write_word_data(struct i2c_client *client, int page,
    int reg, u16 word)
    {
    int ret;
    if (page > 0)
    return -ENXIO;
    switch (reg) {
    case PMBUS_IOUT_OC_FAULT_LIMIT:
//
// Decode Linear11 word from pmbus_core back to a plain integer
// and write as the U8.0 byte the device expects.
//
    ret = i2c_smbus_write_byte_data(client, LX1308_MFR_IOUT_OCP3_FAULT,
    clamp_val(linear11_to_int(word), 0, 255));
    break;
    case PMBUS_IOUT_OC_WARN_LIMIT:
    ret = i2c_smbus_write_byte_data(client, LX1308_MFR_IOUT_OCP3_WARN,
    clamp_val(linear11_to_int(word), 0, 255));
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static struct pmbus_driver_info lx1308_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = linear,
    .format[PSC_CURRENT_IN] = linear,
    .format[PSC_CURRENT_OUT] = linear,
    .format[PSC_POWER] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
    | PMBUS_HAVE_IIN | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT
    | PMBUS_HAVE_PIN | PMBUS_HAVE_POUT
    | PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP
    | PMBUS_HAVE_STATUS_INPUT,
    .read_word_data  = lx1308_read_word_data,
    .write_word_data = lx1308_write_word_data,
    };
    static const struct of_device_id lx1308_of_match[] = {
    { .compatible = "luxshare,lx1308" },
    { }
    };
    MODULE_DEVICE_TABLE(of, lx1308_of_match);
    static const struct i2c_device_id lx1308_id[] = {
    { "lx1308" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lx1308_id);
#[no_mangle]
unsafe extern "C" fn lx1308_probe(client: *mut i2c_client) -> c_int {
    static int lx1308_probe(struct i2c_client *client)
    {
    u8 buf[I2C_SMBUS_BLOCK_MAX + 1];
    const struct i2c_device_id *mid;
    int ret;
    ret = i2c_smbus_read_block_data(client, PMBUS_MFR_ID, buf);
    if (ret < 0)
    return dev_err_probe(&client.dev, ret,
    "Failed to read manufacturer id\n");
    buf[ret] = '\0';
    if (ret != 12 || strncmp(buf, "LUXSHARE", 8))
    return dev_err_probe(&client.dev, -ENODEV,
    "Unsupported Manufacturer ID '%s'\n", buf);
    ret = i2c_smbus_read_block_data(client, PMBUS_MFR_MODEL, buf);
    if (ret < 0)
    return dev_err_probe(&client.dev, ret,
    "Failed to read Manufacturer Model\n");
    buf[ret] = '\0';
    for (mid = lx1308_id; mid.name[0]; mid++) {
    if (!strncasecmp(mid.name, buf, strlen(mid.name)))
    break;
    }
    if (!mid.name[0])
    return dev_err_probe(&client.dev, -ENODEV,
    "Unsupported Manufacturer Model '%s'\n", buf);
    ret = i2c_smbus_read_block_data(client, PMBUS_MFR_REVISION, buf);
    if (ret < 0)
    return dev_err_probe(&client.dev, ret,
    "Failed to read Manufacturer Revision\n");
    buf[ret] = '\0';
    if (ret != 12 || buf[0] != 'V')
    return dev_err_probe(&client.dev, -ENODEV,
    "Unsupported Manufacturer Revision '%s'\n", buf);
    return pmbus_do_probe(client, &lx1308_info);
    }
    static struct i2c_driver lx1308_driver = {
    .driver = {
    .name = "lx1308",
    .of_match_table = lx1308_of_match,
    },
    .probe = lx1308_probe,
    .id_table = lx1308_id,
    };
    module_i2c_driver(lx1308_driver);
    MODULE_AUTHOR("Brian Chiang <chiang.brian@inventec.com>");
    MODULE_DESCRIPTION("PMBus driver for Luxshare LX1308");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
