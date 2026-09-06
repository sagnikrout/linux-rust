//! Automatically rewritten from C to Rust
//! Source: fs/udf/partition.c
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
// partition.c
//
// PURPOSE
// Partition handling routines for the OSTA-UDF(tm) filesystem.
//
// COPYRIGHT
// (C) 1998-2001 Ben Fennema
//
// HISTORY
//
// 12/06/98 blf  Created file.
//

    uint32_t udf_get_pblock(struct super_block *sb, uint32_t block,
    uint16_t partition, uint32_t offset)
    {
    struct udf_sb_info *sbi = UDF_SB(sb);
    struct udf_part_map *map;
    if (partition >= sbi.s_partitions) {
    udf_debug("block=%u, partition=%u, offset=%u: invalid partition\n",
    block, partition, offset);
    return 0xFFFFFFFF;
    }
    map = &sbi.s_partmaps[partition];
    if (map.s_partition_func)
    return map.s_partition_func(sb, block, partition, offset);
    else
    return map.s_partition_root + block + offset;
    }
    uint32_t udf_get_pblock_virt15(struct super_block *sb, uint32_t block,
    uint16_t partition, uint32_t offset)
    {
    struct buffer_head *bh = core::ptr::null_mut();
    uint32_t newblock;
    uint32_t index;
    uint32_t loc;
    struct udf_sb_info *sbi = UDF_SB(sb);
    struct udf_part_map *map;
    struct udf_virtual_data *vdata;
    struct udf_inode_info *iinfo = UDF_I(sbi.s_vat_inode);
    int err;
    map = &sbi.s_partmaps[partition];
    vdata = &map.s_type_specific.s_virtual;
    if (block >= vdata.s_num_entries) {
    udf_debug("Trying to access block beyond end of VAT (%u max %u)\n",
    block, vdata.s_num_entries);
    return 0xFFFFFFFF;
    }
    if (iinfo.i_alloc_type == ICBTAG_FLAG_AD_IN_ICB) {
    loc = le32_to_cpu(((__le32 *)(iinfo.i_data +
    vdata.s_start_offset))[block]);
    goto translate;
    }
    index = (sb.s_blocksize - vdata.s_start_offset) / sizeof(uint32_t);
    if (block >= index) {
    block -= index;
    newblock = 1 + (block / (sb.s_blocksize / sizeof(uint32_t)));
    index = block % (sb.s_blocksize / sizeof(uint32_t));
    } else {
    newblock = 0;
    index = vdata.s_start_offset / sizeof(uint32_t) + block;
    }
    bh = udf_bread(sbi.s_vat_inode, newblock, 0, &err);
    if (!bh) {
    udf_debug("get_pblock(UDF_VIRTUAL_MAP:%p,%u,%u)\n",
    sb, block, partition);
    return 0xFFFFFFFF;
    }
    loc = le32_to_cpu(((__le32 *)bh.b_data)[index]);
    brelse(bh);
    translate:
    if (iinfo.i_location.partitionReferenceNum == partition) {
    udf_debug("recursive call to udf_get_pblock!\n");
    return 0xFFFFFFFF;
    }
    return udf_get_pblock(sb, loc,
    iinfo.i_location.partitionReferenceNum,
    offset);
    }
    inline uint32_t udf_get_pblock_virt20(struct super_block *sb, uint32_t block,
    uint16_t partition, uint32_t offset)
    {
    return udf_get_pblock_virt15(sb, block, partition, offset);
    }
    uint32_t udf_get_pblock_spar15(struct super_block *sb, uint32_t block,
    uint16_t partition, uint32_t offset)
    {
    int i;
    struct sparingTable *st = core::ptr::null_mut();
    struct udf_sb_info *sbi = UDF_SB(sb);
    struct udf_part_map *map;
    uint32_t packet;
    struct udf_sparing_data *sdata;
    map = &sbi.s_partmaps[partition];
    sdata = &map.s_type_specific.s_sparing;
    packet = (block + offset) & ~(sdata.s_packet_len - 1);
    for (i = 0; i < 4; i++) {
    if (sdata.s_spar_map[i] != core::ptr::null_mut()) {
    st = (struct sparingTable *)
    sdata.s_spar_map[i].b_data;
    break;
    }
    }
    if (st) {
    for (i = 0; i < le16_to_cpu(st.reallocationTableLen); i++) {
    struct sparingEntry *entry = &st.mapEntry[i];
    let mut origLoc: u32 = le32_to_cpu(entry.origLocation);
    if (origLoc >= 0xFFFFFFF0)
    break;
#[no_mangle]
pub unsafe extern "C" fn if(packet: origLoc ==) -> else {
    else if (origLoc == packet)
    return le32_to_cpu(entry.mappedLocation) +
    ((block + offset) &
    (sdata.s_packet_len - 1));
#[no_mangle]
pub unsafe extern "C" fn if(packet: origLoc >) -> else {
    else if (origLoc > packet)
    break;
    }
    }
    return map.s_partition_root + block + offset;
    }
#[no_mangle]
pub unsafe extern "C" fn udf_relocate_blocks(sb: *mut super_block, old_block: c_long, new_block: *mut c_long) -> c_int {
    int udf_relocate_blocks(struct super_block *sb, long old_block, long *new_block)
    {
    struct udf_sparing_data *sdata;
    struct sparingTable *st = core::ptr::null_mut();
    struct sparingEntry mapEntry;
    uint32_t packet;
    int i, j, k, l;
    struct udf_sb_info *sbi = UDF_SB(sb);
    u16 reallocationTableLen;
    struct buffer_head *bh;
    let mut ret: c_int = 0;
    mutex_lock(&sbi.s_alloc_mutex);
    for (i = 0; i < sbi.s_partitions; i++) {
    struct udf_part_map *map = &sbi.s_partmaps[i];
    if (old_block > map.s_partition_root &&
    old_block < map.s_partition_root + map.s_partition_len) {
    sdata = &map.s_type_specific.s_sparing;
    packet = (old_block - map.s_partition_root) &
    ~(sdata.s_packet_len - 1);
    for (j = 0; j < 4; j++)
    if (sdata.s_spar_map[j] != core::ptr::null_mut()) {
    st = (struct sparingTable *)
    sdata.s_spar_map[j].b_data;
    break;
    }
    if (!st) {
    ret = 1;
    goto out;
    }
    reallocationTableLen =
    le16_to_cpu(st.reallocationTableLen);
    for (k = 0; k < reallocationTableLen; k++) {
    struct sparingEntry *entry = &st.mapEntry[k];
    let mut origLoc: u32 = le32_to_cpu(entry.origLocation);
    if (origLoc == 0xFFFFFFFF) {
    for (; j < 4; j++) {
    int len;
    bh = sdata.s_spar_map[j];
    if (!bh)
    continue;
    st = (struct sparingTable *)
    bh.b_data;
    entry.origLocation =
    cpu_to_le32(packet);
    len =
    sizeof(struct sparingTable) +
    reallocationTableLen *
    sizeof(struct sparingEntry);
    udf_update_tag((char *)st, len);
    mark_buffer_dirty(bh);
    }
// new_block = le32_to_cpu(
    entry.mappedLocation) +
    ((old_block -
    map.s_partition_root) &
    (sdata.s_packet_len - 1));
    ret = 0;
    goto out;
    } else if (origLoc == packet) {
// new_block = le32_to_cpu(
    entry.mappedLocation) +
    ((old_block -
    map.s_partition_root) &
    (sdata.s_packet_len - 1));
    ret = 0;
    goto out;
    } else if (origLoc > packet)
    break;
    }
    for (l = k; l < reallocationTableLen; l++) {
    struct sparingEntry *entry = &st.mapEntry[l];
    let mut origLoc: u32 = le32_to_cpu(entry.origLocation);
    if (origLoc != 0xFFFFFFFF)
    continue;
    for (; j < 4; j++) {
    bh = sdata.s_spar_map[j];
    if (!bh)
    continue;
    st = (struct sparingTable *)bh.b_data;
    mapEntry = st.mapEntry[l];
    mapEntry.origLocation =
    cpu_to_le32(packet);
    memmove(&st.mapEntry[k + 1],
    &st.mapEntry[k],
    (l - k) *
    sizeof(struct sparingEntry));
    st.mapEntry[k] = mapEntry;
    udf_update_tag((char *)st,
    sizeof(struct sparingTable) +
    reallocationTableLen *
    sizeof(struct sparingEntry));
    mark_buffer_dirty(bh);
    }
// new_block =
    le32_to_cpu(
    st.mapEntry[k].mappedLocation) +
    ((old_block - map.s_partition_root) &
    (sdata.s_packet_len - 1));
    ret = 0;
    goto out;
    }
    ret = 1;
    goto out;
    } /* if old_block */
    }
    if (i == sbi.s_partitions) {
// outside of partitions
// for now, fail =)
    ret = 1;
    }
    out:
    mutex_unlock(&sbi.s_alloc_mutex);
    return ret;
    }
    static uint32_t udf_try_read_meta(struct inode *inode, uint32_t block,
    uint16_t partition, uint32_t offset)
    {
    struct super_block *sb = inode.i_sb;
    struct udf_part_map *map;
    struct kernel_lb_addr eloc;
    uint32_t elen;
    sector_t ext_offset;
    let mut epos: extent_position = {};
    uint32_t phyblock;
    int8_t etype;
    let mut err: c_int = 0;
    err = inode_bmap(inode, block, &epos, &eloc, &elen, &ext_offset, &etype);
    if (err <= 0 || etype != (EXT_RECORDED_ALLOCATED >> 30))
    phyblock = 0xFFFFFFFF;
    else {
    map = &UDF_SB(sb).s_partmaps[partition];
// map to sparable/physical partition desc
    phyblock = udf_get_pblock(sb, eloc.logicalBlockNum,
    map.s_type_specific.s_metadata.s_phys_partition_ref,
    ext_offset + offset);
    }
    brelse(epos.bh);
    return phyblock;
    }
    uint32_t udf_get_pblock_meta25(struct super_block *sb, uint32_t block,
    uint16_t partition, uint32_t offset)
    {
    struct udf_sb_info *sbi = UDF_SB(sb);
    struct udf_part_map *map;
    struct udf_meta_data *mdata;
    uint32_t retblk;
    struct inode *inode;
    udf_debug("READING from METADATA\n");
    map = &sbi.s_partmaps[partition];
    mdata = &map.s_type_specific.s_metadata;
    inode = mdata.s_metadata_fe ? : mdata.s_mirror_fe;
    if (!inode)
    return 0xFFFFFFFF;
    retblk = udf_try_read_meta(inode, block, partition, offset);
    if (retblk == 0xFFFFFFFF && mdata.s_metadata_fe) {
    udf_warn(sb, "error reading from METADATA, trying to read from MIRROR\n");
    if (!(mdata.s_flags & MF_MIRROR_FE_LOADED)) {
    mdata.s_mirror_fe = udf_find_metadata_inode_efe(sb,
    mdata.s_mirror_file_loc,
    mdata.s_phys_partition_ref);
    if (IS_ERR(mdata.s_mirror_fe))
    mdata.s_mirror_fe = core::ptr::null_mut();
    mdata.s_flags |= MF_MIRROR_FE_LOADED;
    }
    inode = mdata.s_mirror_fe;
    if (!inode)
    return 0xFFFFFFFF;
    retblk = udf_try_read_meta(inode, block, partition, offset);
    }
    return retblk;
    }
