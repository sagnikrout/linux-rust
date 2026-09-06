//! Automatically rewritten from C to Rust
//! Source: drivers/block/swim.c
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
// Driver for SWIM (Sander Woz Integrated Machine) floppy controller
//
// Copyright (C) 2004,2008 Laurent Vivier <Laurent@lvivier.info>
//
// based on Alastair Bridgewater SWIM analysis, 2001
// based on SWIM3 driver (c) Paul Mackerras, 1996
// based on netBSD IWM driver (c) 1997, 1998 Hauke Fath.
//
// 2004-08-21 (lv) - Initial implementation
// 2008-10-30 (lv) - Port to 2.6
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sector_header {
    pub side: c_uchar,
    pub track: c_uchar,
    pub sector: c_uchar,
    pub size: c_uchar,
    pub crc0: c_uchar,
    pub crc1: c_uchar,
    pub __attribute__((packed)): },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct swim {
    REG(write_data)
    REG(write_mark)
    REG(write_CRC)
    REG(write_parameter)
    REG(write_phase)
    REG(write_setup)
    REG(write_mode0)
    REG(write_mode1)
    REG(read_data)
    REG(read_mark)
    REG(read_error)
    REG(read_parameter)
    REG(read_phase)
    REG(read_setup)
    REG(read_status)
    REG(read_handshake)
    pub __attribute__((packed)): },

// IWM registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwm {
    REG(ph0L)
    REG(ph0H)
    REG(ph1L)
    REG(ph1H)
    REG(ph2L)
    REG(ph2H)
    REG(ph3L)
    REG(ph3H)
    REG(mtrOff)
    REG(mtrOn)
    REG(intDrive)
    REG(extDrive)
    REG(q6L)
    REG(q6H)
    REG(q7L)
    REG(q7H)
    pub __attribute__((packed)): },

// bits in phase register
pub const SEEK_POSITIVE: c_uint = 0x070;
pub const SEEK_NEGATIVE: c_uint = 0x074;
pub const STEP: c_uint = 0x071;
pub const MOTOR_ON: c_uint = 0x072;
pub const MOTOR_OFF: c_uint = 0x076;
pub const INDEX: c_uint = 0x073;
pub const EJECT: c_uint = 0x077;
pub const SETMFM: c_uint = 0x171;
pub const SETGCR: c_uint = 0x175;
pub const RELAX: c_uint = 0x033;
pub const LSTRB: c_uint = 0x008;
pub const CA_MASK: c_uint = 0x077;
// Select values for swim_select and swim_readbit
pub const READ_DATA_0: c_uint = 0x074;
pub const ONEMEG_DRIVE: c_uint = 0x075;
pub const SINGLE_SIDED: c_uint = 0x076;
pub const DRIVE_PRESENT: c_uint = 0x077;
pub const DISK_IN: c_uint = 0x170;
pub const WRITE_PROT: c_uint = 0x171;
pub const TRACK_ZERO: c_uint = 0x172;
pub const TACHO: c_uint = 0x173;
pub const READ_DATA_1: c_uint = 0x174;
pub const GCR_MODE: c_uint = 0x175;
pub const SEEK_COMPLETE: c_uint = 0x176;
pub const TWOMEG_MEDIA: c_uint = 0x177;
// Bits in handshake register
pub const MARK_BYTE: c_uint = 0x01;
pub const CRC_ZERO: c_uint = 0x02;
pub const RDDATA: c_uint = 0x04;
pub const SENSE: c_uint = 0x08;
pub const MOTEN: c_uint = 0x10;
pub const ERROR: c_uint = 0x20;
pub const DAT2BYTE: c_uint = 0x40;
pub const DAT1BYTE: c_uint = 0x80;
// bits in setup register
pub const S_INV_WDATA: c_uint = 0x01;
pub const S_3_5_SELECT: c_uint = 0x02;
pub const S_GCR: c_uint = 0x04;
pub const S_FCLK_DIV2: c_uint = 0x08;
pub const S_ERROR_CORR: c_uint = 0x10;
pub const S_IBM_DRIVE: c_uint = 0x20;
pub const S_GCR_WRITE: c_uint = 0x40;
pub const S_TIMEOUT: c_uint = 0x80;
// bits in mode register
pub const CLFIFO: c_uint = 0x01;
pub const ENBL1: c_uint = 0x02;
pub const ENBL2: c_uint = 0x04;
pub const ACTION: c_uint = 0x08;
pub const WRITE_MODE: c_uint = 0x10;
pub const HEDSEL: c_uint = 0x20;
pub const MOTON: c_uint = 0x80;
// ----------------------------------------------------------------------------
    enum drive_location {
    INTERNAL_DRIVE = 0x02,
    EXTERNAL_DRIVE = 0x04,
}

    enum media_type {
    DD_MEDIA,
    HD_MEDIA,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct floppy_state {
// physical properties
    pub /: *mut *mut enum drive_location location; / internal or external drive,
    pub /: *mut *mut int head_number; / single- or double-sided drive,
// media
    pub disk_in: c_int,
    pub ejected: c_int,
    pub type: enum media_type,
    pub write_protected: c_int,
    pub total_secs: c_int,
    pub secpercyl: c_int,
    pub secpertrack: c_int,
// in-use information
    pub track: c_int,
    pub ref_count: c_int,
    pub registered: bool,
    pub disk: *mut gendisk,
    pub tag_set: blk_mq_tag_set,
// parent controller
    pub swd: *mut swim_priv,
}

    enum motor_action {
    OFF,
    ON,
    };
    enum head {
    LOWER_HEAD = 0,
    UPPER_HEAD = 1,
    };
pub const FD_MAX_UNIT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swim_priv {
    pub base: *mut swim __iomem,
    pub lock: spinlock_t,
    pub floppy_count: c_int,
    pub unit: [floppy_state; FD_MAX_UNIT],
}

    extern int swim_read_sector_header(struct swim __iomem *base,
    struct sector_header *header);
    extern int swim_read_sector_data(struct swim __iomem *base,
    unsigned char *data);
    static DEFINE_MUTEX(swim_mutex);
#[no_mangle]
pub unsafe extern "C" fn set_swim_mode(base: *mut swim __iomem, enable: c_int) {
    static inline void set_swim_mode(struct swim __iomem *base, int enable)
    {
    struct iwm __iomem *iwm_base;
    unsigned long flags;
    if (!enable) {
    swim_write(base, mode0, 0xf8);
    return;
    }
    iwm_base = (struct iwm __iomem *)base;
    local_irq_save(flags);
    iwm_read(iwm_base, q7L);
    iwm_read(iwm_base, mtrOff);
    iwm_read(iwm_base, q6H);
    iwm_write(iwm_base, q7H, 0x57);
    iwm_write(iwm_base, q7H, 0x17);
    iwm_write(iwm_base, q7H, 0x57);
    iwm_write(iwm_base, q7H, 0x57);
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn get_swim_mode(base: *mut swim __iomem) -> c_int {
    static inline int get_swim_mode(struct swim __iomem *base)
    {
    unsigned long flags;
    local_irq_save(flags);
    swim_write(base, phase, 0xf5);
    if (swim_read(base, phase) != 0xf5)
    goto is_iwm;
    swim_write(base, phase, 0xf6);
    if (swim_read(base, phase) != 0xf6)
    goto is_iwm;
    swim_write(base, phase, 0xf7);
    if (swim_read(base, phase) != 0xf7)
    goto is_iwm;
    local_irq_restore(flags);
    return 1;
    is_iwm:
    local_irq_restore(flags);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn swim_select(base: *mut swim __iomem, sel: c_int) {
    static inline void swim_select(struct swim __iomem *base, int sel)
    {
    swim_write(base, phase, RELAX);
    via1_set_head(sel & 0x100);
    swim_write(base, phase, sel & CA_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn swim_action(base: *mut swim __iomem, action: c_int) {
    static inline void swim_action(struct swim __iomem *base, int action)
    {
    unsigned long flags;
    local_irq_save(flags);
    swim_select(base, action);
    udelay(1);
    swim_write(base, phase, (LSTRB<<4) | LSTRB);
    udelay(1);
    swim_write(base, phase, (LSTRB<<4) | ((~LSTRB) & 0x0F));
    udelay(1);
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn swim_readbit(base: *mut swim __iomem, bit: c_int) -> c_int {
    static inline int swim_readbit(struct swim __iomem *base, int bit)
    {
    int stat;
    swim_select(base, bit);
    udelay(10);
    stat = swim_read(base, handshake);
    return (stat & SENSE) == 0;
    }
    static inline void swim_drive(struct swim __iomem *base,
    enum drive_location location)
    {
    if (location == INTERNAL_DRIVE) {
    swim_write(base, mode0, EXTERNAL_DRIVE); /* clear drive 1 bit */
    swim_write(base, mode1, INTERNAL_DRIVE); /* set drive 0 bit */
    } else if (location == EXTERNAL_DRIVE) {
    swim_write(base, mode0, INTERNAL_DRIVE); /* clear drive 0 bit */
    swim_write(base, mode1, EXTERNAL_DRIVE); /* set drive 1 bit */
    }
    }
    static inline void swim_motor(struct swim __iomem *base,
    enum motor_action action)
    {
    if (action == ON) {
    int i;
    swim_action(base, MOTOR_ON);
    for (i = 0; i < 2*HZ; i++) {
    swim_select(base, RELAX);
    if (swim_readbit(base, MOTOR_ON))
    break;
    set_current_state(TASK_INTERRUPTIBLE);
    schedule_timeout(1);
    }
    } else if (action == OFF) {
    swim_action(base, MOTOR_OFF);
    swim_select(base, RELAX);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn swim_eject(base: *mut swim __iomem) {
    static inline void swim_eject(struct swim __iomem *base)
    {
    int i;
    swim_action(base, EJECT);
    for (i = 0; i < 2*HZ; i++) {
    swim_select(base, RELAX);
    if (!swim_readbit(base, DISK_IN))
    break;
    set_current_state(TASK_INTERRUPTIBLE);
    schedule_timeout(1);
    }
    swim_select(base, RELAX);
    }
#[no_mangle]
pub unsafe extern "C" fn swim_head(base: *mut swim __iomem, head: enum head) {
    static inline void swim_head(struct swim __iomem *base, enum head head)
    {
// wait drive is ready
    if (head == UPPER_HEAD)
    swim_select(base, READ_DATA_1);
#[no_mangle]
pub unsafe extern "C" fn if(LOWER_HEAD: head ==) -> else {
    else if (head == LOWER_HEAD)
    swim_select(base, READ_DATA_0);
    }
#[no_mangle]
pub unsafe extern "C" fn swim_step(base: *mut swim __iomem) -> c_int {
    static inline int swim_step(struct swim __iomem *base)
    {
    int wait;
    swim_action(base, STEP);
    for (wait = 0; wait < HZ; wait++) {
    set_current_state(TASK_INTERRUPTIBLE);
    schedule_timeout(1);
    swim_select(base, RELAX);
    if (!swim_readbit(base, STEP))
    return 0;
    }
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn swim_track00(base: *mut swim __iomem) -> c_int {
    static inline int swim_track00(struct swim __iomem *base)
    {
    int try;
    swim_action(base, SEEK_NEGATIVE);
    for (try = 0; try < 100; try++) {
    swim_select(base, RELAX);
    if (swim_readbit(base, TRACK_ZERO))
    break;
    if (swim_step(base))
    return -1;
    }
    if (swim_readbit(base, TRACK_ZERO))
    return 0;
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn swim_seek(base: *mut swim __iomem, step: c_int) -> c_int {
    static inline int swim_seek(struct swim __iomem *base, int step)
    {
    if (step == 0)
    return 0;
    if (step < 0) {
    swim_action(base, SEEK_NEGATIVE);
    step = -step;
    } else
    swim_action(base, SEEK_POSITIVE);
    for ( ; step > 0; step--) {
    if (swim_step(base))
    return -1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn swim_track(fs: *mut floppy_state, track: c_int) -> c_int {
    static inline int swim_track(struct floppy_state *fs,  int track)
    {
    struct swim __iomem *base = fs.swd.base;
    int ret;
    ret = swim_seek(base, track - fs.track);
    if (ret == 0)
    fs.track = track;
    else {
    swim_track00(base);
    fs.track = 0;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn floppy_eject(fs: *mut floppy_state) -> c_int {
    static int floppy_eject(struct floppy_state *fs)
    {
    struct swim __iomem *base = fs.swd.base;
    swim_drive(base, fs.location);
    swim_motor(base, OFF);
    swim_eject(base);
    fs.disk_in = 0;
    fs.ejected = 1;
    return 0;
    }
    static inline int swim_read_sector(struct floppy_state *fs,
    int side, int track,
    int sector, unsigned char *buffer)
    {
    struct swim __iomem *base = fs.swd.base;
    unsigned long flags;
    struct sector_header header;
    let mut ret: c_int = -1;
    short i;
    swim_track(fs, track);
    swim_write(base, mode1, MOTON);
    swim_head(base, side);
    swim_write(base, mode0, side);
    local_irq_save(flags);
    for (i = 0; i < 36; i++) {
    ret = swim_read_sector_header(base, &header);
    if (!ret && (header.sector == sector)) {
// found
    ret = swim_read_sector_data(base, buffer);
    break;
    }
    }
    local_irq_restore(flags);
    swim_write(base, mode0, MOTON);
    if ((header.side != side)  || (header.track != track) ||
    (header.sector != sector))
    return 0;
    return ret;
    }
    static blk_status_t floppy_read_sectors(struct floppy_state *fs,
    int req_sector, int sectors_nb,
    unsigned char *buffer)
    {
    struct swim __iomem *base = fs.swd.base;
    int ret;
    int side, track, sector;
    int i, try;
    swim_drive(base, fs.location);
    for (i = req_sector; i < req_sector + sectors_nb; i++) {
    int x;
    track = i / fs.secpercyl;
    x = i % fs.secpercyl;
    side = x / fs.secpertrack;
    sector = x % fs.secpertrack + 1;
    try = 5;
    do {
    ret = swim_read_sector(fs, side, track, sector,
    buffer);
    if (try-- == 0)
    return BLK_STS_IOERR;
    } while (ret != 512);
    buffer += ret;
    }
    return 0;
    }
    static blk_status_t swim_queue_rq(struct blk_mq_hw_ctx *hctx,
    const struct blk_mq_queue_data *bd)
    {
    struct floppy_state *fs = hctx.queue.queuedata;
    struct swim_priv *swd = fs.swd;
    struct request *req = bd.rq;
    blk_status_t err;
    if (!spin_trylock_irq(&swd.lock))
    return BLK_STS_DEV_RESOURCE;
    blk_mq_start_request(req);
    if (!fs.disk_in || rq_data_dir(req) == WRITE) {
    err = BLK_STS_IOERR;
    goto out;
    }
    do {
    err = floppy_read_sectors(fs, blk_rq_pos(req),
    blk_rq_cur_sectors(req),
    bio_data(req.bio));
    } while (blk_update_request(req, err, blk_rq_cur_bytes(req)));
    __blk_mq_end_request(req, err);
    err = BLK_STS_OK;
    out:
    spin_unlock_irq(&swd.lock);
    return err;
    }
    static struct floppy_struct floppy_type[4] = {
    {    0,  0, 0,  0, 0, 0x00, 0x00, 0x00, 0x00, core::ptr::null_mut() }, /* no testing   */
    {  720,  9, 1, 80, 0, 0x2A, 0x02, 0xDF, 0x50, core::ptr::null_mut() }, /* 360KB SS 3.5"*/
    { 1440,  9, 2, 80, 0, 0x2A, 0x02, 0xDF, 0x50, core::ptr::null_mut() }, /* 720KB 3.5"   */
    { 2880, 18, 2, 80, 0, 0x1B, 0x00, 0xCF, 0x6C, core::ptr::null_mut() }, /* 1.44MB 3.5"  */
    };
    static int get_floppy_geometry(struct floppy_state *fs, int type,
    struct floppy_struct **g)
    {
    if (type >= ARRAY_SIZE(floppy_type))
    return -EINVAL;
    if (type)
// g = &floppy_type[type];
    else if (fs.type == HD_MEDIA) /* High-Density media */
// g = &floppy_type[3];
    else if (fs.head_number == 2) /* double-sided */
// g = &floppy_type[2];
    else
// g = &floppy_type[1];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn setup_medium(fs: *mut floppy_state) {
    static void setup_medium(struct floppy_state *fs)
    {
    struct swim __iomem *base = fs.swd.base;
    if (swim_readbit(base, DISK_IN)) {
    struct floppy_struct *g;
    fs.disk_in = 1;
    fs.write_protected = swim_readbit(base, WRITE_PROT);
    if (swim_track00(base))
    printk(KERN_ERR
    "SWIM: cannot move floppy head to track 0\n");
    swim_track00(base);
    fs.type = swim_readbit(base, TWOMEG_MEDIA) ?
    HD_MEDIA : DD_MEDIA;
    fs.head_number = swim_readbit(base, SINGLE_SIDED) ? 1 : 2;
    get_floppy_geometry(fs, 0, &g);
    fs.total_secs = g.size;
    fs.secpercyl = g.head * g.sect;
    fs.secpertrack = g.sect;
    fs.track = 0;
    } else {
    fs.disk_in = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn floppy_open(disk: *mut gendisk, mode: blk_mode_t) -> c_int {
    static int floppy_open(struct gendisk *disk, blk_mode_t mode)
    {
    struct floppy_state *fs = disk.private_data;
    struct swim __iomem *base = fs.swd.base;
    int err;
    if (fs.ref_count == -1 || (fs.ref_count && mode & BLK_OPEN_EXCL))
    return -EBUSY;
    if (mode & BLK_OPEN_EXCL)
    fs.ref_count = -1;
    else
    fs.ref_count++;
    swim_write(base, setup, S_IBM_DRIVE  | S_FCLK_DIV2);
    udelay(10);
    swim_drive(base, fs.location);
    swim_motor(base, ON);
    swim_action(base, SETMFM);
    if (fs.ejected)
    setup_medium(fs);
    if (!fs.disk_in) {
    err = -ENXIO;
    goto out;
    }
    set_capacity(fs.disk, fs.total_secs);
    if (mode & BLK_OPEN_NDELAY)
    return 0;
    if (mode & (BLK_OPEN_READ | BLK_OPEN_WRITE)) {
    if (disk_check_media_change(disk) && fs.disk_in)
    fs.ejected = 0;
    if ((mode & BLK_OPEN_WRITE) && fs.write_protected) {
    err = -EROFS;
    goto out;
    }
    }
    return 0;
    out:
    if (fs.ref_count < 0)
    fs.ref_count = 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: fs->ref_count >) -> else {
    else if (fs.ref_count > 0)
    --fs.ref_count;
    if (fs.ref_count == 0)
    swim_motor(base, OFF);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn floppy_unlocked_open(disk: *mut gendisk, mode: blk_mode_t) -> c_int {
    static int floppy_unlocked_open(struct gendisk *disk, blk_mode_t mode)
    {
    int ret;
    mutex_lock(&swim_mutex);
    ret = floppy_open(disk, mode);
    mutex_unlock(&swim_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn floppy_release(disk: *mut gendisk) {
    static void floppy_release(struct gendisk *disk)
    {
    struct floppy_state *fs = disk.private_data;
    struct swim __iomem *base = fs.swd.base;
    mutex_lock(&swim_mutex);
    if (fs.ref_count < 0)
    fs.ref_count = 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: fs->ref_count >) -> else {
    else if (fs.ref_count > 0)
    --fs.ref_count;
    if (fs.ref_count == 0)
    swim_motor(base, OFF);
    mutex_unlock(&swim_mutex);
    }
    static int floppy_ioctl(struct block_device *bdev, blk_mode_t mode,
    unsigned int cmd, unsigned long param)
    {
    struct floppy_state *fs = bdev.bd_disk.private_data;
    int err;
    if ((cmd & 0x80) && !capable(CAP_SYS_ADMIN))
    return -EPERM;
    switch (cmd) {
    case FDEJECT:
    if (fs.ref_count != 1)
    return -EBUSY;
    mutex_lock(&swim_mutex);
    err = floppy_eject(fs);
    mutex_unlock(&swim_mutex);
    return err;
    case FDGETPRM:
    if (copy_to_user((void __user *) param, (void *) &floppy_type,
    sizeof(struct floppy_struct)))
    return -EFAULT;
    return 0;
    }
    return -ENOTTY;
    }
#[no_mangle]
unsafe extern "C" fn floppy_getgeo(disk: *mut gendisk, geo: *mut hd_geometry) -> c_int {
    static int floppy_getgeo(struct gendisk *disk, struct hd_geometry *geo)
    {
    struct floppy_state *fs = disk.private_data;
    struct floppy_struct *g;
    int ret;
    ret = get_floppy_geometry(fs, 0, &g);
    if (ret)
    return ret;
    geo.heads = g.head;
    geo.sectors = g.sect;
    geo.cylinders = g.track;
    return 0;
    }
    static unsigned int floppy_check_events(struct gendisk *disk,
    unsigned int clearing)
    {
    struct floppy_state *fs = disk.private_data;
    return fs.ejected ? DISK_EVENT_MEDIA_CHANGE : 0;
    }
    static const struct block_device_operations floppy_fops = {
    .owner		 = THIS_MODULE,
    .open		 = floppy_unlocked_open,
    .release	 = floppy_release,
    .ioctl		 = floppy_ioctl,
    .getgeo		 = floppy_getgeo,
    .check_events	 = floppy_check_events,
    };
#[no_mangle]
unsafe extern "C" fn swim_add_floppy(swd: *mut swim_priv, location: enum drive_location) -> c_int {
    static int swim_add_floppy(struct swim_priv *swd, enum drive_location location)
    {
    struct floppy_state *fs = &swd.unit[swd.floppy_count];
    struct swim __iomem *base = swd.base;
    fs.location = location;
    swim_drive(base, location);
    swim_motor(base, OFF);
    fs.type = HD_MEDIA;
    fs.head_number = 2;
    fs.ref_count = 0;
    fs.ejected = 1;
    swd.floppy_count++;
    return 0;
    }
    static const struct blk_mq_ops swim_mq_ops = {
    .queue_rq = swim_queue_rq,
    };
#[no_mangle]
unsafe extern "C" fn swim_cleanup_floppy_disk(fs: *mut floppy_state) {
    static void swim_cleanup_floppy_disk(struct floppy_state *fs)
    {
    struct gendisk *disk = fs.disk;
    if (!disk)
    return;
    if (fs.registered)
    del_gendisk(fs.disk);
    put_disk(disk);
    blk_mq_free_tag_set(&fs.tag_set);
    }
#[no_mangle]
unsafe extern "C" fn swim_floppy_init(swd: *mut swim_priv) -> c_int {
    static int swim_floppy_init(struct swim_priv *swd)
    {
    struct queue_limits lim = {
    .features		= BLK_FEAT_ROTATIONAL,
    };
    int err;
    int drive;
    struct swim __iomem *base = swd.base;
// scan floppy drives
    swim_drive(base, INTERNAL_DRIVE);
    if (swim_readbit(base, DRIVE_PRESENT) &&
    !swim_readbit(base, ONEMEG_DRIVE))
    swim_add_floppy(swd, INTERNAL_DRIVE);
    swim_drive(base, EXTERNAL_DRIVE);
    if (swim_readbit(base, DRIVE_PRESENT) &&
    !swim_readbit(base, ONEMEG_DRIVE))
    swim_add_floppy(swd, EXTERNAL_DRIVE);
// register floppy drives
    err = register_blkdev(FLOPPY_MAJOR, "fd");
    if (err) {
    printk(KERN_ERR "Unable to get major %d for SWIM floppy\n",
    FLOPPY_MAJOR);
    return -EBUSY;
    }
    spin_lock_init(&swd.lock);
    for (drive = 0; drive < swd.floppy_count; drive++) {
    err = blk_mq_alloc_sq_tag_set(&swd.unit[drive].tag_set,
    &swim_mq_ops, 2, 0);
    if (err)
    goto exit_put_disks;
    swd.unit[drive].disk =
    blk_mq_alloc_disk(&swd.unit[drive].tag_set, &lim,
    &swd.unit[drive]);
    if (IS_ERR(swd.unit[drive].disk)) {
    blk_mq_free_tag_set(&swd.unit[drive].tag_set);
    err = PTR_ERR(swd.unit[drive].disk);
    goto exit_put_disks;
    }
    swd.unit[drive].swd = swd;
    }
    for (drive = 0; drive < swd.floppy_count; drive++) {
    swd.unit[drive].disk.flags = GENHD_FL_REMOVABLE;
    swd.unit[drive].disk.major = FLOPPY_MAJOR;
    swd.unit[drive].disk.first_minor = drive;
    swd.unit[drive].disk.minors = 1;
    sprintf(swd.unit[drive].disk.disk_name, "fd%d", drive);
    swd.unit[drive].disk.fops = &floppy_fops;
    swd.unit[drive].disk.flags |= GENHD_FL_NO_PART;
    swd.unit[drive].disk.events = DISK_EVENT_MEDIA_CHANGE;
    swd.unit[drive].disk.private_data = &swd.unit[drive];
    set_capacity(swd.unit[drive].disk, 2880);
    err = add_disk(swd.unit[drive].disk);
    if (err)
    goto exit_put_disks;
    swd.unit[drive].registered = true;
    }
    return 0;
    exit_put_disks:
    unregister_blkdev(FLOPPY_MAJOR, "fd");
    do {
    swim_cleanup_floppy_disk(&swd.unit[drive]);
    } while (drive--);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn swim_probe(dev: *mut platform_device) -> c_int {
    static int swim_probe(struct platform_device *dev)
    {
    struct resource *res;
    struct swim __iomem *swim_base;
    struct swim_priv *swd;
    int ret;
    res = platform_get_resource(dev, IORESOURCE_MEM, 0);
    if (!res) {
    ret = -ENODEV;
    goto out;
    }
    if (!request_mem_region(res.start, resource_size(res), CARDNAME)) {
    ret = -EBUSY;
    goto out;
    }
    swim_base = (struct swim __iomem *)res.start;
    if (!swim_base) {
    ret = -ENOMEM;
    goto out_release_io;
    }
// probe device
    set_swim_mode(swim_base, 1);
    if (!get_swim_mode(swim_base)) {
    printk(KERN_INFO "SWIM device not found !\n");
    ret = -ENODEV;
    goto out_release_io;
    }
// set platform driver data
    swd = kzalloc_obj(struct swim_priv);
    if (!swd) {
    ret = -ENOMEM;
    goto out_release_io;
    }
    platform_set_drvdata(dev, swd);
    swd.base = swim_base;
    ret = swim_floppy_init(swd);
    if (ret)
    goto out_kfree;
    return 0;
    out_kfree:
    kfree(swd);
    out_release_io:
    release_mem_region(res.start, resource_size(res));
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn swim_remove(dev: *mut platform_device) {
    static void swim_remove(struct platform_device *dev)
    {
    struct swim_priv *swd = platform_get_drvdata(dev);
    int drive;
    struct resource *res;
    for (drive = 0; drive < swd.floppy_count; drive++)
    swim_cleanup_floppy_disk(&swd.unit[drive]);
    unregister_blkdev(FLOPPY_MAJOR, "fd");
// eject floppies
    for (drive = 0; drive < swd.floppy_count; drive++)
    floppy_eject(&swd.unit[drive]);
    res = platform_get_resource(dev, IORESOURCE_MEM, 0);
    if (res)
    release_mem_region(res.start, resource_size(res));
    kfree(swd);
    }
    static struct platform_driver swim_driver = {
    .probe  = swim_probe,
    .remove = swim_remove,
    .driver   = {
    .name	= CARDNAME,
    },
    };
#[no_mangle]
unsafe extern "C" fn swim_init() -> int __init {
    static int __init swim_init(void)
    {
    printk(KERN_INFO "SWIM floppy driver %s\n", DRIVER_VERSION);
    return platform_driver_register(&swim_driver);
    }
    module_init(swim_init);
#[no_mangle]
unsafe extern "C" fn swim_exit() -> void __exit {
    static void __exit swim_exit(void)
    {
    platform_driver_unregister(&swim_driver);
    }
    module_exit(swim_exit);
    MODULE_DESCRIPTION("Driver for SWIM floppy controller");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Laurent Vivier <laurent@lvivier.info>");
    MODULE_ALIAS_BLOCKDEV_MAJOR(FLOPPY_MAJOR);
