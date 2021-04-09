cfg_if::cfg_if! {
  if #[cfg(feature = "dummy-runtime")] {
    mod dummy;
    pub use dummy::{DummyAsyncManager as AsyncManager, spawn, spawn_with_handle, block_on};
  } else if #[cfg(feature = "async-std-runtime")] {
    mod async_std;
    pub use self::async_std::{AsyncStdAsyncManager as AsyncManager, spawn, spawn_with_handle, block_on};
  } else if #[cfg(feature = "wasm-bindgen-runtime")] {
    mod wasm_bindgen;
    pub use self::wasm_bindgen::{WasmBindgenAsyncManager as AsyncManager, spawn, spawn_with_handle, block_on};
  } else if #[cfg(feature = "tokio-runtime")] {
    mod tokio;
    pub use self::tokio::{TokioAsyncManager as AsyncManager, spawn, spawn_with_handle, block_on};
  }
  else {
    std::compile_error!("Please choose a runtime feature: thread-pool-runtime, async-std-runtime, wasm-bindgen-runtime, dummy-runtime");
  }
}
