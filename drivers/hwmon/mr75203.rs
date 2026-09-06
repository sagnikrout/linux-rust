//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/mr75203.c
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
// Copyright (C) 2020 MaxLinear, Inc.
//
// This driver is a hardware monitoring driver for PVT controller
// (MR75203) which is used to configure & control Moortec embedded
// analog IP to enable multiple embedded temperature sensor(TS),
// voltage monitor(VM) & process detector(PD) modules.
//

// PVT Common register
pub const PVT_IP_CONFIG: c_uint = 0x04;

pub const TS_NUM_SFT: c_int = 0;

pub const PD_NUM_SFT: c_int = 8;

pub const VM_NUM_SFT: c_int = 16;

pub const CH_NUM_SFT: c_int = 24;

// Macro Common Register
pub const CLK_SYNTH: c_uint = 0x00;
pub const CLK_SYNTH_LO_SFT: c_int = 0;
pub const CLK_SYNTH_HI_SFT: c_int = 8;
pub const CLK_SYNTH_HOLD_SFT: c_int = 16;

pub const CLK_SYS_CYCLES_MAX: c_int = 514;
pub const CLK_SYS_CYCLES_MIN: c_int = 2;
pub const SDIF_DISABLE: c_uint = 0x04;
pub const SDIF_STAT: c_uint = 0x08;

pub const SDIF_W: c_uint = 0x0c;

pub const SDIF_WRN_R: c_uint = 0x00;
pub const SDIF_ADDR_SFT: c_int = 24;
pub const SDIF_HALT: c_uint = 0x10;
pub const SDIF_CTRL: c_uint = 0x14;
pub const SDIF_SMPL_CTRL: c_uint = 0x20;
// TS & PD Individual Macro Register
pub const COM_REG_SIZE: c_uint = 0x40;

// VM Individual Macro Register
pub const VM_COM_REG_SIZE: c_uint = 0x200;

    (VM_COM_REG_SIZE + 0x40 + 0x200 * (vm) + 0x4 * (ch))
// SDA Slave Register
pub const IP_CTRL: c_uint = 0x00;

pub const IP_CFG: c_uint = 0x01;

pub const CFG0_PARALLEL_OUT: c_int = 0;
pub const CFG0_12_BIT: c_int = 0;
pub const CFG1_VOL_MEAS_MODE: c_int = 0;
pub const CFG1_PARALLEL_OUT: c_int = 0;
pub const CFG1_14_BIT: c_int = 0;
pub const IP_DATA: c_uint = 0x03;
pub const IP_POLL: c_uint = 0x04;

pub const IP_TMR: c_uint = 0x05;
pub const POWER_DELAY_CYCLE_256: c_uint = 0x100;
pub const POWER_DELAY_CYCLE_64: c_uint = 0x40;
pub const PVT_POLL_DELAY_US: c_int = 20;
pub const PVT_POLL_TIMEOUT_US: c_int = 20000;
pub const PVT_CONV_BITS: c_int = 10;
pub const PVT_N_CONST: c_int = 90;
pub const PVT_R_CONST: c_int = 245805;

pub const PVT_TEMP_MAX_mC: c_int = 125000;
// Temperature coefficients for series 5
pub const PVT_SERIES5_H_CONST: c_int = 200000;
pub const PVT_SERIES5_G_CONST: c_int = 60000;

pub const PVT_SERIES5_CAL5_CONST: c_int = 4094;
// Temperature coefficients for series 6
pub const PVT_SERIES6_H_CONST: c_int = 249400;
pub const PVT_SERIES6_G_CONST: c_int = 57400;
pub const PVT_SERIES6_J_CONST: c_int = 0;
pub const PVT_SERIES6_CAL5_CONST: c_int = 4096;
pub const TEMPERATURE_SENSOR_SERIES_5: c_int = 5;
pub const TEMPERATURE_SENSOR_SERIES_6: c_int = 6;
pub const PRE_SCALER_X1: c_int = 1;
pub const PRE_SCALER_X2: c_int = 2;
//
// struct voltage_device - VM single input parameters.
// @vm_map: Map channel number to VM index.
// @ch_map: Map channel number to channel index.
// @pre_scaler: Pre scaler value (1 or 2) used to normalize the voltage output
// result.
//
// The structure provides mapping between channel-number (0..N-1) to VM-index
// (0..num_vm-1) and channel-index (0..ch_num-1) where N = num_vm * ch_num.
// It also provides normalization factor for the VM equation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct voltage_device {
    pub vm_map: u32,
    pub ch_map: u32,
    pub pre_scaler: u32,
}

//
// struct voltage_channels - VM channel count.
// @total: Total number of channels in all VMs.
// @max: Maximum number of channels among all VMs.
//
// The structure provides channel count information across all VMs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct voltage_channels {
    pub total: u32,
    pub max: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct temp_coeff {
    pub h: u32,
    pub g: u32,
    pub cal5: u32,
    pub j: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvt_device {
    pub c_map: *mut regmap,
    pub t_map: *mut regmap,
    pub p_map: *mut regmap,
    pub v_map: *mut regmap,
    pub clk: *mut clk,
    pub rst: *mut reset_control,
    pub dbgfs_dir: *mut dentry,
    pub vd: *mut voltage_device,
    pub vm_channels: voltage_channels,
    pub ts_coeff: temp_coeff,
    pub t_num: u32,
    pub p_num: u32,
    pub v_num: u32,
    pub ip_freq: u32,
}

    static ssize_t pvt_ts_coeff_j_read(struct file *file, char __user *user_buf,
    size_t count, loff_t *ppos)
    {
    struct pvt_device *pvt = file.private_data;
    unsigned int len;
    char buf[13];
    len = scnprintf(buf, sizeof(buf), "%d\n", pvt.ts_coeff.j);
    return simple_read_from_buffer(user_buf, count, ppos, buf, len);
    }
    static ssize_t pvt_ts_coeff_j_write(struct file *file,
    const char __user *user_buf,
    size_t count, loff_t *ppos)
    {
    struct pvt_device *pvt = file.private_data;
    int ret;
    ret = kstrtos32_from_user(user_buf, count, 0, &pvt.ts_coeff.j);
    if (ret)
    return ret;
    return count;
    }
    static const struct file_operations pvt_ts_coeff_j_fops = {
    .read = pvt_ts_coeff_j_read,
    .write = pvt_ts_coeff_j_write,
    .open = simple_open,
    .owner = THIS_MODULE,
    .llseek = default_llseek,
    };
#[no_mangle]
unsafe extern "C" fn devm_pvt_ts_dbgfs_remove(data: *mut c_void) {
    static void devm_pvt_ts_dbgfs_remove(void *data)
    {
    struct pvt_device *pvt = (struct pvt_device *)data;
    debugfs_remove_recursive(pvt.dbgfs_dir);
    pvt.dbgfs_dir = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn pvt_ts_dbgfs_create(pvt: *mut pvt_device, dev: *mut device) -> c_int {
    static int pvt_ts_dbgfs_create(struct pvt_device *pvt, struct device *dev)
    {
    pvt.dbgfs_dir = debugfs_create_dir(dev_name(dev), core::ptr::null_mut());
    debugfs_create_u32("ts_coeff_h", 0644, pvt.dbgfs_dir,
    &pvt.ts_coeff.h);
    debugfs_create_u32("ts_coeff_g", 0644, pvt.dbgfs_dir,
    &pvt.ts_coeff.g);
    debugfs_create_u32("ts_coeff_cal5", 0644, pvt.dbgfs_dir,
    &pvt.ts_coeff.cal5);
    debugfs_create_file("ts_coeff_j", 0644, pvt.dbgfs_dir, pvt,
    &pvt_ts_coeff_j_fops);
    return devm_add_action_or_reset(dev, devm_pvt_ts_dbgfs_remove, pvt);
    }
    static umode_t pvt_is_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_temp:
    if (attr == hwmon_temp_input)
    return 0444;
    break;
    case hwmon_in:
    if (attr == hwmon_in_input)
    return 0444;
    break;
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pvt_calc_temp(pvt: *mut pvt_device, nbs: u32) -> c_long {
    static long pvt_calc_temp(struct pvt_device *pvt, u32 nbs)
    {
//
// Convert the register value to degrees centigrade temperature:
// T = G + H * (n / cal5 - 0.5) + J * F
//
    struct temp_coeff *ts_coeff = &pvt.ts_coeff;
    s64 tmp = ts_coeff.g +
    div_s64(ts_coeff.h * (s64)nbs, ts_coeff.cal5) -
    ts_coeff.h / 2 +
    div_s64(ts_coeff.j * (s64)pvt.ip_freq, HZ_PER_MHZ);
    return clamp_val(tmp, PVT_TEMP_MIN_mC, PVT_TEMP_MAX_mC);
    }
#[no_mangle]
unsafe extern "C" fn pvt_read_temp(dev: *mut device, attr: u32, channel: c_int, val: *mut c_long) -> c_int {
    static int pvt_read_temp(struct device *dev, u32 attr, int channel, long *val)
    {
    struct pvt_device *pvt = dev_get_drvdata(dev);
    struct regmap *t_map = pvt.t_map;
    u32 stat, nbs;
    int ret;
    switch (attr) {
    case hwmon_temp_input:
    ret = regmap_read_poll_timeout(t_map, SDIF_DONE(channel),
    stat, stat & SDIF_SMPL_DONE,
    PVT_POLL_DELAY_US,
    PVT_POLL_TIMEOUT_US);
    if (ret)
    return ret;
    ret = regmap_read(t_map, SDIF_DATA(channel), &nbs);
    if (ret < 0)
    return ret;
    nbs &= SAMPLE_DATA_MSK;
//
// Convert the register value to
// degrees centigrade temperature
//
// val = pvt_calc_temp(pvt, nbs);
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn pvt_read_in(dev: *mut device, attr: u32, channel: c_int, val: *mut c_long) -> c_int {
    static int pvt_read_in(struct device *dev, u32 attr, int channel, long *val)
    {
    struct pvt_device *pvt = dev_get_drvdata(dev);
    struct regmap *v_map = pvt.v_map;
    u32 n, stat, pre_scaler;
    u8 vm_idx, ch_idx;
    int ret;
    if (channel >= pvt.vm_channels.total)
    return -EINVAL;
    vm_idx = pvt.vd[channel].vm_map;
    ch_idx = pvt.vd[channel].ch_map;
    switch (attr) {
    case hwmon_in_input:
    ret = regmap_read_poll_timeout(v_map, VM_SDIF_DONE(vm_idx),
    stat, stat & SDIF_SMPL_DONE,
    PVT_POLL_DELAY_US,
    PVT_POLL_TIMEOUT_US);
    if (ret)
    return ret;
    ret = regmap_read(v_map, VM_SDIF_DATA(vm_idx, ch_idx), &n);
    if (ret < 0)
    return ret;
    n &= SAMPLE_DATA_MSK;
    pre_scaler = pvt.vd[channel].pre_scaler;
//
// Convert the N bitstream count into voltage.
// To support negative voltage calculation for 64bit machines
// n must be cast to long, since n and *val differ both in
// signedness and in size.
// Division is used instead of right shift, because for signed
// numbers, the sign bit is used to fill the vacated bit
// positions, and if the number is negative, 1 is used.
// BIT(x) may not be used instead of (1 << x) because it's
// unsigned.
//
// val = pre_scaler * (PVT_N_CONST * (long)n - PVT_R_CONST)
    (1 << PVT_CONV_BITS);
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static int pvt_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    switch (type) {
    case hwmon_temp:
    return pvt_read_temp(dev, attr, channel, val);
    case hwmon_in:
    return pvt_read_in(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static struct hwmon_channel_info pvt_temp = {
    .type = hwmon_temp,
    };
    static struct hwmon_channel_info pvt_in = {
    .type = hwmon_in,
    };
    static const struct hwmon_ops pvt_hwmon_ops = {
    .is_visible = pvt_is_visible,
    .read = pvt_read,
    };
    static struct hwmon_chip_info pvt_chip_info = {
    .ops = &pvt_hwmon_ops,
    };
#[no_mangle]
unsafe extern "C" fn pvt_init(pvt: *mut pvt_device) -> c_int {
    static int pvt_init(struct pvt_device *pvt)
    {
    u16 sys_freq, key, middle, low = 4, high = 8;
    struct regmap *t_map = pvt.t_map;
    struct regmap *p_map = pvt.p_map;
    struct regmap *v_map = pvt.v_map;
    let mut t_num: u32 = pvt.t_num;
    let mut p_num: u32 = pvt.p_num;
    let mut v_num: u32 = pvt.v_num;
    u32 clk_synth, val;
    int ret;
    sys_freq = clk_get_rate(pvt.clk) / HZ_PER_MHZ;
    while (high >= low) {
    middle = (low + high + 1) / 2;
    key = DIV_ROUND_CLOSEST(sys_freq, middle);
    if (key > CLK_SYS_CYCLES_MAX) {
    low = middle + 1;
    continue;
    } else if (key < CLK_SYS_CYCLES_MIN) {
    high = middle - 1;
    continue;
    } else {
    break;
    }
    }
//
// The system supports 'clk_sys' to 'clk_ip' frequency ratios
// from 2:1 to 512:1
//
    key = clamp_val(key, CLK_SYS_CYCLES_MIN, CLK_SYS_CYCLES_MAX) - 2;
    clk_synth = ((key + 1) >> 1) << CLK_SYNTH_LO_SFT |
    (key >> 1) << CLK_SYNTH_HI_SFT |
    (key >> 1) << CLK_SYNTH_HOLD_SFT | CLK_SYNTH_EN;
    pvt.ip_freq = clk_get_rate(pvt.clk) / (key + 2);
    if (t_num) {
    ret = regmap_write(t_map, SDIF_SMPL_CTRL, 0x0);
    if (ret < 0)
    return ret;
    ret = regmap_write(t_map, SDIF_HALT, 0x0);
    if (ret < 0)
    return ret;
    ret = regmap_write(t_map, CLK_SYNTH, clk_synth);
    if (ret < 0)
    return ret;
    ret = regmap_write(t_map, SDIF_DISABLE, 0x0);
    if (ret < 0)
    return ret;
    ret = regmap_read_poll_timeout(t_map, SDIF_STAT,
    val, !(val & SDIF_BUSY),
    PVT_POLL_DELAY_US,
    PVT_POLL_TIMEOUT_US);
    if (ret)
    return ret;
    val = CFG0_MODE_2 | CFG0_PARALLEL_OUT | CFG0_12_BIT |
    IP_CFG << SDIF_ADDR_SFT | SDIF_WRN_W | SDIF_PROG;
    ret = regmap_write(t_map, SDIF_W, val);
    if (ret < 0)
    return ret;
    ret = regmap_read_poll_timeout(t_map, SDIF_STAT,
    val, !(val & SDIF_BUSY),
    PVT_POLL_DELAY_US,
    PVT_POLL_TIMEOUT_US);
    if (ret)
    return ret;
    val = POWER_DELAY_CYCLE_256 | IP_TMR << SDIF_ADDR_SFT |
    SDIF_WRN_W | SDIF_PROG;
    ret = regmap_write(t_map, SDIF_W, val);
    if (ret < 0)
    return ret;
    ret = regmap_read_poll_timeout(t_map, SDIF_STAT,
    val, !(val & SDIF_BUSY),
    PVT_POLL_DELAY_US,
    PVT_POLL_TIMEOUT_US);
    if (ret)
    return ret;
    val = IP_RST_REL | IP_RUN_CONT | IP_AUTO |
    IP_CTRL << SDIF_ADDR_SFT |
    SDIF_WRN_W | SDIF_PROG;
    ret = regmap_write(t_map, SDIF_W, val);
    if (ret < 0)
    return ret;
    }
    if (p_num) {
    ret = regmap_write(p_map, SDIF_HALT, 0x0);
    if (ret < 0)
    return ret;
    ret = regmap_write(p_map, SDIF_DISABLE, BIT(p_num) - 1);
    if (ret < 0)
    return ret;
    ret = regmap_write(p_map, CLK_SYNTH, clk_synth);
    if (ret < 0)
    return ret;
    }
    if (v_num) {
    ret = regmap_write(v_map, SDIF_SMPL_CTRL, 0x0);
    if (ret < 0)
    return ret;
    ret = regmap_write(v_map, SDIF_HALT, 0x0);
    if (ret < 0)
    return ret;
    ret = regmap_write(v_map, CLK_SYNTH, clk_synth);
    if (ret < 0)
    return ret;
    ret = regmap_write(v_map, SDIF_DISABLE, 0x0);
    if (ret < 0)
    return ret;
    ret = regmap_read_poll_timeout(v_map, SDIF_STAT,
    val, !(val & SDIF_BUSY),
    PVT_POLL_DELAY_US,
    PVT_POLL_TIMEOUT_US);
    if (ret)
    return ret;
    val = (BIT(pvt.vm_channels.max) - 1) | VM_CH_INIT |
    IP_POLL << SDIF_ADDR_SFT | SDIF_WRN_W | SDIF_PROG;
    ret = regmap_write(v_map, SDIF_W, val);
    if (ret < 0)
    return ret;
    ret = regmap_read_poll_timeout(v_map, SDIF_STAT,
    val, !(val & SDIF_BUSY),
    PVT_POLL_DELAY_US,
    PVT_POLL_TIMEOUT_US);
    if (ret)
    return ret;
    val = CFG1_VOL_MEAS_MODE | CFG1_PARALLEL_OUT |
    CFG1_14_BIT | IP_CFG << SDIF_ADDR_SFT |
    SDIF_WRN_W | SDIF_PROG;
    ret = regmap_write(v_map, SDIF_W, val);
    if (ret < 0)
    return ret;
    ret = regmap_read_poll_timeout(v_map, SDIF_STAT,
    val, !(val & SDIF_BUSY),
    PVT_POLL_DELAY_US,
    PVT_POLL_TIMEOUT_US);
    if (ret)
    return ret;
    val = POWER_DELAY_CYCLE_64 | IP_TMR << SDIF_ADDR_SFT |
    SDIF_WRN_W | SDIF_PROG;
    ret = regmap_write(v_map, SDIF_W, val);
    if (ret < 0)
    return ret;
    ret = regmap_read_poll_timeout(v_map, SDIF_STAT,
    val, !(val & SDIF_BUSY),
    PVT_POLL_DELAY_US,
    PVT_POLL_TIMEOUT_US);
    if (ret)
    return ret;
    val = IP_RST_REL | IP_RUN_CONT | IP_AUTO | IP_VM_MODE |
    IP_CTRL << SDIF_ADDR_SFT |
    SDIF_WRN_W | SDIF_PROG;
    ret = regmap_write(v_map, SDIF_W, val);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
    static struct regmap_config pvt_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    };
    static int pvt_get_regmap(struct platform_device *pdev, char *reg_name,
    struct pvt_device *pvt)
    {
    struct device *dev = &pdev.dev;
    struct regmap **reg_map;
    void __iomem *io_base;
    if (!strcmp(reg_name, "common"))
    reg_map = &pvt.c_map;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(reg_name, _arg: "ts")) -> else {
    else if (!strcmp(reg_name, "ts"))
    reg_map = &pvt.t_map;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(reg_name, _arg: "pd")) -> else {
    else if (!strcmp(reg_name, "pd"))
    reg_map = &pvt.p_map;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(reg_name, _arg: "vm")) -> else {
    else if (!strcmp(reg_name, "vm"))
    reg_map = &pvt.v_map;
    else
    return -EINVAL;
    io_base = devm_platform_ioremap_resource_byname(pdev, reg_name);
    if (IS_ERR(io_base))
    return PTR_ERR(io_base);
    pvt_regmap_config.name = reg_name;
// reg_map = devm_regmap_init_mmio(dev, io_base, &pvt_regmap_config);
    if (IS_ERR(*reg_map)) {
    dev_err(dev, "failed to init register map\n");
    return PTR_ERR(*reg_map);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pvt_reset_control_assert(data: *mut c_void) {
    static void pvt_reset_control_assert(void *data)
    {
    struct pvt_device *pvt = data;
    reset_control_assert(pvt.rst);
    }
#[no_mangle]
unsafe extern "C" fn pvt_reset_control_deassert(dev: *mut device, pvt: *mut pvt_device) -> c_int {
    static int pvt_reset_control_deassert(struct device *dev, struct pvt_device *pvt)
    {
    int ret;
    ret = reset_control_deassert(pvt.rst);
    if (ret)
    return ret;
    return devm_add_action_or_reset(dev, pvt_reset_control_assert, pvt);
    }
    static int pvt_get_active_channel(struct device *dev, struct pvt_device *pvt,
    u32 vm_num, u32 ch_num, u8 *vm_idx)
    {
    u8 vm_active_ch[VM_NUM_MAX];
    int ret, i, j, k;
    ret = device_property_read_u8_array(dev, "moortec,vm-active-channels",
    vm_active_ch, vm_num);
    if (ret) {
//
// Incase "moortec,vm-active-channels" property is not defined,
// we assume each VM sensor has all of its channels active.
//
    memset(vm_active_ch, ch_num, vm_num);
    pvt.vm_channels.max = ch_num;
    pvt.vm_channels.total = ch_num * vm_num;
    } else {
    for (i = 0; i < vm_num; i++) {
    if (vm_active_ch[i] > ch_num) {
    dev_err(dev, "invalid active channels: %u\n",
    vm_active_ch[i]);
    return -EINVAL;
    }
    pvt.vm_channels.total += vm_active_ch[i];
    if (vm_active_ch[i] > pvt.vm_channels.max)
    pvt.vm_channels.max = vm_active_ch[i];
    }
    }
//
// Map between the channel-number to VM-index and channel-index.
// Example - 3 VMs, "moortec,vm_active_ch" = <5 2 4>:
// vm_map = [0 0 0 0 0 1 1 2 2 2 2]
// ch_map = [0 1 2 3 4 0 1 0 1 2 3]
//
    pvt.vd = devm_kcalloc(dev, pvt.vm_channels.total, sizeof(*pvt.vd),
    GFP_KERNEL);
    if (!pvt.vd)
    return -ENOMEM;
    k = 0;
    for (i = 0; i < vm_num; i++) {
    for (j = 0; j < vm_active_ch[i]; j++) {
    pvt.vd[k].vm_map = vm_idx[i];
    pvt.vd[k].ch_map = j;
    k++;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pvt_get_pre_scaler(dev: *mut device, pvt: *mut pvt_device) -> c_int {
    static int pvt_get_pre_scaler(struct device *dev, struct pvt_device *pvt)
    {
    u8 *pre_scaler_ch_list;
    int i, ret, num_ch;
    u32 channel;
// Set default pre-scaler value to be 1.
    for (i = 0; i < pvt.vm_channels.total; i++)
    pvt.vd[i].pre_scaler = PRE_SCALER_X1;
// Get number of channels configured in "moortec,vm-pre-scaler-x2".
    num_ch = device_property_count_u8(dev, "moortec,vm-pre-scaler-x2");
    if (num_ch <= 0)
    return 0;
    pre_scaler_ch_list = kcalloc(num_ch, sizeof(*pre_scaler_ch_list),
    GFP_KERNEL);
    if (!pre_scaler_ch_list)
    return -ENOMEM;
// Get list of all channels that have pre-scaler of 2.
    ret = device_property_read_u8_array(dev, "moortec,vm-pre-scaler-x2",
    pre_scaler_ch_list, num_ch);
    if (ret)
    goto out;
    for (i = 0; i < num_ch; i++) {
    channel = pre_scaler_ch_list[i];
    pvt.vd[channel].pre_scaler = PRE_SCALER_X2;
    }
    out:
    kfree(pre_scaler_ch_list);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pvt_set_temp_coeff(dev: *mut device, pvt: *mut pvt_device) -> c_int {
    static int pvt_set_temp_coeff(struct device *dev, struct pvt_device *pvt)
    {
    struct temp_coeff *ts_coeff = &pvt.ts_coeff;
    u32 series;
    int ret;
// Incase ts-series property is not defined, use default 5.
    ret = device_property_read_u32(dev, "moortec,ts-series", &series);
    if (ret)
    series = TEMPERATURE_SENSOR_SERIES_5;
    switch (series) {
    case TEMPERATURE_SENSOR_SERIES_5:
    ts_coeff.h = PVT_SERIES5_H_CONST;
    ts_coeff.g = PVT_SERIES5_G_CONST;
    ts_coeff.j = PVT_SERIES5_J_CONST;
    ts_coeff.cal5 = PVT_SERIES5_CAL5_CONST;
    break;
    case TEMPERATURE_SENSOR_SERIES_6:
    ts_coeff.h = PVT_SERIES6_H_CONST;
    ts_coeff.g = PVT_SERIES6_G_CONST;
    ts_coeff.j = PVT_SERIES6_J_CONST;
    ts_coeff.cal5 = PVT_SERIES6_CAL5_CONST;
    break;
    default:
    dev_err(dev, "invalid temperature sensor series (%u)\n",
    series);
    return -EINVAL;
    }
    dev_dbg(dev, "temperature sensor series = %u\n", series);
// Override ts-coeff-h/g/j/cal5 if they are defined.
    device_property_read_u32(dev, "moortec,ts-coeff-h", &ts_coeff.h);
    device_property_read_u32(dev, "moortec,ts-coeff-g", &ts_coeff.g);
    device_property_read_u32(dev, "moortec,ts-coeff-j", &ts_coeff.j);
    device_property_read_u32(dev, "moortec,ts-coeff-cal5", &ts_coeff.cal5);
    dev_dbg(dev, "ts-coeff: h = %u, g = %u, j = %d, cal5 = %u\n",
    ts_coeff.h, ts_coeff.g, ts_coeff.j, ts_coeff.cal5);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mr75203_probe(pdev: *mut platform_device) -> c_int {
    static int mr75203_probe(struct platform_device *pdev)
    {
    u32 ts_num, vm_num, pd_num, ch_num, val, index, i;
    const struct hwmon_channel_info **pvt_info;
    struct device *dev = &pdev.dev;
    u32 *temp_config, *in_config;
    struct device *hwmon_dev;
    struct pvt_device *pvt;
    int ret;
    pvt = devm_kzalloc(dev, sizeof(*pvt), GFP_KERNEL);
    if (!pvt)
    return -ENOMEM;
    ret = pvt_get_regmap(pdev, "common", pvt);
    if (ret)
    return ret;
    pvt.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(pvt.clk))
    return dev_err_probe(dev, PTR_ERR(pvt.clk), "failed to get clock\n");
    pvt.rst = devm_reset_control_get_optional_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(pvt.rst))
    return dev_err_probe(dev, PTR_ERR(pvt.rst),
    "failed to get reset control\n");
    if (pvt.rst) {
    ret = pvt_reset_control_deassert(dev, pvt);
    if (ret)
    return dev_err_probe(dev, ret,
    "cannot deassert reset control\n");
    }
    ret = regmap_read(pvt.c_map, PVT_IP_CONFIG, &val);
    if (ret < 0)
    return ret;
    ts_num = (val & TS_NUM_MSK) >> TS_NUM_SFT;
    pd_num = (val & PD_NUM_MSK) >> PD_NUM_SFT;
    vm_num = (val & VM_NUM_MSK) >> VM_NUM_SFT;
    ch_num = (val & CH_NUM_MSK) >> CH_NUM_SFT;
    pvt.t_num = ts_num;
    pvt.p_num = pd_num;
    pvt.v_num = vm_num;
    val = 0;
    if (ts_num)
    val++;
    if (vm_num)
    val++;
    if (!val)
    return -ENODEV;
    pvt_info = devm_kcalloc(dev, val + 2, sizeof(*pvt_info), GFP_KERNEL);
    if (!pvt_info)
    return -ENOMEM;
    pvt_info[0] = HWMON_CHANNEL_INFO(chip, HWMON_C_REGISTER_TZ);
    index = 1;
    if (ts_num) {
    ret = pvt_get_regmap(pdev, "ts", pvt);
    if (ret)
    return ret;
    ret = pvt_set_temp_coeff(dev, pvt);
    if (ret)
    return ret;
    temp_config = devm_kcalloc(dev, ts_num + 1,
    sizeof(*temp_config), GFP_KERNEL);
    if (!temp_config)
    return -ENOMEM;
    memset32(temp_config, HWMON_T_INPUT, ts_num);
    pvt_temp.config = temp_config;
    pvt_info[index++] = &pvt_temp;
    pvt_ts_dbgfs_create(pvt, dev);
    }
    if (pd_num) {
    ret = pvt_get_regmap(pdev, "pd", pvt);
    if (ret)
    return ret;
    }
    if (vm_num) {
    u8 vm_idx[VM_NUM_MAX];
    ret = pvt_get_regmap(pdev, "vm", pvt);
    if (ret)
    return ret;
    ret = device_property_read_u8_array(dev, "intel,vm-map", vm_idx,
    vm_num);
    if (ret) {
//
// Incase intel,vm-map property is not defined, we
// assume incremental channel numbers.
//
    for (i = 0; i < vm_num; i++)
    vm_idx[i] = i;
    } else {
    for (i = 0; i < vm_num; i++)
    if (vm_idx[i] >= vm_num || vm_idx[i] == 0xff) {
    pvt.v_num = i;
    vm_num = i;
    break;
    }
    }
    ret = pvt_get_active_channel(dev, pvt, vm_num, ch_num, vm_idx);
    if (ret)
    return ret;
    ret = pvt_get_pre_scaler(dev, pvt);
    if (ret)
    return ret;
    in_config = devm_kcalloc(dev, pvt.vm_channels.total + 1,
    sizeof(*in_config), GFP_KERNEL);
    if (!in_config)
    return -ENOMEM;
    memset32(in_config, HWMON_I_INPUT, pvt.vm_channels.total);
    in_config[pvt.vm_channels.total] = 0;
    pvt_in.config = in_config;
    pvt_info[index++] = &pvt_in;
    }
    ret = pvt_init(pvt);
    if (ret) {
    dev_err(dev, "failed to init pvt: %d\n", ret);
    return ret;
    }
    pvt_chip_info.info = pvt_info;
    hwmon_dev = devm_hwmon_device_register_with_info(dev, "pvt",
    pvt,
    &pvt_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct of_device_id moortec_pvt_of_match[] = {
    { .compatible = "moortec,mr75203" },
    { }
    };
    MODULE_DEVICE_TABLE(of, moortec_pvt_of_match);
    static struct platform_driver moortec_pvt_driver = {
    .driver = {
    .name = "moortec-pvt",
    .of_match_table = moortec_pvt_of_match,
    },
    .probe = mr75203_probe,
    };
    module_platform_driver(moortec_pvt_driver);
    MODULE_DESCRIPTION("Moortec Semiconductor MR75203 PVT Controller driver");
    MODULE_LICENSE("GPL v2");
