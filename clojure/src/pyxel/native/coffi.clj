(ns pyxel.native.coffi
  (:require [coffi.ffi :as ffi :refer [defcfn]]
            [coffi.mem :as mem]))
(ffi/load-system-library "pyxel_c_wrapper")

;; pyxel common functions
(defcfn pyxel_btn
  "Given a string, measures its length in bytes."
  pyxel_btn [::mem/int] ::mem/byte)

(defcfn pyxel_camera0
  "Given a string, measures its length in bytes."
  pyxel_camera0 [] ::mem/void)

(defcfn pyxel_camera
  "Given a string, measures its length in bytes."
  pyxel_camera [::mem/double ::mem/double] ::mem/void)

(defcfn pyxel_dither
  "Given a string, measures its length in bytes."
  pyxel_dither [::mem/float] ::mem/void)

(defcfn pyxel_perf_monitor
  "Given a string, measures its length in bytes."
  pyxel_perf_monitor [::mem/byte] ::mem/void)

(defcfn pyxel_pget
  "Given a string, measures its length in bytes."
  pyxel_pget [::mem/double ::mem/double] ::mem/byte)

(defcfn pyxel_pset
  "Given a string, measures its length in bytes."
  pyxel_pset [::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_pal
  "Given a string, measures its length in bytes."
  pyxel_pal [::mem/byte ::mem/byte] ::mem/void)

(defcfn pyxel_pal0
  "Given a string, measures its length in bytes."
  pyxel_pal0 [] ::mem/void)

(defcfn pyxel_clip
  "Given a string, measures its length in bytes."
  pyxel_clip [::mem/double ::mem/double ::mem/double ::mem/double] ::mem/void)

(defcfn pyxel_clip0
  "Given a string, measures its length in bytes."
  pyxel_clip0 [] ::mem/void)

(defcfn pyxel_cls
  "Given a string, measures its length in bytes."
  pyxel_cls [::mem/byte] ::mem/void)
#_(cls_c 0)

(defcfn pyxel_line
  "Given a string, measures its length in bytes."
  pyxel_line [::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_circ
  ""
  pyxel_circ [::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_circb
  ""
  pyxel_circb [::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_rect
  ""
  pyxel_rect [::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_rectb
  ""
  pyxel_rectb [::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_elli
  ""
  pyxel_elli [::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)


(defcfn pyxel_ellib
  ""
  pyxel_ellib [::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_tri
  ""
  pyxel_tri [::mem/double ::mem/double
             ::mem/double ::mem/double
             ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_trib
  ""
  pyxel_trib [::mem/double ::mem/double
              ::mem/double ::mem/double
              ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_fill
  ""
  pyxel_fill [::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_text
  "Given a string, measures its length in bytes."
  pyxel_text [::mem/double
              ::mem/double
              ::mem/c-string
              ::mem/byte] ::mem/void)

(defcfn pyxel_load
  "Given a string, measures its length in bytes."
  pyxel_load [::mem/c-string
              ::mem/byte
              ::mem/byte
              ::mem/byte
              ::mem/byte
              ::mem/byte
              ::mem/byte
              ::mem/byte] ::mem/void)

(defcfn pyxel_init
  "Given a string, measures its length in bytes."
  pyxel_init [::mem/int
              ::mem/int
              ::mem/c-string
              ::mem/int
              ::mem/int
              ::mem/int
              ::mem/int
              ::mem/int] ::mem/int)

(defcfn pyxel_show
  "Given a string, measures its length in bytes."
  pyxel_show [] ::mem/void)

(defcfn pyxel_run
  "Given a string, measures its length in bytes."
  pyxel_run [[::ffi/fn [] ::mem/void]
             [::ffi/fn [] ::mem/void]] ::mem/void)

(defcfn pyxel_blt
  "Given a string, measures its length in bytes."
  pyxel_blt
  [::mem/double ::mem/double
   ::mem/int
   ::mem/double ::mem/double
   ::mem/double ::mem/double
   ::mem/int ::mem/double ::mem/double]
  ::mem/void)

(defcfn pyxel_blt_screen
  "Given a string, measures its length in bytes."
  pyxel_blt_screen
  [::mem/double ::mem/double
   ::mem/pointer
   ::mem/double ::mem/double
   ::mem/double ::mem/double
   ::mem/int ::mem/double ::mem/double]
  ::mem/void)

(defcfn pyxel_mouse
  "Given a string, measures its length in bytes."
  pyxel_mouse [::mem/byte] ::mem/void)

(defcfn pyxel_mouse_x
  "Given a string, measures its length in bytes."
  pyxel_mouse_x [] ::mem/double)

(defcfn pyxel_mouse_y
  "Given a string, measures its length in bytes."
  pyxel_mouse_y [] ::mem/double)


;; pyxel image functions
(defcfn image_new
  "Create a new image with given dimensions"
  image_new [::mem/int ::mem/int] ::mem/pointer)

(defcfn image_from
  "Create an image from file"
  image_from [::mem/c-string ::mem/byte] ::mem/pointer)

(defcfn image_save
  "Save image to file"
  image_save [::mem/pointer ::mem/c-string ::mem/int] ::mem/int)

(defcfn image_free
  "Free image memory"
  image_free [::mem/pointer] ::mem/void)

(defcfn image_width
  "Get image width"
  image_width [::mem/pointer] ::mem/int)

(defcfn image_height
  "Get image height"
  image_height [::mem/pointer] ::mem/int)

(defcfn image_data_ptr
  "Get raw image data pointer"
  image_data_ptr [::mem/pointer] ::mem/pointer)

(defcfn image_data_length
  "Get image data length"
  image_data_length [::mem/pointer] ::mem/int)

(defcfn image_clip
  "Set clip area"
  image_clip [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/double] ::mem/void)

(defcfn image_clip0
  "Reset clip area"
  image_clip0 [::mem/pointer] ::mem/void)

(defcfn image_camera
  "Set camera position"
  image_camera [::mem/pointer ::mem/double ::mem/double] ::mem/void)

(defcfn image_camera0
  "Reset camera position"
  image_camera0 [::mem/pointer] ::mem/void)

(defcfn image_dither
  "Set dithering"
  image_dither [::mem/pointer ::mem/float] ::mem/void)

(defcfn image_pal
  "Set color palette"
  image_pal [::mem/pointer ::mem/byte ::mem/byte] ::mem/void)

(defcfn image_pal0
  "Reset color palette"
  image_pal0 [::mem/pointer] ::mem/void)

(defcfn image_cls
  "Clear screen"
  image_cls [::mem/pointer ::mem/byte] ::mem/void)

(defcfn image_pget
  "Get pixel color"
  image_pget [::mem/pointer ::mem/double ::mem/double] ::mem/byte)

(defcfn image_pset
  "Set pixel color"
  image_pset [::mem/pointer ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_line
  "Draw line"
  image_line [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_rect
  "Draw rectangle"
  image_rect [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_rectb
  "Draw rectangle border"
  image_rectb [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_circ
  "Draw circle"
  image_circ [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_circb
  "Draw circle border"
  image_circb [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_elli
  "Draw ellipse"
  image_elli [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_ellib
  "Draw ellipse border"
  image_ellib [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_tri
  "Draw triangle"
  image_tri [::mem/pointer
             ::mem/double ::mem/double
             ::mem/double ::mem/double
             ::mem/double ::mem/double
             ::mem/byte] ::mem/void)

(defcfn image_trib
  "Draw triangle border"
  image_trib [::mem/pointer
              ::mem/double ::mem/double
              ::mem/double ::mem/double
              ::mem/double ::mem/double
              ::mem/byte] ::mem/void)

(defcfn image_fill
  "Fill area"
  image_fill [::mem/pointer ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_text
  "Draw text"
  image_text [::mem/pointer ::mem/double ::mem/double ::mem/c-string ::mem/byte] ::mem/void)

(defcfn image_blt
  "Blit image"
  image_blt [::mem/pointer
             ::mem/double ::mem/double
             ::mem/pointer
             ::mem/double ::mem/double
             ::mem/double ::mem/double
             ::mem/int ::mem/double ::mem/double] ::mem/void)

(defcfn pyxel_screen
  "Get screen image"
  pyxel_screen [] ::mem/pointer)

(comment
  (require '[dispatch-async :refer [dispatch_async_f]])
  (def image_1 (image_new 256 256))
  (def image_2 (image_from "/Users/shiyigu/Documents/pyxel_games/pyxel/python/pyxel/examples/assets/noguchi_128x128.png" 0))
  (def image_2_data_ptr (image_data_ptr image_2))
  (def image_2_length (image_data_length image_2))
  (def image_2_raw (.reinterpret image_2_data_ptr image_2_length))
  (.getAtIndex image_2_raw java.lang.foreign.ValueLayout/JAVA_BYTE 2)

  (defn dev-update [])
  (defn dev-draw []
    (pyxel_cls 1)
    (pyxel_circ 260 72 30 8)
    )
  (image_rect image_2 0 0 3 1 5)
  (image_width image_2)
  (image_height image_2)

  (defn pyxel-update [] (dev-update) )
  (defn pyxel-draw [] (dev-draw) )

  #_(px/pyxel_load "/Users/shiyigu/Documents/pyxel_games/pyxel/python/pyxel/examples/assets/sample.pyxres" 0 0 0 0 0 0 0)
  ;; #_(pyxel_load "examples/assets/sample.pyxres" 0 0 0 0 0 0 0)

  (dispatch_async_f #(pyxel_init 160 120 "Hello, Pyxel" 0 0 0 0 0))
  (defn run-pxyel []
    (println "Running Pyxel")
    (pyxel_run pyxel-update pyxel-draw))

  (dispatch_async_f run-pxyel)
  )