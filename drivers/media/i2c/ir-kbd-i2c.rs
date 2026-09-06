//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ir-kbd-i2c.c
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
// keyboard input driver for i2c IR remote controls
//
// Copyright (c) 2000-2003 Gerd Knorr <kraxel@bytesex.org>
// modified for PixelView (BT878P+W/FM) by
// Michal Kochanowicz <mkochano@pld.org.pl>
// Christoph Bartelmus <lirc@bartelmus.de>
// modified for KNC ONE TV Station/Anubis Typhoon TView Tuner by
// Ulrich Mueller <ulrich.mueller42@web.de>
// modified for em2820 based USB TV tuners by
// Markus Rechberger <mrechberger@gmail.com>
// modified for DViCO Fusion HDTV 5 RT GOLD by
// Chaogui Zhang <czhang1974@gmail.com>
// modified for MSI TV@nywhere Plus by
// Henry Wong <henry@stuffedcow.net>
// Mark Schultz <n9xmj@yahoo.com>
// Brian Rogers <brian_rogers@comcast.net>
// modified for AVerMedia Cardbus by
// Oldrich Jedlicka <oldium.pro@seznam.cz>
// Zilog Transmitter portions/ideas were derived from GPLv2+ sources:
// - drivers/char/pctv_zilogir.[ch] from Hauppauge Broadway product
// Copyright 2011 Hauppauge Computer works
// - drivers/staging/media/lirc/lirc_zilog.c
// Copyright (c) 2000 Gerd Knorr <kraxel@goldbach.in-berlin.de>
// Michal Kochanowicz <mkochano@pld.org.pl>
// Christoph Bartelmus <lirc@bartelmus.de>
// Ulrich Mueller <ulrich.mueller42@web.de>
// Stefan Jahn <stefan@lkcc.org>
// Jerome Brock <jbrock@users.sourceforge.net>
// Thomas Reitmayr (treitmayr@yahoo.com)
// Mark Weaver <mark@npsl.co.uk>
// Jarod Wilson <jarod@redhat.com>
// Copyright (C) 2011 Andy Walls <awalls@md.metrocast.net>
//

pub const FLAG_TX: c_int = 1;
pub const FLAG_HDPVR: c_int = 2;
    static bool enable_hdpvr;
    module_param(enable_hdpvr, bool, 0644);
    static int get_key_haup_common(struct IR_i2c *ir, enum rc_proto *protocol,
    u32 *scancode, u8 *ptoggle, int size)
    {
    unsigned char buf[6];
    int start, range, toggle, dev, code, ircode, vendor;
// poll IR chip
    if (size != i2c_master_recv(ir.c, buf, size))
    return -EIO;
    if (buf[0] & 0x80) {
    let mut offset: c_int = (size == 6) ? 3 : 0;
// split rc5 data block ...
    start  = (buf[offset] >> 7) &    1;
    range  = (buf[offset] >> 6) &    1;
    toggle = (buf[offset] >> 5) &    1;
    dev    =  buf[offset]       & 0x1f;
    code   = (buf[offset+1] >> 2) & 0x3f;
// rc5 has two start bits
// the first bit must be one
// the second bit defines the command range:
// 1 = 0-63, 0 = 64 - 127
//
    if (!start)
// no key pressed
    return 0;
// filter out invalid key presses
    ircode = (start << 12) | (toggle << 11) | (dev << 6) | code;
    if ((ircode & 0x1fff) == 0x1fff)
    return 0;
    if (!range)
    code += 64;
    dev_dbg(&ir.rc.dev,
    "ir hauppauge (rc5): s%d r%d t%d dev=%d code=%d\n",
    start, range, toggle, dev, code);
// protocol = RC_PROTO_RC5;
// scancode = RC_SCANCODE_RC5(dev, code);
// ptoggle = toggle;
    return 1;
    } else if (size == 6 && (buf[0] & 0x40)) {
    code = buf[4];
    dev = buf[3];
    vendor = get_unaligned_be16(buf + 1);
    if (vendor == 0x800f) {
// ptoggle = (dev & 0x80) != 0;
// protocol = RC_PROTO_RC6_MCE;
    dev &= 0x7f;
    dev_dbg(&ir.rc.dev,
    "ir hauppauge (rc6-mce): t%d vendor=%d dev=%d code=%d\n",
// ptoggle, vendor, dev, code);
    } else {
// ptoggle = 0;
// protocol = RC_PROTO_RC6_6A_32;
    dev_dbg(&ir.rc.dev,
    "ir hauppauge (rc6-6a-32): vendor=%d dev=%d code=%d\n",
    vendor, dev, code);
    }
// scancode = RC_SCANCODE_RC6_6A(vendor, dev, code);
    return 1;
    }
    return 0;
    }
    static int get_key_haup(struct IR_i2c *ir, enum rc_proto *protocol,
    u32 *scancode, u8 *toggle)
    {
    return get_key_haup_common(ir, protocol, scancode, toggle, 3);
    }
    static int get_key_haup_xvr(struct IR_i2c *ir, enum rc_proto *protocol,
    u32 *scancode, u8 *toggle)
    {
    int ret;
    unsigned char buf[1] = { 0 };
//
// This is the same apparent "are you ready?" poll command observed
// watching Windows driver traffic and implemented in lirc_zilog. With
// this added, we get far saner remote behavior with z8 chips on usb
// connected devices, even with the default polling interval of 100ms.
//
    ret = i2c_master_send(ir.c, buf, 1);
    if (ret != 1)
    return (ret < 0) ? ret : -EINVAL;
    return get_key_haup_common(ir, protocol, scancode, toggle, 6);
    }
    static int get_key_pixelview(struct IR_i2c *ir, enum rc_proto *protocol,
    u32 *scancode, u8 *toggle)
    {
    int rc;
    unsigned char b;
// poll IR chip
    rc = i2c_master_recv(ir.c, &b, 1);
    if (rc != 1) {
    dev_dbg(&ir.rc.dev, "read error\n");
    if (rc < 0)
    return rc;
    return -EIO;
    }
// protocol = RC_PROTO_OTHER;
// scancode = b;
// toggle = 0;
    return 1;
    }
    static int get_key_fusionhdtv(struct IR_i2c *ir, enum rc_proto *protocol,
    u32 *scancode, u8 *toggle)
    {
    int rc;
    unsigned char buf[4];
// poll IR chip
    rc = i2c_master_recv(ir.c, buf, 4);
    if (rc != 4) {
    dev_dbg(&ir.rc.dev, "read error\n");
    if (rc < 0)
    return rc;
    return -EIO;
    }
    if (buf[0] != 0 || buf[1] != 0 || buf[2] != 0 || buf[3] != 0)
    dev_dbg(&ir.rc.dev, "%s: %*ph\n", __func__, 4, buf);
// no key pressed or signal from other ir remote
    if(buf[0] != 0x1 ||  buf[1] != 0xfe)
    return 0;
// protocol = RC_PROTO_UNKNOWN;
// scancode = buf[2];
// toggle = 0;
    return 1;
    }
    static int get_key_knc1(struct IR_i2c *ir, enum rc_proto *protocol,
    u32 *scancode, u8 *toggle)
    {
    int rc;
    unsigned char b;
// poll IR chip
    rc = i2c_master_recv(ir.c, &b, 1);
    if (rc != 1) {
    dev_dbg(&ir.rc.dev, "read error\n");
    if (rc < 0)
    return rc;
    return -EIO;
    }
// it seems that 0xFE indicates that a button is still hold
    down, while 0xff indicates that no button is hold
    down. 0xfe sequences are sometimes interrupted by 0xFF */
    dev_dbg(&ir.rc.dev, "key %02x\n", b);
    if (b == 0xff)
    return 0;
    if (b == 0xfe)
// keep old data
    return 1;
// protocol = RC_PROTO_UNKNOWN;
// scancode = b;
// toggle = 0;
    return 1;
    }
    static int get_key_geniatech(struct IR_i2c *ir, enum rc_proto *protocol,
    u32 *scancode, u8 *toggle)
    {
    int i, rc;
    unsigned char b;
// poll IR chip
    for (i = 0; i < 4; i++) {
    rc = i2c_master_recv(ir.c, &b, 1);
    if (rc == 1)
    break;
    msleep(20);
    }
    if (rc != 1) {
    dev_dbg(&ir.rc.dev, "read error\n");
    if (rc < 0)
    return rc;
    return -EIO;
    }
// don't repeat the key
    if (ir.old == b)
    return 0;
    ir.old = b;
// decode to RC5
    b &= 0x7f;
    b = (b - 1) / 2;
    dev_dbg(&ir.rc.dev, "key %02x\n", b);
// protocol = RC_PROTO_RC5;
// scancode = b;
// toggle = ir->old >> 7;
    return 1;
    }
    static int get_key_avermedia_cardbus(struct IR_i2c *ir, enum rc_proto *protocol,
    u32 *scancode, u8 *toggle)
    {
    unsigned char subaddr, key, keygroup;
    struct i2c_msg msg[] = { { .addr = ir.c.addr, .flags = 0,
    .buf = &subaddr, .len = 1},
    { .addr = ir.c.addr, .flags = I2C_M_RD,
    .buf = &key, .len = 1} };
    subaddr = 0x0d;
    if (2 != i2c_transfer(ir.c.adapter, msg, 2)) {
    dev_dbg(&ir.rc.dev, "read error\n");
    return -EIO;
    }
    if (key == 0xff)
    return 0;
    subaddr = 0x0b;
    msg[1].buf = &keygroup;
    if (2 != i2c_transfer(ir.c.adapter, msg, 2)) {
    dev_dbg(&ir.rc.dev, "read error\n");
    return -EIO;
    }
    if (keygroup == 0xff)
    return 0;
    dev_dbg(&ir.rc.dev, "read key 0x%02x/0x%02x\n", key, keygroup);
    if (keygroup < 2 || keygroup > 4) {
    dev_warn(&ir.rc.dev, "warning: invalid key group 0x%02x for key 0x%02x\n",
    keygroup, key);
    }
    key |= (keygroup & 1) << 6;
// protocol = RC_PROTO_UNKNOWN;
// scancode = key;
    if (ir.c.addr == 0x41) /* AVerMedia EM78P153 */
// scancode |= keygroup << 8;
// toggle = 0;
    return 1;
    }
// -----------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn ir_key_poll(ir: *mut IR_i2c) -> c_int {
    static int ir_key_poll(struct IR_i2c *ir)
    {
    let mut protocol: enum rc_proto = 0;
    let mut scancode: u32 = 0;
    let mut toggle: u8 = 0;
    int rc;
    dev_dbg(&ir.rc.dev, "%s\n", __func__);
    rc = ir.get_key(ir, &protocol, &scancode, &toggle);
    if (rc < 0) {
    dev_warn(&ir.rc.dev, "error %d\n", rc);
    return rc;
    }
    if (rc) {
    dev_dbg(&ir.rc.dev, "%s: proto = 0x%04x, scancode = 0x%08x\n",
    __func__, protocol, scancode);
    rc_keydown(ir.rc, protocol, scancode, toggle);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ir_work(work: *mut work_struct) {
    static void ir_work(struct work_struct *work)
    {
    int rc;
    struct IR_i2c *ir = container_of(work, struct IR_i2c, work.work);
//
// If the transmit code is holding the lock, skip polling for
// IR, we'll get it to it next time round
//
    if (mutex_trylock(&ir.lock)) {
    rc = ir_key_poll(ir);
    mutex_unlock(&ir.lock);
    if (rc == -ENODEV) {
    rc_unregister_device(ir.rc);
    rc_free_device(ir.rc);
    ir.rc = core::ptr::null_mut();
    return;
    }
    }
    schedule_delayed_work(&ir.work, msecs_to_jiffies(ir.polling_interval));
    }
#[no_mangle]
unsafe extern "C" fn ir_open(dev: *mut rc_dev) -> c_int {
    static int ir_open(struct rc_dev *dev)
    {
    struct IR_i2c *ir = dev.priv;
    schedule_delayed_work(&ir.work, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ir_close(dev: *mut rc_dev) {
    static void ir_close(struct rc_dev *dev)
    {
    struct IR_i2c *ir = dev.priv;
    cancel_delayed_work_sync(&ir.work);
    }
// Zilog Transmit Interface
pub const XTAL_FREQ: c_int = 18432000;
pub const ZILOG_SEND: c_uint = 0x80;
pub const ZILOG_UIR_END: c_uint = 0x40;
pub const ZILOG_INIT_END: c_uint = 0x20;
pub const ZILOG_LIR_END: c_uint = 0x10;
pub const ZILOG_STATUS_OK: c_uint = 0x80;
pub const ZILOG_STATUS_TX: c_uint = 0x40;
pub const ZILOG_STATUS_SET: c_uint = 0x20;
//
// As you can see here, very few different lengths of pulse and space
// can be encoded. This means that the hardware does not work well with
// recorded IR. It's best to work with generated IR, like from ir-ctl or
// the in-kernel encoders.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct code_block {
    pub length: u8,
    pub /: *mut *mut u16 pulse[7]; / not aligned,
    pub carrier_pulse: u8,
    pub carrier_space: u8,
    pub /: *mut *mut u16 space[8]; / not aligned,
    pub codes: [u8; 61],
    pub csum: [u8; 2],
    pub __packed: },
    static int send_data_block(struct IR_i2c *ir, int cmd,
    struct code_block *code_block)
    {
    pub ret: int i, j,,
    pub p: *mut u8 buf[5],,
    pub &code_block->length: p =,
    pub i++): for (i = 0; p < code_block->csum;,
    pub p++: *mut code_block->csum[i & 1] ^=,
    pub &code_block->length: p =,
    pub {: *mut *mut for (i = 0; i < sizeof(code_block);),
    pub i: *mut *mut int tosend = sizeof(code_block) -,
    if (tosend > 4)
    pub 4: tosend =,
    pub 1: buf[0] = i +,
    pub ++j): for (j = 0; j < tosend;,
    pub j]: buf[1 + j] = p[i +,
    pub buf): *mut *mut dev_dbg(&ir->rc->dev, "%ph", tosend + 1,,
    pub 1): ret = i2c_master_send(ir->tx_c, buf, tosend +,
    if (ret != tosend + 1) {
    dev_dbg(&ir.rc.dev,
    pub ret): "i2c_master_send failed with %d\n",,
    pub -EIO: return ret < 0 ? ret :,
    }
    pub tosend: i +=,
    }
    pub 0: buf[0] =,
    pub cmd: buf[1] =,
    pub 2): ret = i2c_master_send(ir->tx_c, buf,,
    if (ret != 2) {
    pub ret): dev_err(&ir->rc->dev, "i2c_master_send failed with %d\n",,
    pub -EIO: return ret < 0 ? ret :,
    }
    pub 5000): usleep_range(2000,,
    pub 1): ret = i2c_master_send(ir->tx_c, buf,,
    if (ret != 1) {
    pub ret): dev_err(&ir->rc->dev, "i2c_master_send failed with %d\n",,
    pub -EIO: return ret < 0 ? ret :,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn zilog_init(ir: *mut IR_i2c) -> c_int {
    static int zilog_init(struct IR_i2c *ir)
    {
    pub }: code_block code_block = { .length = sizeof(code_block),
    pub buf: [u8; 4],
    pub ret: c_int,
    pub &code_block.pulse[3]): put_unaligned_be16(0x1000,,
    pub &code_block): ret = send_data_block(ir, ZILOG_INIT_END,,
    if (ret)
    pub ret: return,
    pub 4): ret = i2c_master_recv(ir->tx_c, buf,,
    if (ret != 4) {
    dev_err(&ir.c.dev, "failed to retrieve firmware version: %d\n",
    pub -EIO: return ret < 0 ? ret :,
    }
    dev_info(&ir.c.dev, "Zilog/Hauppauge IR blaster firmware version %d.%d.%d\n",
    pub buf[3]): buf[1], buf[2],,
    pub 0: return,
    }
//
// If the last slot for pulse is the same as the current slot for pulse,
// then use slot no 7.
//
#[no_mangle]
unsafe extern "C" fn copy_codes(dst: *mut u8, src: *mut u8, count: c_uint) {
    static void copy_codes(u8 *dst, u8 *src, unsigned int count)
    {
    pub 0xff: u8 c, last =,
    while (count--) {
    pub src++: *mut c =,
    if ((c & 0xf0) == last) {
// dst++ = 0x70 | (c & 0xf);
    } else {
// dst++ = c;
    pub 0xf0: last = c &,
    }
    }
    }
//
// When looking for repeats, we don't care about the trailing space. This
// is set to the shortest possible anyway.
//
#[no_mangle]
unsafe extern "C" fn cmp_no_trail(a: *mut u8, b: *mut u8, count: c_uint) -> c_int {
    static int cmp_no_trail(u8 *a, u8 *b, unsigned int count)
    {
    while (--count) {
    if (*a++ != *b++)
    pub 1: return,
    }
    pub 0xf0): *mut *mut *mut return (a & 0xf0) - (b &,
    }
#[no_mangle]
unsafe extern "C" fn find_slot(array: *mut u16, size: c_uint, val: u16) -> c_int {
    static int find_slot(u16 *array, unsigned int size, u16 val)
    {
    pub i: c_int,
    pub {: for (i = 0; i < size; i++),
    if (get_unaligned_be16(&array[i]) == val) {
    pub i: return,
    } else if (!array[i]) {
    pub &array[i]): put_unaligned_be16(val,,
    pub i: return,
    }
    }
    pub -1: return,
    }
    static int zilog_ir_format(struct rc_dev *rcdev, unsigned int *txbuf,
    unsigned int count, struct code_block *code_block)
    {
    pub rcdev->priv: *mut *mut IR_i2c ir =,
    pub 0: int rep, i, l, p = 0, s, c =,
    pub repeating: bool,
    pub codes: [u8; 174],
    code_block.carrier_pulse = DIV_ROUND_CLOSEST(
    pub ir->carrier): *mut *mut ir->duty_cycle  XTAL_FREQ / 1000,,
    code_block.carrier_space = DIV_ROUND_CLOSEST(
    pub ir->carrier): *mut *mut (100 - ir->duty_cycle)  XTAL_FREQ / 1000,,
    pub {: for (i = 0; i < count; i++),
    if (c >= ARRAY_SIZE(codes) - 1) {
    pub transmit\n"): dev_warn(&rcdev->dev, "IR too long, cannot,
    pub -EINVAL: return,
    }
//
// Lengths more than 142220us cannot be encoded; also
// this checks for multiply overflow
//
    if (txbuf[i] > 142220)
    pub -EINVAL: return,
    pub 40000): *mut *mut l = DIV_ROUND_CLOSEST((XTAL_FREQ / 1000)  txbuf[i],,
    if (i & 1) {
    s = find_slot(code_block.space,
    pub l): ARRAY_SIZE(code_block->space),,
    if (s == -1) {
    pub transmit"): dev_warn(&rcdev->dev, "Too many different lengths spaces, cannot,
    pub -EINVAL: return,
    }
// We have a pulse and space
    pub s: codes[c++] = (p << 4) |,
    } else {
    p = find_slot(code_block.pulse,
    pub l): ARRAY_SIZE(code_block->pulse),,
    if (p == -1) {
    pub transmit"): dev_warn(&rcdev->dev, "Too many different lengths pulses, cannot,
    pub -EINVAL: return,
    }
    }
    }
// We have to encode the trailing pulse. Find the shortest space
    pub 0: s =,
    pub {: for (i = 1; i < ARRAY_SIZE(code_block->space); i++),
    pub get_unaligned_be16(&code_block->space[i]): u16 d =,
    if (get_unaligned_be16(&code_block.space[s]) > d)
    pub i: s =,
    }
    pub s: codes[c++] = (p << 4) |,
    pub c): dev_dbg(&rcdev->dev, "generated %d codes\n",,
//
// Are the last N codes (so pulse + space) repeating 3 times?
// if so we can shorten the codes list and use code 0xc0 to repeat
// them.
//
    pub false: repeating =,
    pub {: for (rep = c / 3; rep >= 1; rep--),
    if (!memcmp(&codes[c - rep * 3], &codes[c - rep * 2], rep) &&
    !cmp_no_trail(&codes[c - rep], &codes[c - rep * 2], rep)) {
    pub true: repeating =,
    }
    }
    if (repeating) {
// first copy any leading non-repeating
    pub 3: *mut *mut int leading = c - rep,
    if (leading >= ARRAY_SIZE(code_block.codes) - 3 - rep) {
    pub transmit\n"): dev_warn(&rcdev->dev, "IR too long, cannot,
    pub -EINVAL: return,
    }
    pub rep): dev_dbg(&rcdev->dev, "found trailing %d repeat\n",,
    pub leading): copy_codes(code_block->codes, codes,,
    pub 0x82: code_block->codes[leading] =,
    copy_codes(code_block.codes + leading + 1, codes + leading,
    pub rep: c = leading + 1 +,
    pub 0xc0: code_block->codes[c++] =,
    } else {
    if (c >= ARRAY_SIZE(code_block.codes) - 3) {
    pub transmit\n"): dev_warn(&rcdev->dev, "IR too long, cannot,
    pub -EINVAL: return,
    }
    pub repeat\n"): dev_dbg(&rcdev->dev, "found no trailing,
    pub 0x82: code_block->codes[0] =,
    pub c): copy_codes(code_block->codes + 1, codes,,
    pub 0xc4: code_block->codes[c++] =,
    }
    while (c < ARRAY_SIZE(code_block.codes))
    pub 0x83: code_block->codes[c++] =,
    pub 0: return,
    }
    static int zilog_tx(struct rc_dev *rcdev, unsigned int *txbuf,
    unsigned int count)
    {
    pub rcdev->priv: *mut *mut IR_i2c ir =,
    pub }: code_block code_block = { .length = sizeof(code_block),
    pub buf: [u8; 2],
    pub i: int ret,,
    pub &code_block): ret = zilog_ir_format(rcdev, txbuf, count,,
    if (ret)
    pub ret: return,
    pub mutex_lock_interruptible(&ir->lock): ret =,
    if (ret)
    pub ret: return,
    pub &code_block): ret = send_data_block(ir, ZILOG_UIR_END,,
    if (ret)
    pub out_unlock: goto,
    pub 1): ret = i2c_master_recv(ir->tx_c, buf,,
    if (ret != 1) {
    pub ret): dev_err(&ir->rc->dev, "i2c_master_recv failed with %d\n",,
    pub out_unlock: goto,
    }
    pub buf[0]): dev_dbg(&ir->rc->dev, "code set status: %02x\n",,
    if (buf[0] != (ZILOG_STATUS_OK | ZILOG_STATUS_SET)) {
    dev_err(&ir.rc.dev, "unexpected IR TX response %02x\n",
    pub -EIO: ret =,
    pub out_unlock: goto,
    }
    pub 0x00: buf[0] =,
    pub ZILOG_SEND: buf[1] =,
    pub 2): ret = i2c_master_send(ir->tx_c, buf,,
    if (ret != 2) {
    pub ret): dev_err(&ir->rc->dev, "i2c_master_send failed with %d\n",,
    if (ret >= 0)
    pub -EIO: ret =,
    pub out_unlock: goto,
    }
    pub sent\n"): dev_dbg(&ir->rc->dev, "send command,
//
// This bit NAKs until the device is ready, so we retry it
// sleeping a bit each time.  This seems to be what the windows
// driver does, approximately.
// Try for up to 1s.
//
    pub {: for (i = 0; i < 20; ++i),
    pub 1): ret = i2c_master_send(ir->tx_c, buf,,
    if (ret == 1)
    dev_dbg(&ir.rc.dev,
    "NAK expected: i2c_master_send failed with %d (try %d)\n",
    pub 1): ret, i +,
    }
    if (ret != 1) {
    dev_err(&ir.rc.dev,
    "IR TX chip never got ready: last i2c_master_send failed with %d\n",
    if (ret >= 0)
    pub -EIO: ret =,
    pub out_unlock: goto,
    }
    pub 1): ret = i2c_master_recv(ir->tx_c, buf,,
    if (ret != 1) {
    pub ret): dev_err(&ir->rc->dev, "i2c_master_recv failed with %d\n",,
    pub -EIO: ret =,
    pub out_unlock: goto,
    } else if (buf[0] != ZILOG_STATUS_OK) {
    dev_err(&ir.rc.dev, "unexpected IR TX response #2: %02x\n",
    pub -EIO: ret =,
    pub out_unlock: goto,
    }
    pub complete\n"): dev_dbg(&ir->rc->dev, "transmit,
// Oh good, it worked
    pub count: ret =,
    out_unlock:
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn zilog_tx_carrier(dev: *mut rc_dev, carrier: u32) -> c_int {
    static int zilog_tx_carrier(struct rc_dev *dev, u32 carrier)
    {
    pub dev->priv: *mut *mut IR_i2c ir =,
    if (carrier > 500000 || carrier < 20000)
    pub -EINVAL: return,
    pub carrier: ir->carrier =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn zilog_tx_duty_cycle(dev: *mut rc_dev, duty_cycle: u32) -> c_int {
    static int zilog_tx_duty_cycle(struct rc_dev *dev, u32 duty_cycle)
    {
    pub dev->priv: *mut *mut IR_i2c ir =,
    pub duty_cycle: ir->duty_cycle =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ir_probe(client: *mut i2c_client) -> c_int {
    static int ir_probe(struct i2c_client *client)
    {
    pub i2c_client_get_device_id(client): *const *const i2c_device_id id =,
    pub NULL: *mut *mut char ir_codes =,
    pub NULL: *const *const char name =,
    pub RC_PROTO_BIT_UNKNOWN: u64 rc_proto =,
    pub ir: *mut IR_i2c,
    pub NULL: *mut *mut rc_dev rc =,
    pub client->adapter: *mut *mut i2c_adapter adap =,
    pub client->addr: unsigned short addr =,
    pub 0: bool probe_tx = (id->driver_data & FLAG_TX) !=,
    pub err: c_int,
    if ((id.driver_data & FLAG_HDPVR) && !enable_hdpvr) {
    pub enable\n"): dev_err(&client->dev, "IR for HDPVR is known to cause problems during recording, use enable_hdpvr modparam to,
    pub -ENODEV: return,
    }
    pub GFP_KERNEL): *mut *mut ir = devm_kzalloc(&client->dev, sizeof(ir),,
    if (!ir)
    pub -ENOMEM: return,
    pub client: ir->c =,
    pub DEFAULT_POLLING_INTERVAL: ir->polling_interval =,
    pub ir): i2c_set_clientdata(client,,
    switch(addr) {
    case 0x64:
    pub "Pixelview": name =,
    pub get_key_pixelview: ir->get_key =,
    pub RC_PROTO_BIT_OTHER: rc_proto =,
    pub RC_MAP_EMPTY: ir_codes =,
    case 0x18:
    case 0x1f:
    case 0x1a:
    pub "Hauppauge": name =,
    pub get_key_haup: ir->get_key =,
    pub RC_PROTO_BIT_RC5: rc_proto =,
    pub RC_MAP_HAUPPAUGE: ir_codes =,
    case 0x30:
    pub One": name = "KNC,
    pub get_key_knc1: ir->get_key =,
    pub RC_PROTO_BIT_OTHER: rc_proto =,
    pub RC_MAP_EMPTY: ir_codes =,
    case 0x33:
    pub "Geniatech": name =,
    pub get_key_geniatech: ir->get_key =,
    pub RC_PROTO_BIT_RC5: rc_proto =,
    pub RC_MAP_TOTAL_MEDIA_IN_HAND_02: ir_codes =,
    pub 0xfc: ir->old =,
    case 0x6b:
    pub "FusionHDTV": name =,
    pub get_key_fusionhdtv: ir->get_key =,
    pub RC_PROTO_BIT_UNKNOWN: rc_proto =,
    pub RC_MAP_FUSIONHDTV_MCE: ir_codes =,
    case 0x40:
    pub remote": name = "AVerMedia Cardbus,
    pub get_key_avermedia_cardbus: ir->get_key =,
    pub RC_PROTO_BIT_OTHER: rc_proto =,
    pub RC_MAP_AVERMEDIA_CARDBUS: ir_codes =,
    case 0x41:
    pub EM78P153": name = "AVerMedia,
    pub get_key_avermedia_cardbus: ir->get_key =,
    pub RC_PROTO_BIT_OTHER: rc_proto =,
// RM-KV remote, seems to be same as RM-K6
    pub RC_MAP_AVERMEDIA_M733A_RM_K6: ir_codes =,
    case 0x71:
    pub Z8": name = "Hauppauge/Zilog,
    pub get_key_haup_xvr: ir->get_key =,
    rc_proto    = RC_PROTO_BIT_RC5 | RC_PROTO_BIT_RC6_MCE |
    pub RC_MAP_HAUPPAUGE: ir_codes =,
    pub 125: ir->polling_interval =,
    pub true: probe_tx =,
    }
// Let the caller override settings
    if (client.dev.platform_data) {
    const struct IR_i2c_init_data *init_data =
    pub init_data->ir_codes: ir_codes =,
    pub init_data->rc_dev: rc =,
    pub init_data->name: name =,
    if (init_data.type)
    pub init_data->type: rc_proto =,
    if (init_data.polling_interval)
    pub init_data->polling_interval: ir->polling_interval =,
    switch (init_data.internal_get_key_func) {
    case IR_KBD_GET_KEY_CUSTOM:
// The bridge driver provided us its own function
    pub init_data->get_key: ir->get_key =,
    case IR_KBD_GET_KEY_PIXELVIEW:
    pub get_key_pixelview: ir->get_key =,
    case IR_KBD_GET_KEY_HAUP:
    pub get_key_haup: ir->get_key =,
    case IR_KBD_GET_KEY_KNC1:
    pub get_key_knc1: ir->get_key =,
    case IR_KBD_GET_KEY_GENIATECH:
    pub get_key_geniatech: ir->get_key =,
    case IR_KBD_GET_KEY_FUSIONHDTV:
    pub get_key_fusionhdtv: ir->get_key =,
    case IR_KBD_GET_KEY_HAUP_XVR:
    pub get_key_haup_xvr: ir->get_key =,
    case IR_KBD_GET_KEY_AVERMEDIA_CARDBUS:
    pub get_key_avermedia_cardbus: ir->get_key =,
    }
    }
    if (!rc) {
//
// If platform_data doesn't specify rc_dev, initialize it
// internally
//
    pub rc_allocate_device(RC_DRIVER_SCANCODE): rc =,
    if (!rc)
    pub -ENOMEM: return,
    }
    pub rc: ir->rc =,
// Make sure we are all setup before going on
    if (!name || !ir.get_key || !rc_proto || !ir_codes) {
    dev_warn(&client.dev, "Unsupported device at address 0x%02x\n",
    pub -ENODEV: err =,
    pub err_out_free: goto,
    }
    pub ir_codes: ir->ir_codes =,
    snprintf(ir.phys, sizeof(ir.phys), "%s/%s", dev_name(&adap.dev),
//
// Initialize input_dev fields
// It doesn't make sense to allow overriding them via platform_data
//
    pub BUS_I2C: rc->input_id.bustype =,
    pub ir->phys: rc->input_phys =,
    pub name: rc->device_name =,
    pub &client->dev: rc->dev.parent =,
    pub ir: rc->priv =,
    pub ir_open: rc->open =,
    pub ir_close: rc->close =,
//
// Initialize the other fields of rc_dev
//
    pub ir->ir_codes: rc->map_name =,
    pub rc_proto: rc->allowed_protocols =,
    if (!rc.driver_name)
    pub KBUILD_MODNAME: rc->driver_name =,
    pub ir_work): INIT_DELAYED_WORK(&ir->work,,
    if (probe_tx) {
    pub 0x70): ir->tx_c = i2c_new_dummy_device(client->adapter,,
    if (IS_ERR(ir.tx_c)) {
    pub address"): dev_err(&client->dev, "failed to setup tx i2c,
    pub PTR_ERR(ir->tx_c): err =,
    pub err_out_free: goto,
    } else if (!zilog_init(ir)) {
    pub 38000: ir->carrier =,
    pub 40: ir->duty_cycle =,
    pub zilog_tx: rc->tx_ir =,
    pub zilog_tx_carrier: rc->s_tx_carrier =,
    pub zilog_tx_duty_cycle: rc->s_tx_duty_cycle =,
    }
    }
    pub rc_register_device(rc): err =,
    if (err)
    pub err_out_free: goto,
    pub 0: return,
    err_out_free:
    if (!IS_ERR(ir.tx_c))
// Only frees rc if it were allocated internally
    pub err: return,
    }
#[no_mangle]
unsafe extern "C" fn ir_remove(client: *mut i2c_client) {
    static void ir_remove(struct i2c_client *client)
    {
    pub i2c_get_clientdata(client): *mut *mut IR_i2c ir =,
    }
    static const struct i2c_device_id ir_kbd_id[] = {
// Generic entry for any IR receiver
    { .name = "ir_video", .driver_data = 0 },
// IR device specific entries should be added here
    { .name = "ir_z8f0811_haup", .driver_data = FLAG_TX },
    { .name = "ir_z8f0811_hdpvr", .driver_data = FLAG_TX | FLAG_HDPVR },
    { }
}

    MODULE_DEVICE_TABLE(i2c, ir_kbd_id);
    static struct i2c_driver ir_kbd_driver = {
    .driver = {
    .name   = "ir-kbd-i2c",
    },
    .probe          = ir_probe,
    .remove         = ir_remove,
    .id_table       = ir_kbd_id,
    };
    module_i2c_driver(ir_kbd_driver);
// -----------------------------------------------------------------------
    MODULE_AUTHOR("Gerd Knorr, Michal Kochanowicz, Christoph Bartelmus, Ulrich Mueller");
    MODULE_DESCRIPTION("input driver for i2c IR remote controls");
    MODULE_LICENSE("GPL");
