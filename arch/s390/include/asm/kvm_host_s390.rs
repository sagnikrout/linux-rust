//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/kvm_host_s390.h
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
// definition for kernel virtual machines on s390
//
// Copyright IBM Corp. 2008, 2018
//
// Author(s): Carsten Otte <cotte@de.ibm.com>
//

// Macro flag: #define KVM_HAVE_MMU_RWLOCK
pub const KVM_MAX_VCPUS: c_int = 255;
pub const KVM_INTERNAL_MEM_SLOTS: c_int = 1;
pub const KVM_S390_MANAGES_S390_GUEST: c_int = 1;
//
// These seem to be used for allocating ->chip in the routing table, which we
// don't use. 1 is as small as we can get to reduce the needed memory. If we
// need to look at ->chip later on, we'll need to revisit this.
//
pub const KVM_NR_IRQCHIPS: c_int = 1;
pub const KVM_IRQCHIP_NUM_PINS: c_int = 1;
pub const KVM_HALT_POLL_NS_DEFAULT: c_int = 50000;
// s390-specific vcpu->requests bit members

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_stat {
    pub generic: kvm_vcpu_stat_generic,
    pub exit_userspace: u64,
    pub exit_null: u64,
    pub exit_external_request: u64,
    pub exit_io_request: u64,
    pub exit_external_interrupt: u64,
    pub exit_stop_request: u64,
    pub exit_validity: u64,
    pub exit_instruction: u64,
    pub exit_pei: u64,
    pub halt_no_poll_steal: u64,
    pub instruction_lctl: u64,
    pub instruction_lctlg: u64,
    pub instruction_stctl: u64,
    pub instruction_stctg: u64,
    pub exit_program_interruption: u64,
    pub exit_instr_and_program: u64,
    pub exit_operation_exception: u64,
    pub deliver_ckc: u64,
    pub deliver_cputm: u64,
    pub deliver_external_call: u64,
    pub deliver_emergency_signal: u64,
    pub deliver_service_signal: u64,
    pub deliver_virtio: u64,
    pub deliver_stop_signal: u64,
    pub deliver_prefix_signal: u64,
    pub deliver_restart_signal: u64,
    pub deliver_program: u64,
    pub deliver_io: u64,
    pub deliver_machine_check: u64,
    pub exit_wait_state: u64,
    pub inject_ckc: u64,
    pub inject_cputm: u64,
    pub inject_external_call: u64,
    pub inject_emergency_signal: u64,
    pub inject_mchk: u64,
    pub inject_pfault_init: u64,
    pub inject_program: u64,
    pub inject_restart: u64,
    pub inject_set_prefix: u64,
    pub inject_stop_signal: u64,
    pub instruction_epsw: u64,
    pub instruction_gs: u64,
    pub instruction_io_other: u64,
    pub instruction_lpsw: u64,
    pub instruction_lpswe: u64,
    pub instruction_lpswey: u64,
    pub instruction_pfmf: u64,
    pub instruction_ptff: u64,
    pub instruction_sck: u64,
    pub instruction_sckpf: u64,
    pub instruction_stidp: u64,
    pub instruction_spx: u64,
    pub instruction_stpx: u64,
    pub instruction_stap: u64,
    pub instruction_iske: u64,
    pub instruction_ri: u64,
    pub instruction_rrbe: u64,
    pub instruction_sske: u64,
    pub instruction_ipte_interlock: u64,
    pub instruction_stsi: u64,
    pub instruction_stfl: u64,
    pub instruction_tb: u64,
    pub instruction_tpi: u64,
    pub instruction_tprot: u64,
    pub instruction_tsch: u64,
    pub instruction_sie: u64,
    pub instruction_essa: u64,
    pub instruction_sthyi: u64,
    pub instruction_sigp_sense: u64,
    pub instruction_sigp_sense_running: u64,
    pub instruction_sigp_external_call: u64,
    pub instruction_sigp_emergency: u64,
    pub instruction_sigp_cond_emergency: u64,
    pub instruction_sigp_start: u64,
    pub instruction_sigp_stop: u64,
    pub instruction_sigp_stop_store_status: u64,
    pub instruction_sigp_store_status: u64,
    pub instruction_sigp_store_adtl_status: u64,
    pub instruction_sigp_arch: u64,
    pub instruction_sigp_prefix: u64,
    pub instruction_sigp_restart: u64,
    pub instruction_sigp_init_cpu_reset: u64,
    pub instruction_sigp_cpu_reset: u64,
    pub instruction_sigp_unknown: u64,
    pub instruction_diagnose_10: u64,
    pub instruction_diagnose_44: u64,
    pub instruction_diagnose_9c: u64,
    pub diag_9c_ignored: u64,
    pub diag_9c_forward: u64,
    pub instruction_diagnose_258: u64,
    pub instruction_diagnose_308: u64,
    pub instruction_diagnose_500: u64,
    pub instruction_diagnose_other: u64,
    pub pfault_sync: u64,
    pub signal_exits: u64,
}

// irq types in ascend order of priorities
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irq_types {
    IRQ_PEND_SET_PREFIX = 0,
    IRQ_PEND_RESTART,
    IRQ_PEND_SIGP_STOP,
    IRQ_PEND_IO_ISC_7,
    IRQ_PEND_IO_ISC_6,
    IRQ_PEND_IO_ISC_5,
    IRQ_PEND_IO_ISC_4,
    IRQ_PEND_IO_ISC_3,
    IRQ_PEND_IO_ISC_2,
    IRQ_PEND_IO_ISC_1,
    IRQ_PEND_IO_ISC_0,
    IRQ_PEND_VIRTIO,
    IRQ_PEND_PFAULT_DONE,
    IRQ_PEND_PFAULT_INIT,
    IRQ_PEND_EXT_HOST,
    IRQ_PEND_EXT_SERVICE,
    IRQ_PEND_EXT_SERVICE_EV,
    IRQ_PEND_EXT_TIMING,
    IRQ_PEND_EXT_CPU_TIMER,
    IRQ_PEND_EXT_CLOCK_COMP,
    IRQ_PEND_EXT_EXTERNAL,
    IRQ_PEND_EXT_EMERGENCY,
    IRQ_PEND_EXT_MALFUNC,
    IRQ_PEND_EXT_IRQ_KEY,
    IRQ_PEND_MCHK_REP,
    IRQ_PEND_PROG,
    IRQ_PEND_SVC,
    IRQ_PEND_MCHK_EX,
    IRQ_PEND_COUNT
}

// We have 2M for virtio device descriptor pages. Smallest amount of
// memory per page is 24 bytes (1 queue), so (2048*1024) / 24 = 87381
//
pub const KVM_S390_MAX_VIRTIO_IRQS: c_int = 87381;
//
// Repressible (non-floating) machine check interrupts
// subclass bits in MCIC
//
pub const MCHK_EXTD_BIT: c_int = 58;
pub const MCHK_DEGR_BIT: c_int = 56;
pub const MCHK_WARN_BIT: c_int = 55;

// Exigent machine check interrupts subclass bits in MCIC
pub const MCHK_SD_BIT: c_int = 63;
pub const MCHK_PD_BIT: c_int = 62;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_interrupt_info {
    pub list: list_head,
    pub type: u64,
    pub io: kvm_s390_io_info,
    pub ext: kvm_s390_ext_info,
    pub pgm: kvm_s390_pgm_info,
    pub emerg: kvm_s390_emerg_info,
    pub extcall: kvm_s390_extcall_info,
    pub prefix: kvm_s390_prefix_info,
    pub stop: kvm_s390_stop_info,
    pub mchk: kvm_s390_mchk_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_irq_payload {
    pub io: kvm_s390_io_info,
    pub ext: kvm_s390_ext_info,
    pub pgm: kvm_s390_pgm_info,
    pub emerg: kvm_s390_emerg_info,
    pub extcall: kvm_s390_extcall_info,
    pub prefix: kvm_s390_prefix_info,
    pub stop: kvm_s390_stop_info,
    pub mchk: kvm_s390_mchk_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_local_interrupt {
    pub lock: spinlock_t,
    pub KVM_MAX_VCPUS): DECLARE_BITMAP(sigp_emerg_pending,,
    pub irq: kvm_s390_irq_payload,
    pub pending_irqs: c_ulong,
}

pub const FIRQ_LIST_IO_ISC_0: c_int = 0;
pub const FIRQ_LIST_IO_ISC_1: c_int = 1;
pub const FIRQ_LIST_IO_ISC_2: c_int = 2;
pub const FIRQ_LIST_IO_ISC_3: c_int = 3;
pub const FIRQ_LIST_IO_ISC_4: c_int = 4;
pub const FIRQ_LIST_IO_ISC_5: c_int = 5;
pub const FIRQ_LIST_IO_ISC_6: c_int = 6;
pub const FIRQ_LIST_IO_ISC_7: c_int = 7;
pub const FIRQ_LIST_PFAULT: c_int = 8;
pub const FIRQ_LIST_VIRTIO: c_int = 9;
pub const FIRQ_LIST_COUNT: c_int = 10;
pub const FIRQ_CNTR_IO: c_int = 0;
pub const FIRQ_CNTR_SERVICE: c_int = 1;
pub const FIRQ_CNTR_VIRTIO: c_int = 2;
pub const FIRQ_CNTR_PFAULT: c_int = 3;
pub const FIRQ_MAX_COUNT: c_int = 4;
// mask the AIS mode for a given ISC

pub const KVM_S390_AIS_MODE_ALL: c_int = 0;
pub const KVM_S390_AIS_MODE_SINGLE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_float_interrupt {
    pub pending_irqs: c_ulong,
    pub masked_irqs: c_ulong,
    pub lock: spinlock_t,
    pub lists: [list_head; FIRQ_LIST_COUNT],
    pub counters: [c_int; FIRQ_MAX_COUNT],
    pub mchk: kvm_s390_mchk_info,
    pub srv_signal: kvm_s390_ext_info,
    pub last_sleep_cpu: c_int,
    pub ais_lock: spinlock_t,
    pub simm: u8,
    pub nimm: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_hw_wp_info_arch {
    pub addr: c_ulong,
    pub phys_addr: c_ulong,
    pub len: c_int,
    pub old_data: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_hw_bp_info_arch {
    pub addr: c_ulong,
    pub len: c_int,
}

//
// Only the upper 16 bits of kvm_guest_debug->control are arch specific.
// Further KVM_GUESTDBG flags which an be used from userspace can be found in
// arch/s390/include/uapi/asm/kvm.h
//
pub const KVM_GUESTDBG_EXIT_PENDING: c_uint = 0x10000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_guestdbg_info_arch {
    pub cr0: c_ulong,
    pub cr9: c_ulong,
    pub cr10: c_ulong,
    pub cr11: c_ulong,
    pub hw_bp_info: *mut kvm_hw_bp_info_arch,
    pub hw_wp_info: *mut kvm_hw_wp_info_arch,
    pub nr_hw_bp: c_int,
    pub nr_hw_wp: c_int,
    pub last_bp: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv_vcpu {
    pub handle: u64,
    pub stor_base: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_arch {
    pub sie_block: *mut kvm_s390_sie_block,
// if vsie is active, currently executed shadow sie control block
    pub vsie_block: *mut kvm_s390_sie_block,
    pub host_acrs: [c_uint; NUM_ACRS],
    pub host_gscb: *mut gs_cb,
    pub local_int: kvm_s390_local_interrupt,
    pub ckc_timer: hrtimer,
    pub pgm: kvm_s390_pgm_info,
    pub gmap: *mut gmap,
    pub guestdbg: kvm_guestdbg_info_arch,
    pub pfault_token: c_ulong,
    pub pfault_select: c_ulong,
    pub pfault_compare: c_ulong,
    pub cputm_enabled: bool,
//
// The seqcount protects updates to cputm_start and sie_block.cputm,
// this way we can have non-blocking reads with consistent values.
// Only the owning VCPU thread (vcpu->cpu) is allowed to change these
// values and to start/stop/enable/disable cpu timer accounting.
//
    pub cputm_seqcount: seqcount_t,
    pub cputm_start: __u64,
    pub gs_enabled: bool,
    pub skey_enabled: bool,
// Indicator if the access registers have been loaded from guest
    pub acrs_loaded: bool,
    pub initialized: bool,
    pub pv: kvm_s390_pv_vcpu,
    pub diag318_info: diag318_info,
    pub mc: *mut kvm_s390_mmu_cache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vm_stat {
    pub generic: kvm_vm_stat_generic,
    pub inject_io: u64,
    pub io_390_adapter_map: u64,
    pub io_390_adapter_unmap: u64,
    pub io_390_inatomic: u64,
    pub io_flic_inject_airq: u64,
    pub io_set_adapter_int: u64,
    pub io_390_inatomic_no_inject: u64,
    pub inject_float_mchk: u64,
    pub inject_pfault_done: u64,
    pub inject_service_signal: u64,
    pub inject_virtio: u64,
    pub aen_forward: u64,
    pub gmap_shadow_create: u64,
    pub gmap_shadow_reuse: u64,
    pub gmap_shadow_r1_entry: u64,
    pub gmap_shadow_r2_entry: u64,
    pub gmap_shadow_r3_entry: u64,
    pub gmap_shadow_sg_entry: u64,
    pub gmap_shadow_pg_entry: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arch_memory_slot {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_map_info {
    pub list: list_head,
    pub guest_addr: __u64,
    pub addr: __u64,
    pub page: *mut page,
//
// True if the page is long-term pinned. False if long-term pinning
// failed and this entry exists only to preserve MAP/UNMAP symmetry.
//
    pub pinned: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_io_adapter {
    pub id: c_uint,
    pub isc: c_int,
    pub maskable: bool,
    pub masked: bool,
    pub swap: bool,
    pub suppressible: bool,
    pub maps_lock: spinlock_t,
    pub maps: list_head,
    pub nr_maps: c_uint,
}

pub const MAX_S390_ADAPTER_MAPS: c_int = 256;
// maximum size of facilities and facility mask is 2k bytes

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_cpu_model {
// facility mask supported by kvm & hosting machine
    pub fac_mask: [__u64; S390_ARCH_FAC_MASK_SIZE_U64],
    pub subfuncs: kvm_s390_vm_cpu_subfunc,
// facility list requested by guest (in dma page)
    pub fac_list: *mut __u64,
    pub cpuid: u64,
    pub ibc: c_ushort,
// subset of available UV-features for pv-guests enabled by user space
    pub uv_feat_guest: kvm_s390_vm_cpu_uv_feat,
}

pub const S390_ARCH_FAC_FORMAT_2: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_flcb2 {
    pub reserved0: [u8; 7],
    pub length: u8,
}

extern "C" {
    pub fn int(vcpu: *mut *mut crypto_hook)(struct kvm_vcpu) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_crypto {
    pub crycb: *mut kvm_s390_crypto_cb,
    pub pqap_hook_rwsem: rw_semaphore,
    pub pqap_hook: *mut crypto_hook,
    pub crycbd: __u32,
    pub aes_kw: __u8,
    pub dea_kw: __u8,
    pub apie: __u8,
}

pub const APCB0_MASK_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_apcb0 {
    pub /: *mut *mut __u64 apm[APCB0_MASK_SIZE]; / 0x0000,
    pub /: *mut *mut __u64 aqm[APCB0_MASK_SIZE]; / 0x0008,
    pub /: *mut *mut __u64 adm[APCB0_MASK_SIZE]; / 0x0010,
    pub /: *mut *mut __u64 reserved18; / 0x0018,
}

pub const APCB1_MASK_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_apcb1 {
    pub /: *mut *mut __u64 apm[APCB1_MASK_SIZE]; / 0x0000,
    pub /: *mut *mut __u64 aqm[APCB1_MASK_SIZE]; / 0x0020,
    pub /: *mut *mut __u64 adm[APCB1_MASK_SIZE]; / 0x0040,
    pub /: *mut *mut __u64 reserved60[4]; / 0x0060,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_crypto_cb {
    pub /: *mut *mut kvm_s390_apcb0 apcb0; / 0x0000,
    pub /: *mut *mut __u8 reserved20[0x0048 - 0x0020]; / 0x0020,
    pub /: *mut *mut __u8 dea_wrapping_key_mask[24]; / 0x0048,
    pub /: *mut *mut __u8 aes_wrapping_key_mask[32]; / 0x0060,
    pub /: *mut *mut kvm_s390_apcb1 apcb1; / 0x0080,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_gisa {
    pub next_alert: u32,
    pub ipm: u8,
    pub reserved01: [u8; 2],
    pub iam: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_gib {
    pub alert_list_origin: u32,
    pub reserved01: u32,
    pub nisc:3: u8,
    pub reserved03: [u8; 3],
    pub reserved04: [u32; 5],
}

//
// sie_page2 has to be allocated as DMA because fac_list, crycb and
// gisa need 31bit addresses in the sie control block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sie_page2 {
    pub /: *mut *mut __u64 fac_list[S390_ARCH_FAC_LIST_SIZE_U64]; / 0x0000,
    pub /: *mut *mut kvm_s390_crypto_cb crycb; / 0x0800,
    pub /: *mut *mut kvm_s390_gisa gisa; / 0x0900,
    pub /: *mut *mut *mut kvm kvm; / 0x0920,
    pub /: *mut *mut u8 reserved928[0x1000 - 0x928]; / 0x0928,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_vsie {
    pub mutex: mutex,
    pub addr_to_page: radix_tree_root,
    pub page_count: c_int,
    pub next: c_int,
    pub pages: [*mut vsie_page; KVM_MAX_VCPUS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_gisa_iam {
    pub mask: u8,
    pub ref_lock: spinlock_t,
    pub 1]: u32 ref_count[MAX_ISC +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_gisa_interrupt {
    pub origin: *mut kvm_s390_gisa,
    pub alert: kvm_s390_gisa_iam,
    pub timer: hrtimer,
    pub expires: u64,
    pub KVM_MAX_VCPUS): DECLARE_BITMAP(kicked_mask,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv {
    pub handle: u64,
    pub guest_len: u64,
    pub stor_base: c_ulong,
    pub stor_var: *mut c_void,
    pub dumping: bool,
    pub set_aside: *mut c_void,
    pub need_cleanup: list_head,
    pub mmu_notifier: mmu_notifier,
// Protects against concurrent import-like operations
    pub import_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arch {
    pub sca: *mut esca_block,
    pub dbf: *mut debug_info_t,
    pub float_int: kvm_s390_float_interrupt,
    pub flic: *mut kvm_device,
    pub gmap: *mut gmap,
    pub mem_limit: c_ulong,
    pub css_support: c_int,
    pub use_irqchip: c_int,
    pub use_cmma: c_int,
    pub use_pfmfi: c_int,
    pub use_skf: c_int,
    pub use_zpci_interp: c_int,
    pub user_cpu_state_ctrl: c_int,
    pub user_sigp: c_int,
    pub user_stsi: c_int,
    pub user_instr0: c_int,
    pub user_operexec: c_int,
    pub allow_vsie_esamode: c_int,
    pub adapters: [*mut s390_io_adapter; MAX_S390_IO_ADAPTERS],
    pub ipte_wq: wait_queue_head_t,
    pub ipte_lock_count: c_int,
    pub ipte_mutex: mutex,
    pub start_stop_lock: spinlock_t,
    pub sie_page2: *mut sie_page2,
    pub model: kvm_s390_cpu_model,
    pub crypto: kvm_s390_crypto,
    pub vsie: kvm_s390_vsie,
    pub epdx: u8,
    pub epoch: u64,
    pub migration_mode: c_int,
    pub cmma_dirty_pages: core::sync::atomic::AtomicI64,
// subset of available cpu features enabled by user space
    pub KVM_S390_VM_CPU_FEAT_NR_BITS): DECLARE_BITMAP(cpu_feat,,
// indexed by vcpu_idx
    pub KVM_MAX_VCPUS): DECLARE_BITMAP(idle_mask,,
    pub gisa_int: kvm_s390_gisa_interrupt,
    pub pv: kvm_s390_pv,
    pub kzdev_list: list_head,
    pub kzdev_list_lock: spinlock_t,
    pub mc: *mut kvm_s390_mmu_cache,
}

extern "C" {
    pub fn IS_ERR_VALUE(_arg: addr) -> return;
}
pub const ASYNC_PF_PER_VCPU: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arch_async_pf {
    pub pfault_token: c_ulong,
}

extern "C" {
    pub fn kvm_arch_can_dequeue_async_page_present(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_arch_crypto_clear_masks(kvm: *mut kvm);
}
pub const SIE64_RETURN_NORMAL: c_int = 0;
pub const SIE64_RETURN_MCCK: c_int = 1;
extern "C" {
    pub fn __sie64a(_arg: virt_to_phys(sie_block), _arg: sie_block, _arg: rsa, _arg: gasce) -> return;
}
extern "C" {
    pub fn kvm_s390_pv_is_protected(kvm: *mut kvm) -> bool;
}
extern "C" {
    pub fn kvm_s390_pv_cpu_is_protected(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_s390_gisc_register(kvm: *mut kvm, gisc: u32) -> c_int;
}
extern "C" {
    pub fn kvm_s390_gisc_unregister(kvm: *mut kvm, gisc: u32) -> c_int;
}
extern "C" {
    pub fn kvm_s390_is_gpa_in_memslot(kvm: *mut kvm, gpa: gpa_t) -> bool;
}
extern "C" {
    pub fn kvm_arch_free_vm(kvm: *mut kvm);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_kvm_hook {
    pub kvm): *mut *mut *mut int (kvm_register)(void opaque, struct kvm,
    pub opaque): *mut *mut void (kvm_unregister)(void,
}
