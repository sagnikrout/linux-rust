//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/ttusb-dec/ttusb_dec.c
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
// TTUSB DEC Driver
//
// Copyright (C) 2003-2004 Alex Woods <linux-dvb@giblets.org>
// IR support by Peter Beutner <p.beutner@gmx.net>
//

    static int debug;
    static int output_pva;
    static int enable_rc;
    module_param(debug, int, 0644);
    MODULE_PARM_DESC(debug, "Turn on/off debugging (default:off).");
    module_param(output_pva, int, 0444);
    MODULE_PARM_DESC(output_pva, "Output PVA from dvr device (default:off)");
    module_param(enable_rc, int, 0644);
    MODULE_PARM_DESC(enable_rc, "Turn on/off IR remote control(default: off)");
    DVB_DEFINE_MOD_OPT_ADAPTER_NR(adapter_nr);

pub const COMMAND_PIPE: c_uint = 0x03;
pub const RESULT_PIPE: c_uint = 0x04;
pub const IN_PIPE: c_uint = 0x08;
pub const OUT_PIPE: c_uint = 0x07;
pub const IRQ_PIPE: c_uint = 0x0A;
pub const COMMAND_PACKET_SIZE: c_uint = 0x3c;
pub const ARM_PACKET_SIZE: c_uint = 0x1000;
pub const IRQ_PACKET_SIZE: c_uint = 0x8;
pub const ISO_BUF_COUNT: c_uint = 0x04;
pub const FRAMES_PER_ISO_BUF: c_uint = 0x04;
pub const ISO_FRAME_SIZE: c_uint = 0x0380;
pub const MAX_PVA_LENGTH: c_int = 6144;
    enum ttusb_dec_model {
    TTUSB_DEC2000T,
    TTUSB_DEC2540T,
    TTUSB_DEC3000S
    };
    enum ttusb_dec_packet_type {
    TTUSB_DEC_PACKET_PVA,
    TTUSB_DEC_PACKET_SECTION,
    TTUSB_DEC_PACKET_EMPTY
    };
    enum ttusb_dec_interface {
    TTUSB_DEC_INTERFACE_INITIAL,
    TTUSB_DEC_INTERFACE_IN,
    TTUSB_DEC_INTERFACE_OUT
    };
    typedef int (dvb_filter_pes2ts_cb_t) (void *, unsigned char *);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_filter_pes2ts {
    pub buf: [c_uchar; 188],
    pub cc: c_uchar,
    pub cb: *mut dvb_filter_pes2ts_cb_t,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttusb_dec {
    pub model: enum ttusb_dec_model,
    pub model_name: *mut c_char,
    pub firmware_name: *mut c_char,
    pub can_playback: c_int,
// DVB bits
    pub adapter: dvb_adapter,
    pub dmxdev: dmxdev,
    pub demux: dvb_demux,
    pub frontend: dmx_frontend,
    pub dvb_net: dvb_net,
    pub fe: *mut *mut dvb_frontend,
    pub pid: [u16; DMX_PES_OTHER],
// USB bits
    pub udev: *mut usb_device,
    pub trans_count: u8,
    pub command_pipe: c_uint,
    pub result_pipe: c_uint,
    pub in_pipe: c_uint,
    pub out_pipe: c_uint,
    pub irq_pipe: c_uint,
    pub interface: enum ttusb_dec_interface,
    pub usb_mutex: mutex,
    pub irq_buffer: *mut c_void,
    pub irq_urb: *mut urb,
    pub irq_dma_handle: dma_addr_t,
    pub iso_buffer: *mut c_void,
    pub iso_urb: [*mut urb; ISO_BUF_COUNT],
    pub iso_stream_count: c_int,
    pub iso_mutex: mutex,
    pub 4]: u8 packet[MAX_PVA_LENGTH +,
    pub packet_type: enum ttusb_dec_packet_type,
    pub packet_state: c_int,
    pub packet_length: c_int,
    pub packet_payload_length: c_int,
    pub next_packet_id: u16,
    pub pva_stream_count: c_int,
    pub filter_stream_count: c_int,
    pub a_pes2ts: dvb_filter_pes2ts,
    pub v_pes2ts: dvb_filter_pes2ts,
    pub MAX_PVA_LENGTH]: u8 v_pes[16 +,
    pub v_pes_length: c_int,
    pub v_pes_postbytes: c_int,
    pub urb_frame_list: list_head,
    pub urb_bh_work: work_struct,
    pub urb_frame_list_lock: spinlock_t,
    pub audio_filter: *mut dvb_demux_filter,
    pub video_filter: *mut dvb_demux_filter,
    pub filter_info_list: list_head,
    pub filter_info_list_lock: spinlock_t,
    pub rc_input_dev: *mut input_dev,
    pub rc_phys: [c_char; 64],
    pub /: *mut *mut int active; / Loaded successfully,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct urb_frame {
    pub data: [u8; ISO_FRAME_SIZE],
    pub length: c_int,
    pub urb_frame_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_info {
    pub stream_id: u8,
    pub filter: *mut dvb_demux_filter,
    pub filter_info_list: list_head,
}

    static u16 rc_keys[] = {
    KEY_POWER,
    KEY_MUTE,
    KEY_1,
    KEY_2,
    KEY_3,
    KEY_4,
    KEY_5,
    KEY_6,
    KEY_7,
    KEY_8,
    KEY_9,
    KEY_0,
    KEY_CHANNELUP,
    KEY_VOLUMEDOWN,
    KEY_OK,
    KEY_VOLUMEUP,
    KEY_CHANNELDOWN,
    KEY_PREVIOUS,
    KEY_ESC,
    KEY_RED,
    KEY_GREEN,
    KEY_YELLOW,
    KEY_BLUE,
    KEY_OPTION,
    KEY_M,
    KEY_RADIO
    };
    static void dvb_filter_pes2ts_init(struct dvb_filter_pes2ts *p2ts,
    unsigned short pid,
    dvb_filter_pes2ts_cb_t *cb, void *priv)
    {
    unsigned char *buf=p2ts.buf;
    buf[0]=0x47;
    buf[1]=(pid>>8);
    buf[2]=pid&0xff;
    p2ts.cc=0;
    p2ts.cb=cb;
    p2ts.priv=priv;
    }
    static int dvb_filter_pes2ts(struct dvb_filter_pes2ts *p2ts,
    unsigned char *pes, int len, int payload_start)
    {
    unsigned char *buf=p2ts.buf;
    let mut ret: c_int = 0, rest;
// len=6+((pes[4]<<8)|pes[5]);
    if (payload_start)
    buf[1]|=0x40;
    else
    buf[1]&=~0x40;
    while (len>=184) {
    buf[3]=0x10|((p2ts.cc++)&0x0f);
    memcpy(buf+4, pes, 184);
    if ((ret=p2ts.cb(p2ts.priv, buf)))
    return ret;
    len-=184; pes+=184;
    buf[1]&=~0x40;
    }
    if (!len)
    return 0;
    buf[3]=0x30|((p2ts.cc++)&0x0f);
    rest=183-len;
    if (rest) {
    buf[5]=0x00;
    if (rest-1)
    memset(buf+6, 0xff, rest-1);
    }
    buf[4]=rest;
    memcpy(buf+5+rest, pes, len);
    return p2ts.cb(p2ts.priv, buf);
    }
    static void ttusb_dec_set_model(struct ttusb_dec *dec,
    enum ttusb_dec_model model);
#[no_mangle]
unsafe extern "C" fn ttusb_dec_handle_irq(urb: *mut urb) {
    static void ttusb_dec_handle_irq( struct urb *urb)
    {
    struct ttusb_dec *dec = urb.context;
    char *buffer = dec.irq_buffer;
    int retval;
    let mut index: c_int = buffer[4];
    switch(urb.status) {
    case 0: /*success*/
    break;
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
    case -ETIME:
// this urb is dead, cleanup
    dprintk("%s:urb shutting down with status: %d\n",
    __func__, urb.status);
    return;
    default:
    dprintk("%s:nonzero status received: %d\n",
    __func__,urb.status);
    goto exit;
    }
    if ((buffer[0] == 0x1) && (buffer[2] == 0x15))  {
//
// IR - Event
//
// this is an fact a bit too simple implementation;
// the box also reports a keyrepeat signal
// (with buffer[3] == 0x40) in an interval of ~100ms.
// But to handle this correctly we had to imlemenent some
// kind of timer which signals a 'key up' event if no
// keyrepeat signal is received for lets say 200ms.
// this should/could be added later ...
// for now lets report each signal as a key down and up
//
    if (index - 1 < ARRAY_SIZE(rc_keys)) {
    dprintk("%s:rc signal:%d\n", __func__, index);
    input_report_key(dec.rc_input_dev, rc_keys[index - 1], 1);
    input_sync(dec.rc_input_dev);
    input_report_key(dec.rc_input_dev, rc_keys[index - 1], 0);
    input_sync(dec.rc_input_dev);
    }
    }
    exit:
    retval = usb_submit_urb(urb, GFP_ATOMIC);
    if (retval)
    printk("%s - usb_commit_urb failed with result: %d\n",
    __func__, retval);
    }
#[no_mangle]
unsafe extern "C" fn crc16(crc: u16, buf: *const u8, len: usize) -> u16 {
    static u16 crc16(u16 crc, const u8 *buf, size_t len)
    {
    u16 tmp;
    while (len--) {
    crc ^= *buf++;
    crc ^= (u8)crc >> 4;
    tmp = (u8)crc;
    crc ^= (tmp ^ (tmp << 1)) << 4;
    }
    return crc;
    }
    static int ttusb_dec_send_command(struct ttusb_dec *dec, const u8 command,
    int param_length, const u8 params[],
    int *result_length, u8 cmd_result[])
    {
    int result, actual_len;
    u8 *b;
    dprintk("%s\n", __func__);
    b = kzalloc(COMMAND_PACKET_SIZE + 4, GFP_KERNEL);
    if (!b)
    return -ENOMEM;
    result = mutex_lock_interruptible(&dec.usb_mutex);
    if (result) {
    printk("%s: Failed to lock usb mutex.\n", __func__);
    goto err_free;
    }
    b[0] = 0xaa;
    b[1] = ++dec.trans_count;
    b[2] = command;
    b[3] = param_length;
    if (params)
    memcpy(&b[4], params, param_length);
    if (debug) {
    printk(KERN_DEBUG "%s: command: %*ph\n",
    __func__, param_length, b);
    }
    result = usb_bulk_msg(dec.udev, dec.command_pipe, b,
    COMMAND_PACKET_SIZE + 4, &actual_len, 1000);
    if (result) {
    printk("%s: command bulk message failed: error %d\n",
    __func__, result);
    goto err_mutex_unlock;
    }
    result = usb_bulk_msg(dec.udev, dec.result_pipe, b,
    COMMAND_PACKET_SIZE + 4, &actual_len, 1000);
    if (result) {
    printk("%s: result bulk message failed: error %d\n",
    __func__, result);
    goto err_mutex_unlock;
    } else {
    if (debug) {
    printk(KERN_DEBUG "%s: result: %*ph\n",
    __func__, actual_len, b);
    }
    if (result_length)
// result_length = b[3];
    if (cmd_result && b[3] > 0)
    memcpy(cmd_result, &b[4], b[3]);
    }
    err_mutex_unlock:
    mutex_unlock(&dec.usb_mutex);
    err_free:
    kfree(b);
    return result;
    }
    static int ttusb_dec_get_stb_state (struct ttusb_dec *dec, unsigned int *mode,
    unsigned int *model, unsigned int *version)
    {
    u8 c[COMMAND_PACKET_SIZE];
    int c_length;
    int result;
    __be32 tmp;
    dprintk("%s\n", __func__);
    result = ttusb_dec_send_command(dec, 0x08, 0, core::ptr::null_mut(), &c_length, c);
    if (result)
    return result;
    if (c_length >= 0x0c) {
    if (mode != core::ptr::null_mut()) {
    memcpy(&tmp, c, 4);
// mode = ntohl(tmp);
    }
    if (model != core::ptr::null_mut()) {
    memcpy(&tmp, &c[4], 4);
// model = ntohl(tmp);
    }
    if (version != core::ptr::null_mut()) {
    memcpy(&tmp, &c[8], 4);
// version = ntohl(tmp);
    }
    return 0;
    } else {
    return -ENOENT;
    }
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_audio_pes2ts_cb(priv: *mut c_void, data: *mut c_uchar) -> c_int {
    static int ttusb_dec_audio_pes2ts_cb(void *priv, unsigned char *data)
    {
    struct ttusb_dec *dec = priv;
    dec.audio_filter.feed.cb.ts(data, 188, core::ptr::null_mut(), 0,
    &dec.audio_filter.feed.feed.ts, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_video_pes2ts_cb(priv: *mut c_void, data: *mut c_uchar) -> c_int {
    static int ttusb_dec_video_pes2ts_cb(void *priv, unsigned char *data)
    {
    struct ttusb_dec *dec = priv;
    dec.video_filter.feed.cb.ts(data, 188, core::ptr::null_mut(), 0,
    &dec.video_filter.feed.feed.ts, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_set_pids(dec: *mut ttusb_dec) {
    static void ttusb_dec_set_pids(struct ttusb_dec *dec)
    {
    u8 b[] = { 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff };
    let mut pcr: __be16 = htons(dec.pid[DMX_PES_PCR]);
    let mut audio: __be16 = htons(dec.pid[DMX_PES_AUDIO]);
    let mut video: __be16 = htons(dec.pid[DMX_PES_VIDEO]);
    dprintk("%s\n", __func__);
    memcpy(&b[0], &pcr, 2);
    memcpy(&b[2], &audio, 2);
    memcpy(&b[4], &video, 2);
    ttusb_dec_send_command(dec, 0x50, sizeof(b), b, core::ptr::null_mut(), core::ptr::null_mut());
    dvb_filter_pes2ts_init(&dec.a_pes2ts, dec.pid[DMX_PES_AUDIO],
    ttusb_dec_audio_pes2ts_cb, dec);
    dvb_filter_pes2ts_init(&dec.v_pes2ts, dec.pid[DMX_PES_VIDEO],
    ttusb_dec_video_pes2ts_cb, dec);
    dec.v_pes_length = 0;
    dec.v_pes_postbytes = 0;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_process_pva(dec: *mut ttusb_dec, pva: *mut u8, length: c_int) {
    static void ttusb_dec_process_pva(struct ttusb_dec *dec, u8 *pva, int length)
    {
    if (length < 8) {
    printk("%s: packet too short - discarding\n", __func__);
    return;
    }
    if (length > 8 + MAX_PVA_LENGTH) {
    printk("%s: packet too long - discarding\n", __func__);
    return;
    }
    switch (pva[2]) {
    case 0x01: {		/* VideoStream */
    let mut prebytes: c_int = pva[5] & 0x03;
    let mut postbytes: c_int = (pva[5] & 0x0c) >> 2;
    __be16 v_pes_payload_length;
    if (output_pva) {
    dec.video_filter.feed.cb.ts(pva, length, core::ptr::null_mut(), 0,
    &dec.video_filter.feed.feed.ts, core::ptr::null_mut());
    return;
    }
    if (dec.v_pes_postbytes > 0 &&
    dec.v_pes_postbytes == prebytes) {
    memcpy(&dec.v_pes[dec.v_pes_length],
    &pva[12], prebytes);
    dvb_filter_pes2ts(&dec.v_pes2ts, dec.v_pes,
    dec.v_pes_length + prebytes, 1);
    }
    if (pva[5] & 0x10) {
    dec.v_pes[7] = 0x80;
    dec.v_pes[8] = 0x05;
    dec.v_pes[9] = 0x21 | ((pva[8] & 0xc0) >> 5);
    dec.v_pes[10] = ((pva[8] & 0x3f) << 2) |
    ((pva[9] & 0xc0) >> 6);
    dec.v_pes[11] = 0x01 |
    ((pva[9] & 0x3f) << 2) |
    ((pva[10] & 0x80) >> 6);
    dec.v_pes[12] = ((pva[10] & 0x7f) << 1) |
    ((pva[11] & 0xc0) >> 7);
    dec.v_pes[13] = 0x01 | ((pva[11] & 0x7f) << 1);
    memcpy(&dec.v_pes[14], &pva[12 + prebytes],
    length - 12 - prebytes);
    dec.v_pes_length = 14 + length - 12 - prebytes;
    } else {
    dec.v_pes[7] = 0x00;
    dec.v_pes[8] = 0x00;
    memcpy(&dec.v_pes[9], &pva[8], length - 8);
    dec.v_pes_length = 9 + length - 8;
    }
    dec.v_pes_postbytes = postbytes;
    if (dec.v_pes[9 + dec.v_pes[8]] == 0x00 &&
    dec.v_pes[10 + dec.v_pes[8]] == 0x00 &&
    dec.v_pes[11 + dec.v_pes[8]] == 0x01)
    dec.v_pes[6] = 0x84;
    else
    dec.v_pes[6] = 0x80;
    v_pes_payload_length = htons(dec.v_pes_length - 6 +
    postbytes);
    memcpy(&dec.v_pes[4], &v_pes_payload_length, 2);
    if (postbytes == 0)
    dvb_filter_pes2ts(&dec.v_pes2ts, dec.v_pes,
    dec.v_pes_length, 1);
    break;
    }
    case 0x02:		/* MainAudioStream */
    if (output_pva) {
    dec.audio_filter.feed.cb.ts(pva, length, core::ptr::null_mut(), 0,
    &dec.audio_filter.feed.feed.ts, core::ptr::null_mut());
    return;
    }
    dvb_filter_pes2ts(&dec.a_pes2ts, &pva[8], length - 8,
    pva[5] & 0x10);
    break;
    default:
    printk("%s: unknown PVA type: %02x.\n", __func__,
    pva[2]);
    break;
    }
    }
    static void ttusb_dec_process_filter(struct ttusb_dec *dec, u8 *packet,
    int length)
    {
    struct list_head *item;
    struct filter_info *finfo;
    struct dvb_demux_filter *filter = core::ptr::null_mut();
    unsigned long flags;
    u8 sid;
    sid = packet[1];
    spin_lock_irqsave(&dec.filter_info_list_lock, flags);
    for (item = dec.filter_info_list.next; item != &dec.filter_info_list;
    item = item.next) {
    finfo = list_entry(item, struct filter_info, filter_info_list);
    if (finfo.stream_id == sid) {
    filter = finfo.filter;
    break;
    }
    }
    spin_unlock_irqrestore(&dec.filter_info_list_lock, flags);
    if (filter)
    filter.feed.cb.sec(&packet[2], length - 2, core::ptr::null_mut(), 0,
    &filter.filter, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_process_packet(dec: *mut ttusb_dec) {
    static void ttusb_dec_process_packet(struct ttusb_dec *dec)
    {
    int i;
    let mut csum: u16 = 0;
    u16 packet_id;
    if (dec.packet_length % 2) {
    printk("%s: odd sized packet - discarding\n", __func__);
    return;
    }
    for (i = 0; i < dec.packet_length; i += 2)
    csum ^= ((dec.packet[i] << 8) + dec.packet[i + 1]);
    if (csum) {
    printk("%s: checksum failed - discarding\n", __func__);
    return;
    }
    packet_id = dec.packet[dec.packet_length - 4] << 8;
    packet_id += dec.packet[dec.packet_length - 3];
    if ((packet_id != dec.next_packet_id) && dec.next_packet_id) {
    printk("%s: warning: lost packets between %u and %u\n",
    __func__, dec.next_packet_id - 1, packet_id);
    }
    if (packet_id == 0xffff)
    dec.next_packet_id = 0x8000;
    else
    dec.next_packet_id = packet_id + 1;
    switch (dec.packet_type) {
    case TTUSB_DEC_PACKET_PVA:
    if (dec.pva_stream_count)
    ttusb_dec_process_pva(dec, dec.packet,
    dec.packet_payload_length);
    break;
    case TTUSB_DEC_PACKET_SECTION:
    if (dec.filter_stream_count)
    ttusb_dec_process_filter(dec, dec.packet,
    dec.packet_payload_length);
    break;
    case TTUSB_DEC_PACKET_EMPTY:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn swap_bytes(b: *mut u8, length: c_int) {
    static void swap_bytes(u8 *b, int length)
    {
    length -= length % 2;
    for (; length; b += 2, length -= 2)
    swap(*b, *(b + 1));
    }
    static void ttusb_dec_process_urb_frame(struct ttusb_dec *dec, u8 *b,
    int length)
    {
    swap_bytes(b, length);
    while (length) {
    switch (dec.packet_state) {
    case 0:
    case 1:
    case 2:
    if (*b++ == 0xaa)
    dec.packet_state++;
    else
    dec.packet_state = 0;
    length--;
    break;
    case 3:
    if (*b == 0x00) {
    dec.packet_state++;
    dec.packet_length = 0;
    } else if (*b != 0xaa) {
    dec.packet_state = 0;
    }
    b++;
    length--;
    break;
    case 4:
    dec.packet[dec.packet_length++] = *b++;
    if (dec.packet_length == 2) {
    if (dec.packet[0] == 'A' &&
    dec.packet[1] == 'V') {
    dec.packet_type =
    TTUSB_DEC_PACKET_PVA;
    dec.packet_state++;
    } else if (dec.packet[0] == 'S') {
    dec.packet_type =
    TTUSB_DEC_PACKET_SECTION;
    dec.packet_state++;
    } else if (dec.packet[0] == 0x00) {
    dec.packet_type =
    TTUSB_DEC_PACKET_EMPTY;
    dec.packet_payload_length = 2;
    dec.packet_state = 7;
    } else {
    printk("%s: unknown packet type: %02x%02x\n",
    __func__,
    dec.packet[0], dec.packet[1]);
    dec.packet_state = 0;
    }
    }
    length--;
    break;
    case 5:
    dec.packet[dec.packet_length++] = *b++;
    if (dec.packet_type == TTUSB_DEC_PACKET_PVA &&
    dec.packet_length == 8) {
    dec.packet_state++;
    dec.packet_payload_length = 8 +
    (dec.packet[6] << 8) +
    dec.packet[7];
    } else if (dec.packet_type ==
    TTUSB_DEC_PACKET_SECTION &&
    dec.packet_length == 5) {
    dec.packet_state++;
    dec.packet_payload_length = 5 +
    ((dec.packet[3] & 0x0f) << 8) +
    dec.packet[4];
    }
    length--;
    break;
    case 6: {
    int remainder = dec.packet_payload_length -
    dec.packet_length;
    if (length >= remainder) {
    memcpy(dec.packet + dec.packet_length,
    b, remainder);
    dec.packet_length += remainder;
    b += remainder;
    length -= remainder;
    dec.packet_state++;
    } else {
    memcpy(&dec.packet[dec.packet_length],
    b, length);
    dec.packet_length += length;
    length = 0;
    }
    break;
    }
    case 7: {
    let mut tail: c_int = 4;
    dec.packet[dec.packet_length++] = *b++;
    if (dec.packet_type == TTUSB_DEC_PACKET_SECTION &&
    dec.packet_payload_length % 2)
    tail++;
    if (dec.packet_length ==
    dec.packet_payload_length + tail) {
    ttusb_dec_process_packet(dec);
    dec.packet_state = 0;
    }
    length--;
    break;
    }
    default:
    printk("%s: illegal packet state encountered.\n",
    __func__);
    dec.packet_state = 0;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_process_urb_frame_list(t: *mut work_struct) {
    static void ttusb_dec_process_urb_frame_list(struct work_struct *t)
    {
    struct ttusb_dec *dec = from_work(dec, t, urb_bh_work);
    struct list_head *item;
    struct urb_frame *frame;
    unsigned long flags;
    while (1) {
    spin_lock_irqsave(&dec.urb_frame_list_lock, flags);
    if ((item = dec.urb_frame_list.next) != &dec.urb_frame_list) {
    frame = list_entry(item, struct urb_frame,
    urb_frame_list);
    list_del(&frame.urb_frame_list);
    } else {
    spin_unlock_irqrestore(&dec.urb_frame_list_lock,
    flags);
    return;
    }
    spin_unlock_irqrestore(&dec.urb_frame_list_lock, flags);
    ttusb_dec_process_urb_frame(dec, frame.data, frame.length);
    kfree(frame);
    }
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_process_urb(urb: *mut urb) {
    static void ttusb_dec_process_urb(struct urb *urb)
    {
    struct ttusb_dec *dec = urb.context;
    if (!urb.status) {
    int i;
    for (i = 0; i < FRAMES_PER_ISO_BUF; i++) {
    struct usb_iso_packet_descriptor *d;
    u8 *b;
    int length;
    struct urb_frame *frame;
    d = &urb.iso_frame_desc[i];
    b = urb.transfer_buffer + d.offset;
    length = d.actual_length;
    if ((frame = kmalloc_obj(struct urb_frame, GFP_ATOMIC))) {
    unsigned long flags;
    memcpy(frame.data, b, length);
    frame.length = length;
    spin_lock_irqsave(&dec.urb_frame_list_lock,
    flags);
    list_add_tail(&frame.urb_frame_list,
    &dec.urb_frame_list);
    spin_unlock_irqrestore(&dec.urb_frame_list_lock,
    flags);
    queue_work(system_bh_wq, &dec.urb_bh_work);
    }
    }
    } else {
// -ENOENT is expected when unlinking urbs
    if (urb.status != -ENOENT)
    dprintk("%s: urb error: %d\n", __func__,
    urb.status);
    }
    if (dec.iso_stream_count)
    usb_submit_urb(urb, GFP_ATOMIC);
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_setup_urbs(dec: *mut ttusb_dec) {
    static void ttusb_dec_setup_urbs(struct ttusb_dec *dec)
    {
    int i, j, buffer_offset = 0;
    dprintk("%s\n", __func__);
    for (i = 0; i < ISO_BUF_COUNT; i++) {
    let mut frame_offset: c_int = 0;
    struct urb *urb = dec.iso_urb[i];
    urb.dev = dec.udev;
    urb.context = dec;
    urb.complete = ttusb_dec_process_urb;
    urb.pipe = dec.in_pipe;
    urb.transfer_flags = URB_ISO_ASAP;
    urb.interval = 1;
    urb.number_of_packets = FRAMES_PER_ISO_BUF;
    urb.transfer_buffer_length = ISO_FRAME_SIZE *
    FRAMES_PER_ISO_BUF;
    urb.transfer_buffer = dec.iso_buffer + buffer_offset;
    buffer_offset += ISO_FRAME_SIZE * FRAMES_PER_ISO_BUF;
    for (j = 0; j < FRAMES_PER_ISO_BUF; j++) {
    urb.iso_frame_desc[j].offset = frame_offset;
    urb.iso_frame_desc[j].length = ISO_FRAME_SIZE;
    frame_offset += ISO_FRAME_SIZE;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_stop_iso_xfer(dec: *mut ttusb_dec) {
    static void ttusb_dec_stop_iso_xfer(struct ttusb_dec *dec)
    {
    int i;
    dprintk("%s\n", __func__);
    if (mutex_lock_interruptible(&dec.iso_mutex))
    return;
    dec.iso_stream_count--;
    if (!dec.iso_stream_count) {
    for (i = 0; i < ISO_BUF_COUNT; i++)
    usb_kill_urb(dec.iso_urb[i]);
    }
    mutex_unlock(&dec.iso_mutex);
    }
// Setting the interface of the DEC tends to take down the USB communications
// for a short period, so it's important not to call this function just before
// trying to talk to it.
//
    static int ttusb_dec_set_interface(struct ttusb_dec *dec,
    enum ttusb_dec_interface interface)
    {
    let mut result: c_int = 0;
    u8 b[] = { 0x05 };
    if (interface != dec.interface) {
    switch (interface) {
    case TTUSB_DEC_INTERFACE_INITIAL:
    result = usb_set_interface(dec.udev, 0, 0);
    break;
    case TTUSB_DEC_INTERFACE_IN:
    result = ttusb_dec_send_command(dec, 0x80, sizeof(b),
    b, core::ptr::null_mut(), core::ptr::null_mut());
    if (result)
    return result;
    result = usb_set_interface(dec.udev, 0, 8);
    break;
    case TTUSB_DEC_INTERFACE_OUT:
    result = usb_set_interface(dec.udev, 0, 1);
    break;
    }
    if (result)
    return result;
    dec.interface = interface;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_start_iso_xfer(dec: *mut ttusb_dec) -> c_int {
    static int ttusb_dec_start_iso_xfer(struct ttusb_dec *dec)
    {
    int i, result;
    dprintk("%s\n", __func__);
    if (mutex_lock_interruptible(&dec.iso_mutex))
    return -EAGAIN;
    if (!dec.iso_stream_count) {
    ttusb_dec_setup_urbs(dec);
    dec.packet_state = 0;
    dec.v_pes_postbytes = 0;
    dec.next_packet_id = 0;
    for (i = 0; i < ISO_BUF_COUNT; i++) {
    if ((result = usb_submit_urb(dec.iso_urb[i],
    GFP_ATOMIC))) {
    printk("%s: failed urb submission %d: error %d\n",
    __func__, i, result);
    while (i) {
    usb_kill_urb(dec.iso_urb[i - 1]);
    i--;
    }
    mutex_unlock(&dec.iso_mutex);
    return result;
    }
    }
    }
    dec.iso_stream_count++;
    mutex_unlock(&dec.iso_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_start_ts_feed(dvbdmxfeed: *mut dvb_demux_feed) -> c_int {
    static int ttusb_dec_start_ts_feed(struct dvb_demux_feed *dvbdmxfeed)
    {
    struct dvb_demux *dvbdmx = dvbdmxfeed.demux;
    struct ttusb_dec *dec = dvbdmx.priv;
    u8 b0[] = { 0x05 };
    let mut result: c_int = 0;
    dprintk("%s\n", __func__);
    dprintk("  ts_type:");
    if (dvbdmxfeed.ts_type & TS_DECODER)
    dprintk(" TS_DECODER");
    if (dvbdmxfeed.ts_type & TS_PACKET)
    dprintk(" TS_PACKET");
    if (dvbdmxfeed.ts_type & TS_PAYLOAD_ONLY)
    dprintk(" TS_PAYLOAD_ONLY");
    dprintk("\n");
    switch (dvbdmxfeed.pes_type) {
    case DMX_PES_VIDEO:
    dprintk("  pes_type: DMX_PES_VIDEO\n");
    dec.pid[DMX_PES_PCR] = dvbdmxfeed.pid;
    dec.pid[DMX_PES_VIDEO] = dvbdmxfeed.pid;
    dec.video_filter = dvbdmxfeed.filter;
    ttusb_dec_set_pids(dec);
    break;
    case DMX_PES_AUDIO:
    dprintk("  pes_type: DMX_PES_AUDIO\n");
    dec.pid[DMX_PES_AUDIO] = dvbdmxfeed.pid;
    dec.audio_filter = dvbdmxfeed.filter;
    ttusb_dec_set_pids(dec);
    break;
    case DMX_PES_TELETEXT:
    dec.pid[DMX_PES_TELETEXT] = dvbdmxfeed.pid;
    dprintk("  pes_type: DMX_PES_TELETEXT(not supported)\n");
    return -ENOSYS;
    case DMX_PES_PCR:
    dprintk("  pes_type: DMX_PES_PCR\n");
    dec.pid[DMX_PES_PCR] = dvbdmxfeed.pid;
    ttusb_dec_set_pids(dec);
    break;
    case DMX_PES_OTHER:
    dprintk("  pes_type: DMX_PES_OTHER(not supported)\n");
    return -ENOSYS;
    default:
    dprintk("  pes_type: unknown (%d)\n", dvbdmxfeed.pes_type);
    return -EINVAL;
    }
    result = ttusb_dec_send_command(dec, 0x80, sizeof(b0), b0, core::ptr::null_mut(), core::ptr::null_mut());
    if (result)
    return result;
    dec.pva_stream_count++;
    return ttusb_dec_start_iso_xfer(dec);
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_start_sec_feed(dvbdmxfeed: *mut dvb_demux_feed) -> c_int {
    static int ttusb_dec_start_sec_feed(struct dvb_demux_feed *dvbdmxfeed)
    {
    struct ttusb_dec *dec = dvbdmxfeed.demux.priv;
    u8 b0[] = { 0x00, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0xff, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00 };
    __be16 pid;
    u8 c[COMMAND_PACKET_SIZE];
    int c_length;
    int result;
    struct filter_info *finfo;
    unsigned long flags;
    let mut x: u8 = 1;
    dprintk("%s\n", __func__);
    pid = htons(dvbdmxfeed.pid);
    memcpy(&b0[0], &pid, 2);
    memcpy(&b0[4], &x, 1);
    memcpy(&b0[5], &dvbdmxfeed.filter.filter.filter_value[0], 1);
    result = ttusb_dec_send_command(dec, 0x60, sizeof(b0), b0,
    &c_length, c);
    if (!result) {
    if (c_length == 2) {
    if (!(finfo = kmalloc_obj(struct filter_info, GFP_ATOMIC)))
    return -ENOMEM;
    finfo.stream_id = c[1];
    finfo.filter = dvbdmxfeed.filter;
    spin_lock_irqsave(&dec.filter_info_list_lock, flags);
    list_add_tail(&finfo.filter_info_list,
    &dec.filter_info_list);
    spin_unlock_irqrestore(&dec.filter_info_list_lock,
    flags);
    dvbdmxfeed.priv = finfo;
    dec.filter_stream_count++;
    return ttusb_dec_start_iso_xfer(dec);
    }
    return -EAGAIN;
    } else
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_start_feed(dvbdmxfeed: *mut dvb_demux_feed) -> c_int {
    static int ttusb_dec_start_feed(struct dvb_demux_feed *dvbdmxfeed)
    {
    struct dvb_demux *dvbdmx = dvbdmxfeed.demux;
    dprintk("%s\n", __func__);
    if (!dvbdmx.dmx.frontend)
    return -EINVAL;
    dprintk("  pid: 0x%04X\n", dvbdmxfeed.pid);
    switch (dvbdmxfeed.type) {
    case DMX_TYPE_TS:
    return ttusb_dec_start_ts_feed(dvbdmxfeed);
    case DMX_TYPE_SEC:
    return ttusb_dec_start_sec_feed(dvbdmxfeed);
    default:
    dprintk("  type: unknown (%d)\n", dvbdmxfeed.type);
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_stop_ts_feed(dvbdmxfeed: *mut dvb_demux_feed) -> c_int {
    static int ttusb_dec_stop_ts_feed(struct dvb_demux_feed *dvbdmxfeed)
    {
    struct ttusb_dec *dec = dvbdmxfeed.demux.priv;
    u8 b0[] = { 0x00 };
    ttusb_dec_send_command(dec, 0x81, sizeof(b0), b0, core::ptr::null_mut(), core::ptr::null_mut());
    dec.pva_stream_count--;
    ttusb_dec_stop_iso_xfer(dec);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_stop_sec_feed(dvbdmxfeed: *mut dvb_demux_feed) -> c_int {
    static int ttusb_dec_stop_sec_feed(struct dvb_demux_feed *dvbdmxfeed)
    {
    struct ttusb_dec *dec = dvbdmxfeed.demux.priv;
    u8 b0[] = { 0x00, 0x00 };
    struct filter_info *finfo = dvbdmxfeed.priv;
    unsigned long flags;
    b0[1] = finfo.stream_id;
    spin_lock_irqsave(&dec.filter_info_list_lock, flags);
    list_del(&finfo.filter_info_list);
    spin_unlock_irqrestore(&dec.filter_info_list_lock, flags);
    kfree(finfo);
    ttusb_dec_send_command(dec, 0x62, sizeof(b0), b0, core::ptr::null_mut(), core::ptr::null_mut());
    dec.filter_stream_count--;
    ttusb_dec_stop_iso_xfer(dec);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_stop_feed(dvbdmxfeed: *mut dvb_demux_feed) -> c_int {
    static int ttusb_dec_stop_feed(struct dvb_demux_feed *dvbdmxfeed)
    {
    dprintk("%s\n", __func__);
    switch (dvbdmxfeed.type) {
    case DMX_TYPE_TS:
    return ttusb_dec_stop_ts_feed(dvbdmxfeed);
    case DMX_TYPE_SEC:
    return ttusb_dec_stop_sec_feed(dvbdmxfeed);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_free_iso_urbs(dec: *mut ttusb_dec) {
    static void ttusb_dec_free_iso_urbs(struct ttusb_dec *dec)
    {
    int i;
    dprintk("%s\n", __func__);
    for (i = 0; i < ISO_BUF_COUNT; i++)
    usb_free_urb(dec.iso_urb[i]);
    kfree(dec.iso_buffer);
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_alloc_iso_urbs(dec: *mut ttusb_dec) -> c_int {
    static int ttusb_dec_alloc_iso_urbs(struct ttusb_dec *dec)
    {
    int i;
    dprintk("%s\n", __func__);
    dec.iso_buffer = kcalloc(FRAMES_PER_ISO_BUF * ISO_BUF_COUNT,
    ISO_FRAME_SIZE, GFP_KERNEL);
    if (!dec.iso_buffer)
    return -ENOMEM;
    for (i = 0; i < ISO_BUF_COUNT; i++) {
    struct urb *urb;
    if (!(urb = usb_alloc_urb(FRAMES_PER_ISO_BUF, GFP_ATOMIC))) {
    ttusb_dec_free_iso_urbs(dec);
    return -ENOMEM;
    }
    dec.iso_urb[i] = urb;
    }
    ttusb_dec_setup_urbs(dec);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_init_bh_work(dec: *mut ttusb_dec) {
    static void ttusb_dec_init_bh_work(struct ttusb_dec *dec)
    {
    spin_lock_init(&dec.urb_frame_list_lock);
    INIT_LIST_HEAD(&dec.urb_frame_list);
    INIT_WORK(&dec.urb_bh_work, ttusb_dec_process_urb_frame_list);
    }
#[no_mangle]
unsafe extern "C" fn ttusb_init_rc(dec: *mut ttusb_dec) -> c_int {
    static int ttusb_init_rc( struct ttusb_dec *dec)
    {
    struct input_dev *input_dev;
    u8 b[] = { 0x00, 0x01 };
    int i;
    int err;
    usb_make_path(dec.udev, dec.rc_phys, sizeof(dec.rc_phys));
    strlcat(dec.rc_phys, "/input0", sizeof(dec.rc_phys));
    input_dev = input_allocate_device();
    if (!input_dev)
    return -ENOMEM;
    input_dev.name = "ttusb_dec remote control";
    input_dev.phys = dec.rc_phys;
    input_dev.evbit[0] = BIT_MASK(EV_KEY);
    input_dev.keycodesize = sizeof(u16);
    input_dev.keycodemax = 0x1a;
    input_dev.keycode = rc_keys;
    for (i = 0; i < ARRAY_SIZE(rc_keys); i++)
    set_bit(rc_keys[i], input_dev.keybit);
    err = input_register_device(input_dev);
    if (err) {
    input_free_device(input_dev);
    return err;
    }
    dec.rc_input_dev = input_dev;
    if (usb_submit_urb(dec.irq_urb, GFP_KERNEL))
    printk("%s: usb_submit_urb failed\n",__func__);
// enable irq pipe
    ttusb_dec_send_command(dec,0xb0,sizeof(b),b,core::ptr::null_mut(),core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_init_v_pes(dec: *mut ttusb_dec) {
    static void ttusb_dec_init_v_pes(struct ttusb_dec *dec)
    {
    dprintk("%s\n", __func__);
    dec.v_pes[0] = 0x00;
    dec.v_pes[1] = 0x00;
    dec.v_pes[2] = 0x01;
    dec.v_pes[3] = 0xe0;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_init_usb(dec: *mut ttusb_dec) -> c_int {
    static int ttusb_dec_init_usb(struct ttusb_dec *dec)
    {
    int result;
    dprintk("%s\n", __func__);
    mutex_init(&dec.usb_mutex);
    mutex_init(&dec.iso_mutex);
    dec.command_pipe = usb_sndbulkpipe(dec.udev, COMMAND_PIPE);
    dec.result_pipe = usb_rcvbulkpipe(dec.udev, RESULT_PIPE);
    dec.in_pipe = usb_rcvisocpipe(dec.udev, IN_PIPE);
    dec.out_pipe = usb_sndisocpipe(dec.udev, OUT_PIPE);
    dec.irq_pipe = usb_rcvintpipe(dec.udev, IRQ_PIPE);
    if(enable_rc) {
    dec.irq_urb = usb_alloc_urb(0, GFP_KERNEL);
    if(!dec.irq_urb) {
    return -ENOMEM;
    }
    dec.irq_buffer = usb_alloc_coherent(dec.udev,IRQ_PACKET_SIZE,
    GFP_KERNEL, &dec.irq_dma_handle);
    if(!dec.irq_buffer) {
    usb_free_urb(dec.irq_urb);
    return -ENOMEM;
    }
    usb_fill_int_urb(dec.irq_urb, dec.udev,dec.irq_pipe,
    dec.irq_buffer, IRQ_PACKET_SIZE,
    ttusb_dec_handle_irq, dec, 1);
    dec.irq_urb.transfer_dma = dec.irq_dma_handle;
    dec.irq_urb.transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    }
    result = ttusb_dec_alloc_iso_urbs(dec);
    if (result) {
    usb_free_urb(dec.irq_urb);
    usb_free_coherent(dec.udev, IRQ_PACKET_SIZE,
    dec.irq_buffer, dec.irq_dma_handle);
    }
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_boot_dsp(dec: *mut ttusb_dec) -> c_int {
    static int ttusb_dec_boot_dsp(struct ttusb_dec *dec)
    {
    int i, j, actual_len, result, size, trans_count;
    u8 b0[] = { 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x61, 0x00 };
    u8 b1[] = { 0x61 };
    u8 *b;
    char idstring[21];
    const u8 *firmware = core::ptr::null_mut();
    let mut firmware_size: usize = 0;
    let mut firmware_csum: u16 = 0;
    __be16 firmware_csum_ns;
    __be32 firmware_size_nl;
    u32 crc32_csum, crc32_check;
    __be32 tmp;
    const struct firmware *fw_entry = core::ptr::null_mut();
    dprintk("%s\n", __func__);
    result = request_firmware(&fw_entry, dec.firmware_name, &dec.udev.dev);
    if (result) {
    printk(KERN_ERR "%s: Firmware (%s) unavailable.\n",
    __func__, dec.firmware_name);
    return result;
    }
    firmware = fw_entry.data;
    firmware_size = fw_entry.size;
    if (firmware_size < 60) {
    printk("%s: firmware size too small for DSP code (%zu < 60).\n",
    __func__, firmware_size);
    release_firmware(fw_entry);
    return -ENOENT;
    }
// a 32 bit checksum over the first 56 bytes of the DSP Code is stored
    at offset 56 of file, so use it to check if the firmware file is
    valid. */
    crc32_csum = crc32(~0L, firmware, 56) ^ ~0L;
    memcpy(&tmp, &firmware[56], 4);
    crc32_check = ntohl(tmp);
    if (crc32_csum != crc32_check) {
    printk("%s: crc32 check of DSP code failed (calculated 0x%08x != 0x%08x in file), file invalid.\n",
    __func__, crc32_csum, crc32_check);
    release_firmware(fw_entry);
    return -ENOENT;
    }
    memcpy(idstring, &firmware[36], 20);
    idstring[20] = '\0';
    printk(KERN_INFO "ttusb_dec: found DSP code \"%s\".\n", idstring);
    firmware_size_nl = htonl(firmware_size);
    memcpy(b0, &firmware_size_nl, 4);
    firmware_csum = crc16(~0, firmware, firmware_size) ^ ~0;
    firmware_csum_ns = htons(firmware_csum);
    memcpy(&b0[6], &firmware_csum_ns, 2);
    result = ttusb_dec_send_command(dec, 0x41, sizeof(b0), b0, core::ptr::null_mut(), core::ptr::null_mut());
    if (result) {
    release_firmware(fw_entry);
    return result;
    }
    trans_count = 0;
    j = 0;
    b = kmalloc(ARM_PACKET_SIZE, GFP_KERNEL);
    if (b == core::ptr::null_mut()) {
    release_firmware(fw_entry);
    return -ENOMEM;
    }
    for (i = 0; i < firmware_size; i += COMMAND_PACKET_SIZE) {
    size = firmware_size - i;
    if (size > COMMAND_PACKET_SIZE)
    size = COMMAND_PACKET_SIZE;
    b[j + 0] = 0xaa;
    b[j + 1] = trans_count++;
    b[j + 2] = 0xf0;
    b[j + 3] = size;
    memcpy(&b[j + 4], &firmware[i], size);
    j += COMMAND_PACKET_SIZE + 4;
    if (j >= ARM_PACKET_SIZE) {
    result = usb_bulk_msg(dec.udev, dec.command_pipe, b,
    ARM_PACKET_SIZE, &actual_len,
    100);
    j = 0;
    } else if (size < COMMAND_PACKET_SIZE) {
    result = usb_bulk_msg(dec.udev, dec.command_pipe, b,
    j - COMMAND_PACKET_SIZE + size,
    &actual_len, 100);
    }
    }
    result = ttusb_dec_send_command(dec, 0x43, sizeof(b1), b1, core::ptr::null_mut(), core::ptr::null_mut());
    release_firmware(fw_entry);
    kfree(b);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_init_stb(dec: *mut ttusb_dec) -> c_int {
    static int ttusb_dec_init_stb(struct ttusb_dec *dec)
    {
    int result;
    let mut mode: c_uint = 0, model = 0, version = 0;
    dprintk("%s\n", __func__);
    result = ttusb_dec_get_stb_state(dec, &mode, &model, &version);
    if (result)
    return result;
    if (!mode) {
    if (version == 0xABCDEFAB)
    printk(KERN_INFO "ttusb_dec: no version info in Firmware\n");
    else
    printk(KERN_INFO "ttusb_dec: Firmware %x.%02x%c%c\n",
    version >> 24, (version >> 16) & 0xff,
    (version >> 8) & 0xff, version & 0xff);
    result = ttusb_dec_boot_dsp(dec);
    if (result)
    return result;
    } else {
// We can't trust the USB IDs that some firmwares
    give the box */
    switch (model) {
    case 0x00070001:
    case 0x00070008:
    case 0x0007000c:
    ttusb_dec_set_model(dec, TTUSB_DEC3000S);
    break;
    case 0x00070009:
    case 0x00070013:
    ttusb_dec_set_model(dec, TTUSB_DEC2000T);
    break;
    case 0x00070011:
    ttusb_dec_set_model(dec, TTUSB_DEC2540T);
    break;
    default:
    printk(KERN_ERR "%s: unknown model returned by firmware (%08x) - please report\n",
    __func__, model);
    return -ENOENT;
    }
    if (version >= 0x01770000)
    dec.can_playback = 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_init_dvb(dec: *mut ttusb_dec) -> c_int {
    static int ttusb_dec_init_dvb(struct ttusb_dec *dec)
    {
    int result;
    dprintk("%s\n", __func__);
    if ((result = dvb_register_adapter(&dec.adapter,
    dec.model_name, THIS_MODULE,
    &dec.udev.dev,
    adapter_nr)) < 0) {
    printk("%s: dvb_register_adapter failed: error %d\n",
    __func__, result);
    return result;
    }
    dec.demux.dmx.capabilities = DMX_TS_FILTERING | DMX_SECTION_FILTERING;
    dec.demux.priv = (void *)dec;
    dec.demux.filternum = 31;
    dec.demux.feednum = 31;
    dec.demux.start_feed = ttusb_dec_start_feed;
    dec.demux.stop_feed = ttusb_dec_stop_feed;
    dec.demux.write_to_decoder = core::ptr::null_mut();
    if ((result = dvb_dmx_init(&dec.demux)) < 0) {
    printk("%s: dvb_dmx_init failed: error %d\n", __func__,
    result);
    dvb_unregister_adapter(&dec.adapter);
    return result;
    }
    dec.dmxdev.filternum = 32;
    dec.dmxdev.demux = &dec.demux.dmx;
    dec.dmxdev.capabilities = 0;
    if ((result = dvb_dmxdev_init(&dec.dmxdev, &dec.adapter)) < 0) {
    printk("%s: dvb_dmxdev_init failed: error %d\n",
    __func__, result);
    dvb_dmx_release(&dec.demux);
    dvb_unregister_adapter(&dec.adapter);
    return result;
    }
    dec.frontend.source = DMX_FRONTEND_0;
    if ((result = dec.demux.dmx.add_frontend(&dec.demux.dmx,
    &dec.frontend)) < 0) {
    printk("%s: dvb_dmx_init failed: error %d\n", __func__,
    result);
    dvb_dmxdev_release(&dec.dmxdev);
    dvb_dmx_release(&dec.demux);
    dvb_unregister_adapter(&dec.adapter);
    return result;
    }
    if ((result = dec.demux.dmx.connect_frontend(&dec.demux.dmx,
    &dec.frontend)) < 0) {
    printk("%s: dvb_dmx_init failed: error %d\n", __func__,
    result);
    dec.demux.dmx.remove_frontend(&dec.demux.dmx, &dec.frontend);
    dvb_dmxdev_release(&dec.dmxdev);
    dvb_dmx_release(&dec.demux);
    dvb_unregister_adapter(&dec.adapter);
    return result;
    }
    dvb_net_init(&dec.adapter, &dec.dvb_net, &dec.demux.dmx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_exit_dvb(dec: *mut ttusb_dec) {
    static void ttusb_dec_exit_dvb(struct ttusb_dec *dec)
    {
    dprintk("%s\n", __func__);
    dvb_net_release(&dec.dvb_net);
    dec.demux.dmx.close(&dec.demux.dmx);
    dec.demux.dmx.remove_frontend(&dec.demux.dmx, &dec.frontend);
    dvb_dmxdev_release(&dec.dmxdev);
    dvb_dmx_release(&dec.demux);
    if (dec.fe) {
    dvb_unregister_frontend(dec.fe);
    dvb_frontend_detach(dec.fe);
    }
    dvb_unregister_adapter(&dec.adapter);
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_exit_rc(dec: *mut ttusb_dec) {
    static void ttusb_dec_exit_rc(struct ttusb_dec *dec)
    {
    dprintk("%s\n", __func__);
    if (dec.rc_input_dev) {
    input_unregister_device(dec.rc_input_dev);
    dec.rc_input_dev = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_exit_usb(dec: *mut ttusb_dec) {
    static void ttusb_dec_exit_usb(struct ttusb_dec *dec)
    {
    int i;
    dprintk("%s\n", __func__);
    if (enable_rc) {
// we have to check whether the irq URB is already submitted.
// As the irq is submitted after the interface is changed,
// this is the best method i figured out.
// Any others?
    if (dec.interface == TTUSB_DEC_INTERFACE_IN)
    usb_kill_urb(dec.irq_urb);
    usb_free_urb(dec.irq_urb);
    usb_free_coherent(dec.udev, IRQ_PACKET_SIZE,
    dec.irq_buffer, dec.irq_dma_handle);
    }
    dec.iso_stream_count = 0;
    for (i = 0; i < ISO_BUF_COUNT; i++)
    usb_kill_urb(dec.iso_urb[i]);
    ttusb_dec_free_iso_urbs(dec);
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_exit_bh_work(dec: *mut ttusb_dec) {
    static void ttusb_dec_exit_bh_work(struct ttusb_dec *dec)
    {
    struct list_head *item;
    struct urb_frame *frame;
    cancel_work_sync(&dec.urb_bh_work);
    while ((item = dec.urb_frame_list.next) != &dec.urb_frame_list) {
    frame = list_entry(item, struct urb_frame, urb_frame_list);
    list_del(&frame.urb_frame_list);
    kfree(frame);
    }
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_init_filters(dec: *mut ttusb_dec) {
    static void ttusb_dec_init_filters(struct ttusb_dec *dec)
    {
    INIT_LIST_HEAD(&dec.filter_info_list);
    spin_lock_init(&dec.filter_info_list_lock);
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_exit_filters(dec: *mut ttusb_dec) {
    static void ttusb_dec_exit_filters(struct ttusb_dec *dec)
    {
    struct list_head *item;
    struct filter_info *finfo;
    while ((item = dec.filter_info_list.next) != &dec.filter_info_list) {
    finfo = list_entry(item, struct filter_info, filter_info_list);
    list_del(&finfo.filter_info_list);
    kfree(finfo);
    }
    }
    static int fe_send_command(struct dvb_frontend* fe, const u8 command,
    int param_length, const u8 params[],
    int *result_length, u8 cmd_result[])
    {
    let mut dec: *mut ttusb_dec = fe.dvb.priv;
    return ttusb_dec_send_command(dec, command, param_length, params, result_length, cmd_result);
    }
    static const struct ttusbdecfe_config fe_config = {
    .send_command = fe_send_command
    };
    static int ttusb_dec_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct usb_device *udev;
    struct ttusb_dec *dec;
    int result;
    dprintk("%s\n", __func__);
    udev = interface_to_usbdev(intf);
    if (!(dec = kzalloc_obj(struct ttusb_dec))) {
    printk("%s: couldn't allocate memory.\n", __func__);
    return -ENOMEM;
    }
    usb_set_intfdata(intf, (void *)dec);
    switch (id.idProduct) {
    case 0x1006:
    ttusb_dec_set_model(dec, TTUSB_DEC3000S);
    break;
    case 0x1008:
    ttusb_dec_set_model(dec, TTUSB_DEC2000T);
    break;
    case 0x1009:
    ttusb_dec_set_model(dec, TTUSB_DEC2540T);
    break;
    }
    dec.udev = udev;
    result = ttusb_dec_init_usb(dec);
    if (result)
    goto err_usb;
    result = ttusb_dec_init_stb(dec);
    if (result)
    goto err_stb;
    result = ttusb_dec_init_dvb(dec);
    if (result)
    goto err_stb;
    dec.adapter.priv = dec;
    switch (id.idProduct) {
    case 0x1006:
    dec.fe = ttusbdecfe_dvbs_attach(&fe_config);
    break;
    case 0x1008:
    case 0x1009:
    dec.fe = ttusbdecfe_dvbt_attach(&fe_config);
    break;
    }
    if (dec.fe == core::ptr::null_mut()) {
    printk("dvb-ttusb-dec: A frontend driver was not found for device [%04x:%04x]\n",
    le16_to_cpu(dec.udev.descriptor.idVendor),
    le16_to_cpu(dec.udev.descriptor.idProduct));
    } else {
    if (dvb_register_frontend(&dec.adapter, dec.fe)) {
    printk("budget-ci: Frontend registration failed!\n");
    if (dec.fe.ops.release)
    dec.fe.ops.release(dec.fe);
    dec.fe = core::ptr::null_mut();
    }
    }
    ttusb_dec_init_v_pes(dec);
    ttusb_dec_init_filters(dec);
    ttusb_dec_init_bh_work(dec);
    dec.active = 1;
    ttusb_dec_set_interface(dec, TTUSB_DEC_INTERFACE_IN);
    if (enable_rc)
    ttusb_init_rc(dec);
    return 0;
    err_stb:
    ttusb_dec_exit_usb(dec);
    err_usb:
    kfree(dec);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ttusb_dec_disconnect(intf: *mut usb_interface) {
    static void ttusb_dec_disconnect(struct usb_interface *intf)
    {
    struct ttusb_dec *dec = usb_get_intfdata(intf);
    usb_set_intfdata(intf, core::ptr::null_mut());
    dprintk("%s\n", __func__);
    if (dec.active) {
    ttusb_dec_exit_bh_work(dec);
    ttusb_dec_exit_filters(dec);
    if(enable_rc)
    ttusb_dec_exit_rc(dec);
    ttusb_dec_exit_usb(dec);
    ttusb_dec_exit_dvb(dec);
    }
    kfree(dec);
    }
    static void ttusb_dec_set_model(struct ttusb_dec *dec,
    enum ttusb_dec_model model)
    {
    dec.model = model;
    switch (model) {
    case TTUSB_DEC2000T:
    dec.model_name = "DEC2000-t";
    dec.firmware_name = "dvb-ttusb-dec-2000t.fw";
    break;
    case TTUSB_DEC2540T:
    dec.model_name = "DEC2540-t";
    dec.firmware_name = "dvb-ttusb-dec-2540t.fw";
    break;
    case TTUSB_DEC3000S:
    dec.model_name = "DEC3000-s";
    dec.firmware_name = "dvb-ttusb-dec-3000s.fw";
    break;
    }
    }
    static const struct usb_device_id ttusb_dec_table[] = {
    {USB_DEVICE(0x0b48, 0x1006)},	/* DEC3000-s */
// {USB_DEVICE(0x0b48, 0x1007)},	   Unconfirmed
    {USB_DEVICE(0x0b48, 0x1008)},	/* DEC2000-t */
    {USB_DEVICE(0x0b48, 0x1009)},	/* DEC2540-t */
    {}
    };
    static struct usb_driver ttusb_dec_driver = {
    .name		= "ttusb-dec",
    .probe		= ttusb_dec_probe,
    .disconnect	= ttusb_dec_disconnect,
    .id_table	= ttusb_dec_table,
    };
    module_usb_driver(ttusb_dec_driver);
    MODULE_AUTHOR("Alex Woods <linux-dvb@giblets.org>");
    MODULE_DESCRIPTION(DRIVER_NAME);
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(usb, ttusb_dec_table);
