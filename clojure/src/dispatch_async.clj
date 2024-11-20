(ns dispatch-async
  (:require [coffi.mem :as mem]
            [coffi.ffi :as ffi]))

(ffi/load-library "/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation")

(def get-main-queue
  (memoize (fn get-main-queue [] (ffi/find-symbol "_dispatch_main_q"))))

(def dispatch_async_f
  (memoize (fn dispatch_async_f [] (ffi/make-downcall "dispatch_async_f"
  [::mem/pointer ::mem/pointer [::ffi/fn [::mem/pointer] ::mem/void]] ::mem/void))))

(defn dispatch-async [callback]
  (let [callback-wrapper (fn callback-wrapper [_]
                           (try
                           (callback)
                            (catch Exception e
                              (println "Error in dispatch-async callback:" e))))
        upcall-ptr (mem/serialize callback-wrapper
                                  [::ffi/fn [::mem/pointer] ::mem/void]
                                  (mem/global-arena))]
    ((dispatch_async_f) (get-main-queue) upcall-ptr upcall-ptr)))

#_(dispatch-async (fn [] (println "Hello, world!")))