//! Automatically rewritten from C to Rust
//! Source: drivers/thunderbolt/eeprom.c
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
// Thunderbolt driver - eeprom access
//
// Copyright (c) 2014 Andreas Noever <andreas.noever@gmail.com>
// Copyright (C) 2018, Intel Corporation
//

//
// tb_eeprom_ctl_write() - write control word
//
#[no_mangle]
unsafe extern "C" fn tb_eeprom_ctl_write(sw: *mut tb_switch, ctl: *mut tb_eeprom_ctl) -> c_int {
    static int tb_eeprom_ctl_write(struct tb_switch *sw, struct tb_eeprom_ctl *ctl)
    {
    return tb_sw_write(sw, ctl, TB_CFG_SWITCH, sw.cap_plug_events + ROUTER_CS_4, 1);
    }
//
// tb_eeprom_ctl_read() - read control word
//
#[no_mangle]
unsafe extern "C" fn tb_eeprom_ctl_read(sw: *mut tb_switch, ctl: *mut tb_eeprom_ctl) -> c_int {
    static int tb_eeprom_ctl_read(struct tb_switch *sw, struct tb_eeprom_ctl *ctl)
    {
    return tb_sw_read(sw, ctl, TB_CFG_SWITCH, sw.cap_plug_events + ROUTER_CS_4, 1);
    }
    enum tb_eeprom_transfer {
    TB_EEPROM_IN,
    TB_EEPROM_OUT,
    };
//
// tb_eeprom_active - enable rom access
//
// WARNING: Always disable access after usage. Otherwise the controller will
// fail to reprobe.
//
#[no_mangle]
unsafe extern "C" fn tb_eeprom_active(sw: *mut tb_switch, enable: bool) -> c_int {
    static int tb_eeprom_active(struct tb_switch *sw, bool enable)
    {
    struct tb_eeprom_ctl ctl;
    let mut res: c_int = tb_eeprom_ctl_read(sw, &ctl);
    if (res)
    return res;
    if (enable) {
    ctl.bit_banging_enable = 1;
    res = tb_eeprom_ctl_write(sw, &ctl);
    if (res)
    return res;
    ctl.fl_cs = 0;
    return tb_eeprom_ctl_write(sw, &ctl);
    } else {
    ctl.fl_cs = 1;
    res = tb_eeprom_ctl_write(sw, &ctl);
    if (res)
    return res;
    ctl.bit_banging_enable = 0;
    return tb_eeprom_ctl_write(sw, &ctl);
    }
    }
//
// tb_eeprom_transfer - transfer one bit
//
// If TB_EEPROM_IN is passed, then the bit can be retrieved from ctl->fl_do.
// If TB_EEPROM_OUT is passed, then ctl->fl_di will be written.
//
    static int tb_eeprom_transfer(struct tb_switch *sw, struct tb_eeprom_ctl *ctl,
    enum tb_eeprom_transfer direction)
    {
    int res;
    if (direction == TB_EEPROM_OUT) {
    res = tb_eeprom_ctl_write(sw, ctl);
    if (res)
    return res;
    }
    ctl.fl_sk = 1;
    res = tb_eeprom_ctl_write(sw, ctl);
    if (res)
    return res;
    if (direction == TB_EEPROM_IN) {
    res = tb_eeprom_ctl_read(sw, ctl);
    if (res)
    return res;
    }
    ctl.fl_sk = 0;
    return tb_eeprom_ctl_write(sw, ctl);
    }
//
// tb_eeprom_out - write one byte to the bus
//
#[no_mangle]
unsafe extern "C" fn tb_eeprom_out(sw: *mut tb_switch, val: u8) -> c_int {
    static int tb_eeprom_out(struct tb_switch *sw, u8 val)
    {
    struct tb_eeprom_ctl ctl;
    int i;
    let mut res: c_int = tb_eeprom_ctl_read(sw, &ctl);
    if (res)
    return res;
    for (i = 0; i < 8; i++) {
    ctl.fl_di = val & 0x80;
    res = tb_eeprom_transfer(sw, &ctl, TB_EEPROM_OUT);
    if (res)
    return res;
    val <<= 1;
    }
    return 0;
    }
//
// tb_eeprom_in - read one byte from the bus
//
#[no_mangle]
unsafe extern "C" fn tb_eeprom_in(sw: *mut tb_switch, val: *mut u8) -> c_int {
    static int tb_eeprom_in(struct tb_switch *sw, u8 *val)
    {
    struct tb_eeprom_ctl ctl;
    int i;
    let mut res: c_int = tb_eeprom_ctl_read(sw, &ctl);
    if (res)
    return res;
// val = 0;
    for (i = 0; i < 8; i++) {
// val <<= 1;
    res = tb_eeprom_transfer(sw, &ctl, TB_EEPROM_IN);
    if (res)
    return res;
// val |= ctl.fl_do;
    }
    return 0;
    }
//
// tb_eeprom_get_drom_offset - get drom offset within eeprom
//
#[no_mangle]
unsafe extern "C" fn tb_eeprom_get_drom_offset(sw: *mut tb_switch, offset: *mut u16) -> c_int {
    static int tb_eeprom_get_drom_offset(struct tb_switch *sw, u16 *offset)
    {
    struct tb_cap_plug_events cap;
    int res;
    if (!sw.cap_plug_events) {
    tb_sw_warn(sw, "no TB_CAP_PLUG_EVENTS, cannot read eeprom\n");
    return -ENODEV;
    }
    res = tb_sw_read(sw, &cap, TB_CFG_SWITCH, sw.cap_plug_events,
    sizeof(cap) / 4);
    if (res)
    return res;
    if (!cap.eeprom_ctl.present || cap.eeprom_ctl.not_present) {
    tb_sw_warn(sw, "no NVM\n");
    return -ENODEV;
    }
    if (cap.drom_offset > 0xffff) {
    tb_sw_warn(sw, "drom offset is larger than 0xffff: %#x\n",
    cap.drom_offset);
    return -ENXIO;
    }
// offset = cap.drom_offset;
    return 0;
    }
//
// tb_eeprom_read_n - read count bytes from offset into val
//
    static int tb_eeprom_read_n(struct tb_switch *sw, u16 offset, u8 *val,
    size_t count)
    {
    u16 drom_offset;
    int i, res;
    res = tb_eeprom_get_drom_offset(sw, &drom_offset);
    if (res)
    return res;
    offset += drom_offset;
    res = tb_eeprom_active(sw, true);
    if (res)
    return res;
    res = tb_eeprom_out(sw, 3);
    if (res)
    return res;
    res = tb_eeprom_out(sw, offset >> 8);
    if (res)
    return res;
    res = tb_eeprom_out(sw, offset);
    if (res)
    return res;
    for (i = 0; i < count; i++) {
    res = tb_eeprom_in(sw, val + i);
    if (res)
    return res;
    }
    return tb_eeprom_active(sw, false);
    }
#[no_mangle]
unsafe extern "C" fn tb_crc8(data: *mut u8, len: c_int) -> u8 {
    static u8 tb_crc8(u8 *data, int len)
    {
    int i, j;
    let mut val: u8 = 0xff;
    for (i = 0; i < len; i++) {
    val ^= data[i];
    for (j = 0; j < 8; j++)
    val = (val << 1) ^ ((val & 0x80) ? 7 : 0);
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn tb_crc32(data: *mut c_void, len: usize) -> u32 {
    static u32 tb_crc32(void *data, size_t len)
    {
    return ~crc32c(~0, data, len);
    }
pub const TB_DROM_DATA_START: c_int = 13;
pub const TB_DROM_HEADER_SIZE: c_int = 22;
pub const USB4_DROM_HEADER_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_drom_header {
// BYTE 0
    pub /: *mut *mut u8 uid_crc8; / checksum for uid,
// BYTES 1-8
    pub uid: u64,
// BYTES 9-12
    pub /: *mut *mut u32 data_crc32; / checksum for data_len bytes starting at byte 13,
// BYTE 13
    pub /: *mut *mut u8 device_rom_revision; / should be <= 1,
    pub data_len:12: u16,
    pub reserved:4: u8,
// BYTES 16-21 - Only for TBT DROM, nonexistent in USB4 DROM
    pub vendor_id: u16,
    pub model_id: u16,
    pub model_rev: u8,
    pub eeprom_rev: u8,
    pub __packed: },
    enum tb_drom_entry_type {
// force unsigned to prevent "one-bit signed bitfield" warning
    TB_DROM_ENTRY_GENERIC = 0U,
    TB_DROM_ENTRY_PORT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_drom_entry_header {
    pub len: u8,
    pub index:6: u8,
    pub /: *mut *mut bool port_disabled:1; / only valid if type is TB_DROM_ENTRY_PORT,
    pub type:1: enum tb_drom_entry_type,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_drom_entry_generic {
    pub header: tb_drom_entry_header,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_drom_entry_port {
// BYTES 0-1
    pub header: tb_drom_entry_header,
// BYTE 2
    pub dual_link_port_rid:4: u8,
    pub link_nr:1: u8,
    pub unknown1:2: u8,
    pub has_dual_link_port:1: bool,
// BYTE 3
    pub dual_link_port_nr:6: u8,
    pub unknown2:2: u8,
// BYTES 4 - 5 TODO decode
    pub micro2:4: u8,
    pub micro1:4: u8,
    pub micro3: u8,
// BYTES 6-7, TODO: verify (find hardware that has these set)
    pub peer_port_rid:4: u8,
    pub unknown3:3: u8,
    pub has_peer_port:1: bool,
    pub peer_port_nr:6: u8,
    pub unknown4:2: u8,
    pub __packed: },
// USB4 product descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_drom_entry_desc {
    pub header: tb_drom_entry_header,
    pub bcdUSBSpec: u16,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdProductFWRevision: u16,
    pub TID: u32,
    pub productHWRevision: u8,
}

//
// tb_drom_read_uid_only() - Read UID directly from DROM
// @sw: Router whose UID to read
// @uid: UID is placed here
//
// Does not use the cached copy in sw->drom. Used during resume to check switch
// identity.
//
// Return: %0 on success, negative errno otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn tb_drom_read_uid_only(sw: *mut tb_switch, uid: *mut u64) -> c_int {
    int tb_drom_read_uid_only(struct tb_switch *sw, u64 *uid)
    {
    u8 data[9];
    u8 crc;
    int res;
// read uid
    res = tb_eeprom_read_n(sw, 0, data, 9);
    if (res)
    return res;
    crc = tb_crc8(data + 1, 8);
    if (crc != data[0]) {
    tb_sw_warn(sw, "uid crc8 mismatch (expected: %#x, got: %#x)\n",
    data[0], crc);
    return -EIO;
    }
// uid = *(u64 *)(data+1);
    return 0;
    }
    static int tb_drom_parse_entry_generic(struct tb_switch *sw,
    struct tb_drom_entry_header *header)
    {
    const struct tb_drom_entry_generic *entry =
    (const struct tb_drom_entry_generic *)header;
    switch (header.index) {
    case 1:
// Length includes 2 bytes header so remove it before copy
    sw.vendor_name = kstrndup(entry.data,
    header.len - sizeof(*header), GFP_KERNEL);
    if (!sw.vendor_name)
    return -ENOMEM;
    break;
    case 2:
    sw.device_name = kstrndup(entry.data,
    header.len - sizeof(*header), GFP_KERNEL);
    if (!sw.device_name)
    return -ENOMEM;
    break;
    case 9: {
    const struct tb_drom_entry_desc *desc =
    (const struct tb_drom_entry_desc *)entry;
    if (!sw.vendor && !sw.device) {
    sw.vendor = desc.idVendor;
    sw.device = desc.idProduct;
    }
    break;
    }
    }
    return 0;
    }
    static int tb_drom_parse_entry_port(struct tb_switch *sw,
    struct tb_drom_entry_header *header)
    {
    struct tb_port *port;
    int res;
    enum tb_port_type type;
//
// Some DROMs list more ports than the controller actually has
// so we skip those but allow the parser to continue.
//
    if (header.index > sw.config.max_port_number) {
    dev_info_once(&sw.dev, "ignoring unnecessary extra entries in DROM\n");
    return 0;
    }
    port = &sw.ports[header.index];
    port.disabled = header.port_disabled;
    if (port.disabled)
    return 0;
    res = tb_port_read(port, &type, TB_CFG_PORT, 2, 1);
    if (res)
    return res;
    type &= 0xffffff;
    if (type == TB_TYPE_PORT) {
    struct tb_drom_entry_port *entry = (void *) header;
    if (header.len != sizeof(*entry)) {
    tb_sw_warn(sw,
    "port entry has size %#x (expected %#zx)\n",
    header.len, sizeof(struct tb_drom_entry_port));
    return -EIO;
    }
    port.link_nr = entry.link_nr;
    if (entry.has_dual_link_port) {
    if (entry.dual_link_port_nr > sw.config.max_port_number) {
    tb_sw_warn(sw,
    "port entry has invalid dual link port number %u\n",
    entry.dual_link_port_nr);
    return -EIO;
    }
    port.dual_link_port =
    &port.sw.ports[entry.dual_link_port_nr];
    }
    }
    return 0;
    }
//
// tb_drom_parse_entries - parse the linked list of drom entries
//
// Drom must have been copied to sw->drom.
//
#[no_mangle]
unsafe extern "C" fn tb_drom_parse_entries(sw: *mut tb_switch, header_size: usize) -> c_int {
    static int tb_drom_parse_entries(struct tb_switch *sw, size_t header_size)
    {
    struct tb_drom_header *header = (void *) sw.drom;
    let mut pos: u16 = header_size;
    let mut drom_size: u16 = header.data_len + TB_DROM_DATA_START;
    int res;
    while (pos < drom_size) {
    struct tb_drom_entry_header *entry = (void *) (sw.drom + pos);
    if (pos + 1 == drom_size || pos + entry.len > drom_size
    || !entry.len) {
    tb_sw_warn(sw, "DROM buffer overrun\n");
    return -EIO;
    }
    switch (entry.type) {
    case TB_DROM_ENTRY_GENERIC:
    res = tb_drom_parse_entry_generic(sw, entry);
    break;
    case TB_DROM_ENTRY_PORT:
    res = tb_drom_parse_entry_port(sw, entry);
    break;
    }
    if (res)
    return res;
    pos += entry.len;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tb_switch_drom_alloc(sw: *mut tb_switch, size: usize) -> c_int {
    static int tb_switch_drom_alloc(struct tb_switch *sw, size_t size)
    {
    sw.drom = kzalloc(size, GFP_KERNEL);
    if (!sw.drom)
    return -ENOMEM;

    sw.drom_blob.data = sw.drom;
    sw.drom_blob.size = size;

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tb_switch_drom_free(sw: *mut tb_switch) {
    static void tb_switch_drom_free(struct tb_switch *sw)
    {

    sw.drom_blob.data = core::ptr::null_mut();
    sw.drom_blob.size = 0;

    kfree(sw.drom);
    sw.drom = core::ptr::null_mut();
    }
//
// tb_drom_copy_efi - copy drom supplied by EFI to sw->drom if present
//
#[no_mangle]
unsafe extern "C" fn tb_drom_copy_efi(sw: *mut tb_switch, size: *mut u16) -> c_int {
    static int tb_drom_copy_efi(struct tb_switch *sw, u16 *size)
    {
    struct device *dev = sw.tb.nhi.dev;
    int len, res;
    len = device_property_count_u8(dev, "ThunderboltDROM");
    if (len < 0 || len < sizeof(struct tb_drom_header))
    return -EINVAL;
    res = tb_switch_drom_alloc(sw, len);
    if (res)
    return res;
    res = device_property_read_u8_array(dev, "ThunderboltDROM", sw.drom,
    len);
    if (res)
    goto err;
// size = ((struct tb_drom_header *)sw->drom)->data_len +
    TB_DROM_DATA_START;
    if (*size > len)
    goto err;
    return 0;
    err:
    tb_switch_drom_free(sw);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn tb_drom_copy_nvm(sw: *mut tb_switch, size: *mut u16) -> c_int {
    static int tb_drom_copy_nvm(struct tb_switch *sw, u16 *size)
    {
    u16 drom_offset;
    int ret;
    if (!sw.dma_port)
    return -ENODEV;
    ret = tb_eeprom_get_drom_offset(sw, &drom_offset);
    if (ret)
    return ret;
    if (!drom_offset)
    return -ENODEV;
    ret = dma_port_flash_read(sw.dma_port, drom_offset + 14, size,
    sizeof(*size));
    if (ret)
    return ret;
// Size includes CRC8 + UID + CRC32
// size += 1 + 8 + 4;
    ret = tb_switch_drom_alloc(sw, *size);
    if (ret)
    return ret;
    ret = dma_port_flash_read(sw.dma_port, drom_offset, sw.drom, *size);
    if (ret) {
    tb_switch_drom_free(sw);
    return ret;
    }
//
// Read UID from the minimal DROM because the one in NVM is just
// a placeholder.
//
    tb_drom_read_uid_only(sw, &sw.uid);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb4_copy_drom(sw: *mut tb_switch, size: *mut u16) -> c_int {
    static int usb4_copy_drom(struct tb_switch *sw, u16 *size)
    {
    int ret;
    ret = usb4_switch_drom_read(sw, 14, size, sizeof(*size));
    if (ret)
    return ret;
// Size includes CRC8 + UID + CRC32
// size += 1 + 8 + 4;
    ret = tb_switch_drom_alloc(sw, *size);
    if (ret)
    return ret;
    ret = usb4_switch_drom_read(sw, 0, sw.drom, *size);
    if (ret)
    tb_switch_drom_free(sw);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tb_drom_bit_bang(sw: *mut tb_switch, size: *mut u16) -> c_int {
    static int tb_drom_bit_bang(struct tb_switch *sw, u16 *size)
    {
    int ret;
    ret = tb_eeprom_read_n(sw, 14, (u8 *)size, 2);
    if (ret)
    return ret;
// size &= 0x3ff;
// size += TB_DROM_DATA_START;
    tb_sw_dbg(sw, "reading DROM (length: %#x)\n", *size);
    if (*size < sizeof(struct tb_drom_header)) {
    tb_sw_warn(sw, "DROM too small, aborting\n");
    return -EIO;
    }
    ret = tb_switch_drom_alloc(sw, *size);
    if (ret)
    return ret;
    ret = tb_eeprom_read_n(sw, 0, sw.drom, *size);
    if (ret)
    tb_switch_drom_free(sw);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tb_drom_parse_v1(sw: *mut tb_switch) -> c_int {
    static int tb_drom_parse_v1(struct tb_switch *sw)
    {
    const struct tb_drom_header *header =
    (const struct tb_drom_header *)sw.drom;
    u32 crc;
    crc = tb_crc8((u8 *) &header.uid, 8);
    if (crc != header.uid_crc8) {
    tb_sw_warn(sw,
    "DROM UID CRC8 mismatch (expected: %#x, got: %#x)\n",
    header.uid_crc8, crc);
    return -EIO;
    }
    if (!sw.uid)
    sw.uid = header.uid;
    sw.vendor = header.vendor_id;
    sw.device = header.model_id;
    crc = tb_crc32(sw.drom + TB_DROM_DATA_START, header.data_len);
    if (crc != header.data_crc32) {
    tb_sw_warn(sw,
    "DROM data CRC32 mismatch (expected: %#x, got: %#x), continuing\n",
    header.data_crc32, crc);
    }
    return tb_drom_parse_entries(sw, TB_DROM_HEADER_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn usb4_drom_parse(sw: *mut tb_switch) -> c_int {
    static int usb4_drom_parse(struct tb_switch *sw)
    {
    const struct tb_drom_header *header =
    (const struct tb_drom_header *)sw.drom;
    u32 crc;
    crc = tb_crc32(sw.drom + TB_DROM_DATA_START, header.data_len);
    if (crc != header.data_crc32) {
    tb_sw_warn(sw,
    "DROM data CRC32 mismatch (expected: %#x, got: %#x), continuing\n",
    header.data_crc32, crc);
    }
    return tb_drom_parse_entries(sw, USB4_DROM_HEADER_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn tb_drom_parse(sw: *mut tb_switch, size: u16) -> c_int {
    static int tb_drom_parse(struct tb_switch *sw, u16 size)
    {
    const struct tb_drom_header *header = (const void *)sw.drom;
    int ret;
    if (header.data_len + TB_DROM_DATA_START != size) {
    tb_sw_warn(sw, "DROM size mismatch\n");
    ret = -EIO;
    goto err;
    }
    tb_sw_dbg(sw, "DROM version: %d\n", header.device_rom_revision);
    switch (header.device_rom_revision) {
    case 3:
    ret = usb4_drom_parse(sw);
    break;
    default:
    tb_sw_warn(sw, "DROM device_rom_revision %#x unknown\n",
    header.device_rom_revision);
    fallthrough;
    case 1:
    ret = tb_drom_parse_v1(sw);
    break;
    }
    if (ret) {
    tb_sw_warn(sw, "parsing DROM failed\n");
    goto err;
    }
    return 0;
    err:
    tb_switch_drom_free(sw);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tb_drom_host_read(sw: *mut tb_switch) -> c_int {
    static int tb_drom_host_read(struct tb_switch *sw)
    {
    u16 size;
    if (tb_switch_is_usb4(sw)) {
    usb4_switch_read_uid(sw, &sw.uid);
    if (!usb4_copy_drom(sw, &size))
    return tb_drom_parse(sw, size);
    } else {
    if (!tb_drom_copy_efi(sw, &size))
    return tb_drom_parse(sw, size);
    if (!tb_drom_copy_nvm(sw, &size))
    return tb_drom_parse(sw, size);
    tb_drom_read_uid_only(sw, &sw.uid);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tb_drom_device_read(sw: *mut tb_switch) -> c_int {
    static int tb_drom_device_read(struct tb_switch *sw)
    {
    u16 size;
    int ret;
    if (tb_switch_is_usb4(sw)) {
    usb4_switch_read_uid(sw, &sw.uid);
    ret = usb4_copy_drom(sw, &size);
    } else {
    ret = tb_drom_bit_bang(sw, &size);
    }
    if (ret)
    return ret;
    return tb_drom_parse(sw, size);
    }
//
// tb_drom_read() - Copy DROM to sw->drom and parse it
// @sw: Router whose DROM to read and parse
//
// This function reads router DROM and if successful parses the entries and
// populates the fields in @sw accordingly. Can be called for any router
// generation.
//
// Return: %0 on success, negative errno otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn tb_drom_read(sw: *mut tb_switch) -> c_int {
    int tb_drom_read(struct tb_switch *sw)
    {
    if (sw.drom)
    return 0;
    if (!tb_route(sw))
    return tb_drom_host_read(sw);
    return tb_drom_device_read(sw);
    }
