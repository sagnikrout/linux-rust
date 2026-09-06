//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/omap-keypad.c
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
// linux/drivers/input/keyboard/omap-keypad.c
//
// OMAP Keypad Driver
//
// Copyright (C) 2003 Nokia Corporation
// Written by Timo Teräs <ext-timo.teras@nokia.com>
//
// Added support for H2 & H3 Keypad
// Copyright (C) 2004 Texas Instruments
//

    static void omap_kp_tasklet(unsigned long);
    static void omap_kp_timer(struct timer_list *);
    static unsigned char keypad_state[8];
    static DEFINE_MUTEX(kp_enable_mutex);
    let mut kp_enable: static int = 1;
    let mut kp_cur_group: static int = -1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_kp {
    pub input: *mut input_dev,
    pub timer: timer_list,
    pub irq: c_int,
    pub rows: c_uint,
    pub cols: c_uint,
    pub delay: c_ulong,
    pub debounce: c_uint,
    pub keymap: [c_ushort; ],
}

    static DECLARE_TASKLET_DISABLED_OLD(kp_tasklet, omap_kp_tasklet);
#[no_mangle]
unsafe extern "C" fn omap_kp_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t omap_kp_interrupt(int irq, void *dev_id)
    {
// disable keyboard interrupt and schedule for handling
    omap_writew(1, OMAP1_MPUIO_BASE + OMAP_MPUIO_KBD_MASKIT);
    tasklet_schedule(&kp_tasklet);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn omap_kp_timer(unused: *mut timer_list) {
    static void omap_kp_timer(struct timer_list *unused)
    {
    tasklet_schedule(&kp_tasklet);
    }
#[no_mangle]
unsafe extern "C" fn omap_kp_scan_keypad(omap_kp: *mut omap_kp, state: *mut c_uchar) {
    static void omap_kp_scan_keypad(struct omap_kp *omap_kp, unsigned char *state)
    {
    let mut col: c_int = 0;
// disable keyboard interrupt and schedule for handling
    omap_writew(1, OMAP1_MPUIO_BASE + OMAP_MPUIO_KBD_MASKIT);
// read the keypad status
    omap_writew(0xff, OMAP1_MPUIO_BASE + OMAP_MPUIO_KBC);
    for (col = 0; col < omap_kp.cols; col++) {
    omap_writew(~(1 << col) & 0xff,
    OMAP1_MPUIO_BASE + OMAP_MPUIO_KBC);
    udelay(omap_kp.delay);
    state[col] = ~omap_readw(OMAP1_MPUIO_BASE +
    OMAP_MPUIO_KBR_LATCH) & 0xff;
    }
    omap_writew(0x00, OMAP1_MPUIO_BASE + OMAP_MPUIO_KBC);
    udelay(2);
    }
#[no_mangle]
unsafe extern "C" fn omap_kp_tasklet(data: c_ulong) {
    static void omap_kp_tasklet(unsigned long data)
    {
    struct omap_kp *omap_kp_data = (struct omap_kp *) data;
    unsigned short *keycodes = omap_kp_data.input.keycode;
    let mut row_shift: c_uint = get_count_order(omap_kp_data.cols);
    unsigned char new_state[8], changed, key_down = 0;
    int col, row;
// check for any changes
    omap_kp_scan_keypad(omap_kp_data, new_state);
// check for changes and print those
    for (col = 0; col < omap_kp_data.cols; col++) {
    changed = new_state[col] ^ keypad_state[col];
    key_down |= new_state[col];
    if (changed == 0)
    continue;
    for (row = 0; row < omap_kp_data.rows; row++) {
    int key;
    if (!(changed & (1 << row)))
    continue;

    printk(KERN_INFO "omap-keypad: key %d-%d %s\n", col,
    row, (new_state[col] & (1 << row)) ?
    "pressed" : "released");

    key = keycodes[MATRIX_SCAN_CODE(row, col, row_shift)];
    if (!(kp_cur_group == (key & GROUP_MASK) ||
    kp_cur_group == -1))
    continue;
    kp_cur_group = key & GROUP_MASK;
    input_report_key(omap_kp_data.input, key & ~GROUP_MASK,
    new_state[col] & (1 << row));

    }
    }
    input_sync(omap_kp_data.input);
    memcpy(keypad_state, new_state, sizeof(keypad_state));
    if (key_down) {
// some key is pressed - keep irq disabled and use timer
// to poll the keypad
    mod_timer(&omap_kp_data.timer, jiffies + HZ / 20);
    } else {
// enable interrupts
    omap_writew(0, OMAP1_MPUIO_BASE + OMAP_MPUIO_KBD_MASKIT);
    kp_cur_group = -1;
    }
    }
    static ssize_t omap_kp_enable_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return sprintf(buf, "%u\n", kp_enable);
    }
    static ssize_t omap_kp_enable_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct omap_kp *omap_kp = dev_get_drvdata(dev);
    int state;
    if (sscanf(buf, "%u", &state) != 1)
    return -EINVAL;
    if ((state != 1) && (state != 0))
    return -EINVAL;
    scoped_guard(mutex, &kp_enable_mutex) {
    if (state != kp_enable) {
    if (state)
    enable_irq(omap_kp.irq);
    else
    disable_irq(omap_kp.irq);
    kp_enable = state;
    }
    }
    return strnlen(buf, count);
    }
    static DEVICE_ATTR(enable, S_IRUGO | S_IWUSR, omap_kp_enable_show, omap_kp_enable_store);
    static struct attribute *omap_kp_attrs[] = {
    &dev_attr_enable.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(omap_kp);
#[no_mangle]
unsafe extern "C" fn omap_kp_probe(pdev: *mut platform_device) -> c_int {
    static int omap_kp_probe(struct platform_device *pdev)
    {
    struct omap_kp *omap_kp;
    struct input_dev *input_dev;
    struct omap_kp_platform_data *pdata = dev_get_platdata(&pdev.dev);
    int ret;
    unsigned int row_shift, keycodemax;
    if (!pdata.rows || !pdata.cols || !pdata.keymap_data) {
    printk(KERN_ERR "No rows, cols or keymap_data from pdata\n");
    return -EINVAL;
    }
    row_shift = get_count_order(pdata.cols);
    keycodemax = pdata.rows << row_shift;
    omap_kp = kzalloc_flex(*omap_kp, keymap, keycodemax);
    input_dev = input_allocate_device();
    if (!omap_kp || !input_dev) {
    kfree(omap_kp);
    input_free_device(input_dev);
    return -ENOMEM;
    }
    platform_set_drvdata(pdev, omap_kp);
    omap_kp.input = input_dev;
// Disable the interrupt for the MPUIO keyboard
    omap_writew(1, OMAP1_MPUIO_BASE + OMAP_MPUIO_KBD_MASKIT);
    if (pdata.delay)
    omap_kp.delay = pdata.delay;
    omap_kp.rows = pdata.rows;
    omap_kp.cols = pdata.cols;
    timer_setup(&omap_kp.timer, omap_kp_timer, 0);
// get the irq and init timer
    kp_tasklet.data = (unsigned long) omap_kp;
    tasklet_enable(&kp_tasklet);
// setup input device
    input_dev.name = "omap-keypad";
    input_dev.phys = "omap-keypad/input0";
    input_dev.dev.parent = &pdev.dev;
    input_dev.id.bustype = BUS_HOST;
    input_dev.id.vendor = 0x0001;
    input_dev.id.product = 0x0001;
    input_dev.id.version = 0x0100;
    if (pdata.rep)
    __set_bit(EV_REP, input_dev.evbit);
    ret = matrix_keypad_build_keymap(pdata.keymap_data, core::ptr::null_mut(),
    pdata.rows, pdata.cols,
    omap_kp.keymap, input_dev);
    if (ret < 0)
    goto err2;
    ret = input_register_device(omap_kp.input);
    if (ret < 0) {
    printk(KERN_ERR "Unable to register omap-keypad input device\n");
    goto err2;
    }
    if (pdata.dbounce)
    omap_writew(0xff, OMAP1_MPUIO_BASE + OMAP_MPUIO_GPIO_DEBOUNCING);
// scan current status and enable interrupt
    omap_kp_scan_keypad(omap_kp, keypad_state);
    omap_kp.irq = platform_get_irq(pdev, 0);
    if (omap_kp.irq >= 0) {
    if (request_irq(omap_kp.irq, omap_kp_interrupt, 0,
    "omap-keypad", omap_kp) < 0)
    goto err3;
    }
    omap_writew(0, OMAP1_MPUIO_BASE + OMAP_MPUIO_KBD_MASKIT);
    return 0;
    err3:
    input_unregister_device(omap_kp.input);
    input_dev = core::ptr::null_mut();
    err2:
    kfree(omap_kp);
    input_free_device(input_dev);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn omap_kp_remove(pdev: *mut platform_device) {
    static void omap_kp_remove(struct platform_device *pdev)
    {
    struct omap_kp *omap_kp = platform_get_drvdata(pdev);
// disable keypad interrupt handling
    tasklet_disable(&kp_tasklet);
    omap_writew(1, OMAP1_MPUIO_BASE + OMAP_MPUIO_KBD_MASKIT);
    free_irq(omap_kp.irq, omap_kp);
    timer_shutdown_sync(&omap_kp.timer);
    tasklet_kill(&kp_tasklet);
// unregister everything
    input_unregister_device(omap_kp.input);
    kfree(omap_kp);
    }
    static struct platform_driver omap_kp_driver = {
    .probe		= omap_kp_probe,
    .remove		= omap_kp_remove,
    .driver		= {
    .name	= "omap-keypad",
    .dev_groups = omap_kp_groups,
    },
    };
    module_platform_driver(omap_kp_driver);
    MODULE_AUTHOR("Timo Teräs");
    MODULE_DESCRIPTION("OMAP Keypad Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:omap-keypad");
