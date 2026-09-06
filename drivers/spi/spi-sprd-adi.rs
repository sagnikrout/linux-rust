//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-sprd-adi.c
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


//
// Copyright (C) 2017 Spreadtrum Communications Inc.
//
// SPDX-License-Identifier: GPL-2.0
//

// Registers definitions for ADI controller
pub const REG_ADI_CTRL0: c_uint = 0x4;
pub const REG_ADI_CHN_PRIL: c_uint = 0x8;
pub const REG_ADI_CHN_PRIH: c_uint = 0xc;
pub const REG_ADI_INT_EN: c_uint = 0x10;
pub const REG_ADI_INT_RAW: c_uint = 0x14;
pub const REG_ADI_INT_MASK: c_uint = 0x18;
pub const REG_ADI_INT_CLR: c_uint = 0x1c;
pub const REG_ADI_GSSI_CFG0: c_uint = 0x20;
pub const REG_ADI_GSSI_CFG1: c_uint = 0x24;
pub const REG_ADI_RD_CMD: c_uint = 0x28;
pub const REG_ADI_RD_DATA: c_uint = 0x2c;
pub const REG_ADI_ARM_FIFO_STS: c_uint = 0x30;
pub const REG_ADI_STS: c_uint = 0x34;
pub const REG_ADI_EVT_FIFO_STS: c_uint = 0x38;
pub const REG_ADI_ARM_CMD_STS: c_uint = 0x3c;
pub const REG_ADI_CHN_EN: c_uint = 0x40;

pub const REG_ADI_CHN_EN1: c_uint = 0x20c;
// Bits definitions for register REG_ADI_GSSI_CFG0

// Bits definitions for register REG_ADI_RD_DATA

pub const RD_ADDR_SHIFT: c_int = 16;

// Bits definitions for register REG_ADI_ARM_FIFO_STS

//
// ADI slave devices include RTC, ADC, regulator, charger, thermal and so on.
// ADI supports 12/14bit address for r2p0, and additional 17bit for r3p0 or
// later versions. Since bit[1:0] are zero, so the spec describe them as
// 10/12/15bit address mode.
// The 10bit mode supports sigle slave, 12/15bit mode supports 3 slave, the
// high two bits is slave_id.
// The slave devices address offset is 0x8000 for 10/12bit address mode,
// and 0x20000 for 15bit mode.
//

pub const ADI_10BIT_SLAVE_OFFSET: c_uint = 0x8000;

pub const ADI_12BIT_SLAVE_OFFSET: c_uint = 0x8000;

pub const ADI_15BIT_SLAVE_OFFSET: c_uint = 0x20000;
// Timeout (ms) for the trylock of hardware spinlocks
pub const ADI_HWSPINLOCK_TIMEOUT: c_int = 5000;
//
// ADI controller has 50 channels including 2 software channels
// and 48 hardware channels.
//
pub const ADI_HW_CHNS: c_int = 50;
pub const ADI_FIFO_DRAIN_TIMEOUT: c_int = 1000;
pub const ADI_READ_TIMEOUT: c_int = 2000;
//
// Read back address from REG_ADI_RD_DATA bit[30:16] which maps to:
// REG_ADI_RD_CMD bit[14:0] for r2p0
// REG_ADI_RD_CMD bit[16:2] for r3p0
//

pub const RDBACK_ADDR_SHIFT_R3: c_int = 2;
// Registers definitions for PMIC watchdog controller
pub const REG_WDG_LOAD_LOW: c_uint = 0x0;
pub const REG_WDG_LOAD_HIGH: c_uint = 0x4;
pub const REG_WDG_CTRL: c_uint = 0x8;
pub const REG_WDG_LOCK: c_uint = 0x20;
// Bits definitions for register REG_WDG_CTRL

// Bits definitions for register REG_MODULE_EN

// Registers definitions for PMIC
pub const PMIC_RST_STATUS: c_uint = 0xee8;
pub const PMIC_MODULE_EN: c_uint = 0xc08;
pub const PMIC_CLK_EN: c_uint = 0xc18;
pub const PMIC_WDG_BASE: c_uint = 0x80;
// Definition of PMIC reset status register
pub const HWRST_STATUS_SECURITY: c_uint = 0x02;
pub const HWRST_STATUS_RECOVERY: c_uint = 0x20;
pub const HWRST_STATUS_NORMAL: c_uint = 0x40;
pub const HWRST_STATUS_ALARM: c_uint = 0x50;
pub const HWRST_STATUS_SLEEP: c_uint = 0x60;
pub const HWRST_STATUS_FASTBOOT: c_uint = 0x30;
pub const HWRST_STATUS_SPECIAL: c_uint = 0x70;
pub const HWRST_STATUS_PANIC: c_uint = 0x80;
pub const HWRST_STATUS_CFTREBOOT: c_uint = 0x90;
pub const HWRST_STATUS_AUTODLOADER: c_uint = 0xa0;
pub const HWRST_STATUS_IQMODE: c_uint = 0xb0;
pub const HWRST_STATUS_SPRDISK: c_uint = 0xc0;
pub const HWRST_STATUS_FACTORYTEST: c_uint = 0xe0;
pub const HWRST_STATUS_WATCHDOG: c_uint = 0xf0;
// Use default timeout 50 ms that converts to watchdog values

pub const WDG_UNLOCK_KEY: c_uint = 0xe551;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_adi_wdg {
    pub base: u32,
    pub rst_sts: u32,
    pub wdg_en: u32,
    pub wdg_clk: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_adi_data {
    pub slave_offset: u32,
    pub slave_addr_size: u32,
    pub reg): *mut *mut int (read_check)(u32 val, u32,
    pub data): *mut *mut int (restart)(struct sys_off_data,
    pub p): *mut *mut void (wdg_rst)(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_adi {
    pub ctlr: *mut spi_controller,
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub hwlock: *mut hwspinlock,
    pub slave_vbase: c_ulong,
    pub slave_pbase: c_ulong,
    pub data: *const sprd_adi_data,
}

#[no_mangle]
unsafe extern "C" fn sprd_adi_check_addr(sadi: *mut sprd_adi, reg: u32) -> c_int {
    static int sprd_adi_check_addr(struct sprd_adi *sadi, u32 reg)
    {
    if (reg >= sadi.data.slave_addr_size) {
    dev_err(sadi.dev,
    "slave address offset is incorrect, reg = 0x%x\n",
    reg);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_adi_drain_fifo(sadi: *mut sprd_adi) -> c_int {
    static int sprd_adi_drain_fifo(struct sprd_adi *sadi)
    {
    let mut timeout: u32 = ADI_FIFO_DRAIN_TIMEOUT;
    u32 sts;
    do {
    sts = readl_relaxed(sadi.base + REG_ADI_ARM_FIFO_STS);
    if (sts & BIT_FIFO_EMPTY)
    break;
    cpu_relax();
    } while (--timeout);
    if (timeout == 0) {
    dev_err(sadi.dev, "drain write fifo timeout\n");
    return -EBUSY;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_adi_fifo_is_full(sadi: *mut sprd_adi) -> c_int {
    static int sprd_adi_fifo_is_full(struct sprd_adi *sadi)
    {
    return readl_relaxed(sadi.base + REG_ADI_ARM_FIFO_STS) & BIT_FIFO_FULL;
    }
#[no_mangle]
unsafe extern "C" fn sprd_adi_read_check(val: u32, addr: u32) -> c_int {
    static int sprd_adi_read_check(u32 val, u32 addr)
    {
    u32 rd_addr;
    rd_addr = (val & RD_ADDR_MASK) >> RD_ADDR_SHIFT;
    if (rd_addr != addr) {
    pr_err("ADI read error, addr = 0x%x, val = 0x%x\n", addr, val);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_adi_read_check_r2(val: u32, reg: u32) -> c_int {
    static int sprd_adi_read_check_r2(u32 val, u32 reg)
    {
    return sprd_adi_read_check(val, reg & RDBACK_ADDR_MASK_R2);
    }
#[no_mangle]
unsafe extern "C" fn sprd_adi_read_check_r3(val: u32, reg: u32) -> c_int {
    static int sprd_adi_read_check_r3(u32 val, u32 reg)
    {
    return sprd_adi_read_check(val, (reg & RDBACK_ADDR_MASK_R3) >> RDBACK_ADDR_SHIFT_R3);
    }
#[no_mangle]
unsafe extern "C" fn sprd_adi_read(sadi: *mut sprd_adi, reg: u32, read_val: *mut u32) -> c_int {
    static int sprd_adi_read(struct sprd_adi *sadi, u32 reg, u32 *read_val)
    {
    let mut read_timeout: c_int = ADI_READ_TIMEOUT;
    unsigned long flags;
    u32 val;
    let mut ret: c_int = 0;
    if (sadi.hwlock) {
    ret = hwspin_lock_timeout_irqsave(sadi.hwlock,
    ADI_HWSPINLOCK_TIMEOUT,
    &flags);
    if (ret) {
    dev_err(sadi.dev, "get the hw lock failed\n");
    return ret;
    }
    }
    ret = sprd_adi_check_addr(sadi, reg);
    if (ret)
    goto out;
//
// Set the slave address offset need to read into RD_CMD register,
// then ADI controller will start to transfer automatically.
//
    writel_relaxed(reg, sadi.base + REG_ADI_RD_CMD);
//
// Wait read operation complete, the BIT_RD_CMD_BUSY will be set
// simultaneously when writing read command to register, and the
// BIT_RD_CMD_BUSY will be cleared after the read operation is
// completed.
//
    do {
    val = readl_relaxed(sadi.base + REG_ADI_RD_DATA);
    if (!(val & BIT_RD_CMD_BUSY))
    break;
    cpu_relax();
    } while (--read_timeout);
    if (read_timeout == 0) {
    dev_err(sadi.dev, "ADI read timeout\n");
    ret = -EBUSY;
    goto out;
    }
//
// The return value before adi r5p0 includes data and read register
// address, from bit 0to bit 15 are data, and from bit 16 to bit 30
// are read register address. Then we can check the returned register
// address to validate data.
//
    if (sadi.data.read_check) {
    ret = sadi.data.read_check(val, reg);
    if (ret < 0)
    goto out;
    }
// read_val = val & RD_VALUE_MASK;
    out:
    if (sadi.hwlock)
    hwspin_unlock_irqrestore(sadi.hwlock, &flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sprd_adi_write(sadi: *mut sprd_adi, reg: u32, val: u32) -> c_int {
    static int sprd_adi_write(struct sprd_adi *sadi, u32 reg, u32 val)
    {
    let mut timeout: u32 = ADI_FIFO_DRAIN_TIMEOUT;
    unsigned long flags;
    int ret;
    if (sadi.hwlock) {
    ret = hwspin_lock_timeout_irqsave(sadi.hwlock,
    ADI_HWSPINLOCK_TIMEOUT,
    &flags);
    if (ret) {
    dev_err(sadi.dev, "get the hw lock failed\n");
    return ret;
    }
    }
    ret = sprd_adi_check_addr(sadi, reg);
    if (ret)
    goto out;
    ret = sprd_adi_drain_fifo(sadi);
    if (ret < 0)
    goto out;
//
// we should wait for write fifo is empty before writing data to PMIC
// registers.
//
    do {
    if (!sprd_adi_fifo_is_full(sadi)) {
// we need virtual register address to write.
    writel_relaxed(val, (void __iomem *)(sadi.slave_vbase + reg));
    break;
    }
    cpu_relax();
    } while (--timeout);
    if (timeout == 0) {
    dev_err(sadi.dev, "write fifo is full\n");
    ret = -EBUSY;
    }
    out:
    if (sadi.hwlock)
    hwspin_unlock_irqrestore(sadi.hwlock, &flags);
    return ret;
    }
    static int sprd_adi_transfer_one(struct spi_controller *ctlr,
    struct spi_device *spi_dev,
    struct spi_transfer *t)
    {
    struct sprd_adi *sadi = spi_controller_get_devdata(ctlr);
    u32 reg, val;
    int ret;
    if (t.rx_buf) {
    reg = *(u32 *)t.rx_buf;
    ret = sprd_adi_read(sadi, reg, &val);
// (u32 *)t->rx_buf = val;
    } else if (t.tx_buf) {
    u32 *p = (u32 *)t.tx_buf;
    reg = *p++;
    val = *p;
    ret = sprd_adi_write(sadi, reg, val);
    } else {
    dev_err(sadi.dev, "no buffer for transfer\n");
    ret = -EINVAL;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sprd_adi_set_wdt_rst_mode(p: *mut c_void) {
    static void sprd_adi_set_wdt_rst_mode(void *p)
    {

    u32 val;
    struct sprd_adi *sadi = (struct sprd_adi *)p;
// Init watchdog reset mode
    sprd_adi_read(sadi, PMIC_RST_STATUS, &val);
    val |= HWRST_STATUS_WATCHDOG;
    sprd_adi_write(sadi, PMIC_RST_STATUS, val);

    }
    static int sprd_adi_restart(struct sprd_adi *sadi, unsigned long mode,
    const char *cmd, struct sprd_adi_wdg *wdg)
    {
    u32 val, reboot_mode = 0;
    if (!cmd)
    reboot_mode = HWRST_STATUS_NORMAL;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(cmd, _arg: "recovery", _arg: 8)) -> else {
    else if (!strncmp(cmd, "recovery", 8))
    reboot_mode = HWRST_STATUS_RECOVERY;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(cmd, _arg: "alarm", _arg: 5)) -> else {
    else if (!strncmp(cmd, "alarm", 5))
    reboot_mode = HWRST_STATUS_ALARM;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(cmd, _arg: "fastsleep", _arg: 9)) -> else {
    else if (!strncmp(cmd, "fastsleep", 9))
    reboot_mode = HWRST_STATUS_SLEEP;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(cmd, _arg: "bootloader", _arg: 10)) -> else {
    else if (!strncmp(cmd, "bootloader", 10))
    reboot_mode = HWRST_STATUS_FASTBOOT;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(cmd, _arg: "panic", _arg: 5)) -> else {
    else if (!strncmp(cmd, "panic", 5))
    reboot_mode = HWRST_STATUS_PANIC;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(cmd, _arg: "special", _arg: 7)) -> else {
    else if (!strncmp(cmd, "special", 7))
    reboot_mode = HWRST_STATUS_SPECIAL;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(cmd, _arg: "cftreboot", _arg: 9)) -> else {
    else if (!strncmp(cmd, "cftreboot", 9))
    reboot_mode = HWRST_STATUS_CFTREBOOT;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(cmd, _arg: "autodloader", _arg: 11)) -> else {
    else if (!strncmp(cmd, "autodloader", 11))
    reboot_mode = HWRST_STATUS_AUTODLOADER;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(cmd, _arg: "iqmode", _arg: 6)) -> else {
    else if (!strncmp(cmd, "iqmode", 6))
    reboot_mode = HWRST_STATUS_IQMODE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(cmd, _arg: "sprdisk", _arg: 7)) -> else {
    else if (!strncmp(cmd, "sprdisk", 7))
    reboot_mode = HWRST_STATUS_SPRDISK;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(cmd, _arg: "tospanic", _arg: 8)) -> else {
    else if (!strncmp(cmd, "tospanic", 8))
    reboot_mode = HWRST_STATUS_SECURITY;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(cmd, _arg: "factorytest", _arg: 11)) -> else {
    else if (!strncmp(cmd, "factorytest", 11))
    reboot_mode = HWRST_STATUS_FACTORYTEST;
    else
    reboot_mode = HWRST_STATUS_NORMAL;
// Record the reboot mode
    sprd_adi_read(sadi, wdg.rst_sts, &val);
    val &= ~HWRST_STATUS_WATCHDOG;
    val |= reboot_mode;
    sprd_adi_write(sadi, wdg.rst_sts, val);
// Enable the interface clock of the watchdog
    sprd_adi_read(sadi, wdg.wdg_en, &val);
    val |= BIT_WDG_EN;
    sprd_adi_write(sadi, wdg.wdg_en, val);
// Enable the work clock of the watchdog
    sprd_adi_read(sadi, wdg.wdg_clk, &val);
    val |= BIT_WDG_EN;
    sprd_adi_write(sadi, wdg.wdg_clk, val);
// Unlock the watchdog
    sprd_adi_write(sadi, wdg.base + REG_WDG_LOCK, WDG_UNLOCK_KEY);
    sprd_adi_read(sadi, wdg.base + REG_WDG_CTRL, &val);
    val |= BIT_WDG_NEW;
    sprd_adi_write(sadi, wdg.base + REG_WDG_CTRL, val);
// Load the watchdog timeout value, 50ms is always enough.
    sprd_adi_write(sadi, wdg.base + REG_WDG_LOAD_HIGH, 0);
    sprd_adi_write(sadi, wdg.base + REG_WDG_LOAD_LOW,
    WDG_LOAD_VAL & WDG_LOAD_MASK);
// Start the watchdog to reset system
    sprd_adi_read(sadi, wdg.base + REG_WDG_CTRL, &val);
    val |= BIT_WDG_RUN | BIT_WDG_RST;
    sprd_adi_write(sadi, wdg.base + REG_WDG_CTRL, val);
// Lock the watchdog
    sprd_adi_write(sadi, wdg.base + REG_WDG_LOCK, ~WDG_UNLOCK_KEY);
    mdelay(1000);
    dev_emerg(sadi.dev, "Unable to restart system\n");
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn sprd_adi_restart_sc9860(data: *mut sys_off_data) -> c_int {
    static int sprd_adi_restart_sc9860(struct sys_off_data *data)
    {
    struct sprd_adi_wdg wdg = {
    .base = PMIC_WDG_BASE,
    .rst_sts = PMIC_RST_STATUS,
    .wdg_en = PMIC_MODULE_EN,
    .wdg_clk = PMIC_CLK_EN,
    };
    return sprd_adi_restart(data.cb_data, data.mode, data.cmd, &wdg);
    }
#[no_mangle]
unsafe extern "C" fn sprd_adi_hw_init(sadi: *mut sprd_adi) {
    static void sprd_adi_hw_init(struct sprd_adi *sadi)
    {
    struct device_node *np = sadi.dev.of_node;
    int i, size, chn_cnt;
    const __be32 *list;
    u32 tmp;
// Set all channels as default priority
    writel_relaxed(0, sadi.base + REG_ADI_CHN_PRIL);
    writel_relaxed(0, sadi.base + REG_ADI_CHN_PRIH);
// Set clock auto gate mode
    tmp = readl_relaxed(sadi.base + REG_ADI_GSSI_CFG0);
    tmp &= ~BIT_CLK_ALL_ON;
    writel_relaxed(tmp, sadi.base + REG_ADI_GSSI_CFG0);
// Set hardware channels setting
    list = of_get_property(np, "sprd,hw-channels", &size);
    if (!list || !size) {
    dev_info(sadi.dev, "no hw channels setting in node\n");
    return;
    }
    chn_cnt = size / 8;
    for (i = 0; i < chn_cnt; i++) {
    u32 value;
    let mut chn_id: u32 = be32_to_cpu(*list++);
    let mut chn_config: u32 = be32_to_cpu(*list++);
// Channel 0 and 1 are software channels
    if (chn_id < 2)
    continue;
    writel_relaxed(chn_config, sadi.base +
    REG_ADI_CHN_ADDR(chn_id));
    if (chn_id < 32) {
    value = readl_relaxed(sadi.base + REG_ADI_CHN_EN);
    value |= BIT(chn_id);
    writel_relaxed(value, sadi.base + REG_ADI_CHN_EN);
    } else if (chn_id < ADI_HW_CHNS) {
    value = readl_relaxed(sadi.base + REG_ADI_CHN_EN1);
    value |= BIT(chn_id - 32);
    writel_relaxed(value, sadi.base + REG_ADI_CHN_EN1);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn sprd_adi_probe(pdev: *mut platform_device) -> c_int {
    static int sprd_adi_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    const struct sprd_adi_data *data;
    struct spi_controller *ctlr;
    struct sprd_adi *sadi;
    struct resource *res;
    u16 num_chipselect;
    int ret;
    if (!np) {
    dev_err(&pdev.dev, "can not find the adi bus node\n");
    return -ENODEV;
    }
    data = of_device_get_match_data(&pdev.dev);
    if (!data) {
    dev_err(&pdev.dev, "no matching driver data found\n");
    return -EINVAL;
    }
    pdev.id = of_alias_get_id(np, "spi");
    num_chipselect = of_get_child_count(np);
    ctlr = devm_spi_alloc_host(&pdev.dev, sizeof(struct sprd_adi));
    if (!ctlr)
    return -ENOMEM;
    dev_set_drvdata(&pdev.dev, ctlr);
    sadi = spi_controller_get_devdata(ctlr);
    sadi.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(sadi.base))
    return PTR_ERR(sadi.base);
    sadi.slave_vbase = (unsigned long)sadi.base +
    data.slave_offset;
    sadi.slave_pbase = res.start + data.slave_offset;
    sadi.ctlr = ctlr;
    sadi.dev = &pdev.dev;
    sadi.data = data;
    ret = of_hwspin_lock_get_id(np, 0);
    if (ret > 0 || (IS_ENABLED(CONFIG_HWSPINLOCK) && ret == 0)) {
    sadi.hwlock =
    devm_hwspin_lock_request_specific(&pdev.dev, ret);
    if (!sadi.hwlock)
    return -ENXIO;
    } else {
    switch (ret) {
    case 0:
//
// Only reachable with CONFIG_HWSPINLOCK=n, where the
// of_hwspin_lock_get_id() stub returns 0.
//
    fallthrough;
    case -ENOENT:
    dev_info(&pdev.dev, "no hardware spinlock supplied\n");
    break;
    default:
    return dev_err_probe(&pdev.dev, ret, "failed to find hwlock id\n");
    }
    }
    sprd_adi_hw_init(sadi);
    if (sadi.data.wdg_rst)
    sadi.data.wdg_rst(sadi);
    ctlr.bus_num = pdev.id;
    ctlr.num_chipselect = num_chipselect;
    ctlr.flags = SPI_CONTROLLER_HALF_DUPLEX;
    ctlr.bits_per_word_mask = 0;
    ctlr.transfer_one = sprd_adi_transfer_one;
    ret = devm_spi_register_controller(&pdev.dev, ctlr);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "failed to register SPI controller\n");
    if (sadi.data.restart) {
    ret = devm_register_restart_handler(&pdev.dev,
    sadi.data.restart,
    sadi);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "can not register restart handler\n");
    }
    return 0;
    }
    static struct sprd_adi_data sc9860_data = {
    .slave_offset = ADI_10BIT_SLAVE_OFFSET,
    .slave_addr_size = ADI_10BIT_SLAVE_ADDR_SIZE,
    .read_check = sprd_adi_read_check_r2,
    .restart = sprd_adi_restart_sc9860,
    .wdg_rst = sprd_adi_set_wdt_rst_mode,
    };
    static struct sprd_adi_data sc9863_data = {
    .slave_offset = ADI_12BIT_SLAVE_OFFSET,
    .slave_addr_size = ADI_12BIT_SLAVE_ADDR_SIZE,
    .read_check = sprd_adi_read_check_r3,
    };
    static struct sprd_adi_data ums512_data = {
    .slave_offset = ADI_15BIT_SLAVE_OFFSET,
    .slave_addr_size = ADI_15BIT_SLAVE_ADDR_SIZE,
    .read_check = sprd_adi_read_check_r3,
    };
    static const struct of_device_id sprd_adi_of_match[] = {
    {
    .compatible = "sprd,sc9860-adi",
    .data = &sc9860_data,
    },
    {
    .compatible = "sprd,sc9863-adi",
    .data = &sc9863_data,
    },
    {
    .compatible = "sprd,ums512-adi",
    .data = &ums512_data,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, sprd_adi_of_match);
    static struct platform_driver sprd_adi_driver = {
    .driver = {
    .name = "sprd-adi",
    .of_match_table = sprd_adi_of_match,
    },
    .probe = sprd_adi_probe,
    };
    module_platform_driver(sprd_adi_driver);
    MODULE_DESCRIPTION("Spreadtrum ADI Controller Driver");
    MODULE_AUTHOR("Baolin Wang <Baolin.Wang@spreadtrum.com>");
    MODULE_LICENSE("GPL v2");
