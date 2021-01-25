#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate libc;
use jni::objects::*;
use jni::sys::{jbyteArray, jint, jlong};
use jni::*;
use std::ffi::CString;
use winapi::shared::minwindef::{DWORD, HINSTANCE, LPVOID};
use winapi::shared::ntdef::TRUE;
use winapi::um::winnt::BOOLEAN;

fn cppErrPrint(env: JNIEnv, message: String) {
    let newline_string = format!("{}\n", message);
    let jstring = env.new_string(newline_string).unwrap();
    env.call_static_method(
        "com/maddox/rts/RTS",
        "cppErrPrint",
        "(Ljava/lang/String;)V",
        &[JValue::Object(*jstring)],
    ).unwrap();
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_fileLength(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jlong {
    let file = fd as *mut PHYSFS_File;

    if cfg!(debug_assertions) {
        cppErrPrint(
            env,
            format!("PhysFSInputStream#fileLength for file {:?}", file),
        );
    }

    unsafe { PHYSFS_fileLength(file) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_tell(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jlong {
    let file = fd as *mut PHYSFS_File;

    if cfg!(debug_assertions) {
        cppErrPrint(env, format!("PhysFSInputStream#tell for file {:?}", file));
    }

    unsafe { PHYSFS_tell(file) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_readBytes(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
    java_buf: jbyteArray,
    offset: jint,
    len: jint,
) -> jint {
    let file = fd as *mut PHYSFS_File;

    if cfg!(debug_assertions) {
        cppErrPrint(
            env,
            format!(
                "PhysFSInputStream#readBytes for file {:?} offset {} len {}",
                file, offset, len
            ),
        );
    }

    unsafe {
        let mut vec = vec![0 as i8; len as usize];
        let c_buf = &mut vec[..];

        let res = PHYSFS_readBytes(
            file,
            c_buf.as_mut_ptr() as *mut libc::c_void,
            len as PHYSFS_uint64,
        ) as jint;

        env.set_byte_array_region(java_buf, offset, c_buf).unwrap();

        res
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_openRead(
    env: JNIEnv,
    obj: JObject,
    file_name: JString,
) -> jlong {
    let file_java_str = env.get_string(file_name).unwrap();

    let file_str = file_java_str
        .to_str()
        .unwrap()
        .replace("\\", "/")
        .to_ascii_uppercase()
        .replace(".CLASS", ".class");

    let file_c_str = CString::new(file_str.clone()).unwrap();

    if cfg!(debug_assertions) {
        cppErrPrint(
            env,
            format!("PhysFSInputStream#openRead for file {}", file_str),
        );
    }

    unsafe { PHYSFS_openRead(file_c_str.as_ptr()) as jlong }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_seek(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
    pos: jlong,
) -> jint {
    let file = fd as *mut PHYSFS_File;

    if cfg!(debug_assertions) {
        cppErrPrint(
            env,
            format!("PhysFSInputStream#seek for file {:?} pos {}", file, pos),
        );
    }

    unsafe { PHYSFS_seek(file, pos as PHYSFS_uint64) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_eof(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jint {
    let file = fd as *mut PHYSFS_File;

    if cfg!(debug_assertions) {
        cppErrPrint(env, format!("PhysFSInputStream#eof for file {:?}", file));
    }

    unsafe { PHYSFS_eof(file) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_close(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jint {
    let file = fd as *mut PHYSFS_File;

    if cfg!(debug_assertions) {
        cppErrPrint(env, format!("PhysFSInputStream#close for file {:?}", file));
    }

    unsafe { PHYSFS_close(file) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_init(
    env: JNIEnv,
    class: JClass,
) -> jint {
    if cfg!(debug_assertions) {
        cppErrPrint(env, "PhysFS.init".to_string());
    }

    unsafe { PHYSFS_init(std::ptr::null()) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_deinit(
    env: JNIEnv,
    class: JClass,
) -> jint {
    if cfg!(debug_assertions) {
        cppErrPrint(env, "PhysFS.deinit".to_string());
    }

    unsafe { PHYSFS_deinit() }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_exists(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
) -> jint {
    let file_java_str = env.get_string(file_name).unwrap();

    let file_str = file_java_str
        .to_str()
        .unwrap()
        .replace("\\", "/")
        .to_ascii_uppercase()
        .replace(".CLASS", ".class");

    let file_c_str = CString::new(file_str.clone()).unwrap();

    if cfg!(debug_assertions) {
        cppErrPrint(
            env,
            format!("PhysFS.exists for file {}", file_str),
        );
    }

    unsafe { PHYSFS_exists(file_c_str.as_ptr()) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_mount(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
    append: jint,
) -> jint {
    let file_str = env.get_string(file_name).unwrap();

    if cfg!(debug_assertions) {
        cppErrPrint(
            env,
            format!(
                "PhysFS.mount for file {} append {}",
                file_str.to_str().unwrap(),
                append
            ),
        );
    }

    unsafe { PHYSFS_mount(file_str.as_ptr(), std::ptr::null(), append) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_mountAt(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
    mount_point: JString,
    append: jint,
) -> jint {
    let file_str = env.get_string(file_name).unwrap();

    let mount_point_java_str = env.get_string(mount_point).unwrap();
    let mount_point_str = mount_point_java_str.to_str().unwrap().to_ascii_uppercase();
    let mount_point_c_str = CString::new(mount_point_str.clone()).unwrap();

    if cfg!(debug_assertions) {
        cppErrPrint(
            env,
            format!(
                "PhysFS.mountAt for file {} mount_point {} append {}",
                file_str.to_str().unwrap(),
                mount_point_str,
                append
            ),
        );
    }

    unsafe { PHYSFS_mount(file_str.as_ptr(), mount_point_c_str.as_ptr(), append) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_unmount(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
) -> jint {
    let file_str = env.get_string(file_name).unwrap();

    if cfg!(debug_assertions) {
        cppErrPrint(
            env,
            format!("PhysFS.unmount for file {}", file_str.to_str().unwrap()),
        );
    }

    unsafe { PHYSFS_unmount(file_str.as_ptr()) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_getLastErrorCode(
    env: JNIEnv,
    class: JClass,
) -> jint {
    if cfg!(debug_assertions) {
        cppErrPrint(env, "PhysFS.getLastErrorCode".to_string());
    }

    unsafe { PHYSFS_getLastErrorCode() }
}

#[allow(unused_variables)]
#[no_mangle]
extern "system" fn DllMain(dllHandle: HINSTANCE, reason: DWORD, reserved: LPVOID) -> BOOLEAN {
    return TRUE;
}
