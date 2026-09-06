//! Automatically rewritten from C to Rust
//! Source: tools/perf/scripts/python/Perf-Trace-Util/Context.c
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
// Context.c.  Python interfaces for perf script.
//
// Copyright (C) 2010 Tom Zanussi <tzanussi@gmail.com>
//
// Use Py_ssize_t for '#' formats to avoid DeprecationWarning: PY_SSIZE_T_CLEAN
// will be required for '#' formats.
//
// Macro flag: #define PY_SSIZE_T_CLEAN

    PyCapsule_GetPointer((arg1), (arg2))

    PyBytes_FromStringAndSize((arg1), (arg2))

    PyUnicode_AsUTF8(arg)
    PyMODINIT_FUNC PyInit_perf_trace_context(void);
    static struct scripting_context *get_args(PyObject *args, const char *name, PyObject **arg2)
    {
    let mut cnt: c_int = 1 + !!arg2;
    PyObject *context;
    if (!PyArg_UnpackTuple(args, name, 1, cnt, &context, arg2))
    return core::ptr::null_mut();
    return _PyCapsule_GetPointer(context, core::ptr::null_mut());
    }
    static struct scripting_context *get_scripting_context(PyObject *args)
    {
    return get_args(args, "context", core::ptr::null_mut());
    }

    static PyObject *perf_trace_context_common_pc(PyObject *obj, PyObject *args)
    {
    struct scripting_context *c = get_scripting_context(args);
    if (!c)
    return core::ptr::null_mut();
    return Py_BuildValue("i", common_pc(c));
    }
    static PyObject *perf_trace_context_common_flags(PyObject *obj,
    PyObject *args)
    {
    struct scripting_context *c = get_scripting_context(args);
    if (!c)
    return core::ptr::null_mut();
    return Py_BuildValue("i", common_flags(c));
    }
    static PyObject *perf_trace_context_common_lock_depth(PyObject *obj,
    PyObject *args)
    {
    struct scripting_context *c = get_scripting_context(args);
    if (!c)
    return core::ptr::null_mut();
    return Py_BuildValue("i", common_lock_depth(c));
    }

    static PyObject *perf_sample_insn(PyObject *obj, PyObject *args)
    {
    struct scripting_context *c = get_scripting_context(args);
    if (!c)
    return core::ptr::null_mut();
    if (c.sample.ip && !c.sample.insn_len && thread__maps(c.al.thread)) {
    struct machine *machine =  maps__machine(thread__maps(c.al.thread));
    perf_sample__fetch_insn(c.sample, c.al.thread, machine);
    }
    if (!c.sample.insn_len)
    Py_RETURN_NONE; /* N.B. This is a return statement */
    return _PyBytes_FromStringAndSize(c.sample.insn, c.sample.insn_len);
    }
    static PyObject *perf_set_itrace_options(PyObject *obj, PyObject *args)
    {
    struct scripting_context *c;
    const char *itrace_options;
    let mut retval: c_int = -1;
    PyObject *str;
    c = get_args(args, "itrace_options", &str);
    if (!c)
    return core::ptr::null_mut();
    if (!c.session || !c.session.itrace_synth_opts)
    goto out;
    if (c.session.itrace_synth_opts.set) {
    retval = 1;
    goto out;
    }
    itrace_options = _PyUnicode_AsUTF8(str);
    retval = itrace_do_parse_synth_opts(c.session.itrace_synth_opts, itrace_options, 0);
    out:
    return Py_BuildValue("i", retval);
    }
    static PyObject *perf_sample_src(PyObject *obj, PyObject *args, bool get_srccode)
    {
    struct scripting_context *c = get_scripting_context(args);
    let mut line: c_uint = 0;
    char *srcfile = core::ptr::null_mut();
    char *srccode = core::ptr::null_mut();
    PyObject *result;
    struct map *map;
    struct dso *dso;
    let mut len: c_int = 0;
    u64 addr;
    if (!c)
    return core::ptr::null_mut();
    map = c.al.map;
    addr = c.al.addr;
    dso = map ? map__dso(map) : core::ptr::null_mut();
    if (dso)
    srcfile = get_srcline_split(dso, map__rip_2objdump(map, addr), &line);
    if (get_srccode) {
    if (srcfile)
    srccode = find_sourceline(srcfile, line, &len);
    result = Py_BuildValue("(sIs#)", srcfile, line, srccode, (Py_ssize_t)len);
    } else {
    result = Py_BuildValue("(sI)", srcfile, line);
    }
    free(srcfile);
    return result;
    }
    static PyObject *perf_sample_srcline(PyObject *obj, PyObject *args)
    {
    return perf_sample_src(obj, args, false);
    }
    static PyObject *perf_sample_srccode(PyObject *obj, PyObject *args)
    {
    return perf_sample_src(obj, args, true);
    }
    static PyObject *__perf_config_get(PyObject *obj, PyObject *args)
    {
    const char *config_name;
    if (!PyArg_ParseTuple(args, "s", &config_name))
    return core::ptr::null_mut();
    return Py_BuildValue("s", perf_config_get(config_name));
    }
    static PyMethodDef ContextMethods[] = {

    { "common_pc", perf_trace_context_common_pc, METH_VARARGS,
    "Get the common preempt count event field value."},
    { "common_flags", perf_trace_context_common_flags, METH_VARARGS,
    "Get the common flags event field value."},
    { "common_lock_depth", perf_trace_context_common_lock_depth,
    METH_VARARGS,	"Get the common lock depth event field value."},

    { "perf_sample_insn", perf_sample_insn,
    METH_VARARGS,	"Get the machine code instruction."},
    { "perf_set_itrace_options", perf_set_itrace_options,
    METH_VARARGS,	"Set --itrace options."},
    { "perf_sample_srcline", perf_sample_srcline,
    METH_VARARGS,	"Get source file name and line number."},
    { "perf_sample_srccode", perf_sample_srccode,
    METH_VARARGS,	"Get source file name, line number and line."},
    { "perf_config_get", __perf_config_get, METH_VARARGS, "Get perf config entry"},
    { core::ptr::null_mut(), core::ptr::null_mut(), 0, core::ptr::null_mut()}
    };
#[no_mangle]
pub unsafe extern "C" fn PyInit_perf_trace_context() -> PyMODINIT_FUNC {
    PyMODINIT_FUNC PyInit_perf_trace_context(void)
    {
    static struct PyModuleDef moduledef = {
    PyModuleDef_HEAD_INIT,
    "perf_trace_context",	/* m_name */
    "",			/* m_doc */
    -1,			/* m_size */
    ContextMethods,		/* m_methods */
    core::ptr::null_mut(),			/* m_reload */
    core::ptr::null_mut(),			/* m_traverse */
    core::ptr::null_mut(),			/* m_clear */
    core::ptr::null_mut(),			/* m_free */
    };
    PyObject *mod;
    mod = PyModule_Create(&moduledef);
// Add perf_script_context to the module so it can be imported
    PyObject_SetAttrString(mod, "perf_script_context", Py_None);
    return mod;
    }
