(ns async-utils
  "Utilities for interop with c."
  (:import com.sun.jna.Pointer
           com.sun.jna.Platform
           java.util.concurrent.Executors))

(def ^:no-doc main-class-loader @clojure.lang.Compiler/LOADER)
(def ^:no-doc void Void/TYPE)

(defmacro ^:no-doc defc
  ([fn-name lib ret]
   `(defc ~fn-name ~lib ~ret []))
  ([fn-name lib ret args]
   (let [cfn-sym (with-meta (gensym "cfn") {:tag 'com.sun.jna.Function})]
     `(let [~cfn-sym (delay (.getFunction ~(with-meta `(deref ~lib) {:tag 'com.sun.jna.NativeLibrary})
                                          ~(name fn-name)))]
        (defn- ~fn-name [~@args]
          (.invoke (deref ~cfn-sym)
                   ~ret (to-array [~@args])))))))

(def ^:no-doc
  objlib (delay
           (com.sun.jna.NativeLibrary/getInstance "CoreFoundation")))

(def ^:no-doc
  main-queue (delay
               (.getGlobalVariableAddress ^com.sun.jna.NativeLibrary @objlib "_dispatch_main_q")))

(defc dispatch_async_f  objlib void [queue context work])

(defonce ^:no-doc
  callbacks (atom []))

(deftype ^:no-doc DispatchCallback [f]
  com.sun.jna.CallbackProxy
  (getParameterTypes [_]
    (into-array Class  [Pointer]))
  (getReturnType [_]
    void)
  (callback ^void [_ args]
    (.setContextClassLoader (Thread/currentThread) main-class-loader)

    (import 'com.sun.jna.Native)
    ;; https://java-native-access.github.io/jna/4.2.1/com/sun/jna/Native.html#detach-boolean-
    ;; for some other info search https://java-native-access.github.io/jna/4.2.1/ for CallbackThreadInitializer

    ;; turning off detach here might give a performance benefit,
    ;; but more importantly, it prevents jna from spamming stdout
    ;; with "JNA: could not detach thread"
    (com.sun.jna.Native/detach false)
    (f)
    ;; need turn detach back on so that
    ;; we don't prevent the jvm exiting
    ;; now that we're done
    (com.sun.jna.Native/detach true)))

(defn dispatch-async
  "Run `f` on the main thread. Will return immediately."
  [f]
  (let [callback (DispatchCallback. f)]
      (dispatch_async_f @main-queue nil callback)
      ;; please don't garbage collect me :D
      (identity callback)
      nil))