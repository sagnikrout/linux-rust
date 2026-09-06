//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/opal.h
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
// PowerNV OPAL definitions.
//
// Copyright 2011 IBM Corp.
//

// We calculate number of sg entries based on PAGE_SIZE

// Default time to sleep or delay between OPAL_BUSY/OPAL_BUSY_EVENT loops
pub const OPAL_BUSY_DELAY_MS: c_int = 10;
// /sys/firmware/opal
// /ibm,opal
// API functions
extern "C" {
    pub fn opal_invalid_call() -> i64;
}
extern "C" {
    pub fn opal_console_flush(term_number: i64) -> i64;
}
extern "C" {
    pub fn opal_tpo_read(token: u64, year_mon_day: *mut __be32, hour_min: *mut __be32) -> i64;
}
extern "C" {
    pub fn opal_cec_power_down(request: u64) -> i64;
}
extern "C" {
    pub fn opal_cec_reboot() -> i64;
}
extern "C" {
    pub fn opal_cec_reboot2(reboot_type: u32, diag: *const c_char) -> i64;
}
extern "C" {
    pub fn opal_read_nvram(buffer: u64, size: u64, offset: u64) -> i64;
}
extern "C" {
    pub fn opal_write_nvram(buffer: u64, size: u64, offset: u64) -> i64;
}
extern "C" {
    pub fn opal_handle_interrupt(isn: u64, outstanding_event_mask: *mut __be64) -> i64;
}
extern "C" {
    pub fn opal_poll_events(outstanding_event_mask: *mut __be64) -> i64;
}
extern "C" {
    pub fn opal_set_xive(isn: u32, server: u16, priority: u8) -> i64;
}
extern "C" {
    pub fn opal_get_xive(isn: u32, server: *mut __be16, priority: *mut u8) -> i64;
}
extern "C" {
    pub fn opal_pci_shpc(phb_id: u64, shpc_action: u64, state: *mut u8) -> i64;
}
extern "C" {
    pub fn opal_pci_msi_eoi(phb_id: u64, hw_irq: u32) -> i64;
}
extern "C" {
    pub fn opal_start_cpu(thread_number: u64, start_address: u64) -> i64;
}
extern "C" {
    pub fn opal_query_cpu_status(thread_number: u64, thread_status: *mut u8) -> i64;
}
extern "C" {
    pub fn opal_write_oppanel(lines: *mut oppanel_line_t, num_lines: u64) -> i64;
}
extern "C" {
    pub fn opal_pci_reset(id: u64, reset_scope: u8, assert_state: u8) -> i64;
}
extern "C" {
    pub fn opal_pci_fence_phb(phb_id: u64) -> i64;
}
extern "C" {
    pub fn opal_pci_reinit(phb_id: u64, reinit_scope: u64, data: u64) -> i64;
}
extern "C" {
    pub fn opal_pci_mask_pe_error(phb_id: u64, pe_number: u16, error_type: u8, mask_action: u8) -> i64;
}
extern "C" {
    pub fn opal_set_slot_led_status(phb_id: u64, slot_id: u64, led_type: u8, led_action: u8) -> i64;
}
extern "C" {
    pub fn opal_get_epow_status(epow_status: *mut __be16, num_epow_classes: *mut __be16) -> i64;
}
extern "C" {
    pub fn opal_get_dpo_status(dpo_timeout: *mut __be64) -> i64;
}
extern "C" {
    pub fn opal_set_system_attention_led(led_action: u8) -> i64;
}
extern "C" {
    pub fn opal_pci_poll(id: u64) -> i64;
}
extern "C" {
    pub fn opal_return_cpu() -> i64;
}
extern "C" {
    pub fn opal_check_token(token: u64) -> i64;
}
extern "C" {
    pub fn opal_reinit_cpus(flags: u64) -> i64;
}
extern "C" {
    pub fn opal_xscom_read(gcid: u32, pcb_addr: u64, val: *mut __be64) -> i64;
}
extern "C" {
    pub fn opal_xscom_write(gcid: u32, pcb_addr: u64, val: u64) -> i64;
}
extern "C" {
    pub fn opal_read_elog(buffer: u64, size: u64, log_id: u64) -> i64;
}
extern "C" {
    pub fn opal_get_elog_size(log_id: *mut __be64, size: *mut __be64, elog_type: *mut __be64) -> i64;
}
extern "C" {
    pub fn opal_write_elog(buffer: u64, size: u64, offset: u64) -> i64;
}
extern "C" {
    pub fn opal_send_ack_elog(log_id: u64) -> i64;
}
extern "C" {
    pub fn opal_resend_pending_logs();
}
extern "C" {
    pub fn opal_validate_flash(buffer: u64, size: *mut u32, result: *mut u32) -> i64;
}
extern "C" {
    pub fn opal_manage_flash(op: u8) -> i64;
}
extern "C" {
    pub fn opal_update_flash(blk_list: u64) -> i64;
}
extern "C" {
    pub fn opal_dump_init(dump_type: u8) -> i64;
}
extern "C" {
    pub fn opal_dump_info(dump_id: *mut __be32, dump_size: *mut __be32) -> i64;
}
extern "C" {
    pub fn opal_dump_info2(dump_id: *mut __be32, dump_size: *mut __be32, dump_type: *mut __be32) -> i64;
}
extern "C" {
    pub fn opal_dump_read(dump_id: u32, buffer: u64) -> i64;
}
extern "C" {
    pub fn opal_dump_ack(dump_id: u32) -> i64;
}
extern "C" {
    pub fn opal_dump_resend_notification() -> i64;
}
extern "C" {
    pub fn opal_get_msg(buffer: u64, size: u64) -> i64;
}
extern "C" {
    pub fn opal_check_completion(buffer: u64, size: u64, token: u64) -> i64;
}
extern "C" {
    pub fn opal_sync_host_reboot() -> i64;
}
extern "C" {
    pub fn opal_sensor_read(sensor_hndl: u32, token: c_int, sensor_data: *mut __be32) -> i64;
}
extern "C" {
    pub fn opal_sensor_read_u64(sensor_hndl: u32, token: c_int, sensor_data: *mut __be64) -> i64;
}
extern "C" {
    pub fn opal_handle_hmi() -> i64;
}
extern "C" {
    pub fn opal_handle_hmi2(out_flags: *mut __be64) -> i64;
}
extern "C" {
    pub fn opal_register_dump_region(id: u32, start: u64, end: u64) -> i64;
}
extern "C" {
    pub fn opal_unregister_dump_region(id: u32) -> i64;
}
extern "C" {
    pub fn opal_slw_set_reg(cpu_pir: u64, sprn: u64, val: u64) -> i64;
}
extern "C" {
    pub fn opal_config_cpu_idle_state(state: u64, flag: u64) -> i64;
}
extern "C" {
    pub fn opal_pci_set_phb_cxl_mode(phb_id: u64, mode: u64, pe_number: u64) -> i64;
}
extern "C" {
    pub fn opal_pci_get_pbcq_tunnel_bar(phb_id: u64, addr: *mut u64) -> i64;
}
extern "C" {
    pub fn opal_pci_set_pbcq_tunnel_bar(phb_id: u64, addr: u64) -> i64;
}
extern "C" {
    pub fn opal_prd_msg(msg: *mut opal_prd_msg) -> i64;
}
extern "C" {
    pub fn opal_get_device_tree(phandle: u32, buf: u64, len: u64) -> i64;
}
extern "C" {
    pub fn opal_pci_get_presence_state(id: u64, data: u64) -> i64;
}
extern "C" {
    pub fn opal_pci_get_power_state(id: u64, data: u64) -> i64;
}
extern "C" {
    pub fn opal_pci_poll2(id: u64, data: u64) -> i64;
}
extern "C" {
    pub fn opal_int_get_xirr(out_xirr: *mut __be32, just_poll: bool) -> i64;
}
extern "C" {
    pub fn opal_int_set_cppr(cppr: u8) -> i64;
}
extern "C" {
    pub fn opal_int_eoi(xirr: u32) -> i64;
}
extern "C" {
    pub fn opal_int_set_mfrr(cpu: u32, mfrr: u8) -> i64;
}
extern "C" {
    pub fn opal_nmmu_set_ptcr(chip_id: u64, ptcr: u64) -> i64;
}
extern "C" {
    pub fn opal_xive_reset(version: u64) -> i64;
}
extern "C" {
    pub fn opal_xive_donate_page(chip_id: u32, addr: u64) -> i64;
}
extern "C" {
    pub fn opal_xive_alloc_vp_block(alloc_order: u32) -> i64;
}
extern "C" {
    pub fn opal_xive_free_vp_block(vp: u64) -> i64;
}
extern "C" {
    pub fn opal_xive_allocate_irq_raw(chip_id: u32) -> i64;
}
extern "C" {
    pub fn opal_xive_free_irq(girq: u32) -> i64;
}
extern "C" {
    pub fn opal_xive_sync(type: u32, id: u32) -> i64;
}
extern "C" {
    pub fn opal_xive_dump(type: u32, id: u32) -> i64;
}
extern "C" {
    pub fn opal_xive_get_vp_state(vp: u64, out_w01: *mut __be64) -> i64;
}
extern "C" {
    pub fn opal_imc_counters_start(type: u32, cpu_pir: u64) -> i64;
}
extern "C" {
    pub fn opal_imc_counters_stop(type: u32, cpu_pir: u64) -> i64;
}
extern "C" {
    pub fn opal_get_powercap(handle: u32, token: c_int, pcap: *mut u32) -> c_int;
}
extern "C" {
    pub fn opal_set_powercap(handle: u32, token: c_int, pcap: u32) -> c_int;
}
extern "C" {
    pub fn opal_get_power_shift_ratio(handle: u32, token: c_int, psr: *mut u32) -> c_int;
}
extern "C" {
    pub fn opal_set_power_shift_ratio(handle: u32, token: c_int, psr: u32) -> c_int;
}
extern "C" {
    pub fn opal_sensor_group_clear(group_hndl: u32, token: c_int) -> c_int;
}
extern "C" {
    pub fn opal_sensor_group_enable(group_hndl: u32, token: c_int, enable: bool) -> c_int;
}
extern "C" {
    pub fn opal_nx_coproc_init(chip_id: u32, ct: u32) -> c_int;
}
extern "C" {
    pub fn opal_mpipl_update(op: opal_mpipl_ops, src: u64, dest: u64, size: u64) -> i64;
}
extern "C" {
    pub fn opal_mpipl_register_tag(tag: opal_mpipl_tags, addr: u64) -> i64;
}
extern "C" {
    pub fn opal_mpipl_query_tag(tag: opal_mpipl_tags, addr: *mut __be64) -> i64;
}
extern "C" {
    pub fn opal_signal_system_reset(cpu: i32) -> i64;
}
extern "C" {
    pub fn opal_quiesce(shutdown_type: u64, cpu: i32) -> i64;
}
// Internal functions
extern "C" {
    pub fn opal_configure_cores() -> void __init;
}
extern "C" {
    pub fn opal_get_chars(vtermno: u32, buf: *mut u8, count: usize) -> isize;
}
extern "C" {
    pub fn opal_flush_chars(vtermno: u32, wait: bool) -> c_int;
}
extern "C" {
    pub fn opal_flush_console(vtermno: u32) -> c_int;
}
extern "C" {
    pub fn hvc_opal_init_early();
}
extern "C" {
    pub fn opal_async_get_token_interruptible() -> c_int;
}
extern "C" {
    pub fn opal_async_release_token(token: c_int) -> c_int;
}
extern "C" {
    pub fn opal_async_wait_response(token: u64, msg: *mut opal_msg) -> c_int;
}
extern "C" {
    pub fn opal_get_sensor_data(sensor_hndl: u32, sensor_data: *mut u32) -> c_int;
}
extern "C" {
    pub fn opal_get_sensor_data_u64(sensor_hndl: u32, sensor_data: *mut u64) -> c_int;
}
extern "C" {
    pub fn sensor_group_enable(grp_hndl: u32, enable: bool) -> c_int;
}
extern "C" {
    pub fn opal_get_boot_time() -> time64_t;
}
extern "C" {
    pub fn opal_nvram_init();
}
extern "C" {
    pub fn opal_flash_update_init();
}
extern "C" {
    pub fn opal_flash_update_print_message();
}
extern "C" {
    pub fn opal_elog_init() -> c_int;
}
extern "C" {
    pub fn opal_platform_dump_init();
}
extern "C" {
    pub fn opal_sys_param_init();
}
extern "C" {
    pub fn opal_msglog_init();
}
extern "C" {
    pub fn opal_msglog_sysfs_init();
}
extern "C" {
    pub fn opal_async_comp_init() -> c_int;
}
extern "C" {
    pub fn opal_sensor_init() -> c_int;
}
extern "C" {
    pub fn opal_hmi_handler_init() -> c_int;
}
extern "C" {
    pub fn opal_event_init() -> c_int;
}
extern "C" {
    pub fn opal_power_control_init() -> c_int;
}
extern "C" {
    pub fn opal_machine_check(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn opal_mce_check_early_recovery(regs: *mut pt_regs) -> bool;
}
extern "C" {
    pub fn opal_hmi_exception_early(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn opal_hmi_exception_early2(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn opal_handle_hmi_exception(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn opal_shutdown();
}
extern "C" {
    pub fn opal_resync_timebase() -> c_int;
}
extern "C" {
    pub fn opal_lpc_init();
}
extern "C" {
    pub fn opal_kmsg_init();
}
extern "C" {
    pub fn opal_event_request(opal_event_nr: c_uint) -> c_int;
}
extern "C" {
    pub fn opal_free_sg_list(sg: *mut opal_sg_list);
}
extern "C" {
    pub fn opal_error_code(rc: c_int) -> c_int;
}
extern "C" {
    pub fn opal_msglog_copy(to: *mut c_char, pos: loff_t, count: usize) -> isize;
}
extern "C" {
    pub fn be64_to_cpu(_arg: msg.params[1]) -> return;
}
extern "C" {
    pub fn opal_wake_poller();
}
extern "C" {
    pub fn opal_powercap_init();
}
extern "C" {
    pub fn opal_psr_init();
}
extern "C" {
    pub fn opal_sensor_groups_init();
}

