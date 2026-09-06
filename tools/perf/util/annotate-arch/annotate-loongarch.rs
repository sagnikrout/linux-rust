//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/annotate-arch/annotate-loongarch.c
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
// Perf annotate functions.
//
// Copyright (C) 2020-2023 Loongson Technology Corporation Limited
//

    static int loongarch_call__parse(const struct arch *arch, struct ins_operands *ops,
    struct map_symbol *ms,
    struct disasm_line *dl __maybe_unused)
    {
    char *c, *endptr, *tok, *name;
    struct map *map = ms.map;
    struct addr_map_symbol target;
    c = strchr(ops.raw, '#');
    if (c++ == core::ptr::null_mut())
    return -1;
    ops.target.addr = strtoull(c, &endptr, 16);
    name = strchr(endptr, '<');
    name++;
    if (arch.objdump.skip_functions_char &&
    strchr(name, arch.objdump.skip_functions_char))
    return -1;
    tok = strchr(name, '>');
    if (tok == core::ptr::null_mut())
    return -1;
// tok = '\0';
    ops.target.name = strdup(name);
// tok = '>';
    if (ops.target.name == core::ptr::null_mut())
    return -1;
    target = (struct addr_map_symbol) {
    .ms = { .map = map__get(map), },
    .addr = map__objdump_2mem(map, ops.target.addr),
    };
    if (maps__find_ams(thread__maps(ms.thread), &target) == 0 &&
    map__rip_2objdump(target.ms.map, map__map_ip(target.ms.map, target.addr)) == ops.target.addr)
    ops.target.sym = target.ms.sym;
    addr_map_symbol__exit(&target);
    return 0;
    }
    static const struct ins_ops loongarch_call_ops = {
    .parse	   = loongarch_call__parse,
    .scnprintf = call__scnprintf,
    .is_call   = true,
    };
    static int loongarch_jump__parse(const struct arch *arch, struct ins_operands *ops,
    struct map_symbol *ms,
    struct disasm_line *dl __maybe_unused)
    {
    struct map *map = ms.map;
    struct symbol *sym = ms.sym;
    struct addr_map_symbol target = {
    .ms = { .map = map__get(map), },
    };
    const char *c = strchr(ops.raw, '#');
    u64 start, end;
    ops.jump.raw_comment = strchr(ops.raw, arch.objdump.comment_char);
    ops.jump.raw_func_start = strchr(ops.raw, '<');
    if (ops.jump.raw_func_start && c > ops.jump.raw_func_start)
    c = core::ptr::null_mut();
    if (c++ != core::ptr::null_mut())
    ops.target.addr = strtoull(c, core::ptr::null_mut(), 16);
    else
    ops.target.addr = strtoull(ops.raw, core::ptr::null_mut(), 16);
    target.addr = map__objdump_2mem(map, ops.target.addr);
    start = map__unmap_ip(map, sym.start);
    end = map__unmap_ip(map, sym.end);
    ops.target.outside = target.addr < start || target.addr >= end;
    if (maps__find_ams(thread__maps(ms.thread), &target) == 0 &&
    map__rip_2objdump(target.ms.map, map__map_ip(target.ms.map, target.addr)) == ops.target.addr)
    ops.target.sym = target.ms.sym;
    if (!ops.target.outside) {
    ops.target.offset = target.addr - start;
    ops.target.offset_avail = true;
    } else {
    ops.target.offset_avail = false;
    }
    addr_map_symbol__exit(&target);
    return 0;
    }
    static const struct ins_ops loongarch_jump_ops = {
    .free	   = jump__delete,
    .parse	   = loongarch_jump__parse,
    .scnprintf = jump__scnprintf,
    .is_jump   = true,
    };
    static
    const struct ins_ops *loongarch__associate_ins_ops(struct arch *arch, const char *name)
    {
    const struct ins_ops *ops = core::ptr::null_mut();
    if (!strcmp(name, "bl"))
    ops = &loongarch_call_ops;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(name, _arg: "jirl")) -> else {
    else if (!strcmp(name, "jirl"))
    ops = &ret_ops;
    else if (!strcmp(name, "b") ||
    !strncmp(name, "beq", 3) ||
    !strncmp(name, "bne", 3) ||
    !strncmp(name, "blt", 3) ||
    !strncmp(name, "bge", 3) ||
    !strncmp(name, "bltu", 4) ||
    !strncmp(name, "bgeu", 4))
    ops = &loongarch_jump_ops;
    else
    return core::ptr::null_mut();
    arch__associate_ins_ops(arch, name, ops);
    return ops;
    }
    const struct arch *arch__new_loongarch(const struct e_machine_and_e_flags *id,
    const char *cpuid __maybe_unused)
    {
    struct arch *arch = zalloc(sizeof(*arch));
    if (!arch)
    return core::ptr::null_mut();
    arch.name = "loongarch";
    arch.id = *id;
    arch.associate_instruction_ops = loongarch__associate_ins_ops;
    arch.objdump.comment_char = '#';
    return arch;
    }
