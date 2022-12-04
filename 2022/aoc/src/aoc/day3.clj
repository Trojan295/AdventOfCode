(ns aoc.day3
  (:require [clojure.set :as set]
            [aoc.utils :as utils]))

(defn item-to-priority
  [item]
  (if (Character/isUpperCase item)
    (- (int item) 38)
    (- (int item) 96)))

(defn process-group
  [group]
  (->> group
       (map char-array)
       (map set)
       (apply set/intersection)
       (first)
       (item-to-priority)))

(defn process-lines
  [lines]
  (->> lines
       (partition 3)
       (map process-group)
       (reduce +)))


(defn run-day3
  []
  (let [result (-> "resources/day3.txt"
                   (utils/process-file-by-line process-lines))]
    result))
