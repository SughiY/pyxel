(ns pyxel.common
  "High level API for Pyxel game engine"
  (:require [pyxel.native.coffi :as native]))

;; Define the protocol with methods accepting [this options]
(defprotocol PyxelCommon
  (camera [this options])
  (cls [this options])
  (line [this options])
  (rect [this options])
  (rectb [this options])
  (circ [this options])
  (circb [this options])
  (tri [this options])
  (trib [this options])
  (elli [this options])
  (ellib [this options])
  (fill [this options])
  (text [this options])
  (blt [this options])
  (bltm [this options])
  (pget [this options])
  (pset [this options])
  (pal [this options])
  (clip [this options])
  (data [this])
  (data-reset! [this d])
  (data-ptr [this])
  (data-length [this])
  (load [this filename])
  (width [this])
  (height [this]))

(defn frame-count
  "Get current frame count"
  []
  (native/pyxel_frame_count))

(defn perf_monitor
  "Enable/disable performance monitor"
  [enable]
  (native/pyxel_perf_monitor (if enable 1 0)))

(defn init
  "Initialize Pyxel with the given options"
  [{:keys [width height title fps quit-key capture-scale capture-sec bg-color]
    :or {title "Pyxel" fps 0 quit-key 0 capture-scale 0 capture-sec 0 bg-color 0}}]
  (native/pyxel_init width height title fps quit-key capture-scale capture-sec bg-color))

(defn run
  "Run Pyxel with update and draw functions"
  [update-fn draw-fn]
  (native/pyxel_run update-fn draw-fn))

(defn btn
  "Check if button is pressed"
  [key-id]
  (native/pyxel_btn key-id))

(defn mouse
  "Show/hide mouse cursor"
  [visible]
  (native/pyxel_mouse (if visible 1 0)))

(defn mouse-pos
  "Get current mouse position"
  []
  {:x (native/pyxel_mouse_x)
   :y (native/pyxel_mouse_y)})


(comment
  (require '[dispatch-async :refer [dispatch_async_f]])
  (require '[pyxel.image :as image])
  (defn dev-update [])
  (defn dev-draw []
    (cls screen-img {:col 1})
    (text screen-img {:x 0 :y 0 :text (str (width screen-img)) :col 7})
    )
  (defn pyxel-update [] (dev-update))
  (defn pyxel-draw [] (dev-draw))
  (dispatch_async_f #(init {:width 160 :height 120 :title "Hello, Pyxel"}))
  (dispatch_async_f #(native/pyxel_run pyxel-update pyxel-draw))
  (def screen-img (image/screen))
  (def image_2_data_ptr (data-ptr screen-img))
  (def image_2_length (data-length screen-img))
  (def image_2_raw (.reinterpret image_2_data_ptr image_2_length))
  (.getAtIndex image_2_raw java.lang.foreign.ValueLayout/JAVA_BYTE 1)
  (let [idx 1
        val 10]
  (.setAtIndex image_2_raw java.lang.foreign.ValueLayout/JAVA_BYTE 2 val)
    )
  (.isReadOnly image_2_raw)
  )