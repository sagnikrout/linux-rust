//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/ibmasr.c
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


// SPDX-License-Identifier: GPL-1.0+
//
// IBM Automatic Server Restart driver.
//
// Copyright (c) 2005 Andrey Panin <pazke@donpac.ru>
//
// Based on driver written by Pete Reynolds.
// Copyright (c) IBM Corporation, 1998-2004.
//

    enum {
    ASMTYPE_UNKNOWN,
    ASMTYPE_TOPAZ,
    ASMTYPE_JASPER,
    ASMTYPE_PEARL,
    ASMTYPE_JUNIPER,
    ASMTYPE_SPRUCE,
    };
pub const TOPAZ_ASR_REG_OFFSET: c_int = 4;
pub const TOPAZ_ASR_TOGGLE: c_uint = 0x40;
pub const TOPAZ_ASR_DISABLE: c_uint = 0x80;
// PEARL ASR S/W REGISTER SUPERIO PORT ADDRESSES
pub const PEARL_BASE: c_uint = 0xe04;
pub const PEARL_WRITE: c_uint = 0xe06;
pub const PEARL_READ: c_uint = 0xe07;
pub const PEARL_ASR_DISABLE_MASK: c_uint = 0x80	/* bit 7: disable = 1, enable = 0 */;
pub const PEARL_ASR_TOGGLE_MASK: c_uint = 0x40	/* bit 6: 0, then 1, then 0 */;
// JASPER OFFSET FROM SIO BASE ADDR TO ASR S/W REGISTERS.
pub const JASPER_ASR_REG_OFFSET: c_uint = 0x38;
pub const JASPER_ASR_DISABLE_MASK: c_uint = 0x01	/* bit 0: disable = 1, enable = 0 */;
pub const JASPER_ASR_TOGGLE_MASK: c_uint = 0x02	/* bit 1: 0, then 1, then 0 */;
pub const JUNIPER_BASE_ADDRESS: c_uint = 0x54b	/* Base address of Juniper ASR */;
pub const JUNIPER_ASR_DISABLE_MASK: c_uint = 0x01	/* bit 0: disable = 1 enable = 0 */;
pub const JUNIPER_ASR_TOGGLE_MASK: c_uint = 0x02	/* bit 1: 0, then 1, then 0 */;
pub const SPRUCE_BASE_ADDRESS: c_uint = 0x118e	/* Base address of Spruce ASR */;
pub const SPRUCE_ASR_DISABLE_MASK: c_uint = 0x01	/* bit 1: disable = 1 enable = 0 */;
pub const SPRUCE_ASR_TOGGLE_MASK: c_uint = 0x02	/* bit 0: 0, then 1, then 0 */;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    static unsigned long asr_is_open;
    static char asr_expect_close;
    static unsigned int asr_type, asr_base, asr_length;
    static unsigned int asr_read_addr, asr_write_addr;
    static unsigned char asr_toggle_mask, asr_disable_mask;
    static DEFINE_SPINLOCK(asr_lock);
#[no_mangle]
unsafe extern "C" fn __asr_toggle() {
    static void __asr_toggle(void)
    {
    unsigned char reg;
    reg = inb(asr_read_addr);
    outb(reg & ~asr_toggle_mask, asr_write_addr);
    reg = inb(asr_read_addr);
    outb(reg | asr_toggle_mask, asr_write_addr);
    reg = inb(asr_read_addr);
    outb(reg & ~asr_toggle_mask, asr_write_addr);
    reg = inb(asr_read_addr);
    }
#[no_mangle]
unsafe extern "C" fn asr_toggle() {
    static void asr_toggle(void)
    {
    spin_lock(&asr_lock);
    __asr_toggle();
    spin_unlock(&asr_lock);
    }
#[no_mangle]
unsafe extern "C" fn asr_enable() {
    static void asr_enable(void)
    {
    unsigned char reg;
    spin_lock(&asr_lock);
    if (asr_type == ASMTYPE_TOPAZ) {
// asr_write_addr == asr_read_addr
    reg = inb(asr_read_addr);
    outb(reg & ~(TOPAZ_ASR_TOGGLE | TOPAZ_ASR_DISABLE),
    asr_read_addr);
    } else {
//
// First make sure the hardware timer is reset by toggling
// ASR hardware timer line.
//
    __asr_toggle();
    reg = inb(asr_read_addr);
    outb(reg & ~asr_disable_mask, asr_write_addr);
    }
    reg = inb(asr_read_addr);
    spin_unlock(&asr_lock);
    }
#[no_mangle]
unsafe extern "C" fn asr_disable() {
    static void asr_disable(void)
    {
    unsigned char reg;
    spin_lock(&asr_lock);
    reg = inb(asr_read_addr);
    if (asr_type == ASMTYPE_TOPAZ)
// asr_write_addr == asr_read_addr
    outb(reg | TOPAZ_ASR_TOGGLE | TOPAZ_ASR_DISABLE,
    asr_read_addr);
    else {
    outb(reg | asr_toggle_mask, asr_write_addr);
    reg = inb(asr_read_addr);
    outb(reg | asr_disable_mask, asr_write_addr);
    }
    reg = inb(asr_read_addr);
    spin_unlock(&asr_lock);
    }
#[no_mangle]
unsafe extern "C" fn asr_get_base_address() -> int __init {
    static int __init asr_get_base_address(void)
    {
    unsigned char low, high;
    const char *type = "";
    asr_length = 1;
    switch (asr_type) {
    case ASMTYPE_TOPAZ:
// SELECT SuperIO CHIP FOR QUERYING
    (WRITE 0x07 TO BOTH 0x2E and 0x2F) */
    outb(0x07, 0x2e);
    outb(0x07, 0x2f);
// SELECT AND READ THE HIGH-NIBBLE OF THE GPIO BASE ADDRESS
    outb(0x60, 0x2e);
    high = inb(0x2f);
// SELECT AND READ THE LOW-NIBBLE OF THE GPIO BASE ADDRESS
    outb(0x61, 0x2e);
    low = inb(0x2f);
    asr_base = (high << 16) | low;
    asr_read_addr = asr_write_addr =
    asr_base + TOPAZ_ASR_REG_OFFSET;
    asr_length = 5;
    break;
    case ASMTYPE_JASPER:
    type = "Jaspers ";

    u32 r;
// Suggested fix
    pdev = pci_get_bus_and_slot(0, DEVFN(0x1f, 0));
    if (pdev == core::ptr::null_mut())
    return -ENODEV;
    pci_read_config_dword(pdev, 0x58, &r);
    asr_base = r & 0xFFFE;
    pci_dev_put(pdev);

// FIXME: need to use pci_config_lock here,
    but it's not exported */
// spin_lock_irqsave(&pci_config_lock, flags);
// Select the SuperIO chip in the PCI I/O port register
    outl(0x8000f858, 0xcf8);
// BUS 0, Slot 1F, fnc 0, offset 58
//
// Read the base address for the SuperIO chip.
// Only the lower 16 bits are valid, but the address is word
// aligned so the last bit must be masked off.
//
    asr_base = inl(0xcfc) & 0xfffe;
// spin_unlock_irqrestore(&pci_config_lock, flags);

    asr_read_addr = asr_write_addr =
    asr_base + JASPER_ASR_REG_OFFSET;
    asr_toggle_mask = JASPER_ASR_TOGGLE_MASK;
    asr_disable_mask = JASPER_ASR_DISABLE_MASK;
    asr_length = JASPER_ASR_REG_OFFSET + 1;
    break;
    case ASMTYPE_PEARL:
    type = "Pearls ";
    asr_base = PEARL_BASE;
    asr_read_addr = PEARL_READ;
    asr_write_addr = PEARL_WRITE;
    asr_toggle_mask = PEARL_ASR_TOGGLE_MASK;
    asr_disable_mask = PEARL_ASR_DISABLE_MASK;
    asr_length = 4;
    break;
    case ASMTYPE_JUNIPER:
    type = "Junipers ";
    asr_base = JUNIPER_BASE_ADDRESS;
    asr_read_addr = asr_write_addr = asr_base;
    asr_toggle_mask = JUNIPER_ASR_TOGGLE_MASK;
    asr_disable_mask = JUNIPER_ASR_DISABLE_MASK;
    break;
    case ASMTYPE_SPRUCE:
    type = "Spruce's ";
    asr_base = SPRUCE_BASE_ADDRESS;
    asr_read_addr = asr_write_addr = asr_base;
    asr_toggle_mask = SPRUCE_ASR_TOGGLE_MASK;
    asr_disable_mask = SPRUCE_ASR_DISABLE_MASK;
    break;
    }
    if (!request_region(asr_base, asr_length, "ibmasr")) {
    pr_err("address %#x already in use\n", asr_base);
    return -EBUSY;
    }
    pr_info("found %sASR @ addr %#x\n", type, asr_base);
    return 0;
    }
    static ssize_t asr_write(struct file *file, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    if (count) {
    if (!nowayout) {
    size_t i;
// In case it was set long ago
    asr_expect_close = 0;
    for (i = 0; i != count; i++) {
    char c;
    if (get_user(c, buf + i))
    return -EFAULT;
    if (c == 'V')
    asr_expect_close = 42;
    }
    }
    asr_toggle();
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn asr_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long asr_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    static const struct watchdog_info ident = {
    .options =	WDIOF_KEEPALIVEPING |
    WDIOF_MAGICCLOSE,
    .identity =	"IBM ASR",
    };
    void __user *argp = (void __user *)arg;
    int __user *p = argp;
    int heartbeat;
    switch (cmd) {
    case WDIOC_GETSUPPORT:
    return copy_to_user(argp, &ident, sizeof(ident)) ? -EFAULT : 0;
    case WDIOC_GETSTATUS:
    case WDIOC_GETBOOTSTATUS:
    return put_user(0, p);
    case WDIOC_SETOPTIONS:
    {
    int new_options, retval = -EINVAL;
    if (get_user(new_options, p))
    return -EFAULT;
    if (new_options & WDIOS_DISABLECARD) {
    asr_disable();
    retval = 0;
    }
    if (new_options & WDIOS_ENABLECARD) {
    asr_enable();
    asr_toggle();
    retval = 0;
    }
    return retval;
    }
    case WDIOC_KEEPALIVE:
    asr_toggle();
    return 0;
//
// The hardware has a fixed timeout value, so no WDIOC_SETTIMEOUT
// and WDIOC_GETTIMEOUT always returns 256.
//
    case WDIOC_GETTIMEOUT:
    heartbeat = 256;
    return put_user(heartbeat, p);
    default:
    return -ENOTTY;
    }
    }
#[no_mangle]
unsafe extern "C" fn asr_open(inode: *mut inode, file: *mut file) -> c_int {
    static int asr_open(struct inode *inode, struct file *file)
    {
    if (test_and_set_bit(0, &asr_is_open))
    return -EBUSY;
    asr_toggle();
    asr_enable();
    return stream_open(inode, file);
    }
#[no_mangle]
unsafe extern "C" fn asr_release(inode: *mut inode, file: *mut file) -> c_int {
    static int asr_release(struct inode *inode, struct file *file)
    {
    if (asr_expect_close == 42)
    asr_disable();
    else {
    pr_crit("unexpected close, not stopping watchdog!\n");
    asr_toggle();
    }
    clear_bit(0, &asr_is_open);
    asr_expect_close = 0;
    return 0;
    }
    static const struct file_operations asr_fops = {
    .owner =		THIS_MODULE,
    .write =		asr_write,
    .unlocked_ioctl =	asr_ioctl,
    .compat_ioctl =		compat_ptr_ioctl,
    .open =			asr_open,
    .release =		asr_release,
    };
    static struct miscdevice asr_miscdev = {
    .minor =	WATCHDOG_MINOR,
    .name =		"watchdog",
    .fops =		&asr_fops,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmasr_id {
    pub desc: *const c_char,
    pub type: c_int,
}

    static struct ibmasr_id ibmasr_id_table[] __initdata = {
    { "IBM Automatic Server Restart - eserver xSeries 220", ASMTYPE_TOPAZ },
    { "IBM Automatic Server Restart - Machine Type 8673", ASMTYPE_PEARL },
    { "IBM Automatic Server Restart - Machine Type 8480", ASMTYPE_JASPER },
    { "IBM Automatic Server Restart - Machine Type 8482", ASMTYPE_JUNIPER },
    { "IBM Automatic Server Restart - Machine Type 8648", ASMTYPE_SPRUCE },
    { core::ptr::null_mut() }
    };
#[no_mangle]
unsafe extern "C" fn ibmasr_init() -> int __init {
    static int __init ibmasr_init(void)
    {
    struct ibmasr_id *id;
    int rc;
    for (id = ibmasr_id_table; id.desc; id++) {
    if (dmi_find_device(DMI_DEV_TYPE_OTHER, id.desc, core::ptr::null_mut())) {
    asr_type = id.type;
    break;
    }
    }
    if (!asr_type)
    return -ENODEV;
    rc = asr_get_base_address();
    if (rc)
    return rc;
    rc = misc_register(&asr_miscdev);
    if (rc < 0) {
    release_region(asr_base, asr_length);
    pr_err("failed to register misc device\n");
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ibmasr_exit() -> void __exit {
    static void __exit ibmasr_exit(void)
    {
    if (!nowayout)
    asr_disable();
    misc_deregister(&asr_miscdev);
    release_region(asr_base, asr_length);
    }
    module_init(ibmasr_init);
    module_exit(ibmasr_exit);
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    MODULE_DESCRIPTION("IBM Automatic Server Restart driver");
    MODULE_AUTHOR("Andrey Panin");
    MODULE_LICENSE("GPL");
