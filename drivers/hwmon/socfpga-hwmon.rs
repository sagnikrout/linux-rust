//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/socfpga-hwmon.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Altera SoC FPGA hardware monitoring driver
//
// Copyright (c) 2026 Altera Corporation
//
// Authors:
// Nazim Amirul <muhammad.nazim.amirul.nazle.asmade@altera.com>
// Tze Yee Ng <tze.yee.ng@altera.com>
//

pub const SOCFPGA_HWMON_MAXSENSORS: c_int = 16;

pub const SOCFPGA_HWMON_PAGE_SHIFT: c_int = 16;

    (((page) << SOCFPGA_HWMON_PAGE_SHIFT) | \
    ((channel) & SOCFPGA_HWMON_CHANNEL_MASK))
pub const SOCFPGA_HWMON_ATTR_VISIBLE: c_int = 0444;
// Temperature from SDM is signed Q8.8 degrees Celsius (8 fractional bits).
pub const SOCFPGA_HWMON_TEMP_FRAC_BITS: c_int = 8;

pub const SOCFPGA_HWMON_TEMP_MDEG_SCALE: c_int = 1000;
// Voltage from SDM is unsigned Q16 volts (16 fractional bits).
pub const SOCFPGA_HWMON_VOLT_FRAC_BITS: c_int = 16;

pub const SOCFPGA_HWMON_VOLT_MV_SCALE: c_int = 1000;
pub const ETEMP_INACTIVE: c_uint = 0x80000000U;
pub const ETEMP_TOO_OLD: c_uint = 0x80000001U;
pub const ETEMP_NOT_PRESENT: c_uint = 0x80000002U;
pub const ETEMP_TIMEOUT: c_uint = 0x80000003U;
pub const ETEMP_CORRUPT: c_uint = 0x80000004U;
pub const ETEMP_BUSY: c_uint = 0x80000005U;
pub const ETEMP_NOT_INITIALIZED: c_uint = 0x800000FFU;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct socfpga_hwmon_channel {
    pub reg: u32,
    pub label: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct socfpga_hwmon_board_data {
    pub temp: *const socfpga_hwmon_channel,
    pub num_temp: c_uint,
    pub volt: *const socfpga_hwmon_channel,
    pub num_volt: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct socfpga_hwmon_priv {
    pub chan: *mut stratix10_svc_chan,
    pub client: stratix10_svc_client,
    pub completion: completion,
    pub /: *mut *mut mutex lock; / protect SVC calls,
    pub async: bool,
    pub /: *mut *mut int last_err; / sync-mode SVC result; 0 on success,
    pub temperature: u32,
    pub voltage: u32,
    pub temperature_channels: c_int,
    pub voltage_channels: c_int,
    pub temp_chan_names: [*const c_char; SOCFPGA_HWMON_MAXSENSORS],
    pub volt_chan_names: [*const c_char; SOCFPGA_HWMON_MAXSENSORS],
    pub temp_chan: [u32; SOCFPGA_HWMON_MAXSENSORS],
    pub volt_chan: [u32; SOCFPGA_HWMON_MAXSENSORS],
}

    static umode_t socfpga_hwmon_is_visible(const void *dev,
    enum hwmon_sensor_types type,
    u32 attr, int chan)
    {
    const struct socfpga_hwmon_priv *priv = dev;
    switch (type) {
    case hwmon_temp:
    if (chan < priv.temperature_channels)
    return SOCFPGA_HWMON_ATTR_VISIBLE;
    return 0;
    case hwmon_in:
    if (chan < priv.voltage_channels)
    return SOCFPGA_HWMON_ATTR_VISIBLE;
    return 0;
    default:
    return 0;
    }
    }
    static void socfpga_hwmon_readtemp_cb(struct stratix10_svc_client *client,
    struct stratix10_svc_cb_data *data)
    {
    struct socfpga_hwmon_priv *priv = client.priv;
    priv.last_err = -EIO;
    if (data.status == BIT(SVC_STATUS_OK)) {
    priv.last_err = 0;
    priv.temperature = (u32)*(unsigned long *)data.kaddr1;
    } else if (data.kaddr1) {
    dev_err(client.dev, "%s failed with status 0x%x, value 0x%lx\n",
    __func__, data.status,
// (unsigned long *)data->kaddr1);
    } else {
    dev_err(client.dev, "%s failed with status 0x%x\n",
    __func__, data.status);
    }
    complete(&priv.completion);
    }
    static void socfpga_hwmon_readvolt_cb(struct stratix10_svc_client *client,
    struct stratix10_svc_cb_data *data)
    {
    struct socfpga_hwmon_priv *priv = client.priv;
    priv.last_err = -EIO;
    if (data.status == BIT(SVC_STATUS_OK)) {
    priv.last_err = 0;
    priv.voltage = (u32)*(unsigned long *)data.kaddr1;
    } else if (data.kaddr1) {
    dev_err(client.dev, "%s failed with status 0x%x, value 0x%lx\n",
    __func__, data.status,
// (unsigned long *)data->kaddr1);
    } else {
    dev_err(client.dev, "%s failed with status 0x%x\n",
    __func__, data.status);
    }
    complete(&priv.completion);
    }
#[no_mangle]
unsafe extern "C" fn socfpga_hwmon_parse_temp(val: *mut c_long, temperature: u32) -> c_int {
    static int socfpga_hwmon_parse_temp(long *val, u32 temperature)
    {
    switch (temperature) {
    case ETEMP_INACTIVE:
    case ETEMP_NOT_PRESENT:
    case ETEMP_CORRUPT:
    case ETEMP_NOT_INITIALIZED:
    return -EOPNOTSUPP;
    case ETEMP_TIMEOUT:
    case ETEMP_BUSY:
    case ETEMP_TOO_OLD:
    return -EAGAIN;
    default:
// SDM returns a 16-bit signed Q8.8 value in the low 16 bits.
// val = (long)(s16)(temperature & SOCFPGA_HWMON_CHANNEL_MASK)
    SOCFPGA_HWMON_TEMP_MDEG_SCALE / SOCFPGA_HWMON_TEMP_FRAC_DIV;
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn socfpga_hwmon_encode_temp_arg(reg: u32, arg: *mut u64) -> c_int {
    static int socfpga_hwmon_encode_temp_arg(u32 reg, u64 *arg)
    {
    let mut page: u32 = (reg >> SOCFPGA_HWMON_PAGE_SHIFT) & SOCFPGA_HWMON_CHANNEL_MASK;
    let mut channel: u32 = reg & SOCFPGA_HWMON_CHANNEL_MASK;
    if (channel >= SOCFPGA_HWMON_MAXSENSORS)
    return -EINVAL;
// arg = (1ULL << channel) | ((u64)page << SOCFPGA_HWMON_PAGE_SHIFT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_hwmon_encode_volt_arg(reg: u32, arg: *mut u64) -> c_int {
    static int socfpga_hwmon_encode_volt_arg(u32 reg, u64 *arg)
    {
    let mut channel: u32 = reg & SOCFPGA_HWMON_CHANNEL_MASK;
    if (channel >= SOCFPGA_HWMON_MAXSENSORS)
    return -EINVAL;
// arg = 1ULL << channel;
    return 0;
    }
    static int socfpga_hwmon_async_read(struct device *dev,
    enum hwmon_sensor_types type,
    struct stratix10_svc_client_msg *msg)
    {
    struct socfpga_hwmon_priv *priv = dev_get_drvdata(dev);
    let mut data: stratix10_svc_cb_data = {};
    let mut deadline: c_ulong = jiffies + HWMON_TIMEOUT;
    void *handle = core::ptr::null_mut();
    int status, index, ret;
    for (index = 0; index < HWMON_ASYNC_MSG_RETRY; index++) {
    status = stratix10_svc_async_send(priv.chan, msg, &handle,
    core::ptr::null_mut(), core::ptr::null_mut());
    if (status == 0)
    break;
    dev_warn(dev, "Failed to send async message: %d\n", status);
    usleep_range(HWMON_RETRY_SLEEP_US, HWMON_RETRY_SLEEP_US * 2);
    }
    if (status && !handle) {
    dev_err(dev, "Failed to send async message after %u retries: %d\n",
    HWMON_ASYNC_MSG_RETRY, status);
    return status;
    }
    ret = -ETIMEDOUT;
    while (!time_after(jiffies, deadline)) {
    status = stratix10_svc_async_poll(priv.chan, handle, &data);
    if (status == -EAGAIN) {
// still in progress
    } else if (status < 0) {
    ret = status;
    break;
    } else if (status == 0) {
    ret = 0;
    break;
    }
    usleep_range(HWMON_RETRY_SLEEP_US, HWMON_RETRY_SLEEP_US * 2);
    }
    if (ret) {
    dev_err(dev, "Failed to get async response\n");
    goto done;
    }
    if (data.status) {
    dev_err(dev, "%s returned 0x%x from SDM\n", __func__,
    data.status);
    ret = -EFAULT;
    goto done;
    }
    if (type == hwmon_temp)
    priv.temperature = (u32)*(unsigned long *)data.kaddr1;
    else
    priv.voltage = (u32)*(unsigned long *)data.kaddr1;
    ret = 0;
    done:
    stratix10_svc_async_done(priv.chan, handle);
    return ret;
    }
    static int socfpga_hwmon_sync_read(struct device *dev,
    enum hwmon_sensor_types type,
    struct stratix10_svc_client_msg *msg)
    {
    struct socfpga_hwmon_priv *priv = dev_get_drvdata(dev);
    int ret;
    reinit_completion(&priv.completion);
    if (type == hwmon_temp)
    priv.client.receive_cb = socfpga_hwmon_readtemp_cb;
    else
    priv.client.receive_cb = socfpga_hwmon_readvolt_cb;
    ret = stratix10_svc_send(priv.chan, msg);
    if (ret < 0)
    goto status_done;
    ret = wait_for_completion_timeout(&priv.completion, HWMON_TIMEOUT);
    if (!ret) {
    dev_err(priv.client.dev, "timeout waiting for SMC call\n");
    ret = -ETIMEDOUT;
    goto status_done;
    }
    ret = priv.last_err;
    status_done:
    stratix10_svc_done(priv.chan);
    return ret;
    }
    static int socfpga_hwmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int chan, long *val)
    {
    struct socfpga_hwmon_priv *priv = dev_get_drvdata(dev);
    let mut msg: stratix10_svc_client_msg = {0};
    int ret;
    if (chan >= SOCFPGA_HWMON_MAXSENSORS)
    return -EOPNOTSUPP;
    switch (type) {
    case hwmon_temp:
    ret = socfpga_hwmon_encode_temp_arg(priv.temp_chan[chan],
    &msg.arg[0]);
    if (ret)
    return ret;
    msg.command = COMMAND_HWMON_READTEMP;
    break;
    case hwmon_in:
    ret = socfpga_hwmon_encode_volt_arg(priv.volt_chan[chan],
    &msg.arg[0]);
    if (ret)
    return ret;
    msg.command = COMMAND_HWMON_READVOLT;
    break;
    default:
    return -EOPNOTSUPP;
    }
    guard(mutex)(&priv.lock);
    if (priv.async)
    ret = socfpga_hwmon_async_read(dev, type, &msg);
    else
    ret = socfpga_hwmon_sync_read(dev, type, &msg);
    if (ret)
    return ret;
    if (type == hwmon_temp)
    ret = socfpga_hwmon_parse_temp(val, priv.temperature);
    else
// SDM returns Q16 volts; convert to hwmon millivolts.
// val = (long)priv->voltage * SOCFPGA_HWMON_VOLT_MV_SCALE
    SOCFPGA_HWMON_VOLT_FRAC_DIV;
    return ret;
    }
    static int socfpga_hwmon_read_string(struct device *dev,
    enum hwmon_sensor_types type, u32 attr,
    int chan, const char **str)
    {
    struct socfpga_hwmon_priv *priv = dev_get_drvdata(dev);
    switch (type) {
    case hwmon_in:
// str = priv->volt_chan_names[chan];
    return 0;
    case hwmon_temp:
// str = priv->temp_chan_names[chan];
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static const struct hwmon_ops socfpga_hwmon_ops = {
    .is_visible = socfpga_hwmon_is_visible,
    .read = socfpga_hwmon_read,
    .read_string = socfpga_hwmon_read_string,
    };
    static const struct hwmon_channel_info *socfpga_hwmon_info[] = {
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL),
    HWMON_CHANNEL_INFO(in,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info socfpga_hwmon_chip_info = {
    .ops = &socfpga_hwmon_ops,
    .info = socfpga_hwmon_info,
    };
    static const struct socfpga_hwmon_channel s10_hwmon_volt_channels[] = {
    { SOCFPGA_HWMON_CHAN(0, 2), "0.8V VCC" },
    { SOCFPGA_HWMON_CHAN(0, 3), "1.8V VCCIO_SDM" },
    { SOCFPGA_HWMON_CHAN(0, 6), "0.9V VCCERAM" },
    };
    static const struct socfpga_hwmon_channel s10_hwmon_temp_channels[] = {
    { SOCFPGA_HWMON_CHAN(0, 0), "Main Die SDM" },
    };
    static const struct socfpga_hwmon_board_data s10_hwmon_board = {
    .temp = s10_hwmon_temp_channels,
    .num_temp = ARRAY_SIZE(s10_hwmon_temp_channels),
    .volt = s10_hwmon_volt_channels,
    .num_volt = ARRAY_SIZE(s10_hwmon_volt_channels),
    };
    static const struct socfpga_hwmon_channel agilex_hwmon_volt_channels[] = {
    { SOCFPGA_HWMON_CHAN(0, 2), "0.8V VCC" },
    { SOCFPGA_HWMON_CHAN(0, 3), "1.8V VCCIO_SDM" },
    { SOCFPGA_HWMON_CHAN(0, 4), "1.8V VCCPT" },
    { SOCFPGA_HWMON_CHAN(0, 5), "1.2V VCCCRCORE" },
    { SOCFPGA_HWMON_CHAN(0, 6), "0.9V VCCH" },
    { SOCFPGA_HWMON_CHAN(0, 7), "0.8V VCCL" },
    };
    static const struct socfpga_hwmon_channel agilex_hwmon_temp_channels[] = {
    { SOCFPGA_HWMON_CHAN(0, 0), "Main Die SDM" },
    { SOCFPGA_HWMON_CHAN(1, 0), "Main Die corner bottom left max" },
    { SOCFPGA_HWMON_CHAN(2, 0), "Main Die corner top left max" },
    { SOCFPGA_HWMON_CHAN(3, 0), "Main Die corner bottom right max" },
    { SOCFPGA_HWMON_CHAN(4, 0), "Main Die corner top right max" },
    };
    static const struct socfpga_hwmon_board_data agilex_hwmon_board = {
    .temp = agilex_hwmon_temp_channels,
    .num_temp = ARRAY_SIZE(agilex_hwmon_temp_channels),
    .volt = agilex_hwmon_volt_channels,
    .num_volt = ARRAY_SIZE(agilex_hwmon_volt_channels),
    };
    static const struct socfpga_hwmon_board_data *
    socfpga_hwmon_get_board(struct device *dev)
    {
    struct device_node *np = dev.of_node;
    if (!np)
    return core::ptr::null_mut();
    if (of_device_is_compatible(np, "intel,stratix10-svc"))
    return &s10_hwmon_board;
    if (of_device_is_compatible(np, "intel,agilex-svc"))
    return &agilex_hwmon_board;
    return core::ptr::null_mut();
    }
    static int socfpga_hwmon_init_channels(struct device *dev,
    const struct socfpga_hwmon_board_data *board,
    struct socfpga_hwmon_priv *priv)
    {
    unsigned int i;
    if (board.num_temp > SOCFPGA_HWMON_MAXSENSORS ||
    board.num_volt > SOCFPGA_HWMON_MAXSENSORS)
    return -EINVAL;
    for (i = 0; i < board.num_temp; i++) {
    priv.temp_chan_names[i] = board.temp[i].label;
    priv.temp_chan[i] = board.temp[i].reg;
    }
    priv.temperature_channels = board.num_temp;
    for (i = 0; i < board.num_volt; i++) {
    priv.volt_chan_names[i] = board.volt[i].label;
    priv.volt_chan[i] = board.volt[i].reg;
    }
    priv.voltage_channels = board.num_volt;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_hwmon_release_svc(data: *mut c_void) {
    static void socfpga_hwmon_release_svc(void *data)
    {
    struct socfpga_hwmon_priv *priv = data;
    if (priv.async)
    stratix10_svc_remove_async_client(priv.chan);
    stratix10_svc_free_channel(priv.chan);
    }
#[no_mangle]
unsafe extern "C" fn socfpga_hwmon_probe(pdev: *mut platform_device) -> c_int {
    static int socfpga_hwmon_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device *parent = dev.parent;
    const struct socfpga_hwmon_board_data *board;
    struct socfpga_hwmon_priv *priv;
    struct device *hwmon_dev;
    int ret;
    if (!parent || !parent.of_node) {
    dev_err(dev, "missing parent device node\n");
    return -ENODEV;
    }
    board = socfpga_hwmon_get_board(parent);
    if (!board) {
    dev_err(dev, "unsupported service layer compatible\n");
    return -ENODEV;
    }
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.client.dev = dev;
    priv.client.priv = priv;
    init_completion(&priv.completion);
    mutex_init(&priv.lock);
    ret = socfpga_hwmon_init_channels(dev, board, priv);
    if (ret)
    return ret;
    priv.chan = stratix10_svc_request_channel_byname(&priv.client,
    SVC_CLIENT_HWMON);
    if (IS_ERR(priv.chan)) {
    ret = PTR_ERR(priv.chan);
    if (ret == -EPROBE_DEFER)
    dev_dbg(dev, "service channel %s not ready, deferring probe\n",
    SVC_CLIENT_HWMON);
    else
    dev_err(dev, "couldn't get service channel %s: %d\n",
    SVC_CLIENT_HWMON, ret);
    return ret;
    }
    ret = stratix10_svc_add_async_client(priv.chan, false);
    switch (ret) {
    case 0:
    priv.async = true;
    break;
    case -EINVAL:
    case -EOPNOTSUPP:
//
// stratix10_svc_add_async_client() returns -EINVAL when the
// async controller is not initialized; fall back to sync mode.
//
    dev_dbg(dev, "async operations not supported, using sync mode\n");
    priv.async = false;
    break;
    default:
    dev_err(dev, "failed to add async client: %d\n", ret);
    stratix10_svc_free_channel(priv.chan);
    return ret;
    }
    ret = devm_add_action_or_reset(dev, socfpga_hwmon_release_svc, priv);
    if (ret)
    return ret;
    hwmon_dev = devm_hwmon_device_register_with_info(dev, "socfpga_hwmon",
    priv,
    &socfpga_hwmon_chip_info,
    core::ptr::null_mut());
    if (IS_ERR(hwmon_dev))
    return PTR_ERR(hwmon_dev);
    platform_set_drvdata(pdev, priv);
    return 0;
    }
    static struct platform_driver socfpga_hwmon_driver = {
    .probe = socfpga_hwmon_probe,
    .driver = {
    .name = "socfpga-hwmon",
    },
    };
    module_platform_driver(socfpga_hwmon_driver);
    MODULE_AUTHOR("Nazim Amirul <muhammad.nazim.amirul.nazle.asmade@altera.com>");
    MODULE_AUTHOR("Tze Yee Ng <tze.yee.ng@altera.com>");
    MODULE_DESCRIPTION("Altera SoC FPGA hardware monitoring driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:socfpga-hwmon");
