(ns hello-world
  (:require [coffi.mem :as mem :refer [defalias]]
            [coffi.ffi :as ffi :refer [defcfn]]
            [nrepl.server :refer [start-server stop-server]])
)

(ffi/load-library "rust/target/debug/libpyxel_c_wrapper.dylib")

(defcfn cls_c
  "Given a string, measures its length in bytes."
  cls_c [::mem/byte] ::mem/void)
#_(cls_c 0)

(defcfn circ_c
  ""
  circ_c [::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn circb_c
  ""
  circb_c [::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn rect_c
  ""
  rect_c [::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)


(defcfn pyxel_init
  "Given a string, measures its length in bytes."
  pyxel_init [::mem/int ::mem/int] ::mem/int)

#_(pyxel_init)

(defcfn pyxel_show
  "Given a string, measures its length in bytes."
  pyxel_show [] ::mem/void)

(defcfn pyxel_run
  "Given a string, measures its length in bytes."
  pyxel_run [[::ffi/fn [] ::mem/void]
             [::ffi/fn [] ::mem/void]] ::mem/void)

#_(pyxel_show)

(defn -main [& args]
  (println "hello world")
;;   (pyxel_init 160 120)
;;   (try
;;     (println "ran init")
;;     (catch Exception e
;;       (println e)))
;;   (cls_c 0)
;;   (println "ran cls")
;;   (pyxel_show)

  (println "Starting application...")

    ;; Start the nREPL server on the main thread

  (pyxel_init 128 128)


  (def radius (atom 0))
  (pyxel_run
   (fn []
     (swap! radius (fn [r] (mod (inc r) 20))))
   (fn []
     (try
       (cls_c 12)
       (circb_c 60 60 @radius 8)
     (catch Exception e
       (println e)))))
      ;; Keep the application running to maintain the nREPL server

  (println "show"))