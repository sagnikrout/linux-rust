//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/mce/apei.c
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
// Bridge between MCE and APEI
//
// On some machine, corrected memory errors are reported via APEI
// generic hardware error source (GHES) instead of corrected Machine
// Check. These corrected memory errors can be reported to user space
// through /dev/mcelog via faking a corrected Machine Check, so that
// the error memory page can be offlined by /sbin/mcelog if the error
// count for one page is beyond the threshold.
//
// For fatal MCE, save MCE record into persistent storage via ERST, so
// that the MCE record can be logged after reboot via ERST.
//
// Copyright 2010 Intel Corp.
// Author: Huang Ying <ying.huang@intel.com>
//

#[no_mangle]
pub unsafe extern "C" fn apei_mce_report_mem_error(severity: c_int, mem_err: *mut cper_sec_mem_err) {
    void apei_mce_report_mem_error(int severity, struct cper_sec_mem_err *mem_err)
    {
    struct mce_hw_err err;
    struct mce *m;
    int lsb;
    if (!(mem_err.validation_bits & CPER_MEM_VALID_PA))
    return;
//
// Even if the ->validation_bits are set for address mask,
// to be extra safe, check and reject an error radius '0',
// and fall back to the default page size.
//
    if (mem_err.validation_bits & CPER_MEM_VALID_PA_MASK)
    lsb = find_first_bit((void *)&mem_err.physical_addr_mask, PAGE_SHIFT);
    else
    lsb = PAGE_SHIFT;
    mce_prep_record(&err);
    m = &err.m;
    m.bank = -1;
// Fake a memory read error with unknown channel
    m.status = MCI_STATUS_VAL | MCI_STATUS_EN | MCI_STATUS_ADDRV | MCI_STATUS_MISCV | 0x9f;
    m.misc = (MCI_MISC_ADDR_PHYS << 6) | lsb;
    if (severity >= GHES_SEV_RECOVERABLE)
    m.status |= MCI_STATUS_UC;
    if (severity >= GHES_SEV_PANIC) {
    m.status |= MCI_STATUS_PCC;
    m.tsc = rdtsc();
    }
    m.addr = mem_err.physical_addr;
    mce_log(&err);
    }
    EXPORT_SYMBOL_GPL(apei_mce_report_mem_error);
#[no_mangle]
pub unsafe extern "C" fn apei_smca_report_x86_error(ctx_info: *mut cper_ia_proc_ctx, lapic_id: u64) -> c_int {
    int apei_smca_report_x86_error(struct cper_ia_proc_ctx *ctx_info, u64 lapic_id)
    {
    const u64 *i_mce = ((const u64 *) (ctx_info + 1));
    unsigned int cpu, num_regs;
    let mut apicid_found: bool = false;
    struct mce_hw_err err;
    struct mce *m;
    if (!boot_cpu_has(X86_FEATURE_SMCA))
    return -EINVAL;
//
// The starting address of the register array extracted from BERT must
// match with the first expected register in the register layout of
// SMCA address space. This address corresponds to banks's MCA_STATUS
// register.
//
// Match any MCi_STATUS register by turning off bank numbers.
//
    if ((ctx_info.msr_addr & MSR_AMD64_SMCA_MC0_STATUS) !=
    MSR_AMD64_SMCA_MC0_STATUS)
    return -EINVAL;
//
// The number of registers in the register array is determined by
// Register Array Size/8 as defined in UEFI spec v2.8, sec N.2.4.2.2.
// Sanity-check registers array size.
//
    num_regs = ctx_info.reg_arr_size >> 3;
    if (!num_regs)
    return -EINVAL;
    for_each_possible_cpu(cpu) {
    if (cpu_data(cpu).topo.initial_apicid == lapic_id) {
    apicid_found = true;
    break;
    }
    }
    if (!apicid_found)
    return -EINVAL;
    m = &err.m;
    memset(&err, 0, sizeof(struct mce_hw_err));
    mce_prep_record_common(m);
    mce_prep_record_per_cpu(cpu, m);
    m.bank = (ctx_info.msr_addr >> 4) & 0xFF;
//
// The SMCA register layout is fixed and includes 16 registers.
// The end of the array may be variable, but the beginning is known.
// Cap the number of registers to expected max (15).
//
    if (num_regs > 15)
    num_regs = 15;
    switch (num_regs) {
// MCA_SYND2
    case 15:
    err.vendor.amd.synd2 = *(i_mce + 14);
    fallthrough;
// MCA_SYND1
    case 14:
    err.vendor.amd.synd1 = *(i_mce + 13);
    fallthrough;
// MCA_MISC4
    case 13:
// MCA_MISC3
    case 12:
// MCA_MISC2
    case 11:
// MCA_MISC1
    case 10:
// MCA_DEADDR
    case 9:
// MCA_DESTAT
    case 8:
// reserved
    case 7:
// MCA_SYND
    case 6:
    m.synd = *(i_mce + 5);
    fallthrough;
// MCA_IPID
    case 5:
    m.ipid = *(i_mce + 4);
    fallthrough;
// MCA_CONFIG
    case 4:
// MCA_MISC0
    case 3:
    m.misc = *(i_mce + 2);
    fallthrough;
// MCA_ADDR
    case 2:
    m.addr = *(i_mce + 1);
    fallthrough;
// MCA_STATUS
    case 1:
    m.status = *i_mce;
    }
    mce_log(&err);
    return 0;
    }

    GUID_INIT(0x75a574e3, 0x5052, 0x4b29, 0x8a, 0x8e, 0xbe, 0x2c,	\
    0x64, 0x90, 0xb8, 0x9d)

    GUID_INIT(0xfe08ffbe, 0x95e4, 0x4be7, 0xbc, 0x73, 0x40, 0x96,	\
    0x04, 0x4a, 0x38, 0xfc)
//
// CPER specification (in UEFI specification 2.3 appendix N) requires
// byte-packed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_mce_record {
    pub hdr: cper_record_header,
    pub sec_hdr: cper_section_descriptor,
    pub mce: mce,
    pub __packed: },
#[no_mangle]
pub unsafe extern "C" fn apei_write_mce(m: *mut mce) -> c_int {
    int apei_write_mce(struct mce *m)
    {
    pub rcd: cper_mce_record,
    pub sizeof(rcd)): memset(&rcd, 0,,
    pub CPER_SIG_SIZE): memcpy(rcd.hdr.signature, CPER_SIG_RECORD,,
    pub CPER_RECORD_REV: rcd.hdr.revision =,
    pub CPER_SIG_END: rcd.hdr.signature_end =,
    pub 1: rcd.hdr.section_count =,
    pub CPER_SEV_FATAL: rcd.hdr.error_severity =,
// timestamp, platform_id, partition_id are all invalid
    pub 0: rcd.hdr.validation_bits =,
    pub sizeof(rcd): rcd.hdr.record_length =,
    pub CPER_CREATOR_MCE: rcd.hdr.creator_id =,
    pub CPER_NOTIFY_MCE: rcd.hdr.notification_type =,
    pub cper_next_record_id(): rcd.hdr.record_id =,
    pub CPER_HW_ERROR_FLAGS_PREVERR: rcd.hdr.flags =,
    pub )&rcd: *mut *mut rcd.sec_hdr.section_offset = (void )&rcd.mce - (void,
    pub sizeof(rcd.mce): rcd.sec_hdr.section_length =,
    pub CPER_SEC_REV: rcd.sec_hdr.revision =,
// fru_id and fru_text is invalid
    pub 0: rcd.sec_hdr.validation_bits =,
    pub CPER_SEC_PRIMARY: rcd.sec_hdr.flags =,
    pub CPER_SECTION_TYPE_MCE: rcd.sec_hdr.section_type =,
    pub CPER_SEV_FATAL: rcd.sec_hdr.section_severity =,
    pub sizeof(*m)): *mut memcpy(&rcd.mce, m,,
    pub erst_write(&rcd.hdr): return,
    }
#[no_mangle]
pub unsafe extern "C" fn apei_read_mce(m: *mut mce, record_id: *mut u64) -> isize {
    ssize_t apei_read_mce(struct mce *m, u64 *record_id)
    {
    pub rcd: cper_mce_record,
    pub pos: int rc,,
    pub erst_get_record_id_begin(&pos): rc =,
    if (rc)
    pub rc: return,
    retry:
    pub record_id): rc = erst_get_record_id_next(&pos,,
    if (rc)
    pub out: goto,
// no more record
    if (*record_id == APEI_ERST_INVALID_RECORD_ID)
    pub out: goto,
    rc = erst_read_record(*record_id, &rcd.hdr, sizeof(rcd), sizeof(rcd),
// someone else has cleared the record, try next one
    if (rc == -ENOENT)
    pub retry: goto,
#[no_mangle]
pub unsafe extern "C" fn if(0: rc <) -> else {
    else if (rc < 0)
    pub out: goto,
    pub sizeof(*m)): *mut memcpy(m, &rcd.mce,,
    pub sizeof(*m): *mut rc =,
    out:
    pub rc: return,
    }
// Check whether there is record in ERST
#[no_mangle]
pub unsafe extern "C" fn apei_check_mce() -> c_int {
    int apei_check_mce(void)
    {
    pub erst_get_record_count(): return,
    }
#[no_mangle]
pub unsafe extern "C" fn apei_clear_mce(record_id: u64) -> c_int {
    int apei_clear_mce(u64 record_id)
    {
    pub erst_clear(record_id): return,
    }
