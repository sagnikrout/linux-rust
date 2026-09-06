//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/parsers/redboot.c
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
// Parse RedBoot-style Flash Image System (FIS) tables and
// produce a Linux partition array to match.
//
// Copyright © 2001      Red Hat UK Limited
// Copyright © 2001-2010 David Woodhouse <dwmw2@infradead.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fis_image_desc {
    pub name: unsigned char name[16]; // Null terminated,
    pub image: u32 flash_base; // Address within FLASH of,
    pub executes: u32 mem_base; // Address in memory where it,
    pub image: u32 size; // Length of,
    pub point: u32 entry_point; // Execution entry,
    pub data: u32 data_length; // Length of actual,
    pub sizeof(u32))]: *mut *mut unsigned char _pad[256 - (16 + 7,
    pub descriptor: u32 desc_cksum; // Checksum over image,
    pub data: u32 file_cksum; // Checksum over image,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fis_list {
    pub img: *mut fis_image_desc,
    pub next: *mut fis_list,
}

    let mut directory: static int = CONFIG_MTD_REDBOOT_DIRECTORY_BLOCK;
    module_param(directory, int, 0);
#[no_mangle]
pub unsafe extern "C" fn redboot_checksum(img: *mut fis_image_desc) -> c_int {
    static inline int redboot_checksum(struct fis_image_desc *img)
    {
// RedBoot doesn't actually write the desc_cksum field yet AFAICT
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn parse_redboot_of(master: *mut mtd_info) {
    static void parse_redboot_of(struct mtd_info *master)
    {
    struct device_node *np;
    struct device_node *npart;
    u32 dirblock;
    int ret;
    np = mtd_get_of_node(master);
    if (!np)
    return;
    npart = of_get_child_by_name(np, "partitions");
    if (!npart)
    return;
    ret = of_property_read_u32(npart, "fis-index-block", &dirblock);
    of_node_put(npart);
    if (ret)
    return;
//
// Assign the block found in the device tree to the local
// directory block pointer.
//
    directory = dirblock;
    }
    static int parse_redboot_partitions(struct mtd_info *master,
    const struct mtd_partition **pparts,
    struct mtd_part_parser_data *data)
    {
    let mut nrparts: c_int = 0;
    struct fis_image_desc *buf;
    struct mtd_partition *parts;
    struct fis_list *fl = core::ptr::null_mut(), *tmp_fl;
    int ret, i;
    size_t retlen;
    char *names;
    char *nullname;
    let mut namelen: c_int = 0;
    let mut nulllen: c_int = 0;
    int numslots;
    unsigned long offset;

    static char nullstring[] = "unallocated";

    parse_redboot_of(master);
    if (directory < 0) {
    offset = master.size + directory * master.erasesize;
    while (mtd_block_isbad(master, offset)) {
    if (!offset) {
    nogood:
    pr_notice("Failed to find a non-bad block to check for RedBoot partition table\n");
    return -EIO;
    }
    offset -= master.erasesize;
    }
    } else {
    offset = (unsigned long) directory * master.erasesize;
    while (mtd_block_isbad(master, offset)) {
    offset += master.erasesize;
    if (offset == master.size)
    goto nogood;
    }
    }
    buf = vmalloc(master.erasesize);
    if (!buf)
    return -ENOMEM;
    pr_notice("Searching for RedBoot partition table in %s at offset 0x%lx\n",
    master.name, offset);
    ret = mtd_read(master, offset, master.erasesize, &retlen,
    (void *)buf);
    if (ret)
    goto out;
    if (retlen != master.erasesize) {
    ret = -EIO;
    goto out;
    }
    numslots = (master.erasesize / sizeof(struct fis_image_desc));
    for (i = 0; i < numslots; i++) {
    if (!memcmp(buf[i].name, "FIS directory", 14)) {
// This is apparently the FIS directory entry for the
// FIS directory itself.  The FIS directory size is
// one erase block; if the buf[i].size field is
// swab32(erasesize) then we know we are looking at
// a byte swapped FIS directory - swap all the entries!
// (NOTE: this is 'size' not 'data_length'; size is
// the full size of the entry.)
//
// RedBoot can combine the FIS directory and
    config partitions into a single eraseblock;
    we assume wrong-endian if either the swapped
    'size' matches the eraseblock size precisely,
    or if the swapped size actually fits in an
    eraseblock while the unswapped size doesn't. */
    if (swab32(buf[i].size) == master.erasesize ||
    (buf[i].size > master.erasesize
    && swab32(buf[i].size) < master.erasesize)) {
    int j;
// Update numslots based on actual FIS directory size
    numslots = swab32(buf[i].size) / sizeof(struct fis_image_desc);
    for (j = 0; j < numslots; ++j) {
// A single 0xff denotes a deleted entry.
// Two of them in a row is the end of the table.
//
    if (buf[j].name[0] == 0xff) {
    if (buf[j].name[1] == 0xff) {
    break;
    } else {
    continue;
    }
    }
// The unsigned long fields were written with the
// wrong byte sex, name and pad have no byte sex.
//
    swab32s(&buf[j].flash_base);
    swab32s(&buf[j].mem_base);
    swab32s(&buf[j].size);
    swab32s(&buf[j].entry_point);
    swab32s(&buf[j].data_length);
    swab32s(&buf[j].desc_cksum);
    swab32s(&buf[j].file_cksum);
    }
    } else if (buf[i].size < master.erasesize) {
// Update numslots based on actual FIS directory size
    numslots = buf[i].size / sizeof(struct fis_image_desc);
    }
    break;
    }
    }
    if (i == numslots) {
// Didn't find it
    pr_notice("No RedBoot partition table detected in %s\n",
    master.name);
    ret = 0;
    goto out;
    }
    for (i = 0; i < numslots; i++) {
    struct fis_list *new_fl, **prev;
    size_t name_len;
    if (buf[i].name[0] == 0xff) {
    if (buf[i].name[1] == 0xff) {
    break;
    } else {
    continue;
    }
    }
    if (!redboot_checksum(&buf[i]))
    break;
    name_len = strnlen(buf[i].name, sizeof(buf[i].name));
    if (name_len == sizeof(buf[i].name)) {
    ret = -EINVAL;
    goto out;
    }
    new_fl = kmalloc_obj(struct fis_list);
    namelen += name_len + 1;
    if (!new_fl) {
    ret = -ENOMEM;
    goto out;
    }
    new_fl.img = &buf[i];
    if (data && data.origin)
    buf[i].flash_base -= data.origin;
    else
    buf[i].flash_base &= master.size - 1;
// I'm sure the JFFS2 code has done me permanent damage.
// I now think the following is _normal_
//
    prev = &fl;
    while (*prev && (*prev).img.flash_base < new_fl.img.flash_base)
    prev = &(*prev).next;
    new_fl.next = *prev;
// prev = new_fl;
    nrparts++;
    }

    if (fl.img.flash_base) {
    nrparts++;
    nulllen = sizeof(nullstring);
    }
    for (tmp_fl = fl; tmp_fl.next; tmp_fl = tmp_fl.next) {
    if (tmp_fl.img.flash_base + tmp_fl.img.size + master.erasesize <= tmp_fl.next.img.flash_base) {
    nrparts++;
    nulllen = sizeof(nullstring);
    }
    }

    parts = kzalloc(sizeof(*parts) * nrparts + nulllen + namelen, GFP_KERNEL);
    if (!parts) {
    ret = -ENOMEM;
    goto out;
    }
    nullname = (char *)&parts[nrparts];

    if (nulllen > 0)
    strcpy(nullname, nullstring);

    names = nullname + nulllen;
    i = 0;

    if (fl.img.flash_base) {
    parts[0].name = nullname;
    parts[0].size = fl.img.flash_base;
    parts[0].offset = 0;
    i++;
    }

    for ( ; i < nrparts; i++) {
    parts[i].size = fl.img.size;
    parts[i].offset = fl.img.flash_base;
    parts[i].name = names;
    strcpy(names, fl.img.name);

    if (!strcmp(names, "RedBoot") ||
    !strcmp(names, "RedBoot config") ||
    !strcmp(names, "FIS directory")) {
    parts[i].mask_flags = MTD_WRITEABLE;
    }

    names += strlen(names) + 1;

    if (fl.next && fl.img.flash_base + fl.img.size + master.erasesize <= fl.next.img.flash_base) {
    i++;
    parts[i].offset = parts[i - 1].size + parts[i - 1].offset;
    parts[i].size = fl.next.img.flash_base - parts[i].offset;
    parts[i].name = nullname;
    }

    tmp_fl = fl;
    fl = fl.next;
    kfree(tmp_fl);
    }
    ret = nrparts;
// pparts = parts;
    out:
    while (fl) {
    struct fis_list *old = fl;
    fl = fl.next;
    kfree(old);
    }
    vfree(buf);
    return ret;
    }
    static const struct of_device_id mtd_parser_redboot_of_match_table[] = {
    { .compatible = "redboot-fis" },
    {},
    };
    MODULE_DEVICE_TABLE(of, mtd_parser_redboot_of_match_table);
    static struct mtd_part_parser redboot_parser = {
    .parse_fn = parse_redboot_partitions,
    .name = "RedBoot",
    .of_match_table = mtd_parser_redboot_of_match_table,
    };
    module_mtd_part_parser(redboot_parser);
// mtd parsers will request the module by parser name
    MODULE_ALIAS("RedBoot");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("David Woodhouse <dwmw2@infradead.org>");
    MODULE_DESCRIPTION("Parsing code for RedBoot Flash Image System (FIS) tables");
