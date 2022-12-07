(ns aoc.day7
  (:require  [aoc.utils :as utils]))

(defn get-files
  [lines]
  (loop [acc []
         line-seq lines]
    (if (seq line-seq)
      (let [[match size name] (re-matches #"(\d+) (.+)" (first line-seq))]
        (if (some? match)
          (recur (cons [name (read-string size)] acc) (next line-seq))
          (recur acc (next line-seq))))
      acc)))

(defn parse-command
  [line]
  (let [[cd-matches op] (re-matches #"\$ cd (.+)" line)
        ls-matches (re-matches #"\$ ls" line)]
    (cond
      (some? cd-matches) [:cd op]
      (some? ls-matches) [:ls]
      :else [:unknown])))

(defn split-commands
  [lines]
  (when (seq lines)
    (let [command (first lines)
          [output rest] (split-with
                         (fn [line] (nil? (re-matches #"\$ .+" line)))
                         (next lines))
          command (apply conj [(parse-command command)] output)]
      (cons command (split-commands rest)))))

(defn process-cd-command
  [state op]
  (case op
    "/" (assoc state :cwd ["root"])
    ".." (-> (get state :cwd)
             (reverse)
             (next)
             (reverse)
             (vec)
             ((partial assoc state :cwd)))
    (-> (get state :cwd)
        (conj op)
        (vec)
        ((partial assoc state :cwd)))))

(defn process-ls-command
  [state output-seq]
  (let [files (get-files output-seq)
        cwd (get state :cwd)
        path (flatten (map (fn [x] [:dirs x]) cwd))]
    (assoc-in
     state
     (concat path [:files])
     files)))

(defn process-command
  [state command]
  (let [[cmd op] (first command)]
    (case cmd
      :cd (process-cd-command state op)
      :ls (process-ls-command state (next command))
      state)))

(defn process-commands
  [state commands]
  (reduce process-command (apply conj [state] commands)))

(defn calculate-files-size
  [dir]
  (->> (get dir :files)
       (map (fn [x] (get x 1)))
       (reduce +)))

(defn calculate-dir-size
  [dir]
  (let [dirs (get dir :dirs)
        dirs-with-sizes (map (fn [[name subdir]] {name (calculate-dir-size subdir)}) dirs)
        dirs-with-sizes (reduce merge dirs-with-sizes)
        dirs-sizes (map (fn [[_ dir]] (get dir :size)) dirs-with-sizes)
        dirs-sizes (reduce + dirs-sizes)
        files-size (calculate-files-size dir)]
    (assoc dir
           :size (+ files-size dirs-sizes)
           :dirs dirs-with-sizes)))

(defn find-directories
  [dir filter-fn value-fn]
  (let [dirs (get dir :dirs)
        values (map (fn [[_ dir]] (find-directories dir filter-fn value-fn)) dirs)]
    (if (filter-fn dir)
      (cons (value-fn dir) values)
      values)))

(defn find-directory-to-remove
  [lines]
  (let [state (-> lines
                  (split-commands)
                  ((partial process-commands {}))
                  (calculate-dir-size)
                  (get-in [:dirs "root"]))
        used-space (get state :size)
        unused-space (- 70000000 used-space)
        to-free (- 30000000 unused-space)
        dirs (find-directories state
                               (fn [dir] (< to-free (get dir :size)))
                               (fn [dir] (get dir :size)))]
    (flatten dirs)))

(-> (utils/process-file-by-line "resources/day7.txt", find-directory-to-remove)
    (sort))
