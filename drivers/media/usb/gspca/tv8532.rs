//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/gspca/tv8532.c
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
// Quickcam cameras initialization data
//
// V4L2 by Jean-Francois Moine <http://moinejf.free.fr>
//

    MODULE_AUTHOR("Michel Xhaard <mxhaard@users.sourceforge.net>");
    MODULE_DESCRIPTION("TV8532 USB Camera Driver");
    MODULE_LICENSE("GPL");
// specific webcam descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd {
    pub /: *mut *mut gspca_dev gspca_dev; / !! must be the first item,
    pub packet: __u8,
}

    static const struct v4l2_pix_format sif_mode[] = {
    {176, 144, V4L2_PIX_FMT_SBGGR8, V4L2_FIELD_NONE,
    .bytesperline = 176,
    .sizeimage = 176 * 144,
    .colorspace = V4L2_COLORSPACE_SRGB,
    .priv = 1},
    {352, 288, V4L2_PIX_FMT_SBGGR8, V4L2_FIELD_NONE,
    .bytesperline = 352,
    .sizeimage = 352 * 288,
    .colorspace = V4L2_COLORSPACE_SRGB,
    .priv = 0},
    };
// TV-8532A (ICM532A) registers (LE)
pub const R00_PART_CONTROL: c_uint = 0x00;
pub const LATENT_CHANGE: c_uint = 0x80;
pub const EXPO_CHANGE: c_uint = 0x04;
pub const R01_TIMING_CONTROL_LOW: c_uint = 0x01;
pub const CMD_EEprom_Open: c_uint = 0x30;
pub const CMD_EEprom_Close: c_uint = 0x29;
pub const R03_TABLE_ADDR: c_uint = 0x03;
pub const R04_WTRAM_DATA_L: c_uint = 0x04;
pub const R05_WTRAM_DATA_M: c_uint = 0x05;
pub const R06_WTRAM_DATA_H: c_uint = 0x06;
pub const R07_TABLE_LEN: c_uint = 0x07;
pub const R08_RAM_WRITE_ACTION: c_uint = 0x08;
pub const R0C_AD_WIDTHL: c_uint = 0x0c;
pub const R0D_AD_WIDTHH: c_uint = 0x0d;
pub const R0E_AD_HEIGHTL: c_uint = 0x0e;
pub const R0F_AD_HEIGHTH: c_uint = 0x0f;
pub const R10_AD_COL_BEGINL: c_uint = 0x10;
pub const R11_AD_COL_BEGINH: c_uint = 0x11;
pub const MIRROR: c_uint = 0x04	/* [10] */;
pub const R14_AD_ROW_BEGINL: c_uint = 0x14;
pub const R15_AD_ROWBEGINH: c_uint = 0x15;
pub const R1C_AD_EXPOSE_TIMEL: c_uint = 0x1c;
pub const R20_GAIN_G1L: c_uint = 0x20;
pub const R21_GAIN_G1H: c_uint = 0x21;
pub const R22_GAIN_RL: c_uint = 0x22;
pub const R23_GAIN_RH: c_uint = 0x23;
pub const R24_GAIN_BL: c_uint = 0x24;
pub const R25_GAIN_BH: c_uint = 0x25;
pub const R26_GAIN_G2L: c_uint = 0x26;
pub const R27_GAIN_G2H: c_uint = 0x27;
pub const R28_QUANT: c_uint = 0x28;
pub const R29_LINE: c_uint = 0x29;
pub const R2C_POLARITY: c_uint = 0x2c;
pub const R2D_POINT: c_uint = 0x2d;
pub const R2E_POINTH: c_uint = 0x2e;
pub const R2F_POINTB: c_uint = 0x2f;
pub const R30_POINTBH: c_uint = 0x30;
pub const R31_UPD: c_uint = 0x31;
pub const R2A_HIGH_BUDGET: c_uint = 0x2a;
pub const R2B_LOW_BUDGET: c_uint = 0x2b;
pub const R34_VID: c_uint = 0x34;
pub const R35_VIDH: c_uint = 0x35;
pub const R36_PID: c_uint = 0x36;
pub const R37_PIDH: c_uint = 0x37;
pub const R39_Test1: c_uint = 0x39		/* GPIO */;
pub const R3B_Test3: c_uint = 0x3b		/* GPIO */;
pub const R83_AD_IDH: c_uint = 0x83;
pub const R91_AD_SLOPEREG: c_uint = 0x91;
pub const R94_AD_BITCONTROL: c_uint = 0x94;
    static const u8 eeprom_data[][3] = {
// dataH dataM dataL
    {0x01, 0x00, 0x01},
    {0x01, 0x80, 0x11},
    {0x05, 0x00, 0x14},
    {0x05, 0x00, 0x1c},
    {0x0d, 0x00, 0x1e},
    {0x05, 0x00, 0x1f},
    {0x05, 0x05, 0x19},
    {0x05, 0x01, 0x1b},
    {0x05, 0x09, 0x1e},
    {0x0d, 0x89, 0x2e},
    {0x05, 0x89, 0x2f},
    {0x05, 0x0d, 0xd9},
    {0x05, 0x09, 0xf1},
    };
// write 1 byte
    static void reg_w1(struct gspca_dev *gspca_dev,
    __u16 index, __u8 value)
    {
    gspca_dev.usb_buf[0] = value;
    usb_control_msg(gspca_dev.dev,
    usb_sndctrlpipe(gspca_dev.dev, 0),
    0x02,
    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    0,	/* value */
    index, gspca_dev.usb_buf, 1, 500);
    }
// write 2 bytes
    static void reg_w2(struct gspca_dev *gspca_dev,
    u16 index, u16 value)
    {
    gspca_dev.usb_buf[0] = value;
    gspca_dev.usb_buf[1] = value >> 8;
    usb_control_msg(gspca_dev.dev,
    usb_sndctrlpipe(gspca_dev.dev, 0),
    0x02,
    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    0,	/* value */
    index, gspca_dev.usb_buf, 2, 500);
    }
#[no_mangle]
unsafe extern "C" fn tv_8532WriteEEprom(gspca_dev: *mut gspca_dev) {
    static void tv_8532WriteEEprom(struct gspca_dev *gspca_dev)
    {
    int i;
    reg_w1(gspca_dev, R01_TIMING_CONTROL_LOW, CMD_EEprom_Open);
    for (i = 0; i < ARRAY_SIZE(eeprom_data); i++) {
    reg_w1(gspca_dev, R03_TABLE_ADDR, i);
    reg_w1(gspca_dev, R04_WTRAM_DATA_L, eeprom_data[i][2]);
    reg_w1(gspca_dev, R05_WTRAM_DATA_M, eeprom_data[i][1]);
    reg_w1(gspca_dev, R06_WTRAM_DATA_H, eeprom_data[i][0]);
    reg_w1(gspca_dev, R08_RAM_WRITE_ACTION, 0);
    }
    reg_w1(gspca_dev, R07_TABLE_LEN, i);
    reg_w1(gspca_dev, R01_TIMING_CONTROL_LOW, CMD_EEprom_Close);
    }
// this function is called at probe time
    static int sd_config(struct gspca_dev *gspca_dev,
    const struct usb_device_id *id)
    {
    struct cam *cam;
    cam = &gspca_dev.cam;
    cam.cam_mode = sif_mode;
    cam.nmodes = ARRAY_SIZE(sif_mode);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tv_8532_setReg(gspca_dev: *mut gspca_dev) {
    static void tv_8532_setReg(struct gspca_dev *gspca_dev)
    {
    reg_w1(gspca_dev, R3B_Test3, 0x0a);	/* Test0Sel = 10 */
//
    reg_w1(gspca_dev, R0E_AD_HEIGHTL, 0x90);
    reg_w1(gspca_dev, R0F_AD_HEIGHTH, 0x01);
    reg_w2(gspca_dev, R1C_AD_EXPOSE_TIMEL, 0x018f);
    reg_w1(gspca_dev, R10_AD_COL_BEGINL, 0x44);
// begin active line
    reg_w1(gspca_dev, R11_AD_COL_BEGINH, 0x00);
// mirror and digital gain
    reg_w1(gspca_dev, R14_AD_ROW_BEGINL, 0x0a);
    reg_w1(gspca_dev, R94_AD_BITCONTROL, 0x02);
    reg_w1(gspca_dev, R91_AD_SLOPEREG, 0x00);
    reg_w1(gspca_dev, R00_PART_CONTROL, LATENT_CHANGE | EXPO_CHANGE);
// = 0x84
    }
// this function is called at probe and resume time
#[no_mangle]
unsafe extern "C" fn sd_init(gspca_dev: *mut gspca_dev) -> c_int {
    static int sd_init(struct gspca_dev *gspca_dev)
    {
    tv_8532WriteEEprom(gspca_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn setexposure(gspca_dev: *mut gspca_dev, val: i32) {
    static void setexposure(struct gspca_dev *gspca_dev, s32 val)
    {
    reg_w2(gspca_dev, R1C_AD_EXPOSE_TIMEL, val);
    reg_w1(gspca_dev, R00_PART_CONTROL, LATENT_CHANGE | EXPO_CHANGE);
// 0x84
    }
#[no_mangle]
unsafe extern "C" fn setgain(gspca_dev: *mut gspca_dev, val: i32) {
    static void setgain(struct gspca_dev *gspca_dev, s32 val)
    {
    reg_w2(gspca_dev, R20_GAIN_G1L, val);
    reg_w2(gspca_dev, R22_GAIN_RL, val);
    reg_w2(gspca_dev, R24_GAIN_BL, val);
    reg_w2(gspca_dev, R26_GAIN_G2L, val);
    }
// -- start the camera --
#[no_mangle]
unsafe extern "C" fn sd_start(gspca_dev: *mut gspca_dev) -> c_int {
    static int sd_start(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    reg_w1(gspca_dev, R0C_AD_WIDTHL, 0xe8);		/* 0x20; 0x0c */
    reg_w1(gspca_dev, R0D_AD_WIDTHH, 0x03);
//
    reg_w1(gspca_dev, R28_QUANT, 0x90);
// 0x72 compressed mode 0x28
    if (gspca_dev.cam.cam_mode[(int) gspca_dev.curr_mode].priv) {
// 176x144
    reg_w1(gspca_dev, R29_LINE, 0x41);
// CIF - 2 lines/packet
    } else {
// 352x288
    reg_w1(gspca_dev, R29_LINE, 0x81);
// CIF - 2 lines/packet
    }
//
    reg_w1(gspca_dev, R2C_POLARITY, 0x10);		/* slow clock */
    reg_w1(gspca_dev, R2D_POINT, 0x14);
    reg_w1(gspca_dev, R2E_POINTH, 0x01);
    reg_w1(gspca_dev, R2F_POINTB, 0x12);
    reg_w1(gspca_dev, R30_POINTBH, 0x01);
    tv_8532_setReg(gspca_dev);
//
    reg_w1(gspca_dev, R31_UPD, 0x01);	/* update registers */
    msleep(200);
    reg_w1(gspca_dev, R31_UPD, 0x00);	/* end update */
    gspca_dev.empty_packet = 0;		/* check the empty packets */
    sd.packet = 0;				/* ignore the first packets */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sd_stopN(gspca_dev: *mut gspca_dev) {
    static void sd_stopN(struct gspca_dev *gspca_dev)
    {
    reg_w1(gspca_dev, R3B_Test3, 0x0b);	/* Test0Sel = 11 = GPIO */
    }
    static void sd_pkt_scan(struct gspca_dev *gspca_dev,
    u8 *data,			/* isoc packet */
    int len)			/* iso packet length */
    {
    struct sd *sd = (struct sd *) gspca_dev;
    int packet_type0, packet_type1;
    packet_type0 = packet_type1 = INTER_PACKET;
    if (gspca_dev.empty_packet) {
    gspca_dev.empty_packet = 0;
    sd.packet = gspca_dev.pixfmt.height / 2;
    packet_type0 = FIRST_PACKET;
    } else if (sd.packet == 0)
    return;			/* 2 more lines in 352x288 ! */
    sd.packet--;
    if (sd.packet == 0)
    packet_type1 = LAST_PACKET;
// each packet contains:
// - header 2 bytes
// - RGRG line
// - 4 bytes
// - GBGB line
// - 4 bytes
//
    gspca_frame_add(gspca_dev, packet_type0,
    data + 2, gspca_dev.pixfmt.width);
    gspca_frame_add(gspca_dev, packet_type1,
    data + gspca_dev.pixfmt.width + 5,
    gspca_dev.pixfmt.width);
    }
#[no_mangle]
unsafe extern "C" fn sd_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int sd_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct gspca_dev *gspca_dev =
    container_of(ctrl.handler, struct gspca_dev, ctrl_handler);
    gspca_dev.usb_err = 0;
    if (!gspca_dev.streaming)
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_EXPOSURE:
    setexposure(gspca_dev, ctrl.val);
    break;
    case V4L2_CID_GAIN:
    setgain(gspca_dev, ctrl.val);
    break;
    }
    return gspca_dev.usb_err;
    }
    static const struct v4l2_ctrl_ops sd_ctrl_ops = {
    .s_ctrl = sd_s_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn sd_init_controls(gspca_dev: *mut gspca_dev) -> c_int {
    static int sd_init_controls(struct gspca_dev *gspca_dev)
    {
    struct v4l2_ctrl_handler *hdl = &gspca_dev.ctrl_handler;
    gspca_dev.vdev.ctrl_handler = hdl;
    v4l2_ctrl_handler_init(hdl, 2);
    v4l2_ctrl_new_std(hdl, &sd_ctrl_ops,
    V4L2_CID_EXPOSURE, 0, 0x18f, 1, 0x18f);
    v4l2_ctrl_new_std(hdl, &sd_ctrl_ops,
    V4L2_CID_GAIN, 0, 0x7ff, 1, 0x100);
    if (hdl.error) {
    pr_err("Could not initialize controls\n");
    return hdl.error;
    }
    return 0;
    }
// sub-driver description
    static const struct sd_desc sd_desc = {
    .name = MODULE_NAME,
    .config = sd_config,
    .init = sd_init,
    .init_controls = sd_init_controls,
    .start = sd_start,
    .stopN = sd_stopN,
    .pkt_scan = sd_pkt_scan,
    };
// -- module initialisation --
    static const struct usb_device_id device_table[] = {
    {USB_DEVICE(0x046d, 0x0920)},
    {USB_DEVICE(0x046d, 0x0921)},
    {USB_DEVICE(0x0545, 0x808b)},
    {USB_DEVICE(0x0545, 0x8333)},
    {USB_DEVICE(0x0923, 0x010f)},
    {}
    };
    MODULE_DEVICE_TABLE(usb, device_table);
// -- device connect --
    static int sd_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    return gspca_dev_probe(intf, id, &sd_desc, sizeof(struct sd),
    THIS_MODULE);
    }
    static struct usb_driver sd_driver = {
    .name = MODULE_NAME,
    .id_table = device_table,
    .probe = sd_probe,
    .disconnect = gspca_disconnect,

    .suspend = gspca_suspend,
    .resume = gspca_resume,
    .reset_resume = gspca_resume,

    };
    module_usb_driver(sd_driver);
