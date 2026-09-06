//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/saa7146/hexium_gemini.c
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
    hexium_gemini.c - v4l2 driver for Hexium Gemini frame grabber cards
    Visit http://www.mihu.de/linux/saa7146/ and follow the link
    to "hexium" for further details about this card.
    Copyright (C) 2003 Michael Hunold <michael@mihu.de>
//

    static int debug;
    module_param(debug, int, 0);
    MODULE_PARM_DESC(debug, "debug verbosity");
// global variables
    static int hexium_num;
pub const HEXIUM_GEMINI: c_int = 4;
pub const HEXIUM_GEMINI_DUAL: c_int = 5;

pub const HEXIUM_INPUTS: c_int = 9;
    static struct v4l2_input hexium_inputs[HEXIUM_INPUTS] = {
    { 0, "CVBS 1",	V4L2_INPUT_TYPE_CAMERA,	0, 0, HEXIUM_STD, 0, V4L2_IN_CAP_STD },
    { 1, "CVBS 2",	V4L2_INPUT_TYPE_CAMERA,	0, 0, HEXIUM_STD, 0, V4L2_IN_CAP_STD },
    { 2, "CVBS 3",	V4L2_INPUT_TYPE_CAMERA,	0, 0, HEXIUM_STD, 0, V4L2_IN_CAP_STD },
    { 3, "CVBS 4",	V4L2_INPUT_TYPE_CAMERA,	0, 0, HEXIUM_STD, 0, V4L2_IN_CAP_STD },
    { 4, "CVBS 5",	V4L2_INPUT_TYPE_CAMERA,	0, 0, HEXIUM_STD, 0, V4L2_IN_CAP_STD },
    { 5, "CVBS 6",	V4L2_INPUT_TYPE_CAMERA,	0, 0, HEXIUM_STD, 0, V4L2_IN_CAP_STD },
    { 6, "Y/C 1",	V4L2_INPUT_TYPE_CAMERA,	0, 0, HEXIUM_STD, 0, V4L2_IN_CAP_STD },
    { 7, "Y/C 2",	V4L2_INPUT_TYPE_CAMERA,	0, 0, HEXIUM_STD, 0, V4L2_IN_CAP_STD },
    { 8, "Y/C 3",	V4L2_INPUT_TYPE_CAMERA,	0, 0, HEXIUM_STD, 0, V4L2_IN_CAP_STD },
    };
pub const HEXIUM_AUDIOS: c_int = 0;
    struct hexium_data
    {
    s8 adr;
    u8 byte;
    };
pub const HEXIUM_GEMINI_V_1_0: c_int = 1;
pub const HEXIUM_GEMINI_DUAL_V_1_0: c_int = 2;
    struct hexium
    {
    int type;
    struct video_device	video_dev;
    struct i2c_adapter	i2c_adapter;
    int		cur_input;	/* current input */
    v4l2_std_id	cur_std;	/* current standard */
    };
// Samsung KS0127B decoder default registers
    static u8 hexium_ks0127b[0x100]={
// 00*/ 0x00,0x52,0x30,0x40,0x01,0x0C,0x2A,0x10,
// 08*/ 0x00,0x00,0x00,0x60,0x00,0x00,0x0F,0x06,
// 10*/ 0x00,0x00,0xE4,0xC0,0x00,0x00,0x00,0x00,
// 18*/ 0x14,0x9B,0xFE,0xFF,0xFC,0xFF,0x03,0x22,
// 20*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// 28*/ 0x00,0x00,0x00,0x00,0x00,0x2C,0x9B,0x00,
// 30*/ 0x00,0x00,0x10,0x80,0x80,0x10,0x80,0x80,
// 38*/ 0x01,0x04,0x00,0x00,0x00,0x29,0xC0,0x00,
// 40*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// 48*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// 50*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// 58*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// 60*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// 68*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// 70*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// 78*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// 80*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// 88*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// 90*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// 98*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// A0*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// A8*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// B0*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// B8*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// C0*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// C8*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// D0*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// D8*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// E0*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// E8*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// F0*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
// F8*/ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00
    };
    static struct hexium_data hexium_pal[] = {
    { 0x01, 0x52 }, { 0x12, 0x64 }, { 0x2D, 0x2C }, { 0x2E, 0x9B }, { -1 , 0xFF }
    };
    static struct hexium_data hexium_ntsc[] = {
    { 0x01, 0x53 }, { 0x12, 0x04 }, { 0x2D, 0x23 }, { 0x2E, 0x81 }, { -1 , 0xFF }
    };
    static struct hexium_data hexium_secam[] = {
    { 0x01, 0x52 }, { 0x12, 0x64 }, { 0x2D, 0x2C }, { 0x2E, 0x9B }, { -1 , 0xFF }
    };
    static struct hexium_data hexium_input_select[] = {
    { 0x02, 0x60 },
    { 0x02, 0x64 },
    { 0x02, 0x61 },
    { 0x02, 0x65 },
    { 0x02, 0x62 },
    { 0x02, 0x66 },
    { 0x02, 0x68 },
    { 0x02, 0x69 },
    { 0x02, 0x6A },
    };
// fixme: h_offset = 0 for Hexium Gemini *Dual*, which
    are currently *not* supported*/
    static struct saa7146_standard hexium_standards[] = {
    {
    .name	= "PAL",	.id	= V4L2_STD_PAL,
    .v_offset	= 28,	.v_field	= 288,
    .h_offset	= 1,	.h_pixels	= 680,
    .v_max_out	= 576,	.h_max_out	= 768,
    }, {
    .name	= "NTSC",	.id	= V4L2_STD_NTSC,
    .v_offset	= 28,	.v_field	= 240,
    .h_offset	= 1,	.h_pixels	= 640,
    .v_max_out	= 480,	.h_max_out	= 640,
    }, {
    .name	= "SECAM",	.id	= V4L2_STD_SECAM,
    .v_offset	= 28,	.v_field	= 288,
    .h_offset	= 1,	.h_pixels	= 720,
    .v_max_out	= 576,	.h_max_out	= 768,
    }
    };
// bring hardware to a sane state. this has to be done, just in case someone
    wants to capture from this device before it has been properly initialized.
    the capture engine would badly fail, because no valid signal arrives on the
    saa7146, thus leading to timeouts and stuff. */
#[no_mangle]
unsafe extern "C" fn hexium_init_done(dev: *mut saa7146_dev) -> c_int {
    static int hexium_init_done(struct saa7146_dev *dev)
    {
    struct hexium *hexium = (struct hexium *) dev.ext_priv;
    union i2c_smbus_data data;
    let mut i: c_int = 0;
    DEB_D("hexium_init_done called\n");
// initialize the helper ics to useful values
    for (i = 0; i < sizeof(hexium_ks0127b); i++) {
    data.byte = hexium_ks0127b[i];
    if (0 != i2c_smbus_xfer(&hexium.i2c_adapter, 0x6c, 0, I2C_SMBUS_WRITE, i, I2C_SMBUS_BYTE_DATA, &data)) {
    pr_err("hexium_init_done() failed for address 0x%02x\n",
    i);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hexium_set_input(hexium: *mut hexium, input: c_int) -> c_int {
    static int hexium_set_input(struct hexium *hexium, int input)
    {
    union i2c_smbus_data data;
    DEB_D("\n");
    data.byte = hexium_input_select[input].byte;
    if (0 != i2c_smbus_xfer(&hexium.i2c_adapter, 0x6c, 0, I2C_SMBUS_WRITE, hexium_input_select[input].adr, I2C_SMBUS_BYTE_DATA, &data)) {
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hexium_set_standard(hexium: *mut hexium, vdec: *mut hexium_data) -> c_int {
    static int hexium_set_standard(struct hexium *hexium, struct hexium_data *vdec)
    {
    union i2c_smbus_data data;
    let mut i: c_int = 0;
    DEB_D("\n");
    while (vdec[i].adr != -1) {
    data.byte = vdec[i].byte;
    if (0 != i2c_smbus_xfer(&hexium.i2c_adapter, 0x6c, 0, I2C_SMBUS_WRITE, vdec[i].adr, I2C_SMBUS_BYTE_DATA, &data)) {
    pr_err("hexium_init_done: hexium_set_standard() failed for address 0x%02x\n",
    i);
    return -1;
    }
    i++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vidioc_enum_input(file: *mut file, fh: *mut c_void, i: *mut v4l2_input) -> c_int {
    static int vidioc_enum_input(struct file *file, void *fh, struct v4l2_input *i)
    {
    DEB_EE("VIDIOC_ENUMINPUT %d\n", i.index);
    if (i.index >= HEXIUM_INPUTS)
    return -EINVAL;
    memcpy(i, &hexium_inputs[i.index], sizeof(struct v4l2_input));
    DEB_D("v4l2_ioctl: VIDIOC_ENUMINPUT %d\n", i.index);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vidioc_g_input(file: *mut file, fh: *mut c_void, input: *mut c_uint) -> c_int {
    static int vidioc_g_input(struct file *file, void *fh, unsigned int *input)
    {
    struct saa7146_dev *dev = video_drvdata(file);
    struct hexium *hexium = (struct hexium *) dev.ext_priv;
// input = hexium->cur_input;
    DEB_D("VIDIOC_G_INPUT: %d\n", *input);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vidioc_s_input(file: *mut file, fh: *mut c_void, input: c_uint) -> c_int {
    static int vidioc_s_input(struct file *file, void *fh, unsigned int input)
    {
    struct saa7146_dev *dev = video_drvdata(file);
    struct hexium *hexium = (struct hexium *) dev.ext_priv;
    DEB_EE("VIDIOC_S_INPUT %d\n", input);
    if (input >= HEXIUM_INPUTS)
    return -EINVAL;
    hexium.cur_input = input;
    hexium_set_input(hexium, input);
    return 0;
    }
    static struct saa7146_ext_vv vv_data;
// this function only gets called when the probing was successful
#[no_mangle]
unsafe extern "C" fn hexium_attach(dev: *mut saa7146_dev, info: *mut saa7146_pci_extension_data) -> c_int {
    static int hexium_attach(struct saa7146_dev *dev, struct saa7146_pci_extension_data *info)
    {
    struct hexium *hexium;
    int ret;
    DEB_EE("\n");
    hexium = kzalloc_obj(*hexium);
    if (!hexium)
    return -ENOMEM;
    dev.ext_priv = hexium;
// enable i2c-port pins
    saa7146_write(dev, MC1, (MASK_08 | MASK_24 | MASK_10 | MASK_26));
    strscpy(hexium.i2c_adapter.name, "hexium gemini",
    sizeof(hexium.i2c_adapter.name));
    saa7146_i2c_adapter_prepare(dev, &hexium.i2c_adapter, SAA7146_I2C_BUS_BIT_RATE_480);
    if (i2c_add_adapter(&hexium.i2c_adapter) < 0) {
    DEB_S("cannot register i2c-device. skipping.\n");
    kfree(hexium);
    return -EFAULT;
    }
// set HWControl GPIO number 2
    saa7146_setgpio(dev, 2, SAA7146_GPIO_OUTHI);
    saa7146_write(dev, DD1_INIT, 0x07000700);
    saa7146_write(dev, DD1_STREAM_B, 0x00000000);
    saa7146_write(dev, MC2, (MASK_09 | MASK_25 | MASK_10 | MASK_26));
// the rest
    hexium.cur_input = 0;
    hexium_init_done(dev);
    hexium_set_standard(hexium, hexium_pal);
    hexium.cur_std = V4L2_STD_PAL;
    hexium_set_input(hexium, 0);
    hexium.cur_input = 0;
    ret = saa7146_vv_init(dev, &vv_data);
    if (ret) {
    i2c_del_adapter(&hexium.i2c_adapter);
    kfree(hexium);
    return ret;
    }
    vv_data.vid_ops.vidioc_enum_input = vidioc_enum_input;
    vv_data.vid_ops.vidioc_g_input = vidioc_g_input;
    vv_data.vid_ops.vidioc_s_input = vidioc_s_input;
    ret = saa7146_register_device(&hexium.video_dev, dev, "hexium gemini", VFL_TYPE_VIDEO);
    if (ret < 0) {
    pr_err("cannot register capture v4l2 device. skipping.\n");
    saa7146_vv_release(dev);
    i2c_del_adapter(&hexium.i2c_adapter);
    kfree(hexium);
    return ret;
    }
    pr_info("found 'hexium gemini' frame grabber-%d\n", hexium_num);
    hexium_num++;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hexium_detach(dev: *mut saa7146_dev) -> c_int {
    static int hexium_detach(struct saa7146_dev *dev)
    {
    struct hexium *hexium = (struct hexium *) dev.ext_priv;
    DEB_EE("dev:%p\n", dev);
    saa7146_unregister_device(&hexium.video_dev, dev);
    saa7146_vv_release(dev);
    hexium_num--;
    i2c_del_adapter(&hexium.i2c_adapter);
    kfree(hexium);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn std_callback(dev: *mut saa7146_dev, std: *mut saa7146_standard) -> c_int {
    static int std_callback(struct saa7146_dev *dev, struct saa7146_standard *std)
    {
    struct hexium *hexium = (struct hexium *) dev.ext_priv;
    if (V4L2_STD_PAL == std.id) {
    hexium_set_standard(hexium, hexium_pal);
    hexium.cur_std = V4L2_STD_PAL;
    return 0;
    } else if (V4L2_STD_NTSC == std.id) {
    hexium_set_standard(hexium, hexium_ntsc);
    hexium.cur_std = V4L2_STD_NTSC;
    return 0;
    } else if (V4L2_STD_SECAM == std.id) {
    hexium_set_standard(hexium, hexium_secam);
    hexium.cur_std = V4L2_STD_SECAM;
    return 0;
    }
    return -1;
    }
    static struct saa7146_extension hexium_extension;
    static struct saa7146_pci_extension_data hexium_gemini_4bnc = {
    .ext_priv = "Hexium Gemini (4 BNC)",
    .ext = &hexium_extension,
    };
    static struct saa7146_pci_extension_data hexium_gemini_dual_4bnc = {
    .ext_priv = "Hexium Gemini Dual (4 BNC)",
    .ext = &hexium_extension,
    };
    static const struct pci_device_id pci_tbl[] = {
    {
    PCI_VDEVICE_SUB(PHILIPS, PCI_DEVICE_ID_PHILIPS_SAA7146, 0x17c8, 0x2401),
    .driver_data = (unsigned long)&hexium_gemini_4bnc,
    },
    {
    PCI_VDEVICE_SUB(PHILIPS, PCI_DEVICE_ID_PHILIPS_SAA7146, 0x17c8, 0x2402),
    .driver_data = (unsigned long)&hexium_gemini_dual_4bnc,
    },
    { }
    };
    MODULE_DEVICE_TABLE(pci, pci_tbl);
    static struct saa7146_ext_vv vv_data = {
    .inputs = HEXIUM_INPUTS,
    .capabilities = 0,
    .stds = &hexium_standards[0],
    .num_stds = ARRAY_SIZE(hexium_standards),
    .std_callback = &std_callback,
    };
    static struct saa7146_extension hexium_extension = {
    .name = "hexium gemini",
    .flags = SAA7146_USE_I2C_IRQ,
    .pci_tbl = &pci_tbl[0],
    .module = THIS_MODULE,
    .attach = hexium_attach,
    .detach = hexium_detach,
    .irq_mask = 0,
    .irq_func = core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn hexium_init_module() -> int __init {
    static int __init hexium_init_module(void)
    {
    if (0 != saa7146_register_extension(&hexium_extension)) {
    DEB_S("failed to register extension\n");
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hexium_cleanup_module() -> void __exit {
    static void __exit hexium_cleanup_module(void)
    {
    saa7146_unregister_extension(&hexium_extension);
    }
    module_init(hexium_init_module);
    module_exit(hexium_cleanup_module);
    MODULE_DESCRIPTION("video4linux-2 driver for Hexium Gemini frame grabber cards");
    MODULE_AUTHOR("Michael Hunold <michael@mihu.de>");
    MODULE_LICENSE("GPL");
