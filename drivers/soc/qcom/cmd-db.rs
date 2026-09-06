//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/cmd-db.c
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
// Copyright (c) 2016-2018, 2020, The Linux Foundation. All rights reserved.
// Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
//

pub const NUM_PRIORITY: c_int = 2;
pub const MAX_SLV_ID: c_int = 8;
pub const SLAVE_ID_MASK: c_uint = 0x7;
pub const SLAVE_ID_SHIFT: c_int = 16;

//
// struct entry_header: header for each entry in cmddb
//
// @id: resource's identifier
// @priority: unused
// @addr: the address of the resource
// @len: length of the data
// @offset: offset from :@data_offset, start of the data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct entry_header {
    pub id: [u8; 8],
    pub priority: [__le32; NUM_PRIORITY],
    pub addr: __le32,
    pub len: __le16,
    pub offset: __le16,
}

//
// struct rsc_hdr: resource header information
//
// @slv_id: id for the resource
// @header_offset: entry's header at offset from the end of the cmd_db_header
// @data_offset: entry's data at offset from the end of the cmd_db_header
// @cnt: number of entries for HW type
// @version: MSB is major, LSB is minor
// @reserved: reserved for future use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsc_hdr {
    pub slv_id: __le16,
    pub header_offset: __le16,
    pub data_offset: __le16,
    pub cnt: __le16,
    pub version: __le16,
    pub reserved: [__le16; 3],
}

//
// struct cmd_db_header: The DB header information
//
// @version: The cmd db version
// @magic: constant expected in the database
// @header: array of resources
// @checksum: checksum for the header. Unused.
// @reserved: reserved memory
// @data: driver specific data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_db_header {
    pub version: __le32,
    pub magic: [u8; 4],
    pub header: [rsc_hdr; MAX_SLV_ID],
    pub checksum: __le32,
    pub reserved: __le32,
    pub data: [u8; ],
}

//
// DOC: Description of the Command DB database.
//
// At the start of the command DB memory is the cmd_db_header structure.
// The cmd_db_header holds the version, checksum, magic key as well as an
// array for header for each slave (depicted by the rsc_header). Each h/w
// based accelerator is a 'slave' (shared resource) and has slave id indicating
// the type of accelerator. The rsc_header is the header for such individual
// slaves of a given type. The entries for each of these slaves begin at the
// rsc_hdr.header_offset. In addition each slave could have auxiliary data
// that may be needed by the driver. The data for the slave starts at the
// entry_header.offset to the location pointed to by the rsc_hdr.data_offset.
//
// Drivers have a stringified key to a slave/resource. They can query the slave
// information and get the slave id and the auxiliary data and the length of the
// data. Using this information, they can format the request to be sent to the
// h/w accelerator and request a resource state.
//
    static const u8 CMD_DB_MAGIC[] = { 0xdb, 0x30, 0x03, 0x0c };
#[no_mangle]
unsafe extern "C" fn cmd_db_magic_matches(header: *const cmd_db_header) -> bool {
    static bool cmd_db_magic_matches(const struct cmd_db_header *header)
    {
    const u8 *magic = header.magic;
    return memcmp(magic, CMD_DB_MAGIC, ARRAY_SIZE(CMD_DB_MAGIC)) == 0;
    }
    static struct cmd_db_header *cmd_db_header;
    static inline const void *rsc_to_entry_header(const struct rsc_hdr *hdr)
    {
    let mut offset: u16 = le16_to_cpu(hdr.header_offset);
    return cmd_db_header.data + offset;
    }
    static inline void *
    rsc_offset(const struct rsc_hdr *hdr, const struct entry_header *ent)
    {
    let mut offset: u16 = le16_to_cpu(hdr.data_offset);
    let mut loffset: u16 = le16_to_cpu(ent.offset);
    return cmd_db_header.data + offset + loffset;
    }
//
// cmd_db_ready - Indicates if command DB is available
//
// Return: 0 on success, errno otherwise
//
#[no_mangle]
pub unsafe extern "C" fn cmd_db_ready() -> c_int {
    int cmd_db_ready(void)
    {
    if (cmd_db_header == core::ptr::null_mut())
    return -EPROBE_DEFER;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !cmd_db_magic_matches(cmd_db_header)) -> else {
    else if (!cmd_db_magic_matches(cmd_db_header))
    return -EINVAL;
    return 0;
    }
    EXPORT_SYMBOL_GPL(cmd_db_ready);
    static int cmd_db_get_header(const char *id, const struct entry_header **eh,
    const struct rsc_hdr **rh)
    {
    const struct rsc_hdr *rsc_hdr;
    const struct entry_header *ent;
    int ret, i, j;
    u8 query[sizeof(ent.id)] __nonstring;
    ret = cmd_db_ready();
    if (ret)
    return ret;
    strtomem_pad(query, id, 0);
    for (i = 0; i < MAX_SLV_ID; i++) {
    rsc_hdr = &cmd_db_header.header[i];
    if (!rsc_hdr.slv_id)
    break;
    ent = rsc_to_entry_header(rsc_hdr);
    for (j = 0; j < le16_to_cpu(rsc_hdr.cnt); j++, ent++) {
    if (memcmp(ent.id, query, sizeof(ent.id)) == 0) {
    if (eh)
// eh = ent;
    if (rh)
// rh = rsc_hdr;
    return 0;
    }
    }
    }
    return -ENODEV;
    }
//
// cmd_db_read_addr() - Query command db for resource id address.
//
// @id: resource id to query for address
//
// Return: resource address on success, 0 on error
//
// This is used to retrieve resource address based on resource
// id.
//
#[no_mangle]
pub unsafe extern "C" fn cmd_db_read_addr(id: *const c_char) -> u32 {
    u32 cmd_db_read_addr(const char *id)
    {
    int ret;
    const struct entry_header *ent;
    ret = cmd_db_get_header(id, &ent, core::ptr::null_mut());
    return ret < 0 ? 0 : le32_to_cpu(ent.addr);
    }
    EXPORT_SYMBOL_GPL(cmd_db_read_addr);
//
// cmd_db_read_aux_data() - Query command db for aux data.
//
// @id: Resource to retrieve AUX Data on
// @len: size of data buffer returned
//
// Return: pointer to data on success, error pointer otherwise
//
    const void *cmd_db_read_aux_data(const char *id, size_t *len)
    {
    int ret;
    const struct entry_header *ent;
    const struct rsc_hdr *rsc_hdr;
    ret = cmd_db_get_header(id, &ent, &rsc_hdr);
    if (ret)
    return ERR_PTR(ret);
    if (len)
// len = le16_to_cpu(ent->len);
    return rsc_offset(rsc_hdr, ent);
    }
    EXPORT_SYMBOL_GPL(cmd_db_read_aux_data);
//
// cmd_db_match_resource_addr() - Compare if both Resource addresses are same
//
// @addr1: Resource address to compare
// @addr2: Resource address to compare
//
// Return: true if two addresses refer to the same resource, false otherwise
//
#[no_mangle]
pub unsafe extern "C" fn cmd_db_match_resource_addr(addr1: u32, addr2: u32) -> bool {
    bool cmd_db_match_resource_addr(u32 addr1, u32 addr2)
    {
//
// Each RPMh VRM accelerator resource has 3 or 4 contiguous 4-byte
// aligned addresses associated with it. Ignore the offset to check
// for VRM requests.
//
    if (addr1 == addr2)
    return true;
#[no_mangle]
pub unsafe extern "C" fn if(VRM_ADDR(addr2): SLAVE_ID(addr1) == CMD_DB_HW_VRM && VRM_ADDR(addr1) ==) -> else {
    else if (SLAVE_ID(addr1) == CMD_DB_HW_VRM && VRM_ADDR(addr1) == VRM_ADDR(addr2))
    return true;
    return false;
    }
    EXPORT_SYMBOL_GPL(cmd_db_match_resource_addr);
//
// cmd_db_read_slave_id - Get the slave ID for a given resource address
//
// @id: Resource id to query the DB for version
//
// Return: cmd_db_hw_type enum on success, CMD_DB_HW_INVALID on error
//
#[no_mangle]
pub unsafe extern "C" fn cmd_db_read_slave_id(id: *const c_char) -> enum cmd_db_hw_type {
    enum cmd_db_hw_type cmd_db_read_slave_id(const char *id)
    {
    int ret;
    const struct entry_header *ent;
    u32 addr;
    ret = cmd_db_get_header(id, &ent, core::ptr::null_mut());
    if (ret < 0)
    return CMD_DB_HW_INVALID;
    addr = le32_to_cpu(ent.addr);
    return (addr >> SLAVE_ID_SHIFT) & SLAVE_ID_MASK;
    }
    EXPORT_SYMBOL_GPL(cmd_db_read_slave_id);

#[no_mangle]
unsafe extern "C" fn cmd_db_debugfs_dump(seq: *mut seq_file, p: *mut c_void) -> c_int {
    static int cmd_db_debugfs_dump(struct seq_file *seq, void *p)
    {
    int i, j;
    const struct rsc_hdr *rsc;
    const struct entry_header *ent;
    const char *name;
    u16 len, version;
    u8 major, minor;
    seq_puts(seq, "Command DB DUMP\n");
    for (i = 0; i < MAX_SLV_ID; i++) {
    rsc = &cmd_db_header.header[i];
    if (!rsc.slv_id)
    break;
    switch (le16_to_cpu(rsc.slv_id)) {
    case CMD_DB_HW_ARC:
    name = "ARC";
    break;
    case CMD_DB_HW_VRM:
    name = "VRM";
    break;
    case CMD_DB_HW_BCM:
    name = "BCM";
    break;
    default:
    name = "Unknown";
    break;
    }
    version = le16_to_cpu(rsc.version);
    major = version >> 8;
    minor = version;
    seq_printf(seq, "Slave %s (v%u.%u)\n", name, major, minor);
    seq_puts(seq, "-------------------------\n");
    ent = rsc_to_entry_header(rsc);
    for (j = 0; j < le16_to_cpu(rsc.cnt); j++, ent++) {
    seq_printf(seq, "0x%05x: %*pEp", le32_to_cpu(ent.addr),
    (int)strnlen(ent.id, sizeof(ent.id)), ent.id);
    len = le16_to_cpu(ent.len);
    if (len) {
    seq_printf(seq, " [%*ph]",
    len, rsc_offset(rsc, ent));
    }
    seq_putc(seq, '\n');
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn open_cmd_db_debugfs(inode: *mut inode, file: *mut file) -> c_int {
    static int open_cmd_db_debugfs(struct inode *inode, struct file *file)
    {
    return single_open(file, cmd_db_debugfs_dump, inode.i_private);
    }

    static const struct file_operations cmd_db_debugfs_ops = {

    .open = open_cmd_db_debugfs,

    .read = seq_read,
    .llseek = seq_lseek,
    .release = single_release,
    };
#[no_mangle]
unsafe extern "C" fn cmd_db_dev_probe(pdev: *mut platform_device) -> c_int {
    static int cmd_db_dev_probe(struct platform_device *pdev)
    {
    struct reserved_mem *rmem;
    let mut ret: c_int = 0;
    rmem = of_reserved_mem_lookup(pdev.dev.of_node);
    if (!rmem) {
    dev_err(&pdev.dev, "failed to acquire memory region\n");
    return -EINVAL;
    }
    cmd_db_header = devm_memremap(&pdev.dev, rmem.base, rmem.size, MEMREMAP_WC);
    if (IS_ERR(cmd_db_header)) {
    ret = PTR_ERR(cmd_db_header);
    cmd_db_header = core::ptr::null_mut();
    return ret;
    }
    if (!cmd_db_magic_matches(cmd_db_header)) {
    dev_err(&pdev.dev, "Invalid Command DB Magic\n");
    cmd_db_header = core::ptr::null_mut();
    return -EINVAL;
    }
    debugfs_create_file("cmd-db", 0400, core::ptr::null_mut(), core::ptr::null_mut(), &cmd_db_debugfs_ops);
    device_set_pm_not_required(&pdev.dev);
    return 0;
    }
    static const struct of_device_id cmd_db_match_table[] = {
    { .compatible = "qcom,cmd-db" },
    { }
    };
    MODULE_DEVICE_TABLE(of, cmd_db_match_table);
    static struct platform_driver cmd_db_dev_driver = {
    .probe  = cmd_db_dev_probe,
    .driver = {
    .name = "cmd-db",
    .of_match_table = cmd_db_match_table,
    .suppress_bind_attrs = true,
    },
    };
#[no_mangle]
unsafe extern "C" fn cmd_db_device_init() -> int __init {
    static int __init cmd_db_device_init(void)
    {
    return platform_driver_register(&cmd_db_dev_driver);
    }
    core_initcall(cmd_db_device_init);
    MODULE_DESCRIPTION("Qualcomm Technologies, Inc. Command DB Driver");
    MODULE_LICENSE("GPL v2");
