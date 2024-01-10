use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
//use std::path::Path;


fn main() -> Result<()> {
    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    // Dir(std_file="wasiex/usr/local/lib/");

    // args: Vec<Vec<u8>>,
    // env: Vec<(Vec<u8>, Vec<u8>)>,
    // stdin: WasiConfigReadPipe,
    // stdout: WasiConfigWritePipe,
    // stderr: WasiConfigWritePipe,
    // preopen_dirs: Vec<(Dir, PathBuf)>,
    // preopen_sockets: HashMap<u32, TcpListener>,
    // inherit_args: bool,
    // inherit_env: bool,

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        //.inherit_args()?
        .args(&[
            "workdir/source_text.txt".to_string(),
            "workdir/emojize_text.py".to_string(),
        ])
        .unwrap()
        .preopened_dir(
            wasmtime_wasi::Dir::open_ambient_dir(
                "wasiex/usr/local/lib",
                wasmtime_wasi::ambient_authority(),
            )
            .unwrap(),
            "usr/local/lib",
        ).unwrap()
        .preopened_dir(
            wasmtime_wasi::Dir::open_ambient_dir(
                "wasiex/workdir",
                wasmtime_wasi::ambient_authority(),
            )
            .unwrap(),
            "workdir",
        ).unwrap()
        .build();
    let mut store = Store::new(&engine, wasi);

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(&engine, "wasiex/bin/python-3.11.1.wasm")?;
    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    let wasi2 = WasiCtxBuilder::new()
        .inherit_stdio()
        //.inherit_args()?
        .args(&[
            "workdir/source_text.txt".to_string(),
            "workdir/emojize_text.py".to_string(),
        ])
        .unwrap()
        .preopened_dir(
            wasmtime_wasi::Dir::open_ambient_dir(
                "wasiex/usr/local/lib",
                wasmtime_wasi::ambient_authority(),
            )
            .unwrap(),
            "usr/local/lib",
        ).unwrap()
        .preopened_dir(
            wasmtime_wasi::Dir::open_ambient_dir(
                "wasiex/workdir",
                wasmtime_wasi::ambient_authority(),
            )
            .unwrap(),
            "workdir",
        ).unwrap()
        .build();
    // let mut store2 = Store::new(&engine, wasi2);    
    linker
    .get_default(&mut store, "")?
    .typed::<(), ()>(&store)?
    .call(&mut store, ())?;

    // let mut linker2 = Linker::new(&engine);
    // linker2.module(&mut store2, "", &module)?;
    // linker2
    //         .get_default(&mut store2, "")?
    //         .typed::<(), ()>(&store2)?
    //         .call(&mut store2, ())?;

    Ok(())
}