//! Automatically rewritten from C to Rust
//! Source: drivers/auxdisplay/arm-charlcd.c
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
// Driver for the on-board character LCD found on some ARM reference boards
// This is basically an Hitachi HD44780 LCD with a custom IP block to drive it
// https://en.wikipedia.org/wiki/HD44780_Character_LCD
// Currently it will just display the text "ARM Linux" and the linux version
//
// Author: Linus Walleij <triad@df.lth.se>
//

// Offsets to registers
pub const CHAR_COM: c_uint = 0x00U;
pub const CHAR_DAT: c_uint = 0x04U;
pub const CHAR_RD: c_uint = 0x08U;
pub const CHAR_RAW: c_uint = 0x0CU;
pub const CHAR_MASK: c_uint = 0x10U;
pub const CHAR_STAT: c_uint = 0x14U;
pub const CHAR_RAW_CLEAR: c_uint = 0x00000000U;
pub const CHAR_RAW_VALID: c_uint = 0x00000100U;
// Hitachi HD44780 display commands
pub const HD_CLEAR: c_uint = 0x01U;
pub const HD_HOME: c_uint = 0x02U;
pub const HD_ENTRYMODE: c_uint = 0x04U;
pub const HD_ENTRYMODE_INCREMENT: c_uint = 0x02U;
pub const HD_ENTRYMODE_SHIFT: c_uint = 0x01U;
pub const HD_DISPCTRL: c_uint = 0x08U;
pub const HD_DISPCTRL_ON: c_uint = 0x04U;
pub const HD_DISPCTRL_CURSOR_ON: c_uint = 0x02U;
pub const HD_DISPCTRL_CURSOR_BLINK: c_uint = 0x01U;
pub const HD_CRSR_SHIFT: c_uint = 0x10U;
pub const HD_CRSR_SHIFT_DISPLAY: c_uint = 0x08U;
pub const HD_CRSR_SHIFT_DISPLAY_RIGHT: c_uint = 0x04U;
pub const HD_FUNCSET: c_uint = 0x20U;
pub const HD_FUNCSET_8BIT: c_uint = 0x10U;
pub const HD_FUNCSET_2_LINES: c_uint = 0x08U;
pub const HD_FUNCSET_FONT_5X10: c_uint = 0x04U;
pub const HD_SET_CGRAM: c_uint = 0x40U;
pub const HD_SET_DDRAM: c_uint = 0x80U;
pub const HD_BUSY_FLAG: c_uint = 0x80U;
//
// struct charlcd - Private data structure
// @dev: a pointer back to containing device
// @virtbase: the offset to the controller in virtual memory
// @irq: reserved interrupt number
// @complete: completion structure for the last LCD command
// @init_work: delayed work structure to initialize the display on boot
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct charlcd {
    pub dev: *mut device,
    pub virtbase: *mut void __iomem,
    pub irq: c_int,
    pub complete: completion,
    pub init_work: delayed_work,
}

#[no_mangle]
unsafe extern "C" fn charlcd_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t charlcd_interrupt(int irq, void *data)
    {
    struct charlcd *lcd = data;
    u8 status;
    status = readl(lcd.virtbase + CHAR_STAT) & 0x01;
// Clear IRQ
    writel(CHAR_RAW_CLEAR, lcd.virtbase + CHAR_RAW);
    if (status)
    complete(&lcd.complete);
    else
    dev_info(lcd.dev, "Spurious IRQ (%02x)\n", status);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn charlcd_wait_complete_irq(lcd: *mut charlcd) {
    static void charlcd_wait_complete_irq(struct charlcd *lcd)
    {
    int ret;
    ret = wait_for_completion_interruptible_timeout(&lcd.complete,
    CHARLCD_TIMEOUT);
// Disable IRQ after completion
    writel(0x00, lcd.virtbase + CHAR_MASK);
    if (ret < 0) {
    dev_err(lcd.dev,
    "wait_for_completion_interruptible_timeout() returned %d waiting for ready\n",
    ret);
    return;
    }
    if (ret == 0) {
    dev_err(lcd.dev, "charlcd controller timed out waiting for ready\n");
    return;
    }
    }
#[no_mangle]
unsafe extern "C" fn charlcd_4bit_read_char(lcd: *mut charlcd) -> u8 {
    static u8 charlcd_4bit_read_char(struct charlcd *lcd)
    {
    u8 data;
    u32 val;
// If we can, use an IRQ to wait for the data, else poll
    if (lcd.irq >= 0)
    charlcd_wait_complete_irq(lcd);
    else {
    udelay(100);
    readl_poll_timeout_atomic(lcd.virtbase + CHAR_RAW, val,
    val & CHAR_RAW_VALID, 100, 1000);
    writel(CHAR_RAW_CLEAR, lcd.virtbase + CHAR_RAW);
    }
    msleep(1);
// Read the 4 high bits of the data
    data = readl(lcd.virtbase + CHAR_RD) & 0xf0;
//
// The second read for the low bits does not trigger an IRQ
// so in this case we have to poll for the 4 lower bits
//
    udelay(100);
    readl_poll_timeout_atomic(lcd.virtbase + CHAR_RAW, val,
    val & CHAR_RAW_VALID, 100, 1000);
    writel(CHAR_RAW_CLEAR, lcd.virtbase + CHAR_RAW);
    msleep(1);
// Read the 4 low bits of the data
    data |= (readl(lcd.virtbase + CHAR_RD) >> 4) & 0x0f;
    return data;
    }
#[no_mangle]
unsafe extern "C" fn charlcd_4bit_read_bf(lcd: *mut charlcd) -> bool {
    static bool charlcd_4bit_read_bf(struct charlcd *lcd)
    {
    if (lcd.irq >= 0) {
//
// If we'll use IRQs to wait for the busyflag, clear any
// pending flag and enable IRQ
//
    writel(CHAR_RAW_CLEAR, lcd.virtbase + CHAR_RAW);
    init_completion(&lcd.complete);
    writel(0x01, lcd.virtbase + CHAR_MASK);
    }
    readl(lcd.virtbase + CHAR_COM);
    return charlcd_4bit_read_char(lcd) & HD_BUSY_FLAG;
    }
#[no_mangle]
unsafe extern "C" fn charlcd_4bit_wait_busy(lcd: *mut charlcd) {
    static void charlcd_4bit_wait_busy(struct charlcd *lcd)
    {
    let mut retries: c_int = 50;
    udelay(100);
    while (charlcd_4bit_read_bf(lcd) && retries)
    retries--;
    if (!retries)
    dev_err(lcd.dev, "timeout waiting for busyflag\n");
    }
#[no_mangle]
unsafe extern "C" fn charlcd_4bit_command(lcd: *mut charlcd, cmd: u8) {
    static void charlcd_4bit_command(struct charlcd *lcd, u8 cmd)
    {
    let mut cmdlo: u32 = (cmd << 4) & 0xf0;
    let mut cmdhi: u32 = (cmd & 0xf0);
    writel(cmdhi, lcd.virtbase + CHAR_COM);
    udelay(10);
    writel(cmdlo, lcd.virtbase + CHAR_COM);
    charlcd_4bit_wait_busy(lcd);
    }
#[no_mangle]
unsafe extern "C" fn charlcd_4bit_char(lcd: *mut charlcd, ch: u8) {
    static void charlcd_4bit_char(struct charlcd *lcd, u8 ch)
    {
    let mut chlo: u32 = (ch << 4) & 0xf0;
    let mut chhi: u32 = (ch & 0xf0);
    writel(chhi, lcd.virtbase + CHAR_DAT);
    udelay(10);
    writel(chlo, lcd.virtbase + CHAR_DAT);
    charlcd_4bit_wait_busy(lcd);
    }
#[no_mangle]
unsafe extern "C" fn charlcd_4bit_print(lcd: *mut charlcd, line: c_int, str: *const c_char) {
    static void charlcd_4bit_print(struct charlcd *lcd, int line, const char *str)
    {
    u8 offset;
    int i;
//
// We support line 0, 1
// Line 1 runs from 0x00..0x27
// Line 2 runs from 0x28..0x4f
//
    if (line == 0)
    offset = 0;
#[no_mangle]
pub unsafe extern "C" fn if(1: line ==) -> else {
    else if (line == 1)
    offset = 0x28;
    else
    return;
// Set offset
    charlcd_4bit_command(lcd, HD_SET_DDRAM | offset);
// Send string
    for (i = 0; i < strlen(str) && i < 0x28; i++)
    charlcd_4bit_char(lcd, str[i]);
    }
#[no_mangle]
unsafe extern "C" fn charlcd_4bit_init(lcd: *mut charlcd) {
    static void charlcd_4bit_init(struct charlcd *lcd)
    {
// These commands cannot be checked with the busy flag
    writel(HD_FUNCSET | HD_FUNCSET_8BIT, lcd.virtbase + CHAR_COM);
    msleep(5);
    writel(HD_FUNCSET | HD_FUNCSET_8BIT, lcd.virtbase + CHAR_COM);
    udelay(100);
    writel(HD_FUNCSET | HD_FUNCSET_8BIT, lcd.virtbase + CHAR_COM);
    udelay(100);
// Go to 4bit mode
    writel(HD_FUNCSET, lcd.virtbase + CHAR_COM);
    udelay(100);
//
// 4bit mode, 2 lines, 5x8 font, after this the number of lines
// and the font cannot be changed until the next initialization sequence
//
    charlcd_4bit_command(lcd, HD_FUNCSET | HD_FUNCSET_2_LINES);
    charlcd_4bit_command(lcd, HD_DISPCTRL | HD_DISPCTRL_ON);
    charlcd_4bit_command(lcd, HD_ENTRYMODE | HD_ENTRYMODE_INCREMENT);
    charlcd_4bit_command(lcd, HD_CLEAR);
    charlcd_4bit_command(lcd, HD_HOME);
// Put something useful in the display
    charlcd_4bit_print(lcd, 0, "ARM Linux");
    charlcd_4bit_print(lcd, 1, UTS_RELEASE);
    }
#[no_mangle]
unsafe extern "C" fn charlcd_init_work(work: *mut work_struct) {
    static void charlcd_init_work(struct work_struct *work)
    {
    struct charlcd *lcd =
    container_of(work, struct charlcd, init_work.work);
    charlcd_4bit_init(lcd);
    }
#[no_mangle]
unsafe extern "C" fn charlcd_probe(pdev: *mut platform_device) -> int __init {
    static int __init charlcd_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int ret;
    struct charlcd *lcd;
    lcd = devm_kzalloc(dev, sizeof(*lcd), GFP_KERNEL);
    if (!lcd)
    return -ENOMEM;
    lcd.dev = &pdev.dev;
    lcd.virtbase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(lcd.virtbase))
    return PTR_ERR(lcd.virtbase);
    lcd.irq = platform_get_irq(pdev, 0);
// If no IRQ is supplied, we'll survive without it
    if (lcd.irq >= 0) {
    ret = devm_request_irq(dev, lcd.irq, charlcd_interrupt, 0, DRIVERNAME, lcd);
    if (ret)
    return ret;
    }
    platform_set_drvdata(pdev, lcd);
//
// Initialize the display in a delayed work, because
// it is VERY slow and would slow down the boot of the system.
//
    INIT_DELAYED_WORK(&lcd.init_work, charlcd_init_work);
    schedule_delayed_work(&lcd.init_work, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn charlcd_suspend(dev: *mut device) -> c_int {
    static int charlcd_suspend(struct device *dev)
    {
    struct charlcd *lcd = dev_get_drvdata(dev);
// Power the display off
    charlcd_4bit_command(lcd, HD_DISPCTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn charlcd_resume(dev: *mut device) -> c_int {
    static int charlcd_resume(struct device *dev)
    {
    struct charlcd *lcd = dev_get_drvdata(dev);
// Turn the display back on
    charlcd_4bit_command(lcd, HD_DISPCTRL | HD_DISPCTRL_ON);
    return 0;
    }
    static const struct dev_pm_ops charlcd_pm_ops = {
    .suspend = charlcd_suspend,
    .resume = charlcd_resume,
    };
    static const struct of_device_id charlcd_match[] = {
    { .compatible = "arm,versatile-lcd", },
    {}
    };
    static struct platform_driver charlcd_driver = {
    .driver = {
    .name = DRIVERNAME,
    .pm = &charlcd_pm_ops,
    .suppress_bind_attrs = true,
    .of_match_table = charlcd_match,
    },
    };
    builtin_platform_driver_probe(charlcd_driver, charlcd_probe);
