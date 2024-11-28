(ns dispatch-async
  (:require [coffi.mem :as mem]
            [coffi.ffi :as ffi :refer [defcfn]]))

(ffi/load-library "/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation")

(def ^:no-doc main-class-loader @clojure.lang.Compiler/LOADER)

(defcfn dispatch_async_f
  "dispatch_async_f"
  [::mem/pointer ::mem/pointer ::mem/pointer] ::mem/void
  native-fn
  [cb]
  (let [callback-wrapper (fn callback-wrapper [_]
                           (try
                            (.setContextClassLoader (Thread/currentThread) main-class-loader)
                           (println "Dispatching async callback")
                           (cb)
                            (catch Exception e
                              (println "Error in dispatch-async callback:" e))))
        upcall-ptr (mem/serialize callback-wrapper
                                  [::ffi/fn [::mem/pointer] ::mem/void]
                                  (mem/global-arena))
        ]
    (native-fn (ffi/find-symbol "_dispatch_main_q") upcall-ptr upcall-ptr)))

#_(dispatch_async_f (fn [] (println "Hello, world!")))