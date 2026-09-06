//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/lpddr/lpddr_cmds.c
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
// LPDDR flash memory device operations. This module provides read, write,
// erase, lock/unlock support for LPDDR flash memories
// (C) 2008 Korolev Alexey <akorolev@infradead.org>
// (C) 2008 Vasiliy Leonenko <vasiliy.leonenko@gmail.com>
// Many thanks to Roman Borisov for initial enabling
//
// TODO:
// Implement VPP management
// Implement XIP support
// Implement OTP support
//

    static int lpddr_read(struct mtd_info *mtd, loff_t adr, size_t len,
    size_t *retlen, u_char *buf);
    static int lpddr_write_buffers(struct mtd_info *mtd, loff_t to,
    size_t len, size_t *retlen, const u_char *buf);
    static int lpddr_writev(struct mtd_info *mtd, const struct kvec *vecs,
    unsigned long count, loff_t to, size_t *retlen);
    static int lpddr_erase(struct mtd_info *mtd, struct erase_info *instr);
    static int lpddr_lock(struct mtd_info *mtd, loff_t ofs, uint64_t len);
    static int lpddr_unlock(struct mtd_info *mtd, loff_t ofs, uint64_t len);
    static int lpddr_point(struct mtd_info *mtd, loff_t adr, size_t len,
    size_t *retlen, void **mtdbuf, resource_size_t *phys);
    static int lpddr_unpoint(struct mtd_info *mtd, loff_t adr, size_t len);
    static int get_chip(struct map_info *map, struct flchip *chip, int mode);
    static int chip_ready(struct map_info *map, struct flchip *chip, int mode);
    static void put_chip(struct map_info *map, struct flchip *chip);
    struct mtd_info *lpddr_cmdset(struct map_info *map)
    {
    struct lpddr_private *lpddr = map.fldrv_priv;
    struct flchip_shared *shared;
    struct flchip *chip;
    struct mtd_info *mtd;
    int numchips;
    int i, j;
    mtd = kzalloc_obj(*mtd);
    if (!mtd)
    return core::ptr::null_mut();
    mtd.priv = map;
    mtd.type = MTD_NORFLASH;
// Fill in the default mtd operations
    mtd._read = lpddr_read;
    mtd.type = MTD_NORFLASH;
    mtd.flags = MTD_CAP_NORFLASH;
    mtd.flags &= ~MTD_BIT_WRITEABLE;
    mtd._erase = lpddr_erase;
    mtd._write = lpddr_write_buffers;
    mtd._writev = lpddr_writev;
    mtd._lock = lpddr_lock;
    mtd._unlock = lpddr_unlock;
    if (map_is_linear(map)) {
    mtd._point = lpddr_point;
    mtd._unpoint = lpddr_unpoint;
    }
    mtd.size = 1ULL << lpddr.qinfo.DevSizeShift;
    mtd.erasesize = 1 << lpddr.qinfo.UniformBlockSizeShift;
    mtd.writesize = 1 << lpddr.qinfo.BufSizeShift;
    shared = kmalloc_objs(struct flchip_shared, lpddr.numchips);
    if (!shared) {
    kfree(mtd);
    return core::ptr::null_mut();
    }
    chip = &lpddr.chips[0];
    numchips = lpddr.numchips / lpddr.qinfo.HWPartsNum;
    for (i = 0; i < numchips; i++) {
    shared[i].writing = shared[i].erasing = core::ptr::null_mut();
    mutex_init(&shared[i].lock);
    for (j = 0; j < lpddr.qinfo.HWPartsNum; j++) {
// chip = lpddr->chips[i];
    chip.start += (unsigned long)j << lpddr.chipshift;
    chip.oldstate = chip.state = FL_READY;
    chip.priv = &shared[i];
// those should be reset too since
    they create memory references. */
    init_waitqueue_head(&chip.wq);
    mutex_init(&chip.mutex);
    chip++;
    }
    }
    return mtd;
    }
    EXPORT_SYMBOL(lpddr_cmdset);
#[no_mangle]
unsafe extern "C" fn print_drs_error(dsr: c_uint) {
    static void print_drs_error(unsigned int dsr)
    {
    let mut prog_status: c_int = (dsr & DSR_RPS) >> 8;
    if (!(dsr & DSR_AVAILABLE))
    pr_notice("DSR.15: (0) Device not Available\n");
    if ((prog_status & 0x03) == 0x03)
    pr_notice("DSR.9,8: (11) Attempt to program invalid half with 41h command\n");
#[no_mangle]
pub unsafe extern "C" fn if(0x02: prog_status &) -> else {
    else if (prog_status & 0x02)
    pr_notice("DSR.9,8: (10) Object Mode Program attempt in region with Control Mode data\n");
#[no_mangle]
pub unsafe extern "C" fn if(0x01: prog_status &) -> else {
    else if (prog_status &  0x01)
    pr_notice("DSR.9,8: (01) Program attempt in region with Object Mode data\n");
    if (!(dsr & DSR_READY_STATUS))
    pr_notice("DSR.7: (0) Device is Busy\n");
    if (dsr & DSR_ESS)
    pr_notice("DSR.6: (1) Erase Suspended\n");
    if (dsr & DSR_ERASE_STATUS)
    pr_notice("DSR.5: (1) Erase/Blank check error\n");
    if (dsr & DSR_PROGRAM_STATUS)
    pr_notice("DSR.4: (1) Program Error\n");
    if (dsr & DSR_VPPS)
    pr_notice("DSR.3: (1) Vpp low detect, operation aborted\n");
    if (dsr & DSR_PSS)
    pr_notice("DSR.2: (1) Program suspended\n");
    if (dsr & DSR_DPS)
    pr_notice("DSR.1: (1) Aborted Erase/Program attempt on locked block\n");
    }
    static int wait_for_ready(struct map_info *map, struct flchip *chip,
    unsigned int chip_op_time)
    {
    unsigned int timeo, reset_timeo, sleep_time;
    unsigned int dsr;
    let mut chip_state: flstate_t = chip.state;
    let mut ret: c_int = 0;
// set our timeout to 8 times the expected delay
    timeo = chip_op_time * 8;
    if (!timeo)
    timeo = 500000;
    reset_timeo = timeo;
    sleep_time = chip_op_time / 2;
    for (;;) {
    dsr = CMDVAL(map_read(map, map.pfow_base + PFOW_DSR));
    if (dsr & DSR_READY_STATUS)
    break;
    if (!timeo) {
    printk(KERN_ERR "%s: Flash timeout error state %d\n",
    map.name, chip_state);
    ret = -ETIME;
    break;
    }
// OK Still waiting. Drop the lock, wait a while and retry.
    mutex_unlock(&chip.mutex);
    if (sleep_time >= 1000000/HZ) {
//
// Half of the normal delay still remaining
// can be performed with a sleeping delay instead
// of busy waiting.
//
    msleep(sleep_time/1000);
    timeo -= sleep_time;
    sleep_time = 1000000/HZ;
    } else {
    udelay(1);
    cond_resched();
    timeo--;
    }
    mutex_lock(&chip.mutex);
    while (chip.state != chip_state) {
// Someone's suspended the operation: sleep
    DECLARE_WAITQUEUE(wait, current);
    set_current_state(TASK_UNINTERRUPTIBLE);
    add_wait_queue(&chip.wq, &wait);
    mutex_unlock(&chip.mutex);
    schedule();
    remove_wait_queue(&chip.wq, &wait);
    mutex_lock(&chip.mutex);
    }
    if (chip.erase_suspended || chip.write_suspended)  {
// Suspend has occurred while sleep: reset timeout
    timeo = reset_timeo;
    chip.erase_suspended = chip.write_suspended = 0;
    }
    }
// check status for errors
    if (dsr & DSR_ERR) {
// Clear DSR
    map_write(map, CMD(~(DSR_ERR)), map.pfow_base + PFOW_DSR);
    printk(KERN_WARNING"%s: Bad status on wait: 0x%x\n",
    map.name, dsr);
    print_drs_error(dsr);
    ret = -EIO;
    }
    chip.state = FL_READY;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn get_chip(map: *mut map_info, chip: *mut flchip, mode: c_int) -> c_int {
    static int get_chip(struct map_info *map, struct flchip *chip, int mode)
    {
    int ret;
    DECLARE_WAITQUEUE(wait, current);
    retry:
    if (chip.priv && (mode == FL_WRITING || mode == FL_ERASING)
    && chip.state != FL_SYNCING) {
//
// OK. We have possibility for contension on the write/erase
// operations which are global to the real chip and not per
// partition.  So let's fight it over in the partition which
// currently has authority on the operation.
//
// The rules are as follows:
//
// - any write operation must own shared->writing.
//
// - any erase operation must own _both_ shared->writing and
// shared->erasing.
//
// - contension arbitration is handled in the owner's context.
//
// The 'shared' struct can be read and/or written only when
// its lock is taken.
//
    struct flchip_shared *shared = chip.priv;
    struct flchip *contender;
    mutex_lock(&shared.lock);
    contender = shared.writing;
    if (contender && contender != chip) {
//
// The engine to perform desired operation on this
// partition is already in use by someone else.
// Let's fight over it in the context of the chip
// currently using it.  If it is possible to suspend,
// that other partition will do just that, otherwise
// it'll happily send us to sleep.  In any case, when
// get_chip returns success we're clear to go ahead.
//
    ret = mutex_trylock(&contender.mutex);
    mutex_unlock(&shared.lock);
    if (!ret)
    goto retry;
    mutex_unlock(&chip.mutex);
    ret = chip_ready(map, contender, mode);
    mutex_lock(&chip.mutex);
    if (ret == -EAGAIN) {
    mutex_unlock(&contender.mutex);
    goto retry;
    }
    if (ret) {
    mutex_unlock(&contender.mutex);
    return ret;
    }
    mutex_lock(&shared.lock);
// We should not own chip if it is already in FL_SYNCING
// state. Put contender and retry.
    if (chip.state == FL_SYNCING) {
    put_chip(map, contender);
    mutex_unlock(&contender.mutex);
    goto retry;
    }
    mutex_unlock(&contender.mutex);
    }
// Check if we have suspended erase on this chip.
    Must sleep in such a case. */
    if (mode == FL_ERASING && shared.erasing
    && shared.erasing.oldstate == FL_ERASING) {
    mutex_unlock(&shared.lock);
    set_current_state(TASK_UNINTERRUPTIBLE);
    add_wait_queue(&chip.wq, &wait);
    mutex_unlock(&chip.mutex);
    schedule();
    remove_wait_queue(&chip.wq, &wait);
    mutex_lock(&chip.mutex);
    goto retry;
    }
// We now own it
    shared.writing = chip;
    if (mode == FL_ERASING)
    shared.erasing = chip;
    mutex_unlock(&shared.lock);
    }
    ret = chip_ready(map, chip, mode);
    if (ret == -EAGAIN)
    goto retry;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn chip_ready(map: *mut map_info, chip: *mut flchip, mode: c_int) -> c_int {
    static int chip_ready(struct map_info *map, struct flchip *chip, int mode)
    {
    struct lpddr_private *lpddr = map.fldrv_priv;
    let mut ret: c_int = 0;
    DECLARE_WAITQUEUE(wait, current);
// Prevent setting state FL_SYNCING for chip in suspended state.
    if (FL_SYNCING == mode && FL_READY != chip.oldstate)
    goto sleep;
    switch (chip.state) {
    case FL_READY:
    case FL_JEDEC_QUERY:
    return 0;
    case FL_ERASING:
    if (!lpddr.qinfo.SuspEraseSupp ||
    !(mode == FL_READY || mode == FL_POINT))
    goto sleep;
    map_write(map, CMD(LPDDR_SUSPEND),
    map.pfow_base + PFOW_PROGRAM_ERASE_SUSPEND);
    chip.oldstate = FL_ERASING;
    chip.state = FL_ERASE_SUSPENDING;
    ret = wait_for_ready(map, chip, 0);
    if (ret) {
// Oops. something got wrong.
// Resume and pretend we weren't here.
    put_chip(map, chip);
    printk(KERN_ERR "%s: suspend operation failed."
    "State may be wrong\n", map.name);
    return -EIO;
    }
    chip.erase_suspended = 1;
    chip.state = FL_READY;
    return 0;
// Erase suspend
    case FL_POINT:
// Only if there's no operation suspended...
    if (mode == FL_READY && chip.oldstate == FL_READY)
    return 0;
    fallthrough;
    default:
    sleep:
    set_current_state(TASK_UNINTERRUPTIBLE);
    add_wait_queue(&chip.wq, &wait);
    mutex_unlock(&chip.mutex);
    schedule();
    remove_wait_queue(&chip.wq, &wait);
    mutex_lock(&chip.mutex);
    return -EAGAIN;
    }
    }
#[no_mangle]
unsafe extern "C" fn put_chip(map: *mut map_info, chip: *mut flchip) {
    static void put_chip(struct map_info *map, struct flchip *chip)
    {
    if (chip.priv) {
    struct flchip_shared *shared = chip.priv;
    mutex_lock(&shared.lock);
    if (shared.writing == chip && chip.oldstate == FL_READY) {
// We own the ability to write, but we're done
    shared.writing = shared.erasing;
    if (shared.writing && shared.writing != chip) {
// give back the ownership
    struct flchip *loaner = shared.writing;
    mutex_lock(&loaner.mutex);
    mutex_unlock(&shared.lock);
    mutex_unlock(&chip.mutex);
    put_chip(map, loaner);
    mutex_lock(&chip.mutex);
    mutex_unlock(&loaner.mutex);
    wake_up(&chip.wq);
    return;
    }
    shared.erasing = core::ptr::null_mut();
    shared.writing = core::ptr::null_mut();
    } else if (shared.erasing == chip && shared.writing != chip) {
//
// We own the ability to erase without the ability
// to write, which means the erase was suspended
// and some other partition is currently writing.
// Don't let the switch below mess things up since
// we don't have ownership to resume anything.
//
    mutex_unlock(&shared.lock);
    wake_up(&chip.wq);
    return;
    }
    mutex_unlock(&shared.lock);
    }
    switch (chip.oldstate) {
    case FL_ERASING:
    map_write(map, CMD(LPDDR_RESUME),
    map.pfow_base + PFOW_COMMAND_CODE);
    map_write(map, CMD(LPDDR_START_EXECUTION),
    map.pfow_base + PFOW_COMMAND_EXECUTE);
    chip.oldstate = FL_READY;
    chip.state = FL_ERASING;
    break;
    case FL_READY:
    break;
    default:
    printk(KERN_ERR "%s: put_chip() called with oldstate %d!\n",
    map.name, chip.oldstate);
    }
    wake_up(&chip.wq);
    }
    static int do_write_buffer(struct map_info *map, struct flchip *chip,
    unsigned long adr, const struct kvec **pvec,
    unsigned long *pvec_seek, int len)
    {
    struct lpddr_private *lpddr = map.fldrv_priv;
    map_word datum;
    int ret, wbufsize, word_gap;
    const struct kvec *vec;
    unsigned long vec_seek;
    unsigned long prog_buf_ofs;
    wbufsize = 1 << lpddr.qinfo.BufSizeShift;
    mutex_lock(&chip.mutex);
    ret = get_chip(map, chip, FL_WRITING);
    if (ret) {
    mutex_unlock(&chip.mutex);
    return ret;
    }
// Figure out the number of words to write
    word_gap = (-adr & (map_bankwidth(map)-1));
    if (word_gap) {
    word_gap = map_bankwidth(map) - word_gap;
    adr -= word_gap;
    datum = map_word_ff(map);
    }
// Write data
// Get the program buffer offset from PFOW register data first
    prog_buf_ofs = map.pfow_base + CMDVAL(map_read(map,
    map.pfow_base + PFOW_PROGRAM_BUFFER_OFFSET));
    vec = *pvec;
    vec_seek = *pvec_seek;
    do {
    let mut n: c_int = map_bankwidth(map) - word_gap;
    if (n > vec.iov_len - vec_seek)
    n = vec.iov_len - vec_seek;
    if (n > len)
    n = len;
    if (!word_gap && (len < map_bankwidth(map)))
    datum = map_word_ff(map);
    datum = map_word_load_partial(map, datum,
    vec.iov_base + vec_seek, word_gap, n);
    len -= n;
    word_gap += n;
    if (!len || word_gap == map_bankwidth(map)) {
    map_write(map, datum, prog_buf_ofs);
    prog_buf_ofs += map_bankwidth(map);
    word_gap = 0;
    }
    vec_seek += n;
    if (vec_seek == vec.iov_len) {
    vec++;
    vec_seek = 0;
    }
    } while (len);
// pvec = vec;
// pvec_seek = vec_seek;
// GO GO GO
    send_pfow_command(map, LPDDR_BUFF_PROGRAM, adr, wbufsize, core::ptr::null_mut());
    chip.state = FL_WRITING;
    ret = wait_for_ready(map, chip, (1<<lpddr.qinfo.ProgBufferTime));
    if (ret)	{
    printk(KERN_WARNING"%s Buffer program error: %d at %lx\n",
    map.name, ret, adr);
    goto out;
    }
    out:	put_chip(map, chip);
    mutex_unlock(&chip.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn do_erase_oneblock(mtd: *mut mtd_info, adr: loff_t) -> c_int {
    static int do_erase_oneblock(struct mtd_info *mtd, loff_t adr)
    {
    struct map_info *map = mtd.priv;
    struct lpddr_private *lpddr = map.fldrv_priv;
    let mut chipnum: c_int = adr >> lpddr.chipshift;
    struct flchip *chip = &lpddr.chips[chipnum];
    int ret;
    mutex_lock(&chip.mutex);
    ret = get_chip(map, chip, FL_ERASING);
    if (ret) {
    mutex_unlock(&chip.mutex);
    return ret;
    }
    send_pfow_command(map, LPDDR_BLOCK_ERASE, adr, 0, core::ptr::null_mut());
    chip.state = FL_ERASING;
    ret = wait_for_ready(map, chip, (1<<lpddr.qinfo.BlockEraseTime)*1000);
    if (ret) {
    printk(KERN_WARNING"%s Erase block error %d at : %llx\n",
    map.name, ret, adr);
    goto out;
    }
    out:	put_chip(map, chip);
    mutex_unlock(&chip.mutex);
    return ret;
    }
    static int lpddr_read(struct mtd_info *mtd, loff_t adr, size_t len,
    size_t *retlen, u_char *buf)
    {
    struct map_info *map = mtd.priv;
    struct lpddr_private *lpddr = map.fldrv_priv;
    let mut chipnum: c_int = adr >> lpddr.chipshift;
    struct flchip *chip = &lpddr.chips[chipnum];
    let mut ret: c_int = 0;
    mutex_lock(&chip.mutex);
    ret = get_chip(map, chip, FL_READY);
    if (ret) {
    mutex_unlock(&chip.mutex);
    return ret;
    }
    map_copy_from(map, buf, adr, len);
// retlen = len;
    put_chip(map, chip);
    mutex_unlock(&chip.mutex);
    return ret;
    }
    static int lpddr_point(struct mtd_info *mtd, loff_t adr, size_t len,
    size_t *retlen, void **mtdbuf, resource_size_t *phys)
    {
    struct map_info *map = mtd.priv;
    struct lpddr_private *lpddr = map.fldrv_priv;
    let mut chipnum: c_int = adr >> lpddr.chipshift;
    unsigned long ofs, last_end = 0;
    struct flchip *chip = &lpddr.chips[chipnum];
    let mut ret: c_int = 0;
    if (!map.virt)
    return -EINVAL;
// ofs: offset within the first chip that the first read should start
    ofs = adr - (chipnum << lpddr.chipshift);
// mtdbuf = (void *)map->virt + chip->start + ofs;
    while (len) {
    unsigned long thislen;
    if (chipnum >= lpddr.numchips)
    break;
// We cannot point across chips that are virtually disjoint
    if (!last_end)
    last_end = chip.start;
#[no_mangle]
pub unsafe extern "C" fn if(last_end: chip->start !=) -> else {
    else if (chip.start != last_end)
    break;
    if ((len + ofs - 1) >> lpddr.chipshift)
    thislen = (1UL << lpddr.chipshift) - ofs;
    else
    thislen = len;
// get the chip
    mutex_lock(&chip.mutex);
    ret = get_chip(map, chip, FL_POINT);
    mutex_unlock(&chip.mutex);
    if (ret)
    break;
    chip.state = FL_POINT;
    chip.ref_point_counter++;
// retlen += thislen;
    len -= thislen;
    ofs = 0;
    last_end += 1UL << lpddr.chipshift;
    chipnum++;
    chip = &lpddr.chips[chipnum];
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpddr_unpoint(mtd: *mut mtd_info, adr: loff_t, len: usize) -> c_int {
    static int lpddr_unpoint (struct mtd_info *mtd, loff_t adr, size_t len)
    {
    struct map_info *map = mtd.priv;
    struct lpddr_private *lpddr = map.fldrv_priv;
    let mut chipnum: c_int = adr >> lpddr.chipshift, err = 0;
    unsigned long ofs;
// ofs: offset within the first chip that the first read should start
    ofs = adr - (chipnum << lpddr.chipshift);
    while (len) {
    unsigned long thislen;
    struct flchip *chip;
    chip = &lpddr.chips[chipnum];
    if (chipnum >= lpddr.numchips)
    break;
    if ((len + ofs - 1) >> lpddr.chipshift)
    thislen = (1UL << lpddr.chipshift) - ofs;
    else
    thislen = len;
    mutex_lock(&chip.mutex);
    if (chip.state == FL_POINT) {
    chip.ref_point_counter--;
    if (chip.ref_point_counter == 0)
    chip.state = FL_READY;
    } else {
    printk(KERN_WARNING "%s: Warning: unpoint called on non"
    "pointed region\n", map.name);
    err = -EINVAL;
    }
    put_chip(map, chip);
    mutex_unlock(&chip.mutex);
    len -= thislen;
    ofs = 0;
    chipnum++;
    }
    return err;
    }
    static int lpddr_write_buffers(struct mtd_info *mtd, loff_t to, size_t len,
    size_t *retlen, const u_char *buf)
    {
    struct kvec vec;
    vec.iov_base = (void *) buf;
    vec.iov_len = len;
    return lpddr_writev(mtd, &vec, 1, to, retlen);
    }
    static int lpddr_writev(struct mtd_info *mtd, const struct kvec *vecs,
    unsigned long count, loff_t to, size_t *retlen)
    {
    struct map_info *map = mtd.priv;
    struct lpddr_private *lpddr = map.fldrv_priv;
    let mut ret: c_int = 0;
    int chipnum;
    unsigned long ofs, vec_seek, i;
    let mut wbufsize: c_int = 1 << lpddr.qinfo.BufSizeShift;
    let mut len: usize = 0;
    for (i = 0; i < count; i++)
    len += vecs[i].iov_len;
    if (!len)
    return 0;
    chipnum = to >> lpddr.chipshift;
    ofs = to;
    vec_seek = 0;
    do {
// We must not cross write block boundaries
    let mut size: c_int = wbufsize - (ofs & (wbufsize-1));
    if (size > len)
    size = len;
    ret = do_write_buffer(map, &lpddr.chips[chipnum],
    ofs, &vecs, &vec_seek, size);
    if (ret)
    return ret;
    ofs += size;
    (*retlen) += size;
    len -= size;
// Be nice and reschedule with the chip in a usable
// state for other processes
    cond_resched();
    } while (len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpddr_erase(mtd: *mut mtd_info, instr: *mut erase_info) -> c_int {
    static int lpddr_erase(struct mtd_info *mtd, struct erase_info *instr)
    {
    unsigned long ofs, len;
    int ret;
    struct map_info *map = mtd.priv;
    struct lpddr_private *lpddr = map.fldrv_priv;
    let mut size: c_int = 1 << lpddr.qinfo.UniformBlockSizeShift;
    ofs = instr.addr;
    len = instr.len;
    while (len > 0) {
    ret = do_erase_oneblock(mtd, ofs);
    if (ret)
    return ret;
    ofs += size;
    len -= size;
    }
    return 0;
    }
pub const DO_XXLOCK_LOCK: c_int = 1;
pub const DO_XXLOCK_UNLOCK: c_int = 2;
#[no_mangle]
unsafe extern "C" fn do_xxlock(mtd: *mut mtd_info, adr: loff_t, len: u32, thunk: c_int) -> c_int {
    static int do_xxlock(struct mtd_info *mtd, loff_t adr, uint32_t len, int thunk)
    {
    let mut ret: c_int = 0;
    struct map_info *map = mtd.priv;
    struct lpddr_private *lpddr = map.fldrv_priv;
    let mut chipnum: c_int = adr >> lpddr.chipshift;
    struct flchip *chip = &lpddr.chips[chipnum];
    mutex_lock(&chip.mutex);
    ret = get_chip(map, chip, FL_LOCKING);
    if (ret) {
    mutex_unlock(&chip.mutex);
    return ret;
    }
    if (thunk == DO_XXLOCK_LOCK) {
    send_pfow_command(map, LPDDR_LOCK_BLOCK, adr, adr + len, core::ptr::null_mut());
    chip.state = FL_LOCKING;
    } else if (thunk == DO_XXLOCK_UNLOCK) {
    send_pfow_command(map, LPDDR_UNLOCK_BLOCK, adr, adr + len, core::ptr::null_mut());
    chip.state = FL_UNLOCKING;
    } else
    BUG();
    ret = wait_for_ready(map, chip, 1);
    if (ret)	{
    printk(KERN_ERR "%s: block unlock error status %d\n",
    map.name, ret);
    goto out;
    }
    out:	put_chip(map, chip);
    mutex_unlock(&chip.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lpddr_lock(mtd: *mut mtd_info, ofs: loff_t, len: u64) -> c_int {
    static int lpddr_lock(struct mtd_info *mtd, loff_t ofs, uint64_t len)
    {
    return do_xxlock(mtd, ofs, len, DO_XXLOCK_LOCK);
    }
#[no_mangle]
unsafe extern "C" fn lpddr_unlock(mtd: *mut mtd_info, ofs: loff_t, len: u64) -> c_int {
    static int lpddr_unlock(struct mtd_info *mtd, loff_t ofs, uint64_t len)
    {
    return do_xxlock(mtd, ofs, len, DO_XXLOCK_UNLOCK);
    }
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Alexey Korolev <akorolev@infradead.org>");
    MODULE_DESCRIPTION("MTD driver for LPDDR flash chips");
