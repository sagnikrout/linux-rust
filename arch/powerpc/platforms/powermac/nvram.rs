//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powermac/nvram.c
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
// Copyright (C) 2002 Benjamin Herrenschmidt (benh@kernel.crashing.org)
//
// Todo: - add support for the OF persistent properties
//

// Macro flag: #define DEBUG

// Macro flag: #define DBG(x...)

pub const NVRAM_SIZE: c_uint = 0x2000	/* 8kB of non-volatile RAM */;
pub const CORE99_SIGNATURE: c_uint = 0x5a;
pub const CORE99_ADLER_START: c_uint = 0x14;
// On Core99, nvram is either a sharp, a micron or an AMD flash
pub const SM_FLASH_STATUS_DONE: c_uint = 0x80;
pub const SM_FLASH_STATUS_ERR: c_uint = 0x38;
pub const SM_FLASH_CMD_ERASE_CONFIRM: c_uint = 0xd0;
pub const SM_FLASH_CMD_ERASE_SETUP: c_uint = 0x20;
pub const SM_FLASH_CMD_RESET: c_uint = 0xff;
pub const SM_FLASH_CMD_WRITE_SETUP: c_uint = 0x40;
pub const SM_FLASH_CMD_CLEAR_STATUS: c_uint = 0x50;
pub const SM_FLASH_CMD_READ_STATUS: c_uint = 0x70;
// CHRP NVRAM header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chrp_header {
    pub signature: u8,
    pub cksum: u8,
    pub len: u16,
    pub name: [c_char; 12],
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core99_header {
    pub hdr: chrp_header,
    pub adler: u32,
    pub generation: u32,
    pub reserved: [u32; 2],
}

//
// Read and write the non-volatile RAM on PowerMacs and CHRP machines.
//
    static int nvram_naddrs;
    static volatile unsigned char __iomem *nvram_data;
    static int is_core_99;
    static int core99_bank;
    static int nvram_partitions[3];
// XXX Turn that into a sem
    static DEFINE_RAW_SPINLOCK(nv_lock);
    static int (*core99_write_bank)(int bank, u8* datas);
    static int (*core99_erase_bank)(int bank);
    static char *nvram_image;
#[no_mangle]
unsafe extern "C" fn core99_nvram_read_byte(addr: c_int) -> c_uchar {
    static unsigned char core99_nvram_read_byte(int addr)
    {
    if (nvram_image == core::ptr::null_mut())
    return 0xff;
    return nvram_image[addr];
    }
#[no_mangle]
unsafe extern "C" fn core99_nvram_write_byte(addr: c_int, val: c_uchar) {
    static void core99_nvram_write_byte(int addr, unsigned char val)
    {
    if (nvram_image == core::ptr::null_mut())
    return;
    nvram_image[addr] = val;
    }
#[no_mangle]
unsafe extern "C" fn core99_nvram_read(buf: *mut c_char, count: usize, index: *mut loff_t) -> isize {
    static ssize_t core99_nvram_read(char *buf, size_t count, loff_t *index)
    {
    int i;
    if (nvram_image == core::ptr::null_mut())
    return -ENODEV;
    if (*index > NVRAM_SIZE)
    return 0;
    i = *index;
    if (i + count > NVRAM_SIZE)
    count = NVRAM_SIZE - i;
    memcpy(buf, &nvram_image[i], count);
// index = i + count;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn core99_nvram_write(buf: *mut c_char, count: usize, index: *mut loff_t) -> isize {
    static ssize_t core99_nvram_write(char *buf, size_t count, loff_t *index)
    {
    int i;
    if (nvram_image == core::ptr::null_mut())
    return -ENODEV;
    if (*index > NVRAM_SIZE)
    return 0;
    i = *index;
    if (i + count > NVRAM_SIZE)
    count = NVRAM_SIZE - i;
    memcpy(&nvram_image[i], buf, count);
// index = i + count;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn core99_nvram_size() -> isize {
    static ssize_t core99_nvram_size(void)
    {
    if (nvram_image == core::ptr::null_mut())
    return -ENODEV;
    return NVRAM_SIZE;
    }

    static volatile unsigned char __iomem *nvram_addr;
    static int nvram_mult;
#[no_mangle]
unsafe extern "C" fn ppc32_nvram_size() -> isize {
    static ssize_t ppc32_nvram_size(void)
    {
    return NVRAM_SIZE;
    }
#[no_mangle]
unsafe extern "C" fn direct_nvram_read_byte(addr: c_int) -> c_uchar {
    static unsigned char direct_nvram_read_byte(int addr)
    {
    return in_8(&nvram_data[(addr & (NVRAM_SIZE - 1)) * nvram_mult]);
    }
#[no_mangle]
unsafe extern "C" fn direct_nvram_write_byte(addr: c_int, val: c_uchar) {
    static void direct_nvram_write_byte(int addr, unsigned char val)
    {
    out_8(&nvram_data[(addr & (NVRAM_SIZE - 1)) * nvram_mult], val);
    }
#[no_mangle]
unsafe extern "C" fn indirect_nvram_read_byte(addr: c_int) -> c_uchar {
    static unsigned char indirect_nvram_read_byte(int addr)
    {
    unsigned char val;
    unsigned long flags;
    raw_spin_lock_irqsave(&nv_lock, flags);
    out_8(nvram_addr, addr >> 5);
    val = in_8(&nvram_data[(addr & 0x1f) << 4]);
    raw_spin_unlock_irqrestore(&nv_lock, flags);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn indirect_nvram_write_byte(addr: c_int, val: c_uchar) {
    static void indirect_nvram_write_byte(int addr, unsigned char val)
    {
    unsigned long flags;
    raw_spin_lock_irqsave(&nv_lock, flags);
    out_8(nvram_addr, addr >> 5);
    out_8(&nvram_data[(addr & 0x1f) << 4], val);
    raw_spin_unlock_irqrestore(&nv_lock, flags);
    }

#[no_mangle]
unsafe extern "C" fn pmu_nvram_complete(req: *mut adb_request) {
    static void pmu_nvram_complete(struct adb_request *req)
    {
    if (req.arg)
    complete((struct completion *)req.arg);
    }
#[no_mangle]
unsafe extern "C" fn pmu_nvram_read_byte(addr: c_int) -> c_uchar {
    static unsigned char pmu_nvram_read_byte(int addr)
    {
    struct adb_request req;
    DECLARE_COMPLETION_ONSTACK(req_complete);
    req.arg = system_state == SYSTEM_RUNNING ? &req_complete : core::ptr::null_mut();
    if (pmu_request(&req, pmu_nvram_complete, 3, PMU_READ_NVRAM,
    (addr >> 8) & 0xff, addr & 0xff))
    return 0xff;
    if (system_state == SYSTEM_RUNNING)
    wait_for_completion(&req_complete);
    while (!req.complete)
    pmu_poll();
    return req.reply[0];
    }
#[no_mangle]
unsafe extern "C" fn pmu_nvram_write_byte(addr: c_int, val: c_uchar) {
    static void pmu_nvram_write_byte(int addr, unsigned char val)
    {
    struct adb_request req;
    DECLARE_COMPLETION_ONSTACK(req_complete);
    req.arg = system_state == SYSTEM_RUNNING ? &req_complete : core::ptr::null_mut();
    if (pmu_request(&req, pmu_nvram_complete, 4, PMU_WRITE_NVRAM,
    (addr >> 8) & 0xff, addr & 0xff, val))
    return;
    if (system_state == SYSTEM_RUNNING)
    wait_for_completion(&req_complete);
    while (!req.complete)
    pmu_poll();
    }

#[no_mangle]
unsafe extern "C" fn chrp_checksum(hdr: *mut *mut chrp_header) -> u8 {
    static u8 chrp_checksum(struct chrp_header* hdr)
    {
    u8 *ptr;
    let mut sum: u16 = hdr.signature;
    for (ptr = (u8 *)&hdr.len; ptr < hdr.data; ptr++)
    sum += *ptr;
    while (sum > 0xFF)
    sum = (sum & 0xFF) + (sum>>8);
    return sum;
    }
#[no_mangle]
unsafe extern "C" fn core99_calc_adler(buffer: *mut u8) -> u32 {
    static u32 core99_calc_adler(u8 *buffer)
    {
    int cnt;
    u32 low, high;
    buffer += CORE99_ADLER_START;
    low = 1;
    high = 0;
    for (cnt=0; cnt<(NVRAM_SIZE-CORE99_ADLER_START); cnt++) {
    if ((cnt % 5000) == 0) {
    high  %= 65521UL;
    high %= 65521UL;
    }
    low += buffer[cnt];
    high += low;
    }
    low  %= 65521UL;
    high %= 65521UL;
    return (high << 16) | low;
    }
#[no_mangle]
unsafe extern "C" fn core99_check(datas: *mut u8) -> u32 __init {
    static u32 __init core99_check(u8 *datas)
    {
    let mut hdr99: *mut core99_header = (struct core99_header*)datas;
    if (hdr99.hdr.signature != CORE99_SIGNATURE) {
    DBG("Invalid signature\n");
    return 0;
    }
    if (hdr99.hdr.cksum != chrp_checksum(&hdr99.hdr)) {
    DBG("Invalid checksum\n");
    return 0;
    }
    if (hdr99.adler != core99_calc_adler(datas)) {
    DBG("Invalid adler\n");
    return 0;
    }
    return hdr99.generation;
    }
#[no_mangle]
unsafe extern "C" fn sm_erase_bank(bank: c_int) -> c_int {
    static int sm_erase_bank(int bank)
    {
    int stat;
    unsigned long timeout;
    u8 __iomem *base = (u8 __iomem *)nvram_data + core99_bank*NVRAM_SIZE;
    DBG("nvram: Sharp/Micron Erasing bank %d...\n", bank);
    out_8(base, SM_FLASH_CMD_ERASE_SETUP);
    out_8(base, SM_FLASH_CMD_ERASE_CONFIRM);
    timeout = 0;
    do {
    if (++timeout > 1000000) {
    printk(KERN_ERR "nvram: Sharp/Micron flash erase timeout !\n");
    break;
    }
    out_8(base, SM_FLASH_CMD_READ_STATUS);
    stat = in_8(base);
    } while (!(stat & SM_FLASH_STATUS_DONE));
    out_8(base, SM_FLASH_CMD_CLEAR_STATUS);
    out_8(base, SM_FLASH_CMD_RESET);
    if (memchr_inv(base, 0xff, NVRAM_SIZE)) {
    printk(KERN_ERR "nvram: Sharp/Micron flash erase failed !\n");
    return -ENXIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sm_write_bank(bank: c_int, datas: *mut *mut u8) -> c_int {
    static int sm_write_bank(int bank, u8* datas)
    {
    int i, stat = 0;
    unsigned long timeout;
    u8 __iomem *base = (u8 __iomem *)nvram_data + core99_bank*NVRAM_SIZE;
    DBG("nvram: Sharp/Micron Writing bank %d...\n", bank);
    for (i=0; i<NVRAM_SIZE; i++) {
    out_8(base+i, SM_FLASH_CMD_WRITE_SETUP);
    udelay(1);
    out_8(base+i, datas[i]);
    timeout = 0;
    do {
    if (++timeout > 1000000) {
    printk(KERN_ERR "nvram: Sharp/Micron flash write timeout !\n");
    break;
    }
    out_8(base, SM_FLASH_CMD_READ_STATUS);
    stat = in_8(base);
    } while (!(stat & SM_FLASH_STATUS_DONE));
    if (!(stat & SM_FLASH_STATUS_DONE))
    break;
    }
    out_8(base, SM_FLASH_CMD_CLEAR_STATUS);
    out_8(base, SM_FLASH_CMD_RESET);
    if (memcmp(base, datas, NVRAM_SIZE)) {
    printk(KERN_ERR "nvram: Sharp/Micron flash write failed !\n");
    return -ENXIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_erase_bank(bank: c_int) -> c_int {
    static int amd_erase_bank(int bank)
    {
    let mut stat: c_int = 0;
    unsigned long timeout;
    u8 __iomem *base = (u8 __iomem *)nvram_data + core99_bank*NVRAM_SIZE;
    DBG("nvram: AMD Erasing bank %d...\n", bank);
// Unlock 1
    out_8(base+0x555, 0xaa);
    udelay(1);
// Unlock 2
    out_8(base+0x2aa, 0x55);
    udelay(1);
// Sector-Erase
    out_8(base+0x555, 0x80);
    udelay(1);
    out_8(base+0x555, 0xaa);
    udelay(1);
    out_8(base+0x2aa, 0x55);
    udelay(1);
    out_8(base, 0x30);
    udelay(1);
    timeout = 0;
    do {
    if (++timeout > 1000000) {
    printk(KERN_ERR "nvram: AMD flash erase timeout !\n");
    break;
    }
    stat = in_8(base) ^ in_8(base);
    } while (stat != 0);
// Reset
    out_8(base, 0xf0);
    udelay(1);
    if (memchr_inv(base, 0xff, NVRAM_SIZE)) {
    printk(KERN_ERR "nvram: AMD flash erase failed !\n");
    return -ENXIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_write_bank(bank: c_int, datas: *mut *mut u8) -> c_int {
    static int amd_write_bank(int bank, u8* datas)
    {
    int i, stat = 0;
    unsigned long timeout;
    u8 __iomem *base = (u8 __iomem *)nvram_data + core99_bank*NVRAM_SIZE;
    DBG("nvram: AMD Writing bank %d...\n", bank);
    for (i=0; i<NVRAM_SIZE; i++) {
// Unlock 1
    out_8(base+0x555, 0xaa);
    udelay(1);
// Unlock 2
    out_8(base+0x2aa, 0x55);
    udelay(1);
// Write single word
    out_8(base+0x555, 0xa0);
    udelay(1);
    out_8(base+i, datas[i]);
    timeout = 0;
    do {
    if (++timeout > 1000000) {
    printk(KERN_ERR "nvram: AMD flash write timeout !\n");
    break;
    }
    stat = in_8(base) ^ in_8(base);
    } while (stat != 0);
    if (stat != 0)
    break;
    }
// Reset
    out_8(base, 0xf0);
    udelay(1);
    if (memcmp(base, datas, NVRAM_SIZE)) {
    printk(KERN_ERR "nvram: AMD flash write failed !\n");
    return -ENXIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lookup_partitions() -> void __init {
    static void __init lookup_partitions(void)
    {
    u8 buffer[17];
    int i, offset;
    struct chrp_header* hdr;
    if (pmac_newworld) {
    nvram_partitions[pmac_nvram_OF] = -1;
    nvram_partitions[pmac_nvram_XPRAM] = -1;
    nvram_partitions[pmac_nvram_NR] = -1;
    hdr = (struct chrp_header *)buffer;
    offset = 0;
    buffer[16] = 0;
    do {
    for (i=0;i<16;i++)
    buffer[i] = ppc_md.nvram_read_val(offset+i);
    if (!strcmp(hdr.name, "common"))
    nvram_partitions[pmac_nvram_OF] = offset + 0x10;
    if (!strcmp(hdr.name, "APL,MacOS75")) {
    nvram_partitions[pmac_nvram_XPRAM] = offset + 0x10;
    nvram_partitions[pmac_nvram_NR] = offset + 0x110;
    }
    offset += (hdr.len * 0x10);
    } while(offset < NVRAM_SIZE);
    } else {
    nvram_partitions[pmac_nvram_OF] = 0x1800;
    nvram_partitions[pmac_nvram_XPRAM] = 0x1300;
    nvram_partitions[pmac_nvram_NR] = 0x1400;
    }
    DBG("nvram: OF partition at 0x%x\n", nvram_partitions[pmac_nvram_OF]);
    DBG("nvram: XP partition at 0x%x\n", nvram_partitions[pmac_nvram_XPRAM]);
    DBG("nvram: NR partition at 0x%x\n", nvram_partitions[pmac_nvram_NR]);
    }
#[no_mangle]
unsafe extern "C" fn core99_nvram_sync() {
    static void core99_nvram_sync(void)
    {
    struct core99_header* hdr99;
    unsigned long flags;
    if (!is_core_99 || !nvram_data || !nvram_image)
    return;
    raw_spin_lock_irqsave(&nv_lock, flags);
    if (!memcmp(nvram_image, (u8*)nvram_data + core99_bank*NVRAM_SIZE,
    NVRAM_SIZE))
    goto bail;
    DBG("Updating nvram...\n");
    hdr99 = (struct core99_header*)nvram_image;
    hdr99.generation++;
    hdr99.hdr.signature = CORE99_SIGNATURE;
    hdr99.hdr.cksum = chrp_checksum(&hdr99.hdr);
    hdr99.adler = core99_calc_adler(nvram_image);
    core99_bank = core99_bank ? 0 : 1;
    if (core99_erase_bank)
    if (core99_erase_bank(core99_bank)) {
    printk("nvram: Error erasing bank %d\n", core99_bank);
    goto bail;
    }
    if (core99_write_bank)
    if (core99_write_bank(core99_bank, nvram_image))
    printk("nvram: Error writing bank %d\n", core99_bank);
    bail:
    raw_spin_unlock_irqrestore(&nv_lock, flags);

    mdelay(2000);

    }
#[no_mangle]
unsafe extern "C" fn core99_nvram_setup(dp: *mut device_node, addr: c_ulong) -> int __init {
    static int __init core99_nvram_setup(struct device_node *dp, unsigned long addr)
    {
    int i;
    u32 gen_bank0, gen_bank1;
    if (nvram_naddrs < 1) {
    printk(KERN_ERR "nvram: no address\n");
    return -EINVAL;
    }
    nvram_image = memblock_alloc_or_panic(NVRAM_SIZE, SMP_CACHE_BYTES);
    nvram_data = ioremap(addr, NVRAM_SIZE*2);
    nvram_naddrs = 1; /* Make sure we get the correct case */
    DBG("nvram: Checking bank 0...\n");
    gen_bank0 = core99_check((u8 *)nvram_data);
    gen_bank1 = core99_check((u8 *)nvram_data + NVRAM_SIZE);
    core99_bank = (gen_bank0 < gen_bank1) ? 1 : 0;
    DBG("nvram: gen0=%d, gen1=%d\n", gen_bank0, gen_bank1);
    DBG("nvram: Active bank is: %d\n", core99_bank);
    for (i=0; i<NVRAM_SIZE; i++)
    nvram_image[i] = nvram_data[i + core99_bank*NVRAM_SIZE];
    ppc_md.nvram_read_val	= core99_nvram_read_byte;
    ppc_md.nvram_write_val	= core99_nvram_write_byte;
    ppc_md.nvram_read	= core99_nvram_read;
    ppc_md.nvram_write	= core99_nvram_write;
    ppc_md.nvram_size	= core99_nvram_size;
    ppc_md.nvram_sync	= core99_nvram_sync;
    ppc_md.machine_shutdown	= core99_nvram_sync;
//
// Maybe we could be smarter here though making an exclusive list
// of known flash chips is a bit nasty as older OF didn't provide us
// with a useful "compatible" entry. A solution would be to really
// identify the chip using flash id commands and base ourselves on
// a list of known chips IDs
//
    if (of_device_is_compatible(dp, "amd-0137")) {
    core99_erase_bank = amd_erase_bank;
    core99_write_bank = amd_write_bank;
    } else {
    core99_erase_bank = sm_erase_bank;
    core99_write_bank = sm_write_bank;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pmac_nvram_init() -> int __init {
    int __init pmac_nvram_init(void)
    {
    struct device_node *dp;
    struct resource r1, r2;
    let mut s1: c_uint = 0, s2 = 0;
    let mut err: c_int = 0;
    nvram_naddrs = 0;
    dp = of_find_node_by_name(core::ptr::null_mut(), "nvram");
    if (dp == core::ptr::null_mut()) {
    printk(KERN_ERR "Can't find NVRAM device\n");
    return -ENODEV;
    }
// Try to obtain an address
    if (of_address_to_resource(dp, 0, &r1) == 0) {
    nvram_naddrs = 1;
    s1 = resource_size(&r1);
    if (of_address_to_resource(dp, 1, &r2) == 0) {
    nvram_naddrs = 2;
    s2 = resource_size(&r2);
    }
    }
    is_core_99 = of_device_is_compatible(dp, "nvram,flash");
    if (is_core_99) {
    err = core99_nvram_setup(dp, r1.start);
    goto bail;
    }

    if (machine_is(chrp) && nvram_naddrs == 1) {
    nvram_data = ioremap(r1.start, s1);
    nvram_mult = 1;
    ppc_md.nvram_read_val	= direct_nvram_read_byte;
    ppc_md.nvram_write_val	= direct_nvram_write_byte;
    ppc_md.nvram_size	= ppc32_nvram_size;
    } else if (nvram_naddrs == 1) {
    nvram_data = ioremap(r1.start, s1);
    nvram_mult = (s1 + NVRAM_SIZE - 1) / NVRAM_SIZE;
    ppc_md.nvram_read_val	= direct_nvram_read_byte;
    ppc_md.nvram_write_val	= direct_nvram_write_byte;
    ppc_md.nvram_size	= ppc32_nvram_size;
    } else if (nvram_naddrs == 2) {
    nvram_addr = ioremap(r1.start, s1);
    nvram_data = ioremap(r2.start, s2);
    ppc_md.nvram_read_val	= indirect_nvram_read_byte;
    ppc_md.nvram_write_val	= indirect_nvram_write_byte;
    ppc_md.nvram_size	= ppc32_nvram_size;
    } else if (nvram_naddrs == 0 && sys_ctrler == SYS_CTRLER_PMU) {

    nvram_naddrs = -1;
    ppc_md.nvram_read_val	= pmu_nvram_read_byte;
    ppc_md.nvram_write_val	= pmu_nvram_write_byte;
    ppc_md.nvram_size	= ppc32_nvram_size;

    } else {
    printk(KERN_ERR "Incompatible type of NVRAM\n");
    err = -ENXIO;
    }

    bail:
    of_node_put(dp);
    if (err == 0)
    lookup_partitions();
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn pmac_get_partition(partition: c_int) -> c_int {
    int pmac_get_partition(int partition)
    {
    return nvram_partitions[partition];
    }
#[no_mangle]
pub unsafe extern "C" fn pmac_xpram_read(xpaddr: c_int) -> u8 {
    u8 pmac_xpram_read(int xpaddr)
    {
    let mut offset: c_int = pmac_get_partition(pmac_nvram_XPRAM);
    if (offset < 0 || xpaddr < 0 || xpaddr > 0x100)
    return 0xff;
    return ppc_md.nvram_read_val(xpaddr + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn pmac_xpram_write(xpaddr: c_int, data: u8) {
    void pmac_xpram_write(int xpaddr, u8 data)
    {
    let mut offset: c_int = pmac_get_partition(pmac_nvram_XPRAM);
    if (offset < 0 || xpaddr < 0 || xpaddr > 0x100)
    return;
    ppc_md.nvram_write_val(xpaddr + offset, data);
    }
    EXPORT_SYMBOL(pmac_get_partition);
    EXPORT_SYMBOL(pmac_xpram_read);
    EXPORT_SYMBOL(pmac_xpram_write);
