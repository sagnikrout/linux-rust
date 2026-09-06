//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/dfl-n3000-nios.c
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
// DFL device driver for Nios private feature on Intel PAC (Programmable
// Acceleration Card) N3000
//
// Copyright (C) 2019-2020 Intel Corporation, Inc.
//
// Authors:
// Wu Hao <hao.wu@intel.com>
// Xu Yilun <yilun.xu@intel.com>
//

//
// N3000 Nios private feature registers, named as NIOS_SPI_XX on spec.
// NS is the abbreviation of NIOS_SPI.
//
pub const N3000_NS_PARAM: c_uint = 0x8;

pub const N3000_NS_PARAM_SHIFT_MODE_MSB: c_int = 0;
pub const N3000_NS_PARAM_SHIFT_MODE_LSB: c_int = 1;

pub const N3000_NS_CTRL: c_uint = 0x10;

pub const N3000_NS_CTRL_CMD_NOP: c_int = 0;
pub const N3000_NS_CTRL_CMD_RD: c_int = 1;
pub const N3000_NS_CTRL_CMD_WR: c_int = 2;
pub const N3000_NS_STAT: c_uint = 0x18;

// Nios handshake registers, indirect access
pub const N3000_NIOS_INIT: c_uint = 0x1000;

// Mode for retimer A, link 0, the same below

pub const N3000_NIOS_INIT_REQ_FEC_MODE_NO: c_uint = 0x0;
pub const N3000_NIOS_INIT_REQ_FEC_MODE_KR: c_uint = 0x1;
pub const N3000_NIOS_INIT_REQ_FEC_MODE_RS: c_uint = 0x2;
pub const N3000_NIOS_FW_VERSION: c_uint = 0x1004;

// The retimers we use on Intel PAC N3000 is Parkvale, abbreviated to PKVL
pub const N3000_NIOS_PKVL_A_MODE_STS: c_uint = 0x1020;
pub const N3000_NIOS_PKVL_B_MODE_STS: c_uint = 0x1024;

pub const N3000_NIOS_PKVL_MODE_STS_GROUP_OK: c_uint = 0x0;

// When GROUP MASK field == GROUP_OK
pub const N3000_NIOS_PKVL_MODE_ID_RESET: c_uint = 0x0;
pub const N3000_NIOS_PKVL_MODE_ID_4X10G: c_uint = 0x1;
pub const N3000_NIOS_PKVL_MODE_ID_4X25G: c_uint = 0x2;
pub const N3000_NIOS_PKVL_MODE_ID_2X25G: c_uint = 0x3;
pub const N3000_NIOS_PKVL_MODE_ID_2X25G_2X10G: c_uint = 0x4;
pub const N3000_NIOS_PKVL_MODE_ID_1X25G: c_uint = 0x5;

    (N3000_NIOS_INIT_REQ_FEC_MODE_A0_MSK |	\
    N3000_NIOS_INIT_REQ_FEC_MODE_A1_MSK |	\
    N3000_NIOS_INIT_REQ_FEC_MODE_A2_MSK |	\
    N3000_NIOS_INIT_REQ_FEC_MODE_A3_MSK |	\
    N3000_NIOS_INIT_REQ_FEC_MODE_B0_MSK |	\
    N3000_NIOS_INIT_REQ_FEC_MODE_B1_MSK |	\
    N3000_NIOS_INIT_REQ_FEC_MODE_B2_MSK |	\
    N3000_NIOS_INIT_REQ_FEC_MODE_B3_MSK)

    (FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_A0_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_NO) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_A1_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_NO) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_A2_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_NO) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_A3_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_NO) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_B0_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_NO) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_B1_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_NO) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_B2_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_NO) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_B3_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_NO))

    (FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_A0_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_KR) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_A1_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_KR) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_A2_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_KR) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_A3_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_KR) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_B0_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_KR) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_B1_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_KR) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_B2_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_KR) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_B3_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_KR))

    (FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_A0_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_RS) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_A1_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_RS) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_A2_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_RS) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_A3_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_RS) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_B0_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_RS) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_B1_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_RS) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_B2_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_RS) |		\
    FIELD_PREP(N3000_NIOS_INIT_REQ_FEC_MODE_B3_MSK,	\
    N3000_NIOS_INIT_REQ_FEC_MODE_RS))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct n3000_nios {
    pub base: *mut void __iomem,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub altera_spi: *mut platform_device,
}

    static ssize_t nios_fw_version_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct n3000_nios *nn = dev_get_drvdata(dev);
    unsigned int val;
    int ret;
    ret = regmap_read(nn.regmap, N3000_NIOS_FW_VERSION, &val);
    if (ret)
    return ret;
    return sysfs_emit(buf, "%x.%x.%x\n",
    (u8)FIELD_GET(N3000_NIOS_FW_VERSION_MAJOR, val),
    (u8)FIELD_GET(N3000_NIOS_FW_VERSION_MINOR, val),
    (u8)FIELD_GET(N3000_NIOS_FW_VERSION_PATCH, val));
    }
    static DEVICE_ATTR_RO(nios_fw_version);

    (FIELD_GET(N3000_NIOS_PKVL_MODE_STS_GROUP_MSK, (mode_stat)) ==	\
    N3000_NIOS_PKVL_MODE_STS_GROUP_OK)

    ((retimer_mode) != N3000_NIOS_PKVL_MODE_ID_RESET &&	\
    (retimer_mode) != N3000_NIOS_PKVL_MODE_ID_4X10G)
    static int get_retimer_mode(struct n3000_nios *nn, unsigned int mode_stat_reg,
    unsigned int *retimer_mode)
    {
    unsigned int val;
    int ret;
    ret = regmap_read(nn.regmap, mode_stat_reg, &val);
    if (ret)
    return ret;
    if (!IS_MODE_STATUS_OK(val))
    return -EFAULT;
// retimer_mode = FIELD_GET(N3000_NIOS_PKVL_MODE_STS_ID_MSK, val);
    return 0;
    }
    static ssize_t retimer_A_mode_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct n3000_nios *nn = dev_get_drvdata(dev);
    unsigned int mode;
    int ret;
    ret = get_retimer_mode(nn, N3000_NIOS_PKVL_A_MODE_STS, &mode);
    if (ret)
    return ret;
    return sysfs_emit(buf, "0x%x\n", mode);
    }
    static DEVICE_ATTR_RO(retimer_A_mode);
    static ssize_t retimer_B_mode_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct n3000_nios *nn = dev_get_drvdata(dev);
    unsigned int mode;
    int ret;
    ret = get_retimer_mode(nn, N3000_NIOS_PKVL_B_MODE_STS, &mode);
    if (ret)
    return ret;
    return sysfs_emit(buf, "0x%x\n", mode);
    }
    static DEVICE_ATTR_RO(retimer_B_mode);
    static ssize_t fec_mode_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    unsigned int val, retimer_a_mode, retimer_b_mode, fec_modes;
    struct n3000_nios *nn = dev_get_drvdata(dev);
    int ret;
// FEC mode setting is not supported in early FW versions
    ret = regmap_read(nn.regmap, N3000_NIOS_FW_VERSION, &val);
    if (ret)
    return ret;
    if (FIELD_GET(N3000_NIOS_FW_VERSION_MAJOR, val) < 3)
    return sysfs_emit(buf, "not supported\n");
// If no 25G links, FEC mode setting is not supported either
    ret = get_retimer_mode(nn, N3000_NIOS_PKVL_A_MODE_STS, &retimer_a_mode);
    if (ret)
    return ret;
    ret = get_retimer_mode(nn, N3000_NIOS_PKVL_B_MODE_STS, &retimer_b_mode);
    if (ret)
    return ret;
    if (!IS_RETIMER_FEC_SUPPORTED(retimer_a_mode) &&
    !IS_RETIMER_FEC_SUPPORTED(retimer_b_mode))
    return sysfs_emit(buf, "not supported\n");
// get the valid FEC mode for 25G links
    ret = regmap_read(nn.regmap, N3000_NIOS_INIT, &val);
    if (ret)
    return ret;
//
// FEC mode should always be the same for all links, as we set them
// in this way.
//
    fec_modes = (val & N3000_NIOS_INIT_REQ_FEC_MODE_MSK_ALL);
    if (fec_modes == N3000_NIOS_INIT_REQ_FEC_MODE_NO_ALL)
    return sysfs_emit(buf, "no\n");
#[no_mangle]
pub unsafe extern "C" fn if(N3000_NIOS_INIT_REQ_FEC_MODE_KR_ALL: fec_modes ==) -> else {
    else if (fec_modes == N3000_NIOS_INIT_REQ_FEC_MODE_KR_ALL)
    return sysfs_emit(buf, "kr\n");
#[no_mangle]
pub unsafe extern "C" fn if(N3000_NIOS_INIT_REQ_FEC_MODE_RS_ALL: fec_modes ==) -> else {
    else if (fec_modes == N3000_NIOS_INIT_REQ_FEC_MODE_RS_ALL)
    return sysfs_emit(buf, "rs\n");
    return -EFAULT;
    }
    static DEVICE_ATTR_RO(fec_mode);
    static struct attribute *n3000_nios_attrs[] = {
    &dev_attr_nios_fw_version.attr,
    &dev_attr_retimer_A_mode.attr,
    &dev_attr_retimer_B_mode.attr,
    &dev_attr_fec_mode.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(n3000_nios);
#[no_mangle]
unsafe extern "C" fn n3000_nios_init_done_check(nn: *mut n3000_nios) -> c_int {
    static int n3000_nios_init_done_check(struct n3000_nios *nn)
    {
    unsigned int val, state_a, state_b;
    struct device *dev = nn.dev;
    int ret, ret2;
//
// The SPI is shared by the Nios core inside the FPGA, Nios will use
// this SPI master to do some one time initialization after power up,
// and then release the control to OS. The driver needs to poll on
// INIT_DONE to see when driver could take the control.
//
// Please note that after Nios firmware version 3.0.0, INIT_START is
// introduced, so driver needs to trigger START firstly and then check
// INIT_DONE.
//
    ret = regmap_read(nn.regmap, N3000_NIOS_FW_VERSION, &val);
    if (ret)
    return ret;
//
// If Nios version register is totally uninitialized(== 0x0), then the
// Nios firmware is missing. So host could take control of SPI master
// safely, but initialization work for Nios is not done. To restore the
// card, we need to reprogram a new Nios firmware via the BMC chip on
// SPI bus. So the driver doesn't error out, it continues to create the
// spi controller device and spi_board_info for BMC.
//
    if (val == 0) {
    dev_err(dev, "Nios version reg = 0x%x, skip INIT_DONE check, but the retimer may be uninitialized\n",
    val);
    return 0;
    }
    if (FIELD_GET(N3000_NIOS_FW_VERSION_MAJOR, val) >= 3) {
// read NIOS_INIT to check if retimer initialization is done
    ret = regmap_read(nn.regmap, N3000_NIOS_INIT, &val);
    if (ret)
    return ret;
// check if retimers are initialized already
    if (val & (N3000_NIOS_INIT_DONE | N3000_NIOS_INIT_START))
    goto nios_init_done;
// configure FEC mode per module param
    val = N3000_NIOS_INIT_START;
//
// When the retimer is to be set to 10G mode, there is no FEC
// mode setting, so the REQ_FEC_MODE field will be ignored by
// Nios firmware in this case. But we should still fill the FEC
// mode field cause host could not get the retimer working mode
// until the Nios init is done.
//
// For now the driver doesn't support the retimer FEC mode
// switching per user's request. It is always set to Reed
// Solomon FEC.
//
// The driver will set the same FEC mode for all links.
//
    val |= N3000_NIOS_INIT_REQ_FEC_MODE_RS_ALL;
    ret = regmap_write(nn.regmap, N3000_NIOS_INIT, val);
    if (ret)
    return ret;
    }
    nios_init_done:
// polls on NIOS_INIT_DONE
    ret = regmap_read_poll_timeout(nn.regmap, N3000_NIOS_INIT, val,
    val & N3000_NIOS_INIT_DONE,
    N3000_NIOS_INIT_TIME_INTV,
    N3000_NIOS_INIT_TIMEOUT);
    if (ret)
    dev_err(dev, "NIOS_INIT_DONE %s\n",
    (ret == -ETIMEDOUT) ? "timed out" : "check error");
    ret2 = regmap_read(nn.regmap, N3000_NIOS_PKVL_A_MODE_STS, &state_a);
    if (ret2)
    return ret2;
    ret2 = regmap_read(nn.regmap, N3000_NIOS_PKVL_B_MODE_STS, &state_b);
    if (ret2)
    return ret2;
    if (!ret) {
//
// After INIT_DONE is detected, it still needs to check if the
// Nios firmware reports any error during the retimer
// configuration.
//
    if (IS_MODE_STATUS_OK(state_a) && IS_MODE_STATUS_OK(state_b))
    return 0;
//
// If the retimer configuration is failed, the Nios firmware
// will still release the spi controller for host to
// communicate with the BMC. It makes possible for people to
// reprogram a new Nios firmware and restore the card. So the
// driver doesn't error out, it continues to create the spi
// controller device and spi_board_info for BMC.
//
    dev_err(dev, "NIOS_INIT_DONE OK, but err on retimer init\n");
    }
    dev_err(nn.dev, "PKVL_A_MODE_STS 0x%x\n", state_a);
    dev_err(nn.dev, "PKVL_B_MODE_STS 0x%x\n", state_b);
    return ret;
    }
    static struct spi_board_info m10_n3000_info = {
    .modalias = "m10-n3000",
    .max_speed_hz = 12500000,
    .bus_num = 0,
    .chip_select = 0,
    };
#[no_mangle]
unsafe extern "C" fn create_altera_spi_controller(nn: *mut n3000_nios) -> c_int {
    static int create_altera_spi_controller(struct n3000_nios *nn)
    {
    let mut pdata: altera_spi_platform_data = { 0 };
    let mut pdevinfo: platform_device_info = { 0 };
    void __iomem *base = nn.base;
    u64 v;
    v = readq(base + N3000_NS_PARAM);
    pdata.mode_bits = SPI_CS_HIGH;
    if (FIELD_GET(N3000_NS_PARAM_CLK_POL, v))
    pdata.mode_bits |= SPI_CPOL;
    if (FIELD_GET(N3000_NS_PARAM_CLK_PHASE, v))
    pdata.mode_bits |= SPI_CPHA;
    pdata.num_chipselect = FIELD_GET(N3000_NS_PARAM_NUM_CS, v);
    pdata.bits_per_word_mask =
    SPI_BPW_RANGE_MASK(1, FIELD_GET(N3000_NS_PARAM_DATA_WIDTH, v));
    pdata.num_devices = 1;
    pdata.devices = &m10_n3000_info;
    dev_dbg(nn.dev, "%s cs %u bpm 0x%x mode 0x%x\n", __func__,
    pdata.num_chipselect, pdata.bits_per_word_mask,
    pdata.mode_bits);
    pdevinfo.name = "subdev_spi_altera";
    pdevinfo.id = PLATFORM_DEVID_AUTO;
    pdevinfo.parent = nn.dev;
    pdevinfo.data = &pdata;
    pdevinfo.size_data = sizeof(pdata);
    nn.altera_spi = platform_device_register_full(&pdevinfo);
    return PTR_ERR_OR_ZERO(nn.altera_spi);
    }
#[no_mangle]
unsafe extern "C" fn destroy_altera_spi_controller(nn: *mut n3000_nios) {
    static void destroy_altera_spi_controller(struct n3000_nios *nn)
    {
    platform_device_unregister(nn.altera_spi);
    }
#[no_mangle]
unsafe extern "C" fn n3000_nios_poll_stat_timeout(base: *mut void __iomem, v: *mut u64) -> c_int {
    static int n3000_nios_poll_stat_timeout(void __iomem *base, u64 *v)
    {
    int loops;
//
// We don't use the time based timeout here for performance.
//
// The regbus read/write is on the critical path of Intel PAC N3000
// image programming. The time based timeout checking will add too much
// overhead on it. Usually the state changes in 1 or 2 loops on the
// test server, and we set 10000 times loop here for safety.
//
    for (loops = N3000_NIOS_REGBUS_RETRY_COUNT; loops > 0 ; loops--) {
// v = readq(base + N3000_NS_STAT);
    if (*v & N3000_NS_STAT_RW_VAL)
    break;
    cpu_relax();
    }
    return (loops > 0) ? 0 : -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn n3000_nios_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int n3000_nios_reg_write(void *context, unsigned int reg, unsigned int val)
    {
    struct n3000_nios *nn = context;
    u64 v;
    int ret;
    v = FIELD_PREP(N3000_NS_CTRL_CMD_MSK, N3000_NS_CTRL_CMD_WR) |
    FIELD_PREP(N3000_NS_CTRL_ADDR, reg) |
    FIELD_PREP(N3000_NS_CTRL_WR_DATA, val);
    writeq(v, nn.base + N3000_NS_CTRL);
    ret = n3000_nios_poll_stat_timeout(nn.base, &v);
    if (ret)
    dev_err(nn.dev, "fail to write reg 0x%x val 0x%x: %d\n",
    reg, val, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn n3000_nios_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int n3000_nios_reg_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct n3000_nios *nn = context;
    u64 v;
    int ret;
    v = FIELD_PREP(N3000_NS_CTRL_CMD_MSK, N3000_NS_CTRL_CMD_RD) |
    FIELD_PREP(N3000_NS_CTRL_ADDR, reg);
    writeq(v, nn.base + N3000_NS_CTRL);
    ret = n3000_nios_poll_stat_timeout(nn.base, &v);
    if (ret)
    dev_err(nn.dev, "fail to read reg 0x%x: %d\n", reg, ret);
    else
// val = FIELD_GET(N3000_NS_STAT_RD_DATA, v);
    return ret;
    }
    static const struct regmap_config n3000_nios_regbus_cfg = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .fast_io = true,
    .reg_write = n3000_nios_reg_write,
    .reg_read = n3000_nios_reg_read,
    };
#[no_mangle]
unsafe extern "C" fn n3000_nios_probe(ddev: *mut dfl_device) -> c_int {
    static int n3000_nios_probe(struct dfl_device *ddev)
    {
    struct device *dev = &ddev.dev;
    struct n3000_nios *nn;
    int ret;
    nn = devm_kzalloc(dev, sizeof(*nn), GFP_KERNEL);
    if (!nn)
    return -ENOMEM;
    dev_set_drvdata(&ddev.dev, nn);
    nn.dev = dev;
    nn.base = devm_ioremap_resource(&ddev.dev, &ddev.mmio_res);
    if (IS_ERR(nn.base))
    return PTR_ERR(nn.base);
    nn.regmap = devm_regmap_init(dev, core::ptr::null_mut(), nn, &n3000_nios_regbus_cfg);
    if (IS_ERR(nn.regmap))
    return PTR_ERR(nn.regmap);
    ret = n3000_nios_init_done_check(nn);
    if (ret)
    return ret;
    ret = create_altera_spi_controller(nn);
    if (ret)
    dev_err(dev, "altera spi controller create failed: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn n3000_nios_remove(ddev: *mut dfl_device) {
    static void n3000_nios_remove(struct dfl_device *ddev)
    {
    struct n3000_nios *nn = dev_get_drvdata(&ddev.dev);
    destroy_altera_spi_controller(nn);
    }
pub const FME_FEATURE_ID_N3000_NIOS: c_uint = 0xd;
    static const struct dfl_device_id n3000_nios_ids[] = {
    { FME_ID, FME_FEATURE_ID_N3000_NIOS },
    { }
    };
    MODULE_DEVICE_TABLE(dfl, n3000_nios_ids);
    static struct dfl_driver n3000_nios_driver = {
    .drv	= {
    .name       = "dfl-n3000-nios",
    .dev_groups = n3000_nios_groups,
    },
    .id_table = n3000_nios_ids,
    .probe   = n3000_nios_probe,
    .remove  = n3000_nios_remove,
    };
    module_dfl_driver(n3000_nios_driver);
    MODULE_DESCRIPTION("Driver for Nios private feature on Intel PAC N3000");
    MODULE_AUTHOR("Intel Corporation");
    MODULE_LICENSE("GPL v2");
