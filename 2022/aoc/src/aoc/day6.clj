(ns aoc.day6
  (:require [clojure.string :as str]))

(defn find-start-of-packet-marker
  [packet char-count]
  (loop [i 0]
    (let [marker (subs packet i (+ i char-count))
          unique-chars (set (char-array marker))]
      (if (= (count unique-chars) char-count)
        i
        (recur (inc i))))))


(defn run-day6
  [char-count]
  (-> "resources/day6.txt"
      (slurp)
      (str/trim-newline)
      (find-start-of-packet-marker char-count)
      (+ char-count)))
