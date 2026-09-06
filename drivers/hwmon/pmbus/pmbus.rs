//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwmon/pmbus/pmbus.h
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
// pmbus.h - Common defines and structures for PMBus devices
//
// Copyright (c) 2010, 2011 Ericsson AB.
// Copyright (c) 2012 Guenter Roeck
//

//
// Registers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pmbus_regs {
    PMBUS_PAGE			= 0x00,
    PMBUS_OPERATION			= 0x01,
    PMBUS_ON_OFF_CONFIG		= 0x02,
    PMBUS_CLEAR_FAULTS		= 0x03,
    PMBUS_PHASE			= 0x04,

    PMBUS_WRITE_PROTECT		= 0x10,

    PMBUS_CAPABILITY		= 0x19,
    PMBUS_QUERY			= 0x1A,
    PMBUS_SMBALERT_MASK		= 0x1B,
    PMBUS_VOUT_MODE			= 0x20,
    PMBUS_VOUT_COMMAND		= 0x21,
    PMBUS_VOUT_TRIM			= 0x22,
    PMBUS_VOUT_CAL_OFFSET		= 0x23,
    PMBUS_VOUT_MAX			= 0x24,
    PMBUS_VOUT_MARGIN_HIGH		= 0x25,
    PMBUS_VOUT_MARGIN_LOW		= 0x26,
    PMBUS_VOUT_TRANSITION_RATE	= 0x27,
    PMBUS_VOUT_DROOP		= 0x28,
    PMBUS_VOUT_SCALE_LOOP		= 0x29,
    PMBUS_VOUT_SCALE_MONITOR	= 0x2A,

    PMBUS_COEFFICIENTS		= 0x30,
    PMBUS_POUT_MAX			= 0x31,

    PMBUS_FAN_CONFIG_12		= 0x3A,
    PMBUS_FAN_COMMAND_1		= 0x3B,
    PMBUS_FAN_COMMAND_2		= 0x3C,
    PMBUS_FAN_CONFIG_34		= 0x3D,
    PMBUS_FAN_COMMAND_3		= 0x3E,
    PMBUS_FAN_COMMAND_4		= 0x3F,

    PMBUS_VOUT_OV_FAULT_LIMIT	= 0x40,
    PMBUS_VOUT_OV_FAULT_RESPONSE	= 0x41,
    PMBUS_VOUT_OV_WARN_LIMIT	= 0x42,
    PMBUS_VOUT_UV_WARN_LIMIT	= 0x43,
    PMBUS_VOUT_UV_FAULT_LIMIT	= 0x44,
    PMBUS_VOUT_UV_FAULT_RESPONSE	= 0x45,
    PMBUS_IOUT_OC_FAULT_LIMIT	= 0x46,
    PMBUS_IOUT_OC_FAULT_RESPONSE	= 0x47,
    PMBUS_IOUT_OC_LV_FAULT_LIMIT	= 0x48,
    PMBUS_IOUT_OC_LV_FAULT_RESPONSE	= 0x49,
    PMBUS_IOUT_OC_WARN_LIMIT	= 0x4A,
    PMBUS_IOUT_UC_FAULT_LIMIT	= 0x4B,
    PMBUS_IOUT_UC_FAULT_RESPONSE	= 0x4C,

    PMBUS_OT_FAULT_LIMIT		= 0x4F,
    PMBUS_OT_FAULT_RESPONSE		= 0x50,
    PMBUS_OT_WARN_LIMIT		= 0x51,
    PMBUS_UT_WARN_LIMIT		= 0x52,
    PMBUS_UT_FAULT_LIMIT		= 0x53,
    PMBUS_UT_FAULT_RESPONSE		= 0x54,
    PMBUS_VIN_OV_FAULT_LIMIT	= 0x55,
    PMBUS_VIN_OV_FAULT_RESPONSE	= 0x56,
    PMBUS_VIN_OV_WARN_LIMIT		= 0x57,
    PMBUS_VIN_UV_WARN_LIMIT		= 0x58,
    PMBUS_VIN_UV_FAULT_LIMIT	= 0x59,

    PMBUS_IIN_OC_FAULT_LIMIT	= 0x5B,
    PMBUS_IIN_OC_WARN_LIMIT		= 0x5D,

    PMBUS_POUT_OP_FAULT_LIMIT	= 0x68,
    PMBUS_POUT_OP_WARN_LIMIT	= 0x6A,
    PMBUS_PIN_OP_WARN_LIMIT		= 0x6B,

    PMBUS_STATUS_BYTE		= 0x78,
    PMBUS_STATUS_WORD		= 0x79,
    PMBUS_STATUS_VOUT		= 0x7A,
    PMBUS_STATUS_IOUT		= 0x7B,
    PMBUS_STATUS_INPUT		= 0x7C,
    PMBUS_STATUS_TEMPERATURE	= 0x7D,
    PMBUS_STATUS_CML		= 0x7E,
    PMBUS_STATUS_OTHER		= 0x7F,
    PMBUS_STATUS_MFR_SPECIFIC	= 0x80,
    PMBUS_STATUS_FAN_12		= 0x81,
    PMBUS_STATUS_FAN_34		= 0x82,

    PMBUS_READ_VIN			= 0x88,
    PMBUS_READ_IIN			= 0x89,
    PMBUS_READ_VCAP			= 0x8A,
    PMBUS_READ_VOUT			= 0x8B,
    PMBUS_READ_IOUT			= 0x8C,
    PMBUS_READ_TEMPERATURE_1	= 0x8D,
    PMBUS_READ_TEMPERATURE_2	= 0x8E,
    PMBUS_READ_TEMPERATURE_3	= 0x8F,
    PMBUS_READ_FAN_SPEED_1		= 0x90,
    PMBUS_READ_FAN_SPEED_2		= 0x91,
    PMBUS_READ_FAN_SPEED_3		= 0x92,
    PMBUS_READ_FAN_SPEED_4		= 0x93,
    PMBUS_READ_DUTY_CYCLE		= 0x94,
    PMBUS_READ_FREQUENCY		= 0x95,
    PMBUS_READ_POUT			= 0x96,
    PMBUS_READ_PIN			= 0x97,

    PMBUS_REVISION			= 0x98,
    PMBUS_MFR_ID			= 0x99,
    PMBUS_MFR_MODEL			= 0x9A,
    PMBUS_MFR_REVISION		= 0x9B,
    PMBUS_MFR_LOCATION		= 0x9C,
    PMBUS_MFR_DATE			= 0x9D,
    PMBUS_MFR_SERIAL		= 0x9E,

    PMBUS_MFR_VIN_MIN		= 0xA0,
    PMBUS_MFR_VIN_MAX		= 0xA1,
    PMBUS_MFR_IIN_MAX		= 0xA2,
    PMBUS_MFR_PIN_MAX		= 0xA3,
    PMBUS_MFR_VOUT_MIN		= 0xA4,
    PMBUS_MFR_VOUT_MAX		= 0xA5,
    PMBUS_MFR_IOUT_MAX		= 0xA6,
    PMBUS_MFR_POUT_MAX		= 0xA7,

    PMBUS_IC_DEVICE_ID		= 0xAD,
    PMBUS_IC_DEVICE_REV		= 0xAE,

    PMBUS_MFR_MAX_TEMP_1		= 0xC0,
    PMBUS_MFR_MAX_TEMP_2		= 0xC1,
    PMBUS_MFR_MAX_TEMP_3		= 0xC2,

//
// Virtual registers.
// Useful to support attributes which are not supported by standard PMBus
// registers but exist as manufacturer specific registers on individual chips.
// Must be mapped to real registers in device specific code.
//
// Semantics:
// Virtual registers are all word size.
// READ registers are read-only; writes are either ignored or return an error.
// RESET registers are read/write. Reading reset registers returns zero
// (used for detection), writing any value causes the associated history to be
// reset.
// Virtual registers have to be handled in device specific driver code. Chip
// driver code returns non-negative register values if a virtual register is
// supported, or a negative error code if not. The chip driver may return
// -ENODATA or any other error code in this case, though an error code other
// than -ENODATA is handled more efficiently and thus preferred. Either case,
// the calling PMBus core code will abort if the chip driver returns an error
// code when reading or writing virtual registers.
//
    PMBUS_VIRT_BASE			= 0x100,
    PMBUS_VIRT_READ_TEMP_AVG,
    PMBUS_VIRT_READ_TEMP_MIN,
    PMBUS_VIRT_READ_TEMP_MAX,
    PMBUS_VIRT_RESET_TEMP_HISTORY,
    PMBUS_VIRT_READ_VIN_AVG,
    PMBUS_VIRT_READ_VIN_MIN,
    PMBUS_VIRT_READ_VIN_MAX,
    PMBUS_VIRT_RESET_VIN_HISTORY,
    PMBUS_VIRT_READ_IIN_AVG,
    PMBUS_VIRT_READ_IIN_MIN,
    PMBUS_VIRT_READ_IIN_MAX,
    PMBUS_VIRT_RESET_IIN_HISTORY,
    PMBUS_VIRT_READ_PIN_AVG,
    PMBUS_VIRT_READ_PIN_MIN,
    PMBUS_VIRT_READ_PIN_MAX,
    PMBUS_VIRT_RESET_PIN_HISTORY,
    PMBUS_VIRT_READ_POUT_AVG,
    PMBUS_VIRT_READ_POUT_MIN,
    PMBUS_VIRT_READ_POUT_MAX,
    PMBUS_VIRT_RESET_POUT_HISTORY,
    PMBUS_VIRT_READ_VOUT_AVG,
    PMBUS_VIRT_READ_VOUT_MIN,
    PMBUS_VIRT_READ_VOUT_MAX,
    PMBUS_VIRT_RESET_VOUT_HISTORY,
    PMBUS_VIRT_READ_IOUT_AVG,
    PMBUS_VIRT_READ_IOUT_MIN,
    PMBUS_VIRT_READ_IOUT_MAX,
    PMBUS_VIRT_RESET_IOUT_HISTORY,
    PMBUS_VIRT_READ_TEMP2_AVG,
    PMBUS_VIRT_READ_TEMP2_MIN,
    PMBUS_VIRT_READ_TEMP2_MAX,
    PMBUS_VIRT_RESET_TEMP2_HISTORY,

    PMBUS_VIRT_READ_VMON,
    PMBUS_VIRT_VMON_UV_WARN_LIMIT,
    PMBUS_VIRT_VMON_OV_WARN_LIMIT,
    PMBUS_VIRT_VMON_UV_FAULT_LIMIT,
    PMBUS_VIRT_VMON_OV_FAULT_LIMIT,
    PMBUS_VIRT_STATUS_VMON,

//
// RPM and PWM Fan control
//
// Drivers wanting to expose PWM control must define the behaviour of
// PMBUS_VIRT_PWM_[1-4] and PMBUS_VIRT_PWM_ENABLE_[1-4] in the
// {read,write}_word_data callback.
//
// pmbus core provides a default implementation for
// PMBUS_VIRT_FAN_TARGET_[1-4].
//
// TARGET, PWM and PWM_ENABLE members must be defined sequentially;
// pmbus core uses the difference between the provided register and
// it's _1 counterpart to calculate the FAN/PWM ID.
//
    PMBUS_VIRT_FAN_TARGET_1,
    PMBUS_VIRT_FAN_TARGET_2,
    PMBUS_VIRT_FAN_TARGET_3,
    PMBUS_VIRT_FAN_TARGET_4,
    PMBUS_VIRT_PWM_1,
    PMBUS_VIRT_PWM_2,
    PMBUS_VIRT_PWM_3,
    PMBUS_VIRT_PWM_4,
    PMBUS_VIRT_PWM_ENABLE_1,
    PMBUS_VIRT_PWM_ENABLE_2,
    PMBUS_VIRT_PWM_ENABLE_3,
    PMBUS_VIRT_PWM_ENABLE_4,

// Samples for average
//
// Drivers wanting to expose functionality for changing the number of
// samples used for average values should implement support in
// {read,write}_word_data callback for either PMBUS_VIRT_SAMPLES if it
// applies to all types of measurements, or any number of specific
// PMBUS_VIRT_*_SAMPLES registers to allow for individual control.
//
    PMBUS_VIRT_SAMPLES,
    PMBUS_VIRT_IN_SAMPLES,
    PMBUS_VIRT_CURR_SAMPLES,
    PMBUS_VIRT_POWER_SAMPLES,
    PMBUS_VIRT_TEMP_SAMPLES,
}

//
// OPERATION
//

//
// ON_OFF_CONFIG
//

//
// WRITE_PROTECT
//

//
// CAPABILITY
//

//
// VOUT_MODE
//
pub const PB_VOUT_MODE_MODE_MASK: c_uint = 0xe0;
pub const PB_VOUT_MODE_PARAM_MASK: c_uint = 0x1f;
pub const PB_VOUT_MODE_LINEAR: c_uint = 0x00;
pub const PB_VOUT_MODE_VID: c_uint = 0x20;
pub const PB_VOUT_MODE_DIRECT: c_uint = 0x40;
//
// Fan configuration
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pmbus_fan_mode {

//
// STATUS_BYTE, STATUS_WORD (lower)
//

//
// STATUS_WORD (upper)
//

//
// STATUS_IOUT
//

//
// STATUS_VOUT, STATUS_INPUT
//

//
// STATUS_INPUT
//

//
// STATUS_TEMPERATURE
//

//
// STATUS_FAN
//

//
// CML_FAULT_STATUS
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pmbus_sensor_classes {
    PSC_VOLTAGE_IN = 0,
    PSC_VOLTAGE_OUT,
    PSC_CURRENT_IN,
    PSC_CURRENT_OUT,
    PSC_POWER,
    PSC_TEMPERATURE,
    PSC_FAN,
    PSC_PWM,
    PSC_NUM_CLASSES		/* Number of power sensor classes */
}

// Functionality bit mask

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pmbus_data_format {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vrm_version {

// PMBus revision identifiers
pub const PMBUS_REV_10: c_uint = 0x00	/* PMBus revision 1.0 */;
pub const PMBUS_REV_11: c_uint = 0x11	/* PMBus revision 1.1 */;
pub const PMBUS_REV_12: c_uint = 0x22	/* PMBus revision 1.2 */;
pub const PMBUS_REV_13: c_uint = 0x33	/* PMBus revision 1.3 */;
pub const PMBUS_REV_131: c_uint = 0x44	/* PMBus revision 1.3.1 */;
pub const PMBUS_REV_14: c_uint = 0x55	/* PMBus revision 1.4 */;

// Operation type flags for pmbus_update_ts

    struct pmbus_driver_info {
    int pages;		/* Total number of pages */
    u8 phases[PMBUS_PAGES];	/* Number of phases per page */
    enum pmbus_data_format format[PSC_NUM_CLASSES];
    enum vrm_version vrm_version[PMBUS_PAGES]; /* vrm version per page */
//
// Support one set of coefficients for each sensor type
// Used for chips providing data in direct mode.
//
    int m[PSC_NUM_CLASSES];	/* mantissa for direct data format */
    int b[PSC_NUM_CLASSES];	/* offset */
    int R[PSC_NUM_CLASSES];	/* exponent */

    u32 func[PMBUS_PAGES];	/* Functionality, per page */
    u32 pfunc[PMBUS_PHASES];/* Functionality, per phase */
//
// The following functions map manufacturing specific register values
// to PMBus standard register values. Specify only if mapping is
// necessary.
// Functions return the register value (read) or zero (write) if
// successful. A return value of -ENODATA indicates that there is no
// manufacturer specific register, but that a standard PMBus register
// may exist. Any other negative return value indicates that the
// register does not exist, and that no attempt should be made to read
// the standard register.
//
    int (*read_byte_data)(struct i2c_client *client, int page, int reg);
    int (*read_word_data)(struct i2c_client *client, int page, int phase,
    int reg);
    int (*write_byte_data)(struct i2c_client *client, int page, int reg,
    u8 byte);
    int (*write_word_data)(struct i2c_client *client, int page, int reg,
    u16 word);
    int (*write_byte)(struct i2c_client *client, int page, u8 value);
//
// The identify function determines supported PMBus functionality.
// This function is only necessary if a chip driver supports multiple
// chips, and the chip functionality is not pre-determined.
//
    int (*identify)(struct i2c_client *client,
    struct pmbus_driver_info *info);

// Regulator functionality, if supported by this chip driver.
    int num_regulators;
    const struct regulator_desc *reg_desc;

// custom attributes
    const struct attribute_group **groups;

//
// Some chips need a little delay between SMBus communication. When
// set, the generic PMBus helper functions will wait if necessary
// to meet this requirement. The access delay is honored after
// every SMBus operation. The write delay is only honored after
// SMBus write operations.
//
    int access_delay;		/* in microseconds */
    int write_delay;		/* in microseconds */
    int page_change_delay;		/* in microseconds */

//
// Some chips do not support the PMBUS_REVISION command.
// Drivers for such chips can report the supported PMBus revision here.
//
// Drivers must set have_pmbus_revision to true and provide the
// supported PMBus version in pmbus_revision.
//
    bool have_pmbus_revision;	/* true if pmbus_revision is valid */
    u8 pmbus_revision;		/* PMBus revision */
}

// Regulator ops
// Macros for filling in array of struct regulator_desc

//
// _NODE macros are defined for historic reasons and MUST NOT be used in new
// drivers.
//

// Function declarations
extern "C" {
    pub fn pmbus_clear_cache(client: *mut i2c_client);
}
extern "C" {
    pub fn pmbus_set_update(client: *mut i2c_client, reg: u8, update: bool);
}
extern "C" {
    pub fn pmbus_wait(client: *mut i2c_client);
}
extern "C" {
    pub fn pmbus_update_ts(client: *mut i2c_client, op: c_int);
}
extern "C" {
    pub fn pmbus_set_page(client: *mut i2c_client, page: c_int, phase: c_int) -> c_int;
}
extern "C" {
    pub fn pmbus_read_smbus_i2c_block_data(client: *mut i2c_client, reg: u8, data_buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn pmbus_read_byte_data(client: *mut i2c_client, page: c_int, reg: u8) -> c_int;
}
extern "C" {
    pub fn pmbus_write_byte(client: *mut i2c_client, page: c_int, value: u8) -> c_int;
}
extern "C" {
    pub fn pmbus_clear_faults(client: *mut i2c_client);
}
extern "C" {
    pub fn pmbus_check_and_notify_faults(client: *mut i2c_client);
}
extern "C" {
    pub fn pmbus_check_byte_register(client: *mut i2c_client, page: c_int, reg: c_int) -> bool;
}
extern "C" {
    pub fn pmbus_check_word_register(client: *mut i2c_client, page: c_int, reg: c_int) -> bool;
}
extern "C" {
    pub fn pmbus_do_probe(client: *mut i2c_client, info: *mut pmbus_driver_info) -> c_int;
}
// client);
extern "C" {
    pub fn pmbus_lock_interruptible(client: *mut i2c_client) -> c_int;
}
extern "C" {
    pub fn pmbus_lock(client: *mut i2c_client);
}
extern "C" {
    pub fn pmbus_unlock(client: *mut i2c_client);
}
