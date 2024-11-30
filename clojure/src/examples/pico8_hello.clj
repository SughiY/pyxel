(ns examples.pico8-hello
  (:require [pyxel.common :as px]
            [pyxel.constants :as pxc]
            [pyxel.image :as pimg]))

(def logo (memoize #(pimg/from {:filename "examples/assets/hello.png"})))

(def frame-rate 30)
(def col-num 9)
(defn update-sprite []
  (for [col (range 15 (- 15 col-num) -1)
        i (range 1 12)]
    (let [sprite (* i 8)
          base-v (* (px/frame-count) (/ Math/PI frame-rate 0.5))
          t1 (+ base-v
                (* i (/ Math/PI 11 0.5))
                (* -1 col (/ Math/PI col-num 0.7)))
          x (+ 12 sprite (* (Math/sin t1) 3))
          y (+ 38 col (* (Math/cos t1) 5))]
      {:i sprite :x x :y y :col col})))

(defn pyxel-draw []
  (px/cls (pimg/screen) {:col 0})
  (doseq [{:keys [i x y col]} (update-sprite)]
    (px/pal (pimg/screen) {:col1 pxc/COLOR_WHITE :col2 col})
    (px/blt (pimg/screen) {:x x :y y :img (logo) :u i  :v 8 :w 8 :h 8 :colkey 0})
    )

  (px/text (pimg/screen) {:x 37 :y 70 :text "THIS IS PICO-8" :col 14})
  (px/text (pimg/screen) {:x 34 :y 80 :text "NICE TO MEET YOU" :col 12})
  (px/blt (pimg/screen) {:x 60 :y 90 :img (logo) :u 8 :v 0 :w 8 :h 8})
  (px/text (pimg/screen) {:x 37 :y 100 :text (str (mod (px/frame-count) 30)) :col 14})
)

(defn pyxel-update []
  (when (= 1 (px/btn pxc/KEY_Q))
    (println "Quitting")))

(comment
  (require '[dispatch-async :refer [dispatch_async_f]])
  (dispatch_async_f #(px/init {:width 128 :height 128 :title "Hello Pico8"}))
  (dispatch_async_f (fn [] (px/run #(pyxel-update) #(pyxel-draw))))
  (px/perf_monitor true)
  )
