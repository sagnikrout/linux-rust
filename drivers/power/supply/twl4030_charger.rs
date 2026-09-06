//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/twl4030_charger.c
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
// TWL4030/TPS65950 BCI (Battery Charger Interface) driver
//
// Copyright (C) 2010 Gražvydas Ignotas <notasas@gmail.com>
//
// based on twl4030_bci_battery.c by TI
// Copyright (C) 2008 Texas Instruments, Inc.
//

pub const TWL4030_BCIMDEN: c_uint = 0x00;
pub const TWL4030_BCIMDKEY: c_uint = 0x01;
pub const TWL4030_BCIMSTATEC: c_uint = 0x02;
pub const TWL4030_BCIICHG: c_uint = 0x08;
pub const TWL4030_BCIVAC: c_uint = 0x0a;
pub const TWL4030_BCIVBUS: c_uint = 0x0c;
pub const TWL4030_BCIMFSTS3: c_uint = 0x0F;
pub const TWL4030_BCIMFSTS4: c_uint = 0x10;
pub const TWL4030_BCICTL1: c_uint = 0x23;
pub const TWL4030_BB_CFG: c_uint = 0x12;
pub const TWL4030_BCIIREF1: c_uint = 0x27;
pub const TWL4030_BCIIREF2: c_uint = 0x28;
pub const TWL4030_BCIMFKEY: c_uint = 0x11;
pub const TWL4030_BCIMFEN3: c_uint = 0x14;
pub const TWL4030_BCIMFTH8: c_uint = 0x1d;
pub const TWL4030_BCIMFTH9: c_uint = 0x1e;
pub const TWL4030_BCIWDKEY: c_uint = 0x21;
pub const TWL4030_BCIMFSTS1: c_uint = 0x01;

pub const TWL4030_BBSEL_MASK: c_uint = 0x0c;
pub const TWL4030_BBSEL_2V5: c_uint = 0x00;
pub const TWL4030_BBSEL_3V0: c_uint = 0x04;
pub const TWL4030_BBSEL_3V1: c_uint = 0x08;
pub const TWL4030_BBSEL_3V2: c_uint = 0x0c;
pub const TWL4030_BBISEL_MASK: c_uint = 0x03;
pub const TWL4030_BBISEL_25uA: c_uint = 0x00;
pub const TWL4030_BBISEL_150uA: c_uint = 0x01;
pub const TWL4030_BBISEL_500uA: c_uint = 0x02;
pub const TWL4030_BBISEL_1000uA: c_uint = 0x03;

// BCI interrupts

pub const TWL4030_MSTATEC_MASK: c_uint = 0x0f;
pub const TWL4030_MSTATEC_QUICK1: c_uint = 0x02;
pub const TWL4030_MSTATEC_QUICK7: c_uint = 0x07;
pub const TWL4030_MSTATEC_COMPLETE1: c_uint = 0x0b;
pub const TWL4030_MSTATEC_COMPLETE4: c_uint = 0x0e;
//
// If AC (Accessory Charger) voltage exceeds 4.5V (MADC 11)
// then AC is available.
//
#[no_mangle]
pub unsafe extern "C" fn ac_available(channel_vac: *mut iio_channel) -> c_int {
    static inline int ac_available(struct iio_channel *channel_vac)
    {
    int val, err;
    if (!channel_vac)
    return 0;
    err = iio_read_channel_processed(channel_vac, &val);
    if (err < 0)
    return 0;
    return val > 4500;
    }
    static bool allow_usb;
    module_param(allow_usb, bool, 0644);
    MODULE_PARM_DESC(allow_usb, "Allow USB charge drawing default current");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_bci {
    pub dev: *mut device,
    pub ac: *mut power_supply,
    pub usb: *mut power_supply,
    pub transceiver: *mut usb_phy,
    pub usb_nb: notifier_block,
    pub work: work_struct,
    pub irq_chg: c_int,
    pub irq_bci: c_int,
    pub usb_enabled: c_int,
//
// ichg_* and *_cur values in uA. If any are 'large', we set
// CGAIN to '1' which doubles the range for half the
// precision.
//
    pub ichg_hi: unsigned int ichg_eoc, ichg_lo,,
    pub ac_cur: unsigned int usb_cur,,
    pub channel_vac: *mut iio_channel,
    pub ac_is_active: bool,
    pub /: *mut *mut int usb_mode, ac_mode; / charging mode requested,
pub const CHARGE_OFF: c_int = 0;
pub const CHARGE_AUTO: c_int = 1;
pub const CHARGE_LINEAR: c_int = 2;
// When setting the USB current we slowly increase the
// requested current until target is reached or the voltage
// drops below 4.75V.  In the latter case we step back one
// step.
//
    pub usb_cur_target: c_uint,
    pub current_worker: delayed_work,

    pub event: c_ulong,
}

// strings for 'usb_mode' values
    static const char *modes[] = { "off", "auto", "continuous" };
//
// clear and set bits on an given register on a given module
//
#[no_mangle]
unsafe extern "C" fn twl4030_clear_set(mod_no: u8, clear: u8, set: u8, reg: u8) -> c_int {
    static int twl4030_clear_set(u8 mod_no, u8 clear, u8 set, u8 reg)
    {
    let mut val: u8 = 0;
    int ret;
    ret = twl_i2c_read_u8(mod_no, &val, reg);
    if (ret)
    return ret;
    val &= ~clear;
    val |= set;
    return twl_i2c_write_u8(mod_no, val, reg);
    }
#[no_mangle]
unsafe extern "C" fn twl4030_bci_read(reg: u8, val: *mut u8) -> c_int {
    static int twl4030_bci_read(u8 reg, u8 *val)
    {
    return twl_i2c_read_u8(TWL_MODULE_MAIN_CHARGE, val, reg);
    }
#[no_mangle]
unsafe extern "C" fn twl4030_clear_set_boot_bci(clear: u8, set: u8) -> c_int {
    static int twl4030_clear_set_boot_bci(u8 clear, u8 set)
    {
    return twl4030_clear_set(TWL_MODULE_PM_MASTER, clear,
    TWL4030_CONFIG_DONE | TWL4030_BCIAUTOWEN | set,
    TWL4030_PM_MASTER_BOOT_BCI);
    }
#[no_mangle]
unsafe extern "C" fn twl4030bci_read_adc_val(reg: u8) -> c_int {
    static int twl4030bci_read_adc_val(u8 reg)
    {
    int ret, temp;
    u8 val;
// read MSB
    ret = twl4030_bci_read(reg + 1, &val);
    if (ret)
    return ret;
    temp = (int)(val & 0x03) << 8;
// read LSB
    ret = twl4030_bci_read(reg, &val);
    if (ret)
    return ret;
    return temp | val;
    }
//
// TI provided formulas:
// CGAIN == 0: ICHG = (BCIICHG * 1.7) / (2^10 - 1) - 0.85
// CGAIN == 1: ICHG = (BCIICHG * 3.4) / (2^10 - 1) - 1.7
// Here we use integer approximation of:
// CGAIN == 0: val * 1.6618 - 0.85 * 1000
// CGAIN == 1: (val * 1.6618 - 0.85 * 1000) * 2
//
// convert twl register value for currents into uA
//
#[no_mangle]
unsafe extern "C" fn regval2ua(regval: c_int, cgain: bool) -> c_int {
    static int regval2ua(int regval, bool cgain)
    {
    if (cgain)
    return (regval * 16618 - 8500 * 1000) / 5;
    else
    return (regval * 16618 - 8500 * 1000) / 10;
    }
//
// convert uA currents into twl register value
//
#[no_mangle]
unsafe extern "C" fn ua2regval(ua: c_int, cgain: bool) -> c_int {
    static int ua2regval(int ua, bool cgain)
    {
    int ret;
    if (cgain)
    ua /= 2;
    ret = (ua * 10 + 8500 * 1000) / 16618;
// rounding problems
    if (ret < 512)
    ret = 512;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_charger_update_current(bci: *mut twl4030_bci) -> c_int {
    static int twl4030_charger_update_current(struct twl4030_bci *bci)
    {
    int status;
    int cur;
    unsigned reg, cur_reg;
    u8 bcictl1, oldreg, fullreg;
    let mut cgain: bool = false;
    u8 boot_bci;
//
// If AC (Accessory Charger) voltage exceeds 4.5V (MADC 11)
// and AC is enabled, set current for 'ac'
//
    if (ac_available(bci.channel_vac)) {
    cur = bci.ac_cur;
    bci.ac_is_active = true;
    } else {
    cur = bci.usb_cur;
    bci.ac_is_active = false;
    if (cur > bci.usb_cur_target) {
    cur = bci.usb_cur_target;
    bci.usb_cur = cur;
    }
    if (cur < bci.usb_cur_target)
    schedule_delayed_work(&bci.current_worker, USB_CUR_DELAY);
    }
// First, check thresholds and see if cgain is needed
    if (bci.ichg_eoc >= 200000)
    cgain = true;
    if (bci.ichg_lo >= 400000)
    cgain = true;
    if (bci.ichg_hi >= 820000)
    cgain = true;
    if (cur > 852000)
    cgain = true;
    status = twl4030_bci_read(TWL4030_BCICTL1, &bcictl1);
    if (status < 0)
    return status;
    if (twl_i2c_read_u8(TWL_MODULE_PM_MASTER, &boot_bci,
    TWL4030_PM_MASTER_BOOT_BCI) < 0)
    boot_bci = 0;
    boot_bci &= 7;
    if ((!!cgain) != !!(bcictl1 & TWL4030_CGAIN))
// Need to turn for charging while we change the
// CGAIN bit.  Leave it off while everything is
// updated.
//
    twl4030_clear_set_boot_bci(boot_bci, 0);
//
// For ichg_eoc, the hardware only supports reg values matching
// 100XXXX000, and requires the XXXX be stored in the high nibble
// of TWL4030_BCIMFTH8.
//
    reg = ua2regval(bci.ichg_eoc, cgain);
    if (reg > 0x278)
    reg = 0x278;
    if (reg < 0x200)
    reg = 0x200;
    reg = (reg >> 3) & 0xf;
    fullreg = reg << 4;
//
// For ichg_lo, reg value must match 10XXXX0000.
// XXXX is stored in low nibble of TWL4030_BCIMFTH8.
//
    reg = ua2regval(bci.ichg_lo, cgain);
    if (reg > 0x2F0)
    reg = 0x2F0;
    if (reg < 0x200)
    reg = 0x200;
    reg = (reg >> 4) & 0xf;
    fullreg |= reg;
// ichg_eoc and ichg_lo live in same register
    status = twl4030_bci_read(TWL4030_BCIMFTH8, &oldreg);
    if (status < 0)
    return status;
    if (oldreg != fullreg) {
    status = twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE, 0xF4,
    TWL4030_BCIMFKEY);
    if (status < 0)
    return status;
    twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE,
    fullreg, TWL4030_BCIMFTH8);
    }
// ichg_hi threshold must be 1XXXX01100 (I think)
    reg = ua2regval(bci.ichg_hi, cgain);
    if (reg > 0x3E0)
    reg = 0x3E0;
    if (reg < 0x200)
    reg = 0x200;
    fullreg = (reg >> 5) & 0xF;
    fullreg <<= 4;
    status = twl4030_bci_read(TWL4030_BCIMFTH9, &oldreg);
    if (status < 0)
    return status;
    if ((oldreg & 0xF0) != fullreg) {
    fullreg |= (oldreg & 0x0F);
    status = twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE, 0xE7,
    TWL4030_BCIMFKEY);
    if (status < 0)
    return status;
    twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE,
    fullreg, TWL4030_BCIMFTH9);
    }
//
// And finally, set the current.  This is stored in
// two registers.
//
    reg = ua2regval(cur, cgain);
// we have only 10 bits
    if (reg > 0x3ff)
    reg = 0x3ff;
    status = twl4030_bci_read(TWL4030_BCIIREF1, &oldreg);
    if (status < 0)
    return status;
    cur_reg = oldreg;
    status = twl4030_bci_read(TWL4030_BCIIREF2, &oldreg);
    if (status < 0)
    return status;
    cur_reg |= oldreg << 8;
    if (reg != cur_reg) {
// disable write protection for one write access for
// BCIIREF
    status = twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE, 0xE7,
    TWL4030_BCIMFKEY);
    if (status < 0)
    return status;
    status = twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE,
    (reg & 0x100) ? 3 : 2,
    TWL4030_BCIIREF2);
    if (status < 0)
    return status;
// disable write protection for one write access for
// BCIIREF
    status = twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE, 0xE7,
    TWL4030_BCIMFKEY);
    if (status < 0)
    return status;
    status = twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE,
    reg & 0xff,
    TWL4030_BCIIREF1);
    }
    if ((!!cgain) != !!(bcictl1 & TWL4030_CGAIN)) {
// Flip CGAIN and re-enable charging
    bcictl1 ^= TWL4030_CGAIN;
    twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE,
    bcictl1, TWL4030_BCICTL1);
    twl4030_clear_set_boot_bci(0, boot_bci);
    }
    return 0;
    }
    static int twl4030_charger_get_current(void);
#[no_mangle]
unsafe extern "C" fn twl4030_current_worker(data: *mut work_struct) {
    static void twl4030_current_worker(struct work_struct *data)
    {
    int v, curr;
    int res;
    struct twl4030_bci *bci = container_of(data, struct twl4030_bci,
    current_worker.work);
    res = twl4030bci_read_adc_val(TWL4030_BCIVBUS);
    if (res < 0)
    v = 0;
    else
// BCIVBUS uses ADCIN8, 7/1023 V/step
    v = res * 6843;
    curr = twl4030_charger_get_current();
    dev_dbg(bci.dev, "v=%d cur=%d limit=%d target=%d\n", v, curr,
    bci.usb_cur, bci.usb_cur_target);
    if (v < USB_MIN_VOLT) {
// Back up and stop adjusting.
    if (bci.usb_cur >= USB_CUR_STEP)
    bci.usb_cur -= USB_CUR_STEP;
    bci.usb_cur_target = bci.usb_cur;
    } else if (bci.usb_cur >= bci.usb_cur_target ||
    bci.usb_cur + USB_CUR_STEP > USB_MAX_CURRENT) {
// Reached target and voltage is OK - stop
    return;
    } else {
    bci.usb_cur += USB_CUR_STEP;
    schedule_delayed_work(&bci.current_worker, USB_CUR_DELAY);
    }
    twl4030_charger_update_current(bci);
    }
//
// Enable/Disable USB Charge functionality.
//
#[no_mangle]
unsafe extern "C" fn twl4030_charger_enable_usb(bci: *mut twl4030_bci, enable: bool) -> c_int {
    static int twl4030_charger_enable_usb(struct twl4030_bci *bci, bool enable)
    {
    int ret;
    u32 reg;
    if (bci.usb_mode == CHARGE_OFF)
    enable = false;
    if (enable && !IS_ERR_OR_NULL(bci.transceiver)) {
    twl4030_charger_update_current(bci);
// Need to keep phy powered
    if (!bci.usb_enabled) {
    pm_runtime_get_sync(bci.transceiver.dev);
    bci.usb_enabled = 1;
    }
    if (bci.usb_mode == CHARGE_AUTO) {
// Enable interrupts now.
    reg = ~(u32)(TWL4030_ICHGLOW | TWL4030_ICHGEOC |
    TWL4030_TBATOR2 | TWL4030_TBATOR1 |
    TWL4030_BATSTS);
    ret = twl_i2c_write_u8(TWL4030_MODULE_INTERRUPTS, reg,
    TWL4030_INTERRUPTS_BCIIMR1A);
    if (ret < 0) {
    dev_err(bci.dev,
    "failed to unmask interrupts: %d\n",
    ret);
    return ret;
    }
// forcing the field BCIAUTOUSB (BOOT_BCI[1]) to 1
    ret = twl4030_clear_set_boot_bci(0, TWL4030_BCIAUTOUSB);
    }
// forcing USBFASTMCHG(BCIMFSTS4[2]) to 1
    ret = twl4030_clear_set(TWL_MODULE_MAIN_CHARGE, 0,
    TWL4030_USBFASTMCHG, TWL4030_BCIMFSTS4);
    if (bci.usb_mode == CHARGE_LINEAR) {
// Enable interrupts now.
    reg = ~(u32)(TWL4030_ICHGLOW | TWL4030_TBATOR2 |
    TWL4030_TBATOR1 | TWL4030_BATSTS);
    ret = twl_i2c_write_u8(TWL4030_MODULE_INTERRUPTS, reg,
    TWL4030_INTERRUPTS_BCIIMR1A);
    if (ret < 0) {
    dev_err(bci.dev,
    "failed to unmask interrupts: %d\n",
    ret);
    return ret;
    }
    twl4030_clear_set_boot_bci(TWL4030_BCIAUTOAC|TWL4030_CVENAC, 0);
// Watch dog key: WOVF acknowledge
    ret = twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE, 0x33,
    TWL4030_BCIWDKEY);
// 0x24 + EKEY6: off mode
    ret = twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE, 0x2a,
    TWL4030_BCIMDKEY);
// EKEY2: Linear charge: USB path
    ret = twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE, 0x26,
    TWL4030_BCIMDKEY);
// WDKEY5: stop watchdog count
    ret = twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE, 0xf3,
    TWL4030_BCIWDKEY);
// enable MFEN3 access
    ret = twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE, 0x9c,
    TWL4030_BCIMFKEY);
// ICHGEOCEN - end-of-charge monitor (current < 80mA)
// (charging continues)
// ICHGLOWEN - current level monitor (charge continues)
// don't monitor over-current or heat save
//
    ret = twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE, 0xf0,
    TWL4030_BCIMFEN3);
    }
    } else {
    ret = twl4030_clear_set_boot_bci(TWL4030_BCIAUTOUSB, 0);
    ret |= twl_i2c_write_u8(TWL_MODULE_MAIN_CHARGE, 0x2a,
    TWL4030_BCIMDKEY);
    if (bci.usb_enabled) {
    pm_runtime_put_autosuspend(bci.transceiver.dev);
    bci.usb_enabled = 0;
    }
    bci.usb_cur = 0;
    }
    return ret;
    }
//
// Enable/Disable AC Charge funtionality.
//
#[no_mangle]
unsafe extern "C" fn twl4030_charger_enable_ac(bci: *mut twl4030_bci, enable: bool) -> c_int {
    static int twl4030_charger_enable_ac(struct twl4030_bci *bci, bool enable)
    {
    int ret;
    if (bci.ac_mode == CHARGE_OFF)
    enable = false;
    if (enable)
    ret = twl4030_clear_set_boot_bci(0, TWL4030_BCIAUTOAC);
    else
    ret = twl4030_clear_set_boot_bci(TWL4030_BCIAUTOAC, 0);
    return ret;
    }
//
// Enable/Disable charging of Backup Battery.
//
#[no_mangle]
unsafe extern "C" fn twl4030_charger_enable_backup(uvolt: c_int, uamp: c_int) -> c_int {
    static int twl4030_charger_enable_backup(int uvolt, int uamp)
    {
    int ret;
    u8 flags;
    if (uvolt < 2500000 ||
    uamp < 25) {
// disable charging of backup battery
    ret = twl4030_clear_set(TWL_MODULE_PM_RECEIVER,
    TWL4030_BBCHEN, 0, TWL4030_BB_CFG);
    return ret;
    }
    flags = TWL4030_BBCHEN;
    if (uvolt >= 3200000)
    flags |= TWL4030_BBSEL_3V2;
#[no_mangle]
pub unsafe extern "C" fn if(3100000: uvolt >=) -> else {
    else if (uvolt >= 3100000)
    flags |= TWL4030_BBSEL_3V1;
#[no_mangle]
pub unsafe extern "C" fn if(3000000: uvolt >=) -> else {
    else if (uvolt >= 3000000)
    flags |= TWL4030_BBSEL_3V0;
    else
    flags |= TWL4030_BBSEL_2V5;
    if (uamp >= 1000)
    flags |= TWL4030_BBISEL_1000uA;
#[no_mangle]
pub unsafe extern "C" fn if(500: uamp >=) -> else {
    else if (uamp >= 500)
    flags |= TWL4030_BBISEL_500uA;
#[no_mangle]
pub unsafe extern "C" fn if(150: uamp >=) -> else {
    else if (uamp >= 150)
    flags |= TWL4030_BBISEL_150uA;
    else
    flags |= TWL4030_BBISEL_25uA;
    ret = twl4030_clear_set(TWL_MODULE_PM_RECEIVER,
    TWL4030_BBSEL_MASK | TWL4030_BBISEL_MASK,
    flags,
    TWL4030_BB_CFG);
    return ret;
    }
//
// TWL4030 CHG_PRES (AC charger presence) events
//
#[no_mangle]
unsafe extern "C" fn twl4030_charger_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t twl4030_charger_interrupt(int irq, void *arg)
    {
    struct twl4030_bci *bci = arg;
    dev_dbg(bci.dev, "CHG_PRES irq\n");
// reset current on each 'plug' event
    bci.ac_cur = 500000;
    twl4030_charger_update_current(bci);
    power_supply_changed(bci.ac);
    power_supply_changed(bci.usb);
    return IRQ_HANDLED;
    }
//
// TWL4030 BCI monitoring events
//
#[no_mangle]
unsafe extern "C" fn twl4030_bci_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t twl4030_bci_interrupt(int irq, void *arg)
    {
    struct twl4030_bci *bci = arg;
    u8 irqs1, irqs2;
    int ret;
    ret = twl_i2c_read_u8(TWL4030_MODULE_INTERRUPTS, &irqs1,
    TWL4030_INTERRUPTS_BCIISR1A);
    if (ret < 0)
    return IRQ_HANDLED;
    ret = twl_i2c_read_u8(TWL4030_MODULE_INTERRUPTS, &irqs2,
    TWL4030_INTERRUPTS_BCIISR2A);
    if (ret < 0)
    return IRQ_HANDLED;
    dev_dbg(bci.dev, "BCI irq %02x %02x\n", irqs2, irqs1);
    if (irqs1 & (TWL4030_ICHGLOW | TWL4030_ICHGEOC)) {
// charger state change, inform the core
    power_supply_changed(bci.ac);
    power_supply_changed(bci.usb);
    }
    twl4030_charger_update_current(bci);
// various monitoring events, for now we just log them here
    if (irqs1 & (TWL4030_TBATOR2 | TWL4030_TBATOR1))
    dev_warn(bci.dev, "battery temperature out of range\n");
    if (irqs1 & TWL4030_BATSTS)
    dev_crit(bci.dev, "battery disconnected\n");
    if (irqs2 & TWL4030_VBATOV)
    dev_crit(bci.dev, "VBAT overvoltage\n");
    if (irqs2 & TWL4030_VBUSOV)
    dev_crit(bci.dev, "VBUS overvoltage\n");
    if (irqs2 & TWL4030_ACCHGOV)
    dev_crit(bci.dev, "Ac charger overvoltage\n");
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_bci_usb_work(data: *mut work_struct) {
    static void twl4030_bci_usb_work(struct work_struct *data)
    {
    struct twl4030_bci *bci = container_of(data, struct twl4030_bci, work);
    switch (bci.event) {
    case USB_EVENT_VBUS:
    case USB_EVENT_CHARGER:
    twl4030_charger_enable_usb(bci, true);
    break;
    case USB_EVENT_NONE:
    twl4030_charger_enable_usb(bci, false);
    break;
    }
    }
    static int twl4030_bci_usb_ncb(struct notifier_block *nb, unsigned long val,
    void *priv)
    {
    struct twl4030_bci *bci = container_of(nb, struct twl4030_bci, usb_nb);
    dev_dbg(bci.dev, "OTG notify %lu\n", val);
// reset current on each 'plug' event
    if (allow_usb)
    bci.usb_cur_target = 500000;
    else
    bci.usb_cur_target = 100000;
    bci.event = val;
    schedule_work(&bci.work);
    return NOTIFY_OK;
    }
//
// sysfs charger enabled store
//
    static ssize_t
    twl4030_bci_mode_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t n)
    {
    struct twl4030_bci *bci = dev_get_drvdata(dev.parent);
    int mode;
    int status;
    mode = sysfs_match_string(modes, buf);
    if (mode < 0)
    return mode;
    if (dev == &bci.ac.dev) {
    if (mode == 2)
    return -EINVAL;
    twl4030_charger_enable_ac(bci, false);
    bci.ac_mode = mode;
    status = twl4030_charger_enable_ac(bci, true);
    } else {
    twl4030_charger_enable_usb(bci, false);
    bci.usb_mode = mode;
    status = twl4030_charger_enable_usb(bci, true);
    }
    return (status == 0) ? n : status;
    }
//
// sysfs charger enabled show
//
    static ssize_t
    twl4030_bci_mode_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct twl4030_bci *bci = dev_get_drvdata(dev.parent);
    let mut len: c_int = 0;
    int i;
    let mut mode: c_int = bci.usb_mode;
    if (dev == &bci.ac.dev)
    mode = bci.ac_mode;
    for (i = 0; i < ARRAY_SIZE(modes); i++)
    if (mode == i)
    len += sysfs_emit_at(buf, len, "[%s] ", modes[i]);
    else
    len += sysfs_emit_at(buf, len, "%s ", modes[i]);
    buf[len-1] = '\n';
    return len;
    }
    static DEVICE_ATTR(mode, 0644, twl4030_bci_mode_show,
    twl4030_bci_mode_store);
#[no_mangle]
unsafe extern "C" fn twl4030_charger_get_current() -> c_int {
    static int twl4030_charger_get_current(void)
    {
    int curr;
    int ret;
    u8 bcictl1;
    curr = twl4030bci_read_adc_val(TWL4030_BCIICHG);
    if (curr < 0)
    return curr;
    ret = twl4030_bci_read(TWL4030_BCICTL1, &bcictl1);
    if (ret)
    return ret;
    return regval2ua(curr, bcictl1 & TWL4030_CGAIN);
    }
//
// Returns the main charge FSM state
// Or < 0 on failure.
//
#[no_mangle]
unsafe extern "C" fn twl4030bci_state(bci: *mut twl4030_bci) -> c_int {
    static int twl4030bci_state(struct twl4030_bci *bci)
    {
    int ret;
    u8 state;
    ret = twl4030_bci_read(TWL4030_BCIMSTATEC, &state);
    if (ret) {
    dev_err(bci.dev, "error reading BCIMSTATEC\n");
    return ret;
    }
    dev_dbg(bci.dev, "state: %02x\n", state);
    return state;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_bci_state_to_status(state: c_int) -> c_int {
    static int twl4030_bci_state_to_status(int state)
    {
    state &= TWL4030_MSTATEC_MASK;
    if (TWL4030_MSTATEC_QUICK1 <= state && state <= TWL4030_MSTATEC_QUICK7)
    return POWER_SUPPLY_STATUS_CHARGING;
    else if (TWL4030_MSTATEC_COMPLETE1 <= state &&
    state <= TWL4030_MSTATEC_COMPLETE4)
    return POWER_SUPPLY_STATUS_FULL;
    else
    return POWER_SUPPLY_STATUS_NOT_CHARGING;
    }
    static int twl4030_bci_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct twl4030_bci *bci = dev_get_drvdata(psy.dev.parent);
    int is_charging;
    int state;
    int ret;
    state = twl4030bci_state(bci);
    if (state < 0)
    return state;
    if (psy.desc.type == POWER_SUPPLY_TYPE_USB)
    is_charging = state & TWL4030_MSTATEC_USB;
    else
    is_charging = state & TWL4030_MSTATEC_AC;
    if (!is_charging) {
    u8 s;
    ret = twl4030_bci_read(TWL4030_BCIMDEN, &s);
    if (ret < 0)
    return ret;
    if (psy.desc.type == POWER_SUPPLY_TYPE_USB)
    is_charging = s & 1;
    else
    is_charging = s & 2;
    if (is_charging)
// A little white lie
    state = TWL4030_MSTATEC_QUICK1;
    }
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    if (is_charging)
    val.intval = twl4030_bci_state_to_status(state);
    else
    val.intval = POWER_SUPPLY_STATUS_NOT_CHARGING;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
// charging must be active for meaningful result
    if (!is_charging)
    return -ENODATA;
    if (psy.desc.type == POWER_SUPPLY_TYPE_USB) {
    ret = twl4030bci_read_adc_val(TWL4030_BCIVBUS);
    if (ret < 0)
    return ret;
// BCIVBUS uses ADCIN8, 7/1023 V/step
    val.intval = ret * 6843;
    } else {
    ret = twl4030bci_read_adc_val(TWL4030_BCIVAC);
    if (ret < 0)
    return ret;
// BCIVAC uses ADCIN11, 10/1023 V/step
    val.intval = ret * 9775;
    }
    break;
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    if (!is_charging)
    return -ENODATA;
// current measurement is shared between AC and USB
    ret = twl4030_charger_get_current();
    if (ret < 0)
    return ret;
    val.intval = ret;
    break;
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = is_charging &&
    twl4030_bci_state_to_status(state) !=
    POWER_SUPPLY_STATUS_NOT_CHARGING;
    break;
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    val.intval = -1;
    if (psy.desc.type != POWER_SUPPLY_TYPE_USB) {
    if (!bci.ac_is_active)
    val.intval = bci.ac_cur;
    } else {
    if (bci.ac_is_active)
    val.intval = bci.usb_cur_target;
    }
    if (val.intval < 0) {
    u8 bcictl1;
    val.intval = twl4030bci_read_adc_val(TWL4030_BCIIREF1);
    if (val.intval < 0)
    return val.intval;
    ret = twl4030_bci_read(TWL4030_BCICTL1, &bcictl1);
    if (ret < 0)
    return ret;
    val.intval = regval2ua(val.intval, bcictl1 &
    TWL4030_CGAIN);
    }
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int twl4030_bci_set_property(struct power_supply *psy,
    enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    struct twl4030_bci *bci = dev_get_drvdata(psy.dev.parent);
    switch (psp) {
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    if (psy.desc.type == POWER_SUPPLY_TYPE_USB)
    bci.usb_cur_target = val.intval;
    else
    bci.ac_cur = val.intval;
    twl4030_charger_update_current(bci);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int twl4030_bci_property_is_writeable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    switch (psp) {
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    return true;
    default:
    return false;
    }
    }
    static enum power_supply_property twl4030_charger_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT,
    };

    static const struct twl4030_bci_platform_data *
    twl4030_bci_parse_dt(struct device *dev)
    {
    struct device_node *np = dev.of_node;
    struct twl4030_bci_platform_data *pdata;
    u32 num;
    if (!np)
    return core::ptr::null_mut();
    pdata = devm_kzalloc(dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return pdata;
    if (of_property_read_u32(np, "ti,bb-uvolt", &num) == 0)
    pdata.bb_uvolt = num;
    if (of_property_read_u32(np, "ti,bb-uamp", &num) == 0)
    pdata.bb_uamp = num;
    return pdata;
    }

    static inline const struct twl4030_bci_platform_data *
    twl4030_bci_parse_dt(struct device *dev)
    {
    return core::ptr::null_mut();
    }

    static const struct power_supply_desc twl4030_bci_ac_desc = {
    .name		= "twl4030_ac",
    .type		= POWER_SUPPLY_TYPE_MAINS,
    .properties	= twl4030_charger_props,
    .num_properties	= ARRAY_SIZE(twl4030_charger_props),
    .get_property	= twl4030_bci_get_property,
    .set_property	= twl4030_bci_set_property,
    .property_is_writeable	= twl4030_bci_property_is_writeable,
    };
    static const struct power_supply_desc twl4030_bci_usb_desc = {
    .name		= "twl4030_usb",
    .type		= POWER_SUPPLY_TYPE_USB,
    .properties	= twl4030_charger_props,
    .num_properties	= ARRAY_SIZE(twl4030_charger_props),
    .get_property	= twl4030_bci_get_property,
    .set_property	= twl4030_bci_set_property,
    .property_is_writeable	= twl4030_bci_property_is_writeable,
    };
#[no_mangle]
unsafe extern "C" fn twl4030_bci_probe(pdev: *mut platform_device) -> c_int {
    static int twl4030_bci_probe(struct platform_device *pdev)
    {
    struct twl4030_bci *bci;
    const struct twl4030_bci_platform_data *pdata = pdev.dev.platform_data;
    int ret;
    u32 reg;
    bci = devm_kzalloc(&pdev.dev, sizeof(*bci), GFP_KERNEL);
    if (bci == core::ptr::null_mut())
    return -ENOMEM;
    if (!pdata)
    pdata = twl4030_bci_parse_dt(&pdev.dev);
    bci.ichg_eoc = 80100; /* Stop charging when current drops to here */
    bci.ichg_lo = 241000; /* Low threshold */
    bci.ichg_hi = 500000; /* High threshold */
    bci.ac_cur = 500000; /* 500mA */
    if (allow_usb)
    bci.usb_cur_target = 500000;  /* 500mA */
    else
    bci.usb_cur_target = 100000;  /* 100mA */
    bci.usb_mode = CHARGE_AUTO;
    bci.ac_mode = CHARGE_AUTO;
    bci.dev = &pdev.dev;
    bci.irq_chg = platform_get_irq(pdev, 0);
    bci.irq_bci = platform_get_irq(pdev, 1);
    platform_set_drvdata(pdev, bci);
    ret = devm_delayed_work_autocancel(&pdev.dev, &bci.current_worker,
    twl4030_current_worker);
    if (ret)
    return ret;
    ret = devm_work_autocancel(&pdev.dev, &bci.work,
    twl4030_bci_usb_work);
    if (ret)
    return ret;
    bci.channel_vac = devm_iio_channel_get(&pdev.dev, "vac");
    if (IS_ERR(bci.channel_vac)) {
    ret = PTR_ERR(bci.channel_vac);
    if (ret == -EPROBE_DEFER)
    return ret;	/* iio not ready */
    dev_warn(&pdev.dev, "could not request vac iio channel (%d)",
    ret);
    bci.channel_vac = core::ptr::null_mut();
    }
    if (bci.dev.of_node) {
    struct device_node *phynode;
    phynode = of_get_compatible_child(bci.dev.of_node.parent,
    "ti,twl4030-usb");
    if (phynode) {
    bci.usb_nb.notifier_call = twl4030_bci_usb_ncb;
    bci.transceiver = devm_usb_get_phy_by_node(
    bci.dev, phynode, &bci.usb_nb);
    of_node_put(phynode);
    if (IS_ERR(bci.transceiver)) {
    ret = PTR_ERR(bci.transceiver);
    if (ret == -EPROBE_DEFER)
    return ret;	/* phy not ready */
    dev_warn(&pdev.dev, "could not request transceiver (%d)",
    ret);
    bci.transceiver = core::ptr::null_mut();
    }
    }
    }
    bci.ac = devm_power_supply_register(&pdev.dev, &twl4030_bci_ac_desc,
    core::ptr::null_mut());
    if (IS_ERR(bci.ac)) {
    ret = PTR_ERR(bci.ac);
    dev_err(&pdev.dev, "failed to register ac: %d\n", ret);
    return ret;
    }
    bci.usb = devm_power_supply_register(&pdev.dev, &twl4030_bci_usb_desc,
    core::ptr::null_mut());
    if (IS_ERR(bci.usb)) {
    ret = PTR_ERR(bci.usb);
    dev_err(&pdev.dev, "failed to register usb: %d\n", ret);
    return ret;
    }
    ret = devm_request_threaded_irq(&pdev.dev, bci.irq_chg, core::ptr::null_mut(),
    twl4030_charger_interrupt, IRQF_ONESHOT, pdev.name,
    bci);
    if (ret < 0)
    return ret;
    ret = devm_request_threaded_irq(&pdev.dev, bci.irq_bci, core::ptr::null_mut(),
    twl4030_bci_interrupt, IRQF_ONESHOT, pdev.name, bci);
    if (ret < 0)
    return ret;
// Enable interrupts now.
    reg = ~(u32)(TWL4030_ICHGLOW | TWL4030_ICHGEOC | TWL4030_TBATOR2 |
    TWL4030_TBATOR1 | TWL4030_BATSTS);
    ret = twl_i2c_write_u8(TWL4030_MODULE_INTERRUPTS, reg,
    TWL4030_INTERRUPTS_BCIIMR1A);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to unmask interrupts: %d\n", ret);
    return ret;
    }
    reg = ~(u32)(TWL4030_VBATOV | TWL4030_VBUSOV | TWL4030_ACCHGOV);
    ret = twl_i2c_write_u8(TWL4030_MODULE_INTERRUPTS, reg,
    TWL4030_INTERRUPTS_BCIIMR2A);
    if (ret < 0)
    dev_warn(&pdev.dev, "failed to unmask interrupts: %d\n", ret);
    twl4030_charger_update_current(bci);
    if (device_create_file(&bci.usb.dev, &dev_attr_mode))
    dev_warn(&pdev.dev, "could not create sysfs file\n");
    if (device_create_file(&bci.ac.dev, &dev_attr_mode))
    dev_warn(&pdev.dev, "could not create sysfs file\n");
    twl4030_charger_enable_ac(bci, true);
    if (!IS_ERR_OR_NULL(bci.transceiver))
    twl4030_bci_usb_ncb(&bci.usb_nb,
    bci.transceiver.last_event,
    core::ptr::null_mut());
    else
    twl4030_charger_enable_usb(bci, false);
    if (pdata)
    twl4030_charger_enable_backup(pdata.bb_uvolt,
    pdata.bb_uamp);
    else
    twl4030_charger_enable_backup(0, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_bci_remove(pdev: *mut platform_device) {
    static void twl4030_bci_remove(struct platform_device *pdev)
    {
    struct twl4030_bci *bci = platform_get_drvdata(pdev);
    twl4030_charger_enable_ac(bci, false);
    twl4030_charger_enable_usb(bci, false);
    twl4030_charger_enable_backup(0, 0);
    device_remove_file(&bci.usb.dev, &dev_attr_mode);
    device_remove_file(&bci.ac.dev, &dev_attr_mode);
// mask interrupts
    twl_i2c_write_u8(TWL4030_MODULE_INTERRUPTS, 0xff,
    TWL4030_INTERRUPTS_BCIIMR1A);
    twl_i2c_write_u8(TWL4030_MODULE_INTERRUPTS, 0xff,
    TWL4030_INTERRUPTS_BCIIMR2A);
    }
    static const struct of_device_id twl_bci_of_match[] __maybe_unused = {
    {.compatible = "ti,twl4030-bci", },
    { }
    };
    MODULE_DEVICE_TABLE(of, twl_bci_of_match);
    static struct platform_driver twl4030_bci_driver = {
    .probe = twl4030_bci_probe,
    .remove = twl4030_bci_remove,
    .driver	= {
    .name	= "twl4030_bci",
    .of_match_table = of_match_ptr(twl_bci_of_match),
    },
    };
    module_platform_driver(twl4030_bci_driver);
    MODULE_AUTHOR("Gražvydas Ignotas");
    MODULE_DESCRIPTION("TWL4030 Battery Charger Interface driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:twl4030_bci");
    MODULE_IMPORT_NS("IIO_CONSUMER");
