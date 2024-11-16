
(ns pyxel-image
  (:require
   [pyxel]
   [coffi.mem :as mem]
   [coffi.ffi :as ffi :refer [defcfn]]))

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

(comment
  (require '[pyxel :as px])
  (require '[async-utils :refer [dispatch-async]])

  (dispatch-async #((px/pyxel-init 160 120)
                    ()))
  )