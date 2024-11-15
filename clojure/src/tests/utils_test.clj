(ns utils-test
  (:require [clojure.test :refer [deftest is testing]]
            [clojure.string :as str]
            [utils :as u]
            [clojure.set :as set]))

(deftest test-error
  (testing "error-make"
    (is (= (u/error-make :error-type :message :context)
           '(:error-type :message :context)))
    (let [ex (Exception. "hello")]
      (is (= (u/error-make :before ex :after)
             (list :before ex :after)))))

  (testing "error-add macro"
    (let [ex1 (Exception. "first")
          ex2 (Exception. "second")
          base-error (u/error-make :base-error ex1 :init-msg)
          updated-error (u/error-add base-error :additional-info ex2 :more-info)]
      (is (= updated-error (list :additional-info ex2 :more-info :base-error ex1 :init-msg)))))

  (testing "error->str macro"
    (let [error-without-exceptions (u/error-make :error-type :message)
          error-with-exceptions (u/error-make :error-type :message (Exception. "Exception msg 1"))
          error-with-more-exceptions (u/error-add error-with-exceptions
                                                  :more (Exception. "Exception msg 2") "after")]
      (is (= (u/error->str error-without-exceptions)
             ":error-type :message"))

      (is (set/superset?
           (set (str/split-lines (u/error->str error-with-exceptions)))
           #{":error-type :message java.lang.Exception: Exception msg 1"
             "--exception details--"
             "java.lang.Exception: Exception msg 1"}))

      (is (set/superset?
           (set (str/split-lines (u/error->str error-with-more-exceptions)))
           #{":more java.lang.Exception: Exception msg 2 after :error-type :message java.lang.Exception: Exception msg 1"
             "--exception details--"
             "java.lang.Exception: Exception msg 2"
             "java.lang.Exception: Exception msg 1"})))))

(deftest eprotect
  (testing "plain"
    (let [ret (u/with-eprotect :id-test-error
                {:abc 5})]
      (is (= ret {:abc 5})))

    (let [exc (Exception. "hi")
          ret (u/with-eprotect [:id-test-error 53]
                (+ 1 2)
                (throw exc))]
      (is (= {:error (u/error-make [:id-test-error 53] exc)} ret)))

    (let [exc (Exception. "hi5")
          ret (u/with-eprotect {:id :id-test-error5}
                (throw exc)
                (+ 5 6))]
      (is (= {:error (u/error-make :id-test-error5 exc)} ret))))

  (testing "opts error"
    (is (thrown-with-msg?
         IllegalArgumentException
         #"id-or-opts should contain an :id key."
         (try
           (macroexpand-1 '(u/with-eprotect {:abc 123}))
           (catch Exception e
             (throw (.getCause e))))))

    (is (thrown-with-msg?
         IllegalArgumentException
         #"id-or-opts should only contain supported option keys:.*"
         (try
         (macroexpand-1 '(u/with-eprotect {:id 123 :garbage 5}))
           (catch Exception e
           (throw (.getCause e)))))))

  (testing "on-err-str"
    (let [exc (Exception. "hi")
          details* (atom nil)
          ret (u/with-eprotect {:id :id-test-error :on-err-str #(reset! details* %)}
                (throw exc))]
      (is (= {:error (u/error-make :id-test-error exc)} ret))
      (is (= [":id-test-error java.lang.Exception: hi"
              ""
              "--exception details--"
              ""
              "java.lang.Exception: hi"]
             (take 5 (str/split-lines @details*)))))))