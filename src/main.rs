use lucet_runtime::*;
use lucet_wasi::*;

fn main() {
    lucet_wasi::hostcalls::ensure_linked();
    let dl_module = DlModule::load("a.out").unwrap();
    let region = MmapRegion::create(1, &Limits::default()).unwrap();
    let mut instance = region.new_instance(dl_module).unwrap();
    let wasi_ctx = WasiCtx::new(&[]);
    instance.insert_embed_ctx(wasi_ctx);
    instance.run("entrypoint", &[]).unwrap();
}