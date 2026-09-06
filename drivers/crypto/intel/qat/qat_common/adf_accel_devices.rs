//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_accel_devices.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

pub const PCI_DEVICE_ID_INTEL_QAT_4XXXIOV: c_uint = 0x4941;
pub const PCI_DEVICE_ID_INTEL_QAT_401XXIOV: c_uint = 0x4943;
pub const PCI_DEVICE_ID_INTEL_QAT_402XXIOV: c_uint = 0x4945;
pub const PCI_DEVICE_ID_INTEL_QAT_420XXIOV: c_uint = 0x4947;
pub const PCI_DEVICE_ID_INTEL_QAT_6XXX_IOV: c_uint = 0x4949;
pub const ADF_DEVICE_FUSECTL_OFFSET: c_uint = 0x40;
pub const ADF_DEVICE_LEGFUSE_OFFSET: c_uint = 0x4C;
pub const ADF_DEVICE_FUSECTL_MASK: c_uint = 0x80000000;
pub const ADF_PCI_MAX_BARS: c_int = 3;
pub const ADF_DEVICE_NAME_LENGTH: c_int = 32;
pub const ADF_ETR_MAX_RINGS_PER_BANK: c_int = 16;
pub const ADF_MAX_MSIX_VECTOR_NAME: c_int = 48;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adf_accel_capabilities {
    ADF_ACCEL_CAPABILITIES_NULL = 0,
    ADF_ACCEL_CAPABILITIES_CRYPTO_SYMMETRIC = 1,
    ADF_ACCEL_CAPABILITIES_CRYPTO_ASYMMETRIC = 2,
    ADF_ACCEL_CAPABILITIES_CIPHER = 4,
    ADF_ACCEL_CAPABILITIES_AUTHENTICATION = 8,
    ADF_ACCEL_CAPABILITIES_COMPRESSION = 32,
    ADF_ACCEL_CAPABILITIES_LZS_COMPRESSION = 64,
    ADF_ACCEL_CAPABILITIES_RANDOM_NUMBER = 128
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adf_accel_capabilities_ext {
    ADF_ACCEL_CAPABILITIES_EXT_ZSTD_LZ4S = BIT(0),
    ADF_ACCEL_CAPABILITIES_EXT_ZSTD = BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adf_fuses {
    ADF_FUSECTL0,
    ADF_FUSECTL1,
    ADF_FUSECTL2,
    ADF_FUSECTL3,
    ADF_FUSECTL4,
    ADF_FUSECTL5,
    ADF_MAX_FUSES
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_bar {
    pub base_addr: resource_size_t,
    pub virt_addr: *mut void __iomem,
    pub size: resource_size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_irq {
    pub enabled: bool,
    pub name: [c_char; ADF_MAX_MSIX_VECTOR_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_accel_msix {
    pub irqs: *mut adf_irq,
    pub num_entries: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_accel_pci {
    pub pci_dev: *mut pci_dev,
    pub msix_entries: adf_accel_msix,
    pub pci_bars: [adf_bar; ADF_PCI_MAX_BARS],
    pub revid: u8,
    pub sku: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_state {
    DEV_DOWN = 0,
    DEV_UP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_sku_info {
    DEV_SKU_1 = 0,
    DEV_SKU_2,
    DEV_SKU_3,
    DEV_SKU_4,
    DEV_SKU_VF,
    DEV_SKU_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_errors {
    ADF_RAS_CORR,
    ADF_RAS_UNCORR,
    ADF_RAS_FATAL,
    ADF_RAS_ERRORS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_error_counters {
    pub counter: [core::sync::atomic::AtomicI32; ADF_RAS_ERRORS],
    pub sysfs_added: bool,
    pub enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_hw_device_class {
    pub name: *const c_char,
    pub type: adf_device_type,
    pub instances: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arb_info {
    pub arb_cfg: u32,
    pub arb_offset: u32,
    pub wt2sam_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct admin_info {
    pub admin_msg_ur: u32,
    pub admin_msg_lr: u32,
    pub mailbox_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_hw_csr_ops {
    pub size): *mut *mut u64 (build_csr_ring_base_addr)(dma_addr_t addr, u32,
    pub ring): u32,
    pub value): u32 ring, u32,
    pub ring): u32,
    pub value): u32 ring, u32,
    pub bank): *mut *mut *mut u32 (read_csr_stat)(void __iomem csr_base_addr, u32,
    pub bank): *mut *mut *mut u32 (read_csr_uo_stat)(void __iomem csr_base_addr, u32,
    pub bank): *mut *mut *mut u32 (read_csr_e_stat)(void __iomem csr_base_addr, u32,
    pub bank): *mut *mut *mut u32 (read_csr_ne_stat)(void __iomem csr_base_addr, u32,
    pub bank): *mut *mut *mut u32 (read_csr_nf_stat)(void __iomem csr_base_addr, u32,
    pub bank): *mut *mut *mut u32 (read_csr_f_stat)(void __iomem csr_base_addr, u32,
    pub bank): *mut *mut *mut u32 (read_csr_c_stat)(void __iomem csr_base_addr, u32,
    pub bank): *mut *mut *mut u32 (read_csr_exp_stat)(void __iomem csr_base_addr, u32,
    pub bank): *mut *mut *mut u32 (read_csr_exp_int_en)(void __iomem csr_base_addr, u32,
    pub value): u32,
    pub ring): u32,
    pub value): u32 ring, u32,
    pub ring): u32,
    pub addr): u32 ring, dma_addr_t,
    pub bank): *mut *mut *mut u32 (read_csr_int_en)(void __iomem csr_base_addr, u32,
    pub value): u32,
    pub bank): *mut *mut *mut u32 (read_csr_int_flag)(void __iomem csr_base_addr, u32,
    pub value): u32,
    pub bank): *mut *mut *mut u32 (read_csr_int_srcsel)(void __iomem csr_base_addr, u32,
    pub bank): *mut *mut *mut void (write_csr_int_srcsel)(void __iomem csr_base_addr, u32,
    pub value): u32 bank, u32,
    pub bank): *mut *mut *mut u32 (read_csr_int_col_en)(void __iomem csr_base_addr, u32,
    pub value): u32,
    pub bank): *mut *mut *mut u32 (read_csr_int_col_ctl)(void __iomem csr_base_addr, u32,
    pub value): u32,
    pub bank): u32,
    pub value): u32 bank, u32,
    pub bank): *mut *mut *mut u32 (read_csr_ring_srv_arb_en)(void __iomem csr_base_addr, u32,
    pub value): u32,
    pub (*get_int_col_ctl_enable_mask)(void): *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_ras_ops {
    pub accel_dev): *mut *mut void (enable_ras_errors)(struct adf_accel_dev,
    pub accel_dev): *mut *mut void (disable_ras_errors)(struct adf_accel_dev,
    pub reset_required): *mut bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_pfvf_ops {
    pub accel_dev): *mut *mut int (enable_comms)(struct adf_accel_dev,
    pub i): *mut *mut u32 (get_pf2vf_offset)(u32,
    pub i): *mut *mut u32 (get_vf2pf_offset)(u32,
    pub vf_mask): *mut *mut *mut void (enable_vf2pf_interrupts)(void __iomem pmisc_addr, u32,
    pub pmisc_addr): *mut *mut void (disable_all_vf2pf_interrupts)(void __iomem,
    pub pmisc_addr): *mut *mut u32 (disable_pending_vf2pf_interrupts)(void __iomem,
    pub csr_lock): *mut u32 pfvf_offset, struct mutex,
    pub compat_ver): u32 pfvf_offset, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_dc_ops {
    pub algo): *mut *mut *mut int (build_comp_block)(void ctx, enum adf_dc_algo,
    pub algo): *mut *mut *mut int (build_decomp_block)(void ctx, enum adf_dc_algo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_migdev_ops {
    pub mdev): *mut *mut int (init)(struct qat_mig_dev,
    pub mdev): *mut *mut void (cleanup)(struct qat_mig_dev,
    pub mdev): *mut *mut void (reset)(struct qat_mig_dev,
    pub mdev): *mut *mut int (open)(struct qat_mig_dev,
    pub mdev): *mut *mut void (close)(struct qat_mig_dev,
    pub mdev): *mut *mut int (suspend)(struct qat_mig_dev,
    pub mdev): *mut *mut int (resume)(struct qat_mig_dev,
    pub mdev): *mut *mut int (save_state)(struct qat_mig_dev,
    pub mdev): *mut *mut int (save_setup)(struct qat_mig_dev,
    pub mdev): *mut *mut int (load_state)(struct qat_mig_dev,
    pub size): *mut *mut *mut int (load_setup)(struct qat_mig_dev mdev, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_dev_err_mask {
    pub cppagentcmdpar_mask: u32,
    pub parerr_ath_cph_mask: u32,
    pub parerr_cpr_xlt_mask: u32,
    pub parerr_dcpr_ucs_mask: u32,
    pub parerr_pke_mask: u32,
    pub parerr_wat_wcp_mask: u32,
    pub ssmfeatren_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_hw_device_data {
    pub dev_class: *mut adf_hw_device_class,
    pub self): *mut *mut u32 (get_accel_mask)(struct adf_hw_device_data,
    pub self): *mut *mut u32 (get_ae_mask)(struct adf_hw_device_data,
    pub accel_dev): *mut *mut u32 (get_accel_cap)(struct adf_accel_dev,
    pub self): *mut *mut u32 (get_sram_bar_id)(struct adf_hw_device_data,
    pub self): *mut *mut u32 (get_misc_bar_id)(struct adf_hw_device_data,
    pub self): *mut *mut u32 (get_etr_bar_id)(struct adf_hw_device_data,
    pub self): *mut *mut u32 (get_num_aes)(struct adf_hw_device_data,
    pub self): *mut *mut u32 (get_num_accels)(struct adf_hw_device_data,
    pub arb_csrs_info): *mut *mut void (get_arb_info)(struct arb_info,
    pub admin_csrs_info): *mut *mut void (get_admin_info)(struct admin_info,
    pub self): *mut *mut dev_sku_info (get_sku)(struct adf_hw_device_data,
    pub accel_dev): *mut *mut u16 (get_ring_to_svc_map)(struct adf_accel_dev,
    pub accel_dev): *mut *mut int (alloc_irq)(struct adf_accel_dev,
    pub accel_dev): *mut *mut void (free_irq)(struct adf_accel_dev,
    pub accel_dev): *mut *mut void (enable_error_correction)(struct adf_accel_dev,
    pub accel_dev): *mut *mut int (init_admin_comms)(struct adf_accel_dev,
    pub accel_dev): *mut *mut void (exit_admin_comms)(struct adf_accel_dev,
    pub accel_dev): *mut *mut int (send_admin_init)(struct adf_accel_dev,
    pub accel_dev): *mut *mut int (start_timer)(struct adf_accel_dev,
    pub accel_dev): *mut *mut void (stop_timer)(struct adf_accel_dev,
    pub accel_dev): *mut *mut void (check_hb_ctrs)(struct adf_accel_dev,
    pub self): *mut *mut uint32_t (get_hb_clock)(struct adf_hw_device_data,
    pub accel_dev): *mut *mut int (measure_clock)(struct adf_accel_dev,
    pub accel_dev): *mut *mut int (init_arb)(struct adf_accel_dev,
    pub accel_dev): *mut *mut void (exit_arb)(struct adf_accel_dev,
    pub accel_dev): *const *const *const u32 (get_arb_mapping)(struct adf_accel_dev,
    pub accel_dev): *mut *mut int (init_device)(struct adf_accel_dev,
    pub accel_dev): *mut *mut int (enable_pm)(struct adf_accel_dev,
    pub accel_dev): *mut *mut bool (handle_pm_interrupt)(struct adf_accel_dev,
    pub accel_dev): *mut *mut void (disable_iov)(struct adf_accel_dev,
    pub enable): bool,
    pub accel_dev): *mut *mut void (enable_ints)(struct adf_accel_dev,
    pub accel_dev): *mut *mut void (set_ssm_wdtimer)(struct adf_accel_dev,
    pub bank_nr): *mut *mut *mut int (ring_pair_reset)(struct adf_accel_dev accel_dev, u32,
    pub state): *mut adf_bank_state,
    pub state): *mut u32 bank_number, struct adf_bank_state,
    pub accel_dev): *mut *mut void (reset_device)(struct adf_accel_dev,
    pub accel_dev): *mut *mut void (set_msix_rttable)(struct adf_accel_dev,
    pub obj_num): *const *const *const *const char (uof_get_name)(struct adf_accel_dev accel_dev, u32,
    pub accel_dev): *mut *mut u32 (uof_get_num_objs)(struct adf_accel_dev,
    pub obj_num): *mut *mut *mut int (uof_get_obj_type)(struct adf_accel_dev accel_dev, u32,
    pub obj_num): *mut *mut *mut u32 (uof_get_ae_mask)(struct adf_accel_dev accel_dev, u32,
    pub ae_mask): *mut *mut *mut int (get_rp_group)(struct adf_accel_dev accel_dev, u32,
    pub obj_num): *mut *mut *mut u32 (get_ena_thd_mask)(struct adf_accel_dev accel_dev, u32,
    pub accel_dev): *mut *mut int (dev_config)(struct adf_accel_dev,
    pub mask): *mut *mut bool (services_supported)(unsigned long,
    pub svc): adf_base_services,
    pub pfvf_ops: adf_pfvf_ops,
    pub csr_ops: adf_hw_csr_ops,
    pub dc_ops: adf_dc_ops,
    pub ras_ops: adf_ras_ops,
    pub dev_err_mask: adf_dev_err_mask,
    pub rl_data: adf_rl_hw_data,
    pub tl_data: adf_tl_hw_data,
    pub anti_rb_data: adf_anti_rb_hw_data,
    pub kpt_data: adf_kpt_hw_data,
    pub vfmig_ops: qat_migdev_ops,
    pub fw_name: *const c_char,
    pub fw_mmp_name: *const c_char,
    pub fuses: [u32; ADF_MAX_FUSES],
    pub straps: u32,
    pub accel_capabilities_mask: u32,
    pub accel_capabilities_ext_mask: u32,
    pub extended_dc_capabilities: u32,
    pub fw_capabilities: u16,
    pub clock_frequency: u32,
    pub instance_id: u32,
    pub accel_mask: u16,
    pub ae_mask: u32,
    pub admin_ae_mask: u32,
    pub tx_rings_mask: u16,
    pub ring_to_svc_map: u16,
    pub thd_to_arb_map: [u32; ICP_QAT_HW_AE_DELIMITER],
    pub tx_rx_gap: u8,
    pub num_banks: u8,
    pub num_banks_per_vf: u16,
    pub num_rings_per_bank: u8,
    pub num_accel: u8,
    pub num_logical_accel: u8,
    pub num_engines: u8,
    pub num_hb_ctrs: u32,
    pub num_rps: u8,
}

// CSR write macro

//
// CSR write macro to handle cases where the high and low
// offsets are sparsely located.
//

// CSR read macro

pub const ADF_CFG_NUM_SERVICES: c_int = 4;
pub const ADF_SRV_TYPE_BIT_LEN: c_int = 3;
pub const ADF_SRV_TYPE_MASK: c_uint = 0x7;
pub const ADF_AE_ADMIN_THREAD: c_int = 7;
pub const ADF_NUM_THREADS_PER_AE: c_int = 8;
pub const ADF_NUM_PKE_STRAND: c_int = 2;
pub const ADF_AE_STRAND0_THREAD: c_int = 8;
pub const ADF_AE_STRAND1_THREAD: c_int = 9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_fw_loader_data {
    pub fw_loader: *mut icp_qat_fw_loader_handle,
    pub uof_fw: *const firmware,
    pub mmp_fw: *const firmware,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_accel_vf_info {
    pub accel_dev: *mut adf_accel_dev,
    pub /: *mut *mut mutex pf2vf_lock; / protect CSR access for PF2VF messages,
    pub /: *mut *mut mutex pfvf_mig_lock; / protects PFVF state for migration,
    pub vf2pf_ratelimit: ratelimit_state,
    pub vf_nr: u32,
    pub init: bool,
    pub restarting: bool,
    pub vf_compat_ver: u8,
//
// Private area used for device migration.
// Memory allocation and free is managed by migration driver.
//
    pub mig_priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_dc_data {
    pub ovf_buff: *mut u8,
    pub ovf_buff_sz: usize,
    pub ovf_buff_p: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_pm {
    pub debugfs_pm_status: *mut dentry,
    pub present: bool,
    pub idle_irq_counters: c_int,
    pub throttle_irq_counters: c_int,
    pub fw_irq_counters: c_int,
    pub host_ack_counter: c_int,
    pub host_nack_counter: c_int,
    pub pos): *mut *mut char __user buf, size_t count, loff_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_sysfs {
    pub ring_num: c_int,
    pub /: *mut *mut rw_semaphore lock; / protects access to the fields in this struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_accel_dev {
    pub transport: *mut adf_etr_data,
    pub hw_device: *mut adf_hw_device_data,
    pub cfg: *mut adf_cfg_device_data,
    pub fw_loader: *mut adf_fw_loader_data,
    pub admin: *mut adf_admin_comms,
    pub telemetry: *mut adf_telemetry,
    pub dc_data: *mut adf_dc_data,
    pub power_management: adf_pm,
    pub crypto_list: list_head,
    pub compression_list: list_head,
    pub status: c_ulong,
    pub ref_count: core::sync::atomic::AtomicI32,
    pub debugfs_dir: *mut dentry,
    pub fw_cntr_dbgfile: *mut dentry,
    pub cnv_dbgfile: *mut dentry,
    pub list: list_head,
    pub owner: *mut module,
    pub accel_pci_dev: adf_accel_pci,
    pub timer: *mut adf_timer,
    pub heartbeat: *mut adf_heartbeat,
    pub rate_limiting: *mut adf_rl,
    pub sysfs: adf_sysfs,
// protects VF2PF interrupts access
    pub vf2pf_ints_lock: spinlock_t,
// prevents VF2PF handling from racing with VF state teardown
    pub vf2pf_disabled: bool,
// vf_info is non-zero when SR-IOV is init'ed
    pub vf_info: *mut adf_accel_vf_info,
    pub pf: },
    pub irq_enabled: bool,
    pub irq_name: [c_char; ADF_MAX_MSIX_VECTOR_NAME],
    pub pf2vf_bh_tasklet: tasklet_struct,
    pub /: *mut *mut mutex vf2pf_lock; / protect CSR access,
    pub msg_received: completion,
    pub /: *mut *mut pfvf_message response; / temp field holding pf2vf response,
    pub pf_compat_ver: u8,
    pub vf: },
}
