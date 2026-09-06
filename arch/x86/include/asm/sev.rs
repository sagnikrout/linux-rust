//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/sev.h
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
// AMD Encrypted Register State Support
//
// Author: Joerg Roedel <jroedel@suse.de>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es_result {
    ES_OK,			/* All good */
    ES_UNSUPPORTED,		/* Requested operation not supported */
    ES_VMM_ERROR,		/* Unexpected state from the VMM */
    ES_DECODE_FAILED,	/* Instruction decoding failed */
    ES_EXCEPTION,		/* Instruction caused exception */
    ES_RETRY,		/* Retry instruction emulation */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct es_fault_info {
    pub vector: c_ulong,
    pub error_code: c_ulong,
    pub cr2: c_ulong,
}

// ES instruction emulation context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es_em_ctxt {
    pub regs: *mut pt_regs,
    pub insn: insn,
    pub fi: es_fault_info,
}

//
// AMD SEV Confidential computing blob structure. The structure is
// defined in OVMF UEFI firmware header:
// https://github.com/tianocore/edk2/blob/master/OvmfPkg/Include/Guid/ConfidentialComputingSevSnpBlob.h
//
pub const CC_BLOB_SEV_HDR_MAGIC: c_uint = 0x45444d41;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc_blob_sev_info {
    pub magic: u32,
    pub version: u16,
    pub reserved: u16,
    pub secrets_phys: u64,
    pub secrets_len: u32,
    pub rsvd1: u32,
    pub cpuid_phys: u64,
    pub cpuid_len: u32,
    pub rsvd2: u32,
    pub __packed: },
    pub exit_code): *mut *mut void do_vc_no_ghcb(struct pt_regs regs, unsigned long,
    pub 1: u64 mask = (1ULL << bits) -,
    pub mask): return (val &,
    pub real_mode_header: struct,
    pub stack_type: enum,
// Early IDT entry points for #VC handler
    pub vc_no_ghcb(void): extern void,
    pub vc_boot_ghcb(void): extern void,
    pub regs): *mut extern bool handle_vc_boot_ghcb(struct pt_regs,
//
// Individual entries of the SNP CPUID table, as defined by the SNP
// Firmware ABI, Revision 0.9, Section 7.1, Table 14.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_cpuid_fn {
    pub eax_in: u32,
    pub ecx_in: u32,
    pub xcr0_in: u64,
    pub xss_in: u64,
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub __reserved: u64,
    pub __packed: },
//
// SNP CPUID table, as defined by the SNP Firmware ABI, Revision 0.9,
// Section 8.14.2.6. Also noted there is the SNP firmware-enforced limit
// of 64 entries per CPUID table.
//
pub const SNP_CPUID_COUNT_MAX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_cpuid_table {
    pub count: u32,
    pub __reserved1: u32,
    pub __reserved2: u64,
    pub fn: [snp_cpuid_fn; SNP_CPUID_COUNT_MAX],
    pub __packed: },
// PVALIDATE return codes
pub const PVALIDATE_FAIL_SIZEMISMATCH: c_int = 6;
// Software defined (when rFlags.CF = 1)
pub const PVALIDATE_FAIL_NOUPDATE: c_int = 255;
// RMUPDATE detected 4K page and 2MB page overlap.
pub const RMPUPDATE_FAIL_OVERLAP: c_int = 4;
// PSMASH failed due to concurrent access by another CPU
pub const PSMASH_FAIL_INUSE: c_int = 3;
// RMP page size
pub const RMP_PG_SIZE_4K: c_int = 0;
pub const RMP_PG_SIZE_2M: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmp_state {
    pub gpa: u64,
    pub assigned: u8,
    pub pagesize: u8,
    pub immutable: u8,
    pub rsvd: u8,
    pub asid: u32,
    pub __packed: },

// SNP Guest message request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_req_data {
    pub req_gpa: c_ulong,
    pub resp_gpa: c_ulong,
    pub data_gpa: c_ulong,
    pub data_npages: c_uint,
}

pub const MAX_AUTHTAG_LEN: c_int = 32;
pub const AUTHTAG_LEN: c_int = 16;
pub const AAD_LEN: c_int = 48;
pub const MSG_HDR_VER: c_int = 1;

// See SNP spec SNP_GUEST_REQUEST section for the structure
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msg_type {
    SNP_MSG_TYPE_INVALID = 0,
    SNP_MSG_CPUID_REQ,
    SNP_MSG_CPUID_RSP,
    SNP_MSG_KEY_REQ,
    SNP_MSG_KEY_RSP,
    SNP_MSG_REPORT_REQ,
    SNP_MSG_REPORT_RSP,
    SNP_MSG_EXPORT_REQ,
    SNP_MSG_EXPORT_RSP,
    SNP_MSG_IMPORT_REQ,
    SNP_MSG_IMPORT_RSP,
    SNP_MSG_ABSORB_REQ,
    SNP_MSG_ABSORB_RSP,
    SNP_MSG_VMRK_REQ,
    SNP_MSG_VMRK_RSP,

    SNP_MSG_TSC_INFO_REQ = 17,
    SNP_MSG_TSC_INFO_RSP,

    SNP_MSG_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aead_algo {
    SNP_AEAD_INVALID,
    SNP_AEAD_AES_256_GCM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_guest_msg_hdr {
    pub authtag: [u8; MAX_AUTHTAG_LEN],
    pub msg_seqno: u64,
    pub rsvd1: [u8; 8],
    pub algo: u8,
    pub hdr_version: u8,
    pub hdr_sz: u16,
    pub msg_type: u8,
    pub msg_version: u8,
    pub msg_sz: u16,
    pub rsvd2: u32,
    pub msg_vmpck: u8,
    pub rsvd3: [u8; 35],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_guest_msg {
    pub hdr: snp_guest_msg_hdr,
    pub snp_guest_msg_hdr)]: u8 payload[PAGE_SIZE - sizeof(struct,
    pub __packed: },
pub const SNP_TSC_INFO_REQ_SZ: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_tsc_info_req {
    pub rsvd: [u8; SNP_TSC_INFO_REQ_SZ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_tsc_info_resp {
    pub status: u32,
    pub rsvd1: u32,
    pub tsc_scale: u64,
    pub tsc_offset: u64,
    pub tsc_factor: u32,
    pub rsvd2: [u8; 100],
    pub __packed: },
//
// Obtain the mean TSC frequency by decreasing the nominal TSC frequency with
// TSC_FACTOR as documented in the SNP Firmware ABI specification:
//
// GUEST_TSC_FREQ * (1 - (TSC_FACTOR * 0.00001))
//
// which is equivalent to:
//
// GUEST_TSC_FREQ -= (GUEST_TSC_FREQ * TSC_FACTOR) / 100000;
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_guest_req {
    pub req_buf: *mut c_void,
    pub req_sz: usize,
    pub resp_buf: *mut c_void,
    pub resp_sz: usize,
    pub exit_code: u64,
    pub exitinfo2: u64,
    pub vmpck_id: c_uint,
    pub msg_version: u8,
    pub msg_type: u8,
    pub input: snp_req_data,
    pub certs_data: *mut c_void,
}

//
// The secrets page contains 96-bytes of reserved field that can be used by
// the guest OS. The guest OS uses the area to save the message sequence
// number for each VMPCK.
//
// See the GHCB spec section Secret page layout for the format for this area.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct secrets_os_area {
    pub msg_seqno_0: u32,
    pub msg_seqno_1: u32,
    pub msg_seqno_2: u32,
    pub msg_seqno_3: u32,
    pub ap_jump_table_pa: u64,
    pub rsvd: [u8; 40],
    pub guest_usage: [u8; 32],
    pub __packed: },
pub const VMPCK_KEY_LEN: c_int = 32;
// See the SNP spec version 0.9 for secrets page format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_secrets_page {
    pub version: u32,
    pub 31: rsvd1 :,
    pub fms: u32,
    pub rsvd2: u32,
    pub gosvw: [u8; 16],
    pub vmpck0: [u8; VMPCK_KEY_LEN],
    pub vmpck1: [u8; VMPCK_KEY_LEN],
    pub vmpck2: [u8; VMPCK_KEY_LEN],
    pub vmpck3: [u8; VMPCK_KEY_LEN],
    pub os_area: secrets_os_area,
    pub vmsa_tweak_bitmap: [u8; 64],
// SVSM fields
    pub svsm_base: u64,
    pub svsm_size: u64,
    pub svsm_caa: u64,
    pub svsm_max_version: u32,
    pub svsm_guest_vmpl: u8,
    pub rsvd3: [u8; 3],
// The percentage decrease from nominal to mean TSC frequency.
    pub tsc_factor: u32,
// Remainder of page
    pub rsvd4: [u8; 3740],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snp_msg_desc {
// request and response are in unencrypted memory
    pub response: *mut *mut snp_guest_msg request,,
//
// Avoid information leakage by double-buffering shared messages
// in fields that are in regular encrypted memory.
//
    pub secret_response: snp_guest_msg secret_request,,
    pub secrets: *mut snp_secrets_page,
    pub gcm_key: *mut aes_gcm_key,
    pub os_area_msg_seqno: *mut u32,
    pub vmpck: *mut u8,
    pub vmpck_id: c_int,
}

//
// The SVSM Calling Area (CA) related structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svsm_ca {
    pub call_pending: u8,
    pub mem_available: u8,
    pub rsvd1: [u8; 6],
    pub 8]: u8 svsm_buffer[PAGE_SIZE -,
}

pub const SVSM_SUCCESS: c_int = 0;
pub const SVSM_ERR_INCOMPLETE: c_uint = 0x80000000;
pub const SVSM_ERR_UNSUPPORTED_PROTOCOL: c_uint = 0x80000001;
pub const SVSM_ERR_UNSUPPORTED_CALL: c_uint = 0x80000002;
pub const SVSM_ERR_INVALID_ADDRESS: c_uint = 0x80000003;
pub const SVSM_ERR_INVALID_FORMAT: c_uint = 0x80000004;
pub const SVSM_ERR_INVALID_PARAMETER: c_uint = 0x80000005;
pub const SVSM_ERR_INVALID_REQUEST: c_uint = 0x80000006;
pub const SVSM_ERR_BUSY: c_uint = 0x80000007;
pub const SVSM_PVALIDATE_FAIL_SIZEMISMATCH: c_uint = 0x80001006;
//
// The SVSM PVALIDATE related structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svsm_pvalidate_entry {
    pub 52: pfn :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svsm_pvalidate_call {
    pub num_entries: u16,
    pub cur_index: u16,
    pub rsvd1: [u8; 4],
    pub entry: [svsm_pvalidate_entry; ],
}

//
// The SVSM Attestation related structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svsm_loc_entry {
    pub pa: u64,
    pub len: u32,
    pub rsvd: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svsm_attest_call {
    pub report_buf: svsm_loc_entry,
    pub nonce: svsm_loc_entry,
    pub manifest_buf: svsm_loc_entry,
    pub certificates_buf: svsm_loc_entry,
// For attesting a single service
    pub service_guid: [u8; 16],
    pub service_manifest_ver: u32,
    pub rsvd: [u8; 4],
}

// PTE descriptor used for the prepare_pte_enc() operations.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_enc_desc {
    pub kpte: *mut pte_t,
    pub pte_level: c_int,
    pub encrypt: bool,
// pfn of the kpte above
    pub pfn: c_ulong,
// physical address of @pfn
    pub pa: c_ulong,
// virtual address of @pfn
    pub va: *mut c_void,
// memory covered by the pte
    pub size: c_ulong,
    pub new_pgprot: pgprot_t,
}

//
// SVSM protocol structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svsm_call {
    pub caa: *mut svsm_ca,
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub r8: u64,
    pub r9: u64,
    pub rax_out: u64,
    pub rcx_out: u64,
    pub rdx_out: u64,
    pub r8_out: u64,
    pub r9_out: u64,
}

pub const SVSM_CORE_REMAP_CA: c_int = 0;
pub const SVSM_CORE_PVALIDATE: c_int = 1;
pub const SVSM_CORE_CREATE_VCPU: c_int = 2;
pub const SVSM_CORE_DELETE_VCPU: c_int = 3;

pub const SVSM_ATTEST_SERVICES: c_int = 0;
pub const SVSM_ATTEST_SINGLE_SERVICE: c_int = 1;

pub const SVSM_VTPM_QUERY: c_int = 0;
pub const SVSM_VTPM_CMD: c_int = 1;

extern "C" {
    pub fn __sev_es_ist_enter(regs: *mut pt_regs);
}
extern "C" {
    pub fn __sev_es_ist_exit();
}
extern "C" {
    pub fn sev_es_setup_ap_jump_table(rmh: *mut real_mode_header) -> c_int;
}
extern "C" {
    pub fn __sev_es_nmi_complete();
}
extern "C" {
    pub fn sev_es_efi_map_ghcbs_cas(pgd: *mut pgd_t) -> int __init;
}
extern "C" {
    pub fn sev_enable(bp: *mut boot_params);
}
//
// RMPADJUST modifies the RMP permissions of a page of a lesser-
// privileged (numerically higher) VMPL.
//
// If the guest is running at a higher-privilege than the privilege
// level the instruction is targeting, the instruction will succeed,
// otherwise, it will fail.
//
// "rmpadjust" mnemonic support in binutils 2.36 and newer
// "pvalidate" mnemonic support in binutils 2.36 and newer
extern "C" {
    pub fn setup_ghcb();
}
extern "C" {
    pub fn snp_register_ghcb_early(paddr: c_ulong);
}
extern "C" {
    pub fn snp_set_memory_shared(vaddr: c_ulong, npages: c_ulong);
}
extern "C" {
    pub fn snp_set_memory_private(vaddr: c_ulong, npages: c_ulong);
}
extern "C" {
    pub fn snp_set_wakeup_secondary_cpu();
}
extern "C" {
    pub fn snp_init(bp: *mut boot_params) -> bool;
}
extern "C" {
    pub fn snp_dmi_setup();
}
extern "C" {
    pub fn snp_issue_svsm_attest_req(call_id: u64, call: *mut svsm_call, input: *mut svsm_attest_call) -> c_int;
}
extern "C" {
    pub fn snp_accept_memory(start: phys_addr_t, end: phys_addr_t);
}
extern "C" {
    pub fn snp_get_unsupported_features(status: u64) -> u64;
}
extern "C" {
    pub fn sev_get_status() -> u64;
}
extern "C" {
    pub fn sev_show_status();
}
extern "C" {
    pub fn prepare_pte_enc(d: *mut pte_enc_desc) -> c_int;
}
extern "C" {
    pub fn set_pte_enc_mask(kpte: *mut pte_t, pfn: c_ulong, new_prot: pgprot_t);
}
extern "C" {
    pub fn snp_kexec_finish();
}
extern "C" {
    pub fn snp_kexec_begin();
}
extern "C" {
    pub fn snp_msg_init(mdesc: *mut snp_msg_desc, vmpck_id: c_int) -> c_int;
}
extern "C" {
    pub fn snp_msg_free(mdesc: *mut snp_msg_desc);
}
extern "C" {
    pub fn snp_send_guest_request(mdesc: *mut snp_msg_desc, req: *mut snp_guest_req) -> c_int;
}
extern "C" {
    pub fn snp_svsm_vtpm_send_command(buffer: *mut u8) -> c_int;
}
extern "C" {
    pub fn snp_secure_tsc_prepare() -> void __init;
}
extern "C" {
    pub fn snp_secure_tsc_init() -> void __init;
}
extern "C" {
    pub fn savic_register_gpa(gpa: u64) -> es_result;
}
extern "C" {
    pub fn savic_unregister_gpa(gpa: *mut u64) -> es_result;
}
extern "C" {
    pub fn savic_ghcb_msr_read(reg: u32) -> u64;
}
extern "C" {
    pub fn savic_ghcb_msr_write(reg: u32, value: u64);
}
// I/O parameters for CPUID-related helpers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuid_leaf {
    pub fn: u32,
    pub subfn: u32,
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

extern "C" {
    pub fn svsm_perform_msr_protocol(call: *mut svsm_call) -> c_int;
}
extern "C" {
    pub fn __pi_svsm_perform_msr_protocol(call: *mut svsm_call) -> c_int;
}
extern "C" {
    pub fn svsm_issue_call(call: *mut svsm_call, pending: *mut u8);
}
extern "C" {
    pub fn svsm_process_result_codes(call: *mut svsm_call) -> c_int;
}
extern "C" {
    pub fn sev_es_terminate(set: c_uint, reason: c_uint) -> void __noreturn;
}
extern "C" {
    pub fn sev_es_negotiate_protocol() -> bool;
}
extern "C" {
    pub fn sev_es_check_cpu_features() -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psc_desc {
    pub op: psc_op,
    pub ca: *mut svsm_ca,
    pub caa_pa: u64,
}

//
// For SEV guests, a read from the first/last cache-lines of a 4K page
// using the guest key is sufficient to cause a flush of all cache-lines
// associated with that 4K page without incurring all the overhead of a
// full CLFLUSH sequence.
//

pub const snp_vmpl: c_int = 0;

extern "C" {
    pub fn snp_probe_rmptable_info() -> bool;
}
extern "C" {
    pub fn snp_rmptable_init() -> c_int;
}
extern "C" {
    pub fn snp_lookup_rmpentry(pfn: u64, assigned: *mut bool, level: *mut c_int) -> c_int;
}
extern "C" {
    pub fn snp_dump_hva_rmpentry(address: c_ulong);
}
extern "C" {
    pub fn psmash(pfn: u64) -> c_int;
}
extern "C" {
    pub fn rmp_make_private(pfn: u64, gpa: u64, level: pg_level, asid: u32, immutable: bool) -> c_int;
}
extern "C" {
    pub fn rmp_make_shared(pfn: u64, level: pg_level) -> c_int;
}
extern "C" {
    pub fn __snp_leak_pages(pfn: u64, npages: c_uint, dump_rmp: bool);
}
extern "C" {
    pub fn kdump_sev_callback();
}
extern "C" {
    pub fn snp_fixup_e820_tables();
}
extern "C" {
    pub fn snp_prepare() -> c_int;
}
extern "C" {
    pub fn snp_shutdown();
}

