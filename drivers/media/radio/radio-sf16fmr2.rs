//! Automatically rewritten from C to Rust
//! Source: drivers/media/radio/radio-sf16fmr2.c
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
// SF16-FMR2 and SF16-FMD2 radio driver for Linux
// Copyright (c) 2011 Ondrej Zary
//
// Original driver was (c) 2000-2002 Ziglio Frediano, freddy77@angelfire.com
// but almost nothing remained here after conversion to generic TEA575x
// implementation
//

    MODULE_AUTHOR("Ondrej Zary");
    MODULE_DESCRIPTION("MediaForte SF16-FMR2 and SF16-FMD2 FM radio card driver");
    MODULE_LICENSE("GPL");
// these cards can only use two different ports (0x384 and 0x284)
pub const FMR2_MAX: c_int = 2;
    static int radio_nr[FMR2_MAX] = { [0 ... (FMR2_MAX - 1)] = -1 };
    module_param_array(radio_nr, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(radio_nr, "Radio device numbers");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fmr2 {
    pub io: c_int,
    pub v4l2_dev: v4l2_device,
    pub tea: snd_tea575x,
    pub volume: *mut v4l2_ctrl,
    pub balance: *mut v4l2_ctrl,
    pub is_fmd2: bool,
}

    static int num_fmr2_cards;
    static struct fmr2 *fmr2_cards[FMR2_MAX];
    static bool isa_registered;
    static bool pnp_registered;
// the port is hardwired on SF16-FMR2
pub const FMR2_PORT: c_uint = 0x384;
// TEA575x tuner pins

// PT2254A/TC9154A volume control pins

// volume control presence pin

#[no_mangle]
unsafe extern "C" fn fmr2_tea575x_set_pins(tea: *mut snd_tea575x, pins: u8) {
    static void fmr2_tea575x_set_pins(struct snd_tea575x *tea, u8 pins)
    {
    struct fmr2 *fmr2 = tea.private_data;
    let mut bits: u8 = 0;
    bits |= (pins & TEA575X_DATA) ? STR_DATA : 0;
    bits |= (pins & TEA575X_CLK)  ? STR_CLK  : 0;
// WRITE_ENABLE is inverted, DATA must be high during read
    bits |= (pins & TEA575X_WREN) ? 0 : STR_WREN | STR_DATA;
    outb(bits, fmr2.io);
    }
#[no_mangle]
unsafe extern "C" fn fmr2_tea575x_get_pins(tea: *mut snd_tea575x) -> u8 {
    static u8 fmr2_tea575x_get_pins(struct snd_tea575x *tea)
    {
    struct fmr2 *fmr2 = tea.private_data;
    let mut bits: u8 = inb(fmr2.io);
    return  ((bits & STR_DATA) ? TEA575X_DATA : 0) |
    ((bits & STR_MOST) ? TEA575X_MOST : 0);
    }
#[no_mangle]
unsafe extern "C" fn fmr2_tea575x_set_direction(tea: *mut snd_tea575x, output: bool) {
    static void fmr2_tea575x_set_direction(struct snd_tea575x *tea, bool output)
    {
    }
    static const struct snd_tea575x_ops fmr2_tea_ops = {
    .set_pins = fmr2_tea575x_set_pins,
    .get_pins = fmr2_tea575x_get_pins,
    .set_direction = fmr2_tea575x_set_direction,
    };
// TC9154A/PT2254A volume control
// 18-bit shift register bit definitions

// bit 12 is ignored

// bits 15, 16, 17 must be 0

#[no_mangle]
unsafe extern "C" fn tc9154a_set_pins(fmr2: *mut fmr2, pins: u8) {
    static void tc9154a_set_pins(struct fmr2 *fmr2, u8 pins)
    {
    if (!fmr2.tea.mute)
    pins |= STR_WREN;
    outb(pins, fmr2.io);
    }
#[no_mangle]
unsafe extern "C" fn tc9154a_set_attenuation(fmr2: *mut fmr2, att: c_int, channel: u32) {
    static void tc9154a_set_attenuation(struct fmr2 *fmr2, int att, u32 channel)
    {
    int i;
    u32 reg;
    u8 bit;
    reg = TC9154A_ATT_MAJ(att / 10) | TC9154A_ATT_MIN((att % 10) / 2);
    reg |= channel;
// write 18-bit shift register, LSB first
    for (i = 0; i < 18; i++) {
    bit = reg & (1 << i) ? PT_DATA : 0;
    tc9154a_set_pins(fmr2, bit);
    udelay(5);
    tc9154a_set_pins(fmr2, bit | PT_CK);
    udelay(5);
    tc9154a_set_pins(fmr2, bit);
    }
// latch register data
    udelay(5);
    tc9154a_set_pins(fmr2, PT_ST);
    udelay(5);
    tc9154a_set_pins(fmr2, 0);
    }
#[no_mangle]
unsafe extern "C" fn fmr2_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int fmr2_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct snd_tea575x *tea = container_of(ctrl.handler, struct snd_tea575x, ctrl_handler);
    struct fmr2 *fmr2 = tea.private_data;
    int volume, balance, left, right;
    switch (ctrl.id) {
    case V4L2_CID_AUDIO_VOLUME:
    volume = ctrl.val;
    balance = fmr2.balance.cur.val;
    break;
    case V4L2_CID_AUDIO_BALANCE:
    balance = ctrl.val;
    volume = fmr2.volume.cur.val;
    break;
    default:
    return -EINVAL;
    }
    left = right = volume;
    if (balance < 0)
    right = max(0, right + balance);
    if (balance > 0)
    left = max(0, left - balance);
    tc9154a_set_attenuation(fmr2, abs(left - 68), TC9154A_CHANNEL_LEFT);
    tc9154a_set_attenuation(fmr2, abs(right - 68), TC9154A_CHANNEL_RIGHT);
    return 0;
    }
    static const struct v4l2_ctrl_ops fmr2_ctrl_ops = {
    .s_ctrl = fmr2_s_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn fmr2_tea_ext_init(tea: *mut snd_tea575x) -> c_int {
    static int fmr2_tea_ext_init(struct snd_tea575x *tea)
    {
    struct fmr2 *fmr2 = tea.private_data;
// FMR2 can have volume control, FMD2 can't (uses SB16 mixer)
    if (!fmr2.is_fmd2 && inb(fmr2.io) & FMR2_HASVOL) {
    fmr2.volume = v4l2_ctrl_new_std(&tea.ctrl_handler, &fmr2_ctrl_ops, V4L2_CID_AUDIO_VOLUME, 0, 68, 2, 56);
    fmr2.balance = v4l2_ctrl_new_std(&tea.ctrl_handler, &fmr2_ctrl_ops, V4L2_CID_AUDIO_BALANCE, -68, 68, 2, 0);
    if (tea.ctrl_handler.error) {
    printk(KERN_ERR "radio-sf16fmr2: can't initialize controls\n");
    return tea.ctrl_handler.error;
    }
    }
    return 0;
    }
    static const struct pnp_device_id fmr2_pnp_ids[] = {
    { .id = "MFRad13" }, /* tuner subdevice of SF16-FMD2 */
    { .id = "" }
    };
    MODULE_DEVICE_TABLE(pnp, fmr2_pnp_ids);
#[no_mangle]
unsafe extern "C" fn fmr2_probe(fmr2: *mut fmr2, pdev: *mut device, io: c_int) -> c_int {
    static int fmr2_probe(struct fmr2 *fmr2, struct device *pdev, int io)
    {
    int err, i;
    char *card_name = fmr2.is_fmd2 ? "SF16-FMD2" : "SF16-FMR2";
// avoid errors if a card was already registered at given port
    for (i = 0; i < num_fmr2_cards; i++)
    if (io == fmr2_cards[i].io)
    return -EBUSY;
    strscpy(fmr2.v4l2_dev.name, "radio-sf16fmr2",
    sizeof(fmr2.v4l2_dev.name));
    fmr2.io = io;
    if (!request_region(fmr2.io, 2, fmr2.v4l2_dev.name)) {
    printk(KERN_ERR "radio-sf16fmr2: I/O port 0x%x already in use\n", fmr2.io);
    return -EBUSY;
    }
    dev_set_drvdata(pdev, fmr2);
    err = v4l2_device_register(pdev, &fmr2.v4l2_dev);
    if (err < 0) {
    v4l2_err(&fmr2.v4l2_dev, "Could not register v4l2_device\n");
    release_region(fmr2.io, 2);
    return err;
    }
    fmr2.tea.v4l2_dev = &fmr2.v4l2_dev;
    fmr2.tea.private_data = fmr2;
    fmr2.tea.radio_nr = radio_nr[num_fmr2_cards];
    fmr2.tea.ops = &fmr2_tea_ops;
    fmr2.tea.ext_init = fmr2_tea_ext_init;
    strscpy(fmr2.tea.card, card_name, sizeof(fmr2.tea.card));
    snprintf(fmr2.tea.bus_info, sizeof(fmr2.tea.bus_info), "%s:%s",
    fmr2.is_fmd2 ? "PnP" : "ISA", dev_name(pdev));
    if (snd_tea575x_init(&fmr2.tea, THIS_MODULE)) {
    printk(KERN_ERR "radio-sf16fmr2: Unable to detect TEA575x tuner\n");
    release_region(fmr2.io, 2);
    return -ENODEV;
    }
    printk(KERN_INFO "radio-sf16fmr2: %s radio card at 0x%x.\n",
    card_name, fmr2.io);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fmr2_isa_match(pdev: *mut device, ndev: c_uint) -> c_int {
    static int fmr2_isa_match(struct device *pdev, unsigned int ndev)
    {
    struct fmr2 *fmr2 = kzalloc_obj(*fmr2);
    if (!fmr2)
    return 0;
    if (fmr2_probe(fmr2, pdev, FMR2_PORT)) {
    kfree(fmr2);
    return 0;
    }
    dev_set_drvdata(pdev, fmr2);
    fmr2_cards[num_fmr2_cards++] = fmr2;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn fmr2_pnp_probe(pdev: *mut pnp_dev, id: *const pnp_device_id) -> c_int {
    static int fmr2_pnp_probe(struct pnp_dev *pdev, const struct pnp_device_id *id)
    {
    int ret;
    struct fmr2 *fmr2 = kzalloc_obj(*fmr2);
    if (!fmr2)
    return -ENOMEM;
    fmr2.is_fmd2 = true;
    ret = fmr2_probe(fmr2, &pdev.dev, pnp_port_start(pdev, 0));
    if (ret) {
    kfree(fmr2);
    return ret;
    }
    pnp_set_drvdata(pdev, fmr2);
    fmr2_cards[num_fmr2_cards++] = fmr2;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fmr2_remove(fmr2: *mut fmr2) {
    static void fmr2_remove(struct fmr2 *fmr2)
    {
    snd_tea575x_exit(&fmr2.tea);
    release_region(fmr2.io, 2);
    v4l2_device_unregister(&fmr2.v4l2_dev);
    kfree(fmr2);
    }
#[no_mangle]
unsafe extern "C" fn fmr2_isa_remove(pdev: *mut device, ndev: c_uint) {
    static void fmr2_isa_remove(struct device *pdev, unsigned int ndev)
    {
    fmr2_remove(dev_get_drvdata(pdev));
    }
#[no_mangle]
unsafe extern "C" fn fmr2_pnp_remove(pdev: *mut pnp_dev) {
    static void fmr2_pnp_remove(struct pnp_dev *pdev)
    {
    fmr2_remove(pnp_get_drvdata(pdev));
    pnp_set_drvdata(pdev, core::ptr::null_mut());
    }
    static struct isa_driver fmr2_isa_driver = {
    .match		= fmr2_isa_match,
    .remove		= fmr2_isa_remove,
    .driver		= {
    .name	= "radio-sf16fmr2",
    },
    };
    static struct pnp_driver fmr2_pnp_driver = {
    .name		= "radio-sf16fmr2",
    .id_table	= fmr2_pnp_ids,
    .probe		= fmr2_pnp_probe,
    .remove		= fmr2_pnp_remove,
    };
#[no_mangle]
unsafe extern "C" fn fmr2_init() -> int __init {
    static int __init fmr2_init(void)
    {
    int ret;
    ret = pnp_register_driver(&fmr2_pnp_driver);
    if (!ret)
    pnp_registered = true;
    ret = isa_register_driver(&fmr2_isa_driver, 1);
    if (!ret)
    isa_registered = true;
    return (pnp_registered || isa_registered) ? 0 : ret;
    }
#[no_mangle]
unsafe extern "C" fn fmr2_exit() -> void __exit {
    static void __exit fmr2_exit(void)
    {
    if (pnp_registered)
    pnp_unregister_driver(&fmr2_pnp_driver);
    if (isa_registered)
    isa_unregister_driver(&fmr2_isa_driver);
    }
    module_init(fmr2_init);
    module_exit(fmr2_exit);
