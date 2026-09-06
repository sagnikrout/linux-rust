//! Automatically rewritten from C to Rust
//! Source: drivers/char/tlclk.c
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
// Telecom Clock driver for Intel NetStructure(tm) MPCBL0010
//
// Copyright (C) 2005 Kontron Canada
//
// All rights reserved.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or
// NON INFRINGEMENT.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
//
// Send feedback to <sebastien.bouchard@ca.kontron.com> and the current
// Maintainer  <mark.gross@intel.com>
//
// Description : This is the TELECOM CLOCK module driver for the ATCA
// MPCBL0010 ATCA computer.
//

    MODULE_AUTHOR("Sebastien Bouchard <sebastien.bouchard@ca.kontron.com>");
    MODULE_DESCRIPTION("Telecom Clock driver for Intel NetStructure(tm) MPCBL0010");
    MODULE_LICENSE("GPL");
// Hardware Reset of the PLL
pub const RESET_ON: c_uint = 0x00;
pub const RESET_OFF: c_uint = 0x01;
// MODE SELECT
pub const NORMAL_MODE: c_uint = 0x00;
pub const HOLDOVER_MODE: c_uint = 0x10;
pub const FREERUN_MODE: c_uint = 0x20;
// FILTER SELECT
pub const FILTER_6HZ: c_uint = 0x04;
pub const FILTER_12HZ: c_uint = 0x00;
// SELECT REFERENCE FREQUENCY
pub const REF_CLK1_8kHz: c_uint = 0x00;
pub const REF_CLK2_19_44MHz: c_uint = 0x02;
// Select primary or secondary redundant clock
pub const PRIMARY_CLOCK: c_uint = 0x00;
pub const SECONDARY_CLOCK: c_uint = 0x01;
// CLOCK TRANSMISSION DEFINE
pub const CLK_8kHz: c_uint = 0xff;
pub const CLK_16_384MHz: c_uint = 0xfb;
pub const CLK_1_544MHz: c_uint = 0x00;
pub const CLK_2_048MHz: c_uint = 0x01;
pub const CLK_4_096MHz: c_uint = 0x02;
pub const CLK_6_312MHz: c_uint = 0x03;
pub const CLK_8_192MHz: c_uint = 0x04;
pub const CLK_19_440MHz: c_uint = 0x06;
pub const CLK_8_592MHz: c_uint = 0x08;
pub const CLK_11_184MHz: c_uint = 0x09;
pub const CLK_34_368MHz: c_uint = 0x0b;
pub const CLK_44_736MHz: c_uint = 0x0a;
// RECEIVED REFERENCE
pub const AMC_B1: c_int = 0;
pub const AMC_B2: c_int = 1;
// HARDWARE SWITCHING DEFINE
pub const HW_ENABLE: c_uint = 0x80;
pub const HW_DISABLE: c_uint = 0x00;
// HARDWARE SWITCHING MODE DEFINE
pub const PLL_HOLDOVER: c_uint = 0x40;
pub const LOST_CLOCK: c_uint = 0x00;
// ALARMS DEFINE
pub const UNLOCK_MASK: c_uint = 0x10;
pub const HOLDOVER_MASK: c_uint = 0x20;
pub const SEC_LOST_MASK: c_uint = 0x40;
pub const PRI_LOST_MASK: c_uint = 0x80;
// INTERRUPT CAUSE DEFINE
pub const PRI_LOS_01_MASK: c_uint = 0x01;
pub const PRI_LOS_10_MASK: c_uint = 0x02;
pub const SEC_LOS_01_MASK: c_uint = 0x04;
pub const SEC_LOS_10_MASK: c_uint = 0x08;
pub const HOLDOVER_01_MASK: c_uint = 0x10;
pub const HOLDOVER_10_MASK: c_uint = 0x20;
pub const UNLOCK_01_MASK: c_uint = 0x40;
pub const UNLOCK_10_MASK: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlclk_alarms {
    pub lost_clocks: __u32,
    pub lost_primary_clock: __u32,
    pub lost_secondary_clock: __u32,
    pub primary_clock_back: __u32,
    pub secondary_clock_back: __u32,
    pub switchover_primary: __u32,
    pub switchover_secondary: __u32,
    pub pll_holdover: __u32,
    pub pll_end_holdover: __u32,
    pub pll_lost_sync: __u32,
    pub pll_sync: __u32,
}

// Telecom clock I/O register definition
pub const TLCLK_BASE: c_uint = 0xa08;

// 0 = Dynamic allocation of the major device number
pub const TLCLK_MAJOR: c_int = 0;
// sysfs interface definition:
    Upon loading the driver will create a sysfs directory under
    /sys/devices/platform/telco_clock.
    This directory exports the following interfaces.  There operation is
    documented in the MCPBL0010 TPS under the Telecom Clock API section, 11.4.
    alarms				:
    current_ref			:
    received_ref_clk3a		:
    received_ref_clk3b		:
    enable_clk3a_output		:
    enable_clk3b_output		:
    enable_clka0_output		:
    enable_clka1_output		:
    enable_clkb0_output		:
    enable_clkb1_output		:
    filter_select			:
    hardware_switching		:
    hardware_switching_mode		:
    telclock_version		:
    mode_select			:
    refalign			:
    reset				:
    select_amcb1_transmit_clock	:
    select_amcb2_transmit_clock	:
    select_redundant_clock		:
    select_ref_frequency		:
    All sysfs interfaces are integers in hex format, i.e echo 99 > refalign
    has the same effect as echo 0x99 > refalign.
//
    static unsigned int telclk_interrupt;
    static int int_events;		/* Event that generate a interrupt */
    static int got_event;		/* if events processing have been done */
    static void switchover_timeout(struct timer_list *t);
    static struct timer_list switchover_timer;
    static unsigned long tlclk_timer_data;
    static struct tlclk_alarms *alarm_events;
    static DEFINE_SPINLOCK(event_lock);
    let mut tlclk_major: static int = TLCLK_MAJOR;
    static irqreturn_t tlclk_interrupt(int irq, void *dev_id);
    static DECLARE_WAIT_QUEUE_HEAD(wq);
    static unsigned long useflags;
    static DEFINE_MUTEX(tlclk_mutex);
#[no_mangle]
unsafe extern "C" fn tlclk_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int tlclk_open(struct inode *inode, struct file *filp)
    {
    int result;
    mutex_lock(&tlclk_mutex);
    if (test_and_set_bit(0, &useflags)) {
    result = -EBUSY;
// this legacy device is always one per system and it doesn't
// know how to handle multiple concurrent clients.
//
    goto out;
    }
// Make sure there is no interrupt pending while
// initialising interrupt handler
    inb(TLCLK_REG6);
// This device is wired through the FPGA IO space of the ATCA blade
// we can't share this IRQ
    result = request_irq(telclk_interrupt, &tlclk_interrupt,
    0, "telco_clock", tlclk_interrupt);
    if (result == -EBUSY)
    printk(KERN_ERR "tlclk: Interrupt can't be reserved.\n");
    else
    inb(TLCLK_REG6);	/* Clear interrupt events */
    out:
    mutex_unlock(&tlclk_mutex);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn tlclk_release(inode: *mut inode, filp: *mut file) -> c_int {
    static int tlclk_release(struct inode *inode, struct file *filp)
    {
    free_irq(telclk_interrupt, tlclk_interrupt);
    clear_bit(0, &useflags);
    return 0;
    }
    static ssize_t tlclk_read(struct file *filp, char __user *buf, size_t count,
    loff_t *f_pos)
    {
    if (count < sizeof(struct tlclk_alarms))
    return -EIO;
    if (mutex_lock_interruptible(&tlclk_mutex))
    return -EINTR;
    wait_event_interruptible(wq, got_event);
    if (copy_to_user(buf, alarm_events, sizeof(struct tlclk_alarms))) {
    mutex_unlock(&tlclk_mutex);
    return -EFAULT;
    }
    memset(alarm_events, 0, sizeof(struct tlclk_alarms));
    got_event = 0;
    mutex_unlock(&tlclk_mutex);
    return  sizeof(struct tlclk_alarms);
    }
    static const struct file_operations tlclk_fops = {
    .owner = THIS_MODULE,
    .read = tlclk_read,
    .open = tlclk_open,
    .release = tlclk_release,
    .llseek = noop_llseek,
    };
    static struct miscdevice tlclk_miscdev = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = "telco_clock",
    .fops = &tlclk_fops,
    };
    static ssize_t show_current_ref(struct device *d,
    struct device_attribute *attr, char *buf)
    {
    unsigned long ret_val;
    unsigned long flags;
    spin_lock_irqsave(&event_lock, flags);
    ret_val = ((inb(TLCLK_REG1) & 0x08) >> 3);
    spin_unlock_irqrestore(&event_lock, flags);
    return sprintf(buf, "0x%lX\n", ret_val);
    }
    static DEVICE_ATTR(current_ref, S_IRUGO, show_current_ref, core::ptr::null_mut());
    static ssize_t show_telclock_version(struct device *d,
    struct device_attribute *attr, char *buf)
    {
    unsigned long ret_val;
    unsigned long flags;
    spin_lock_irqsave(&event_lock, flags);
    ret_val = inb(TLCLK_REG5);
    spin_unlock_irqrestore(&event_lock, flags);
    return sprintf(buf, "0x%lX\n", ret_val);
    }
    static DEVICE_ATTR(telclock_version, S_IRUGO,
    show_telclock_version, core::ptr::null_mut());
    static ssize_t show_alarms(struct device *d,
    struct device_attribute *attr,  char *buf)
    {
    unsigned long ret_val;
    unsigned long flags;
    spin_lock_irqsave(&event_lock, flags);
    ret_val = (inb(TLCLK_REG2) & 0xf0);
    spin_unlock_irqrestore(&event_lock, flags);
    return sprintf(buf, "0x%lX\n", ret_val);
    }
    static DEVICE_ATTR(alarms, S_IRUGO, show_alarms, core::ptr::null_mut());
    static ssize_t store_received_ref_clk3a(struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    let mut tmp: c_ulong = 0;
    unsigned char val;
    unsigned long flags;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, ": tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG1, 0xef, val);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(received_ref_clk3a, (S_IWUSR|S_IWGRP), core::ptr::null_mut(),
    store_received_ref_clk3a);
    static ssize_t store_received_ref_clk3b(struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    let mut tmp: c_ulong = 0;
    unsigned char val;
    unsigned long flags;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, ": tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG1, 0xdf, val << 1);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(received_ref_clk3b, (S_IWUSR|S_IWGRP), core::ptr::null_mut(),
    store_received_ref_clk3b);
    static ssize_t store_enable_clk3b_output(struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    let mut tmp: c_ulong = 0;
    unsigned char val;
    unsigned long flags;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, ": tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG3, 0x7f, val << 7);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(enable_clk3b_output, (S_IWUSR|S_IWGRP), core::ptr::null_mut(),
    store_enable_clk3b_output);
    static ssize_t store_enable_clk3a_output(struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    unsigned long flags;
    let mut tmp: c_ulong = 0;
    unsigned char val;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, "tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG3, 0xbf, val << 6);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(enable_clk3a_output, (S_IWUSR|S_IWGRP), core::ptr::null_mut(),
    store_enable_clk3a_output);
    static ssize_t store_enable_clkb1_output(struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    unsigned long flags;
    let mut tmp: c_ulong = 0;
    unsigned char val;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, "tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG2, 0xf7, val << 3);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(enable_clkb1_output, (S_IWUSR|S_IWGRP), core::ptr::null_mut(),
    store_enable_clkb1_output);
    static ssize_t store_enable_clka1_output(struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    unsigned long flags;
    let mut tmp: c_ulong = 0;
    unsigned char val;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, "tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG2, 0xfb, val << 2);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(enable_clka1_output, (S_IWUSR|S_IWGRP), core::ptr::null_mut(),
    store_enable_clka1_output);
    static ssize_t store_enable_clkb0_output(struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    unsigned long flags;
    let mut tmp: c_ulong = 0;
    unsigned char val;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, "tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG2, 0xfd, val << 1);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(enable_clkb0_output, (S_IWUSR|S_IWGRP), core::ptr::null_mut(),
    store_enable_clkb0_output);
    static ssize_t store_enable_clka0_output(struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    unsigned long flags;
    let mut tmp: c_ulong = 0;
    unsigned char val;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, "tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG2, 0xfe, val);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(enable_clka0_output, (S_IWUSR|S_IWGRP), core::ptr::null_mut(),
    store_enable_clka0_output);
    static ssize_t store_select_amcb2_transmit_clock(struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    unsigned long flags;
    let mut tmp: c_ulong = 0;
    unsigned char val;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, "tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    if ((val == CLK_8kHz) || (val == CLK_16_384MHz)) {
    SET_PORT_BITS(TLCLK_REG3, 0xc7, 0x28);
    SET_PORT_BITS(TLCLK_REG1, 0xfb, ~val);
    } else if (val >= CLK_8_592MHz) {
    SET_PORT_BITS(TLCLK_REG3, 0xc7, 0x38);
    switch (val) {
    case CLK_8_592MHz:
    SET_PORT_BITS(TLCLK_REG0, 0xfc, 2);
    break;
    case CLK_11_184MHz:
    SET_PORT_BITS(TLCLK_REG0, 0xfc, 0);
    break;
    case CLK_34_368MHz:
    SET_PORT_BITS(TLCLK_REG0, 0xfc, 3);
    break;
    case CLK_44_736MHz:
    SET_PORT_BITS(TLCLK_REG0, 0xfc, 1);
    break;
    }
    } else {
    SET_PORT_BITS(TLCLK_REG3, 0xc7, val << 3);
    }
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(select_amcb2_transmit_clock, (S_IWUSR|S_IWGRP), core::ptr::null_mut(),
    store_select_amcb2_transmit_clock);
    static ssize_t store_select_amcb1_transmit_clock(struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    let mut tmp: c_ulong = 0;
    unsigned char val;
    unsigned long flags;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, "tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    if ((val == CLK_8kHz) || (val == CLK_16_384MHz)) {
    SET_PORT_BITS(TLCLK_REG3, 0xf8, 0x5);
    SET_PORT_BITS(TLCLK_REG1, 0xfb, ~val);
    } else if (val >= CLK_8_592MHz) {
    SET_PORT_BITS(TLCLK_REG3, 0xf8, 0x7);
    switch (val) {
    case CLK_8_592MHz:
    SET_PORT_BITS(TLCLK_REG0, 0xfc, 2);
    break;
    case CLK_11_184MHz:
    SET_PORT_BITS(TLCLK_REG0, 0xfc, 0);
    break;
    case CLK_34_368MHz:
    SET_PORT_BITS(TLCLK_REG0, 0xfc, 3);
    break;
    case CLK_44_736MHz:
    SET_PORT_BITS(TLCLK_REG0, 0xfc, 1);
    break;
    }
    } else {
    SET_PORT_BITS(TLCLK_REG3, 0xf8, val);
    }
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(select_amcb1_transmit_clock, (S_IWUSR|S_IWGRP), core::ptr::null_mut(),
    store_select_amcb1_transmit_clock);
    static ssize_t store_select_redundant_clock(struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    let mut tmp: c_ulong = 0;
    unsigned char val;
    unsigned long flags;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, "tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG1, 0xfe, val);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(select_redundant_clock, (S_IWUSR|S_IWGRP), core::ptr::null_mut(),
    store_select_redundant_clock);
    static ssize_t store_select_ref_frequency(struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    let mut tmp: c_ulong = 0;
    unsigned char val;
    unsigned long flags;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, "tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG1, 0xfd, val);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(select_ref_frequency, (S_IWUSR|S_IWGRP), core::ptr::null_mut(),
    store_select_ref_frequency);
    static ssize_t store_filter_select(struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    let mut tmp: c_ulong = 0;
    unsigned char val;
    unsigned long flags;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, "tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG0, 0xfb, val);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(filter_select, (S_IWUSR|S_IWGRP), core::ptr::null_mut(), store_filter_select);
    static ssize_t store_hardware_switching_mode(struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    let mut tmp: c_ulong = 0;
    unsigned char val;
    unsigned long flags;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, "tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG0, 0xbf, val);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(hardware_switching_mode, (S_IWUSR|S_IWGRP), core::ptr::null_mut(),
    store_hardware_switching_mode);
    static ssize_t store_hardware_switching(struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    let mut tmp: c_ulong = 0;
    unsigned char val;
    unsigned long flags;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, "tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG0, 0x7f, val);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(hardware_switching, (S_IWUSR|S_IWGRP), core::ptr::null_mut(),
    store_hardware_switching);
    static ssize_t store_refalign (struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    let mut tmp: c_ulong = 0;
    unsigned long flags;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, "tmp = 0x%lX\n", tmp);
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG0, 0xf7, 0);
    SET_PORT_BITS(TLCLK_REG0, 0xf7, 0x08);
    SET_PORT_BITS(TLCLK_REG0, 0xf7, 0);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(refalign, (S_IWUSR|S_IWGRP), core::ptr::null_mut(), store_refalign);
    static ssize_t store_mode_select (struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    let mut tmp: c_ulong = 0;
    unsigned char val;
    unsigned long flags;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, "tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG0, 0xcf, val);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(mode_select, (S_IWUSR|S_IWGRP), core::ptr::null_mut(), store_mode_select);
    static ssize_t store_reset (struct device *d,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    let mut tmp: c_ulong = 0;
    unsigned char val;
    unsigned long flags;
    sscanf(buf, "%lX", &tmp);
    dev_dbg(d, "tmp = 0x%lX\n", tmp);
    val = (unsigned char)tmp;
    spin_lock_irqsave(&event_lock, flags);
    SET_PORT_BITS(TLCLK_REG4, 0xfd, val);
    spin_unlock_irqrestore(&event_lock, flags);
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(reset, (S_IWUSR|S_IWGRP), core::ptr::null_mut(), store_reset);
    static struct attribute *tlclk_attrs[] = {
    &dev_attr_current_ref.attr,
    &dev_attr_telclock_version.attr,
    &dev_attr_alarms.attr,
    &dev_attr_received_ref_clk3a.attr,
    &dev_attr_received_ref_clk3b.attr,
    &dev_attr_enable_clk3a_output.attr,
    &dev_attr_enable_clk3b_output.attr,
    &dev_attr_enable_clkb1_output.attr,
    &dev_attr_enable_clka1_output.attr,
    &dev_attr_enable_clkb0_output.attr,
    &dev_attr_enable_clka0_output.attr,
    &dev_attr_select_amcb1_transmit_clock.attr,
    &dev_attr_select_amcb2_transmit_clock.attr,
    &dev_attr_select_redundant_clock.attr,
    &dev_attr_select_ref_frequency.attr,
    &dev_attr_filter_select.attr,
    &dev_attr_hardware_switching_mode.attr,
    &dev_attr_hardware_switching.attr,
    &dev_attr_refalign.attr,
    &dev_attr_mode_select.attr,
    &dev_attr_reset.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(tlclk);
    static struct faux_device *tlclk_device;
#[no_mangle]
unsafe extern "C" fn tlclk_init() -> int __init {
    static int __init tlclk_init(void)
    {
    int ret;
    telclk_interrupt = (inb(TLCLK_REG7) & 0x0f);
    alarm_events = kzalloc_obj(struct tlclk_alarms);
    if (!alarm_events) {
    ret = -ENOMEM;
    goto out1;
    }
    ret = register_chrdev(tlclk_major, "telco_clock", &tlclk_fops);
    if (ret < 0) {
    printk(KERN_ERR "tlclk: can't get major %d.\n", tlclk_major);
    kfree(alarm_events);
    return ret;
    }
    tlclk_major = ret;
// Read telecom clock IRQ number (Set by BIOS)
    if (!request_region(TLCLK_BASE, 8, "telco_clock")) {
    printk(KERN_ERR "tlclk: request_region 0x%X failed.\n",
    TLCLK_BASE);
    ret = -EBUSY;
    goto out2;
    }
    if (0x0F == telclk_interrupt ) { /* not MCPBL0010 ? */
    printk(KERN_ERR "telclk_interrupt = 0x%x non-mcpbl0010 hw.\n",
    telclk_interrupt);
    ret = -ENXIO;
    goto out3;
    }
    timer_setup(&switchover_timer, switchover_timeout, 0);
    ret = misc_register(&tlclk_miscdev);
    if (ret < 0) {
    printk(KERN_ERR "tlclk: misc_register returns %d.\n", ret);
    goto out3;
    }
    tlclk_device = faux_device_create_with_groups("telco_clock", core::ptr::null_mut(), core::ptr::null_mut(), tlclk_groups);
    if (!tlclk_device) {
    ret = -ENODEV;
    goto out4;
    }
    return 0;
    out4:
    misc_deregister(&tlclk_miscdev);
    out3:
    release_region(TLCLK_BASE, 8);
    out2:
    kfree(alarm_events);
    unregister_chrdev(tlclk_major, "telco_clock");
    out1:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tlclk_cleanup() -> void __exit {
    static void __exit tlclk_cleanup(void)
    {
    faux_device_destroy(tlclk_device);
    misc_deregister(&tlclk_miscdev);
    unregister_chrdev(tlclk_major, "telco_clock");
    got_event = 1;
    wake_up_all(&wq);
    release_region(TLCLK_BASE, 8);
    timer_delete_sync(&switchover_timer);
    kfree(alarm_events);
    }
#[no_mangle]
unsafe extern "C" fn switchover_timeout(unused: *mut timer_list) {
    static void switchover_timeout(struct timer_list *unused)
    {
    let mut flags: c_ulong = tlclk_timer_data;
    if ((flags & 1)) {
    if ((inb(TLCLK_REG1) & 0x08) != (flags & 0x08))
    alarm_events.switchover_primary++;
    } else {
    if ((inb(TLCLK_REG1) & 0x08) != (flags & 0x08))
    alarm_events.switchover_secondary++;
    }
// Alarm processing is done, wake up read task
    timer_delete(&switchover_timer);
    got_event = 1;
    wake_up(&wq);
    }
#[no_mangle]
unsafe extern "C" fn tlclk_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t tlclk_interrupt(int irq, void *dev_id)
    {
    unsigned long flags;
    spin_lock_irqsave(&event_lock, flags);
// Read and clear interrupt events
    int_events = inb(TLCLK_REG6);
// Primary_Los changed from 0 to 1 ?
    if (int_events & PRI_LOS_01_MASK) {
    if (inb(TLCLK_REG2) & SEC_LOST_MASK)
    alarm_events.lost_clocks++;
    else
    alarm_events.lost_primary_clock++;
    }
// Primary_Los changed from 1 to 0 ?
    if (int_events & PRI_LOS_10_MASK) {
    alarm_events.primary_clock_back++;
    SET_PORT_BITS(TLCLK_REG1, 0xFE, 1);
    }
// Secondary_Los changed from 0 to 1 ?
    if (int_events & SEC_LOS_01_MASK) {
    if (inb(TLCLK_REG2) & PRI_LOST_MASK)
    alarm_events.lost_clocks++;
    else
    alarm_events.lost_secondary_clock++;
    }
// Secondary_Los changed from 1 to 0 ?
    if (int_events & SEC_LOS_10_MASK) {
    alarm_events.secondary_clock_back++;
    SET_PORT_BITS(TLCLK_REG1, 0xFE, 0);
    }
    if (int_events & HOLDOVER_10_MASK)
    alarm_events.pll_end_holdover++;
    if (int_events & UNLOCK_01_MASK)
    alarm_events.pll_lost_sync++;
    if (int_events & UNLOCK_10_MASK)
    alarm_events.pll_sync++;
// Holdover changed from 0 to 1 ?
    if (int_events & HOLDOVER_01_MASK) {
    alarm_events.pll_holdover++;
// TIMEOUT in ~10ms
    switchover_timer.expires = jiffies + msecs_to_jiffies(10);
    tlclk_timer_data = inb(TLCLK_REG1);
    mod_timer(&switchover_timer, switchover_timer.expires);
    } else {
    got_event = 1;
    wake_up(&wq);
    }
    spin_unlock_irqrestore(&event_lock, flags);
    return IRQ_HANDLED;
    }
    module_init(tlclk_init);
    module_exit(tlclk_cleanup);
