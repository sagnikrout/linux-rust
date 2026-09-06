//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/kxcjk-1013.c
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
// KXCJK-1013 3-axis accelerometer driver
// Copyright (c) 2014, Intel Corporation.
//

pub const KXTF9_REG_HP_XOUT_L: c_uint = 0x00;
pub const KXTF9_REG_HP_XOUT_H: c_uint = 0x01;
pub const KXTF9_REG_HP_YOUT_L: c_uint = 0x02;
pub const KXTF9_REG_HP_YOUT_H: c_uint = 0x03;
pub const KXTF9_REG_HP_ZOUT_L: c_uint = 0x04;
pub const KXTF9_REG_HP_ZOUT_H: c_uint = 0x05;
pub const KXCJK1013_REG_XOUT_L: c_uint = 0x06;
//
// From low byte X axis register, all the other addresses of Y and Z can be
// obtained by just applying axis offset. The following axis defines are just
// provide clarity, but not used.
//
pub const KXCJK1013_REG_XOUT_H: c_uint = 0x07;
pub const KXCJK1013_REG_YOUT_L: c_uint = 0x08;
pub const KXCJK1013_REG_YOUT_H: c_uint = 0x09;
pub const KXCJK1013_REG_ZOUT_L: c_uint = 0x0A;
pub const KXCJK1013_REG_ZOUT_H: c_uint = 0x0B;
pub const KXCJK1013_REG_DCST_RESP: c_uint = 0x0C;
pub const KXCJK1013_REG_WHO_AM_I: c_uint = 0x0F;
pub const KXTF9_REG_TILT_POS_CUR: c_uint = 0x10;
pub const KXTF9_REG_TILT_POS_PREV: c_uint = 0x11;
pub const KXTF9_REG_INT_SRC1: c_uint = 0x15;
pub const KXTF9_REG_INT_SRC2: c_uint = 0x16;
pub const KXCJK1013_REG_INT_SRC1: c_uint = 0x16;
pub const KXCJK1013_REG_INT_SRC2: c_uint = 0x17;
pub const KXCJK1013_REG_STATUS_REG: c_uint = 0x18;
pub const KXCJK1013_REG_INT_REL: c_uint = 0x1A;
pub const KXCJK1013_REG_CTRL1: c_uint = 0x1B;
pub const KXTF9_REG_CTRL2: c_uint = 0x1C;
pub const KXTF9_REG_CTRL3: c_uint = 0x1D;
pub const KXCJK1013_REG_CTRL2: c_uint = 0x1D;
pub const KXCJK1013_REG_INT_CTRL1: c_uint = 0x1E;
pub const KXCJK1013_REG_INT_CTRL2: c_uint = 0x1F;
pub const KXTF9_REG_INT_CTRL3: c_uint = 0x20;
pub const KXCJK1013_REG_DATA_CTRL: c_uint = 0x21;
pub const KXTF9_REG_TILT_TIMER: c_uint = 0x28;
pub const KXCJK1013_REG_WAKE_TIMER: c_uint = 0x29;
pub const KXTF9_REG_TDT_TIMER: c_uint = 0x2B;
pub const KXTF9_REG_TDT_THRESH_H: c_uint = 0x2C;
pub const KXTF9_REG_TDT_THRESH_L: c_uint = 0x2D;
pub const KXTF9_REG_TDT_TAP_TIMER: c_uint = 0x2E;
pub const KXTF9_REG_TDT_TOTAL_TIMER: c_uint = 0x2F;
pub const KXTF9_REG_TDT_LATENCY_TIMER: c_uint = 0x30;
pub const KXTF9_REG_TDT_WINDOW_TIMER: c_uint = 0x31;
pub const KXCJK1013_REG_SELF_TEST: c_uint = 0x3A;
pub const KXTF9_REG_WAKE_THRESH: c_uint = 0x5A;
pub const KXTF9_REG_TILT_ANGLE: c_uint = 0x5C;
pub const KXTF9_REG_HYST_SET: c_uint = 0x5F;
pub const KXCJK1013_REG_WAKE_THRES: c_uint = 0x6A;
// Everything up to 0x11 is equal to KXCJK1013/KXTF9 above
pub const KX023_REG_INS1: c_uint = 0x12;
pub const KX023_REG_INS2: c_uint = 0x13;
pub const KX023_REG_INS3: c_uint = 0x14;
pub const KX023_REG_STAT: c_uint = 0x15;
pub const KX023_REG_INT_REL: c_uint = 0x17;
pub const KX023_REG_CNTL1: c_uint = 0x18;
pub const KX023_REG_CNTL2: c_uint = 0x19;
pub const KX023_REG_CNTL3: c_uint = 0x1A;
pub const KX023_REG_ODCNTL: c_uint = 0x1B;
pub const KX023_REG_INC1: c_uint = 0x1C;
pub const KX023_REG_INC2: c_uint = 0x1D;
pub const KX023_REG_INC3: c_uint = 0x1E;
pub const KX023_REG_INC4: c_uint = 0x1F;
pub const KX023_REG_INC5: c_uint = 0x20;
pub const KX023_REG_INC6: c_uint = 0x21;
pub const KX023_REG_TILT_TIMER: c_uint = 0x22;
pub const KX023_REG_WUFC: c_uint = 0x23;
pub const KX023_REG_TDTRC: c_uint = 0x24;
pub const KX023_REG_TDTC: c_uint = 0x25;
pub const KX023_REG_TTH: c_uint = 0x26;
pub const KX023_REG_TTL: c_uint = 0x27;
pub const KX023_REG_FTD: c_uint = 0x28;
pub const KX023_REG_STD: c_uint = 0x29;
pub const KX023_REG_TLT: c_uint = 0x2A;
pub const KX023_REG_TWS: c_uint = 0x2B;
pub const KX023_REG_ATH: c_uint = 0x30;
pub const KX023_REG_TILT_ANGLE_LL: c_uint = 0x32;
pub const KX023_REG_TILT_ANGLE_HL: c_uint = 0x33;
pub const KX023_REG_HYST_SET: c_uint = 0x34;
pub const KX023_REG_LP_CNTL: c_uint = 0x35;
pub const KX023_REG_BUF_CNTL1: c_uint = 0x3A;
pub const KX023_REG_BUF_CNTL2: c_uint = 0x3B;
pub const KX023_REG_BUF_STATUS_1: c_uint = 0x3C;
pub const KX023_REG_BUF_STATUS_2: c_uint = 0x3D;
pub const KX023_REG_BUF_CLEAR: c_uint = 0x3E;
pub const KX023_REG_BUF_READ: c_uint = 0x3F;
pub const KX023_REG_SELF_TEST: c_uint = 0x60;

pub const KXCJK1013_DATA_MASK_12_BIT: c_uint = 0x0FFF;
pub const KXCJK1013_MAX_STARTUP_TIME_US: c_int = 100000;
pub const KXCJK1013_SLEEP_DELAY_MS: c_int = 2000;

pub const KXCJK1013_REG_INT_SRC1_TAP_NONE: c_int = 0;

// KXCJK: INT_SOURCE2: motion detect, KXTF9: INT_SRC_REG1: tap detect

// KX023 interrupt routing to INT1. INT2 can be configured with INC6

pub const KXCJK1013_DEFAULT_WAKE_THRES: c_int = 1;
// Refer to section 4 of the specification
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kx_odr_start_up_time {
    pub odr_bits: c_int,
    pub usec: c_int,
}

// KXCJK-1013
    static const struct kx_odr_start_up_time kxcjk1013_odr_start_up_times[] = {
    { 0x08, 100000 },
    { 0x09, 100000 },
    { 0x0A, 100000 },
    { 0x0B, 100000 },
    { 0x00, 80000 },
    { 0x01, 41000 },
    { 0x02, 21000 },
    { 0x03, 11000 },
    { 0x04, 6400 },
    { 0x05, 3900 },
    { 0x06, 2700 },
    { 0x07, 2100 },
    { }
    };
// KXCTJ2-1009
    static const struct kx_odr_start_up_time kxtj21009_odr_start_up_times[] = {
    { 0x08, 1240000 },
    { 0x09, 621000 },
    { 0x0A, 309000 },
    { 0x0B, 151000 },
    { 0x00, 80000 },
    { 0x01, 41000 },
    { 0x02, 21000 },
    { 0x03, 11000 },
    { 0x04, 6000 },
    { 0x05, 4000 },
    { 0x06, 3000 },
    { 0x07, 2000 },
    { }
    };
// KXTF9
    static const struct kx_odr_start_up_time kxtf9_odr_start_up_times[] = {
    { 0x01, 81000 },
    { 0x02, 41000 },
    { 0x03, 21000 },
    { 0x04, 11000 },
    { 0x05, 5100 },
    { 0x06, 2700 },
    { }
    };
// KX023-1025
    static const struct kx_odr_start_up_time kx0231025_odr_start_up_times[] = {
// First 4 are not in datasheet, taken from KXCTJ2-1009
    { 0x08, 1240000 },
    { 0x09, 621000 },
    { 0x0A, 309000 },
    { 0x0B, 151000 },
    { 0x00, 81000 },
    { 0x01, 40000 },
    { 0x02, 22000 },
    { 0x03, 12000 },
    { 0x04, 7000 },
    { 0x05, 4400 },
    { 0x06, 3000 },
    { 0x07, 3000 },
    { }
    };
    enum kx_acpi_type {
    ACPI_GENERIC,
    ACPI_SMO8500,
    ACPI_KIOX010A,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kx_chipset_regs {
    pub int_src1: u8,
    pub int_src2: u8,
    pub int_rel: u8,
    pub ctrl1: u8,
    pub wuf_ctrl: u8,
    pub int_ctrl1: u8,
    pub data_ctrl: u8,
    pub wake_timer: u8,
    pub wake_thres: u8,
}

    static const struct kx_chipset_regs kxcjk1013_regs = {
    .int_src1	= KXCJK1013_REG_INT_SRC1,
    .int_src2	= KXCJK1013_REG_INT_SRC2,
    .int_rel	= KXCJK1013_REG_INT_REL,
    .ctrl1		= KXCJK1013_REG_CTRL1,
    .wuf_ctrl	= KXCJK1013_REG_CTRL2,
    .int_ctrl1	= KXCJK1013_REG_INT_CTRL1,
    .data_ctrl	= KXCJK1013_REG_DATA_CTRL,
    .wake_timer	= KXCJK1013_REG_WAKE_TIMER,
    .wake_thres	= KXCJK1013_REG_WAKE_THRES,
    };
    static const struct kx_chipset_regs kxtf9_regs = {
// .int_src1 was moved to INT_SRC2 on KXTF9
    .int_src1	= KXTF9_REG_INT_SRC2,
// .int_src2 is not available
    .int_rel	= KXCJK1013_REG_INT_REL,
    .ctrl1		= KXCJK1013_REG_CTRL1,
    .wuf_ctrl	= KXTF9_REG_CTRL3,
    .int_ctrl1	= KXCJK1013_REG_INT_CTRL1,
    .data_ctrl	= KXCJK1013_REG_DATA_CTRL,
    .wake_timer	= KXCJK1013_REG_WAKE_TIMER,
    .wake_thres	= KXTF9_REG_WAKE_THRESH,
    };
// The registers have totally different names but the bits are compatible
    static const struct kx_chipset_regs kx0231025_regs = {
    .int_src1	= KX023_REG_INS2,
    .int_src2	= KX023_REG_INS3,
    .int_rel	= KX023_REG_INT_REL,
    .ctrl1		= KX023_REG_CNTL1,
    .wuf_ctrl	= KX023_REG_CNTL3,
    .int_ctrl1	= KX023_REG_INC1,
    .data_ctrl	= KX023_REG_ODCNTL,
    .wake_timer	= KX023_REG_WUFC,
    .wake_thres	= KX023_REG_ATH,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kx_chipset_info {
    pub regs: *const kx_chipset_regs,
    pub times: *const kx_odr_start_up_time,
    pub acpi_type: enum kx_acpi_type,
}

    static const struct kx_chipset_info kxcjk1013_info = {
    .regs = &kxcjk1013_regs,
    .times = pm_ptr(kxcjk1013_odr_start_up_times),
    };
    static const struct kx_chipset_info kxcj91008_info = {
    .regs = &kxcjk1013_regs,
    .times = pm_ptr(kxcjk1013_odr_start_up_times),
    };
    static const struct kx_chipset_info kxcj91008_kiox010a_info = {
    .regs = &kxcjk1013_regs,
    .times = pm_ptr(kxcjk1013_odr_start_up_times),
    .acpi_type = ACPI_KIOX010A,
    };
    static const struct kx_chipset_info kxcj91008_kiox020a_info = {
    .regs = &kxcjk1013_regs,
    .times = pm_ptr(kxcjk1013_odr_start_up_times),
    .acpi_type = ACPI_GENERIC,
    };
    static const struct kx_chipset_info kxcj91008_smo8500_info = {
    .regs = &kxcjk1013_regs,
    .times = pm_ptr(kxcjk1013_odr_start_up_times),
    .acpi_type = ACPI_SMO8500,
    };
    static const struct kx_chipset_info kxtj21009_info = {
    .regs = &kxcjk1013_regs,
    .times = pm_ptr(kxtj21009_odr_start_up_times),
    };
    static const struct kx_chipset_info kxtf9_info = {
    .regs = &kxtf9_regs,
    .times = pm_ptr(kxtf9_odr_start_up_times),
    };
    static const struct kx_chipset_info kx0231025_info = {
    .regs = &kx0231025_regs,
    .times = pm_ptr(kx0231025_odr_start_up_times),
    };
    enum kxcjk1013_axis {
    AXIS_X,
    AXIS_Y,
    AXIS_Z,
    AXIS_MAX
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kxcjk1013_data {
    pub client: *mut i2c_client,
    pub dready_trig: *mut iio_trigger,
    pub motion_trig: *mut iio_trigger,
    pub orientation: iio_mount_matrix,
    pub mutex: mutex,
// Ensure timestamp naturally aligned
    struct {
    pub chans: [i16; AXIS_MAX],
    pub timestamp: aligned_s64,
    pub scan: },
    pub odr_bits: u8,
    pub range: u8,
    pub wake_thres: c_int,
    pub wake_dur: c_int,
    pub active_high_intr: bool,
    pub dready_trigger_on: bool,
    pub ev_enable_state: c_int,
    pub motion_trigger_on: bool,
    pub timestamp: i64,
    pub info: *const kx_chipset_info,
}

    enum kxcjk1013_mode {
    STANDBY,
    OPERATION,
    };
    enum kxcjk1013_range {
    KXCJK1013_RANGE_2G,
    KXCJK1013_RANGE_4G,
    KXCJK1013_RANGE_8G,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kx_odr_map {
    pub val: c_int,
    pub val2: c_int,
    pub odr_bits: c_int,
    pub wuf_bits: c_int,
}

    static const struct kx_odr_map samp_freq_table[] = {
    { 0, 781000, 0x08, 0x00 },
    { 1, 563000, 0x09, 0x01 },
    { 3, 125000, 0x0A, 0x02 },
    { 6, 250000, 0x0B, 0x03 },
    { 12, 500000, 0x00, 0x04 },
    { 25, 0, 0x01, 0x05 },
    { 50, 0, 0x02, 0x06 },
    { 100, 0, 0x03, 0x06 },
    { 200, 0, 0x04, 0x06 },
    { 400, 0, 0x05, 0x06 },
    { 800, 0, 0x06, 0x06 },
    { 1600, 0, 0x07, 0x06 },
    };
    static const char *const kxcjk1013_samp_freq_avail =
    "0.781000 1.563000 3.125000 6.250000 12.500000 25 50 100 200 400 800 1600";
    static const struct kx_odr_map kxtf9_samp_freq_table[] = {
    { 25, 0, 0x01, 0x00 },
    { 50, 0, 0x02, 0x01 },
    { 100, 0, 0x03, 0x01 },
    { 200, 0, 0x04, 0x01 },
    { 400, 0, 0x05, 0x01 },
    { 800, 0, 0x06, 0x01 },
    };
    static const char *const kxtf9_samp_freq_avail =
    "25 50 100 200 400 800";
    static const struct {
    u16 scale;
    u8 gsel_0;
    u8 gsel_1;
    } KXCJK1013_scale_table[] = { {9582, 0, 0},
    {19163, 1, 0},
    {38326, 0, 1} };

    enum kiox010a_fn_index {
    KIOX010A_SET_LAPTOP_MODE = 1,
    KIOX010A_SET_TABLET_MODE = 2,
    };
#[no_mangle]
unsafe extern "C" fn kiox010a_dsm(dev: *mut device, fn_index: c_int) -> c_int {
    static int kiox010a_dsm(struct device *dev, int fn_index)
    {
    let mut handle: acpi_handle = ACPI_HANDLE(dev);
    guid_t kiox010a_dsm_guid;
    union acpi_object *obj;
    if (!handle)
    return -ENODEV;
    guid_parse("1f339696-d475-4e26-8cad-2e9f8e6d7a91", &kiox010a_dsm_guid);
    obj = acpi_evaluate_dsm(handle, &kiox010a_dsm_guid, 1, fn_index, core::ptr::null_mut());
    if (!obj)
    return -EIO;
    ACPI_FREE(obj);
    return 0;
    }

    static int kxcjk1013_set_mode(struct kxcjk1013_data *data,
    enum kxcjk1013_mode mode)
    {
    const struct kx_chipset_regs *regs = data.info.regs;
    int ret;
    ret = i2c_smbus_read_byte_data(data.client, regs.ctrl1);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error reading reg_ctrl1\n");
    return ret;
    }
    if (mode == STANDBY)
    ret &= ~KXCJK1013_REG_CTRL1_BIT_PC1;
    else
    ret |= KXCJK1013_REG_CTRL1_BIT_PC1;
    ret = i2c_smbus_write_byte_data(data.client, regs.ctrl1, ret);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error writing reg_ctrl1\n");
    return ret;
    }
    return 0;
    }
    static int kxcjk1013_get_mode(struct kxcjk1013_data *data,
    enum kxcjk1013_mode *mode)
    {
    const struct kx_chipset_regs *regs = data.info.regs;
    int ret;
    ret = i2c_smbus_read_byte_data(data.client, regs.ctrl1);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error reading reg_ctrl1\n");
    return ret;
    }
    if (ret & KXCJK1013_REG_CTRL1_BIT_PC1)
// mode = OPERATION;
    else
// mode = STANDBY;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_set_range(data: *mut kxcjk1013_data, range_index: c_int) -> c_int {
    static int kxcjk1013_set_range(struct kxcjk1013_data *data, int range_index)
    {
    const struct kx_chipset_regs *regs = data.info.regs;
    int ret;
    ret = i2c_smbus_read_byte_data(data.client, regs.ctrl1);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error reading reg_ctrl1\n");
    return ret;
    }
    ret &= ~(KXCJK1013_REG_CTRL1_BIT_GSEL0 |
    KXCJK1013_REG_CTRL1_BIT_GSEL1);
    ret |= (KXCJK1013_scale_table[range_index].gsel_0 << 3);
    ret |= (KXCJK1013_scale_table[range_index].gsel_1 << 4);
    ret = i2c_smbus_write_byte_data(data.client, regs.ctrl1, ret);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error writing reg_ctrl1\n");
    return ret;
    }
    data.range = range_index;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_chip_init(data: *mut kxcjk1013_data) -> c_int {
    static int kxcjk1013_chip_init(struct kxcjk1013_data *data)
    {
    const struct kx_chipset_regs *regs = data.info.regs;
    int ret;

    if (data.info.acpi_type == ACPI_KIOX010A) {
// Make sure the kbd and touchpad on 2-in-1s using 2 KXCJ91008-s work
    kiox010a_dsm(&data.client.dev, KIOX010A_SET_LAPTOP_MODE);
    }

    ret = i2c_smbus_read_byte_data(data.client, KXCJK1013_REG_WHO_AM_I);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error reading who_am_i\n");
    return ret;
    }
    dev_dbg(&data.client.dev, "KXCJK1013 Chip Id %x\n", ret);
    ret = kxcjk1013_set_mode(data, STANDBY);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_read_byte_data(data.client, regs.ctrl1);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error reading reg_ctrl1\n");
    return ret;
    }
// Set 12 bit mode
    ret |= KXCJK1013_REG_CTRL1_BIT_RES;
    ret = i2c_smbus_write_byte_data(data.client, regs.ctrl1, ret);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error reading reg_ctrl\n");
    return ret;
    }
// Setting range to 4G
    ret = kxcjk1013_set_range(data, KXCJK1013_RANGE_4G);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_read_byte_data(data.client, regs.data_ctrl);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error reading reg_data_ctrl\n");
    return ret;
    }
    data.odr_bits = ret;
// Set up INT polarity
    ret = i2c_smbus_read_byte_data(data.client, regs.int_ctrl1);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error reading reg_int_ctrl1\n");
    return ret;
    }
    if (data.active_high_intr)
    ret |= KXCJK1013_REG_INT_CTRL1_BIT_IEA;
    else
    ret &= ~KXCJK1013_REG_INT_CTRL1_BIT_IEA;
    ret = i2c_smbus_write_byte_data(data.client, regs.int_ctrl1, ret);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error writing reg_int_ctrl1\n");
    return ret;
    }
// On KX023, route all used interrupts to INT1 for now
    if (data.info == &kx0231025_info && data.client.irq > 0) {
    ret = i2c_smbus_write_byte_data(data.client, KX023_REG_INC4,
    KX023_REG_INC4_DRDY1 |
    KX023_REG_INC4_WUFI1);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error writing reg_inc4\n");
    return ret;
    }
    }
    ret = kxcjk1013_set_mode(data, OPERATION);
    if (ret < 0)
    return ret;
    data.wake_thres = KXCJK1013_DEFAULT_WAKE_THRES;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_get_startup_times(data: *mut kxcjk1013_data) -> c_int {
    static int kxcjk1013_get_startup_times(struct kxcjk1013_data *data)
    {
    const struct kx_odr_start_up_time *times;
    for (times = data.info.times; times.usec; times++) {
    if (times.odr_bits == data.odr_bits)
    return times.usec;
    }
    return KXCJK1013_MAX_STARTUP_TIME_US;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_set_power_state(data: *mut kxcjk1013_data, on: bool) -> c_int {
    static int kxcjk1013_set_power_state(struct kxcjk1013_data *data, bool on)
    {

    int ret;
    if (on)
    ret = pm_runtime_resume_and_get(&data.client.dev);
    else
    ret = pm_runtime_put_autosuspend(&data.client.dev);
    if (ret < 0) {
    dev_err(&data.client.dev,
    "Failed: %s for %d\n", __func__, on);
    return ret;
    }

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_chip_update_thresholds(data: *mut kxcjk1013_data) -> c_int {
    static int kxcjk1013_chip_update_thresholds(struct kxcjk1013_data *data)
    {
    const struct kx_chipset_regs *regs = data.info.regs;
    int ret;
    ret = i2c_smbus_write_byte_data(data.client, regs.wake_timer, data.wake_dur);
    if (ret < 0) {
    dev_err(&data.client.dev,
    "Error writing reg_wake_timer\n");
    return ret;
    }
    ret = i2c_smbus_write_byte_data(data.client, regs.wake_thres, data.wake_thres);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error writing reg_wake_thres\n");
    return ret;
    }
    return 0;
    }
    static int kxcjk1013_setup_interrupt(struct kxcjk1013_data *data,
    bool status, bool is_new_data)
    {
    const struct kx_chipset_regs *regs = data.info.regs;
    int ret;
    enum kxcjk1013_mode store_mode;
    ret = kxcjk1013_get_mode(data, &store_mode);
    if (ret < 0)
    return ret;
// This is requirement by spec to change state to STANDBY
    ret = kxcjk1013_set_mode(data, STANDBY);
    if (ret < 0)
    return ret;
    if (is_new_data == true) {
    ret = kxcjk1013_chip_update_thresholds(data);
    if (ret < 0)
    return ret;
    }
    ret = i2c_smbus_read_byte_data(data.client, regs.int_ctrl1);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error reading reg_int_ctrl1\n");
    return ret;
    }
    if (status)
    ret |= KXCJK1013_REG_INT_CTRL1_BIT_IEN;
    else
    ret &= ~KXCJK1013_REG_INT_CTRL1_BIT_IEN;
    ret = i2c_smbus_write_byte_data(data.client, regs.int_ctrl1, ret);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error writing reg_int_ctrl1\n");
    return ret;
    }
    ret = i2c_smbus_read_byte_data(data.client, regs.ctrl1);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error reading reg_ctrl1\n");
    return ret;
    }
    if (is_new_data) {
    if (status)
    ret |= KXCJK1013_REG_CTRL1_BIT_DRDY;
    else
    ret &= ~KXCJK1013_REG_CTRL1_BIT_DRDY;
    } else {
    if (status)
    ret |= KXCJK1013_REG_CTRL1_BIT_WUFE;
    else
    ret &= ~KXCJK1013_REG_CTRL1_BIT_WUFE;
    }
    ret = i2c_smbus_write_byte_data(data.client, regs.ctrl1, ret);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error writing reg_ctrl1\n");
    return ret;
    }
    if (store_mode == OPERATION) {
    ret = kxcjk1013_set_mode(data, OPERATION);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
    static const struct kx_odr_map *kxcjk1013_find_odr_value(
    const struct kx_odr_map *map, size_t map_size, int val, int val2)
    {
    int i;
    for (i = 0; i < map_size; ++i) {
    if (map[i].val == val && map[i].val2 == val2)
    return &map[i];
    }
    return ERR_PTR(-EINVAL);
    }
    static int kxcjk1013_convert_odr_value(const struct kx_odr_map *map,
    size_t map_size, int odr_bits,
    int *val, int *val2)
    {
    int i;
    for (i = 0; i < map_size; ++i) {
    if (map[i].odr_bits == odr_bits) {
// val = map[i].val;
// val2 = map[i].val2;
    return IIO_VAL_INT_PLUS_MICRO;
    }
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_set_odr(data: *mut kxcjk1013_data, val: c_int, val2: c_int) -> c_int {
    static int kxcjk1013_set_odr(struct kxcjk1013_data *data, int val, int val2)
    {
    const struct kx_chipset_regs *regs = data.info.regs;
    int ret;
    enum kxcjk1013_mode store_mode;
    const struct kx_odr_map *odr_setting;
    ret = kxcjk1013_get_mode(data, &store_mode);
    if (ret < 0)
    return ret;
    if (data.info == &kxtf9_info)
    odr_setting = kxcjk1013_find_odr_value(kxtf9_samp_freq_table,
    ARRAY_SIZE(kxtf9_samp_freq_table),
    val, val2);
    else
    odr_setting = kxcjk1013_find_odr_value(samp_freq_table,
    ARRAY_SIZE(samp_freq_table),
    val, val2);
    if (IS_ERR(odr_setting))
    return PTR_ERR(odr_setting);
// To change ODR, the chip must be set to STANDBY as per spec
    ret = kxcjk1013_set_mode(data, STANDBY);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(data.client, regs.data_ctrl,
    odr_setting.odr_bits);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error writing data_ctrl\n");
    return ret;
    }
    data.odr_bits = odr_setting.odr_bits;
    ret = i2c_smbus_write_byte_data(data.client, regs.wuf_ctrl,
    odr_setting.wuf_bits);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error writing reg_ctrl2\n");
    return ret;
    }
    if (store_mode == OPERATION) {
    ret = kxcjk1013_set_mode(data, OPERATION);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_get_odr(data: *mut kxcjk1013_data, val: *mut c_int, val2: *mut c_int) -> c_int {
    static int kxcjk1013_get_odr(struct kxcjk1013_data *data, int *val, int *val2)
    {
    if (data.info == &kxtf9_info)
    return kxcjk1013_convert_odr_value(kxtf9_samp_freq_table,
    ARRAY_SIZE(kxtf9_samp_freq_table),
    data.odr_bits, val, val2);
    else
    return kxcjk1013_convert_odr_value(samp_freq_table,
    ARRAY_SIZE(samp_freq_table),
    data.odr_bits, val, val2);
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_get_acc_reg(data: *mut kxcjk1013_data, axis: c_int) -> c_int {
    static int kxcjk1013_get_acc_reg(struct kxcjk1013_data *data, int axis)
    {
    let mut reg: u8 = KXCJK1013_REG_XOUT_L + axis * 2;
    int ret;
    ret = i2c_smbus_read_word_data(data.client, reg);
    if (ret < 0) {
    dev_err(&data.client.dev,
    "failed to read accel_%c registers\n", 'x' + axis);
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_set_scale(data: *mut kxcjk1013_data, val: c_int) -> c_int {
    static int kxcjk1013_set_scale(struct kxcjk1013_data *data, int val)
    {
    int ret, i;
    enum kxcjk1013_mode store_mode;
    for (i = 0; i < ARRAY_SIZE(KXCJK1013_scale_table); ++i) {
    if (KXCJK1013_scale_table[i].scale == val) {
    ret = kxcjk1013_get_mode(data, &store_mode);
    if (ret < 0)
    return ret;
    ret = kxcjk1013_set_mode(data, STANDBY);
    if (ret < 0)
    return ret;
    ret = kxcjk1013_set_range(data, i);
    if (ret < 0)
    return ret;
    if (store_mode == OPERATION) {
    ret = kxcjk1013_set_mode(data, OPERATION);
    if (ret)
    return ret;
    }
    return 0;
    }
    }
    return -EINVAL;
    }
    static int kxcjk1013_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    mutex_lock(&data.mutex);
    if (iio_buffer_enabled(indio_dev))
    ret = -EBUSY;
    else {
    ret = kxcjk1013_set_power_state(data, true);
    if (ret < 0) {
    mutex_unlock(&data.mutex);
    return ret;
    }
    ret = kxcjk1013_get_acc_reg(data, chan.scan_index);
    if (ret < 0) {
    kxcjk1013_set_power_state(data, false);
    mutex_unlock(&data.mutex);
    return ret;
    }
// val = sign_extend32(ret >> chan->scan_type.shift,
    chan.scan_type.realbits - 1);
    ret = kxcjk1013_set_power_state(data, false);
    }
    mutex_unlock(&data.mutex);
    if (ret < 0)
    return ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 0;
// val2 = KXCJK1013_scale_table[data->range].scale;
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_CHAN_INFO_SAMP_FREQ:
    mutex_lock(&data.mutex);
    ret = kxcjk1013_get_odr(data, val, val2);
    mutex_unlock(&data.mutex);
    return ret;
    default:
    return -EINVAL;
    }
    }
    static int kxcjk1013_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int val,
    int val2, long mask)
    {
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_SAMP_FREQ:
    mutex_lock(&data.mutex);
    ret = kxcjk1013_set_odr(data, val, val2);
    mutex_unlock(&data.mutex);
    break;
    case IIO_CHAN_INFO_SCALE:
    if (val)
    return -EINVAL;
    mutex_lock(&data.mutex);
    ret = kxcjk1013_set_scale(data, val2);
    mutex_unlock(&data.mutex);
    break;
    default:
    ret = -EINVAL;
    }
    return ret;
    }
    static int kxcjk1013_read_event(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info,
    int *val, int *val2)
    {
    struct kxcjk1013_data *data = iio_priv(indio_dev);
// val2 = 0;
    switch (info) {
    case IIO_EV_INFO_VALUE:
// val = data->wake_thres;
    break;
    case IIO_EV_INFO_PERIOD:
// val = data->wake_dur;
    break;
    default:
    return -EINVAL;
    }
    return IIO_VAL_INT;
    }
    static int kxcjk1013_write_event(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info,
    int val, int val2)
    {
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    if (data.ev_enable_state)
    return -EBUSY;
    switch (info) {
    case IIO_EV_INFO_VALUE:
    data.wake_thres = val;
    break;
    case IIO_EV_INFO_PERIOD:
    data.wake_dur = val;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int kxcjk1013_read_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir)
    {
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    return data.ev_enable_state;
    }
    static int kxcjk1013_write_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    bool state)
    {
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    int ret;
    if (state && data.ev_enable_state)
    return 0;
    mutex_lock(&data.mutex);
    if (!state && data.motion_trigger_on) {
    data.ev_enable_state = 0;
    mutex_unlock(&data.mutex);
    return 0;
    }
//
// We will expect the enable and disable to do operation in
// reverse order. This will happen here anyway as our
// resume operation uses sync mode runtime pm calls, the
// suspend operation will be delayed by autosuspend delay
// So the disable operation will still happen in reverse of
// enable operation. When runtime pm is disabled the mode
// is always on so sequence doesn't matter
//
    ret = kxcjk1013_set_power_state(data, state);
    if (ret < 0) {
    mutex_unlock(&data.mutex);
    return ret;
    }
    ret =  kxcjk1013_setup_interrupt(data, state, false);
    if (ret < 0) {
    kxcjk1013_set_power_state(data, false);
    data.ev_enable_state = 0;
    mutex_unlock(&data.mutex);
    return ret;
    }
    data.ev_enable_state = state;
    mutex_unlock(&data.mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_buffer_preenable(indio_dev: *mut iio_dev) -> c_int {
    static int kxcjk1013_buffer_preenable(struct iio_dev *indio_dev)
    {
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    return kxcjk1013_set_power_state(data, true);
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_buffer_postdisable(indio_dev: *mut iio_dev) -> c_int {
    static int kxcjk1013_buffer_postdisable(struct iio_dev *indio_dev)
    {
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    return kxcjk1013_set_power_state(data, false);
    }
    static ssize_t kxcjk1013_get_samp_freq_avail(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct iio_dev *indio_dev = dev_to_iio_dev(dev);
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    const char *str;
    if (data.info == &kxtf9_info)
    str = kxtf9_samp_freq_avail;
    else
    str = kxcjk1013_samp_freq_avail;
    return sprintf(buf, "%s\n", str);
    }
    static IIO_DEVICE_ATTR(in_accel_sampling_frequency_available, S_IRUGO,
    kxcjk1013_get_samp_freq_avail, core::ptr::null_mut(), 0);
    static IIO_CONST_ATTR(in_accel_scale_available, "0.009582 0.019163 0.038326");
    static struct attribute *kxcjk1013_attributes[] = {
    &iio_dev_attr_in_accel_sampling_frequency_available.dev_attr.attr,
    &iio_const_attr_in_accel_scale_available.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group kxcjk1013_attrs_group = {
    .attrs = kxcjk1013_attributes,
    };
    static const struct iio_event_spec kxcjk1013_event = {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_EITHER,
    .mask_separate = BIT(IIO_EV_INFO_VALUE) |
    BIT(IIO_EV_INFO_ENABLE) |
    BIT(IIO_EV_INFO_PERIOD)
    };
    static const struct iio_mount_matrix *
    kxcjk1013_get_mount_matrix(const struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan)
    {
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    return &data.orientation;
    }
    static const struct iio_chan_spec_ext_info kxcjk1013_ext_info[] = {
    IIO_MOUNT_MATRIX(IIO_SHARED_BY_TYPE, kxcjk1013_get_mount_matrix),
    { }
    };

    .type = IIO_ACCEL,						\
    .modified = 1,							\
    .channel2 = IIO_MOD_##_axis,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),			\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) |		\
    BIT(IIO_CHAN_INFO_SAMP_FREQ),		\
    .scan_index = AXIS_##_axis,					\
    .scan_type = {							\
    .sign = 's',						\
    .realbits = 12,						\
    .storagebits = 16,					\
    .shift = 4,						\
    .endianness = IIO_LE,					\
    },								\
    .event_spec = &kxcjk1013_event,				\
    .ext_info = kxcjk1013_ext_info,					\
    .num_event_specs = 1						\
    }
    static const struct iio_chan_spec kxcjk1013_channels[] = {
    KXCJK1013_CHANNEL(X),
    KXCJK1013_CHANNEL(Y),
    KXCJK1013_CHANNEL(Z),
    IIO_CHAN_SOFT_TIMESTAMP(3),
    };
    static const struct iio_buffer_setup_ops kxcjk1013_buffer_setup_ops = {
    .preenable		= kxcjk1013_buffer_preenable,
    .postdisable		= kxcjk1013_buffer_postdisable,
    };
    static const struct iio_info kxcjk1013_iio_info = {
    .attrs			= &kxcjk1013_attrs_group,
    .read_raw		= kxcjk1013_read_raw,
    .write_raw		= kxcjk1013_write_raw,
    .read_event_value	= kxcjk1013_read_event,
    .write_event_value	= kxcjk1013_write_event,
    .write_event_config	= kxcjk1013_write_event_config,
    .read_event_config	= kxcjk1013_read_event_config,
    };
    static const unsigned long kxcjk1013_scan_masks[] = {0x7, 0};
#[no_mangle]
unsafe extern "C" fn kxcjk1013_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t kxcjk1013_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    int ret;
    mutex_lock(&data.mutex);
    ret = i2c_smbus_read_i2c_block_data_or_emulated(data.client,
    KXCJK1013_REG_XOUT_L,
    AXIS_MAX * 2,
    (u8 *)data.scan.chans);
    mutex_unlock(&data.mutex);
    if (ret < 0)
    goto err;
    iio_push_to_buffers_with_ts(indio_dev, &data.scan, sizeof(data.scan),
    data.timestamp);
    err:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_trig_reen(trig: *mut iio_trigger) {
    static void kxcjk1013_trig_reen(struct iio_trigger *trig)
    {
    struct iio_dev *indio_dev = iio_trigger_get_drvdata(trig);
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    const struct kx_chipset_regs *regs = data.info.regs;
    int ret;
    ret = i2c_smbus_read_byte_data(data.client, regs.int_rel);
    if (ret < 0)
    dev_err(&data.client.dev, "Error reading reg_int_rel\n");
    }
    static int kxcjk1013_data_rdy_trigger_set_state(struct iio_trigger *trig,
    bool state)
    {
    struct iio_dev *indio_dev = iio_trigger_get_drvdata(trig);
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    int ret;
    mutex_lock(&data.mutex);
    if (!state && data.ev_enable_state && data.motion_trigger_on) {
    data.motion_trigger_on = false;
    mutex_unlock(&data.mutex);
    return 0;
    }
    ret = kxcjk1013_set_power_state(data, state);
    if (ret < 0) {
    mutex_unlock(&data.mutex);
    return ret;
    }
    ret = kxcjk1013_setup_interrupt(data, state, data.motion_trig != trig);
    if (ret < 0) {
    kxcjk1013_set_power_state(data, false);
    mutex_unlock(&data.mutex);
    return ret;
    }
    if (data.motion_trig == trig)
    data.motion_trigger_on = state;
    else
    data.dready_trigger_on = state;
    mutex_unlock(&data.mutex);
    return 0;
    }
    static const struct iio_trigger_ops kxcjk1013_trigger_ops = {
    .set_trigger_state = kxcjk1013_data_rdy_trigger_set_state,
    .reenable = kxcjk1013_trig_reen,
    };
#[no_mangle]
unsafe extern "C" fn kxcjk1013_report_motion_event(indio_dev: *mut iio_dev) {
    static void kxcjk1013_report_motion_event(struct iio_dev *indio_dev)
    {
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    const struct kx_chipset_regs *regs = data.info.regs;
    let mut ret: c_int = i2c_smbus_read_byte_data(data.client, regs.int_src2);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error reading reg_int_src2\n");
    return;
    }
    if (ret & KXCJK1013_REG_INT_SRC2_BIT_XN)
    iio_push_event(indio_dev,
    IIO_MOD_EVENT_CODE(IIO_ACCEL,
    0,
    IIO_MOD_X,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_FALLING),
    data.timestamp);
    if (ret & KXCJK1013_REG_INT_SRC2_BIT_XP)
    iio_push_event(indio_dev,
    IIO_MOD_EVENT_CODE(IIO_ACCEL,
    0,
    IIO_MOD_X,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_RISING),
    data.timestamp);
    if (ret & KXCJK1013_REG_INT_SRC2_BIT_YN)
    iio_push_event(indio_dev,
    IIO_MOD_EVENT_CODE(IIO_ACCEL,
    0,
    IIO_MOD_Y,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_FALLING),
    data.timestamp);
    if (ret & KXCJK1013_REG_INT_SRC2_BIT_YP)
    iio_push_event(indio_dev,
    IIO_MOD_EVENT_CODE(IIO_ACCEL,
    0,
    IIO_MOD_Y,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_RISING),
    data.timestamp);
    if (ret & KXCJK1013_REG_INT_SRC2_BIT_ZN)
    iio_push_event(indio_dev,
    IIO_MOD_EVENT_CODE(IIO_ACCEL,
    0,
    IIO_MOD_Z,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_FALLING),
    data.timestamp);
    if (ret & KXCJK1013_REG_INT_SRC2_BIT_ZP)
    iio_push_event(indio_dev,
    IIO_MOD_EVENT_CODE(IIO_ACCEL,
    0,
    IIO_MOD_Z,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_RISING),
    data.timestamp);
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_event_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t kxcjk1013_event_handler(int irq, void *private)
    {
    struct iio_dev *indio_dev = private;
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    const struct kx_chipset_regs *regs = data.info.regs;
    int ret;
    ret = i2c_smbus_read_byte_data(data.client, regs.int_src1);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error reading reg_int_src1\n");
    goto ack_intr;
    }
    if (ret & KXCJK1013_REG_INT_SRC1_BIT_WUFS) {
    if (data.info == &kxtf9_info)
    iio_push_event(indio_dev,
    IIO_MOD_EVENT_CODE(IIO_ACCEL,
    0,
    IIO_MOD_X_AND_Y_AND_Z,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_RISING),
    data.timestamp);
    else
    kxcjk1013_report_motion_event(indio_dev);
    }
    ack_intr:
    if (data.dready_trigger_on)
    return IRQ_HANDLED;
    ret = i2c_smbus_read_byte_data(data.client, regs.int_rel);
    if (ret < 0)
    dev_err(&data.client.dev, "Error reading reg_int_rel\n");
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_data_rdy_trig_poll(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t kxcjk1013_data_rdy_trig_poll(int irq, void *private)
    {
    struct iio_dev *indio_dev = private;
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    data.timestamp = iio_get_time_ns(indio_dev);
    if (data.dready_trigger_on)
    iio_trigger_poll(data.dready_trig);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: data->motion_trigger_on) -> else {
    else if (data.motion_trigger_on)
    iio_trigger_poll(data.motion_trig);
    if (data.ev_enable_state)
    return IRQ_WAKE_THREAD;
    else
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_probe(client: *mut i2c_client) -> c_int {
    static int kxcjk1013_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    static const char * const regulator_names[] = { "vdd", "vddio" };
    struct kxcjk1013_data *data;
    struct iio_dev *indio_dev;
    struct kxcjk_1013_platform_data *pdata;
    const void *ddata = core::ptr::null_mut();
    const char *name;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    pdata = dev_get_platdata(&client.dev);
    if (pdata) {
    data.active_high_intr = pdata.active_high_intr;
    data.orientation = pdata.orientation;
    } else {
    data.active_high_intr = true; /* default polarity */
    if (!iio_read_acpi_mount_matrix(&client.dev, &data.orientation, "ROTM")) {
    ret = iio_read_mount_matrix(&client.dev, &data.orientation);
    if (ret)
    return ret;
    }
    }
    ret = devm_regulator_bulk_get_enable(&client.dev,
    ARRAY_SIZE(regulator_names),
    regulator_names);
    if (ret)
    return dev_err_probe(&client.dev, ret, "Failed to get regulators\n");
//
// A typical delay of 10ms is required for powering up
// according to the data sheets of supported chips.
// Hence double that to play safe.
//
    msleep(20);
    if (id) {
    name = id.name;
    data.info = (const struct kx_chipset_info *)(id.driver_data);
    } else {
    name = iio_get_acpi_device_name_and_data(&client.dev, &ddata);
    data.info = ddata;
    if (data.info == &kxcj91008_kiox010a_info)
    indio_dev.label = "accel-display";
#[no_mangle]
pub unsafe extern "C" fn if(&kxcj91008_kiox020a_info: data->info ==) -> else {
    else if (data.info == &kxcj91008_kiox020a_info)
    indio_dev.label = "accel-base";
    }
    if (!name)
    return -ENODEV;
    ret = kxcjk1013_chip_init(data);
    if (ret < 0)
    return ret;
    mutex_init(&data.mutex);
    indio_dev.channels = kxcjk1013_channels;
    indio_dev.num_channels = ARRAY_SIZE(kxcjk1013_channels);
    indio_dev.available_scan_masks = kxcjk1013_scan_masks;
    indio_dev.name = name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &kxcjk1013_iio_info;
    if (client.irq > 0 && data.info.acpi_type != ACPI_SMO8500) {
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    kxcjk1013_data_rdy_trig_poll,
    kxcjk1013_event_handler,
    IRQF_TRIGGER_RISING,
    "kxcjk1013_event",
    indio_dev);
    if (ret)
    goto err_poweroff;
    data.dready_trig = devm_iio_trigger_alloc(&client.dev,
    "%s-dev%d",
    indio_dev.name,
    iio_device_id(indio_dev));
    if (!data.dready_trig) {
    ret = -ENOMEM;
    goto err_poweroff;
    }
    data.motion_trig = devm_iio_trigger_alloc(&client.dev,
    "%s-any-motion-dev%d",
    indio_dev.name,
    iio_device_id(indio_dev));
    if (!data.motion_trig) {
    ret = -ENOMEM;
    goto err_poweroff;
    }
    data.dready_trig.ops = &kxcjk1013_trigger_ops;
    iio_trigger_set_drvdata(data.dready_trig, indio_dev);
    ret = iio_trigger_register(data.dready_trig);
    if (ret)
    goto err_poweroff;
    indio_dev.trig = iio_trigger_get(data.dready_trig);
    data.motion_trig.ops = &kxcjk1013_trigger_ops;
    iio_trigger_set_drvdata(data.motion_trig, indio_dev);
    ret = iio_trigger_register(data.motion_trig);
    if (ret) {
    data.motion_trig = core::ptr::null_mut();
    goto err_trigger_unregister;
    }
    }
    ret = iio_triggered_buffer_setup(indio_dev,
    &iio_pollfunc_store_time,
    kxcjk1013_trigger_handler,
    &kxcjk1013_buffer_setup_ops);
    if (ret < 0) {
    dev_err(&client.dev, "iio triggered buffer setup failed\n");
    goto err_trigger_unregister;
    }
    ret = pm_runtime_set_active(&client.dev);
    if (ret)
    goto err_buffer_cleanup;
    pm_runtime_enable(&client.dev);
    pm_runtime_set_autosuspend_delay(&client.dev,
    KXCJK1013_SLEEP_DELAY_MS);
    pm_runtime_use_autosuspend(&client.dev);
    ret = iio_device_register(indio_dev);
    if (ret < 0) {
    dev_err(&client.dev, "unable to register iio device\n");
    goto err_pm_cleanup;
    }
    return 0;
    err_pm_cleanup:
    pm_runtime_dont_use_autosuspend(&client.dev);
    pm_runtime_disable(&client.dev);
    err_buffer_cleanup:
    iio_triggered_buffer_cleanup(indio_dev);
    err_trigger_unregister:
    if (data.dready_trig)
    iio_trigger_unregister(data.dready_trig);
    if (data.motion_trig)
    iio_trigger_unregister(data.motion_trig);
    err_poweroff:
    kxcjk1013_set_mode(data, STANDBY);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_remove(client: *mut i2c_client) {
    static void kxcjk1013_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    pm_runtime_disable(&client.dev);
    pm_runtime_set_suspended(&client.dev);
    iio_triggered_buffer_cleanup(indio_dev);
    if (data.dready_trig) {
    iio_trigger_unregister(data.dready_trig);
    iio_trigger_unregister(data.motion_trig);
    }
    mutex_lock(&data.mutex);
    kxcjk1013_set_mode(data, STANDBY);
    mutex_unlock(&data.mutex);
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_suspend(dev: *mut device) -> c_int {
    static int kxcjk1013_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(to_i2c_client(dev));
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    int ret;
    mutex_lock(&data.mutex);
    ret = kxcjk1013_set_mode(data, STANDBY);
    mutex_unlock(&data.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_resume(dev: *mut device) -> c_int {
    static int kxcjk1013_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(to_i2c_client(dev));
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    let mut ret: c_int = 0;
    mutex_lock(&data.mutex);
    ret = kxcjk1013_set_mode(data, OPERATION);
    if (ret == 0)
    ret = kxcjk1013_set_range(data, data.range);
    mutex_unlock(&data.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_runtime_suspend(dev: *mut device) -> c_int {
    static int kxcjk1013_runtime_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(to_i2c_client(dev));
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    int ret;
    ret = kxcjk1013_set_mode(data, STANDBY);
    if (ret < 0) {
    dev_err(&data.client.dev, "powering off device failed\n");
    return -EAGAIN;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kxcjk1013_runtime_resume(dev: *mut device) -> c_int {
    static int kxcjk1013_runtime_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(to_i2c_client(dev));
    struct kxcjk1013_data *data = iio_priv(indio_dev);
    int ret;
    int sleep_val;
    ret = kxcjk1013_set_mode(data, OPERATION);
    if (ret < 0)
    return ret;
    sleep_val = kxcjk1013_get_startup_times(data);
    if (sleep_val < 20000)
    usleep_range(sleep_val, 20000);
    else
    msleep_interruptible(sleep_val/1000);
    return 0;
    }
    static const struct dev_pm_ops kxcjk1013_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(kxcjk1013_suspend, kxcjk1013_resume)
    RUNTIME_PM_OPS(kxcjk1013_runtime_suspend, kxcjk1013_runtime_resume, core::ptr::null_mut())
    };
    static const struct i2c_device_id kxcjk1013_id[] = {
    { .name = "kxcjk1013", .driver_data = (kernel_ulong_t)&kxcjk1013_info },
    { .name = "kxcj91008", .driver_data = (kernel_ulong_t)&kxcj91008_info },
    { .name = "kxtj21009", .driver_data = (kernel_ulong_t)&kxtj21009_info },
    { .name = "kxtf9", .driver_data = (kernel_ulong_t)&kxtf9_info },
    { .name = "kx023-1025", .driver_data = (kernel_ulong_t)&kx0231025_info },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, kxcjk1013_id);
    static const struct of_device_id kxcjk1013_of_match[] = {
    { .compatible = "kionix,kxcjk1013", &kxcjk1013_info },
    { .compatible = "kionix,kxcj91008", &kxcj91008_info },
    { .compatible = "kionix,kxtj21009", &kxtj21009_info },
    { .compatible = "kionix,kxtf9", &kxtf9_info },
    { .compatible = "kionix,kx023-1025", &kx0231025_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, kxcjk1013_of_match);
    static const struct acpi_device_id kx_acpi_match[] = {
    { "KIOX0008", (kernel_ulong_t)&kxcj91008_info },
    { "KIOX0009", (kernel_ulong_t)&kxtj21009_info },
    { "KIOX000A", (kernel_ulong_t)&kxcj91008_info },
// KXCJ91008 in the display of a yoga 2-in-1
    { "KIOX010A", (kernel_ulong_t)&kxcj91008_kiox010a_info },
// KXCJ91008 in the base of a yoga 2-in-1
    { "KIOX020A", (kernel_ulong_t)&kxcj91008_kiox020a_info },
    { "KXCJ1008", (kernel_ulong_t)&kxcj91008_info },
    { "KXCJ1013", (kernel_ulong_t)&kxcjk1013_info },
    { "KXCJ9000", (kernel_ulong_t)&kxcj91008_info },
    { "KXJ2109",  (kernel_ulong_t)&kxtj21009_info },
    { "KXTJ1009", (kernel_ulong_t)&kxtj21009_info },
    { "SMO8500",  (kernel_ulong_t)&kxcj91008_smo8500_info },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, kx_acpi_match);
    static struct i2c_driver kxcjk1013_driver = {
    .driver = {
    .name	= "kxcjk1013",
    .acpi_match_table = kx_acpi_match,
    .of_match_table = kxcjk1013_of_match,
    .pm	= pm_ptr(&kxcjk1013_pm_ops),
    },
    .probe		= kxcjk1013_probe,
    .remove		= kxcjk1013_remove,
    .id_table	= kxcjk1013_id,
    };
    module_i2c_driver(kxcjk1013_driver);
    MODULE_AUTHOR("Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("KXCJK1013 accelerometer driver");
