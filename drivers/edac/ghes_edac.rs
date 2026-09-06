//! Automatically rewritten from C to Rust
//! Source: drivers/edac/ghes_edac.c
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
// GHES/EDAC Linux driver
//
// Copyright (c) 2013 by Mauro Carvalho Chehab
//
// Red Hat Inc. https://www.redhat.com
//

pub const OTHER_DETAIL_LEN: c_int = 400;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ghes_pvt {
    pub mci: *mut mem_ctl_info,
// Buffers for the error handling routine
    pub other_detail: [c_char; OTHER_DETAIL_LEN],
    pub msg: [c_char; 80],
}

    let mut ghes_refcount: static refcount_t = REFCOUNT_INIT(0);
//
// Access to ghes_pvt must be protected by ghes_lock. The spinlock
// also provides the necessary (implicit) memory barrier for the SMP
// case to make the pointer visible on another CPU.
//
    static struct ghes_pvt *ghes_pvt;
//
// This driver's representation of the system hardware, as collected
// from DMI.
//
    static struct ghes_hw_desc {
    int num_dimms;
    struct dimm_info *dimms;
    } ghes_hw;
// GHES registration mutex
    static DEFINE_MUTEX(ghes_reg_mutex);
//
// Sync with other, potentially concurrent callers of
// ghes_edac_report_mem_error(). We don't know what the
// "inventive" firmware would do.
//
    static DEFINE_SPINLOCK(ghes_lock);
    static bool system_scanned;
    static struct list_head *ghes_devs;
// Memory Device - Type 17 of SMBIOS spec
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memdev_dmi_entry {
    pub type: u8,
    pub length: u8,
    pub handle: u16,
    pub phys_mem_array_handle: u16,
    pub mem_err_info_handle: u16,
    pub total_width: u16,
    pub data_width: u16,
    pub size: u16,
    pub form_factor: u8,
    pub device_set: u8,
    pub device_locator: u8,
    pub bank_locator: u8,
    pub memory_type: u8,
    pub type_detail: u16,
    pub speed: u16,
    pub manufacturer: u8,
    pub serial_number: u8,
    pub asset_tag: u8,
    pub part_number: u8,
    pub attributes: u8,
    pub extended_size: u32,
    pub conf_mem_clk_speed: u16,
    pub __attribute__((__packed__)): },
    static struct dimm_info *find_dimm_by_handle(struct mem_ctl_info *mci, u16 handle)
    {
    pub dimm: *mut dimm_info,
    mci_for_each_dimm(mci, dimm) {
    if (dimm.smbios_handle == handle)
    pub dimm: return,
    }
    pub NULL: return,
    }
#[no_mangle]
unsafe extern "C" fn dimm_setup_label(dimm: *mut dimm_info, handle: u16) {
    static void dimm_setup_label(struct dimm_info *dimm, u16 handle)
    {
    pub NULL: *const *const *const char bank = NULL, device =,
    pub &device): dmi_memdev_name(handle, &bank,,
//
// Set to a NULL string when both bank and device are zero. In this case,
// the label assigned by default will be preserved.
//
    snprintf(dimm.label, sizeof(dimm.label), "%s%s%s",
    (bank && *bank) ? bank : "",
    (bank && *bank && device && *device) ? " " : "",
    pub ""): *mut *mut (device && device) ? device :,
    }
#[no_mangle]
unsafe extern "C" fn assign_dmi_dimm_info(dimm: *mut dimm_info, entry: *mut memdev_dmi_entry) {
    static void assign_dmi_dimm_info(struct dimm_info *dimm, struct memdev_dmi_entry *entry)
    {
    pub BIT(13): u16 rdr_mask = BIT(7) |,
    if (entry.size == 0xffff) {
    pub dimm->idx): pr_info("Can't get DIMM%i size\n",,
    pub /: *mut *mut dimm->nr_pages = MiB_TO_PAGES(32);/ Unknown,
    } else if (entry.size == 0x7fff) {
    pub MiB_TO_PAGES(entry->extended_size): dimm->nr_pages =,
    } else {
    if (entry.size & BIT(15))
    pub 10): dimm->nr_pages = MiB_TO_PAGES((entry->size & 0x7fff) <<,
    else
    pub MiB_TO_PAGES(entry->size): dimm->nr_pages =,
    }
    switch (entry.memory_type) {
    case 0x12:
    if (entry.type_detail & BIT(13))
    pub MEM_RDDR: dimm->mtype =,
    else
    pub MEM_DDR: dimm->mtype =,
    case 0x13:
    if (entry.type_detail & BIT(13))
    pub MEM_RDDR2: dimm->mtype =,
    else
    pub MEM_DDR2: dimm->mtype =,
    case 0x14:
    pub MEM_FB_DDR2: dimm->mtype =,
    case 0x18:
    if (entry.type_detail & BIT(12))
    pub MEM_NVDIMM: dimm->mtype =,
#[no_mangle]
pub unsafe extern "C" fn if(BIT(13): entry->type_detail &) -> else {
    else if (entry.type_detail & BIT(13))
    pub MEM_RDDR3: dimm->mtype =,
    else
    pub MEM_DDR3: dimm->mtype =,
    case 0x1a:
    if (entry.type_detail & BIT(12))
    pub MEM_NVDIMM: dimm->mtype =,
#[no_mangle]
pub unsafe extern "C" fn if(BIT(13): entry->type_detail &) -> else {
    else if (entry.type_detail & BIT(13))
    pub MEM_RDDR4: dimm->mtype =,
    else
    pub MEM_DDR4: dimm->mtype =,
    default:
    if (entry.type_detail & BIT(6))
    pub MEM_RMBS: dimm->mtype =,
#[no_mangle]
pub unsafe extern "C" fn if(rdr_mask: (entry->type_detail & rdr_mask) ==) -> else {
    else if ((entry.type_detail & rdr_mask) == rdr_mask)
    pub MEM_RDR: dimm->mtype =,
#[no_mangle]
pub unsafe extern "C" fn if(BIT(7): entry->type_detail &) -> else {
    else if (entry.type_detail & BIT(7))
    pub MEM_SDR: dimm->mtype =,
#[no_mangle]
pub unsafe extern "C" fn if(BIT(9): entry->type_detail &) -> else {
    else if (entry.type_detail & BIT(9))
    pub MEM_EDO: dimm->mtype =,
    else
    pub MEM_UNKNOWN: dimm->mtype =,
    }
//
// Actually, we can only detect if the memory has bits for
// checksum or not
//
    if (entry.total_width == entry.data_width)
    pub EDAC_NONE: dimm->edac_mode =,
    else
    pub EDAC_SECDED: dimm->edac_mode =,
    pub DEV_UNKNOWN: dimm->dtype =,
    pub /: *mut *mut dimm->grain = 128; / Likely, worse case,
    pub entry->handle): dimm_setup_label(dimm,,
    if (dimm.nr_pages) {
    edac_dbg(1, "DIMM%i: %s size = %d MB%s\n",
    dimm.idx, edac_mem_types[dimm.mtype],
    PAGES_TO_MiB(dimm.nr_pages),
    pub ""): (dimm->edac_mode != EDAC_NONE) ? "(ECC)" :,
    edac_dbg(2, "\ttype %d, detail 0x%02x, width %d(total %d)\n",
    entry.memory_type, entry.type_detail,
    pub entry->data_width): entry->total_width,,
    }
    pub entry->handle: dimm->smbios_handle =,
    }
#[no_mangle]
unsafe extern "C" fn enumerate_dimms(dh: *const dmi_header, arg: *mut c_void) {
    static void enumerate_dimms(const struct dmi_header *dh, void *arg)
    {
    pub )dh: *mut *mut memdev_dmi_entry entry = (memdev_dmi_entry,
    pub )arg: *mut *mut ghes_hw_desc hw = (ghes_hw_desc,
    pub d: *mut dimm_info,
    if (dh.type != DMI_ENTRY_MEM_DEVICE)
// Enlarge the array with additional 16
    if (!hw.num_dimms || !(hw.num_dimms % 16)) {
    pub new: *mut dimm_info,
    new = krealloc_array(hw.dimms, hw.num_dimms + 16,
    pub GFP_KERNEL): sizeof(struct dimm_info),,
    if (!new) {
    }
    pub new: hw->dimms =,
    }
    pub &hw->dimms[hw->num_dimms]: d =,
    pub hw->num_dimms: d->idx =,
    pub entry): assign_dmi_dimm_info(d,,
    }
#[no_mangle]
unsafe extern "C" fn ghes_scan_system() {
    static void ghes_scan_system(void)
    {
    if (system_scanned)
    pub &ghes_hw): dmi_walk(enumerate_dimms,,
    pub true: system_scanned =,
    }
    static int print_mem_error_other_detail(const struct cper_sec_mem_err *mem, char *msg,
    const char *location, unsigned int len)
    {
    pub n: u32,
    if (!msg)
    pub 0: return,
    pub 0: n =,
    pub 1: len -=,
    pub location): n += scnprintf(msg + n, len - n, "APEI location: %s ",,
    if (!(mem.validation_bits & CPER_MEM_VALID_ERROR_STATUS))
    pub out: goto,
    pub mem->error_status): n += scnprintf(msg + n, len - n, "status(0x%016llx): ",,
    pub cper_mem_err_status_str(mem->error_status)): n += scnprintf(msg + n, len - n, "%s ",,
    out:
    pub '\0': msg[n] =,
    pub n: return,
    }
    static int ghes_edac_report_mem_error(struct notifier_block *nb,
    unsigned long val, void *data)
    {
    pub )data: *mut *mut cper_sec_mem_err mem_err = (cper_sec_mem_err,
    pub cmem: cper_mem_err_compact,
    pub e: *mut edac_raw_error_desc,
    pub mci: *mut mem_ctl_info,
    pub val: unsigned long sev =,
    pub pvt: *mut ghes_pvt,
    pub flags: c_ulong,
    pub p: *mut c_char,
//
// We can do the locking below because GHES defers error processing
// from NMI to IRQ context. Whenever that changes, we'd at least
// know.
//
    if (WARN_ON_ONCE(in_nmi()))
    pub NOTIFY_OK: return,
    pub flags): spin_lock_irqsave(&ghes_lock,,
    pub ghes_pvt: pvt =,
    if (!pvt)
    pub unlock: goto,
    pub pvt->mci: mci =,
    pub &mci->error_desc: e =,
// Cleans the error report buffer
    pub (*e)): *mut memset(e, 0, sizeof,
    pub 1: e->error_count =,
    pub 1: e->grain =,
    pub pvt->msg: e->msg =,
    pub pvt->other_detail: e->other_detail =,
    pub -1: e->top_layer =,
    pub -1: e->mid_layer =,
    pub -1: e->low_layer =,
// pvt->other_detail = '\0';
// pvt->msg = '\0';
    switch (sev) {
    case GHES_SEV_CORRECTED:
    pub HW_EVENT_ERR_CORRECTED: e->type =,
    case GHES_SEV_RECOVERABLE:
    pub HW_EVENT_ERR_UNCORRECTED: e->type =,
    case GHES_SEV_PANIC:
    pub HW_EVENT_ERR_FATAL: e->type =,
    default:
    case GHES_SEV_NO:
    pub HW_EVENT_ERR_INFO: e->type =,
    }
    edac_dbg(1, "error validation_bits: 0x%08llx\n",
    pub long)mem_err->validation_bits): (long,
// Error type, mapped on e->msg
    if (mem_err.validation_bits & CPER_MEM_VALID_ERROR_TYPE) {
    pub mem_err->error_type: u8 etype =,
    pub pvt->msg: p =,
    pub cper_mem_err_type_str(etype)): p += snprintf(p, sizeof(pvt->msg), "%s",,
    } else {
    pub error"): strscpy(pvt->msg, "unknown,
    }
// Error address
    if (mem_err.validation_bits & CPER_MEM_VALID_PA) {
    pub PHYS_PFN(mem_err->physical_addr): e->page_frame_number =,
    pub offset_in_page(mem_err->physical_addr): e->offset_in_page =,
    }
// Error grain
    if (mem_err.validation_bits & CPER_MEM_VALID_PA_MASK)
    pub 1: e->grain = ~mem_err->physical_addr_mask +,
// Memory error location, mapped on e->location
    pub e->location: p =,
    pub &cmem): cper_mem_err_pack(mem_err,,
    pub p): p += cper_mem_err_location(&cmem,,
    if (mem_err.validation_bits & CPER_MEM_VALID_MODULE_HANDLE) {
    pub dimm: *mut dimm_info,
    pub p): p += cper_dimm_err_location(&cmem,,
    pub mem_err->mem_dev_handle): dimm = find_dimm_by_handle(mci,,
    if (dimm) {
    pub dimm->idx: e->top_layer =,
    pub dimm->label): strscpy(e->label,,
    }
    }
    if (p > e.location)
// (p - 1) = '\0';
    if (!*e.label)
    pub memory"): strscpy(e->label, "unknown,
// All other fields are mapped on e->other_detail
    pub pvt->other_detail: p =,
    pub OTHER_DETAIL_LEN): p += print_mem_error_other_detail(mem_err, p, e->location,,
    if (p > pvt.other_detail)
// (p - 1) = '\0';
    unlock:
    pub flags): spin_unlock_irqrestore(&ghes_lock,,
    pub NOTIFY_OK: return,
    }
    static struct notifier_block ghes_edac_mem_err_nb = {
    .notifier_call	= ghes_edac_report_mem_error,
    .priority	= 0,
}

#[no_mangle]
unsafe extern "C" fn ghes_edac_register(dev: *mut device) -> c_int {
    static int ghes_edac_register(struct device *dev)
    {
    let mut fake: bool = false;
    struct mem_ctl_info *mci;
    struct ghes_pvt *pvt;
    struct edac_mc_layer layers[1];
    unsigned long flags;
    let mut rc: c_int = 0;
// finish another registration/unregistration instance first
    mutex_lock(&ghes_reg_mutex);
//
// We have only one logical memory controller to which all DIMMs belong.
//
    if (refcount_inc_not_zero(&ghes_refcount))
    goto unlock;
    ghes_scan_system();
// Check if we've got a bogus BIOS
    if (!ghes_hw.num_dimms) {
    fake = true;
    ghes_hw.num_dimms = 1;
    }
    layers[0].type = EDAC_MC_LAYER_ALL_MEM;
    layers[0].size = ghes_hw.num_dimms;
    layers[0].is_virt_csrow = true;
    mci = edac_mc_alloc(0, ARRAY_SIZE(layers), layers, sizeof(struct ghes_pvt));
    if (!mci) {
    pr_info("Can't allocate memory for EDAC data\n");
    rc = -ENOMEM;
    goto unlock;
    }
    pvt		= mci.pvt_info;
    pvt.mci	= mci;
    mci.pdev = dev;
    mci.mtype_cap = MEM_FLAG_EMPTY;
    mci.edac_ctl_cap = EDAC_FLAG_NONE;
    mci.edac_cap = EDAC_FLAG_NONE;
    mci.mod_name = "ghes_edac.c";
    mci.ctl_name = "ghes_edac";
    mci.dev_name = "ghes";
    if (fake) {
    pr_info("This system has a very crappy BIOS: It doesn't even list the DIMMS.\n");
    pr_info("Its SMBIOS info is wrong. It is doubtful that the error report would\n");
    pr_info("work on such system. Use this driver with caution\n");
    }
    pr_info("This system has %d DIMM sockets.\n", ghes_hw.num_dimms);
    if (!fake) {
    struct dimm_info *src, *dst;
    let mut i: c_int = 0;
    mci_for_each_dimm(mci, dst) {
    src = &ghes_hw.dimms[i];
    dst.idx	   = src.idx;
    dst.smbios_handle = src.smbios_handle;
    dst.nr_pages	   = src.nr_pages;
    dst.mtype	   = src.mtype;
    dst.edac_mode	   = src.edac_mode;
    dst.dtype	   = src.dtype;
    dst.grain	   = src.grain;
//
// If no src->label, preserve default label assigned
// from EDAC core.
//
    if (strlen(src.label))
    memcpy(dst.label, src.label, sizeof(src.label));
    i++;
    }
    } else {
    struct dimm_info *dimm = edac_get_dimm(mci, 0, 0, 0);
    dimm.nr_pages = 1;
    dimm.grain = 128;
    dimm.mtype = MEM_UNKNOWN;
    dimm.dtype = DEV_UNKNOWN;
    dimm.edac_mode = EDAC_SECDED;
    }
    rc = edac_mc_add_mc(mci);
    if (rc < 0) {
    pr_info("Can't register with the EDAC core\n");
    edac_mc_free(mci);
    rc = -ENODEV;
    goto unlock;
    }
    spin_lock_irqsave(&ghes_lock, flags);
    ghes_pvt = pvt;
    spin_unlock_irqrestore(&ghes_lock, flags);
    ghes_register_report_chain(&ghes_edac_mem_err_nb);
// only set on success
    refcount_set(&ghes_refcount, 1);
    unlock:
// Not needed anymore
    kfree(ghes_hw.dimms);
    ghes_hw.dimms = core::ptr::null_mut();
    mutex_unlock(&ghes_reg_mutex);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ghes_edac_unregister(ghes: *mut ghes) {
    static void ghes_edac_unregister(struct ghes *ghes)
    {
    struct mem_ctl_info *mci;
    unsigned long flags;
    mutex_lock(&ghes_reg_mutex);
    system_scanned = false;
    memset(&ghes_hw, 0, sizeof(struct ghes_hw_desc));
    if (!refcount_dec_and_test(&ghes_refcount))
    goto unlock;
//
// Wait for the irq handler being finished.
//
    spin_lock_irqsave(&ghes_lock, flags);
    mci = ghes_pvt ? ghes_pvt.mci : core::ptr::null_mut();
    ghes_pvt = core::ptr::null_mut();
    spin_unlock_irqrestore(&ghes_lock, flags);
    if (!mci)
    goto unlock;
    mci = edac_mc_del_mc(mci.pdev);
    if (mci)
    edac_mc_free(mci);
    ghes_unregister_report_chain(&ghes_edac_mem_err_nb);
    unlock:
    mutex_unlock(&ghes_reg_mutex);
    }
#[no_mangle]
unsafe extern "C" fn ghes_edac_init() -> int __init {
    static int __init ghes_edac_init(void)
    {
    struct ghes *g, *g_tmp;
    ghes_devs = ghes_get_devices();
    if (!ghes_devs)
    return -ENODEV;
    if (list_empty(ghes_devs)) {
    pr_info("GHES probing device list is empty\n");
    return -ENODEV;
    }
    list_for_each_entry_safe(g, g_tmp, ghes_devs, elist) {
    ghes_edac_register(g.dev);
    }
    return 0;
    }
    module_init(ghes_edac_init);
#[no_mangle]
unsafe extern "C" fn ghes_edac_exit() -> void __exit {
    static void __exit ghes_edac_exit(void)
    {
    struct ghes *g, *g_tmp;
    list_for_each_entry_safe(g, g_tmp, ghes_devs, elist) {
    ghes_edac_unregister(g);
    }
    }
    module_exit(ghes_edac_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Output ACPI APEI/GHES BIOS detected errors via EDAC");
