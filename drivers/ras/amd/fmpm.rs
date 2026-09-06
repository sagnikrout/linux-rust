//! Automatically rewritten from C to Rust
//! Source: drivers/ras/amd/fmpm.c
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
// FRU (Field-Replaceable Unit) Memory Poison Manager
//
// Copyright (c) 2024, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Authors:
// Naveen Krishna Chatradhi <naveenkrishna.chatradhi@amd.com>
// Muralidhara M K <muralidhara.mk@amd.com>
// Yazen Ghannam <Yazen.Ghannam@amd.com>
//
// Implementation notes, assumptions, and limitations:
//
// - FRU memory poison section and memory poison descriptor definitions are not yet
// included in the UEFI specification. So they are defined here. Afterwards, they
// may be moved to linux/cper.h, if appropriate.
//
// - Platforms based on AMD MI300 systems will be the first to use these structures.
// There are a number of assumptions made here that will need to be generalized
// to support other platforms.
//
// AMD MI300-based platform(s) assumptions:
// - Memory errors are reported through x86 MCA.
// - The entire DRAM row containing a memory error should be retired.
// - There will be (1) FRU memory poison section per CPER.
// - The FRU will be the CPU package (processor socket).
// - The default number of memory poison descriptor entries should be (8).
// - The platform will use ACPI ERST for persistent storage.
// - All FRU records should be saved to persistent storage. Module init will
// fail if any FRU record is not successfully written.
//
// - Boot time memory retirement may occur later than ideal due to dependencies
// on other libraries and drivers. This leaves a gap where bad memory may be
// accessed during early boot stages.
//
// - Enough memory should be pre-allocated for each FRU record to be able to hold
// the expected number of descriptor entries. This, mostly empty, record is
// written to storage during init time. Subsequent writes to the same record
// should allow the Platform to update the stored record in-place. Otherwise,
// if the record is extended, then the Platform may need to perform costly memory
// management operations on the storage. For example, the Platform may spend time
// in Firmware copying and invalidating memory on a relatively slow SPI ROM.
//

// Validation Bits

// FRU Architecture Types
pub const FMP_ARCH_TYPE_X86_CPUID_1_EAX: c_int = 0;
// FRU ID Types
pub const FMP_ID_TYPE_X86_PPIN: c_int = 0;
// FRU Memory Poison Section
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_fru_mem_poison {
    pub checksum: u32,
    pub validation_bits: u64,
    pub fru_arch_type: u32,
    pub fru_arch: u64,
    pub fru_id_type: u32,
    pub fru_id: u64,
    pub nr_entries: u32,
    pub __packed: },
// FRU Descriptor ID Types
pub const FPD_HW_ID_TYPE_MCA_IPID: c_int = 0;
// FRU Descriptor Address Types
pub const FPD_ADDR_TYPE_MCA_ADDR: c_int = 0;
// Memory Poison Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_fru_poison_desc {
    pub timestamp: u64,
    pub hw_id_type: u32,
    pub hw_id: u64,
    pub addr_type: u32,
    pub addr: u64,
    pub __packed: },
// Collection of headers and sections for easy pointer use.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fru_rec {
    pub hdr: cper_record_header,
    pub sec_desc: cper_section_descriptor,
    pub fmp: cper_sec_fru_mem_poison,
    pub entries: [cper_fru_poison_desc; ],
    pub __packed: },
//
// Pointers to the complete CPER record of each FRU.
//
// Memory allocation will include padded space for descriptor entries.
//
    pub fru_records: *mut static struct fru_rec,
// system physical addresses array
    pub spa_entries: *mut static u64,
    pub fmpm_dfs_dir: *mut static struct dentry,
    pub fmpm_dfs_entries: *mut static struct dentry,

    GUID_INIT(0xcd5c2993, 0xf4b2, 0x41b2, 0xb5, 0xd4, 0xf9, 0xc3,	\
    0xa0, 0x33, 0x08, 0x75)

    GUID_INIT(0x5e4706c1, 0x5356, 0x48c6, 0x93, 0x0b, 0x52, 0xf2,	\
    0x12, 0x0a, 0x44, 0x58)
//
// DOC: max_nr_entries (byte)
// Maximum number of descriptor entries possible for each FRU.
//
// Values between '1' and '255' are valid.
// No input or '0' will default to FMPM_DEFAULT_MAX_NR_ENTRIES.
//
    pub max_nr_entries: static u8,
    pub 0644): module_param(max_nr_entries, byte,,
    MODULE_PARM_DESC(max_nr_entries,
    pub FRU"): "Maximum number of memory poison descriptor entries per,
pub const FMPM_DEFAULT_MAX_NR_ENTRIES: c_int = 8;
// Maximum number of FRUs in the system.
pub const FMPM_MAX_NR_FRU: c_int = 256;
    pub max_nr_fru: static unsigned int,
// Total length of record including headers and list of descriptor entries.
    pub max_rec_len: static size_t,

// Total number of SPA entries across all FRUs.
    pub spa_nr_entries: static unsigned int,
//
// Protect the local records cache in fru_records and prevent concurrent
// writes to storage. This is only needed after init once notifier block
// registration is done.
//
// The majority of a record is fixed at module init and will not change
// during run time. The entries within a record will be updated as new
// errors are reported. The mutex should be held whenever the entries are
// accessed during run time.
//
    pub DEFINE_MUTEX(fmpm_update_mutex): static,

    pub i++): for (i = 0; rec = fru_records[i], i < max_nr_fru;,
#[no_mangle]
pub unsafe extern "C" fn get_fmp_len(rec: *mut fru_rec) -> u32 {
    static inline u32 get_fmp_len(struct fru_rec *rec)
    {
    pub cper_section_descriptor): return rec->sec_desc.section_length - sizeof(struct,
    }
    static struct fru_rec *get_fru_record(u64 fru_id)
    {
    pub rec: *mut fru_rec,
    pub i: c_uint,
    for_each_fru(i, rec) {
    if (rec.fmp.fru_id == fru_id)
    pub rec: return,
    }
    pub fru_id): pr_debug("Record not found for FRU 0x%016llx\n",,
    pub NULL: return,
    }
//
// Sum up all bytes within the FRU Memory Poison Section including the Memory
// Poison Descriptor entries.
//
// Don't include the old checksum here. It's a u32 value, so summing each of its
// bytes will give the wrong total.
//
#[no_mangle]
unsafe extern "C" fn do_fmp_checksum(fmp: *mut cper_sec_fru_mem_poison, len: u32) -> u32 {
    static u32 do_fmp_checksum(struct cper_sec_fru_mem_poison *fmp, u32 len)
    {
    pub 0: u32 checksum =,
    pub end: *mut *mut u8 buf,,
// Skip old checksum.
    pub sizeof(u32): *mut *mut buf = (u8 )fmp +,
    pub len: end = buf +,
    while (buf < end)
    pub (u8)(*(buf++)): *mut checksum +=,
    pub checksum: return,
    }
#[no_mangle]
unsafe extern "C" fn update_record_on_storage(rec: *mut fru_rec) -> c_int {
    static int update_record_on_storage(struct fru_rec *rec)
    {
    pub checksum: u32 len,,
    pub ret: c_int,
// Calculate a new checksum.
    pub get_fmp_len(rec): len =,
// Get the current total.
    pub len): checksum = do_fmp_checksum(&rec->fmp,,
// Use the complement value.
    pub -checksum: rec->fmp.checksum =,
    pub storage\n"): pr_debug("Writing to,
    pub erst_write(&rec->hdr): ret =,
    if (ret) {
    pub rec->fmp.fru_id): pr_warn("Storage update failed for FRU 0x%016llx\n",,
    if (ret == -ENOSPC)
    pub storage\n"): pr_warn("Not enough space on,
    }
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn rec_has_valid_entries(rec: *mut fru_rec) -> bool {
    static bool rec_has_valid_entries(struct fru_rec *rec)
    {
    if (!(rec.fmp.validation_bits & FMP_VALID_LIST_ENTRIES))
    pub false: return,
    if (!(rec.fmp.validation_bits & FMP_VALID_LIST))
    pub false: return,
    pub true: return,
    }
//
// Row retirement is done on MI300 systems, and some bits are 'don't
// care' for comparing addresses with unique physical rows.  This
// includes all column bits and the row[13] bit.
//

#[no_mangle]
unsafe extern "C" fn fpds_equal(old: *mut cper_fru_poison_desc, new: *mut cper_fru_poison_desc) -> bool {
    static bool fpds_equal(struct cper_fru_poison_desc *old, struct cper_fru_poison_desc *new)
    {
//
// Ignore timestamp field.
// The same physical error may be reported multiple times due to stuck bits, etc.
//
// Also, order the checks from most->least likely to fail to shortcut the code.
//
    if (MASK_ADDR(old.addr) != MASK_ADDR(new.addr))
    pub false: return,
    if (old.hw_id != new.hw_id)
    pub false: return,
    if (old.addr_type != new.addr_type)
    pub false: return,
    if (old.hw_id_type != new.hw_id_type)
    pub false: return,
    pub true: return,
    }
#[no_mangle]
unsafe extern "C" fn rec_has_fpd(rec: *mut fru_rec, fpd: *mut cper_fru_poison_desc) -> bool {
    static bool rec_has_fpd(struct fru_rec *rec, struct cper_fru_poison_desc *fpd)
    {
    pub i: c_uint,
    pub {: for (i = 0; i < rec->fmp.nr_entries; i++),
    pub &rec->entries[i]: *mut *mut cper_fru_poison_desc fpd_i =,
    if (fpds_equal(fpd_i, fpd)) {
    pub record\n"): pr_debug("Found duplicate,
    pub true: return,
    }
    }
    pub false: return,
    }
    static void save_spa(struct fru_rec *rec, unsigned int entry,
    u64 addr, u64 id, unsigned int cpu)
    {
    pub spa_entry: unsigned int i, fru_idx,,
    pub a_err: atl_err,
    pub spa: c_ulong,
    if (entry >= max_nr_entries) {
    pr_warn_once("FRU descriptor entry %d out-of-bounds (max: %d)\n",
    pub max_nr_entries): entry,,
    }
// spa_nr_entries is always multiple of max_nr_entries
    pub {: for (i = 0; i < spa_nr_entries; i += max_nr_entries),
    pub max_nr_entries: fru_idx = i /,
    if (fru_records[fru_idx] == rec)
    }
    if (i >= spa_nr_entries) {
    pub i): pr_warn_once("FRU record %d not found\n",,
    }
    pub entry: spa_entry = i +,
    if (spa_entry >= spa_nr_entries) {
    pub out-of-bounds\n"): pr_warn_once("spa_entries[] index,
    }
    pub atl_err)): memset(&a_err, 0, sizeof(struct,
    pub addr: a_err.addr =,
    pub id: a_err.ipid =,
    pub cpu: a_err.cpu =,
    pub amd_convert_umc_mca_addr_to_sys_addr(&a_err): spa =,
    if (IS_ERR_VALUE(spa)) {
    pub address\n"): pr_debug("Failed to get system,
    }
    pub spa: spa_entries[spa_entry] =,
    pr_debug("fru_idx: %u, entry: %u, spa_entry: %u, spa: 0x%016llx\n",
    pub spa_entries[spa_entry]): fru_idx, entry, spa_entry,,
    }
#[no_mangle]
unsafe extern "C" fn update_fru_record(rec: *mut fru_rec, m: *mut mce) {
    static void update_fru_record(struct fru_rec *rec, struct mce *m)
    {
    pub &rec->fmp: *mut *mut cper_sec_fru_mem_poison fmp =,
    pub fpd_dest: *mut cper_fru_poison_desc fpd,,
    pub 0: u32 entry =,
    pub cper_fru_poison_desc)): memset(&fpd, 0, sizeof(struct,
    pub m->time: fpd.timestamp =,
    pub FPD_HW_ID_TYPE_MCA_IPID: fpd.hw_id_type =,
    pub m->ipid: fpd.hw_id =,
    pub FPD_ADDR_TYPE_MCA_ADDR: fpd.addr_type =,
    pub m->addr: fpd.addr =,
// This is the first entry, so just save it.
    if (!rec_has_valid_entries(rec))
    pub save_fpd: goto,
// Ignore already recorded errors.
    if (rec_has_fpd(rec, &fpd))
    pub out_unlock: goto,
    if (rec.fmp.nr_entries >= max_nr_entries) {
    pub rec->fmp.fru_id): pr_warn("Exceeded number of entries for FRU 0x%016llx\n",,
    pub out_unlock: goto,
    }
    pub fmp->nr_entries: entry =,
    save_fpd:
    pub m->extcpu): save_spa(rec, entry, m->addr, m->ipid,,
    pub &rec->entries[entry]: fpd_dest =,
    pub cper_fru_poison_desc)): memcpy(fpd_dest, &fpd, sizeof(struct,
    pub 1: fmp->nr_entries = entry +,
    pub FMP_VALID_LIST_ENTRIES: fmp->validation_bits |=,
    pub FMP_VALID_LIST: fmp->validation_bits |=,
    pub entry): pr_debug("Updated FRU 0x%016llx entry #%u\n", fmp->fru_id,,
    out_unlock:
    }
#[no_mangle]
unsafe extern "C" fn retire_dram_row(addr: u64, id: u64, cpu: u32) {
    static void retire_dram_row(u64 addr, u64 id, u32 cpu)
    {
    pub a_err: atl_err,
    pub atl_err)): memset(&a_err, 0, sizeof(struct,
    pub addr: a_err.addr =,
    pub id: a_err.ipid =,
    pub cpu: a_err.cpu =,
    }
#[no_mangle]
unsafe extern "C" fn fru_handle_mem_poison(nb: *mut notifier_block, val: c_ulong, data: *mut c_void) -> c_int {
    static int fru_handle_mem_poison(struct notifier_block *nb, unsigned long val, void *data)
    {
    pub )data: *mut *mut mce m = (mce,
    pub rec: *mut fru_rec,
    if (!mce_is_memory_error(m))
    pub NOTIFY_DONE: return,
    pub m->extcpu): retire_dram_row(m->addr, m->ipid,,
//
// An invalid FRU ID should not happen on real errors. But it
// could happen from software error injection, etc.
//
    pub get_fru_record(m->ppin): rec =,
    if (!rec)
    pub NOTIFY_DONE: return,
    pub m): update_fru_record(rec,,
    pub NOTIFY_OK: return,
    }
    static struct notifier_block fru_mem_poison_nb = {
    .notifier_call  = fru_handle_mem_poison,
    .priority	= MCE_PRIO_LOWEST,
}

#[no_mangle]
unsafe extern "C" fn retire_mem_fmp(rec: *mut fru_rec) {
    static void retire_mem_fmp(struct fru_rec *rec)
    {
    struct cper_sec_fru_mem_poison *fmp = &rec.fmp;
    unsigned int i, cpu;
    for (i = 0; i < fmp.nr_entries; i++) {
    struct cper_fru_poison_desc *fpd = &rec.entries[i];
    let mut err_cpu: c_uint = INVALID_CPU;
    if (fpd.hw_id_type != FPD_HW_ID_TYPE_MCA_IPID)
    continue;
    if (fpd.addr_type != FPD_ADDR_TYPE_MCA_ADDR)
    continue;
    cpus_read_lock();
    for_each_online_cpu(cpu) {
    if (topology_ppin(cpu) == fmp.fru_id) {
    err_cpu = cpu;
    break;
    }
    }
    cpus_read_unlock();
    if (err_cpu == INVALID_CPU)
    continue;
    retire_dram_row(fpd.addr, fpd.hw_id, err_cpu);
    save_spa(rec, i, fpd.addr, fpd.hw_id, err_cpu);
    }
    }
#[no_mangle]
unsafe extern "C" fn retire_mem_records() {
    static void retire_mem_records(void)
    {
    struct fru_rec *rec;
    unsigned int i;
    for_each_fru(i, rec) {
    if (!rec_has_valid_entries(rec))
    continue;
    retire_mem_fmp(rec);
    }
    }
// Set the CPER Record Header and CPER Section Descriptor fields.
#[no_mangle]
unsafe extern "C" fn set_rec_fields(rec: *mut fru_rec) {
    static void set_rec_fields(struct fru_rec *rec)
    {
    struct cper_section_descriptor	*sec_desc = &rec.sec_desc;
    struct cper_record_header	*hdr	  = &rec.hdr;
//
// This is a saved record created with fewer max_nr_entries.
// Update the record lengths and keep everything else as-is.
//
    if (hdr.record_length && hdr.record_length < max_rec_len) {
    pr_debug("Growing record 0x%016llx from %u to %zu bytes\n",
    hdr.record_id, hdr.record_length, max_rec_len);
    goto update_lengths;
    }
    memcpy(hdr.signature, CPER_SIG_RECORD, CPER_SIG_SIZE);
    hdr.revision			= CPER_RECORD_REV;
    hdr.signature_end		= CPER_SIG_END;
//
// Currently, it is assumed that there is one FRU Memory Poison
// section per CPER. But this may change for other implementations.
//
    hdr.section_count		= 1;
// The logged errors are recoverable. Otherwise, they'd never make it here.
    hdr.error_severity		= CPER_SEV_RECOVERABLE;
    hdr.validation_bits		= 0;
    hdr.creator_id			= CPER_CREATOR_FMP;
    hdr.notification_type		= CPER_NOTIFY_MCE;
    hdr.record_id			= cper_next_record_id();
    hdr.flags			= CPER_HW_ERROR_FLAGS_PREVERR;
    sec_desc.section_offset	= sizeof(struct cper_record_header);
    sec_desc.revision		= CPER_SEC_REV;
    sec_desc.validation_bits	= 0;
    sec_desc.flags			= CPER_SEC_PRIMARY;
    sec_desc.section_type		= CPER_SECTION_TYPE_FMP;
    sec_desc.section_severity	= CPER_SEV_RECOVERABLE;
    update_lengths:
    hdr.record_length		= max_rec_len;
    sec_desc.section_length	= max_rec_len - sizeof(struct cper_record_header);
    }
#[no_mangle]
unsafe extern "C" fn save_new_records() -> c_int {
    static int save_new_records(void)
    {
    DECLARE_BITMAP(new_records, FMPM_MAX_NR_FRU);
    struct fru_rec *rec;
    unsigned int i;
    let mut ret: c_int = 0;
    for_each_fru(i, rec) {
// No need to update saved records that match the current record size.
    if (rec.hdr.record_length == max_rec_len)
    continue;
    if (!rec.hdr.record_length)
    set_bit(i, new_records);
    set_rec_fields(rec);
    ret = update_record_on_storage(rec);
    if (ret)
    goto out_clear;
    }
    return ret;
    out_clear:
    for_each_fru(i, rec) {
    if (!test_bit(i, new_records))
    continue;
    erst_clear(rec.hdr.record_id);
    }
    return ret;
    }
// Check that the record matches expected types for the current system.
#[no_mangle]
unsafe extern "C" fn fmp_is_usable(rec: *mut fru_rec) -> bool {
    static bool fmp_is_usable(struct fru_rec *rec)
    {
    struct cper_sec_fru_mem_poison *fmp = &rec.fmp;
    u64 cpuid;
    pr_debug("Validation bits: 0x%016llx\n", fmp.validation_bits);
    if (!(fmp.validation_bits & FMP_VALID_ARCH_TYPE)) {
    pr_debug("Arch type unknown\n");
    return false;
    }
    if (fmp.fru_arch_type != FMP_ARCH_TYPE_X86_CPUID_1_EAX) {
    pr_debug("Arch type not 'x86 Family/Model/Stepping'\n");
    return false;
    }
    if (!(fmp.validation_bits & FMP_VALID_ARCH)) {
    pr_debug("Arch value unknown\n");
    return false;
    }
    cpuid = cpuid_eax(1);
    if (fmp.fru_arch != cpuid) {
    pr_debug("Arch value mismatch: record = 0x%016llx, system = 0x%016llx\n",
    fmp.fru_arch, cpuid);
    return false;
    }
    if (!(fmp.validation_bits & FMP_VALID_ID_TYPE)) {
    pr_debug("FRU ID type unknown\n");
    return false;
    }
    if (fmp.fru_id_type != FMP_ID_TYPE_X86_PPIN) {
    pr_debug("FRU ID type is not 'x86 PPIN'\n");
    return false;
    }
    if (!(fmp.validation_bits & FMP_VALID_ID)) {
    pr_debug("FRU ID value unknown\n");
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn fmp_is_valid(rec: *mut fru_rec) -> bool {
    static bool fmp_is_valid(struct fru_rec *rec)
    {
    struct cper_sec_fru_mem_poison *fmp = &rec.fmp;
    u32 checksum, len;
    len = get_fmp_len(rec);
    if (len < sizeof(struct cper_sec_fru_mem_poison)) {
    pr_debug("fmp length is too small\n");
    return false;
    }
// Checksum must sum to zero for the entire section.
    checksum = do_fmp_checksum(fmp, len) + fmp.checksum;
    if (checksum) {
    pr_debug("fmp checksum failed: sum = 0x%x\n", checksum);
    print_hex_dump_debug("fmp record: ", DUMP_PREFIX_NONE, 16, 1, fmp, len, false);
    return false;
    }
    if (!fmp_is_usable(rec))
    return false;
    return true;
    }
    static struct fru_rec *get_valid_record(struct fru_rec *old)
    {
    struct fru_rec *new;
    if (!fmp_is_valid(old)) {
    pr_debug("Ignoring invalid record\n");
    return core::ptr::null_mut();
    }
    new = get_fru_record(old.fmp.fru_id);
    if (!new)
    pr_debug("Ignoring record for absent FRU\n");
    return new;
    }
//
// Fetch saved records from persistent storage.
//
// For each found record:
// - If it was not created by this module, then ignore it.
// - If it is valid, then copy its data to the local cache.
// - If it is not valid, then erase it.
//
#[no_mangle]
unsafe extern "C" fn get_saved_records() -> c_int {
    static int get_saved_records(void)
    {
    struct fru_rec *old, *new;
    u64 record_id;
    int ret, pos;
    ssize_t len;
    old = kmalloc(FMPM_MAX_REC_LEN, GFP_KERNEL);
    if (!old) {
    ret = -ENOMEM;
    goto out;
    }
    ret = erst_get_record_id_begin(&pos);
    if (ret < 0)
    goto out_end;
    while (!erst_get_record_id_next(&pos, &record_id)) {
    if (record_id == APEI_ERST_INVALID_RECORD_ID)
    goto out_end;
//
// Make sure to clear temporary buffer between reads to avoid
// leftover data from records of various sizes.
//
    memset(old, 0, FMPM_MAX_REC_LEN);
    len = erst_read_record(record_id, &old.hdr, FMPM_MAX_REC_LEN,
    sizeof(struct fru_rec), &CPER_CREATOR_FMP);
    if (len < 0)
    continue;
    new = get_valid_record(old);
    if (!new) {
    erst_clear(record_id);
    continue;
    }
    if (len > max_rec_len) {
    unsigned int saved_nr_entries;
    saved_nr_entries  = len - sizeof(struct fru_rec);
    saved_nr_entries /= sizeof(struct cper_fru_poison_desc);
    pr_warn("Saved record found with %u entries.\n", saved_nr_entries);
    pr_warn("Please increase max_nr_entries to %u.\n", saved_nr_entries);
    ret = -EINVAL;
    goto out_end;
    }
// Restore the record
    memcpy(new, old, len);
    }
    out_end:
    erst_get_record_id_end();
    kfree(old);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn set_fmp_fields(rec: *mut fru_rec, cpu: c_uint) {
    static void set_fmp_fields(struct fru_rec *rec, unsigned int cpu)
    {
    struct cper_sec_fru_mem_poison *fmp = &rec.fmp;
    fmp.fru_arch_type    = FMP_ARCH_TYPE_X86_CPUID_1_EAX;
    fmp.validation_bits |= FMP_VALID_ARCH_TYPE;
// Assume all CPUs in the system have the same value for now.
    fmp.fru_arch	      = cpuid_eax(1);
    fmp.validation_bits |= FMP_VALID_ARCH;
    fmp.fru_id_type      = FMP_ID_TYPE_X86_PPIN;
    fmp.validation_bits |= FMP_VALID_ID_TYPE;
    fmp.fru_id	      = topology_ppin(cpu);
    fmp.validation_bits |= FMP_VALID_ID;
    }
#[no_mangle]
unsafe extern "C" fn init_fmps() -> c_int {
    static int init_fmps(void)
    {
    struct fru_rec *rec;
    unsigned int i, cpu;
    let mut ret: c_int = 0;
    for_each_fru(i, rec) {
    let mut fru_cpu: c_uint = INVALID_CPU;
    cpus_read_lock();
    for_each_online_cpu(cpu) {
    if (topology_physical_package_id(cpu) == i) {
    fru_cpu = cpu;
    break;
    }
    }
    cpus_read_unlock();
    if (fru_cpu == INVALID_CPU) {
    pr_debug("Failed to find matching CPU for FRU #%u\n", i);
    ret = -ENODEV;
    break;
    }
    set_fmp_fields(rec, fru_cpu);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn get_system_info() -> c_int {
    static int get_system_info(void)
    {
// Only load on MI300A systems for now.
    if (!(boot_cpu_data.x86_model >= 0x90 &&
    boot_cpu_data.x86_model <= 0x9f))
    return -ENODEV;
    if (!cpu_feature_enabled(X86_FEATURE_AMD_PPIN)) {
    pr_debug("PPIN feature not available\n");
    return -ENODEV;
    }
// Use CPU socket as FRU for MI300 systems.
    max_nr_fru = topology_max_packages();
    if (!max_nr_fru)
    return -ENODEV;
    if (max_nr_fru > FMPM_MAX_NR_FRU) {
    pr_warn("Too many FRUs to manage: found: %u, max: %u\n",
    max_nr_fru, FMPM_MAX_NR_FRU);
    return -ENODEV;
    }
    if (!max_nr_entries)
    max_nr_entries = FMPM_DEFAULT_MAX_NR_ENTRIES;
    spa_nr_entries = max_nr_fru * max_nr_entries;
    max_rec_len  = sizeof(struct fru_rec);
    max_rec_len += sizeof(struct cper_fru_poison_desc) * max_nr_entries;
    pr_info("max FRUs: %u, max entries: %u, max record length: %lu\n",
    max_nr_fru, max_nr_entries, max_rec_len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_records() {
    static void free_records(void)
    {
    struct fru_rec *rec;
    int i;
    for_each_fru(i, rec)
    kfree(rec);
    kfree(fru_records);
    kfree(spa_entries);
    }
#[no_mangle]
unsafe extern "C" fn allocate_records() -> c_int {
    static int allocate_records(void)
    {
    int i, ret = 0;
    fru_records = kzalloc_objs(struct fru_rec *, max_nr_fru);
    if (!fru_records) {
    ret = -ENOMEM;
    goto out;
    }
    for (i = 0; i < max_nr_fru; i++) {
    fru_records[i] = kzalloc(max_rec_len, GFP_KERNEL);
    if (!fru_records[i]) {
    ret = -ENOMEM;
    goto out_free;
    }
    }
    spa_entries = kcalloc(spa_nr_entries, sizeof(u64), GFP_KERNEL);
    if (!spa_entries) {
    ret = -ENOMEM;
    goto out_free;
    }
    for (i = 0; i < spa_nr_entries; i++)
    spa_entries[i] = INVALID_SPA;
    return ret;
    out_free:
    while (--i >= 0)
    kfree(fru_records[i]);
    kfree(fru_records);
    out:
    return ret;
    }
    static void *fmpm_start(struct seq_file *f, loff_t *pos)
    {
    if (*pos >= (spa_nr_entries + 1))
    return core::ptr::null_mut();
    return pos;
    }
    static void *fmpm_next(struct seq_file *f, void *data, loff_t *pos)
    {
    if (++(*pos) >= (spa_nr_entries + 1))
    return core::ptr::null_mut();
    return pos;
    }
#[no_mangle]
unsafe extern "C" fn fmpm_stop(f: *mut seq_file, data: *mut c_void) {
    static void fmpm_stop(struct seq_file *f, void *data)
    {
    }
pub const SHORT_WIDTH: c_int = 8;
pub const U64_WIDTH: c_int = 18;
pub const TIMESTAMP_WIDTH: c_int = 19;
pub const LONG_WIDTH: c_int = 24;

#[no_mangle]
unsafe extern "C" fn fmpm_show(f: *mut seq_file, data: *mut c_void) -> c_int {
    static int fmpm_show(struct seq_file *f, void *data)
    {
    unsigned int fru_idx, entry, spa_entry, line;
    struct cper_fru_poison_desc *fpd;
    struct fru_rec *rec;
    line = *(loff_t *)data;
    if (line == 0) {
    seq_printf(f, "%-*s", SHORT_WIDTH, "fru_idx");
    seq_printf(f, "%-*s", LONG_WIDTH,  "fru_id");
    seq_printf(f, "%-*s", SHORT_WIDTH, "entry");
    seq_printf(f, "%-*s", LONG_WIDTH,  "timestamp");
    seq_printf(f, "%-*s", LONG_WIDTH,  "hw_id");
    seq_printf(f, "%-*s", LONG_WIDTH,  "addr");
    seq_printf(f, "%-*s", LONG_WIDTH,  "spa");
    goto out_newline;
    }
    spa_entry = line - 1;
    fru_idx	  = spa_entry / max_nr_entries;
    entry	  = spa_entry % max_nr_entries;
    rec = fru_records[fru_idx];
    if (!rec)
    goto out;
    seq_printf(f, "%-*u",		SHORT_WIDTH, fru_idx);
    seq_printf(f, "0x%016llx%-*s",	rec.fmp.fru_id, U64_PAD, "");
    seq_printf(f, "%-*u",		SHORT_WIDTH, entry);
    mutex_lock(&fmpm_update_mutex);
    if (entry >= rec.fmp.nr_entries) {
    seq_printf(f, "%-*s", LONG_WIDTH, "*");
    seq_printf(f, "%-*s", LONG_WIDTH, "*");
    seq_printf(f, "%-*s", LONG_WIDTH, "*");
    seq_printf(f, "%-*s", LONG_WIDTH, "*");
    goto out_unlock;
    }
    fpd = &rec.entries[entry];
    seq_printf(f, "%ptT%-*s",	&fpd.timestamp, TS_PAD,  "");
    seq_printf(f, "0x%016llx%-*s",	fpd.hw_id,	 U64_PAD, "");
    seq_printf(f, "0x%016llx%-*s",	fpd.addr,	 U64_PAD, "");
    if (spa_entries[spa_entry] == INVALID_SPA)
    seq_printf(f, "%-*s", LONG_WIDTH, "*");
    else
    seq_printf(f, "0x%016llx%-*s", spa_entries[spa_entry], U64_PAD, "");
    out_unlock:
    mutex_unlock(&fmpm_update_mutex);
    out_newline:
    seq_putc(f, '\n');
    out:
    return 0;
    }
    static const struct seq_operations fmpm_seq_ops = {
    .start	= fmpm_start,
    .next	= fmpm_next,
    .stop	= fmpm_stop,
    .show	= fmpm_show,
    };
#[no_mangle]
unsafe extern "C" fn fmpm_open(inode: *mut inode, file: *mut file) -> c_int {
    static int fmpm_open(struct inode *inode, struct file *file)
    {
    return seq_open(file, &fmpm_seq_ops);
    }
    static const struct file_operations fmpm_fops = {
    .open		= fmpm_open,
    .release	= seq_release,
    .read		= seq_read,
    .llseek		= seq_lseek,
    };
#[no_mangle]
unsafe extern "C" fn setup_debugfs() {
    static void setup_debugfs(void)
    {
    struct dentry *dfs = ras_get_debugfs_root();
    if (!dfs)
    return;
    fmpm_dfs_dir = debugfs_create_dir("fmpm", dfs);
    if (!fmpm_dfs_dir)
    return;
    fmpm_dfs_entries = debugfs_create_file("entries", 0400, fmpm_dfs_dir, core::ptr::null_mut(), &fmpm_fops);
    if (!fmpm_dfs_entries)
    debugfs_remove(fmpm_dfs_dir);
    }
    static const struct x86_cpu_id fmpm_cpuids[] = {
    X86_MATCH_VENDOR_FAM(AMD, 0x19, core::ptr::null_mut()),
    { }
    };
    MODULE_DEVICE_TABLE(x86cpu, fmpm_cpuids);
#[no_mangle]
unsafe extern "C" fn fru_mem_poison_init() -> int __init {
    static int __init fru_mem_poison_init(void)
    {
    int ret;
    if (!x86_match_cpu(fmpm_cpuids)) {
    ret = -ENODEV;
    goto out;
    }
    if (erst_disable) {
    pr_debug("ERST not available\n");
    ret = -ENODEV;
    goto out;
    }
    ret = get_system_info();
    if (ret)
    goto out;
    ret = allocate_records();
    if (ret)
    goto out;
    ret = init_fmps();
    if (ret)
    goto out_free;
    ret = get_saved_records();
    if (ret)
    goto out_free;
    ret = save_new_records();
    if (ret)
    goto out_free;
    setup_debugfs();
    retire_mem_records();
    mce_register_decode_chain(&fru_mem_poison_nb);
    pr_info("FRU Memory Poison Manager initialized\n");
    return 0;
    out_free:
    free_records();
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fru_mem_poison_exit() -> void __exit {
    static void __exit fru_mem_poison_exit(void)
    {
    mce_unregister_decode_chain(&fru_mem_poison_nb);
    debugfs_remove(fmpm_dfs_dir);
    free_records();
    }
    module_init(fru_mem_poison_init);
    module_exit(fru_mem_poison_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("FRU Memory Poison Manager");
