#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate libc;
use jni::objects::*;
use jni::sys::{jbyteArray, jint, jlong};
use jni::*;
use winapi::shared::minwindef::{DWORD, HINSTANCE, LPVOID};
use winapi::um::winnt::{BOOLEAN, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_fileLength(
    env: JNIEnv,
    obj: JObject,
    fd: jint,
) -> jlong {
    unsafe {
        let mut file = PHYSFS_File {
            opaque: fd as *mut libc::c_void,
        };
        return PHYSFS_fileLength(&mut file);
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_tell(
    env: JNIEnv,
    obj: JObject,
    fd: jint,
) -> jlong {
    unsafe {
        let mut file = PHYSFS_File {
            opaque: fd as *mut libc::c_void,
        };
        return PHYSFS_tell(&mut file);
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_readBytes(
    env: JNIEnv,
    obj: JObject,
    fd: jint,
    java_buf: jbyteArray,
    len: jint,
) -> jint {
    unsafe {
        let mut file = PHYSFS_File {
            opaque: fd as *mut libc::c_void,
        };
        let mut vec = vec![0 as i8; len as usize];
        let c_buf = &mut vec[..];
        let res = PHYSFS_readBytes(
            &mut file,
            c_buf.as_mut_ptr() as *mut libc::c_void,
            len as PHYSFS_uint64,
        ) as jint;
        env.set_byte_array_region(java_buf, 0, c_buf).unwrap();
        return res;
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_openRead(
    env: JNIEnv,
    obj: JObject,
    file_name: JString,
) -> jint {
    unsafe {
        let physfs_file = *PHYSFS_openRead((**env.get_string(file_name).unwrap()).as_ptr());
        return physfs_file.opaque as jint;
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_seek(
    env: JNIEnv,
    obj: JObject,
    fd: jint,
    pos: jlong,
) -> jint {
    unsafe {
        let mut file = PHYSFS_File {
            opaque: fd as *mut libc::c_void,
        };
        return PHYSFS_seek(&mut file, pos as PHYSFS_uint64);
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_close(
    env: JNIEnv,
    obj: JObject,
    fd: jint,
) -> jint {
    unsafe {
        let mut file = PHYSFS_File {
            opaque: fd as *mut libc::c_void,
        };
        return PHYSFS_close(&mut file);
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_mount(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
    append: jint,
) -> jint {
    unsafe {
        return PHYSFS_mount(
            (**env.get_string(file_name).unwrap()).as_ptr(),
            std::ptr::null(),
            append,
        );
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_getLastErrorCode(
    env: JNIEnv,
    class: JClass,
) -> jint {
    unsafe {
        return PHYSFS_getLastErrorCode();
    }
}

#[no_mangle]
extern "system" fn DllMain(_dllHandle: HINSTANCE, reason: DWORD, _: LPVOID) -> BOOLEAN {
    unsafe {
        match reason {
            DLL_PROCESS_ATTACH => PHYSFS_init(std::ptr::null()) as BOOLEAN,
            DLL_PROCESS_DETACH => PHYSFS_deinit() as BOOLEAN,
            _ => 0,
        }
    }
}
