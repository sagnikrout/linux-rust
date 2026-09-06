//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/ps3rom.c
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
// PS3 BD/DVD/CD-ROM Storage Driver
//
// Copyright (C) 2007 Sony Computer Entertainment Inc.
// Copyright 2007 Sony Corp.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3rom_private {
    pub dev: *mut ps3_storage_device,
    pub curr_cmd: *mut scsi_cmnd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lv1_atapi_cmnd_block {
    pub /: *mut *mut u8 pkt[32]; / packet command block,
    pub /: *mut *mut u32 pktlen; / should be 12 for ATAPI 8020,
    pub blocks: u32,
    pub block_size: u32,
    pub /: *mut *mut u32 proto; / transfer mode,
    pub /: *mut *mut u32 in_out; / transfer direction,
    pub /: *mut *mut u64 buffer; / parameter except command block,
    pub /: *mut *mut u32 arglen; / length above,
}

    enum lv1_atapi_proto {
    NON_DATA_PROTO     = 0,
    PIO_DATA_IN_PROTO  = 1,
    PIO_DATA_OUT_PROTO = 2,
    DMA_PROTO = 3
    };
    enum lv1_atapi_in_out {
    DIR_WRITE = 0,		/* memory . device */
    DIR_READ = 1		/* device . memory */
    };
    static int ps3rom_sdev_configure(struct scsi_device *scsi_dev,
    struct queue_limits *lim)
    {
    struct ps3rom_private *priv = shost_priv(scsi_dev.host);
    struct ps3_storage_device *dev = priv.dev;
    dev_dbg(&dev.sbd.core, "%s:%u: id %u, lun %llu, channel %u\n", __func__,
    __LINE__, scsi_dev.id, scsi_dev.lun, scsi_dev.channel);
//
// ATAPI SFF8020 devices use MODE_SENSE_10,
// so we can prohibit MODE_SENSE_6
//
    scsi_dev.use_10_for_ms = 1;
// we don't support {READ,WRITE}_6
    scsi_dev.use_10_for_rw = 1;
    return 0;
    }
    static int ps3rom_atapi_request(struct ps3_storage_device *dev,
    struct scsi_cmnd *cmd)
    {
    struct lv1_atapi_cmnd_block atapi_cmnd;
    let mut opcode: c_uchar = cmd.cmnd[0];
    int res;
    u64 lpar;
    dev_dbg(&dev.sbd.core, "%s:%u: send ATAPI command 0x%02x\n", __func__,
    __LINE__, opcode);
    memset(&atapi_cmnd, 0, sizeof(struct lv1_atapi_cmnd_block));
    memcpy(&atapi_cmnd.pkt, cmd.cmnd, 12);
    atapi_cmnd.pktlen = 12;
    atapi_cmnd.block_size = 1; /* transfer size is block_size * blocks */
    atapi_cmnd.blocks = atapi_cmnd.arglen = scsi_bufflen(cmd);
    atapi_cmnd.buffer = dev.bounce_lpar;
    switch (cmd.sc_data_direction) {
    case DMA_FROM_DEVICE:
    if (scsi_bufflen(cmd) >= CD_FRAMESIZE)
    atapi_cmnd.proto = DMA_PROTO;
    else
    atapi_cmnd.proto = PIO_DATA_IN_PROTO;
    atapi_cmnd.in_out = DIR_READ;
    break;
    case DMA_TO_DEVICE:
    if (scsi_bufflen(cmd) >= CD_FRAMESIZE)
    atapi_cmnd.proto = DMA_PROTO;
    else
    atapi_cmnd.proto = PIO_DATA_OUT_PROTO;
    atapi_cmnd.in_out = DIR_WRITE;
    scsi_sg_copy_to_buffer(cmd, dev.bounce_buf, dev.bounce_size);
    break;
    default:
    atapi_cmnd.proto = NON_DATA_PROTO;
    break;
    }
    lpar = ps3_mm_phys_to_lpar(__pa(&atapi_cmnd));
    res = lv1_storage_send_device_command(dev.sbd.dev_id,
    LV1_STORAGE_SEND_ATAPI_COMMAND,
    lpar, sizeof(atapi_cmnd),
    atapi_cmnd.buffer,
    atapi_cmnd.arglen, &dev.tag);
    if (res == LV1_DENIED_BY_POLICY) {
    dev_dbg(&dev.sbd.core,
    "%s:%u: ATAPI command 0x%02x denied by policy\n",
    __func__, __LINE__, opcode);
    return DID_ERROR << 16;
    }
    if (res) {
    dev_err(&dev.sbd.core,
    "%s:%u: ATAPI command 0x%02x failed %d\n", __func__,
    __LINE__, opcode, res);
    return DID_ERROR << 16;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn srb10_lba(cmd: *const scsi_cmnd) -> c_uint {
    static inline unsigned int srb10_lba(const struct scsi_cmnd *cmd)
    {
    return cmd.cmnd[2] << 24 | cmd.cmnd[3] << 16 | cmd.cmnd[4] << 8 |
    cmd.cmnd[5];
    }
#[no_mangle]
pub unsafe extern "C" fn srb10_len(cmd: *const scsi_cmnd) -> c_uint {
    static inline unsigned int srb10_len(const struct scsi_cmnd *cmd)
    {
    return cmd.cmnd[7] << 8 | cmd.cmnd[8];
    }
    static int ps3rom_read_request(struct ps3_storage_device *dev,
    struct scsi_cmnd *cmd, u32 start_sector,
    u32 sectors)
    {
    int res;
    dev_dbg(&dev.sbd.core, "%s:%u: read %u sectors starting at %u\n",
    __func__, __LINE__, sectors, start_sector);
    res = lv1_storage_read(dev.sbd.dev_id,
    dev.regions[dev.region_idx].id, start_sector,
    sectors, 0, dev.bounce_lpar, &dev.tag);
    if (res) {
    dev_err(&dev.sbd.core, "%s:%u: read failed %d\n", __func__,
    __LINE__, res);
    return DID_ERROR << 16;
    }
    return 0;
    }
    static int ps3rom_write_request(struct ps3_storage_device *dev,
    struct scsi_cmnd *cmd, u32 start_sector,
    u32 sectors)
    {
    int res;
    dev_dbg(&dev.sbd.core, "%s:%u: write %u sectors starting at %u\n",
    __func__, __LINE__, sectors, start_sector);
    scsi_sg_copy_to_buffer(cmd, dev.bounce_buf, dev.bounce_size);
    res = lv1_storage_write(dev.sbd.dev_id,
    dev.regions[dev.region_idx].id, start_sector,
    sectors, 0, dev.bounce_lpar, &dev.tag);
    if (res) {
    dev_err(&dev.sbd.core, "%s:%u: write failed %d\n", __func__,
    __LINE__, res);
    return DID_ERROR << 16;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ps3rom_queuecommand_lck(cmd: *mut scsi_cmnd) -> enum scsi_qc_status {
    static enum scsi_qc_status ps3rom_queuecommand_lck(struct scsi_cmnd *cmd)
    {
    struct ps3rom_private *priv = shost_priv(cmd.device.host);
    struct ps3_storage_device *dev = priv.dev;
    unsigned char opcode;
    int res;
    priv.curr_cmd = cmd;
    opcode = cmd.cmnd[0];
//
// While we can submit READ/WRITE SCSI commands as ATAPI commands,
// it's recommended for various reasons (performance, error handling,
// ...) to use lv1_storage_{read,write}() instead
//
    switch (opcode) {
    case READ_10:
    res = ps3rom_read_request(dev, cmd, srb10_lba(cmd),
    srb10_len(cmd));
    break;
    case WRITE_10:
    res = ps3rom_write_request(dev, cmd, srb10_lba(cmd),
    srb10_len(cmd));
    break;
    default:
    res = ps3rom_atapi_request(dev, cmd);
    break;
    }
    if (res) {
    scsi_build_sense(cmd, 0, ILLEGAL_REQUEST, 0, 0);
    cmd.result = res;
    priv.curr_cmd = core::ptr::null_mut();
    scsi_done(cmd);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn DEF_SCSI_QCMD(_arg: ps3rom_queuecommand) -> static {
    static DEF_SCSI_QCMD(ps3rom_queuecommand)
    static int decode_lv1_status(u64 status, unsigned char *sense_key,
    unsigned char *asc, unsigned char *ascq)
    {
    if (((status >> 24) & 0xff) != SAM_STAT_CHECK_CONDITION)
    return -1;
// sense_key = (status >> 16) & 0xff;
// asc       = (status >>  8) & 0xff;
// ascq      =  status        & 0xff;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ps3rom_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ps3rom_interrupt(int irq, void *data)
    {
    struct ps3_storage_device *dev = data;
    struct Scsi_Host *host;
    struct ps3rom_private *priv;
    struct scsi_cmnd *cmd;
    int res;
    u64 tag, status;
    unsigned char sense_key, asc, ascq;
    res = lv1_storage_get_async_status(dev.sbd.dev_id, &tag, &status);
//
// status = -1 may mean that ATAPI transport completed OK, but
// ATAPI command itself resulted CHECK CONDITION
// so, upper layer should issue REQUEST_SENSE to check the sense data
//
    if (tag != dev.tag)
    dev_err(&dev.sbd.core,
    "%s:%u: tag mismatch, got %llx, expected %llx\n",
    __func__, __LINE__, tag, dev.tag);
    if (res) {
    dev_err(&dev.sbd.core, "%s:%u: res=%d status=0x%llx\n",
    __func__, __LINE__, res, status);
    return IRQ_HANDLED;
    }
    host = ps3_system_bus_get_drvdata(&dev.sbd);
    priv = shost_priv(host);
    cmd = priv.curr_cmd;
    if (!status) {
// OK, completed
    if (cmd.sc_data_direction == DMA_FROM_DEVICE) {
    int len;
    len = scsi_sg_copy_from_buffer(cmd,
    dev.bounce_buf,
    dev.bounce_size);
    scsi_set_resid(cmd, scsi_bufflen(cmd) - len);
    }
    cmd.result = DID_OK << 16;
    goto done;
    }
    if (cmd.cmnd[0] == REQUEST_SENSE) {
// SCSI spec says request sense should never get error
    dev_err(&dev.sbd.core, "%s:%u: end error without autosense\n",
    __func__, __LINE__);
    cmd.result = DID_ERROR << 16 | SAM_STAT_CHECK_CONDITION;
    goto done;
    }
    if (decode_lv1_status(status, &sense_key, &asc, &ascq)) {
    cmd.result = DID_ERROR << 16;
    goto done;
    }
    scsi_build_sense(cmd, 0, sense_key, asc, ascq);
    done:
    priv.curr_cmd = core::ptr::null_mut();
    scsi_done(cmd);
    return IRQ_HANDLED;
    }
    static const struct scsi_host_template ps3rom_host_template = {
    .name =			DEVICE_NAME,
    .sdev_configure =	ps3rom_sdev_configure,
    .queuecommand =		ps3rom_queuecommand,
    .can_queue =		1,
    .this_id =		7,
    .sg_tablesize =		SG_ALL,
    .emulated =             1,		/* only sg driver uses this */
    .max_sectors =		PS3ROM_MAX_SECTORS,
    .module =		THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ps3rom_probe(_dev: *mut ps3_system_bus_device) -> c_int {
    static int ps3rom_probe(struct ps3_system_bus_device *_dev)
    {
    struct ps3_storage_device *dev = to_ps3_storage_device(&_dev.core);
    int error;
    struct Scsi_Host *host;
    struct ps3rom_private *priv;
    if (dev.blk_size != CD_FRAMESIZE) {
    dev_err(&dev.sbd.core,
    "%s:%u: cannot handle block size %llu\n", __func__,
    __LINE__, dev.blk_size);
    return -EINVAL;
    }
    dev.bounce_size = BOUNCE_SIZE;
    dev.bounce_buf = kmalloc(BOUNCE_SIZE, GFP_DMA);
    if (!dev.bounce_buf)
    return -ENOMEM;
    error = ps3stor_setup(dev, ps3rom_interrupt);
    if (error)
    goto fail_free_bounce;
    host = scsi_host_alloc(&ps3rom_host_template,
    sizeof(struct ps3rom_private));
    if (!host) {
    dev_err(&dev.sbd.core, "%s:%u: scsi_host_alloc failed\n",
    __func__, __LINE__);
    error = -ENOMEM;
    goto fail_teardown;
    }
    priv = shost_priv(host);
    ps3_system_bus_set_drvdata(&dev.sbd, host);
    priv.dev = dev;
// One device/LUN per SCSI bus
    host.max_id = 1;
    host.max_lun = 1;
    error = scsi_add_host(host, &dev.sbd.core);
    if (error) {
    dev_err(&dev.sbd.core, "%s:%u: scsi_host_alloc failed %d\n",
    __func__, __LINE__, error);
    error = -ENODEV;
    goto fail_host_put;
    }
    scsi_scan_host(host);
    return 0;
    fail_host_put:
    scsi_host_put(host);
    ps3_system_bus_set_drvdata(&dev.sbd, core::ptr::null_mut());
    fail_teardown:
    ps3stor_teardown(dev);
    fail_free_bounce:
    kfree(dev.bounce_buf);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn ps3rom_remove(_dev: *mut ps3_system_bus_device) {
    static void ps3rom_remove(struct ps3_system_bus_device *_dev)
    {
    struct ps3_storage_device *dev = to_ps3_storage_device(&_dev.core);
    struct Scsi_Host *host = ps3_system_bus_get_drvdata(&dev.sbd);
    scsi_remove_host(host);
    ps3stor_teardown(dev);
    scsi_host_put(host);
    ps3_system_bus_set_drvdata(&dev.sbd, core::ptr::null_mut());
    kfree(dev.bounce_buf);
    }
    static struct ps3_system_bus_driver ps3rom = {
    .match_id	= PS3_MATCH_ID_STOR_ROM,
    .core.name	= DEVICE_NAME,
    .core.owner	= THIS_MODULE,
    .probe		= ps3rom_probe,
    .remove		= ps3rom_remove
    };
#[no_mangle]
unsafe extern "C" fn ps3rom_init() -> int __init {
    static int __init ps3rom_init(void)
    {
    return ps3_system_bus_driver_register(&ps3rom);
    }
#[no_mangle]
unsafe extern "C" fn ps3rom_exit() -> void __exit {
    static void __exit ps3rom_exit(void)
    {
    ps3_system_bus_driver_unregister(&ps3rom);
    }
    module_init(ps3rom_init);
    module_exit(ps3rom_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("PS3 BD/DVD/CD-ROM Storage Driver");
    MODULE_AUTHOR("Sony Corporation");
    MODULE_ALIAS(PS3_MODULE_ALIAS_STOR_ROM);
