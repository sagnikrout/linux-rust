//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/mtd.h
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
// Copyright © 1999-2010 David Woodhouse <dwmw2@infradead.org> et al.
//

//
// If the erase fails, fail_addr might indicate exactly which block failed. If
// fail_addr = MTD_FAIL_ADDR_UNKNOWN, the failure was not at the device level
// or was not specific to any particular block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erase_info {
    pub addr: u64,
    pub len: u64,
    pub fail_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_erase_region_info {
    pub /: *mut *mut uint64_t offset; / At which this region starts, from the beginning of the MTD,
    pub /: *mut *mut uint32_t erasesize; / For this region,
    pub /: *mut *mut uint32_t numblocks; / Number of blocks of erasesize in this region,
    pub /: *mut *mut *mut unsigned long lockmap; / If keeping bitmap of locks,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_req_stats {
    pub uncorrectable_errors: c_uint,
    pub corrected_bitflips: c_uint,
    pub max_bitflips: c_uint,
}

//
// struct mtd_oob_ops - oob operation operands
// @mode:	operation mode
//
// @len:	number of data bytes to write/read
//
// @retlen:	number of data bytes written/read
//
// @ooblen:	number of oob bytes to write/read
// @oobretlen:	number of oob bytes written/read
// @ooboffs:	offset of oob data in the oob area (only relevant when
// mode = MTD_OPS_PLACE_OOB or MTD_OPS_RAW)
// @datbuf:	data buffer - if NULL only oob data are read/written
// @oobbuf:	oob data buffer
//
// Note, some MTD drivers do not allow you to write more than one OOB area at
// one go. If you try to do that on such an MTD device, -EINVAL will be
// returned. If you want to make your implementation portable on all kind of MTD
// devices you should split the write request into several sub-requests when the
// request crosses a page boundary.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_oob_ops {
    pub mode: c_uint,
    pub len: usize,
    pub retlen: usize,
    pub ooblen: usize,
    pub oobretlen: usize,
    pub ooboffs: u32,
    pub datbuf: *mut u8,
    pub oobbuf: *mut u8,
    pub stats: *mut mtd_req_stats,
}

//
// struct mtd_oob_region - oob region definition
// @offset: region offset
// @length: region length
//
// This structure describes a region of the OOB area, and is used
// to retrieve ECC or free bytes sections.
// Each section is defined by an offset within the OOB area and a
// length.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_oob_region {
    pub offset: u32,
    pub length: u32,
}

//
// struct mtd_ooblayout_ops - NAND OOB layout operations
// @ecc: function returning an ECC region in the OOB area.
// Should return -ERANGE if %section exceeds the total number of
// ECC sections.
// @free: function returning a free region in the OOB area.
// Should return -ERANGE if %section exceeds the total number of
// free sections.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_ooblayout_ops {
    pub oobecc): *mut mtd_oob_region,
    pub oobfree): *mut mtd_oob_region,
}

//
// struct mtd_pairing_info - page pairing information
//
// @pair: pair id
// @group: group id
//
// The term "pair" is used here, even though TLC NANDs might group pages by 3
// (3 bits in a single cell). A pair should regroup all pages that are sharing
// the same cell. Pairs are then indexed in ascending order.
//
// @group is defining the position of a page in a given pair. It can also be
// seen as the bit position in the cell: page attached to bit 0 belongs to
// group 0, page attached to bit 1 belongs to group 1, etc.
//
// Example:
// The H27UCG8T2BTR-BC datasheet describes the following pairing scheme:
//
// group-0		group-1
//
// pair-0	page-0		page-4
// pair-1	page-1		page-5
// pair-2	page-2		page-8
// ...
// pair-127	page-251	page-255
//
// Note that the "group" and "pair" terms were extracted from Samsung and
// Hynix datasheets, and might be referenced under other names in other
// datasheets (Micron is describing this concept as "shared pages").
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_pairing_info {
    pub pair: c_int,
    pub group: c_int,
}

//
// struct mtd_pairing_scheme - page pairing scheme description
//
// @ngroups: number of groups. Should be related to the number of bits
// per cell.
// @get_info: converts a write-unit (page number within an erase block) into
// mtd_pairing information (pair + group). This function should
// fill the info parameter based on the wunit index or return
// -EINVAL if the wunit parameter is invalid.
// @get_wunit: converts pairing information into a write-unit (page) number.
// This function should return the wunit index pointed by the
// pairing information described in the info argument. It should
// return -EINVAL, if there's no wunit corresponding to the
// passed pairing information.
//
// See mtd_pairing_info documentation for a detailed explanation of the
// pair and group concepts.
//
// The mtd_pairing_scheme structure provides a generic solution to represent
// NAND page pairing scheme. Instead of exposing two big tables to do the
// write-unit <-> (pair + group) conversions, we ask the MTD drivers to
// implement the ->get_info() and ->get_wunit() functions.
//
// MTD users will then be able to query these information by using the
// mtd_pairing_info_to_wunit() and mtd_wunit_to_pairing_info() helpers.
//
// @ngroups is here to help MTD users iterating over all the pages in a
// given pair. This value can be retrieved by MTD users using the
// mtd_pairing_groups() helper.
//
// Examples are given in the mtd_pairing_info_to_wunit() and
// mtd_wunit_to_pairing_info() documentation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_pairing_scheme {
    pub ngroups: c_int,
    pub info): *mut mtd_pairing_info,
    pub info): *const mtd_pairing_info,
}

//
// struct mtd_debug_info - debugging information for an MTD device.
//
// @dfs_dir: direntry object of the MTD device debugfs directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_debug_info {
    pub dfs_dir: *mut dentry,
}

//
// struct mtd_part - MTD partition specific fields
//
// @node: list node used to add an MTD partition to the parent partition list
// @offset: offset of the partition relatively to the parent offset
// @size: partition size. Should be equal to mtd->size unless
// MTD_SLC_ON_MLC_EMULATION is set
// @flags: original flags (before the mtdpart logic decided to tweak them based
// on flash constraints, like eraseblock/pagesize alignment)
//
// This struct is embedded in mtd_info and contains partition-specific
// properties/fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_part {
    pub node: list_head,
    pub offset: u64,
    pub size: u64,
    pub flags: u32,
}

//
// struct mtd_master - MTD master specific fields
//
// @partitions_lock: lock protecting accesses to the partition list. Protects
// not only the master partition list, but also all
// sub-partitions.
// @suspended: set to 1 when the device is suspended, 0 otherwise
//
// This struct is embedded in mtd_info and contains master-specific
// properties/fields. The master is the root MTD device from the MTD partition
// point of view.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_master {
    pub partitions_lock: mutex,
    pub chrdev_lock: mutex,
    pub 1: unsigned int suspended :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_info {
    pub type: u_char,
    pub flags: u32,
    pub MTD: uint64_t size; // Total size of the,
// "Major" erase size for the device. Naïve users may take this
// to be the only erase size available, or may use the more detailed
// information below if they desire
//
    pub erasesize: u32,
// Minimal writable flash unit size. In case of NOR flash it is 1 (even
// though individual bits can be cleared), in case of NAND flash it is
// one NAND page (or half, or one-fourths of it), in case of ECC-ed NOR
// it is of ECC block size, etc. It is illegal to have writesize = 0.
// Any driver registering a struct mtd_info must ensure a writesize of
// 1 or larger.
//
    pub writesize: u32,
//
// Size of the write buffer used by the MTD. MTD devices having a write
// buffer can write multiple writesize chunks at a time. E.g. while
// writing 4 * writesize bytes to a device with 2 * writesize bytes
// buffer the MTD driver can (but doesn't have to) do 2 writesize
// operations, but not 4. Currently, all NANDs have writebufsize
// equivalent to writesize (NAND page size). Some NOR flashes do have
// writebufsize greater than writesize.
//
    pub writebufsize: u32,
    pub 16): uint32_t oobsize; // Amount of OOB data per block (e.g.,
    pub block: uint32_t oobavail; // Available OOB bytes per,
//
// If erasesize is a power of 2 then the shift is stored in
// erasesize_shift otherwise erasesize_shift is zero. Ditto writesize.
//
    pub erasesize_shift: c_uint,
    pub writesize_shift: c_uint,
// Masks based on erasesize_shift and writesize_shift
    pub erasesize_mask: c_uint,
    pub writesize_mask: c_uint,
//
// read ops return -EUCLEAN if max number of bitflips corrected on any
// one region comprising an ecc step equals or exceeds this value.
// Settable by driver, else defaults to ecc_strength.  User can override
// in sysfs.  N.B. The meaning of the -EUCLEAN return code has changed;
// see Documentation/ABI/testing/sysfs-class-mtd for more detail.
//
    pub bitflip_threshold: c_uint,
// Kernel-only stuff starts here.
    pub name: *const c_char,
    pub index: c_int,
// OOB layout description
    pub ooblayout: *const mtd_ooblayout_ops,
// NAND pairing scheme, only provided for MLC/TLC NANDs
    pub pairing: *const mtd_pairing_scheme,
// the ecc step size.
    pub ecc_step_size: c_uint,
// max number of correctible bit errors per ecc step
    pub ecc_strength: c_uint,
// Data for variable erase regions. If numeraseregions is zero,
// it means that the whole device has erasesize as given above.
//
    pub numeraseregions: c_int,
    pub eraseregions: *mut mtd_erase_region_info,
//
// Do not call via these pointers, use corresponding mtd_*()
// wrappers instead.
//
    pub instr): *mut *mut *mut int (_erase) (struct mtd_info mtd, struct erase_info,
    pub phys): *mut *mut *mut *mut size_t retlen, void virt, resource_size_t,
    pub len): *mut *mut *mut int (_unpoint) (struct mtd_info mtd, loff_t from, size_t,
    pub buf): *mut *mut size_t retlen, u_char,
    pub buf): *const *const size_t retlen, u_char,
    pub buf): *const *const size_t retlen, u_char,
    pub ops): *mut mtd_oob_ops,
    pub ops): *mut mtd_oob_ops,
    pub buf): *mut *mut size_t retlen, struct otp_info,
    pub buf): *mut *mut size_t len, size_t retlen, u_char,
    pub buf): *mut *mut size_t retlen, struct otp_info,
    pub buf): *mut *mut size_t len, size_t retlen, u_char,
    pub buf): *const u_char,
    pub len): usize,
    pub len): usize,
    pub retlen): *mut unsigned long count, loff_t to, size_t,
    pub mtd): *mut *mut void (_sync) (struct mtd_info,
    pub len): *mut *mut *mut int (_lock) (struct mtd_info mtd, loff_t ofs, uint64_t,
    pub len): *mut *mut *mut int (_unlock) (struct mtd_info mtd, loff_t ofs, uint64_t,
    pub len): *mut *mut *mut int (_is_locked) (struct mtd_info mtd, loff_t ofs, uint64_t,
    pub ofs): *mut *mut *mut int (_block_isreserved) (struct mtd_info mtd, loff_t,
    pub ofs): *mut *mut *mut int (_block_isbad) (struct mtd_info mtd, loff_t,
    pub ofs): *mut *mut *mut int (_block_markbad) (struct mtd_info mtd, loff_t,
    pub len): *mut *mut *mut int (_max_bad_blocks) (struct mtd_info mtd, loff_t ofs, size_t,
    pub mtd): *mut *mut int (_suspend) (struct mtd_info,
    pub mtd): *mut *mut void (_resume) (struct mtd_info,
    pub mtd): *mut *mut void (_reboot) (struct mtd_info,
//
// If the driver is something smart, like UBI, it may need to maintain
// its own reference counting. The below functions are only for driver.
//
    pub mtd): *mut *mut int (_get_device) (struct mtd_info,
    pub mtd): *mut *mut void (_put_device) (struct mtd_info,
//
// flag indicates a panic write, low level drivers can take appropriate
// action if required to ensure writes go through
//
    pub oops_panic_write: bool,
    pub /: *mut *mut notifier_block reboot_notifier; / default mode before reboot,
// ECC status information
    pub ecc_stats: mtd_ecc_stats,
// Subpage shift (NAND)
    pub subpage_sft: c_int,
    pub priv: *mut c_void,
    pub owner: *mut module,
    pub dev: device,
    pub refcnt: kref,
    pub dbg: mtd_debug_info,
    pub nvmem: *mut nvmem_device,
    pub otp_user_nvmem: *mut nvmem_device,
    pub otp_factory_nvmem: *mut nvmem_device,
//
// Parent device from the MTD partition point of view.
//
// MTD masters do not have any parent, MTD partitions do. The parent
// MTD device can itself be a partition.
//
    pub parent: *mut mtd_info,
// List of partitions attached to this MTD device
    pub partitions: list_head,
    pub part: mtd_part,
    pub master: mtd_master,
}

extern "C" {
    pub fn mtd_ooblayout_count_freebytes(mtd: *mut mtd_info) -> c_int;
}
extern "C" {
    pub fn mtd_ooblayout_count_eccbytes(mtd: *mut mtd_info) -> c_int;
}
extern "C" {
    pub fn dev_of_node(_arg: &mtd->dev) -> return;
}
extern "C" {
    pub fn mtd_pairing_groups(mtd: *mut mtd_info) -> c_int;
}
extern "C" {
    pub fn mtd_erase(mtd: *mut mtd_info, instr: *mut erase_info) -> c_int;
}
extern "C" {
    pub fn mtd_unpoint(mtd: *mut mtd_info, from: loff_t, len: usize) -> c_int;
}
extern "C" {
    pub fn mtd_read_oob(mtd: *mut mtd_info, from: loff_t, ops: *mut mtd_oob_ops) -> c_int;
}
extern "C" {
    pub fn mtd_write_oob(mtd: *mut mtd_info, to: loff_t, ops: *mut mtd_oob_ops) -> c_int;
}
extern "C" {
    pub fn mtd_lock_user_prot_reg(mtd: *mut mtd_info, from: loff_t, len: usize) -> c_int;
}
extern "C" {
    pub fn mtd_erase_user_prot_reg(mtd: *mut mtd_info, from: loff_t, len: usize) -> c_int;
}
extern "C" {
    pub fn mtd_lock(mtd: *mut mtd_info, ofs: loff_t, len: u64) -> c_int;
}
extern "C" {
    pub fn mtd_unlock(mtd: *mut mtd_info, ofs: loff_t, len: u64) -> c_int;
}
extern "C" {
    pub fn mtd_is_locked(mtd: *mut mtd_info, ofs: loff_t, len: u64) -> c_int;
}
extern "C" {
    pub fn mtd_block_isreserved(mtd: *mut mtd_info, ofs: loff_t) -> c_int;
}
extern "C" {
    pub fn mtd_block_isbad(mtd: *mut mtd_info, ofs: loff_t) -> c_int;
}
extern "C" {
    pub fn mtd_block_markbad(mtd: *mut mtd_info, ofs: loff_t) -> c_int;
}
extern "C" {
    pub fn do_div(_arg: sz, _arg: mtd->erasesize) -> return;
}
//
// mtd_align_erase_req - Adjust an erase request to align things on eraseblock
// boundaries.
// @mtd: the MTD device this erase request applies on
// @req: the erase request to adjust
//
// This function will adjust @req->addr and @req->len to align them on
// @mtd->erasesize. Of course we expect @mtd->erasesize to be != 0.
//
extern "C" {
    pub fn do_div(_arg: sz, _arg: mtd->writesize) -> return;
}
extern "C" {
    pub fn mtd_div_by_ws(_arg: mtd_mod_by_eb(offs, _arg: mtd), _arg: mtd) -> return;
}
// Kernel-side ioctl definitions

extern "C" {
    pub fn mtd_device_unregister(master: *mut mtd_info) -> c_int;
}
extern "C" {
    pub fn __get_mtd_device(mtd: *mut mtd_info) -> c_int;
}
extern "C" {
    pub fn __put_mtd_device(mtd: *mut mtd_info);
}
extern "C" {
    pub fn put_mtd_device(mtd: *mut mtd_info);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_notifier {
    pub mtd): *mut *mut void (add)(struct mtd_info,
    pub mtd): *mut *mut void (remove)(struct mtd_info,
    pub list: list_head,
}

extern "C" {
    pub fn register_mtd_user(new: *mut mtd_notifier);
}
extern "C" {
    pub fn unregister_mtd_user(old: *mut mtd_notifier) -> c_int;
}
extern "C" {
    pub fn mtd_is_bitflip(mtd_is_eccerr(err: err) ||) -> return;
}
extern "C" {
    pub fn mtd_mmap_capabilities(mtd: *mut mtd_info) -> unsigned;
}

extern "C" {
    pub fn mtd_check_expert_analysis_mode() -> bool;
}

