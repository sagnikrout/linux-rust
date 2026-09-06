//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/max20730.c
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
// Driver for MAX20710, MAX20730, MAX20734, and MAX20743 Integrated,
// Step-Down Switching Regulators
//
// Copyright 2019 Google LLC.
// Copyright 2020 Maxim Integrated
//

    enum chips {
    max20710,
    max20730,
    max20734,
    max20743
    };
    enum {
    MAX20730_DEBUGFS_VOUT_MIN = 0,
    MAX20730_DEBUGFS_FREQUENCY,
    MAX20730_DEBUGFS_PG_DELAY,
    MAX20730_DEBUGFS_INTERNAL_GAIN,
    MAX20730_DEBUGFS_BOOT_VOLTAGE,
    MAX20730_DEBUGFS_OUT_V_RAMP_RATE,
    MAX20730_DEBUGFS_OC_PROTECT_MODE,
    MAX20730_DEBUGFS_SS_TIMING,
    MAX20730_DEBUGFS_IMAX,
    MAX20730_DEBUGFS_OPERATION,
    MAX20730_DEBUGFS_ON_OFF_CONFIG,
    MAX20730_DEBUGFS_SMBALERT_MASK,
    MAX20730_DEBUGFS_VOUT_MODE,
    MAX20730_DEBUGFS_VOUT_COMMAND,
    MAX20730_DEBUGFS_VOUT_MAX,
    MAX20730_DEBUGFS_NUM_ENTRIES
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max20730_data {
    pub id: enum chips,
    pub info: pmbus_driver_info,
    pub /: *mut *mut mutex lock; / Used to protect against parallel writes,
    pub mfr_devset1: u16,
    pub mfr_devset2: u16,
    pub mfr_voutmin: u16,
    pub vout_voltage_divider: [u32; 2],
}

pub const PMBUS_SMB_ALERT_MASK: c_uint = 0x1B;
pub const MAX20730_MFR_VOUT_MIN: c_uint = 0xd1;
pub const MAX20730_MFR_DEVSET1: c_uint = 0xd2;
pub const MAX20730_MFR_DEVSET2: c_uint = 0xd3;

pub const MAX20730_MFR_VOUT_MIN_BIT_POS: c_int = 0;

pub const MAX20730_MFR_DEVSET1_RGAIN_BIT_POS: c_int = 13;
pub const MAX20730_MFR_DEVSET1_OTP_BIT_POS: c_int = 11;
pub const MAX20730_MFR_DEVSET1_VBOOT_BIT_POS: c_int = 8;
pub const MAX20730_MFR_DEVSET1_OCP_BIT_POS: c_int = 5;
pub const MAX20730_MFR_DEVSET1_FSW_BIT_POS: c_int = 2;
pub const MAX20730_MFR_DEVSET1_TSTAT_BIT_POS: c_int = 0;

pub const MAX20730_MFR_DEVSET2_IMAX_BIT_POS: c_int = 8;
pub const MAX20730_MFR_DEVSET2_VRATE_BIT_POS: c_int = 6;
pub const MAX20730_MFR_DEVSET2_OCPM_BIT_POS: c_int = 5;
pub const MAX20730_MFR_DEVSET2_SS_BIT_POS: c_int = 0;
pub const DEBUG_FS_DATA_MAX: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max20730_debugfs_data {
    pub client: *mut i2c_client,
    pub debugfs_entries: [c_int; MAX20730_DEBUGFS_NUM_ENTRIES],
}

    struct max20730_debugfs_data, debugfs_entries[(y)])

    static ssize_t max20730_debugfs_read(struct file *file, char __user *buf,
    size_t count, loff_t *ppos)
    {
    int ret, len;
    int *idxp = file.private_data;
    let mut idx: c_int = *idxp;
    struct max20730_debugfs_data *psu = to_psu(idxp, idx);
    const struct pmbus_driver_info *info;
    const struct max20730_data *data;
    char tbuf[DEBUG_FS_DATA_MAX] = { 0 };
    char *result = tbuf;
    u16 val;
    info = pmbus_get_driver_info(psu.client);
    data = to_max20730_data(info);
    switch (idx) {
    case MAX20730_DEBUGFS_VOUT_MIN:
    ret = VOLT_FROM_REG(data.mfr_voutmin * 10000);
    len = scnprintf(tbuf, DEBUG_FS_DATA_MAX, "%d.%d\n",
    ret / 10000, ret % 10000);
    break;
    case MAX20730_DEBUGFS_FREQUENCY:
    val = (data.mfr_devset1 & MAX20730_MFR_DEVSET1_FSW_MASK)
    >> MAX20730_MFR_DEVSET1_FSW_BIT_POS;
    if (val == 0)
    ret = 400;
#[no_mangle]
pub unsafe extern "C" fn if(1: val ==) -> else {
    else if (val == 1)
    ret = 500;
#[no_mangle]
pub unsafe extern "C" fn if(3: val == 2 || val ==) -> else {
    else if (val == 2 || val == 3)
    ret = 600;
#[no_mangle]
pub unsafe extern "C" fn if(4: val ==) -> else {
    else if (val == 4)
    ret = 700;
#[no_mangle]
pub unsafe extern "C" fn if(5: val ==) -> else {
    else if (val == 5)
    ret = 800;
    else
    ret = 900;
    len = scnprintf(tbuf, DEBUG_FS_DATA_MAX, "%d\n", ret);
    break;
    case MAX20730_DEBUGFS_PG_DELAY:
    val = (data.mfr_devset1 & MAX20730_MFR_DEVSET1_TSTAT_MASK)
    >> MAX20730_MFR_DEVSET1_TSTAT_BIT_POS;
    if (val == 0)
    result = "2000\n";
#[no_mangle]
pub unsafe extern "C" fn if(1: val ==) -> else {
    else if (val == 1)
    result = "125\n";
#[no_mangle]
pub unsafe extern "C" fn if(2: val ==) -> else {
    else if (val == 2)
    result = "62.5\n";
    else
    result = "32\n";
    break;
    case MAX20730_DEBUGFS_INTERNAL_GAIN:
    val = (data.mfr_devset1 & MAX20730_MFR_DEVSET1_RGAIN_MASK)
    >> MAX20730_MFR_DEVSET1_RGAIN_BIT_POS;
    if (data.id == max20734) {
// AN6209
    if (val == 0)
    result = "0.8\n";
#[no_mangle]
pub unsafe extern "C" fn if(1: val ==) -> else {
    else if (val == 1)
    result = "3.2\n";
#[no_mangle]
pub unsafe extern "C" fn if(2: val ==) -> else {
    else if (val == 2)
    result = "1.6\n";
    else
    result = "6.4\n";
    } else if (data.id == max20730 || data.id == max20710) {
// AN6042 or AN6140
    if (val == 0)
    result = "0.9\n";
#[no_mangle]
pub unsafe extern "C" fn if(1: val ==) -> else {
    else if (val == 1)
    result = "3.6\n";
#[no_mangle]
pub unsafe extern "C" fn if(2: val ==) -> else {
    else if (val == 2)
    result = "1.8\n";
    else
    result = "7.2\n";
    } else if (data.id == max20743) {
// AN6042
    if (val == 0)
    result = "0.45\n";
#[no_mangle]
pub unsafe extern "C" fn if(1: val ==) -> else {
    else if (val == 1)
    result = "1.8\n";
#[no_mangle]
pub unsafe extern "C" fn if(2: val ==) -> else {
    else if (val == 2)
    result = "0.9\n";
    else
    result = "3.6\n";
    } else {
    result = "Not supported\n";
    }
    break;
    case MAX20730_DEBUGFS_BOOT_VOLTAGE:
    val = (data.mfr_devset1 & MAX20730_MFR_DEVSET1_VBOOT_MASK)
    >> MAX20730_MFR_DEVSET1_VBOOT_BIT_POS;
    if (val == 0)
    result = "0.6484\n";
#[no_mangle]
pub unsafe extern "C" fn if(1: val ==) -> else {
    else if (val == 1)
    result = "0.8984\n";
#[no_mangle]
pub unsafe extern "C" fn if(2: val ==) -> else {
    else if (val == 2)
    result = "1.0\n";
    else
    result = "Invalid\n";
    break;
    case MAX20730_DEBUGFS_OUT_V_RAMP_RATE:
    val = (data.mfr_devset2 & MAX20730_MFR_DEVSET2_VRATE)
    >> MAX20730_MFR_DEVSET2_VRATE_BIT_POS;
    if (val == 0)
    result = "4\n";
#[no_mangle]
pub unsafe extern "C" fn if(1: val ==) -> else {
    else if (val == 1)
    result = "2\n";
#[no_mangle]
pub unsafe extern "C" fn if(2: val ==) -> else {
    else if (val == 2)
    result = "1\n";
    else
    result = "Invalid\n";
    break;
    case MAX20730_DEBUGFS_OC_PROTECT_MODE:
    ret = (data.mfr_devset2 & MAX20730_MFR_DEVSET2_OCPM_MASK)
    >> MAX20730_MFR_DEVSET2_OCPM_BIT_POS;
    len = scnprintf(tbuf, DEBUG_FS_DATA_MAX, "%d\n", ret);
    break;
    case MAX20730_DEBUGFS_SS_TIMING:
    val = (data.mfr_devset2 & MAX20730_MFR_DEVSET2_SS_MASK)
    >> MAX20730_MFR_DEVSET2_SS_BIT_POS;
    if (val == 0)
    result = "0.75\n";
#[no_mangle]
pub unsafe extern "C" fn if(1: val ==) -> else {
    else if (val == 1)
    result = "1.5\n";
#[no_mangle]
pub unsafe extern "C" fn if(2: val ==) -> else {
    else if (val == 2)
    result = "3\n";
    else
    result = "6\n";
    break;
    case MAX20730_DEBUGFS_IMAX:
    ret = (data.mfr_devset2 & MAX20730_MFR_DEVSET2_IMAX_MASK)
    >> MAX20730_MFR_DEVSET2_IMAX_BIT_POS;
    len = scnprintf(tbuf, DEBUG_FS_DATA_MAX, "%d\n", ret);
    break;
    case MAX20730_DEBUGFS_OPERATION:
    ret = i2c_smbus_read_byte_data(psu.client, PMBUS_OPERATION);
    if (ret < 0)
    return ret;
    len = scnprintf(tbuf, DEBUG_FS_DATA_MAX, "%d\n", ret);
    break;
    case MAX20730_DEBUGFS_ON_OFF_CONFIG:
    ret = i2c_smbus_read_byte_data(psu.client, PMBUS_ON_OFF_CONFIG);
    if (ret < 0)
    return ret;
    len = scnprintf(tbuf, DEBUG_FS_DATA_MAX, "%d\n", ret);
    break;
    case MAX20730_DEBUGFS_SMBALERT_MASK:
    ret = i2c_smbus_read_word_data(psu.client,
    PMBUS_SMB_ALERT_MASK);
    if (ret < 0)
    return ret;
    len = scnprintf(tbuf, DEBUG_FS_DATA_MAX, "%d\n", ret);
    break;
    case MAX20730_DEBUGFS_VOUT_MODE:
    ret = i2c_smbus_read_byte_data(psu.client, PMBUS_VOUT_MODE);
    if (ret < 0)
    return ret;
    len = scnprintf(tbuf, DEBUG_FS_DATA_MAX, "%d\n", ret);
    break;
    case MAX20730_DEBUGFS_VOUT_COMMAND:
    ret = i2c_smbus_read_word_data(psu.client, PMBUS_VOUT_COMMAND);
    if (ret < 0)
    return ret;
    ret = VOLT_FROM_REG(ret * 10000);
    len = scnprintf(tbuf, DEBUG_FS_DATA_MAX,
    "%d.%d\n", ret / 10000, ret % 10000);
    break;
    case MAX20730_DEBUGFS_VOUT_MAX:
    ret = i2c_smbus_read_word_data(psu.client, PMBUS_VOUT_MAX);
    if (ret < 0)
    return ret;
    ret = VOLT_FROM_REG(ret * 10000);
    len = scnprintf(tbuf, DEBUG_FS_DATA_MAX,
    "%d.%d\n", ret / 10000, ret % 10000);
    break;
    default:
    result = "Invalid\n";
    }
    len = strlen(result);
    return simple_read_from_buffer(buf, count, ppos, result, len);
    }
    static const struct file_operations max20730_fops = {
    .llseek = noop_llseek,
    .read = max20730_debugfs_read,
    .write = core::ptr::null_mut(),
    .open = simple_open,
    };
    static int max20730_init_debugfs(struct i2c_client *client,
    struct max20730_data *data)
    {
    int ret, i;
    struct dentry *debugfs;
    struct dentry *max20730_dir;
    struct max20730_debugfs_data *psu;
    ret = i2c_smbus_read_word_data(client, MAX20730_MFR_DEVSET2);
    if (ret < 0)
    return ret;
    data.mfr_devset2 = ret;
    ret = i2c_smbus_read_word_data(client, MAX20730_MFR_VOUT_MIN);
    if (ret < 0)
    return ret;
    data.mfr_voutmin = ret;
    psu = devm_kzalloc(&client.dev, sizeof(*psu), GFP_KERNEL);
    if (!psu)
    return -ENOMEM;
    psu.client = client;
    debugfs = pmbus_get_debugfs_dir(client);
    if (!debugfs)
    return -ENOENT;
    max20730_dir = debugfs_create_dir(client.name, debugfs);
    for (i = 0; i < MAX20730_DEBUGFS_NUM_ENTRIES; ++i)
    psu.debugfs_entries[i] = i;
    debugfs_create_file("vout_min", 0444, max20730_dir,
    &psu.debugfs_entries[MAX20730_DEBUGFS_VOUT_MIN],
    &max20730_fops);
    debugfs_create_file("frequency", 0444, max20730_dir,
    &psu.debugfs_entries[MAX20730_DEBUGFS_FREQUENCY],
    &max20730_fops);
    debugfs_create_file("power_good_delay", 0444, max20730_dir,
    &psu.debugfs_entries[MAX20730_DEBUGFS_PG_DELAY],
    &max20730_fops);
    debugfs_create_file("internal_gain", 0444, max20730_dir,
    &psu.debugfs_entries[MAX20730_DEBUGFS_INTERNAL_GAIN],
    &max20730_fops);
    debugfs_create_file("boot_voltage", 0444, max20730_dir,
    &psu.debugfs_entries[MAX20730_DEBUGFS_BOOT_VOLTAGE],
    &max20730_fops);
    debugfs_create_file("out_voltage_ramp_rate", 0444, max20730_dir,
    &psu.debugfs_entries[MAX20730_DEBUGFS_OUT_V_RAMP_RATE],
    &max20730_fops);
    debugfs_create_file("oc_protection_mode", 0444, max20730_dir,
    &psu.debugfs_entries[MAX20730_DEBUGFS_OC_PROTECT_MODE],
    &max20730_fops);
    debugfs_create_file("soft_start_timing", 0444, max20730_dir,
    &psu.debugfs_entries[MAX20730_DEBUGFS_SS_TIMING],
    &max20730_fops);
    debugfs_create_file("imax", 0444, max20730_dir,
    &psu.debugfs_entries[MAX20730_DEBUGFS_IMAX],
    &max20730_fops);
    debugfs_create_file("operation", 0444, max20730_dir,
    &psu.debugfs_entries[MAX20730_DEBUGFS_OPERATION],
    &max20730_fops);
    debugfs_create_file("on_off_config", 0444, max20730_dir,
    &psu.debugfs_entries[MAX20730_DEBUGFS_ON_OFF_CONFIG],
    &max20730_fops);
    debugfs_create_file("smbalert_mask", 0444, max20730_dir,
    &psu.debugfs_entries[MAX20730_DEBUGFS_SMBALERT_MASK],
    &max20730_fops);
    debugfs_create_file("vout_mode", 0444, max20730_dir,
    &psu.debugfs_entries[MAX20730_DEBUGFS_VOUT_MODE],
    &max20730_fops);
    debugfs_create_file("vout_command", 0444, max20730_dir,
    &psu.debugfs_entries[MAX20730_DEBUGFS_VOUT_COMMAND],
    &max20730_fops);
    debugfs_create_file("vout_max", 0444, max20730_dir,
    &psu.debugfs_entries[MAX20730_DEBUGFS_VOUT_MAX],
    &max20730_fops);
    return 0;
    }

    static int max20730_init_debugfs(struct i2c_client *client,
    struct max20730_data *data)
    {
    return 0;
    }

    static const struct i2c_device_id max20730_id[];
//
// Convert discreet value to direct data format. Strictly speaking, all passed
// values are constants, so we could do that calculation manually. On the
// downside, that would make the driver more difficult to maintain, so lets
// use this approach.
//
    static u16 val_to_direct(int v, enum pmbus_sensor_classes class,
    const struct pmbus_driver_info *info)
    {
    int R = info.R[class] - 3;	/* take milli-units into account */
    let mut b: c_int = info.b[class] * 1000;
    long d;
    d = v * info.m[class] + b;
//
// R < 0 is true for all callers, so we don't need to bother
// about the R > 0 case.
//
    while (R < 0) {
    d = DIV_ROUND_CLOSEST(d, 10);
    R++;
    }
    return (u16)d;
    }
    static long direct_to_val(u16 w, enum pmbus_sensor_classes class,
    const struct pmbus_driver_info *info)
    {
    let mut R: c_int = info.R[class] - 3;
    let mut b: c_int = info.b[class] * 1000;
    let mut m: c_int = info.m[class];
    let mut d: c_long = (s16)w;
    if (m == 0)
    return 0;
    while (R < 0) {
    d *= 10;
    R++;
    }
    d = (d - b) / m;
    return d;
    }
    static u32 max_current[][5] = {
    [max20710] = { 6200, 8000, 9700, 11600 },
    [max20730] = { 13000, 16600, 20100, 23600 },
    [max20734] = { 21000, 27000, 32000, 38000 },
    [max20743] = { 18900, 24100, 29200, 34100 },
    };
    static int max20730_read_word_data(struct i2c_client *client, int page,
    int phase, int reg)
    {
    const struct pmbus_driver_info *info = pmbus_get_driver_info(client);
    const struct max20730_data *data = to_max20730_data(info);
    let mut ret: c_int = 0;
    u32 max_c;
    switch (reg) {
    case PMBUS_OT_FAULT_LIMIT:
    switch ((data.mfr_devset1 >> 11) & 0x3) {
    case 0x0:
    ret = val_to_direct(150000, PSC_TEMPERATURE, info);
    break;
    case 0x1:
    ret = val_to_direct(130000, PSC_TEMPERATURE, info);
    break;
    default:
    ret = -ENODATA;
    break;
    }
    break;
    case PMBUS_IOUT_OC_FAULT_LIMIT:
    max_c = max_current[data.id][(data.mfr_devset1 >> 5) & 0x3];
    ret = val_to_direct(max_c, PSC_CURRENT_OUT, info);
    break;
    case PMBUS_READ_VOUT:
    ret = pmbus_read_word_data(client, page, phase, reg);
    if (ret > 0 && data.vout_voltage_divider[0] && data.vout_voltage_divider[1]) {
    u64 temp = DIV_ROUND_CLOSEST_ULL((u64)ret * data.vout_voltage_divider[1],
    data.vout_voltage_divider[0]);
    ret = clamp_val(temp, 0, 0xffff);
    }
    break;
    default:
    ret = -ENODATA;
    break;
    }
    return ret;
    }
    static int max20730_write_word_data(struct i2c_client *client, int page,
    int reg, u16 word)
    {
    struct pmbus_driver_info *info;
    struct max20730_data *data;
    u16 devset1;
    let mut ret: c_int = 0;
    int idx;
    info = (struct pmbus_driver_info *)pmbus_get_driver_info(client);
    data = to_max20730_data(info);
    mutex_lock(&data.lock);
    devset1 = data.mfr_devset1;
    switch (reg) {
    case PMBUS_OT_FAULT_LIMIT:
    devset1 &= ~(BIT(11) | BIT(12));
    if (direct_to_val(word, PSC_TEMPERATURE, info) < 140000)
    devset1 |= BIT(11);
    break;
    case PMBUS_IOUT_OC_FAULT_LIMIT:
    devset1 &= ~(BIT(5) | BIT(6));
    idx = find_closest(direct_to_val(word, PSC_CURRENT_OUT, info),
    max_current[data.id], 4);
    devset1 |= (idx << 5);
    break;
    default:
    ret = -ENODATA;
    break;
    }
    if (!ret && devset1 != data.mfr_devset1) {
    ret = i2c_smbus_write_word_data(client, MAX20730_MFR_DEVSET1,
    devset1);
    if (!ret) {
    data.mfr_devset1 = devset1;
    pmbus_clear_cache(client);
    }
    }
    mutex_unlock(&data.lock);
    return ret;
    }
    static const struct pmbus_driver_info max20730_info[] = {
    [max20710] = {
    .pages = 1,
    .read_word_data = max20730_read_word_data,
    .write_word_data = max20730_write_word_data,
// Source : Maxim AN6140 and AN6042
    .format[PSC_TEMPERATURE] = direct,
    .m[PSC_TEMPERATURE] = 21,
    .b[PSC_TEMPERATURE] = 5887,
    .R[PSC_TEMPERATURE] = -1,
    .format[PSC_VOLTAGE_IN] = direct,
    .m[PSC_VOLTAGE_IN] = 3609,
    .b[PSC_VOLTAGE_IN] = 0,
    .R[PSC_VOLTAGE_IN] = -2,
    .format[PSC_CURRENT_OUT] = direct,
    .m[PSC_CURRENT_OUT] = 153,
    .b[PSC_CURRENT_OUT] = 4976,
    .R[PSC_CURRENT_OUT] = -1,
    .format[PSC_VOLTAGE_OUT] = linear,
    .func[0] = PMBUS_HAVE_VIN |
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP |
    PMBUS_HAVE_STATUS_INPUT,
    },
    [max20730] = {
    .pages = 1,
    .read_word_data = max20730_read_word_data,
    .write_word_data = max20730_write_word_data,
// Source : Maxim AN6042
    .format[PSC_TEMPERATURE] = direct,
    .m[PSC_TEMPERATURE] = 21,
    .b[PSC_TEMPERATURE] = 5887,
    .R[PSC_TEMPERATURE] = -1,
    .format[PSC_VOLTAGE_IN] = direct,
    .m[PSC_VOLTAGE_IN] = 3609,
    .b[PSC_VOLTAGE_IN] = 0,
    .R[PSC_VOLTAGE_IN] = -2,
//
// Values in the datasheet are adjusted for temperature and
// for the relationship between Vin and Vout.
// Unfortunately, the data sheet suggests that Vout measurement
// may be scaled with a resistor array. This is indeed the case
// at least on the evaulation boards. As a result, any in-driver
// adjustments would either be wrong or require elaborate means
// to configure the scaling. Instead of doing that, just report
// raw values and let userspace handle adjustments.
//
    .format[PSC_CURRENT_OUT] = direct,
    .m[PSC_CURRENT_OUT] = 153,
    .b[PSC_CURRENT_OUT] = 4976,
    .R[PSC_CURRENT_OUT] = -1,
    .format[PSC_VOLTAGE_OUT] = linear,
    .func[0] = PMBUS_HAVE_VIN |
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP |
    PMBUS_HAVE_STATUS_INPUT,
    },
    [max20734] = {
    .pages = 1,
    .read_word_data = max20730_read_word_data,
    .write_word_data = max20730_write_word_data,
// Source : Maxim AN6209
    .format[PSC_TEMPERATURE] = direct,
    .m[PSC_TEMPERATURE] = 21,
    .b[PSC_TEMPERATURE] = 5887,
    .R[PSC_TEMPERATURE] = -1,
    .format[PSC_VOLTAGE_IN] = direct,
    .m[PSC_VOLTAGE_IN] = 3592,
    .b[PSC_VOLTAGE_IN] = 0,
    .R[PSC_VOLTAGE_IN] = -2,
    .format[PSC_CURRENT_OUT] = direct,
    .m[PSC_CURRENT_OUT] = 111,
    .b[PSC_CURRENT_OUT] = 3461,
    .R[PSC_CURRENT_OUT] = -1,
    .format[PSC_VOLTAGE_OUT] = linear,
    .func[0] = PMBUS_HAVE_VIN |
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP |
    PMBUS_HAVE_STATUS_INPUT,
    },
    [max20743] = {
    .pages = 1,
    .read_word_data = max20730_read_word_data,
    .write_word_data = max20730_write_word_data,
// Source : Maxim AN6042
    .format[PSC_TEMPERATURE] = direct,
    .m[PSC_TEMPERATURE] = 21,
    .b[PSC_TEMPERATURE] = 5887,
    .R[PSC_TEMPERATURE] = -1,
    .format[PSC_VOLTAGE_IN] = direct,
    .m[PSC_VOLTAGE_IN] = 3597,
    .b[PSC_VOLTAGE_IN] = 0,
    .R[PSC_VOLTAGE_IN] = -2,
    .format[PSC_CURRENT_OUT] = direct,
    .m[PSC_CURRENT_OUT] = 95,
    .b[PSC_CURRENT_OUT] = 5014,
    .R[PSC_CURRENT_OUT] = -1,
    .format[PSC_VOLTAGE_OUT] = linear,
    .func[0] = PMBUS_HAVE_VIN |
    PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP |
    PMBUS_HAVE_STATUS_INPUT,
    },
    };
#[no_mangle]
unsafe extern "C" fn max20730_probe(client: *mut i2c_client) -> c_int {
    static int max20730_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    u8 buf[I2C_SMBUS_BLOCK_MAX + 1];
    struct max20730_data *data;
    enum chips chip_id;
    int ret;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_READ_BYTE_DATA |
    I2C_FUNC_SMBUS_READ_WORD_DATA |
    I2C_FUNC_SMBUS_BLOCK_DATA))
    return -ENODEV;
    ret = i2c_smbus_read_block_data(client, PMBUS_MFR_ID, buf);
    if (ret < 0) {
    dev_err(&client.dev, "Failed to read Manufacturer ID\n");
    return ret;
    }
    if (ret != 5 || strncmp(buf, "MAXIM", 5)) {
    buf[ret] = '\0';
    dev_err(dev, "Unsupported Manufacturer ID '%s'\n", buf);
    return -ENODEV;
    }
//
// The chips support reading PMBUS_MFR_MODEL. On both MAX20730
// and MAX20734, reading it returns M20743. Presumably that is
// the reason why the command is not documented. Unfortunately,
// that means that there is no reliable means to detect the chip.
// However, we can at least detect the chip series. Compare
// the returned value against 'M20743' and bail out if there is
// a mismatch. If that doesn't work for all chips, we may have
// to remove this check.
//
    ret = i2c_smbus_read_block_data(client, PMBUS_MFR_MODEL, buf);
    if (ret < 0) {
    dev_err(dev, "Failed to read Manufacturer Model\n");
    return ret;
    }
    if (ret != 6 || strncmp(buf, "M20743", 6)) {
    buf[ret] = '\0';
    dev_err(dev, "Unsupported Manufacturer Model '%s'\n", buf);
    return -ENODEV;
    }
    ret = i2c_smbus_read_block_data(client, PMBUS_MFR_REVISION, buf);
    if (ret < 0) {
    dev_err(dev, "Failed to read Manufacturer Revision\n");
    return ret;
    }
    if (ret != 1 || buf[0] != 'F') {
    buf[ret] = '\0';
    dev_err(dev, "Unsupported Manufacturer Revision '%s'\n", buf);
    return -ENODEV;
    }
    chip_id = (uintptr_t)i2c_get_match_data(client);
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.id = chip_id;
    mutex_init(&data.lock);
    memcpy(&data.info, &max20730_info[chip_id], sizeof(data.info));
    if (of_property_read_u32_array(client.dev.of_node, "vout-voltage-divider",
    data.vout_voltage_divider,
    ARRAY_SIZE(data.vout_voltage_divider)) != 0)
    memset(data.vout_voltage_divider, 0, sizeof(data.vout_voltage_divider));
    if (data.vout_voltage_divider[1] < data.vout_voltage_divider[0]) {
    dev_err(dev,
    "The total resistance of voltage divider is less than output resistance\n");
    return -EINVAL;
    }
    ret = i2c_smbus_read_word_data(client, MAX20730_MFR_DEVSET1);
    if (ret < 0)
    return ret;
    data.mfr_devset1 = ret;
    ret = pmbus_do_probe(client, &data.info);
    if (ret < 0)
    return ret;
    ret = max20730_init_debugfs(client, data);
    if (ret)
    dev_warn(dev, "Failed to register debugfs: %d\n",
    ret);
    return 0;
    }
    static const struct i2c_device_id max20730_id[] = {
    { .name = "max20710", .driver_data = max20710 },
    { .name = "max20730", .driver_data = max20730 },
    { .name = "max20734", .driver_data = max20734 },
    { .name = "max20743", .driver_data = max20743 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max20730_id);
    static const struct of_device_id max20730_of_match[] = {
    { .compatible = "maxim,max20710", .data = (void *)max20710 },
    { .compatible = "maxim,max20730", .data = (void *)max20730 },
    { .compatible = "maxim,max20734", .data = (void *)max20734 },
    { .compatible = "maxim,max20743", .data = (void *)max20743 },
    { },
    };
    MODULE_DEVICE_TABLE(of, max20730_of_match);
    static struct i2c_driver max20730_driver = {
    .driver = {
    .name = "max20730",
    .of_match_table = max20730_of_match,
    },
    .probe = max20730_probe,
    .id_table = max20730_id,
    };
    module_i2c_driver(max20730_driver);
    MODULE_AUTHOR("Guenter Roeck <linux@roeck-us.net>");
    MODULE_DESCRIPTION("PMBus driver for Maxim MAX20710 / MAX20730 / MAX20734 / MAX20743");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
