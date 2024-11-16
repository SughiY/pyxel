(ns hello-world
  (:require [pyxel-common :refer :all]
            [async-utils :refer [ dispatch-async]]
            ))

(def radius (atom 0))
(def coordinates (atom [0 0]))

#_(pyxel_mouse 1)
#_(pyxel_perf_monitor 1)
#_(pyxel_load "examples/assets/sample.pyxres" 0 0 0 0 0 0 0)

(defn clamp [min max x]
  (if (< x min) min (if (> x max) max x)))

(def limit (partial clamp 0 127))

(print @coordinates)




(def protag {:u 0
             :v 0
             :w 8
             :h 8})

(def agent {:u 8
            :v 0
            :w 8
            :h 8})

(defrecord vec2 [x y])

(defn w-with-direction [{{:keys [x]} :motion {:keys [w]} :sprite}]
  (cond
    (< x 0) (- w)
    (> x 0) w
    :else w))

(defn draw-sprite [{:keys [sprite coord] :as agent}]
  (pyxel_blt (:x coord) (:y coord) 0 (:u sprite) (:v sprite) (w-with-direction agent) (:h sprite) 0 0 1))


(defn change-move-intention [agent]
  (assoc agent :motion (vec2. (- 1 (rand-int 3)) (- 1 (rand-int 3)))))

(defn create-agent [x y]
  {:coord (vec2. x y)
   :motion (vec2. 1 1)
   :sprite agent
   :message-queue []
   :preUpdate
   (fn [agent]
     (if (< (rand-int 400) 20)
       (change-move-intention agent)
       agent))})

(defn get-direction [coord1 coord2]
  (let [x (- (:x coord2) (:x coord1))
        y (- (:y coord2) (:y coord1))]
    (vec2. (if (zero? x) 0 (if (neg? x) -1 1))
           (if (zero? y) 0 (if (neg? y) -1 1)))))

(defn apply-message [agent message]
  (case (:type message)
    :come (let [direction (get-direction (:coord agent) (:coord message))]
            (assoc agent :motion direction))
    :leave (let [direction (get-direction (:coord message) (:coord agent))]
             (assoc agent :motion direction))
    :else agent))

(defn read-message [agent]
  (let [[message r] (:message-queue agent)]
    (when message (print message))
    (if message
      (-> agent
          (assoc :message-queue r)
          (apply-message message))
      agent)))

(def agents (atom (repeatedly 100 #(create-agent (rand-int 160) (rand-int 120)))))

(def player (atom {:coord (vec2. 62 62)
                   :sprite protag
                   :motion (vec2. 0 0)}))

(def KeyRight 1073741903)
(def KeyLeft  1073741904)
(def KeyDown  1073741905)
(def KeyUp  1073741906)

(defn motion-system [{:keys [coord motion] :as agent}]
  (let [ new-x (limit (+ (:x coord) (* 0.5 (:x motion))))
        new-y (limit (+ (:y coord) (* 0.5 (:y motion))))]
    (assoc agent :coord (vec2. new-x new-y))))

(defn player-move-system [coord]
  (cond
    (pyxel_btn KeyLeft) (update coord :x dec)
    (pyxel_btn KeyRight) (update coord :x inc)
    (pyxel_btn KeyUp) (update coord :y dec)
    (pyxel_btn KeyDown) (update coord :y inc)
    :else coord))
(defn dev-update []
  (reset! player (-> @player
                     (update :coord player-move-system)))
  (->> @agents
       (map (fn [ag] ((get ag :preUpdate) ag)))
       (map read-message)
       (map motion-system)
       (reset! agents))
  )



(defn dev-draw []
(pyxel_clip 30 30 60 60)
(pyxel_cls 7)
  (pyxel_cls 7)
  (pyxel_tri 10 10 20 20 30 10 8)
(pyxel_clip 0 0 128 128)
(pyxel_fill 0 1 1)
  (pyxel_text 10 10 "Hello, Pyxel!" 13)
  (draw-sprite @player)
  (doseq [agent @agents]
    (draw-sprite agent))
  )

(defn pyxel-update []
  (try
    (dev-update)
    (catch Exception e
      (println e))))

(defn pyxel-draw []
  (try
    (dev-draw)
    (catch Exception e
      (println e))))

(defn -main [& args]
  (dispatch-async (fn []
                    (pyxel_init 128 128 "Hello world" 0 0 0 0 0)
                    (pyxel_run
                     pyxel-update
                     pyxel-draw)))

  (println "show"))