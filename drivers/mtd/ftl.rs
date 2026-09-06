//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/ftl.c
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


// This version ported to the Linux-MTD system by dwmw2@infradead.org
//
// Fixes: Arnaldo Carvalho de Melo <acme@conectiva.com.br>
// - fixes some leaks on failure in build_maps and ftl_notify_add, cleanups
//
// Based on:
//
// ======================================================================
    A Flash Translation Layer memory card driver
    This driver implements a disk-like block device driver with an
    apparent block size of 512 bytes for flash memory cards.
    ftl_cs.c 1.62 2000/02/01 00:59:04
    The contents of this file are subject to the Mozilla Public
    License Version 1.1 (the "License"); you may not use this file
    except in compliance with the License. You may obtain a copy of
    the License at http://www.mozilla.org/MPL/
    Software distributed under the License is distributed on an "AS
    IS" basis, WITHOUT WARRANTY OF ANY KIND, either express or
    implied. See the License for the specific language governing
    rights and limitations under the License.
    The initial developer of the original code is David A. Hinds
    <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
    are Copyright © 1999 David A. Hinds.  All Rights Reserved.
    Alternatively, the contents of this file may be used under the
    terms of the GNU General Public License version 2 (the "GPL"), in
    which case the provisions of the GPL are applicable instead of the
    above.  If you wish to allow the use of your version of this file
    only under the terms of the GPL and not to allow others to use
    your version of this file under the MPL, indicate your decision
    by deleting the provisions above and replace them with the notice
    and other provisions required by the GPL.  If you do not delete
    the provisions above, a recipient may use your version of this
    file under either the MPL or the GPL.
    LEGAL NOTE: The FTL format is patented by M-Systems.  They have
    granted a license for its use with PCMCIA devices:
    "M-Systems grants a royalty-free, non-exclusive license under
    any presently existing M-Systems intellectual property rights
    necessary for the design and development of FTL-compatible
    drivers, file systems and utilities using the data formats with
    PCMCIA PC Cards as described in the PCMCIA Flash Translation
    Layer (FTL) Specification."
    Use of the FTL format for non-PCMCIA applications may be an
    infringement of these patents.  For additional information,
    contact M-Systems directly. M-Systems since acquired by Sandisk.
    ======================================================================*/

// #define PSYCHO_DEBUG

// ====================================================================
// Parameters that can be set with 'insmod'
    let mut shuffle_freq: static int = 50;
    module_param(shuffle_freq, int, 0);
// ====================================================================
// Major device # for FTL device

pub const FTL_MAJOR: c_int = 44;

// ====================================================================
// Maximum number of separate memory devices we'll allow
pub const MAX_DEV: c_int = 4;
// Maximum number of regions per device
pub const MAX_REGION: c_int = 4;
// Maximum number of partitions in an FTL region
pub const PART_BITS: c_int = 4;
// Maximum number of outstanding erase requests per socket
pub const MAX_ERASE: c_int = 8;
// Sector size -- shouldn't need to change
pub const SECTOR_SIZE: c_int = 512;
// Each memory region corresponds to a minor device
    typedef struct partition_t {
    struct mtd_blktrans_dev mbd;
    uint32_t		state;
    uint32_t		*VirtualBlockMap;
    uint32_t		FreeTotal;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eun_info_t {
    pub Offset: u32,
    pub EraseCount: u32,
    pub Free: u32,
    pub Deleted: u32,
    pub EUNInfo: *mut },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfer_info_t {
    pub Offset: u32,
    pub EraseCount: u32,
    pub state: u16,
    pub XferInfo: *mut },
    pub bam_index: u16,
    pub bam_cache: *mut u32,
    pub DataUnits: u16,
    pub BlocksPerUnit: u32,
    pub header: erase_unit_header_t,
    pub partition_t: },
// Partition state flags
pub const FTL_FORMATTED: c_uint = 0x01;
// Transfer unit states
pub const XFER_UNKNOWN: c_uint = 0x00;
pub const XFER_ERASING: c_uint = 0x01;
pub const XFER_ERASED: c_uint = 0x02;
pub const XFER_PREPARED: c_uint = 0x03;
pub const XFER_FAILED: c_uint = 0x04;
// ======================================================================
    Scan_header() checks to see if a memory region contains an FTL
    partition.  build_maps() reads all the erase unit headers, builds
    the erase unit map, and then builds the virtual page map.
    ======================================================================*/
#[no_mangle]
unsafe extern "C" fn scan_header(part: *mut partition_t) -> c_int {
    static int scan_header(partition_t *part)
    {
    pub header: erase_unit_header_t,
    pub max_offset: loff_t offset,,
    pub ret: usize,
    pub err: c_int,
    pub 0: part->header.FormattedSize =,
    pub (0x100000<part->mbd.mtd->size)?0x100000:part->mbd.mtd->size: max_offset =,
// Search first megabyte for a valid FTL header
    pub 0: for (offset =,
    pub max_offset: (offset + sizeof(header)) <,
    offset += part.mbd.mtd.erasesize ? : 0x2000) {
    err = mtd_read(part.mbd.mtd, offset, sizeof(header), &ret,
    pub )&header): *mut (unsigned char,
    if (err)
    pub err: return,
    pub break: if (strcmp(header.DataOrgTuple+3, "FTL100") == 0),
    }
    if (offset == max_offset) {
    pub found.\n"): printk(KERN_NOTICE "ftl_cs: FTL header not,
    pub -ENOENT: return,
    }
    if (header.BlockSize != 9 ||
    (header.EraseUnitSize < 10) || (header.EraseUnitSize > 31) ||
    (header.NumTransferUnits >= le16_to_cpu(header.NumEraseUnits))) {
    pub corrupt!\n"): printk(KERN_NOTICE "ftl_cs: FTL header,
    pub -1: return,
    }
    if ((1 << header.EraseUnitSize) != part.mbd.mtd.erasesize) {
    printk(KERN_NOTICE "ftl: FTL EraseUnitSize %x != MTD erasesize %x\n",
    pub header.EraseUnitSize,part->mbd.mtd->erasesize): 1 <<,
    pub -1: return,
    }
    pub header: part->header =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn build_maps(part: *mut partition_t) -> c_int {
    static int build_maps(partition_t *part)
    {
    pub header: erase_unit_header_t,
    pub i: uint16_t xvalid, xtrans,,
    pub j: unsigned blocks,,
    pub -1: int hdr_ok, ret =,
    pub retval: isize,
    pub offset: loff_t,
// Set up erase unit maps
    part.DataUnits = le16_to_cpu(part.header.NumEraseUnits) -
    pub part->DataUnits): part->EUNInfo = kmalloc_objs(struct eun_info_t,,
    if (!part.EUNInfo)
    pub out: goto,
    pub i++): for (i = 0; i < part->DataUnits;,
    pub 0xffffffff: part->EUNInfo[i].Offset =,
    part.XferInfo =
    pub part->header.NumTransferUnits): kmalloc_objs(struct xfer_info_t,,
    if (!part.XferInfo)
    pub out_EUNInfo: goto,
    pub 0: xvalid = xtrans =,
    pub {: for (i = 0; i < le16_to_cpu(part->header.NumEraseUnits); i++),
    offset = ((i + le16_to_cpu(part.header.FirstPhysicalEUN))
    pub part->header.EraseUnitSize): <<,
    ret = mtd_read(part.mbd.mtd, offset, sizeof(header), &retval,
    pub )&header): *mut (unsigned char,
    if (ret)
    pub out_XferInfo: goto,
    pub -1: ret =,
// Is this a transfer partition?
    pub 0): hdr_ok = (strcmp(header.DataOrgTuple+3, "FTL100") ==,
    if (hdr_ok && (le16_to_cpu(header.LogicalEUN) < part.DataUnits) &&
    (part.EUNInfo[le16_to_cpu(header.LogicalEUN)].Offset == 0xffffffff)) {
    pub offset: part->EUNInfo[le16_to_cpu(header.LogicalEUN)].Offset =,
    part.EUNInfo[le16_to_cpu(header.LogicalEUN)].EraseCount =
    } else {
    if (xtrans == part.header.NumTransferUnits) {
    printk(KERN_NOTICE "ftl_cs: format error: too many "
    pub units!\n"): "transfer,
    pub out_XferInfo: goto,
    }
    if (hdr_ok && (le16_to_cpu(header.LogicalEUN) == 0xffff)) {
    pub XFER_PREPARED: part->XferInfo[xtrans].state =,
    pub le32_to_cpu(header.EraseCount): part->XferInfo[xtrans].EraseCount =,
    } else {
    pub XFER_UNKNOWN: part->XferInfo[xtrans].state =,
// Pick anything reasonable for the erase count
    part.XferInfo[xtrans].EraseCount =
    }
    pub offset: part->XferInfo[xtrans].Offset =,
    }
    }
// Check for format trouble
    pub part->header: header =,
    if ((xtrans != header.NumTransferUnits) ||
    (xvalid+xtrans != le16_to_cpu(header.NumEraseUnits))) {
    printk(KERN_NOTICE "ftl_cs: format error: erase units "
    pub up!\n"): "don't add,
    pub out_XferInfo: goto,
    }
// Set up virtual page map
    pub header.BlockSize: blocks = le32_to_cpu(header.FormattedSize) >>,
    pub sizeof(uint32_t)): part->VirtualBlockMap = vmalloc_array(blocks,,
    if (!part.VirtualBlockMap)
    pub out_XferInfo: goto,
    pub sizeof(uint32_t)): *mut *mut memset(part->VirtualBlockMap, 0xff, blocks,
    pub header.BlockSize: part->BlocksPerUnit = (1 << header.EraseUnitSize) >>,
    part.bam_cache = kmalloc_array(part.BlocksPerUnit, sizeof(uint32_t),
    if (!part.bam_cache)
    pub out_VirtualBlockMap: goto,
    pub 0xffff: part->bam_index =,
    pub 0: part->FreeTotal =,
    pub {: for (i = 0; i < part->DataUnits; i++),
    pub 0: part->EUNInfo[i].Free =,
    pub 0: part->EUNInfo[i].Deleted =,
    pub le32_to_cpu(header.BAMOffset): offset = part->EUNInfo[i].Offset +,
    ret = mtd_read(part.mbd.mtd, offset,
    part.BlocksPerUnit * sizeof(uint32_t), &retval,
    pub )part->bam_cache): *mut (unsigned char,
    if (ret)
    pub out_bam_cache: goto,
    pub {: for (j = 0; j < part->BlocksPerUnit; j++),
    if (BLOCK_FREE(le32_to_cpu(part.bam_cache[j]))) {
    } else if ((BLOCK_TYPE(le32_to_cpu(part.bam_cache[j])) == BLOCK_DATA) &&
    (BLOCK_NUMBER(le32_to_cpu(part.bam_cache[j])) < blocks))
    part.VirtualBlockMap[BLOCK_NUMBER(le32_to_cpu(part.bam_cache[j]))] =
    pub header.BlockSize): (i << header.EraseUnitSize) + (j <<,
#[no_mangle]
pub unsafe extern "C" fn if(_arg: BLOCK_DELETED(le32_to_cpu(part->bam_cache[j]))) -> else {
    else if (BLOCK_DELETED(le32_to_cpu(part.bam_cache[j])))
    }
    }
    pub 0: ret =,
    pub out: goto,
    out_bam_cache:
    out_VirtualBlockMap:
    out_XferInfo:
    out_EUNInfo:
    out:
    pub ret: return,
    } /* build_maps */
// ======================================================================
    Erase_xfer() schedules an asynchronous erase operation for a
    transfer unit.
    ======================================================================*/
    static int erase_xfer(partition_t *part,
    uint16_t xfernum)
    {
    pub ret: c_int,
    pub xfer: *mut xfer_info_t,
    pub erase: *mut erase_info,
    pub &part->XferInfo[xfernum]: xfer =,
    pub xfer->Offset): pr_debug("ftl_cs: erasing xfer unit at 0x%x\n",,
    pub XFER_ERASING: xfer->state =,
// Is there a free erase slot? Always in MTD.
    pub erase_info): erase=kmalloc_obj(struct,
    if (!erase)
    pub -ENOMEM: return,
    pub xfer->Offset: erase->addr =,
    pub part->header.EraseUnitSize: erase->len = 1ULL <<,
    pub erase): ret = mtd_erase(part->mbd.mtd,,
    if (!ret) {
    pub XFER_ERASED: xfer->state =,
    } else {
    pub XFER_FAILED: xfer->state =,
    pub ret): pr_notice("ftl_cs: erase failed: err = %d\n",,
    }
    pub ret: return,
    } /* erase_xfer */
// ======================================================================
    Prepare_xfer() takes a freshly erased transfer unit and gives
    it an appropriate header.
    ======================================================================*/
#[no_mangle]
unsafe extern "C" fn prepare_xfer(part: *mut partition_t, i: c_int) -> c_int {
    static int prepare_xfer(partition_t *part, int i)
    {
    pub header: erase_unit_header_t,
    pub xfer: *mut xfer_info_t,
    pub ret: int nbam,,
    pub ctl: u32,
    pub retlen: isize,
    pub offset: loff_t,
    pub &part->XferInfo[i]: xfer =,
    pub XFER_FAILED: xfer->state =,
    pub xfer->Offset): pr_debug("ftl_cs: preparing xfer unit at 0x%x\n",,
// Write the transfer unit header
    pub part->header: header =,
    pub cpu_to_le16(0xffff): header.LogicalEUN =,
    pub cpu_to_le32(xfer->EraseCount): header.EraseCount =,
    ret = mtd_write(part.mbd.mtd, xfer.Offset, sizeof(header), &retlen,
    pub )&header): *mut (u_char,
    if (ret) {
    pub ret: return,
    }
// Write the BAM stub
    nbam = DIV_ROUND_UP(part.BlocksPerUnit * sizeof(uint32_t) +
    pub SECTOR_SIZE): le32_to_cpu(part->header.BAMOffset),,
    pub le32_to_cpu(part->header.BAMOffset): offset = xfer->Offset +,
    pub cpu_to_le32(BLOCK_CONTROL): ctl =,
    pub {: for (i = 0; i < nbam; i++, offset += sizeof(uint32_t)),
    ret = mtd_write(part.mbd.mtd, offset, sizeof(uint32_t), &retlen,
    pub )&ctl): *mut (u_char,
    if (ret)
    pub ret: return,
    }
    pub XFER_PREPARED: xfer->state =,
    pub 0: return,
    } /* prepare_xfer */
// ======================================================================
    Copy_erase_unit() takes a full erase block and a transfer unit,
    copies everything to the transfer unit, then swaps the block
    pointers.
    All data blocks are copied to the corresponding blocks in the
    target unit, so the virtual block map does not need to be
    updated.
    ======================================================================*/
    static int copy_erase_unit(partition_t *part, uint16_t srcunit,
    uint16_t xferunit)
    {
    pub buf: [u_char; SECTOR_SIZE],
    pub eun: *mut eun_info_t,
    pub xfer: *mut xfer_info_t,
    pub i: uint32_t src, dest, free,,
    pub unit: u16,
    pub ret: c_int,
    pub retlen: isize,
    pub offset: loff_t,
    pub cpu_to_le16(srcunit): uint16_t srcunitswap =,
    pub &part->EUNInfo[srcunit]: eun =,
    pub &part->XferInfo[xferunit]: xfer =,
    pr_debug("ftl_cs: copying block 0x%x to 0x%x\n",
    pub xfer->Offset): eun->Offset,,
// Read current BAM
    if (part.bam_index != srcunit) {
    pub le32_to_cpu(part->header.BAMOffset): offset = eun->Offset +,
    ret = mtd_read(part.mbd.mtd, offset,
    part.BlocksPerUnit * sizeof(uint32_t), &retlen,
    pub )(part->bam_cache)): *mut (u_char,
// mark the cache bad, in case we get an error later
    pub 0xffff: part->bam_index =,
    if (ret) {
    pub copy_erase_unit()!\n"): printk( KERN_WARNING "ftl: Failed to read BAM cache in,
    pub ret: return,
    }
    }
// Write the LogicalEUN for the transfer unit
    pub XFER_UNKNOWN: xfer->state =,
    pub /: *mut *mut offset = xfer->Offset + 20; / Bad!,
    pub cpu_to_le16(0x7fff): unit =,
    ret = mtd_write(part.mbd.mtd, offset, sizeof(uint16_t), &retlen,
    pub )&unit): *mut (u_char,
    if (ret) {
    pub copy_erase_unit()!\n"): printk( KERN_WARNING "ftl: Failed to write back to BAM cache in,
    pub ret: return,
    }
// Copy all data blocks from source unit to transfer unit
    pub xfer->Offset: src = eun->Offset; dest =,
    pub 0: free =,
    pub 0: ret =,
    pub {: for (i = 0; i < part->BlocksPerUnit; i++),
    switch (BLOCK_TYPE(le32_to_cpu(part.bam_cache[i]))) {
    case BLOCK_CONTROL:
// This gets updated later
    case BLOCK_DATA:
    case BLOCK_REPLACEMENT:
    ret = mtd_read(part.mbd.mtd, src, SECTOR_SIZE, &retlen,
    pub )buf): *mut (u_char,
    if (ret) {
    pub copy_erase_unit\n"): printk(KERN_WARNING "ftl: Error reading old xfer unit in,
    pub ret: return,
    }
    ret = mtd_write(part.mbd.mtd, dest, SECTOR_SIZE, &retlen,
    pub )buf): *mut (u_char,
    if (ret)  {
    pub copy_erase_unit\n"): printk(KERN_WARNING "ftl: Error writing new xfer unit in,
    pub ret: return,
    }
    default:
// All other blocks must be free
    pub cpu_to_le32(0xffffffff): part->bam_cache[i] =,
    }
    pub SECTOR_SIZE: src +=,
    pub SECTOR_SIZE: dest +=,
    }
// Write the BAM to the transfer unit
    ret = mtd_write(part.mbd.mtd,
    xfer.Offset + le32_to_cpu(part.header.BAMOffset),
    part.BlocksPerUnit * sizeof(int32_t),
    &retlen,
    pub )part->bam_cache): *mut (u_char,
    if (ret) {
    pub copy_erase_unit\n"): printk( KERN_WARNING "ftl: Error writing BAM in,
    pub ret: return,
    }
// All clear? Then update the LogicalEUN again
    ret = mtd_write(part.mbd.mtd, xfer.Offset + 20, sizeof(uint16_t),
    pub )&srcunitswap): *mut &retlen, (u_char,
    if (ret) {
    pub copy_erase_unit\n"): printk(KERN_WARNING "ftl: Error writing new LogicalEUN in,
    pub ret: return,
    }
// Update the maps and usage stats
    pub eun->EraseCount): swap(xfer->EraseCount,,
    pub eun->Offset): swap(xfer->Offset,,
    pub eun->Free: part->FreeTotal -=,
    pub free: part->FreeTotal +=,
    pub free: eun->Free =,
    pub 0: eun->Deleted =,
// Now, the cache should be valid for the new block
    pub srcunit: part->bam_index =,
    pub 0: return,
    } /* copy_erase_unit */
// ======================================================================
    reclaim_block() picks a full erase unit and a transfer unit and
    then calls copy_erase_unit() to copy one to the other.  Then, it
    schedules an erase on the expired block.
    What's a good way to decide which transfer unit and which erase
    unit to use?  Beats me.  My way is to always pick the transfer
    unit with the fewest erases, and usually pick the data unit with
    the most deleted blocks.  But with a small probability, pick the
    oldest data unit instead.  This means that we generally postpone
    the next reclamation as long as possible, but shuffle static
    stuff around a bit for wear leveling.
    ======================================================================*/
#[no_mangle]
unsafe extern "C" fn reclaim_block(part: *mut partition_t) -> c_int {
    static int reclaim_block(partition_t *part)
    {
    pub xfer: uint16_t i, eun,,
    pub best: u32,
    pub ret: int queued,,
    pub space...\n"): pr_debug("ftl_cs: reclaiming,
    pub part->header.NumTransferUnits): pr_debug("NumTransferUnits == %x\n",,
// Pick the least erased transfer unit
    pub 0xffff: best = 0xffffffff; xfer =,
    do {
    pub 0: queued =,
    pub {: for (i = 0; i < part->header.NumTransferUnits; i++),
    pub n=0: c_int,
    if (part.XferInfo[i].state == XFER_UNKNOWN) {
    pub XFER_UNKNOWN\n",i): pr_debug("XferInfo[%d].state ==,
    pub i): erase_xfer(part,,
    }
    if (part.XferInfo[i].state == XFER_ERASING) {
    pub XFER_ERASING\n",i): pr_debug("XferInfo[%d].state ==,
    pub 1: queued =,
    }
#[no_mangle]
pub unsafe extern "C" fn if(XFER_ERASED: part->XferInfo[i].state ==) -> else {
    pub XFER_ERASED\n",i): pr_debug("XferInfo[%d].state ==,
    pub i): prepare_xfer(part,,
    }
    if (part.XferInfo[i].state == XFER_PREPARED) {
    pub XFER_PREPARED\n",i): pr_debug("XferInfo[%d].state ==,
    if (part.XferInfo[i].EraseCount <= best) {
    pub part->XferInfo[i].EraseCount: best =,
    pub i: xfer =,
    }
    }
    if (!n)
    pub part->XferInfo[i].state): pr_debug("XferInfo[%d].state == %x\n",i,,
    }
    if (xfer == 0xffff) {
    if (queued) {
    pr_debug("ftl_cs: waiting for transfer "
    pub prepared...\n"): "unit to be,
    } else {
    pub 0: static int ne =,
    if (++ne < 5)
    printk(KERN_NOTICE "ftl_cs: reclaim failed: no "
    pub units!\n"): "suitable transfer,
    else
    pr_debug("ftl_cs: reclaim failed: no "
    pub units!\n"): "suitable transfer,
    pub -EIO: return,
    }
    }
    pub 0xffff): } while (xfer ==,
    pub 0: eun =,
    if ((jiffies % shuffle_freq) == 0) {
    pub block...\n"): pr_debug("ftl_cs: recycling freshest,
    pub 0xffffffff: best =,
    pub i++): for (i = 0; i < part->DataUnits;,
    if (part.EUNInfo[i].EraseCount <= best) {
    pub part->EUNInfo[i].EraseCount: best =,
    pub i: eun =,
    }
    } else {
    pub 0: best =,
    pub i++): for (i = 0; i < part->DataUnits;,
    if (part.EUNInfo[i].Deleted >= best) {
    pub part->EUNInfo[i].Deleted: best =,
    pub i: eun =,
    }
    if (best == 0) {
    pub 0: static int ne =,
    if (++ne < 5)
    printk(KERN_NOTICE "ftl_cs: reclaim failed: "
    pub blocks!\n"): "no free,
    else
    pr_debug("ftl_cs: reclaim failed: "
    pub blocks!\n"): "no free,
    pub -EIO: return,
    }
    }
    pub xfer): ret = copy_erase_unit(part, eun,,
    if (!ret)
    pub xfer): erase_xfer(part,,
    else
    pub failed!\n"): printk(KERN_NOTICE "ftl_cs: copy_erase_unit,
    pub ret: return,
    } /* reclaim_block */
// ======================================================================
    Find_free() searches for a free block.  If necessary, it updates
    the BAM cache for the erase unit containing the free block.  It
    returns the block index -- the erase unit is just the currently
    cached unit.  If there are no free blocks, it returns 0 -- this
    is never a valid data block because it contains the header.
    ======================================================================*/

#[no_mangle]
unsafe extern "C" fn dump_lists(part: *mut partition_t) {
    static void dump_lists(partition_t *part)
    {
    pub i: c_int,
    pub part->FreeTotal): printk(KERN_DEBUG "ftl_cs: Free total = %d\n",,
    pub i++): for (i = 0; i < part->DataUnits;,
    printk(KERN_DEBUG "ftl_cs:   unit %d: %d phys, %d free, "
    "%d deleted\n", i,
    part.EUNInfo[i].Offset >> part.header.EraseUnitSize,
    pub part->EUNInfo[i].Deleted): part->EUNInfo[i].Free,,
    }

#[no_mangle]
unsafe extern "C" fn find_free(part: *mut partition_t) -> u32 {
    static uint32_t find_free(partition_t *part)
    {
    pub eun: uint16_t stop,,
    pub blk: u32,
    pub retlen: usize,
    pub ret: c_int,
// Find an erase unit with some free space
    pub part->bam_index: stop = (part->bam_index == 0xffff) ? 0 :,
    pub stop: eun =,
    do {
    pub break: if (part->EUNInfo[eun].Free != 0),
// Wrap around at end of table
    pub 0: if (++eun == part->DataUnits) eun =,
    pub stop): } while (eun !=,
    if (part.EUNInfo[eun].Free == 0)
    pub 0: return,
// Is this unit's BAM cached?
    if (eun != part.bam_index) {
// Invalidate cache
    pub 0xffff: part->bam_index =,
    ret = mtd_read(part.mbd.mtd,
    part.EUNInfo[eun].Offset + le32_to_cpu(part.header.BAMOffset),
    part.BlocksPerUnit * sizeof(uint32_t),
    &retlen,
    pub )(part->bam_cache)): *mut (u_char,
    if (ret) {
    pub find_free\n"): printk(KERN_WARNING"ftl: Error reading BAM in,
    pub 0: return,
    }
    pub eun: part->bam_index =,
    }
// Find a free block
    pub blk++): for (blk = 0; blk < part->BlocksPerUnit;,
    pub break: if (BLOCK_FREE(le32_to_cpu(part->bam_cache[blk]))),
    if (blk == part.BlocksPerUnit) {

    pub 0: static int ne =,
    if (++ne == 1)

    pub list!\n"): printk(KERN_NOTICE "ftl_cs: bad free,
    pub 0: return,
    }
    pub eun): pr_debug("ftl_cs: found free block at %d in %d\n", blk,,
    pub blk: return,
    } /* find_free */
// ======================================================================
    Read a series of sectors from an FTL partition.
    ======================================================================*/
    static int ftl_read(partition_t *part, caddr_t buffer,
    u_long sector, u_long nblocks)
    {
    pub bsize: uint32_t log_addr,,
    pub i: u_long,
    pub ret: c_int,
    pub retlen: size_t offset,,
    pr_debug("ftl_cs: ftl_read(0x%p, 0x%lx, %ld)\n",
    pub nblocks): part, sector,,
    if (!(part.state & FTL_FORMATTED)) {
    pub partition\n"): printk(KERN_NOTICE "ftl_cs: bad,
    pub -EIO: return,
    }
    pub part->header.EraseUnitSize: bsize = 1 <<,
    pub {: for (i = 0; i < nblocks; i++),
    if (((sector+i) * SECTOR_SIZE) >= le32_to_cpu(part.header.FormattedSize)) {
    pub offset\n"): printk(KERN_NOTICE "ftl_cs: bad read,
    pub -EIO: return,
    }
    pub part->VirtualBlockMap[sector+i]: log_addr =,
    if (log_addr == 0xffffffff)
    pub SECTOR_SIZE): memset(buffer, 0,,
    else {
    offset = (part.EUNInfo[log_addr / bsize].Offset
    pub bsize)): + (log_addr %,
    ret = mtd_read(part.mbd.mtd, offset, SECTOR_SIZE, &retlen,
    pub )buffer): *mut (u_char,
    if (ret) {
    pub ftl_read()\n"): printk(KERN_WARNING "Error reading MTD device in,
    pub ret: return,
    }
    }
    pub SECTOR_SIZE: buffer +=,
    }
    pub 0: return,
    } /* ftl_read */
// ======================================================================
    Write a series of sectors to an FTL partition
    ======================================================================*/
    static int set_bam_entry(partition_t *part, uint32_t log_addr,
    uint32_t virt_addr)
    {
    pub le_virt_addr: uint32_t bsize, blk,,

    pub old_addr: u32,

    pub eun: u16,
    pub ret: c_int,
    pub offset: size_t retlen,,
    pr_debug("ftl_cs: set_bam_entry(0x%p, 0x%x, 0x%x)\n",
    pub virt_addr): part, log_addr,,
    pub part->header.EraseUnitSize: bsize = 1 <<,
    pub bsize: eun = log_addr /,
    pub SECTOR_SIZE: blk = (log_addr % bsize) /,
    offset = (part.EUNInfo[eun].Offset + blk * sizeof(uint32_t) +

    ret = mtd_read(part.mbd.mtd, offset, sizeof(uint32_t), &retlen,
    pub )&old_addr): *mut (u_char,
    if (ret) {
    pub %d\n",ret): printk(KERN_WARNING"ftl: Error reading old_addr in set_bam_entry:,
    pub ret: return,
    }
    pub le32_to_cpu(old_addr): old_addr =,
    if (((virt_addr == 0xfffffffe) && !BLOCK_FREE(old_addr)) ||
    ((virt_addr == 0) && (BLOCK_TYPE(old_addr) != BLOCK_DATA)) ||
    (!BLOCK_DELETED(virt_addr) && (old_addr != 0xfffffffe))) {
    pub 0: static int ne =,
    if (++ne < 5) {
    pub inconsistency!\n"): printk(KERN_NOTICE "ftl_cs: set_bam_entry(),
    printk(KERN_NOTICE "ftl_cs:   log_addr = 0x%x, old = 0x%x"
    pub virt_addr): ", new = 0x%x\n", log_addr, old_addr,,
    }
    pub -EIO: return,
    }

    pub cpu_to_le32(virt_addr): le_virt_addr =,
    if (part.bam_index == eun) {

    if (le32_to_cpu(part.bam_cache[blk]) != old_addr) {
    pub 0: static int ne =,
    if (++ne < 5) {
    printk(KERN_NOTICE "ftl_cs: set_bam_entry() "
    printk(KERN_NOTICE "ftl_cs:   log_addr = 0x%x, cache"
    " = 0x%x\n",
    pub old_addr): le32_to_cpu(part->bam_cache[blk]),,
    }
    pub -EIO: return,
    }

    pub le_virt_addr: part->bam_cache[blk] =,
    }
    ret = mtd_write(part.mbd.mtd, offset, sizeof(uint32_t), &retlen,
    pub )&le_virt_addr): *mut (u_char,
    if (ret) {
    pub failed!\n"): printk(KERN_NOTICE "ftl_cs: set_bam_entry(),
    printk(KERN_NOTICE "ftl_cs:   log_addr = 0x%x, new = 0x%x\n",
    pub virt_addr): log_addr,,
    }
    pub ret: return,
    } /* set_bam_entry */
    static int ftl_write(partition_t *part, caddr_t buffer,
    u_long sector, u_long nblocks)
    {
    pub blk: uint32_t bsize, log_addr, virt_addr, old_addr,,
    pub i: u_long,
    pub ret: c_int,
    pub offset: size_t retlen,,
    pr_debug("ftl_cs: ftl_write(0x%p, %ld, %ld)\n",
    pub nblocks): part, sector,,
    if (!(part.state & FTL_FORMATTED)) {
    pub partition\n"): printk(KERN_NOTICE "ftl_cs: bad,
    pub -EIO: return,
    }
// See if we need to reclaim space, before we start
    while (part.FreeTotal < nblocks) {
    pub reclaim_block(part): ret =,
    if (ret)
    pub ret: return,
    }
    pub part->header.EraseUnitSize: bsize = 1 <<,
    pub BLOCK_DATA: *mut *mut virt_addr = sector  SECTOR_SIZE |,
    pub {: for (i = 0; i < nblocks; i++),
    if (virt_addr >= le32_to_cpu(part.header.FormattedSize)) {
    pub offset\n"): printk(KERN_NOTICE "ftl_cs: bad write,
    pub -EIO: return,
    }
// Grab a free block
    pub find_free(part): blk =,
    if (blk == 0) {
    pub 0: static int ne =,
    if (++ne < 5)
    printk(KERN_NOTICE "ftl_cs: internal error: "
    pub blocks!\n"): "no free,
    pub -ENOSPC: return,
    }
// Tag the BAM entry, and write the new block
    pub SECTOR_SIZE: *mut *mut *mut log_addr = part->bam_index  bsize + blk,
    if (set_bam_entry(part, log_addr, 0xfffffffe))
    pub -EIO: return,
    offset = (part.EUNInfo[part.bam_index].Offset +
    pub SECTOR_SIZE): *mut *mut blk,
    pub buffer): ret = mtd_write(part->mbd.mtd, offset, SECTOR_SIZE, &retlen,,
    if (ret) {
    pub failed!\n"): printk(KERN_NOTICE "ftl_cs: block write,
    printk(KERN_NOTICE "ftl_cs:   log_addr = 0x%x, virt_addr"
    " = 0x%x, Offset = 0x%zx\n", log_addr, virt_addr,
    pub -EIO: return,
    }
// Only delete the old entry when the new entry is ready
    pub part->VirtualBlockMap[sector+i]: old_addr =,
    if (old_addr != 0xffffffff) {
    pub 0xffffffff: part->VirtualBlockMap[sector+i] =,
    if (set_bam_entry(part, old_addr, 0))
    pub -EIO: return,
    }
// Finally, set up the new pointers
    if (set_bam_entry(part, log_addr, virt_addr))
    pub -EIO: return,
    pub log_addr: part->VirtualBlockMap[sector+i] =,
    pub SECTOR_SIZE: buffer +=,
    pub SECTOR_SIZE: virt_addr +=,
    }
    pub 0: return,
    } /* ftl_write */
#[no_mangle]
unsafe extern "C" fn ftl_getgeo(dev: *mut mtd_blktrans_dev, geo: *mut hd_geometry) -> c_int {
    static int ftl_getgeo(struct mtd_blktrans_dev *dev, struct hd_geometry *geo)
    {
    pub mbd): *mut *mut partition_t part = container_of(dev, struct partition_t,,
    pub sect: u_long,
// Sort of arbitrary: round size down to 4KiB boundary
    pub le32_to_cpu(part->header.FormattedSize)/SECTOR_SIZE: sect =,
    pub 1: geo->heads =,
    pub 8: geo->sectors =,
    pub 3: geo->cylinders = sect >>,
    pub 0: return,
    }
    static int ftl_readsect(struct mtd_blktrans_dev *dev,
    unsigned long block, char *buf)
    {
    pub 1): *mut *mut return ftl_read((void )dev, buf, block,,
    }
    static int ftl_writesect(struct mtd_blktrans_dev *dev,
    unsigned long block, char *buf)
    {
    pub 1): *mut *mut return ftl_write((void )dev, buf, block,,
    }
    static int ftl_discardsect(struct mtd_blktrans_dev *dev,
    unsigned long sector, unsigned nr_sects)
    {
    pub mbd): *mut *mut partition_t part = container_of(dev, struct partition_t,,
    pub part->header.EraseUnitSize: uint32_t bsize = 1 <<,
    pr_debug("FTL erase sector %ld for %d sectors\n",
    pub nr_sects): sector,,
    while (nr_sects) {
    pub part->VirtualBlockMap[sector]: uint32_t old_addr =,
    if (old_addr != 0xffffffff) {
    pub 0xffffffff: part->VirtualBlockMap[sector] =,
    if (set_bam_entry(part, old_addr, 0))
    pub -EIO: return,
    }
    }
    pub 0: return,
    }
// ====================================================================
#[no_mangle]
unsafe extern "C" fn ftl_freepart(part: *mut partition_t) {
    static void ftl_freepart(partition_t *part)
    {
    pub NULL: part->VirtualBlockMap =,
    pub NULL: part->EUNInfo =,
    pub NULL: part->XferInfo =,
    pub NULL: part->bam_cache =,
    } /* ftl_freepart */
#[no_mangle]
unsafe extern "C" fn ftl_add_mtd(tr: *mut mtd_blktrans_ops, mtd: *mut mtd_info) {
    static void ftl_add_mtd(struct mtd_blktrans_ops *tr, struct mtd_info *mtd)
    {
    pub partition: *mut partition_t,
    pub kzalloc_obj(partition_t): partition =,
    if (!partition) {
    printk(KERN_WARNING "No memory to scan for FTL on %s\n",
    }
    pub mtd: partition->mbd.mtd =,
    if ((scan_header(partition) == 0) &&
    (build_maps(partition) == 0)) {
    pub FTL_FORMATTED: partition->state =,

    printk(KERN_INFO "ftl_cs: opening %d KiB FTL partition\n",
    pub 10): le32_to_cpu(partition->header.FormattedSize) >>,

    pub 9: partition->mbd.size = le32_to_cpu(partition->header.FormattedSize) >>,
    pub tr: partition->mbd.tr =,
    pub -1: partition->mbd.devnum =,
    if (!add_mtd_blktrans_dev(&partition.mbd))
    }
    }
#[no_mangle]
unsafe extern "C" fn ftl_remove_dev(dev: *mut mtd_blktrans_dev) {
    static void ftl_remove_dev(struct mtd_blktrans_dev *dev)
    {
    pub )dev): *mut ftl_freepart((partition_t,
    }
    static struct mtd_blktrans_ops ftl_tr = {
    .name		= "ftl",
    .major		= FTL_MAJOR,
    .part_bits	= PART_BITS,
    .blksize 	= SECTOR_SIZE,
    .readsect	= ftl_readsect,
    .writesect	= ftl_writesect,
    .discard	= ftl_discardsect,
    .getgeo		= ftl_getgeo,
    .add_mtd	= ftl_add_mtd,
    .remove_dev	= ftl_remove_dev,
    .owner		= THIS_MODULE,
}

    module_mtd_blktrans(ftl_tr);
    MODULE_LICENSE("Dual MPL/GPL");
    MODULE_AUTHOR("David Hinds <dahinds@users.sourceforge.net>");
    MODULE_DESCRIPTION("Support code for Flash Translation Layer, used on PCMCIA devices");
