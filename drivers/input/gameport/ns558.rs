//! Automatically rewritten from C to Rust
//! Source: drivers/input/gameport/ns558.c
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
// Copyright (c) 1999-2001 Vojtech Pavlik
// Copyright (c) 1999 Brian Gerst
//
// NS558 based standard IBM game port driver for Linux
//

    MODULE_AUTHOR("Vojtech Pavlik <vojtech@ucw.cz>");
    MODULE_DESCRIPTION("Classic gameport (ISA/PnP) driver");
    MODULE_LICENSE("GPL");
    static int ns558_isa_portlist[] = { 0x201, 0x200, 0x202, 0x203, 0x204, 0x205, 0x207, 0x209,
    0x20b, 0x20c, 0x20e, 0x20f, 0x211, 0x219, 0x101, 0 };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ns558 {
    pub type: c_int,
    pub io: c_int,
    pub size: c_int,
    pub dev: *mut pnp_dev,
    pub gameport: *mut gameport,
    pub node: list_head,
}

    static LIST_HEAD(ns558_list);
//
// ns558_isa_probe() tries to find an isa gameport at the
// specified address, and also checks for mirrors.
// A joystick must be attached for this to work.
//
#[no_mangle]
unsafe extern "C" fn ns558_isa_probe(io: c_int) -> c_int {
    static int ns558_isa_probe(int io)
    {
    int i, j, b;
    unsigned char c, u, v;
    struct ns558 *ns558;
    struct gameport *port;
//
// No one should be using this address.
//
    if (!request_region(io, 1, "ns558-isa"))
    return -EBUSY;
//
// We must not be able to write arbitrary values to the port.
// The lower two axis bits must be 1 after a write.
//
    c = inb(io);
    outb(~c & ~3, io);
    if (~(u = v = inb(io)) & 3) {
    outb(c, io);
    release_region(io, 1);
    return -ENODEV;
    }
//
// After a trigger, there must be at least some bits changing.
//
    for (i = 0; i < 1000; i++) v &= inb(io);
    if (u == v) {
    outb(c, io);
    release_region(io, 1);
    return -ENODEV;
    }
    msleep(3);
//
// After some time (4ms) the axes shouldn't change anymore.
//
    u = inb(io);
    for (i = 0; i < 1000; i++)
    if ((u ^ inb(io)) & 0xf) {
    outb(c, io);
    release_region(io, 1);
    return -ENODEV;
    }
//
// And now find the number of mirrors of the port.
//
    for (i = 1; i < 5; i++) {
    release_region(io & (-1 << (i - 1)), (1 << (i - 1)));
    if (!request_region(io & (-1 << i), (1 << i), "ns558-isa"))
    break;				/* Don't disturb anyone */
    outb(0xff, io & (-1 << i));
    for (j = b = 0; j < 1000; j++)
    if (inb(io & (-1 << i)) != inb((io & (-1 << i)) + (1 << i) - 1)) b++;
    msleep(3);
    if (b > 300) {				/* We allow 30% difference */
    release_region(io & (-1 << i), (1 << i));
    break;
    }
    }
    i--;
    if (i != 4) {
    if (!request_region(io & (-1 << i), (1 << i), "ns558-isa"))
    return -EBUSY;
    }
    ns558 = kzalloc_obj(*ns558);
    port = gameport_allocate_port();
    if (!ns558 || !port) {
    printk(KERN_ERR "ns558: Memory allocation failed.\n");
    release_region(io & (-1 << i), (1 << i));
    kfree(ns558);
    gameport_free_port(port);
    return -ENOMEM;
    }
    ns558.io = io;
    ns558.size = 1 << i;
    ns558.gameport = port;
    port.io = io;
    gameport_set_name(port, "NS558 ISA Gameport");
    gameport_set_phys(port, "isa%04x/gameport0", io & (-1 << i));
    gameport_register_port(port);
    list_add(&ns558.node, &ns558_list);
    return 0;
    }

    static const struct pnp_device_id pnp_devids[] = {
    { .id = "@P@0001" }, /* ALS 100 */
    { .id = "@P@0020" }, /* ALS 200 */
    { .id = "@P@1001" }, /* ALS 100+ */
    { .id = "@P@2001" }, /* ALS 120 */
    { .id = "ASB16fd" }, /* AdLib NSC16 */
    { .id = "AZT3001" }, /* AZT1008 */
    { .id = "CDC0001" }, /* Opl3-SAx */
    { .id = "CSC0001" }, /* CS4232 */
    { .id = "CSC000f" }, /* CS4236 */
    { .id = "CSC0101" }, /* CS4327 */
    { .id = "CTL7001" }, /* SB16 */
    { .id = "CTL7002" }, /* AWE64 */
    { .id = "CTL7005" }, /* Vibra16 */
    { .id = "ENS2020" }, /* SoundscapeVIVO */
    { .id = "ESS0001" }, /* ES1869 */
    { .id = "ESS0005" }, /* ES1878 */
    { .id = "ESS6880" }, /* ES688 */
    { .id = "IBM0012" }, /* CS4232 */
    { .id = "OPT0001" }, /* OPTi Audio16 */
    { .id = "YMH0006" }, /* Opl3-SA */
    { .id = "YMH0022" }, /* Opl3-SAx */
    { .id = "PNPb02f" }, /* Generic */
    { }
    };
    MODULE_DEVICE_TABLE(pnp, pnp_devids);
#[no_mangle]
unsafe extern "C" fn ns558_pnp_probe(dev: *mut pnp_dev, did: *const pnp_device_id) -> c_int {
    static int ns558_pnp_probe(struct pnp_dev *dev, const struct pnp_device_id *did)
    {
    int ioport, iolen;
    struct ns558 *ns558;
    struct gameport *port;
    if (!pnp_port_valid(dev, 0)) {
    printk(KERN_WARNING "ns558: No i/o ports on a gameport? Weird\n");
    return -ENODEV;
    }
    ioport = pnp_port_start(dev, 0);
    iolen = pnp_port_len(dev, 0);
    if (!request_region(ioport, iolen, "ns558-pnp"))
    return -EBUSY;
    ns558 = kzalloc_obj(*ns558);
    port = gameport_allocate_port();
    if (!ns558 || !port) {
    printk(KERN_ERR "ns558: Memory allocation failed\n");
    kfree(ns558);
    gameport_free_port(port);
    return -ENOMEM;
    }
    ns558.io = ioport;
    ns558.size = iolen;
    ns558.dev = dev;
    ns558.gameport = port;
    gameport_set_name(port, "NS558 PnP Gameport");
    gameport_set_phys(port, "pnp%s/gameport0", dev_name(&dev.dev));
    port.dev.parent = &dev.dev;
    port.io = ioport;
    gameport_register_port(port);
    list_add_tail(&ns558.node, &ns558_list);
    return 0;
    }
    static struct pnp_driver ns558_pnp_driver = {
    .name		= "ns558",
    .id_table	= pnp_devids,
    .probe		= ns558_pnp_probe,
    };

    static struct pnp_driver ns558_pnp_driver;

#[no_mangle]
unsafe extern "C" fn ns558_init() -> int __init {
    static int __init ns558_init(void)
    {
    let mut i: c_int = 0;
    int error;
    error = pnp_register_driver(&ns558_pnp_driver);
    if (error && error != -ENODEV)	/* should be ENOSYS really */
    return error;
//
// Probe ISA ports after PnP, so that PnP ports that are already
// enabled get detected as PnP. This may be suboptimal in multi-device
// configurations, but saves hassle with simple setups.
//
    while (ns558_isa_portlist[i])
    ns558_isa_probe(ns558_isa_portlist[i++]);
    return list_empty(&ns558_list) && error ? -ENODEV : 0;
    }
#[no_mangle]
unsafe extern "C" fn ns558_exit() -> void __exit {
    static void __exit ns558_exit(void)
    {
    struct ns558 *ns558, *safe;
    list_for_each_entry_safe(ns558, safe, &ns558_list, node) {
    gameport_unregister_port(ns558.gameport);
    release_region(ns558.io & ~(ns558.size - 1), ns558.size);
    kfree(ns558);
    }
    pnp_unregister_driver(&ns558_pnp_driver);
    }
    module_init(ns558_init);
    module_exit(ns558_exit);
