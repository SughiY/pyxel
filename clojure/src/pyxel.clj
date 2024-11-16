(ns pyxel
  (:require [coffi.ffi :refer [load-library]]
            ))
(load-library "rust/target/debug/libpyxel_c_wrapper.dylib")
