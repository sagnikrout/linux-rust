//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/hilkbd.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/drivers/hil/hilkbd.c
//
// Copyright (C) 1998 Philip Blundell <philb@gnu.org>
// Copyright (C) 1999 Matthew Wilcox <willy@infradead.org>
// Copyright (C) 1999-2007 Helge Deller <deller@gmx.de>
//
// Very basic HP Human Interface Loop (HIL) driver.
// This driver handles the keyboard on HP300 (m68k) and on some
// HP700 (parisc) series machines.
//

    MODULE_AUTHOR("Philip Blundell, Matthew Wilcox, Helge Deller");
    MODULE_DESCRIPTION("HIL keyboard driver (basic functionality)");
    MODULE_LICENSE("GPL v2");

    static unsigned long hil_base;	/* HPA for the HIL device */
    static unsigned int hil_irq;

pub const HIL_DATA: c_uint = 0x800;
pub const HIL_CMD: c_uint = 0x801;

pub const HILBASE: c_uint = 0xf0428000UL /* HP300 (m68k) port address */;
pub const HIL_DATA: c_uint = 0x1;
pub const HIL_CMD: c_uint = 0x3;
pub const HIL_IRQ: c_int = 2;

// HIL helper functions

// HIL constants
pub const HIL_BUSY: c_uint = 0x02;
pub const HIL_DATA_RDY: c_uint = 0x01;
pub const HIL_SETARD: c_uint = 0xA0		/* set auto-repeat delay */;
pub const HIL_SETARR: c_uint = 0xA2		/* set auto-repeat rate */;
pub const HIL_SETTONE: c_uint = 0xA3		/* set tone generator */;
pub const HIL_CNMT: c_uint = 0xB2		/* clear nmi */;
pub const HIL_INTON: c_uint = 0x5C		/* Turn on interrupts. */;
pub const HIL_INTOFF: c_uint = 0x5D		/* Turn off interrupts. */;
pub const HIL_READKBDSADR: c_uint = 0xF9;
pub const HIL_WRITEKBDSADR: c_uint = 0xE9;
    static unsigned int hphilkeyb_keycode[HIL_KEYCODES_SET1_TBLSIZE] __read_mostly =
    { HIL_KEYCODES_SET1 };
// HIL structure
    static struct {
    struct input_dev *dev;
    unsigned int curdev;
    unsigned char s;
    unsigned char c;
    int valid;
    unsigned char data[16];
    unsigned int ptr;
    spinlock_t lock;
    void *dev_id;	/* native bus device */
    } hil_dev;
#[no_mangle]
unsafe extern "C" fn poll_finished() {
    static void poll_finished(void)
    {
    int down;
    int key;
    unsigned char scode;
    switch (hil_dev.data[0]) {
    case 0x40:
    down = (hil_dev.data[1] & 1) == 0;
    scode = hil_dev.data[1] >> 1;
    key = hphilkeyb_keycode[scode];
    input_report_key(hil_dev.dev, key, down);
    break;
    }
    hil_dev.curdev = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn handle_status(s: c_uchar, c: c_uchar) {
    static inline void handle_status(unsigned char s, unsigned char c)
    {
    if (c & 0x8) {
// End of block
    if (c & 0x10)
    poll_finished();
    } else {
    if (c & 0x10) {
    if (hil_dev.curdev)
    poll_finished();  /* just in case */
    hil_dev.curdev = c & 7;
    hil_dev.ptr = 0;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn handle_data(s: c_uchar, c: c_uchar) {
    static inline void handle_data(unsigned char s, unsigned char c)
    {
    if (hil_dev.curdev) {
    hil_dev.data[hil_dev.ptr++] = c;
    hil_dev.ptr &= 15;
    }
    }
// handle HIL interrupts
#[no_mangle]
unsafe extern "C" fn hil_interrupt(irq: c_int, handle: *mut c_void) -> irqreturn_t {
    static irqreturn_t hil_interrupt(int irq, void *handle)
    {
    unsigned char s, c;
    s = hil_status();
    c = hil_read_data();
    switch (s >> 4) {
    case 0x5:
    handle_status(s, c);
    break;
    case 0x6:
    handle_data(s, c);
    break;
    case 0x4:
    hil_dev.s = s;
    hil_dev.c = c;
    mb();
    hil_dev.valid = 1;
    break;
    }
    return IRQ_HANDLED;
    }
// send a command to the HIL
#[no_mangle]
unsafe extern "C" fn hil_do(cmd: c_uchar, data: *mut c_uchar, len: c_uint) {
    static void hil_do(unsigned char cmd, unsigned char *data, unsigned int len)
    {
    guard(spinlock_irqsave)(&hil_dev.lock);
    while (hil_busy())
// wait */;
    hil_command(cmd);
    while (len--) {
    while (hil_busy())
// wait */;
    hil_write_data(*(data++));
    }
    }
// initialize HIL
#[no_mangle]
unsafe extern "C" fn hil_keyb_init() -> c_int {
    static int hil_keyb_init(void)
    {
    unsigned char c;
    unsigned int i, kbid;
    wait_queue_head_t hil_wait;
    int err;
    if (hil_dev.dev)
    return -ENODEV; /* already initialized */
    init_waitqueue_head(&hil_wait);
    spin_lock_init(&hil_dev.lock);
    hil_dev.dev = input_allocate_device();
    if (!hil_dev.dev)
    return -ENOMEM;
    err = request_irq(HIL_IRQ, hil_interrupt, 0, "hil", hil_dev.dev_id);
    if (err) {
    printk(KERN_ERR "HIL: Can't get IRQ\n");
    goto err1;
    }
// Turn on interrupts
    hil_do(HIL_INTON, core::ptr::null_mut(), 0);
// Look for keyboards
    hil_dev.valid = 0;	/* clear any pending data */
    hil_do(HIL_READKBDSADR, core::ptr::null_mut(), 0);
    wait_event_interruptible_timeout(hil_wait, hil_dev.valid, 3 * HZ);
    if (!hil_dev.valid)
    printk(KERN_WARNING "HIL: timed out, assuming no keyboard present\n");
    c = hil_dev.c;
    hil_dev.valid = 0;
    if (c == 0) {
    kbid = -1;
    printk(KERN_WARNING "HIL: no keyboard present\n");
    } else {
    kbid = ffz(~c);
    printk(KERN_INFO "HIL: keyboard found at id %d\n", kbid);
    }
// set it to raw mode
    c = 0;
    hil_do(HIL_WRITEKBDSADR, &c, 1);
    for (i = 0; i < HIL_KEYCODES_SET1_TBLSIZE; i++)
    if (hphilkeyb_keycode[i] != KEY_RESERVED)
    __set_bit(hphilkeyb_keycode[i], hil_dev.dev.keybit);
    hil_dev.dev.evbit[0]	= BIT_MASK(EV_KEY) | BIT_MASK(EV_REP);
    hil_dev.dev.ledbit[0]	= BIT_MASK(LED_NUML) | BIT_MASK(LED_CAPSL) |
    BIT_MASK(LED_SCROLLL);
    hil_dev.dev.keycodemax	= HIL_KEYCODES_SET1_TBLSIZE;
    hil_dev.dev.keycodesize= sizeof(hphilkeyb_keycode[0]);
    hil_dev.dev.keycode	= hphilkeyb_keycode;
    hil_dev.dev.name	= "HIL keyboard";
    hil_dev.dev.phys	= "hpkbd/input0";
    hil_dev.dev.id.bustype	= BUS_HIL;
    hil_dev.dev.id.vendor	= PCI_VENDOR_ID_HP;
    hil_dev.dev.id.product	= 0x0001;
    hil_dev.dev.id.version	= 0x0010;
    err = input_register_device(hil_dev.dev);
    if (err) {
    printk(KERN_ERR "HIL: Can't register device\n");
    goto err2;
    }
    printk(KERN_INFO "input: %s, ID %d at 0x%08lx (irq %d) found and attached\n",
    hil_dev.dev.name, kbid, HILBASE, HIL_IRQ);
    return 0;
    err2:
    hil_do(HIL_INTOFF, core::ptr::null_mut(), 0);
    free_irq(HIL_IRQ, hil_dev.dev_id);
    err1:
    input_free_device(hil_dev.dev);
    hil_dev.dev = core::ptr::null_mut();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn hil_keyb_exit() {
    static void hil_keyb_exit(void)
    {
    if (HIL_IRQ)
    free_irq(HIL_IRQ, hil_dev.dev_id);
// Turn off interrupts
    hil_do(HIL_INTOFF, core::ptr::null_mut(), 0);
    input_unregister_device(hil_dev.dev);
    hil_dev.dev = core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn hil_probe_chip(dev: *mut parisc_device) -> int __init {
    static int __init hil_probe_chip(struct parisc_device *dev)
    {
// Only allow one HIL keyboard
    if (hil_dev.dev)
    return -ENODEV;
    if (!dev.irq) {
    printk(KERN_WARNING "HIL: IRQ not found for HIL bus at 0x%p\n",
    (void *)dev.hpa.start);
    return -ENODEV;
    }
    hil_base = dev.hpa.start;
    hil_irq  = dev.irq;
    hil_dev.dev_id = dev;
    printk(KERN_INFO "Found HIL bus at 0x%08lx, IRQ %d\n", hil_base, hil_irq);
    return hil_keyb_init();
    }
#[no_mangle]
unsafe extern "C" fn hil_remove_chip(dev: *mut parisc_device) -> void __exit {
    static void __exit hil_remove_chip(struct parisc_device *dev)
    {
    hil_keyb_exit();
    }
    static const struct parisc_device_id hil_tbl[] __initconst = {
    { HPHW_FIO, HVERSION_REV_ANY_ID, HVERSION_ANY_ID, 0x00073 },
    { 0, }
    };

// Disabled to avoid conflicts with the HP SDC HIL drivers
    MODULE_DEVICE_TABLE(parisc, hil_tbl);

    static struct parisc_driver hil_driver __refdata = {
    .name		= "hil",
    .id_table	= hil_tbl,
    .probe		= hil_probe_chip,
    .remove		= __exit_p(hil_remove_chip),
    };
#[no_mangle]
unsafe extern "C" fn hil_init() -> int __init {
    static int __init hil_init(void)
    {
    return register_parisc_driver(&hil_driver);
    }
#[no_mangle]
unsafe extern "C" fn hil_exit() -> void __exit {
    static void __exit hil_exit(void)
    {
    unregister_parisc_driver(&hil_driver);
    }

#[no_mangle]
unsafe extern "C" fn hil_init() -> int __init {
    static int __init hil_init(void)
    {
    int error;
// Only allow one HIL keyboard
    if (hil_dev.dev)
    return -EBUSY;
    if (!MACH_IS_HP300)
    return -ENODEV;
    if (!hwreg_present((void *)(HILBASE + HIL_DATA))) {
    printk(KERN_ERR "HIL: hardware register was not found\n");
    return -ENODEV;
    }
    if (!request_region(HILBASE + HIL_DATA, 2, "hil")) {
    printk(KERN_ERR "HIL: IOPORT region already used\n");
    return -EIO;
    }
    error = hil_keyb_init();
    if (error) {
    release_region(HILBASE + HIL_DATA, 2);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hil_exit() -> void __exit {
    static void __exit hil_exit(void)
    {
    hil_keyb_exit();
    release_region(HILBASE + HIL_DATA, 2);
    }

    module_init(hil_init);
    module_exit(hil_exit);
