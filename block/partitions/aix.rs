//! Automatically rewritten from C to Rust
//! Source: block/partitions/aix.c
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


// SPDX-License-Identifier: GPL-2.0
//
// fs/partitions/aix.c
//
// Copyright (C) 2012-2013 Philippe De Muyter <phdm@macqel.be>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lvm_rec {
    pub /: *mut *mut char lvm_id[4]; / "_LVM",
    pub reserved4: [c_char; 16],
    pub lvmarea_len: __be32,
    pub vgda_len: __be32,
    pub vgda_psn: [__be32; 2],
    pub reserved36: [c_char; 10],
    pub /: *mut *mut __be16 pp_size; / log2(pp_size),
    pub reserved46: [c_char; 12],
    pub version: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgda {
    pub secs: __be32,
    pub usec: __be32,
    pub reserved8: [c_char; 16],
    pub numlvs: __be16,
    pub maxlvs: __be16,
    pub pp_size: __be16,
    pub numpvs: __be16,
    pub total_vgdas: __be16,
    pub vgda_size: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lvd {
    pub lv_ix: __be16,
    pub res2: __be16,
    pub res4: __be16,
    pub maxsize: __be16,
    pub lv_state: __be16,
    pub mirror: __be16,
    pub mirror_policy: __be16,
    pub num_lps: __be16,
    pub res10: [__be16; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lvname {
    pub name: [c_char; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppe {
    pub lv_ix: __be16,
    pub res2: c_ushort,
    pub res4: c_ushort,
    pub lp_ix: __be16,
    pub res8: [c_ushort; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvd {
    pub reserved0: [c_char; 16],
    pub pp_count: __be16,
    pub reserved18: [c_char; 2],
    pub psn_part1: __be32,
    pub reserved24: [c_char; 8],
    pub ppe: [ppe; 1016],
}

pub const LVM_MAXLVS: c_int = 256;
//
// read_lba(): Read bytes from disk, starting at given LBA
// @state
// @lba
// @buffer
// @count
//
// Description:  Reads @count bytes from @state->disk into @buffer.
// Returns number of bytes read on success, 0 on error.
//
    static size_t read_lba(struct parsed_partitions *state, u64 lba, u8 *buffer,
    size_t count)
    {
    let mut totalreadcount: usize = 0;
    if (!buffer || lba + count / 512 > get_capacity(state.disk) - 1ULL)
    return 0;
    while (count) {
    let mut copied: c_int = 512;
    Sector sect;
    unsigned char *data = read_part_sector(state, lba++, &sect);
    if (!data)
    break;
    if (copied > count)
    copied = count;
    memcpy(buffer, data, copied);
    put_dev_sector(sect);
    buffer += copied;
    totalreadcount += copied;
    count -= copied;
    }
    return totalreadcount;
    }
//
// alloc_pvd(): reads physical volume descriptor
// @state
// @lba
//
// Description: Returns pvd on success,  NULL on error.
// Allocates space for pvd and fill it with disk blocks at @lba
// Notes: remember to free pvd when you're done!
//
    static struct pvd *alloc_pvd(struct parsed_partitions *state, u32 lba)
    {
    let mut count: usize = sizeof(struct pvd);
    struct pvd *p;
    p = kmalloc(count, GFP_KERNEL);
    if (!p)
    return core::ptr::null_mut();
    if (read_lba(state, lba, (u8 *) p, count) < count) {
    kfree(p);
    return core::ptr::null_mut();
    }
    return p;
    }
//
// alloc_lvn(): reads logical volume names
// @state
// @lba
//
// Description: Returns lvn on success,  NULL on error.
// Allocates space for lvn and fill it with disk blocks at @lba
// Notes: remember to free lvn when you're done!
//
    static struct lvname *alloc_lvn(struct parsed_partitions *state, u32 lba)
    {
    let mut count: usize = sizeof(struct lvname) * LVM_MAXLVS;
    struct lvname *p;
    p = kmalloc(count, GFP_KERNEL);
    if (!p)
    return core::ptr::null_mut();
    if (read_lba(state, lba, (u8 *) p, count) < count) {
    kfree(p);
    return core::ptr::null_mut();
    }
    return p;
    }
#[no_mangle]
pub unsafe extern "C" fn aix_partition(state: *mut parsed_partitions) -> c_int {
    int aix_partition(struct parsed_partitions *state)
    {
    let mut ret: c_int = 0;
    Sector sect;
    unsigned char *d;
    u32 pp_bytes_size;
    let mut pp_blocks_size: u32 = 0;
    let mut vgda_sector: u32 = 0;
    let mut vgda_len: u32 = 0;
    let mut numlvs: c_int = 0;
    struct pvd *pvd = core::ptr::null_mut();
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lv_info {
    pub pps_per_lv: c_ushort,
    pub pps_found: c_ushort,
    pub lv_is_contiguous: c_uchar,
    pub lvip: *mut },
    pub NULL: *mut *mut lvname n =,
    pub &sect): d = read_part_sector(state, 7,,
    if (d) {
    pub )d: *mut *mut lvm_rec p = (lvm_rec,
    pub be16_to_cpu(p->version): u16 lvm_version =,
    if (lvm_version == 1) {
    pub be16_to_cpu(p->pp_size): int pp_size_log2 =,
    pub pp_size_log2: pp_bytes_size = 1 <<,
    pub 512: pp_blocks_size = pp_bytes_size /,
    seq_buf_printf(&state.pp_buf,
    " AIX LVM header version %u found\n",
    pub be32_to_cpu(p->vgda_len): vgda_len =,
    pub be32_to_cpu(p->vgda_psn[0]): vgda_sector =,
    } else {
    seq_buf_printf(&state.pp_buf,
    " unsupported AIX LVM version %d found\n",
    }
    }
    if (vgda_sector && (d = read_part_sector(state, vgda_sector, &sect))) {
    pub )d: *mut *mut vgda p = (vgda,
    pub be16_to_cpu(p->numlvs): numlvs =,
    }
    pub state->limit): lvip = kzalloc_objs(struct lv_info,,
    if (!lvip)
    pub 0: return,
    if (numlvs && (d = read_part_sector(state, vgda_sector + 1, &sect))) {
    pub )d: *mut *mut lvd p = (lvd,
    pub i: c_int,
    pub 33): n = alloc_lvn(state, vgda_sector + vgda_len -,
    if (n) {
    pub 0: int foundlvs =,
//
// The lvd array was read as a single sector; only the
// struct lvd entries that fit in it are valid.  Bound the
// scan so an on-disk numlvs larger than that cannot walk
// the read buffer out of bounds.
//
    pub &&: for (i = 0; foundlvs < numlvs && i < state->limit,
    pub {: i < SECTOR_SIZE / (int)sizeof(struct lvd); i++),
    pub be16_to_cpu(p[i].num_lps): lvip[i].pps_per_lv =,
    if (lvip[i].pps_per_lv)
    pub 1: foundlvs +=,
    }
// pvd loops depend on n[].name and lvip[].pps_per_lv
    pub 17): pvd = alloc_pvd(state, vgda_sector +,
    }
    }
    if (pvd) {
    pub be16_to_cpu(pvd->pp_count): int numpps =,
    pub be32_to_cpu(pvd->psn_part1): int psn_part1 =,
    pub i: c_int,
    pub -1: int cur_lv_ix =,
    pub 1: int next_lp_ix =,
    pub lp_ix: c_int,
//
// pvd was read into a fixed-size struct pvd whose ppe[] array
// holds ARRAY_SIZE(pvd->ppe) entries.  pp_count is an
// unvalidated on-disk __be16, so clamp the scan to the array
// size to avoid walking past the allocation.
//
    if (numpps > ARRAY_SIZE(pvd.ppe))
    pub ARRAY_SIZE(pvd->ppe): numpps =,
    pub {: for (i = 0; i < numpps; i += 1),
    pub i: *mut *mut ppe p = pvd->ppe +,
    pub lv_ix: c_uint,
    pub be16_to_cpu(p->lp_ix): lp_ix =,
    if (!lp_ix) {
    pub 1: next_lp_ix =,
    }
    pub 1: lv_ix = be16_to_cpu(p->lv_ix) -,
    if (lv_ix >= state.limit) {
    pub -1: cur_lv_ix =,
    }
    pub 1: lvip[lv_ix].pps_found +=,
    if (lp_ix == 1) {
    pub lv_ix: cur_lv_ix =,
    pub 1: next_lp_ix =,
    } else if (lv_ix != cur_lv_ix || lp_ix != next_lp_ix) {
    pub 1: next_lp_ix =,
    }
    if (lp_ix == lvip[lv_ix].pps_per_lv) {
    put_partition(state, lv_ix + 1,
    (i + 1 - lp_ix) * pp_blocks_size + psn_part1,
    pub pp_blocks_size): *mut *mut lvip[lv_ix].pps_per_lv,
    seq_buf_printf(&state.pp_buf, " <%s>\n",
    pub 1: lvip[lv_ix].lv_is_contiguous =,
    pub 1: ret =,
    pub 1: next_lp_ix =,
    } else
    pub 1: next_lp_ix +=,
    }
    pub 1): for (i = 0; i < state->limit; i +=,
    if (lvip[i].pps_found && !lvip[i].lv_is_contiguous) {
    pub char: char tmp[sizeof(n[i].name) + 1]; // null,
    pub n[i].name): snprintf(tmp, sizeof(tmp), "%s",,
    pr_warn("partition %s (%u pp's found) is "
    "not contiguous\n",
    pub lvip[i].pps_found): tmp,,
    }
    }
    pub ret: return,
    }
