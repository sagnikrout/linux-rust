//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/parsers/scpart.c
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
// drivers/mtd/scpart.c: Sercomm Partition Parser
//
// Copyright (C) 2018 NOGUCHI Hiroshi
// Copyright (C) 2022 Mikhail Zhilkin
//

pub const ID_ALREADY_FOUND: c_uint = 0xffffffffUL;
pub const MAP_OFFS_IN_BLK: c_uint = 0x800;
pub const MAP_MIRROR_NUM: c_int = 2;
    static const char sc_part_magic[] = {
    'S', 'C', 'F', 'L', 'M', 'A', 'P', 'O', 'K', '\0',
    };

// assumes that all fields are set by CPU native endian
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sc_part_desc {
    pub part_id: u32,
    pub part_offs: u32,
    pub part_bytes: u32,
}

#[no_mangle]
unsafe extern "C" fn scpart_desc_is_valid(pdesc: *mut sc_part_desc) -> u32 {
    static uint32_t scpart_desc_is_valid(struct sc_part_desc *pdesc)
    {
    return ((pdesc.part_id != 0xffffffffUL) &&
    (pdesc.part_offs != 0xffffffffUL) &&
    (pdesc.part_bytes != 0xffffffffUL));
    }
    static int scpart_scan_partmap(struct mtd_info *master, loff_t partmap_offs,
    struct sc_part_desc **ppdesc)
    {
    let mut cnt: c_int = 0;
    let mut res: c_int = 0;
    int res2;
    uint32_t offs;
    size_t retlen;
    struct sc_part_desc *pdesc = core::ptr::null_mut();
    struct sc_part_desc *tmpdesc;
    uint8_t *buf;
    buf = kzalloc(master.erasesize, GFP_KERNEL);
    if (!buf) {
    res = -ENOMEM;
    goto out;
    }
    res2 = mtd_read(master, partmap_offs, master.erasesize, &retlen, buf);
    if (res2 || retlen != master.erasesize) {
    res = -EIO;
    goto free;
    }
    for (offs = MAP_OFFS_IN_BLK;
    offs < master.erasesize - sizeof(*tmpdesc);
    offs += sizeof(*tmpdesc)) {
    tmpdesc = (struct sc_part_desc *)&buf[offs];
    if (!scpart_desc_is_valid(tmpdesc))
    break;
    cnt++;
    }
    if (cnt > 0) {
    let mut bytes: c_int = cnt * sizeof(*pdesc);
    pdesc = kzalloc_objs(*pdesc, cnt);
    if (!pdesc) {
    res = -ENOMEM;
    goto free;
    }
    memcpy(pdesc, &(buf[MAP_OFFS_IN_BLK]), bytes);
// ppdesc = pdesc;
    res = cnt;
    }
    free:
    kfree(buf);
    out:
    return res;
    }
    static int scpart_find_partmap(struct mtd_info *master,
    struct sc_part_desc **ppdesc)
    {
    let mut magic_found: c_int = 0;
    let mut res: c_int = 0;
    int res2;
    let mut offs: loff_t = 0;
    size_t retlen;
    uint8_t rdbuf[PART_MAGIC_LEN];
    while ((magic_found < MAP_MIRROR_NUM) &&
    (offs < master.size) &&
    !mtd_block_isbad(master, offs)) {
    res2 = mtd_read(master, offs, PART_MAGIC_LEN, &retlen, rdbuf);
    if (res2 || retlen != PART_MAGIC_LEN) {
    res = -EIO;
    goto out;
    }
    if (!memcmp(rdbuf, sc_part_magic, PART_MAGIC_LEN)) {
    pr_debug("Signature found at 0x%llx\n", offs);
    magic_found++;
    res = scpart_scan_partmap(master, offs, ppdesc);
    if (res > 0)
    goto out;
    }
    offs += master.erasesize;
    }
    out:
    if (res > 0)
    pr_info("Valid 'SC PART MAP' (%d partitions) found at 0x%llx\n", res, offs);
    else
    pr_info("No valid 'SC PART MAP' was found\n");
    return res;
    }
    static int scpart_parse(struct mtd_info *master,
    const struct mtd_partition **pparts,
    struct mtd_part_parser_data *data)
    {
    const char *partname;
    int n;
    int nr_scparts;
    let mut nr_parts: c_int = 0;
    let mut res: c_int = 0;
    struct sc_part_desc *scpart_map = core::ptr::null_mut();
    struct mtd_partition *parts = core::ptr::null_mut();
    struct device_node *mtd_node;
    struct device_node *ofpart_node;
    struct device_node *pp;
    mtd_node = mtd_get_of_node(master);
    if (!mtd_node) {
    res = -ENOENT;
    goto out;
    }
    ofpart_node = of_get_child_by_name(mtd_node, "partitions");
    if (!ofpart_node) {
    pr_info("%s: 'partitions' subnode not found on %pOF.\n",
    master.name, mtd_node);
    res = -ENOENT;
    goto out;
    }
    nr_scparts = scpart_find_partmap(master, &scpart_map);
    if (nr_scparts <= 0) {
    pr_info("No any partitions was found in 'SC PART MAP'.\n");
    res = -ENOENT;
    goto free;
    }
    parts = kzalloc_objs(*parts, of_get_child_count(ofpart_node));
    if (!parts) {
    res = -ENOMEM;
    goto free;
    }
    for_each_child_of_node(ofpart_node, pp) {
    u32 scpart_id;
    if (of_property_read_u32(pp, "sercomm,scpart-id", &scpart_id))
    continue;
    for (n = 0 ; n < nr_scparts ; n++)
    if ((scpart_map[n].part_id != ID_ALREADY_FOUND) &&
    (scpart_id == scpart_map[n].part_id))
    break;
    if (n >= nr_scparts)
// not match
    continue;
// add the partition found in OF into MTD partition array
    parts[nr_parts].offset = scpart_map[n].part_offs;
    parts[nr_parts].size = scpart_map[n].part_bytes;
    parts[nr_parts].of_node = pp;
    if (!of_property_read_string(pp, "label", &partname))
    parts[nr_parts].name = partname;
    if (of_property_read_bool(pp, "read-only"))
    parts[nr_parts].mask_flags |= MTD_WRITEABLE;
    if (of_property_read_bool(pp, "lock"))
    parts[nr_parts].mask_flags |= MTD_POWERUP_LOCK;
// mark as 'done'
    scpart_map[n].part_id = ID_ALREADY_FOUND;
    nr_parts++;
    }
    if (nr_parts > 0) {
// pparts = parts;
    res = nr_parts;
    } else
    pr_info("No partition in OF matches partition ID with 'SC PART MAP'.\n");
    of_node_put(pp);
    free:
    of_node_put(ofpart_node);
    kfree(scpart_map);
    if (res <= 0)
    kfree(parts);
    out:
    return res;
    }
    static const struct of_device_id scpart_parser_of_match_table[] = {
    { .compatible = "sercomm,sc-partitions" },
    {},
    };
    MODULE_DEVICE_TABLE(of, scpart_parser_of_match_table);
    static struct mtd_part_parser scpart_parser = {
    .parse_fn = scpart_parse,
    .name = "scpart",
    .of_match_table = scpart_parser_of_match_table,
    };
    module_mtd_part_parser(scpart_parser);
// mtd parsers will request the module by parser name
    MODULE_ALIAS("scpart");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("NOGUCHI Hiroshi <drvlabo@gmail.com>");
    MODULE_AUTHOR("Mikhail Zhilkin <csharper2005@gmail.com>");
    MODULE_DESCRIPTION("Sercomm partition parser");
