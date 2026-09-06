//! Automatically rewritten from C to Rust
//! Source: drivers/char/pc8736x_gpio.c
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
// linux/drivers/char/pc8736x_gpio.c
    National Semiconductor PC8736x GPIO driver.  Allows a user space
    process to play with the GPIO pins.
    Copyright (c) 2005,2006 Jim Cromie <jim.cromie@gmail.com>
    adapted from linux/drivers/char/scx200_gpio.c
    Copyright (c) 2001,2002 Christer Weinigel <wingel@nano-system.com>,
//

    MODULE_AUTHOR("Jim Cromie <jim.cromie@gmail.com>");
    MODULE_DESCRIPTION("NatSemi/Winbond PC-8736x GPIO Pin Driver");
    MODULE_LICENSE("GPL");
    static int major;		/* default to dynamic major */
    module_param(major, int, 0);
    MODULE_PARM_DESC(major, "Major device number");
    static DEFINE_MUTEX(pc8736x_gpio_config_lock);
    static unsigned pc8736x_gpio_base;
    static u8 pc8736x_gpio_shadow[4];
pub const SIO_BASE1: c_uint = 0x2E	/* 1st command-reg to check */;
pub const SIO_BASE2: c_uint = 0x4E	/* alt command-reg to check */;
pub const SIO_SID: c_uint = 0x20	/* SuperI/O ID Register */;
pub const SIO_SID_PC87365: c_uint = 0xe5	/* Expected value in ID Register for PC87365 */;
pub const SIO_SID_PC87366: c_uint = 0xe9	/* Expected value in ID Register for PC87366 */;
pub const SIO_CF1: c_uint = 0x21	/* chip config, bit0 is chip enable */;

pub const SIO_UNIT_SEL: c_uint = 0x7	/* unit select reg */;
pub const SIO_UNIT_ACT: c_uint = 0x30	/* unit enable */;
pub const SIO_GPIO_UNIT: c_uint = 0x7	/* unit number of GPIO */;
pub const SIO_VLM_UNIT: c_uint = 0x0D;
pub const SIO_TMS_UNIT: c_uint = 0x0E;
// config-space addrs to read/write each unit's runtime addr
pub const SIO_BASE_HADDR: c_uint = 0x60;
pub const SIO_BASE_LADDR: c_uint = 0x61;
// GPIO config-space pin-control addresses
pub const SIO_GPIO_PIN_SELECT: c_uint = 0xF0;
pub const SIO_GPIO_PIN_CONFIG: c_uint = 0xF1;
pub const SIO_GPIO_PIN_EVENT: c_uint = 0xF2;
    let mut superio_cmd: static unsigned char = 0;
    static unsigned char selected_device = 0xFF;	/* bogus start val */
// GPIO port runtime access, functionality
    static int port_offset[] = { 0, 4, 8, 10 };	/* non-uniform offsets ! */
// static int event_capable[] = { 1, 1, 0, 0 };   ports 2,3 are hobbled
pub const PORT_OUT: c_int = 0;
pub const PORT_IN: c_int = 1;
pub const PORT_EVT_EN: c_int = 2;
pub const PORT_EVT_STST: c_int = 3;
    static struct platform_device *pdev;  /* use in dev_*() */
#[no_mangle]
pub unsafe extern "C" fn superio_outb(addr: c_int, val: c_int) {
    static inline void superio_outb(int addr, int val)
    {
    outb_p(addr, superio_cmd);
    outb_p(val, superio_cmd + 1);
    }
#[no_mangle]
pub unsafe extern "C" fn superio_inb(addr: c_int) -> c_int {
    static inline int superio_inb(int addr)
    {
    outb_p(addr, superio_cmd);
    return inb_p(superio_cmd + 1);
    }
#[no_mangle]
unsafe extern "C" fn pc8736x_superio_present() -> c_int {
    static int pc8736x_superio_present(void)
    {
    int id;
// try the 2 possible values, read a hardware reg to verify
    superio_cmd = SIO_BASE1;
    id = superio_inb(SIO_SID);
    if (id == SIO_SID_PC87365 || id == SIO_SID_PC87366)
    return superio_cmd;
    superio_cmd = SIO_BASE2;
    id = superio_inb(SIO_SID);
    if (id == SIO_SID_PC87365 || id == SIO_SID_PC87366)
    return superio_cmd;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn device_select(devldn: unsigned) {
    static void device_select(unsigned devldn)
    {
    superio_outb(SIO_UNIT_SEL, devldn);
    selected_device = devldn;
    }
#[no_mangle]
unsafe extern "C" fn select_pin(iminor: unsigned) {
    static void select_pin(unsigned iminor)
    {
// select GPIO port/pin from device minor number
    device_select(SIO_GPIO_UNIT);
    superio_outb(SIO_GPIO_PIN_SELECT,
    ((iminor << 1) & 0xF0) | (iminor & 0x7));
    }
    static inline u32 pc8736x_gpio_configure_fn(unsigned index, u32 mask, u32 bits,
    u32 func_slct)
    {
    u32 config, new_config;
    mutex_lock(&pc8736x_gpio_config_lock);
    device_select(SIO_GPIO_UNIT);
    select_pin(index);
// read current config value
    config = superio_inb(func_slct);
// set new config
    new_config = (config & mask) | bits;
    superio_outb(func_slct, new_config);
    mutex_unlock(&pc8736x_gpio_config_lock);
    return config;
    }
#[no_mangle]
unsafe extern "C" fn pc8736x_gpio_configure(index: unsigned, mask: u32, bits: u32) -> u32 {
    static u32 pc8736x_gpio_configure(unsigned index, u32 mask, u32 bits)
    {
    return pc8736x_gpio_configure_fn(index, mask, bits,
    SIO_GPIO_PIN_CONFIG);
    }
#[no_mangle]
unsafe extern "C" fn pc8736x_gpio_get(minor: unsigned) -> c_int {
    static int pc8736x_gpio_get(unsigned minor)
    {
    int port, bit, val;
    port = minor >> 3;
    bit = minor & 7;
    val = inb_p(pc8736x_gpio_base + port_offset[port] + PORT_IN);
    val >>= bit;
    val &= 1;
    dev_dbg(&pdev.dev, "_gpio_get(%d from %x bit %d) == val %d\n",
    minor, pc8736x_gpio_base + port_offset[port] + PORT_IN, bit,
    val);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn pc8736x_gpio_set(minor: unsigned, val: c_int) {
    static void pc8736x_gpio_set(unsigned minor, int val)
    {
    int port, bit, curval;
    minor &= 0x1f;
    port = minor >> 3;
    bit = minor & 7;
    curval = inb_p(pc8736x_gpio_base + port_offset[port] + PORT_OUT);
    dev_dbg(&pdev.dev, "addr:%x cur:%x bit-pos:%d cur-bit:%x + new:%d . bit-new:%d\n",
    pc8736x_gpio_base + port_offset[port] + PORT_OUT,
    curval, bit, (curval & ~(1 << bit)), val, (val << bit));
    val = (curval & ~(1 << bit)) | (val << bit);
    dev_dbg(&pdev.dev, "gpio_set(minor:%d port:%d bit:%d)"
    " %2x . %2x\n", minor, port, bit, curval, val);
    outb_p(val, pc8736x_gpio_base + port_offset[port] + PORT_OUT);
    curval = inb_p(pc8736x_gpio_base + port_offset[port] + PORT_OUT);
    val = inb_p(pc8736x_gpio_base + port_offset[port] + PORT_IN);
    dev_dbg(&pdev.dev, "wrote %x, read: %x\n", curval, val);
    pc8736x_gpio_shadow[port] = val;
    }
#[no_mangle]
unsafe extern "C" fn pc8736x_gpio_current(minor: unsigned) -> c_int {
    static int pc8736x_gpio_current(unsigned minor)
    {
    int port, bit;
    minor &= 0x1f;
    port = minor >> 3;
    bit = minor & 7;
    return ((pc8736x_gpio_shadow[port] >> bit) & 0x01);
    }
#[no_mangle]
unsafe extern "C" fn pc8736x_gpio_change(index: unsigned) {
    static void pc8736x_gpio_change(unsigned index)
    {
    pc8736x_gpio_set(index, !pc8736x_gpio_current(index));
    }
    static struct nsc_gpio_ops pc8736x_gpio_ops = {
    .owner		= THIS_MODULE,
    .gpio_config	= pc8736x_gpio_configure,
    .gpio_dump	= nsc_gpio_dump,
    .gpio_get	= pc8736x_gpio_get,
    .gpio_set	= pc8736x_gpio_set,
    .gpio_change	= pc8736x_gpio_change,
    .gpio_current	= pc8736x_gpio_current
    };
#[no_mangle]
unsafe extern "C" fn pc8736x_gpio_open(inode: *mut inode, file: *mut file) -> c_int {
    static int pc8736x_gpio_open(struct inode *inode, struct file *file)
    {
    let mut m: unsigned = iminor(inode);
    file.private_data = &pc8736x_gpio_ops;
    dev_dbg(&pdev.dev, "open %d\n", m);
    if (m >= PC8736X_GPIO_CT)
    return -EINVAL;
    return nonseekable_open(inode, file);
    }
    static const struct file_operations pc8736x_gpio_fileops = {
    .owner	= THIS_MODULE,
    .open	= pc8736x_gpio_open,
    .write	= nsc_gpio_write,
    .read	= nsc_gpio_read,
    };
#[no_mangle]
unsafe extern "C" fn pc8736x_init_shadow() -> void __init {
    static void __init pc8736x_init_shadow(void)
    {
    int port;
// read the current values driven on the GPIO signals
    for (port = 0; port < 4; ++port)
    pc8736x_gpio_shadow[port]
    = inb_p(pc8736x_gpio_base + port_offset[port]
    + PORT_OUT);
    }
    static struct cdev pc8736x_gpio_cdev;
#[no_mangle]
unsafe extern "C" fn pc8736x_gpio_init() -> int __init {
    static int __init pc8736x_gpio_init(void)
    {
    int rc;
    dev_t devid;
    pdev = platform_device_alloc(DEVNAME, 0);
    if (!pdev)
    return -ENOMEM;
    rc = platform_device_add(pdev);
    if (rc) {
    rc = -ENODEV;
    goto undo_platform_dev_alloc;
    }
    dev_info(&pdev.dev, "NatSemi pc8736x GPIO Driver Initializing\n");
    if (!pc8736x_superio_present()) {
    rc = -ENODEV;
    dev_err(&pdev.dev, "no device found\n");
    goto undo_platform_dev_add;
    }
    pc8736x_gpio_ops.dev = &pdev.dev;
// Verify that chip and it's GPIO unit are both enabled.
    My BIOS does this, so I take minimum action here
//
    rc = superio_inb(SIO_CF1);
    if (!(rc & 0x01)) {
    rc = -ENODEV;
    dev_err(&pdev.dev, "device not enabled\n");
    goto undo_platform_dev_add;
    }
    device_select(SIO_GPIO_UNIT);
    if (!superio_inb(SIO_UNIT_ACT)) {
    rc = -ENODEV;
    dev_err(&pdev.dev, "GPIO unit not enabled\n");
    goto undo_platform_dev_add;
    }
// read the GPIO unit base addr that chip responds to
    pc8736x_gpio_base = (superio_inb(SIO_BASE_HADDR) << 8
    | superio_inb(SIO_BASE_LADDR));
    if (!request_region(pc8736x_gpio_base, PC8736X_GPIO_RANGE, DEVNAME)) {
    rc = -ENODEV;
    dev_err(&pdev.dev, "GPIO ioport %x busy\n",
    pc8736x_gpio_base);
    goto undo_platform_dev_add;
    }
    dev_info(&pdev.dev, "GPIO ioport %x reserved\n", pc8736x_gpio_base);
    if (major) {
    devid = MKDEV(major, 0);
    rc = register_chrdev_region(devid, PC8736X_GPIO_CT, DEVNAME);
    } else {
    rc = alloc_chrdev_region(&devid, 0, PC8736X_GPIO_CT, DEVNAME);
    major = MAJOR(devid);
    }
    if (rc < 0) {
    dev_err(&pdev.dev, "register-chrdev failed: %d\n", rc);
    goto undo_request_region;
    }
    if (!major) {
    major = rc;
    dev_dbg(&pdev.dev, "got dynamic major %d\n", major);
    }
    pc8736x_init_shadow();
// ignore minor errs, and succeed
    cdev_init(&pc8736x_gpio_cdev, &pc8736x_gpio_fileops);
    cdev_add(&pc8736x_gpio_cdev, devid, PC8736X_GPIO_CT);
    return 0;
    undo_request_region:
    release_region(pc8736x_gpio_base, PC8736X_GPIO_RANGE);
    undo_platform_dev_add:
    platform_device_del(pdev);
    undo_platform_dev_alloc:
    platform_device_put(pdev);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn pc8736x_gpio_cleanup() -> void __exit {
    static void __exit pc8736x_gpio_cleanup(void)
    {
    dev_dbg(&pdev.dev, "cleanup\n");
    cdev_del(&pc8736x_gpio_cdev);
    unregister_chrdev_region(MKDEV(major,0), PC8736X_GPIO_CT);
    release_region(pc8736x_gpio_base, PC8736X_GPIO_RANGE);
    platform_device_unregister(pdev);
    }
    module_init(pc8736x_gpio_init);
    module_exit(pc8736x_gpio_cleanup);
