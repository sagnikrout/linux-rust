//! Automatically rewritten from C to Rust
//! Source: drivers/s390/char/sclp_mem.c
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
// Memory hotplug support via sclp
//
// Copyright IBM Corp. 2025
//

pub const SCLP_CMDW_ASSIGN_STORAGE: c_uint = 0x000d0001;
pub const SCLP_CMDW_UNASSIGN_STORAGE: c_uint = 0x000c0001;
    static LIST_HEAD(sclp_mem_list);
    static u8 sclp_max_storage_id;
    static DECLARE_BITMAP(sclp_storage_ids, 256);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_increment {
    pub list: list_head,
    pub rn: u16,
    pub standby: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclp_mem {
    pub kobj: kobject,
    pub id: c_uint,
    pub memmap_on_memory: c_uint,
    pub config: c_uint,

    pub early_shadow_mapped: c_uint,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclp_mem_arg {
    pub sclp_mems: *mut sclp_mem,
    pub kset: *mut kset,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct assign_storage_sccb {
    pub header: sccb_header,
    pub rn: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct attach_storage_sccb {
    pub header: sccb_header,
    pub :16: u16,
    pub assigned: u16,
    pub :32: u32,
    pub entries: [u32; ],
    pub __packed: },
#[no_mangle]
pub unsafe extern "C" fn arch_get_memory_phys_device(start_pfn: c_ulong) -> c_int {
    int arch_get_memory_phys_device(unsigned long start_pfn)
    {
    if (!sclp.rzm)
    pub 0: return,
    pub ilog2(sclp.rzm): return PFN_PHYS(start_pfn) >>,
    }
#[no_mangle]
unsafe extern "C" fn rn2addr(rn: u16) -> c_ulong {
    static unsigned long rn2addr(u16 rn)
    {
    pub sclp.rzm: *mut *mut return (unsigned long)(rn - 1),
    }
#[no_mangle]
unsafe extern "C" fn do_assign_storage(cmd: sclp_cmdw_t, rn: u16) -> c_int {
    static int do_assign_storage(sclp_cmdw_t cmd, u16 rn)
    {
    pub sccb: *mut assign_storage_sccb,
    pub rc: c_int,
    pub GFP_DMA): *mut *mut sccb = (void )get_zeroed_page(GFP_KERNEL |,
    if (!sccb)
    pub -ENOMEM: return,
    pub PAGE_SIZE: sccb->header.length =,
    pub rn: sccb->rn =,
    pub SCLP_QUEUE_INTERVAL): rc = sclp_sync_request_timeout(cmd, sccb,,
    if (rc)
    pub out: goto,
    switch (sccb.header.response_code) {
    case 0x0020:
    case 0x0120:
    default:
    pr_warn("assign storage failed (cmd=0x%08x, response=0x%04x, rn=0x%04x)\n",
    pub rn): cmd, sccb->header.response_code,,
    pub -EIO: rc =,
    }
    out:
    pub long)sccb): free_page((unsigned,
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn sclp_assign_storage(rn: u16) -> c_int {
    static int sclp_assign_storage(u16 rn)
    {
    pub start: c_ulong,
    pub rc: c_int,
    pub rn): rc = do_assign_storage(SCLP_CMDW_ASSIGN_STORAGE,,
    if (rc)
    pub rc: return,
    pub rn2addr(rn): start =,
    pub sclp.rzm): storage_key_init_range(start, start +,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn sclp_unassign_storage(rn: u16) -> c_int {
    static int sclp_unassign_storage(u16 rn)
    {
    pub rn): return do_assign_storage(SCLP_CMDW_UNASSIGN_STORAGE,,
    }
#[no_mangle]
unsafe extern "C" fn sclp_attach_storage(id: u8) -> c_int {
    static int sclp_attach_storage(u8 id)
    {
    pub sccb: *mut attach_storage_sccb,
    pub i: int rc,,
    pub GFP_DMA): *mut *mut sccb = (void )get_zeroed_page(GFP_KERNEL |,
    if (!sccb)
    pub -ENOMEM: return,
    pub PAGE_SIZE: sccb->header.length =,
    pub 0x40: sccb->header.function_code =,
    rc = sclp_sync_request_timeout(0x00080001 | id << 8, sccb,
    if (rc)
    pub out: goto,
    switch (sccb.header.response_code) {
    case 0x0020:
    pub sclp_storage_ids): set_bit(id,,
    pub {: for (i = 0; i < sccb->assigned; i++),
    if (sccb.entries[i])
    pub 16): sclp_unassign_storage(sccb->entries[i] >>,
    }
    default:
    pub -EIO: rc =,
    }
    out:
    pub long)sccb): free_page((unsigned,
    pub rc: return,
    }
    static int sclp_mem_change_state(unsigned long start, unsigned long size,
    int online)
    {
    pub incr: *mut memory_increment,
    pub istart: c_ulong,
    pub 0: int rc =,
    list_for_each_entry(incr, &sclp_mem_list, list) {
    pub rn2addr(incr->rn): istart =,
    if (start + size - 1 < istart)
    if (start > istart + sclp.rzm - 1)
    if (online)
    pub sclp_assign_storage(incr->rn): rc |=,
    else
    if (rc == 0)
    pub 1: incr->standby = online ? 0 :,
    }
    pub 0: return rc ? -EIO :,
    }
#[no_mangle]
unsafe extern "C" fn sclp_config_mem_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    static ssize_t sclp_config_mem_show(struct kobject *kobj, struct kobj_attribute *attr, char *buf)
    {
    pub kobj): *mut *mut sclp_mem sclp_mem = container_of(kobj, sclp_mem,,
    pub READ_ONCE(sclp_mem->config)): return sysfs_emit(buf, "%u\n",,
    }
    static ssize_t sclp_config_mem_store(struct kobject *kobj, struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    pub block_size: unsigned long addr,,
    pub sclp_mem: *mut sclp_mem,
    pub mem: *mut memory_block,
    pub id: c_uchar,
    pub value: bool,
    pub rc: c_int,
    pub &value): rc = kstrtobool(buf,,
    if (rc)
    pub rc: return,
    pub kobj): sclp_mem = container_of(kobj, struct sclp_mem,,
    pub memory_block_size_bytes(): block_size =,
    pub block_size: *mut *mut addr = sclp_mem->id,
//
// Hold device_hotplug_lock when adding/removing memory blocks.
// Additionally, also protect calls to memory_block_get() and
// sclp_attach_storage().
//
    pub lock_device_hotplug_sysfs(): rc =,
    if (rc)
    pub out: goto,
    for_each_clear_bit(id, sclp_storage_ids, sclp_max_storage_id + 1)
    if (value) {
    if (sclp_mem.config)
    pub out_unlock: goto,
    pub 1): rc = sclp_mem_change_state(addr, block_size,,
    if (rc)
    pub out_unlock: goto,
//
// Set entire memory block CMMA state to nodat. Later, when
// page tables pages are allocated via __add_memory(), those
// regions are marked __arch_set_page_dat().
//
    pub PAGE_SHIFT): *mut *mut __arch_set_page_nodat((void )__va(addr), block_size >>,
    rc = __add_memory(0, addr, block_size,
    sclp_mem.memmap_on_memory ?
    pub MHP_NONE): MHP_MEMMAP_ON_MEMORY :,
    if (rc) {
    pub 0): sclp_mem_change_state(addr, block_size,,
    pub out_unlock: goto,
    }
    pub memory_block_get(phys_to_block_id(addr)): mem =,
    pub 1): WRITE_ONCE(sclp_mem->config,,
    } else {
    if (!sclp_mem.config)
    pub out_unlock: goto,
    pub memory_block_get(phys_to_block_id(addr)): mem =,
    if (mem.state != MEM_OFFLINE) {
    pub -EBUSY: rc =,
    pub out_unlock: goto,
    }
    pub 0): sclp_mem_change_state(addr, block_size,,
    pub block_size): __remove_memory(addr,,

    if (sclp_mem.early_shadow_mapped) {
    pub end: unsigned long start,,
    pub long)kasan_mem_to_shadow(__va(addr)): start = (unsigned,
    pub KASAN_SHADOW_SCALE_SHIFT): end = start + (block_size >>,
    pub NULL): vmemmap_free(start, end,,
    pub 0: sclp_mem->early_shadow_mapped =,
    }

    pub 0): WRITE_ONCE(sclp_mem->config,,
    }
    out_unlock:
    out:
    pub count: return rc ? rc :,
    }
    static struct kobj_attribute sclp_config_mem_attr =
    pub sclp_config_mem_store): __ATTR(config, 0644, sclp_config_mem_show,,
#[no_mangle]
unsafe extern "C" fn sclp_memmap_on_memory_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    static ssize_t sclp_memmap_on_memory_show(struct kobject *kobj, struct kobj_attribute *attr, char *buf)
    {
    pub kobj): *mut *mut sclp_mem sclp_mem = container_of(kobj, sclp_mem,,
    pub READ_ONCE(sclp_mem->memmap_on_memory)): return sysfs_emit(buf, "%u\n",,
    }
    static ssize_t sclp_memmap_on_memory_store(struct kobject *kobj, struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    pub sclp_mem: *mut sclp_mem,
    pub block_size: c_ulong,
    pub mem: *mut memory_block,
    pub value: bool,
    pub rc: c_int,
    pub &value): rc = kstrtobool(buf,,
    if (rc)
    pub rc: return,
    if (value && !mhp_supports_memmap_on_memory())
    pub -EOPNOTSUPP: return,
    pub lock_device_hotplug_sysfs(): rc =,
    if (rc)
    pub rc: return,
    pub memory_block_size_bytes(): block_size =,
    pub kobj): sclp_mem = container_of(kobj, struct sclp_mem,,
    pub block_size)): *mut *mut mem = memory_block_get(phys_to_block_id(sclp_mem->id,
    if (!mem) {
    pub value): WRITE_ONCE(sclp_mem->memmap_on_memory,,
    } else {
    pub -EBUSY: rc =,
    }
    pub count: return rc ? rc :,
    }
    static const struct kobj_type ktype = {
    .sysfs_ops = &kobj_sysfs_ops,
}

    static struct kobj_attribute sclp_memmap_attr =
    __ATTR(memmap_on_memory, 0644, sclp_memmap_on_memory_show, sclp_memmap_on_memory_store);
    static struct attribute *sclp_mem_attrs[] = {
    &sclp_config_mem_attr.attr,
    &sclp_memmap_attr.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group sclp_mem_attr_group = {
    .attrs = sclp_mem_attrs,
    };
    static int sclp_create_mem(struct sclp_mem *sclp_mem, struct kset *kset,
    unsigned int id, bool config, bool memmap_on_memory)
    {
    int rc;
    sclp_mem.memmap_on_memory = memmap_on_memory;
    sclp_mem.config = config;

    sclp_mem.early_shadow_mapped = config;

    sclp_mem.id = id;
    kobject_init(&sclp_mem.kobj, &ktype);
    rc = kobject_add(&sclp_mem.kobj, &kset.kobj, "memory%d", id);
    if (rc)
    return rc;
    return sysfs_create_group(&sclp_mem.kobj, &sclp_mem_attr_group);
    }
#[no_mangle]
unsafe extern "C" fn sclp_create_configured_mem(mem: *mut memory_block, argument: *mut c_void) -> c_int {
    static int sclp_create_configured_mem(struct memory_block *mem, void *argument)
    {
    struct sclp_mem *sclp_mems;
    struct sclp_mem_arg *arg;
    struct kset *kset;
    unsigned int id;
    id = mem.dev.id;
    arg = (struct sclp_mem_arg *)argument;
    sclp_mems = arg.sclp_mems;
    kset = arg.kset;
    return sclp_create_mem(&sclp_mems[id], kset, id, true, false);
    }
    static void __init align_to_block_size(unsigned long *start,
    unsigned long *size,
    unsigned long alignment)
    {
    unsigned long start_align, size_align;
    start_align = roundup(*start, alignment);
    size_align = rounddown(*start + *size, alignment) - start_align;
    pr_info("Standby memory at 0x%lx (%luM of %luM usable)\n",
// start, size_align >> 20, *size >> 20);
// start = start_align;
// size = size_align;
    }
    static int __init sclp_create_standby_mems_merged(struct sclp_mem *sclp_mems,
    struct kset *kset, u16 rn)
    {
    unsigned long start, size, addr, block_size;
    static u16 first_rn, num;
    unsigned int id;
    let mut rc: c_int = 0;
    if (rn && first_rn && (first_rn + num == rn)) {
    num++;
    return rc;
    }
    if (!first_rn)
    goto skip_add;
    start = rn2addr(first_rn);
    size = (unsigned long)num * sclp.rzm;
    if (start >= ident_map_size)
    goto skip_add;
    if (start + size > ident_map_size)
    size = ident_map_size - start;
    block_size = memory_block_size_bytes();
    align_to_block_size(&start, &size, block_size);
    if (!size)
    goto skip_add;
    for (addr = start; addr < start + size; addr += block_size) {
    id = addr / block_size;
    rc = sclp_create_mem(&sclp_mems[id], kset, id, false,
    mhp_supports_memmap_on_memory());
    if (rc)
    break;
    }
    skip_add:
    first_rn = rn;
    num = 1;
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn sclp_create_standby_mems(sclp_mems: *mut sclp_mem, kset: *mut kset) -> int __init {
    static int __init sclp_create_standby_mems(struct sclp_mem *sclp_mems, struct kset *kset)
    {
    struct memory_increment *incr;
    let mut rc: c_int = 0;
    list_for_each_entry(incr, &sclp_mem_list, list) {
    if (incr.standby)
    rc = sclp_create_standby_mems_merged(sclp_mems, kset, incr.rn);
    if (rc)
    return rc;
    }
    return sclp_create_standby_mems_merged(sclp_mems, kset, 0);
    }
#[no_mangle]
unsafe extern "C" fn sclp_init_mem() -> int __init {
    static int __init sclp_init_mem(void)
    {
    let mut block_size: c_ulong = memory_block_size_bytes();
    unsigned int max_sclp_mems;
    struct sclp_mem *sclp_mems;
    struct sclp_mem_arg arg;
    struct kset *kset;
    int rc;
    max_sclp_mems = roundup(sclp.rnmax * sclp.rzm, block_size) / block_size;
// Allocate memory for all blocks ahead of time.
    sclp_mems = kzalloc_objs(struct sclp_mem, max_sclp_mems);
    if (!sclp_mems)
    return -ENOMEM;
    kset = kset_create_and_add("memory", core::ptr::null_mut(), firmware_kobj);
    if (!kset)
    return -ENOMEM;
// Initial memory is in the "configured" state already.
    arg.sclp_mems = sclp_mems;
    arg.kset = kset;
    rc = for_each_memory_block(&arg, sclp_create_configured_mem);
    if (rc)
    return rc;
// Standby memory is "deconfigured".
    return sclp_create_standby_mems(sclp_mems, kset);
    }
#[no_mangle]
unsafe extern "C" fn insert_increment(rn: u16, standby: c_int, assigned: c_int) -> void __init {
    static void __init insert_increment(u16 rn, int standby, int assigned)
    {
    struct memory_increment *incr, *new_incr;
    struct list_head *prev;
    u16 last_rn;
    new_incr = kzalloc_obj(*new_incr);
    if (!new_incr)
    return;
    new_incr.rn = rn;
    new_incr.standby = standby;
    last_rn = 0;
    prev = &sclp_mem_list;
    list_for_each_entry(incr, &sclp_mem_list, list) {
    if (assigned && incr.rn > rn)
    break;
    if (!assigned && incr.rn - last_rn > 1)
    break;
    last_rn = incr.rn;
    prev = &incr.list;
    }
    if (!assigned)
    new_incr.rn = last_rn + 1;
    if (new_incr.rn > sclp.rnmax) {
    kfree(new_incr);
    return;
    }
    list_add(&new_incr.list, prev);
    }
#[no_mangle]
unsafe extern "C" fn sclp_setup_memory() -> int __init {
    static int __init sclp_setup_memory(void)
    {
    struct read_storage_sccb *sccb;
    int i, id, assigned, rc;
// No standby memory in kdump mode
    if (oldmem_data.start)
    return 0;
    if ((sclp.facilities & 0xe00000000000UL) != 0xe00000000000UL)
    return 0;
    rc = -ENOMEM;
    sccb = (void *)__get_free_page(GFP_KERNEL | GFP_DMA);
    if (!sccb)
    goto out;
    assigned = 0;
    for (id = 0; id <= sclp_max_storage_id; id++) {
    memset(sccb, 0, PAGE_SIZE);
    sccb.header.length = PAGE_SIZE;
    rc = sclp_sync_request(SCLP_CMDW_READ_STORAGE_INFO | id << 8, sccb);
    if (rc)
    goto out;
    switch (sccb.header.response_code) {
    case 0x0010:
    set_bit(id, sclp_storage_ids);
    for (i = 0; i < sccb.assigned; i++) {
    if (!sccb.entries[i])
    continue;
    assigned++;
    insert_increment(sccb.entries[i] >> 16, 0, 1);
    }
    break;
    case 0x0310:
    break;
    case 0x0410:
    for (i = 0; i < sccb.assigned; i++) {
    if (!sccb.entries[i])
    continue;
    assigned++;
    insert_increment(sccb.entries[i] >> 16, 1, 1);
    }
    break;
    default:
    rc = -EIO;
    break;
    }
    if (!rc)
    sclp_max_storage_id = sccb.max_id;
    }
    if (rc || list_empty(&sclp_mem_list))
    goto out;
    for (i = 1; i <= sclp.rnmax - assigned; i++)
    insert_increment(0, 1, 0);
    rc = sclp_init_mem();
    out:
    free_page((unsigned long)sccb);
    return rc;
    }
    __initcall(sclp_setup_memory);
