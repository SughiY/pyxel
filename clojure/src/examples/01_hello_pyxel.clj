(ns examples.01-hello-pyxel
  (:require
   [pyxel.common :as px]
   [pyxel.constants :as pxc]
   [pyxel.image :as pimg]))


(def logo (memoize #(pimg/from {:filename "examples/assets/pyxel_logo_38x16.png"})))
(defn pyxel-draw []
   (px/cls (pimg/screen) {:col 0})
   (px/text (pimg/screen) {:x 55 :y 41 :text "Hello, Pyxel!" :col (mod (px/frame-count) 16)})
   (px/blt (pimg/screen) {:x 61 :y 66 :img (logo) :u 0 :v 0 :w 38 :h 16})
  )
(defn pyxel-update []
  (when (= 1 (px/btn pxc/KEY_Q))
    (println "Quitting")))

(comment
  (require '[dispatch-async :refer [dispatch_async_f]])
  (dispatch_async_f #(px/init {:width 160 :height 120 :title "Hello Pyxel"}))
  (dispatch_async_f (fn [] (px/run #(pyxel-update) #(pyxel-draw))))
  )
