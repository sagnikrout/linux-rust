//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/cgbc-core.c
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
// Congatec Board Controller core driver.
//
// The x86 Congatec modules have an embedded micro controller named Board
// Controller. This Board Controller has a Watchdog timer, some GPIOs, and two
// I2C busses.
//
// Copyright (C) 2024 Bootlin
//
// Author: Thomas Richard <thomas.richard@bootlin.com>
//

pub const CGBC_IO_SESSION_BASE: c_uint = 0x0E20;
pub const CGBC_IO_SESSION_END: c_uint = 0x0E30;
pub const CGBC_IO_CMD_BASE: c_uint = 0x0E00;
pub const CGBC_IO_CMD_END: c_uint = 0x0E10;

pub const CGBC_MASK_DATA_COUNT: c_uint = 0x1F;
pub const CGBC_MASK_ERROR_CODE: c_uint = 0x1F;
pub const CGBC_STATUS_DATA_READY: c_uint = 0x00;

pub const CGBC_SESSION_CMD: c_uint = 0x00;
pub const CGBC_SESSION_CMD_IDLE: c_uint = 0x00;
pub const CGBC_SESSION_CMD_REQUEST: c_uint = 0x01;
pub const CGBC_SESSION_DATA: c_uint = 0x01;
pub const CGBC_SESSION_STATUS: c_uint = 0x02;
pub const CGBC_SESSION_STATUS_FREE: c_uint = 0x03;
pub const CGBC_SESSION_ACCESS: c_uint = 0x04;
pub const CGBC_SESSION_ACCESS_GAINED: c_uint = 0x00;
pub const CGBC_SESSION_VALID_MIN: c_uint = 0x02;
pub const CGBC_SESSION_VALID_MAX: c_uint = 0xFE;
pub const CGBC_CMD_STROBE: c_uint = 0x00;
pub const CGBC_CMD_INDEX: c_uint = 0x02;
pub const CGBC_CMD_INDEX_CBM_MAN8: c_uint = 0x00;
pub const CGBC_CMD_INDEX_CBM_AUTO32: c_uint = 0x03;
pub const CGBC_CMD_DATA: c_uint = 0x04;
pub const CGBC_CMD_ACCESS: c_uint = 0x0C;
pub const CGBC_CMD_GET_FW_REV: c_uint = 0x21;
    static struct platform_device *cgbc_pdev;
// Wait the Board Controller is ready to receive some session commands
#[no_mangle]
unsafe extern "C" fn cgbc_wait_device(cgbc: *mut cgbc_device_data) -> c_int {
    static int cgbc_wait_device(struct cgbc_device_data *cgbc)
    {
    u16 status;
    int ret;
    ret = readx_poll_timeout(ioread16, cgbc.io_session + CGBC_SESSION_STATUS, status,
    status == CGBC_SESSION_STATUS_FREE, 0, 500000);
    if (ret || ioread32(cgbc.io_session + CGBC_SESSION_ACCESS))
    ret = -ENODEV;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cgbc_session_command(cgbc: *mut cgbc_device_data, cmd: u8) -> c_int {
    static int cgbc_session_command(struct cgbc_device_data *cgbc, u8 cmd)
    {
    int ret;
    u8 val;
    ret = readx_poll_timeout(ioread8, cgbc.io_session + CGBC_SESSION_CMD, val,
    val == CGBC_SESSION_CMD_IDLE, 0, 100000);
    if (ret)
    return ret;
    iowrite8(cmd, cgbc.io_session + CGBC_SESSION_CMD);
    ret = readx_poll_timeout(ioread8, cgbc.io_session + CGBC_SESSION_CMD, val,
    val == CGBC_SESSION_CMD_IDLE, 0, 100000);
    if (ret)
    return ret;
    ret = (int)ioread8(cgbc.io_session + CGBC_SESSION_DATA);
    iowrite8(CGBC_SESSION_STATUS_FREE, cgbc.io_session + CGBC_SESSION_STATUS);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cgbc_session_request(cgbc: *mut cgbc_device_data) -> c_int {
    static int cgbc_session_request(struct cgbc_device_data *cgbc)
    {
    int ret;
    ret = cgbc_wait_device(cgbc);
    if (ret)
    return dev_err_probe(cgbc.dev, ret, "device not found or not ready\n");
    cgbc.session = cgbc_session_command(cgbc, CGBC_SESSION_CMD_REQUEST);
// The Board Controller sent us a wrong session handle, we cannot communicate with it
    if (cgbc.session < CGBC_SESSION_VALID_MIN || cgbc.session > CGBC_SESSION_VALID_MAX)
    return dev_err_probe(cgbc.dev, -ECONNREFUSED,
    "failed to get a valid session handle\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgbc_session_release(cgbc: *mut cgbc_device_data) {
    static void cgbc_session_release(struct cgbc_device_data *cgbc)
    {
    if (cgbc_session_command(cgbc, cgbc.session) != cgbc.session)
    dev_warn(cgbc.dev, "failed to release session\n");
    }
#[no_mangle]
unsafe extern "C" fn cgbc_command_lock(cgbc: *mut cgbc_device_data) -> bool {
    static bool cgbc_command_lock(struct cgbc_device_data *cgbc)
    {
    iowrite8(cgbc.session, cgbc.io_cmd + CGBC_CMD_ACCESS);
    return ioread8(cgbc.io_cmd + CGBC_CMD_ACCESS) == cgbc.session;
    }
#[no_mangle]
unsafe extern "C" fn cgbc_command_unlock(cgbc: *mut cgbc_device_data) {
    static void cgbc_command_unlock(struct cgbc_device_data *cgbc)
    {
    iowrite8(cgbc.session, cgbc.io_cmd + CGBC_CMD_ACCESS);
    }
    int cgbc_command(struct cgbc_device_data *cgbc, void *cmd, unsigned int cmd_size, void *data,
    unsigned int data_size, u8 *status)
    {
    let mut checksum: u8 = 0, data_checksum = 0, istatus = 0, val;
    u8 *_data = (u8 *)data;
    u8 *_cmd = (u8 *)cmd;
    let mut mode_change: c_int = -1;
    bool lock;
    int ret, i;
    mutex_lock(&cgbc.lock);
// Request access
    ret = readx_poll_timeout(cgbc_command_lock, cgbc, lock, lock, 0, 100000);
    if (ret)
    goto out;
// Wait board controller is ready
    ret = readx_poll_timeout(ioread8, cgbc.io_cmd + CGBC_CMD_STROBE, val,
    val == CGBC_CMD_STROBE, 0, 100000);
    if (ret)
    goto release;
// Write command packet
    if (cmd_size <= 2) {
    iowrite8(CGBC_CMD_INDEX_CBM_MAN8, cgbc.io_cmd + CGBC_CMD_INDEX);
    } else {
    iowrite8(CGBC_CMD_INDEX_CBM_AUTO32, cgbc.io_cmd + CGBC_CMD_INDEX);
    if ((cmd_size % 4) != 0x03)
    mode_change = (cmd_size & 0xFFFC) - 1;
    }
    for (i = 0; i < cmd_size; i++) {
    iowrite8(_cmd[i], cgbc.io_cmd + CGBC_CMD_DATA + (i % 4));
    checksum ^= _cmd[i];
    if (mode_change == i)
    iowrite8((i + 1) | CGBC_CMD_INDEX_CBM_MAN8, cgbc.io_cmd + CGBC_CMD_INDEX);
    }
// Append checksum byte
    iowrite8(checksum, cgbc.io_cmd + CGBC_CMD_DATA + (i % 4));
// Perform command strobe
    iowrite8(cgbc.session, cgbc.io_cmd + CGBC_CMD_STROBE);
// Rewind cmd buffer index
    iowrite8(CGBC_CMD_INDEX_CBM_AUTO32, cgbc.io_cmd + CGBC_CMD_INDEX);
// Wait command completion
    ret = read_poll_timeout(ioread8, val, val == CGBC_CMD_STROBE, 0, 100000, false,
    cgbc.io_cmd + CGBC_CMD_STROBE);
    if (ret)
    goto release;
    istatus = ioread8(cgbc.io_cmd + CGBC_CMD_DATA);
    checksum = istatus;
// Check command status
    switch (istatus & CGBC_MASK_STATUS) {
    case CGBC_STATUS_DATA_READY:
    if (istatus > data_size)
    istatus = data_size;
    for (i = 0; i < istatus; i++) {
    _data[i] = ioread8(cgbc.io_cmd + CGBC_CMD_DATA + ((i + 1) % 4));
    checksum ^= _data[i];
    }
    data_checksum = ioread8(cgbc.io_cmd + CGBC_CMD_DATA + ((i + 1) % 4));
    istatus &= CGBC_MASK_DATA_COUNT;
    break;
    case CGBC_STATUS_ERROR:
    case CGBC_STATUS_CMD_READY:
    data_checksum = ioread8(cgbc.io_cmd + CGBC_CMD_DATA + 1);
    if ((istatus & CGBC_MASK_STATUS) == CGBC_STATUS_ERROR)
    ret = -EIO;
    istatus = istatus & CGBC_MASK_ERROR_CODE;
    break;
    default:
    data_checksum = ioread8(cgbc.io_cmd + CGBC_CMD_DATA + 1);
    istatus &= CGBC_MASK_ERROR_CODE;
    ret = -EIO;
    break;
    }
// Checksum verification
    if (ret == 0 && data_checksum != checksum)
    ret = -EIO;
    release:
    cgbc_command_unlock(cgbc);
    out:
    mutex_unlock(&cgbc.lock);
    if (status)
// status = istatus;
    return ret;
    }
    EXPORT_SYMBOL_GPL(cgbc_command);
    static struct mfd_cell cgbc_devs[] = {
    { .name = "cgbc-wdt"	},
    { .name = "cgbc-gpio"	},
    { .name = "cgbc-i2c", .id = 1 },
    { .name = "cgbc-i2c", .id = 2 },
    { .name = "cgbc-hwmon"	},
    { .name = "cgbc-backlight" },
    };
#[no_mangle]
unsafe extern "C" fn cgbc_map(cgbc: *mut cgbc_device_data) -> c_int {
    static int cgbc_map(struct cgbc_device_data *cgbc)
    {
    struct device *dev = cgbc.dev;
    struct platform_device *pdev = to_platform_device(dev);
    struct resource *ioport;
    ioport = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if (!ioport)
    return -EINVAL;
    cgbc.io_session = devm_ioport_map(dev, ioport.start, resource_size(ioport));
    if (!cgbc.io_session)
    return -ENOMEM;
    ioport = platform_get_resource(pdev, IORESOURCE_IO, 1);
    if (!ioport)
    return -EINVAL;
    cgbc.io_cmd = devm_ioport_map(dev, ioport.start, resource_size(ioport));
    if (!cgbc.io_cmd)
    return -ENOMEM;
    return 0;
    }
    static const struct resource cgbc_resources[] = {
    {
    .start  = CGBC_IO_SESSION_BASE,
    .end    = CGBC_IO_SESSION_END,
    .flags  = IORESOURCE_IO,
    },
    {
    .start  = CGBC_IO_CMD_BASE,
    .end    = CGBC_IO_CMD_END,
    .flags  = IORESOURCE_IO,
    },
    };
    static ssize_t cgbc_version_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct cgbc_device_data *cgbc = dev_get_drvdata(dev);
    return sysfs_emit(buf, "CGBCP%c%c%c\n", cgbc.version.feature, cgbc.version.major,
    cgbc.version.minor);
    }
    static DEVICE_ATTR_RO(cgbc_version);
    static struct attribute *cgbc_attrs[] = {
    &dev_attr_cgbc_version.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(cgbc);
#[no_mangle]
unsafe extern "C" fn cgbc_get_version(cgbc: *mut cgbc_device_data) -> c_int {
    static int cgbc_get_version(struct cgbc_device_data *cgbc)
    {
    let mut cmd: u8 = CGBC_CMD_GET_FW_REV;
    u8 data[4];
    int ret;
    ret = cgbc_command(cgbc, &cmd, 1, &data, sizeof(data), core::ptr::null_mut());
    if (ret)
    return ret;
    cgbc.version.feature = data[0];
    cgbc.version.major = data[1];
    cgbc.version.minor = data[2];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgbc_init_device(cgbc: *mut cgbc_device_data) -> c_int {
    static int cgbc_init_device(struct cgbc_device_data *cgbc)
    {
    int ret;
    ret = cgbc_session_request(cgbc);
    if (ret)
    return ret;
    ret = cgbc_get_version(cgbc);
    if (ret)
    goto release_session;
    ret = mfd_add_devices(cgbc.dev, -1, cgbc_devs, ARRAY_SIZE(cgbc_devs),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret)
    goto release_session;
    return 0;
    release_session:
    cgbc_session_release(cgbc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cgbc_probe(pdev: *mut platform_device) -> c_int {
    static int cgbc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct cgbc_device_data *cgbc;
    int ret;
    cgbc = devm_kzalloc(dev, sizeof(*cgbc), GFP_KERNEL);
    if (!cgbc)
    return -ENOMEM;
    cgbc.dev = dev;
    ret = cgbc_map(cgbc);
    if (ret)
    return ret;
    mutex_init(&cgbc.lock);
    platform_set_drvdata(pdev, cgbc);
    return cgbc_init_device(cgbc);
    }
#[no_mangle]
unsafe extern "C" fn cgbc_remove(pdev: *mut platform_device) {
    static void cgbc_remove(struct platform_device *pdev)
    {
    struct cgbc_device_data *cgbc = platform_get_drvdata(pdev);
    mfd_remove_devices(&pdev.dev);
    cgbc_session_release(cgbc);
    }
    static struct platform_driver cgbc_driver = {
    .driver		= {
    .name		= "cgbc",
    .dev_groups	= cgbc_groups,
    },
    .probe		= cgbc_probe,
    .remove		= cgbc_remove,
    };
    static const struct dmi_system_id cgbc_dmi_table[] __initconst = {
    {
    .ident = "SA7",
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "congatec"),
    DMI_MATCH(DMI_BOARD_NAME, "conga-SA7"),
    },
    },
    {
    .ident = "SA8",
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "congatec"),
    DMI_MATCH(DMI_BOARD_NAME, "conga-SA8"),
    },
    },
    {}
    };
    MODULE_DEVICE_TABLE(dmi, cgbc_dmi_table);
#[no_mangle]
unsafe extern "C" fn cgbc_init() -> int __init {
    static int __init cgbc_init(void)
    {
    const struct dmi_system_id *id;
    let mut ret: c_int = -ENODEV;
    id = dmi_first_match(cgbc_dmi_table);
    if (IS_ERR_OR_NULL(id))
    return ret;
    cgbc_pdev = platform_device_register_simple("cgbc", PLATFORM_DEVID_NONE, cgbc_resources,
    ARRAY_SIZE(cgbc_resources));
    if (IS_ERR(cgbc_pdev))
    return PTR_ERR(cgbc_pdev);
    return platform_driver_register(&cgbc_driver);
    }
#[no_mangle]
unsafe extern "C" fn cgbc_exit() -> void __exit {
    static void __exit cgbc_exit(void)
    {
    platform_device_unregister(cgbc_pdev);
    platform_driver_unregister(&cgbc_driver);
    }
    module_init(cgbc_init);
    module_exit(cgbc_exit);
    MODULE_DESCRIPTION("Congatec Board Controller Core Driver");
    MODULE_AUTHOR("Thomas Richard <thomas.richard@bootlin.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:cgbc-core");
