(ns aoc.day5
  (:require [aoc.utils :as utils]
            [clojure.string :as str]))

(defn parse-command-line
  [line]
  (->> line
       (re-matches #"move (\d+) from (\d+) to (\d+)")
       (drop 1)
       (map read-string)))

(defn move-crates
  [crates n from to]
  (let [from-vec (get crates (- from 1))
        to-vec (get crates (- to 1))
        to-move (subvec from-vec (- (count from-vec) n) (count from-vec))
        from-after (subvec from-vec 0 (- (count from-vec) n))
        to-after (apply conj to-vec to-move)
        crates (assoc crates (- from 1) from-after)
        crates (assoc crates (- to 1) to-after)]
    crates))

(defn run-day5
  []
  (let [crates [[\D \T \W \F \J \S \H \N]
                [\H \R \P \Q \T \N \B \G]
                [\L \Q \V]
                [\N \B \S \W \R \Q]
                [\N \D \F \T \V \M \B]
                [\M \D \B \V \H \T \R]
                [\D \B \Q \J]
                [\D \N \J \V \R \Z \H \Q]
                [\B \N \H \M \S]]
        commands (utils/process-file-by-line
                  "resources/day5.txt"
                  (fn [lines] (mapv parse-command-line lines)))]
    (->>
     (reduce (fn [crates command] (apply move-crates crates command))
             (apply conj [crates] commands))
     (map last)
     (str/join ""))))
