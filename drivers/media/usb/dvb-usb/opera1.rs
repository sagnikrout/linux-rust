//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/dvb-usb/opera1.c
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
// DVB USB framework compliant Linux driver for the Opera1 DVB-S Card
//
// Copyright (C) 2006 Mario Hlawitschka (dh1pa@amsat.org)
// Copyright (C) 2006 Marco Gittler (g.marco@freenet.de)
//
// see Documentation/driver-api/media/drivers/dvb-usb.rst for more information
//

pub const OPERA_READ_MSG: c_int = 0;
pub const OPERA_WRITE_MSG: c_int = 1;
pub const OPERA_I2C_TUNER: c_uint = 0xd1;
pub const READ_FX2_REG_REQ: c_uint = 0xba;
pub const READ_MAC_ADDR: c_uint = 0x08;
pub const OPERA_WRITE_FX2: c_uint = 0xbb;
pub const OPERA_TUNER_REQ: c_uint = 0xb1;
pub const REG_1F_SYMBOLRATE_BYTE0: c_uint = 0x1f;
pub const REG_20_SYMBOLRATE_BYTE1: c_uint = 0x20;
pub const REG_21_SYMBOLRATE_BYTE2: c_uint = 0x21;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opera1_state {
    pub last_key_pressed: u32,
}

    static int dvb_usb_opera1_debug;
    module_param_named(debug, dvb_usb_opera1_debug, int, 0644);
    MODULE_PARM_DESC(debug,
    "set debugging level (1=info,xfer=2,pll=4,ts=8,err=16,rc=32,fw=64 (or-able))."
    DVB_USB_DEBUG_STATUS);
    DVB_DEFINE_MOD_OPT_ADAPTER_NR(adapter_nr);
    static int opera1_xilinx_rw(struct usb_device *dev, u8 request, u16 value,
    u8 * data, u16 len, int flags)
    {
    int ret;
    u8 tmp;
    u8 *buf;
    unsigned int pipe = (flags == OPERA_READ_MSG) ?
    usb_rcvctrlpipe(dev,0) : usb_sndctrlpipe(dev, 0);
    let mut request_type: u8 = (flags == OPERA_READ_MSG) ? USB_DIR_IN : USB_DIR_OUT;
    buf = kmalloc(len, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    if (flags == OPERA_WRITE_MSG)
    memcpy(buf, data, len);
    ret = usb_control_msg(dev, pipe, request,
    request_type | USB_TYPE_VENDOR, value, 0x0,
    buf, len, 2000);
    if (request == OPERA_TUNER_REQ) {
    tmp = buf[0];
    if (usb_control_msg(dev, usb_rcvctrlpipe(dev, 0),
    OPERA_TUNER_REQ, USB_DIR_IN | USB_TYPE_VENDOR,
    0x01, 0x0, buf, 1, 2000) < 1 || buf[0] != 0x08) {
    ret = 0;
    goto out;
    }
    buf[0] = tmp;
    }
    if (flags == OPERA_READ_MSG)
    memcpy(data, buf, len);
    out:
    kfree(buf);
    return ret;
    }
// I2C
    static int opera1_usb_i2c_msgxfer(struct dvb_usb_device *dev, u16 addr,
    u8 * buf, u16 len)
    {
    let mut ret: c_int = 0;
    u8 request;
    u16 value;
    if (!dev) {
    info("no usb_device");
    return -EINVAL;
    }
    if (mutex_lock_interruptible(&dev.usb_mutex) < 0)
    return -EAGAIN;
    switch (addr>>1){
    case ADDR_B600_VOLTAGE_13V:
    request=0xb6;
    value=0x00;
    break;
    case ADDR_B601_VOLTAGE_18V:
    request=0xb6;
    value=0x01;
    break;
    case ADDR_B1A6_STREAM_CTRL:
    request=0xb1;
    value=0xa6;
    break;
    case ADDR_B880_READ_REMOTE:
    request=0xb8;
    value=0x80;
    break;
    default:
    request=0xb1;
    value=addr;
    }
    ret = opera1_xilinx_rw(dev.udev, request,
    value, buf, len,
    addr&0x01?OPERA_READ_MSG:OPERA_WRITE_MSG);
    mutex_unlock(&dev.usb_mutex);
    return ret;
    }
    static int opera1_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg msg[],
    int num)
    {
    struct dvb_usb_device *d = i2c_get_adapdata(adap);
    let mut i: c_int = 0, tmp = 0;
    if (!d)
    return -ENODEV;
    if (mutex_lock_interruptible(&d.i2c_mutex) < 0)
    return -EAGAIN;
    for (i = 0; i < num; i++) {
    if ((tmp = opera1_usb_i2c_msgxfer(d,
    (msg[i].addr<<1)|(msg[i].flags&I2C_M_RD?0x01:0),
    msg[i].buf,
    msg[i].len
    )) != msg[i].len) {
    break;
    }
    if (dvb_usb_opera1_debug & 0x10)
    info("sending i2c message %d %d", tmp, msg[i].len);
    }
    mutex_unlock(&d.i2c_mutex);
    return num;
    }
#[no_mangle]
unsafe extern "C" fn opera1_i2c_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 opera1_i2c_func(struct i2c_adapter *adapter)
    {
    return I2C_FUNC_I2C;
    }
    static const struct i2c_algorithm opera1_i2c_algo = {
    .master_xfer = opera1_i2c_xfer,
    .functionality = opera1_i2c_func,
    };
    static int opera1_set_voltage(struct dvb_frontend *fe,
    enum fe_sec_voltage voltage)
    {
    static u8 command_13v[1]={0x00};
    static u8 command_18v[1]={0x01};
    struct i2c_msg msg[] = {
    {.addr = ADDR_B600_VOLTAGE_13V,.flags = 0,.buf = command_13v,.len = 1},
    };
    struct dvb_usb_adapter *udev_adap = fe.dvb.priv;
    if (voltage == SEC_VOLTAGE_18) {
    msg[0].addr = ADDR_B601_VOLTAGE_18V;
    msg[0].buf = command_18v;
    }
    i2c_transfer(&udev_adap.dev.i2c_adap, msg, 1);
    return 0;
    }
    static int opera1_stv0299_set_symbol_rate(struct dvb_frontend *fe, u32 srate,
    u32 ratio)
    {
    stv0299_writereg(fe, 0x13, 0x98);
    stv0299_writereg(fe, 0x14, 0x95);
    stv0299_writereg(fe, REG_1F_SYMBOLRATE_BYTE0, (ratio >> 16) & 0xff);
    stv0299_writereg(fe, REG_20_SYMBOLRATE_BYTE1, (ratio >> 8) & 0xff);
    stv0299_writereg(fe, REG_21_SYMBOLRATE_BYTE2, (ratio) & 0xf0);
    return 0;
    }
    static u8 opera1_inittab[] = {
    0x00, 0xa1,
    0x01, 0x15,
    0x02, 0x30,
    0x03, 0x00,
    0x04, 0x7d,
    0x05, 0x05,
    0x06, 0x02,
    0x07, 0x00,
    0x0b, 0x00,
    0x0c, 0x01,
    0x0d, 0x81,
    0x0e, 0x44,
    0x0f, 0x19,
    0x10, 0x3f,
    0x11, 0x84,
    0x12, 0xda,
    0x13, 0x98,
    0x14, 0x95,
    0x15, 0xc9,
    0x16, 0xeb,
    0x17, 0x00,
    0x18, 0x19,
    0x19, 0x8b,
    0x1a, 0x00,
    0x1b, 0x82,
    0x1c, 0x7f,
    0x1d, 0x00,
    0x1e, 0x00,
    REG_1F_SYMBOLRATE_BYTE0, 0x06,
    REG_20_SYMBOLRATE_BYTE1, 0x50,
    REG_21_SYMBOLRATE_BYTE2, 0x10,
    0x22, 0x00,
    0x23, 0x00,
    0x24, 0x37,
    0x25, 0xbc,
    0x26, 0x00,
    0x27, 0x00,
    0x28, 0x00,
    0x29, 0x1e,
    0x2a, 0x14,
    0x2b, 0x1f,
    0x2c, 0x09,
    0x2d, 0x0a,
    0x2e, 0x00,
    0x2f, 0x00,
    0x30, 0x00,
    0x31, 0x1f,
    0x32, 0x19,
    0x33, 0xfc,
    0x34, 0x13,
    0xff, 0xff,
    };
    static struct stv0299_config opera1_stv0299_config = {
    .demod_address = 0xd0>>1,
    .min_delay_ms = 100,
    .mclk = 88000000UL,
    .invert = 1,
    .skip_reinit = 0,
    .lock_output = STV0299_LOCKOUTPUT_0,
    .volt13_op0_op1 = STV0299_VOLT13_OP0,
    .inittab = opera1_inittab,
    .set_symbol_rate = opera1_stv0299_set_symbol_rate,
    };
#[no_mangle]
unsafe extern "C" fn opera1_frontend_attach(d: *mut dvb_usb_adapter) -> c_int {
    static int opera1_frontend_attach(struct dvb_usb_adapter *d)
    {
    d.fe_adap[0].fe = dvb_attach(stv0299_attach, &opera1_stv0299_config,
    &d.dev.i2c_adap);
    if ((d.fe_adap[0].fe) != core::ptr::null_mut()) {
    d.fe_adap[0].fe.ops.set_voltage = opera1_set_voltage;
    return 0;
    }
    info("not attached stv0299");
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn opera1_tuner_attach(adap: *mut dvb_usb_adapter) -> c_int {
    static int opera1_tuner_attach(struct dvb_usb_adapter *adap)
    {
    dvb_attach(
    dvb_pll_attach, adap.fe_adap[0].fe, 0xc0>>1,
    &adap.dev.i2c_adap, DVB_PLL_OPERA1
    );
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn opera1_power_ctrl(d: *mut dvb_usb_device, onoff: c_int) -> c_int {
    static int opera1_power_ctrl(struct dvb_usb_device *d, int onoff)
    {
    let mut val: u8 = onoff ? 0x01 : 0x00;
    if (dvb_usb_opera1_debug)
    info("power %s", onoff ? "on" : "off");
    return opera1_xilinx_rw(d.udev, 0xb7, val,
    &val, 1, OPERA_WRITE_MSG);
    }
#[no_mangle]
unsafe extern "C" fn opera1_streaming_ctrl(adap: *mut dvb_usb_adapter, onoff: c_int) -> c_int {
    static int opera1_streaming_ctrl(struct dvb_usb_adapter *adap, int onoff)
    {
    static u8 buf_start[2] = { 0xff, 0x03 };
    static u8 buf_stop[2] = { 0xff, 0x00 };
    struct i2c_msg start_tuner[] = {
    {.addr = ADDR_B1A6_STREAM_CTRL,.buf = onoff ? buf_start : buf_stop,.len = 2},
    };
    if (dvb_usb_opera1_debug)
    info("streaming %s", onoff ? "on" : "off");
    i2c_transfer(&adap.dev.i2c_adap, start_tuner, 1);
    return 0;
    }
    static int opera1_pid_filter(struct dvb_usb_adapter *adap, int index, u16 pid,
    int onoff)
    {
    u8 b_pid[3];
    struct i2c_msg msg[] = {
    {.addr = ADDR_B1A6_STREAM_CTRL,.buf = b_pid,.len = 3},
    };
    if (dvb_usb_opera1_debug)
    info("pidfilter index: %d pid: %d %s", index, pid,
    onoff ? "on" : "off");
    b_pid[0] = (2 * index) + 4;
    b_pid[1] = onoff ? (pid & 0xff) : (0x00);
    b_pid[2] = onoff ? ((pid >> 8) & 0xff) : (0x00);
    i2c_transfer(&adap.dev.i2c_adap, msg, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn opera1_pid_filter_control(adap: *mut dvb_usb_adapter, onoff: c_int) -> c_int {
    static int opera1_pid_filter_control(struct dvb_usb_adapter *adap, int onoff)
    {
    let mut u: c_int = 0x04;
    u8 b_pid[3];
    struct i2c_msg msg[] = {
    {.addr = ADDR_B1A6_STREAM_CTRL,.buf = b_pid,.len = 3},
    };
    if (dvb_usb_opera1_debug)
    info("%s hw-pidfilter", onoff ? "enable" : "disable");
    for (; u < 0x7e; u += 2) {
    b_pid[0] = u;
    b_pid[1] = 0;
    b_pid[2] = 0x80;
    i2c_transfer(&adap.dev.i2c_adap, msg, 1);
    }
    return 0;
    }
    static struct rc_map_table rc_map_opera1_table[] = {
    {0x5fa0, KEY_1},
    {0x51af, KEY_2},
    {0x5da2, KEY_3},
    {0x41be, KEY_4},
    {0x0bf5, KEY_5},
    {0x43bd, KEY_6},
    {0x47b8, KEY_7},
    {0x49b6, KEY_8},
    {0x05fa, KEY_9},
    {0x45ba, KEY_0},
    {0x09f6, KEY_CHANNELUP},	/*chanup */
    {0x1be5, KEY_CHANNELDOWN},	/*chandown */
    {0x5da3, KEY_VOLUMEDOWN},	/*voldown */
    {0x5fa1, KEY_VOLUMEUP},		/*volup */
    {0x07f8, KEY_SPACE},		/*tab */
    {0x1fe1, KEY_OK},		/*play ok */
    {0x1be4, KEY_ZOOM},		/*zoom */
    {0x59a6, KEY_MUTE},		/*mute */
    {0x5ba5, KEY_RADIO},		/*tv/f */
    {0x19e7, KEY_RECORD},		/*rec */
    {0x01fe, KEY_STOP},		/*Stop */
    {0x03fd, KEY_PAUSE},		/*pause */
    {0x03fc, KEY_SCREEN},		/*<- . */
    {0x07f9, KEY_CAMERA},		/*capture */
    {0x47b9, KEY_ESC},		/*exit */
    {0x43bc, KEY_POWER2},		/*power */
    };
#[no_mangle]
unsafe extern "C" fn opera1_rc_query(dev: *mut dvb_usb_device, event: *mut *mut u32, state: *mut c_int) -> c_int {
    static int opera1_rc_query(struct dvb_usb_device *dev, u32 * event, int *state)
    {
    struct opera1_state *opst = dev.priv;
    u8 rcbuffer[32];
    let mut startmarker1: u16 = 0x10ed;
    let mut startmarker2: u16 = 0x11ec;
    struct i2c_msg read_remote[] = {
    {.addr = ADDR_B880_READ_REMOTE,.buf = rcbuffer,.flags = I2C_M_RD,.len = 32},
    };
    let mut i: c_int = 0;
    let mut send_key: u32 = 0;
    if (i2c_transfer(&dev.i2c_adap, read_remote, 1) == 1) {
    for (i = 0; i < 32; i++) {
    if (rcbuffer[i])
    send_key |= 1;
    if (i < 31)
    send_key = send_key << 1;
    }
    if (send_key & 0x8000)
    send_key = (send_key << 1) | (send_key >> 15 & 0x01);
    if (send_key == 0xffff && opst.last_key_pressed != 0) {
// state = REMOTE_KEY_REPEAT;
// event = opst->last_key_pressed;
    return 0;
    }
    for (; send_key != 0;) {
    if (send_key >> 16 == startmarker2) {
    break;
    } else if (send_key >> 16 == startmarker1) {
    send_key =
    (send_key & 0xfffeffff) | (startmarker1 << 16);
    break;
    } else
    send_key >>= 1;
    }
    if (send_key == 0)
    return 0;
    send_key = (send_key & 0xffff) | 0x0100;
    for (i = 0; i < ARRAY_SIZE(rc_map_opera1_table); i++) {
    if (rc5_scan(&rc_map_opera1_table[i]) == (send_key & 0xffff)) {
// state = REMOTE_KEY_PRESSED;
// event = rc_map_opera1_table[i].keycode;
    opst.last_key_pressed =
    rc_map_opera1_table[i].keycode;
    break;
    }
    opst.last_key_pressed = 0;
    }
    } else
// state = REMOTE_NO_KEY_PRESSED;
    return 0;
    }
    enum {
    CYPRESS_OPERA1_COLD,
    OPERA1_WARM,
    };
    static const struct usb_device_id opera1_table[] = {
    DVB_USB_DEV(CYPRESS, CYPRESS_OPERA1_COLD),
    DVB_USB_DEV(OPERA1, OPERA1_WARM),
    { }
    };
    MODULE_DEVICE_TABLE(usb, opera1_table);
#[no_mangle]
unsafe extern "C" fn opera1_read_mac_address(d: *mut dvb_usb_device, mac[6]: u8) -> c_int {
    static int opera1_read_mac_address(struct dvb_usb_device *d, u8 mac[6])
    {
    int ret;
    u8 command[] = { READ_MAC_ADDR };
    ret = opera1_xilinx_rw(d.udev, 0xb1, 0xa0, command, 1, OPERA_WRITE_MSG);
    if (ret)
    return ret;
    ret = opera1_xilinx_rw(d.udev, 0xb1, 0xa1, mac, 6, OPERA_READ_MSG);
    if (ret)
    return ret;
    return 0;
    }
    static int opera1_xilinx_load_firmware(struct usb_device *dev,
    const char *filename)
    {
    const struct firmware *fw = core::ptr::null_mut();
    u8 *b, *p;
    let mut ret: c_int = 0, i,fpgasize=40;
    u8 testval;
    info("start downloading fpga firmware %s",filename);
    if ((ret = request_firmware(&fw, filename, &dev.dev)) != 0) {
    err("did not find the firmware file '%s'. You can use <kernel_dir>/scripts/get_dvb_firmware to get the firmware",
    filename);
    return ret;
    } else {
    p = kmalloc(fw.size, GFP_KERNEL);
    opera1_xilinx_rw(dev, 0xbc, 0x00, &testval, 1, OPERA_READ_MSG);
    if (p != core::ptr::null_mut() && testval != 0x67) {
    let mut reset: u8 = 0, fpga_command = 0;
    memcpy(p, fw.data, fw.size);
// clear fpga ?
    opera1_xilinx_rw(dev, 0xbc, 0xaa, &fpga_command, 1,
    OPERA_WRITE_MSG);
    for (i = 0; i < fw.size;) {
    if ( (fw.size - i) <fpgasize){
    fpgasize=fw.size-i;
    }
    b = (u8 *) p + i;
    if (opera1_xilinx_rw
    (dev, OPERA_WRITE_FX2, 0x0, b , fpgasize,
    OPERA_WRITE_MSG) != fpgasize
    ) {
    err("error while transferring firmware");
    ret = -EINVAL;
    break;
    }
    i = i + fpgasize;
    }
// restart the CPU
    if (ret || opera1_xilinx_rw
    (dev, 0xa0, 0xe600, &reset, 1,
    OPERA_WRITE_MSG) != 1) {
    err("could not restart the USB controller CPU.");
    ret = -EINVAL;
    }
    }
    }
    kfree(p);
    release_firmware(fw);
    return ret;
    }
    static struct dvb_usb_device_properties opera1_properties = {
    .caps = DVB_USB_IS_AN_I2C_ADAPTER,
    .usb_ctrl = CYPRESS_FX2,
    .firmware = "dvb-usb-opera-01.fw",
    .size_of_priv = sizeof(struct opera1_state),
    .power_ctrl = opera1_power_ctrl,
    .i2c_algo = &opera1_i2c_algo,
    .rc.legacy = {
    .rc_map_table = rc_map_opera1_table,
    .rc_map_size = ARRAY_SIZE(rc_map_opera1_table),
    .rc_interval = 200,
    .rc_query = opera1_rc_query,
    },
    .read_mac_address = opera1_read_mac_address,
    .generic_bulk_ctrl_endpoint = 0x00,
// parameter for the MPEG2-data transfer
    .num_adapters = 1,
    .adapter = {
    {
    .num_frontends = 1,
    .fe = {{
    .frontend_attach = opera1_frontend_attach,
    .streaming_ctrl = opera1_streaming_ctrl,
    .tuner_attach = opera1_tuner_attach,
    .caps =
    DVB_USB_ADAP_HAS_PID_FILTER |
    DVB_USB_ADAP_PID_FILTER_CAN_BE_TURNED_OFF,
    .pid_filter = opera1_pid_filter,
    .pid_filter_ctrl = opera1_pid_filter_control,
    .pid_filter_count = 252,
    .stream = {
    .type = USB_BULK,
    .count = 10,
    .endpoint = 0x82,
    .u = {
    .bulk = {
    .buffersize = 4096,
    }
    }
    },
    }},
    }
    },
    .num_device_descs = 1,
    .devices = {
    {"Opera1 DVB-S USB2.0",
    {&opera1_table[CYPRESS_OPERA1_COLD], core::ptr::null_mut()},
    {&opera1_table[OPERA1_WARM], core::ptr::null_mut()},
    },
    }
    };
    static int opera1_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct usb_device *udev = interface_to_usbdev(intf);
    if (le16_to_cpu(udev.descriptor.idProduct) == USB_PID_OPERA1_WARM &&
    le16_to_cpu(udev.descriptor.idVendor) == USB_VID_OPERA1 &&
    opera1_xilinx_load_firmware(udev, "dvb-usb-opera1-fpga-01.fw") != 0
    ) {
    return -EINVAL;
    }
    if (0 != dvb_usb_device_init(intf, &opera1_properties,
    THIS_MODULE, core::ptr::null_mut(), adapter_nr))
    return -EINVAL;
    return 0;
    }
    static struct usb_driver opera1_driver = {
    .name = "opera1",
    .probe = opera1_probe,
    .disconnect = dvb_usb_device_exit,
    .id_table = opera1_table,
    };
    module_usb_driver(opera1_driver);
    MODULE_AUTHOR("Mario Hlawitschka (c) dh1pa@amsat.org");
    MODULE_AUTHOR("Marco Gittler (c) g.marco@freenet.de");
    MODULE_DESCRIPTION("Driver for Opera1 DVB-S device");
    MODULE_VERSION("0.1");
    MODULE_LICENSE("GPL");
