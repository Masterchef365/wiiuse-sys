extern crate bindgen;
use bindgen::Builder;

fn main() {
    println!("cargo:rustc-link-lib=wiiuse");

    let bindings = win_blacklists(Builder::default())
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path: std::path::PathBuf = std::env::var("OUT_DIR").unwrap().into();
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

/// Blacklists for Windows according to https://github.com/rust-lang/rust-bindgen/issues/1556
fn win_blacklists(builder: Builder) -> Builder {
    builder
        .blacklist_type("LPMONITORINFOEXA?W?")
        .blacklist_type("LPTOP_LEVEL_EXCEPTION_FILTER")
        .blacklist_type("MONITORINFOEXA?W?")
        .blacklist_type("PEXCEPTION_FILTER")
        .blacklist_type("PEXCEPTION_ROUTINE")
        .blacklist_type("PSLIST_HEADER")
        .blacklist_type("PTOP_LEVEL_EXCEPTION_FILTER")
        .blacklist_type("PVECTORED_EXCEPTION_HANDLER")
        .blacklist_type("_?L?P?CONTEXT")
        .blacklist_type("_?L?P?EXCEPTION_POINTERS")
        .blacklist_type("_?P?DISPATCHER_CONTEXT")
        .blacklist_type("_?P?EXCEPTION_REGISTRATION_RECORD")
        .blacklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
        .blacklist_type("_?P?NT_TIB")
        .blacklist_type("tagMONITORINFOEXA")
        .blacklist_type("tagMONITORINFOEXW")
        .blacklist_function("AddVectoredContinueHandler")
        .blacklist_function("AddVectoredExceptionHandler")
        .blacklist_function("CopyContext")
        .blacklist_function("GetThreadContext")
        .blacklist_function("GetXStateFeaturesMask")
        .blacklist_function("InitializeContext")
        .blacklist_function("InitializeContext2")
        .blacklist_function("InitializeSListHead")
        .blacklist_function("InterlockedFlushSList")
        .blacklist_function("InterlockedPopEntrySList")
        .blacklist_function("InterlockedPushEntrySList")
        .blacklist_function("InterlockedPushListSListEx")
        .blacklist_function("LocateXStateFeature")
        .blacklist_function("QueryDepthSList")
        .blacklist_function("RaiseFailFastException")
        .blacklist_function("RtlCaptureContext")
        .blacklist_function("RtlCaptureContext2")
        .blacklist_function("RtlFirstEntrySList")
        .blacklist_function("RtlInitializeSListHead")
        .blacklist_function("RtlInterlockedFlushSList")
        .blacklist_function("RtlInterlockedPopEntrySList")
        .blacklist_function("RtlInterlockedPushEntrySList")
        .blacklist_function("RtlInterlockedPushListSListEx")
        .blacklist_function("RtlQueryDepthSList")
        .blacklist_function("RtlRestoreContext")
        .blacklist_function("RtlUnwindEx")
        .blacklist_function("RtlVirtualUnwind")
        .blacklist_function("SetThreadContext")
        .blacklist_function("SetUnhandledExceptionFilter")
        .blacklist_function("SetXStateFeaturesMask")
        .blacklist_function("UnhandledExceptionFilter")
        .blacklist_function("__C_specific_handler")
}
