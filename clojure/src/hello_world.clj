(ns hello-world
  (:require [coffi.mem :as mem :refer [defalias]]
            [coffi.ffi :as ffi :refer [defcfn]]
            [nrepl.server :refer [start-server stop-server]])
)

(ffi/load-library "resources/debug/libpyxel_c_wrapper.dylib")

(defcfn cls_c
  "Given a string, measures its length in bytes."
  cls_c [::mem/int] ::mem/void)
#_(cls_c 0)

(defcfn pyxel_init
  "Given a string, measures its length in bytes."
  pyxel_init [] ::mem/long)

#_(pyxel_init)

(defcfn pyxel_show
  "Given a string, measures its length in bytes."
  pyxel_show [] ::mem/void)

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

  (pyxel_init)
  (cls_c 3)
  (pyxel_show)
      ;; Keep the application running to maintain the nREPL server

  (println "show"))