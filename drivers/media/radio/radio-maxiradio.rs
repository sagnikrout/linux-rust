//! Automatically rewritten from C to Rust
//! Source: drivers/media/radio/radio-maxiradio.c
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
// Guillemot Maxi Radio FM 2000 PCI radio card driver for Linux
// (C) 2001 Dimitromanolakis Apostolos <apdim@grecian.net>
//
// Based in the radio Maestro PCI driver. Actually it uses the same chip
// for radio but different pci controller.
//
// I didn't have any specs I reversed engineered the protocol from
// the windows driver (radio.dll).
//
// The card uses the TEA5757 chip that includes a search function but it
// is useless as I haven't found any way to read back the frequency. If
// anybody does please mail me.
//
// For the pdf file see:
// http://www.nxp.com/acrobat_download2/expired_datasheets/TEA5757_5759_3.pdf
//
// CHANGES:
// 0.75b
// - better pci interface thanks to Francois Romieu <romieu@cogenit.fr>
//
// 0.75      Sun Feb  4 22:51:27 EET 2001
// - tiding up
// - removed support for multiple devices as it didn't work anyway
//
// BUGS:
// - card unmutes if you change frequency
//
// (c) 2006, 2007 by Mauro Carvalho Chehab <mchehab@kernel.org>:
// - Conversion to V4L2 API
// - Uses video_ioctl2 for parsing and to add debug support
//

    MODULE_AUTHOR("Dimitromanolakis Apostolos, apdim@grecian.net");
    MODULE_DESCRIPTION("Radio driver for the Guillemot Maxi Radio FM2000.");
    MODULE_LICENSE("GPL");
    MODULE_VERSION("1.0.0");
    let mut radio_nr: static int = -1;
    module_param(radio_nr, int, 0644);
    MODULE_PARM_DESC(radio_nr, "Radio device number");
// TEA5757 pin mappings
    let mut clk: static int = 1, data = 2, wren = 4, mo_st = 8, power = 16;
    let mut maxiradio_instance: static atomic_t = ATOMIC_INIT(0);
pub const PCI_VENDOR_ID_GUILLEMOT: c_uint = 0x5046;
pub const PCI_DEVICE_ID_GUILLEMOT_MAXIRADIO: c_uint = 0x1001;
    struct maxiradio
    {
    struct snd_tea575x tea;
    struct v4l2_device v4l2_dev;
    struct pci_dev *pdev;
    u16	io;	/* base of radio io */
    };
    static inline struct maxiradio *to_maxiradio(struct v4l2_device *v4l2_dev)
    {
    return container_of(v4l2_dev, struct maxiradio, v4l2_dev);
    }
#[no_mangle]
unsafe extern "C" fn maxiradio_tea575x_set_pins(tea: *mut snd_tea575x, pins: u8) {
    static void maxiradio_tea575x_set_pins(struct snd_tea575x *tea, u8 pins)
    {
    struct maxiradio *dev = tea.private_data;
    let mut bits: u8 = 0;
    bits |= (pins & TEA575X_DATA) ? data : 0;
    bits |= (pins & TEA575X_CLK)  ? clk  : 0;
    bits |= (pins & TEA575X_WREN) ? wren : 0;
    bits |= power;
    outb(bits, dev.io);
    }
// Note: this card cannot read out the data of the shift registers,
    only the mono/stereo pin works. */
#[no_mangle]
unsafe extern "C" fn maxiradio_tea575x_get_pins(tea: *mut snd_tea575x) -> u8 {
    static u8 maxiradio_tea575x_get_pins(struct snd_tea575x *tea)
    {
    struct maxiradio *dev = tea.private_data;
    let mut bits: u8 = inb(dev.io);
    return  ((bits & data) ? TEA575X_DATA : 0) |
    ((bits & mo_st) ? TEA575X_MOST : 0);
    }
#[no_mangle]
unsafe extern "C" fn maxiradio_tea575x_set_direction(tea: *mut snd_tea575x, output: bool) {
    static void maxiradio_tea575x_set_direction(struct snd_tea575x *tea, bool output)
    {
    }
    static const struct snd_tea575x_ops maxiradio_tea_ops = {
    .set_pins = maxiradio_tea575x_set_pins,
    .get_pins = maxiradio_tea575x_get_pins,
    .set_direction = maxiradio_tea575x_set_direction,
    };
    static int maxiradio_probe(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct maxiradio *dev;
    struct v4l2_device *v4l2_dev;
    let mut retval: c_int = -ENOMEM;
    dev = kzalloc_obj(*dev);
    if (dev == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "not enough memory\n");
    return -ENOMEM;
    }
    v4l2_dev = &dev.v4l2_dev;
    v4l2_device_set_name(v4l2_dev, "maxiradio", &maxiradio_instance);
    retval = v4l2_device_register(&pdev.dev, v4l2_dev);
    if (retval < 0) {
    v4l2_err(v4l2_dev, "Could not register v4l2_device\n");
    goto errfr;
    }
    dev.tea.private_data = dev;
    dev.tea.ops = &maxiradio_tea_ops;
// The data pin cannot be read. This may be a hardware limitation, or
    we just don't know how to read it. */
    dev.tea.cannot_read_data = true;
    dev.tea.v4l2_dev = v4l2_dev;
    dev.tea.radio_nr = radio_nr;
    strscpy(dev.tea.card, "Maxi Radio FM2000", sizeof(dev.tea.card));
    retval = -ENODEV;
    if (!request_region(pci_resource_start(pdev, 0),
    pci_resource_len(pdev, 0), v4l2_dev.name)) {
    dev_err(&pdev.dev, "can't reserve I/O ports\n");
    goto err_hdl;
    }
    if (pci_enable_device(pdev))
    goto err_out_free_region;
    dev.io = pci_resource_start(pdev, 0);
    if (snd_tea575x_init(&dev.tea, THIS_MODULE)) {
    printk(KERN_ERR "radio-maxiradio: Unable to detect TEA575x tuner\n");
    goto err_out_free_region;
    }
    return 0;
    err_out_free_region:
    release_region(pci_resource_start(pdev, 0), pci_resource_len(pdev, 0));
    err_hdl:
    v4l2_device_unregister(v4l2_dev);
    errfr:
    kfree(dev);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn maxiradio_remove(pdev: *mut pci_dev) {
    static void maxiradio_remove(struct pci_dev *pdev)
    {
    struct v4l2_device *v4l2_dev = pci_get_drvdata(pdev);
    struct maxiradio *dev = to_maxiradio(v4l2_dev);
    snd_tea575x_exit(&dev.tea);
// Turn off power
    outb(0, dev.io);
    v4l2_device_unregister(v4l2_dev);
    release_region(pci_resource_start(pdev, 0), pci_resource_len(pdev, 0));
    kfree(dev);
    }
    static const struct pci_device_id maxiradio_pci_tbl[] = {
    { PCI_VENDOR_ID_GUILLEMOT, PCI_DEVICE_ID_GUILLEMOT_MAXIRADIO,
    PCI_ANY_ID, PCI_ANY_ID, },
    { 0 }
    };
    MODULE_DEVICE_TABLE(pci, maxiradio_pci_tbl);
    static struct pci_driver maxiradio_driver = {
    .name		= "radio-maxiradio",
    .id_table	= maxiradio_pci_tbl,
    .probe		= maxiradio_probe,
    .remove		= maxiradio_remove,
    };
    module_pci_driver(maxiradio_driver);
