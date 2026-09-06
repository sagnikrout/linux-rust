//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/saa6588.c
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
    Driver for SAA6588 RDS decoder
    (c) 2005 Hans J. Koch
//

// insmod options
    static unsigned int debug;
    static unsigned int xtal;
    static unsigned int mmbs;
    static unsigned int plvl;
    let mut bufblocks: static unsigned int = 100;
    module_param(debug, int, 0644);
    MODULE_PARM_DESC(debug, "enable debug messages");
    module_param(xtal, int, 0);
    MODULE_PARM_DESC(xtal, "select oscillator frequency (0..3), default 0");
    module_param(mmbs, int, 0);
    MODULE_PARM_DESC(mmbs, "enable MMBS mode: 0=off (default), 1=on");
    module_param(plvl, int, 0);
    MODULE_PARM_DESC(plvl, "select pause level (0..3), default 0");
    module_param(bufblocks, int, 0);
    MODULE_PARM_DESC(bufblocks, "number of buffered blocks, default 100");
    MODULE_DESCRIPTION("v4l2 driver module for SAA6588 RDS decoder");
    MODULE_AUTHOR("Hans J. Koch <koch@hjk-az.de>");
    MODULE_LICENSE("GPL");
// ----------------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa6588 {
    pub sd: v4l2_subdev,
    pub work: delayed_work,
    pub lock: spinlock_t,
    pub buffer: *mut c_uchar,
    pub buf_size: c_uint,
    pub rd_index: c_uint,
    pub wr_index: c_uint,
    pub block_count: c_uint,
    pub last_blocknum: c_uchar,
    pub read_queue: wait_queue_head_t,
    pub data_available_for_read: c_int,
    pub sync: u8,
}

    static inline struct saa6588 *to_saa6588(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct saa6588, sd);
    }
// ----------------------------------------------------------------------
//
// SAA6588 defines
//
// Initialization and mode control byte (0w)
// bit 0+1 (DAC0/DAC1)
pub const cModeStandard: c_uint = 0x00;
pub const cModeFastPI: c_uint = 0x01;
pub const cModeReducedRequest: c_uint = 0x02;
pub const cModeInvalid: c_uint = 0x03;
// bit 2 (RBDS)
pub const cProcessingModeRDS: c_uint = 0x00;
pub const cProcessingModeRBDS: c_uint = 0x04;
// bit 3+4 (SYM0/SYM1)
pub const cErrCorrectionNone: c_uint = 0x00;
pub const cErrCorrection2Bits: c_uint = 0x08;
pub const cErrCorrection5Bits: c_uint = 0x10;
pub const cErrCorrectionNoneRBDS: c_uint = 0x18;
// bit 5 (NWSY)
pub const cSyncNormal: c_uint = 0x00;
pub const cSyncRestart: c_uint = 0x20;
// bit 6 (TSQD)
pub const cSigQualityDetectOFF: c_uint = 0x00;
pub const cSigQualityDetectON: c_uint = 0x40;
// bit 7 (SQCM)
pub const cSigQualityTriggered: c_uint = 0x00;
pub const cSigQualityContinous: c_uint = 0x80;
// Pause level and flywheel control byte (1w)
// bits 0..5 (FEB0..FEB5)
pub const cFlywheelMaxBlocksMask: c_uint = 0x3F;
pub const cFlywheelDefault: c_uint = 0x20;
// bits 6+7 (PL0/PL1)
pub const cPauseLevel_11mV: c_uint = 0x00;
pub const cPauseLevel_17mV: c_uint = 0x40;
pub const cPauseLevel_27mV: c_uint = 0x80;
pub const cPauseLevel_43mV: c_uint = 0xC0;
// Pause time/oscillator frequency/quality detector control byte (1w)
// bits 0..4 (SQS0..SQS4)
pub const cQualityDetectSensMask: c_uint = 0x1F;
pub const cQualityDetectDefault: c_uint = 0x0F;
// bit 5 (SOSC)
pub const cSelectOscFreqOFF: c_uint = 0x00;
pub const cSelectOscFreqON: c_uint = 0x20;
// bit 6+7 (PTF0/PTF1)
pub const cOscFreq_4332kHz: c_uint = 0x00;
pub const cOscFreq_8664kHz: c_uint = 0x40;
pub const cOscFreq_12996kHz: c_uint = 0x80;
pub const cOscFreq_17328kHz: c_uint = 0xC0;
// ----------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn block_from_buf(s: *mut saa6588, buf: *mut c_uchar) -> bool {
    static bool block_from_buf(struct saa6588 *s, unsigned char *buf)
    {
    int i;
    if (s.rd_index == s.wr_index) {
    if (debug > 2)
    v4l2_info(&s.sd, "Read: buffer empty.\n");
    return false;
    }
    if (debug > 2) {
    v4l2_info(&s.sd, "Read: ");
    for (i = s.rd_index; i < s.rd_index + 3; i++)
    v4l2_info(&s.sd, "0x%02x ", s.buffer[i]);
    }
    memcpy(buf, &s.buffer[s.rd_index], 3);
    s.rd_index += 3;
    if (s.rd_index >= s.buf_size)
    s.rd_index = 0;
    s.block_count--;
    if (debug > 2)
    v4l2_info(&s.sd, "%d blocks total.\n", s.block_count);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn read_from_buf(s: *mut saa6588, a: *mut saa6588_command) {
    static void read_from_buf(struct saa6588 *s, struct saa6588_command *a)
    {
    unsigned char __user *buf_ptr = a.buffer;
    unsigned char buf[3];
    unsigned long flags;
    unsigned int rd_blocks;
    unsigned int i;
    a.result = 0;
    if (!a.buffer)
    return;
    while (!a.nonblocking && !s.data_available_for_read) {
    int ret = wait_event_interruptible(s.read_queue,
    s.data_available_for_read);
    if (ret == -ERESTARTSYS) {
    a.result = -EINTR;
    return;
    }
    }
    rd_blocks = a.block_count;
    spin_lock_irqsave(&s.lock, flags);
    if (rd_blocks > s.block_count)
    rd_blocks = s.block_count;
    spin_unlock_irqrestore(&s.lock, flags);
    if (!rd_blocks)
    return;
    for (i = 0; i < rd_blocks; i++) {
    bool got_block;
    spin_lock_irqsave(&s.lock, flags);
    got_block = block_from_buf(s, buf);
    spin_unlock_irqrestore(&s.lock, flags);
    if (!got_block)
    break;
    if (copy_to_user(buf_ptr, buf, 3)) {
    a.result = -EFAULT;
    return;
    }
    buf_ptr += 3;
    a.result += 3;
    }
    spin_lock_irqsave(&s.lock, flags);
    s.data_available_for_read = (s.block_count > 0);
    spin_unlock_irqrestore(&s.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn block_to_buf(s: *mut saa6588, blockbuf: *mut c_uchar) {
    static void block_to_buf(struct saa6588 *s, unsigned char *blockbuf)
    {
    unsigned int i;
    if (debug > 3)
    v4l2_info(&s.sd, "New block: ");
    for (i = 0; i < 3; ++i) {
    if (debug > 3)
    v4l2_info(&s.sd, "0x%02x ", blockbuf[i]);
    s.buffer[s.wr_index] = blockbuf[i];
    s.wr_index++;
    }
    if (s.wr_index >= s.buf_size)
    s.wr_index = 0;
    if (s.wr_index == s.rd_index) {
    s.rd_index += 3;
    if (s.rd_index >= s.buf_size)
    s.rd_index = 0;
    } else
    s.block_count++;
    if (debug > 3)
    v4l2_info(&s.sd, "%d blocks total.\n", s.block_count);
    }
#[no_mangle]
unsafe extern "C" fn saa6588_i2c_poll(s: *mut saa6588) {
    static void saa6588_i2c_poll(struct saa6588 *s)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&s.sd);
    unsigned long flags;
    unsigned char tmpbuf[6];
    unsigned char blocknum;
    unsigned char tmp;
// Although we only need 3 bytes, we have to read at least 6.
    SAA6588 returns garbage otherwise. */
    if (6 != i2c_master_recv(client, &tmpbuf[0], 6)) {
    if (debug > 1)
    v4l2_info(&s.sd, "read error!\n");
    return;
    }
    s.sync = tmpbuf[0] & 0x10;
    if (!s.sync)
    return;
    blocknum = tmpbuf[0] >> 5;
    if (blocknum == s.last_blocknum) {
    if (debug > 3)
    v4l2_info(&s.sd, "Saw block %d again.\n", blocknum);
    return;
    }
    s.last_blocknum = blocknum;
//
    Byte order according to v4l2 specification:
    Byte 0: Least Significant Byte of RDS Block
    Byte 1: Most Significant Byte of RDS Block
    Byte 2 Bit 7: Error bit. Indicates that an uncorrectable error
    occurred during reception of this block.
    Bit 6: Corrected bit. Indicates that an error was
    corrected for this data block.
    Bits 5-3: Same as bits 0-2.
    Bits 2-0: Block number.
    SAA6588 byte order is Status-MSB-LSB, so we have to swap the
    first and the last of the 3 bytes block.
//
    swap(tmpbuf[2], tmpbuf[0]);
// Map 'Invalid block E' to 'Invalid Block'
    if (blocknum == 6)
    blocknum = V4L2_RDS_BLOCK_INVALID;
// And if are not in mmbs mode, then 'Block E' is also mapped
    to 'Invalid Block'. As far as I can tell MMBS is discontinued,
    and if there is ever a need to support E blocks, then please
    contact the linux-media mailinglist. */
#[no_mangle]
pub unsafe extern "C" fn if(5: !mmbs && blocknum ==) -> else {
    else if (!mmbs && blocknum == 5)
    blocknum = V4L2_RDS_BLOCK_INVALID;
    tmp = blocknum;
    tmp |= blocknum << 3;	/* Received offset == Offset Name (OK ?) */
    if ((tmpbuf[2] & 0x03) == 0x03)
    tmp |= V4L2_RDS_BLOCK_ERROR;	 /* uncorrectable error */
#[no_mangle]
pub unsafe extern "C" fn if(0x00: (tmpbuf[2] & 0x03) !=) -> else {
    else if ((tmpbuf[2] & 0x03) != 0x00)
    tmp |= V4L2_RDS_BLOCK_CORRECTED; /* corrected error */
    tmpbuf[2] = tmp;	/* Is this enough ? Should we also check other bits ? */
    spin_lock_irqsave(&s.lock, flags);
    block_to_buf(s, tmpbuf);
    spin_unlock_irqrestore(&s.lock, flags);
    s.data_available_for_read = 1;
    wake_up_interruptible(&s.read_queue);
    }
#[no_mangle]
unsafe extern "C" fn saa6588_work(work: *mut work_struct) {
    static void saa6588_work(struct work_struct *work)
    {
    struct saa6588 *s = container_of(work, struct saa6588, work.work);
    saa6588_i2c_poll(s);
    schedule_delayed_work(&s.work, msecs_to_jiffies(20));
    }
#[no_mangle]
unsafe extern "C" fn saa6588_configure(s: *mut saa6588) {
    static void saa6588_configure(struct saa6588 *s)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&s.sd);
    unsigned char buf[3];
    int rc;
    buf[0] = cSyncRestart;
    if (mmbs)
    buf[0] |= cProcessingModeRBDS;
    buf[1] = cFlywheelDefault;
    switch (plvl) {
    case 0:
    buf[1] |= cPauseLevel_11mV;
    break;
    case 1:
    buf[1] |= cPauseLevel_17mV;
    break;
    case 2:
    buf[1] |= cPauseLevel_27mV;
    break;
    case 3:
    buf[1] |= cPauseLevel_43mV;
    break;
    default:		/* nothing */
    break;
    }
    buf[2] = cQualityDetectDefault | cSelectOscFreqON;
    switch (xtal) {
    case 0:
    buf[2] |= cOscFreq_4332kHz;
    break;
    case 1:
    buf[2] |= cOscFreq_8664kHz;
    break;
    case 2:
    buf[2] |= cOscFreq_12996kHz;
    break;
    case 3:
    buf[2] |= cOscFreq_17328kHz;
    break;
    default:		/* nothing */
    break;
    }
    if (debug)
    v4l2_info(&s.sd, "writing: 0w=0x%02x 1w=0x%02x 2w=0x%02x\n",
    buf[0], buf[1], buf[2]);
    rc = i2c_master_send(client, buf, 3);
    if (rc != 3)
    v4l2_info(&s.sd, "i2c i/o error: rc == %d (should be 3)\n", rc);
    }
// ----------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn saa6588_command(sd: *mut v4l2_subdev, cmd: c_uint, arg: *mut c_void) -> c_long {
    static long saa6588_command(struct v4l2_subdev *sd, unsigned int cmd, void *arg)
    {
    struct saa6588 *s = to_saa6588(sd);
    struct saa6588_command *a = arg;
    switch (cmd) {
// --- close() for /dev/radio ---
    case SAA6588_CMD_CLOSE:
    s.data_available_for_read = 1;
    wake_up_interruptible(&s.read_queue);
    s.data_available_for_read = 0;
    a.result = 0;
    break;
// --- read() for /dev/radio ---
    case SAA6588_CMD_READ:
    read_from_buf(s, a);
    break;
// --- poll() for /dev/radio ---
    case SAA6588_CMD_POLL:
    a.poll_mask = 0;
    if (s.data_available_for_read)
    a.poll_mask |= EPOLLIN | EPOLLRDNORM;
    poll_wait(a.instance, &s.read_queue, a.event_list);
    break;
    default:
// nothing
    return -ENOIOCTLCMD;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn saa6588_g_tuner(sd: *mut v4l2_subdev, vt: *mut v4l2_tuner) -> c_int {
    static int saa6588_g_tuner(struct v4l2_subdev *sd, struct v4l2_tuner *vt)
    {
    struct saa6588 *s = to_saa6588(sd);
    vt.capability |= V4L2_TUNER_CAP_RDS | V4L2_TUNER_CAP_RDS_BLOCK_IO;
    if (s.sync)
    vt.rxsubchans |= V4L2_TUNER_SUB_RDS;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn saa6588_s_tuner(sd: *mut v4l2_subdev, vt: *const v4l2_tuner) -> c_int {
    static int saa6588_s_tuner(struct v4l2_subdev *sd, const struct v4l2_tuner *vt)
    {
    struct saa6588 *s = to_saa6588(sd);
    saa6588_configure(s);
    return 0;
    }
// -----------------------------------------------------------------------
    static const struct v4l2_subdev_core_ops saa6588_core_ops = {
    .command = saa6588_command,
    };
    static const struct v4l2_subdev_tuner_ops saa6588_tuner_ops = {
    .g_tuner = saa6588_g_tuner,
    .s_tuner = saa6588_s_tuner,
    };
    static const struct v4l2_subdev_ops saa6588_ops = {
    .core = &saa6588_core_ops,
    .tuner = &saa6588_tuner_ops,
    };
// ----------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn saa6588_probe(client: *mut i2c_client) -> c_int {
    static int saa6588_probe(struct i2c_client *client)
    {
    struct saa6588 *s;
    struct v4l2_subdev *sd;
    v4l_info(client, "saa6588 found @ 0x%x (%s)\n",
    client.addr << 1, client.adapter.name);
    s = devm_kzalloc(&client.dev, sizeof(*s), GFP_KERNEL);
    if (s == core::ptr::null_mut())
    return -ENOMEM;
    s.buf_size = bufblocks * 3;
    s.buffer = devm_kzalloc(&client.dev, s.buf_size, GFP_KERNEL);
    if (s.buffer == core::ptr::null_mut())
    return -ENOMEM;
    sd = &s.sd;
    v4l2_i2c_subdev_init(sd, client, &saa6588_ops);
    spin_lock_init(&s.lock);
    s.block_count = 0;
    s.wr_index = 0;
    s.rd_index = 0;
    s.last_blocknum = 0xff;
    init_waitqueue_head(&s.read_queue);
    s.data_available_for_read = 0;
    saa6588_configure(s);
// start polling via eventd
    INIT_DELAYED_WORK(&s.work, saa6588_work);
    schedule_delayed_work(&s.work, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn saa6588_remove(client: *mut i2c_client) {
    static void saa6588_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct saa6588 *s = to_saa6588(sd);
    v4l2_device_unregister_subdev(sd);
    cancel_delayed_work_sync(&s.work);
    }
// -----------------------------------------------------------------------
    static const struct i2c_device_id saa6588_id[] = {
    { .name = "saa6588" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, saa6588_id);
    static struct i2c_driver saa6588_driver = {
    .driver = {
    .name	= "saa6588",
    },
    .probe		= saa6588_probe,
    .remove		= saa6588_remove,
    .id_table	= saa6588_id,
    };
    module_i2c_driver(saa6588_driver);
