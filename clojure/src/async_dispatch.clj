(ns async-dispatch
    (:require [coffi.mem :as mem]
              [coffi.ffi :as ffi :refer [defcfn]]))

(ffi/load-library "/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation")

(def main-queue (ffi/find-symbol "_dispatch_main_q"))

(ffi/find-symbol "_dispatch_main_q")

(defcfn dispatch_async_f
  "
  void (*dispatch_function_t) (void *_Nullable)
  void dispatch_async_f(dispatch_queue_t queue, void *_Nullable context, dispatch_function_t work);
  "
  dispatch_async_f [::mem/pointer ::mem/pointer [::ffi/fn [::mem/pointer] ::mem/void]] ::mem/void)


(defn callback [_] (println "Hello, World!"))
(dispatch_async_f main-queue main-queue callback)