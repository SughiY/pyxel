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
  (data-ptr [this])
  (data-length [this])
  (load [this filename])
  (width [this])
  (height [this]))

;; Create a deftype for ImageType wrapping the image pointer
(deftype ImageType [img-ptr]
  PyxelCommon
  (camera [this {:keys [x y]}]
    (if x
      (native/image_camera img-ptr x y)
      (native/image_camera0 img-ptr)))
  (cls [this {:keys [col]}]
    (native/image_cls img-ptr col))
  (line [this {:keys [x1 y1 x2 y2 col]}]
    (native/image_line img-ptr x1 y1 x2 y2 col))
  (rect [this {:keys [x y w h col]}]
    (native/image_rect img-ptr x y w h col))
  (rectb [this {:keys [x y w h col]}]
    (native/image_rectb img-ptr x y w h col))
  (circ [this {:keys [x y r col]}]
    (native/image_circ img-ptr x y r col))
  (circb [this {:keys [x y r col]}]
    (native/image_circb img-ptr x y r col))
  (elli [this {:keys [x y rx ry col]}]
    (native/image_elli img-ptr x y rx ry col))
  (ellib [this {:keys [x y rx ry col]}]
    (native/image_ellib img-ptr x y rx ry col))
  (tri [this {:keys [x1 y1 x2 y2 x3 y3 col]}]
    (native/image_tri img-ptr x1 y1 x2 y2 x3 y3 col))
  (trib [this {:keys [x1 y1 x2 y2 x3 y3 col]}]
    (native/image_trib img-ptr x1 y1 x2 y2 x3 y3 col))
  (fill [this {:keys [x y col]}]
    (native/image_fill img-ptr x y col))
  (text [this {:keys [x y text col]}]
    (native/image_text img-ptr x y text col))
  (blt [this {:keys [x y img src-x src-y width height colkey u v]}]
    (native/image_blt img-ptr x y (:img-ptr img) src-x src-y width height colkey u v))
  (pget [this {:keys [x y]}]
    (native/image_pget img-ptr x y))
  (pset [this {:keys [x y col]}]
    (native/image_pset img-ptr x y col))
  (pal [this {:keys [col1 col2]}]
    (if col2
      (native/image_pal img-ptr col1 col2)
      (native/image_pal0 img-ptr)))
  (clip [this {:keys [x y w h]}]
    (if w
      (native/image_clip img-ptr x y w h)
      (native/image_clip0 img-ptr)))
  (data-ptr [this]
    (native/image_data_ptr img-ptr))
  (data-length [this]
    (native/image_data_length img-ptr))
  (load [this filename]
    (native/image_from filename 0))
  (width [this]
    (native/image_width img-ptr))
  (height [this]
    (native/image_height img-ptr))
  )


(defn screen []
  "Return the screen image"
  (ImageType. (native/pyxel_screen)))

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
  (defn dev-update [])
  (defn dev-draw []
    (cls screen-img {:col 1})
    (text screen-img {:x 0 :y 0 :text (str (width screen-img)) :col 7})
    )
  (defn pyxel-update [] (dev-update))
  (defn pyxel-draw [] (dev-draw))
  (dispatch_async_f #(init {:width 160 :height 120 :title "Hello, Pyxel"}))
  (dispatch_async_f #(native/pyxel_run pyxel-update pyxel-draw))
  (def screen-img (screen))
  (def image_2_data_ptr (data-ptr screen-img))
  (def image_2_length (data-length screen-img))
  (def image_2_raw (.reinterpret image_2_data_ptr image_2_length))
  (.getAtIndex image_2_raw java.lang.foreign.ValueLayout/JAVA_BYTE 1)
  (.setAtIndex image_2_raw java.lang.foreign.ValueLayout/JAVA_BYTE 1 10)
  (.isReadOnly image_2_raw)
  )