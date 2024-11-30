(ns examples.bunny-mark
  (:require [pyxel.constants :as c]
            [pyxel.common :as px]
            [pyxel.image :as pxi]))

(def texture
  (memoize
   #(pxi/from {:filename "examples/assets/collide.png"})))

(defn create-bunny []
  {:u 40
   :v 8
   :w 8
   :h 8
   :x 0
   :y 0
   :vx (+ 1 (rand 10))
   :vy (+ 1 (rand 5))})

(def gravity 0.5)

(defn move [{:keys [x y vx vy] :as bunny}]
  (cond-> bunny
    true (-> (update :x + vx)
             (update :y + vy)
             (update :vy + gravity))
    (neg? x) (-> (update :vx * -1)
                 (assoc :x 8))
    (neg? y) (-> (update :vy * -0.8)
                 (assoc :y 8))
    (> x 152) (-> (update :vx * -1)
                  (assoc :x 152))
    (> y 112) (-> (update :vy * -0.8)
                  (cond->
                   (> (rand-int 10) 8) (update :vx * (+ -1 (rand 0.1))))
                  (assoc :y 112)
                ;;   (cond->
                ;;    (> (rand-int 10) 5) (update :vy - (+ 3 (rand-int 4))))
    )
    ))

(def bunnies (atom []))

(defn add-bunnies []
  (swap! bunnies concat (repeat 100 (create-bunny))))

(defn pyxel-update []
  (when (= 1 (px/btn c/KEY_SPACE))
    (add-bunnies))
  (swap! bunnies #(map move %))
  )

(defn pyxel-draw []
  (px/cls (pxi/screen) {:col 0})
  (doseq [bunny @bunnies]
    (px/blt (pxi/screen) (assoc bunny :img (texture))))
  (px/text (pxi/screen) {:x 0 :y 112 :text (str "bunnies: " (count @bunnies)) :col 2})
  )

(comment
  (require '[dispatch-async :refer [dispatch_async_f]])
  (dispatch_async_f #(px/init {:width 160 :height 120 :fps 30 :title "Pyxel bunny mark"}))
  (dispatch_async_f (fn [] (px/run #(pyxel-update) #(pyxel-draw))))
  (px/perf_monitor true)
  )