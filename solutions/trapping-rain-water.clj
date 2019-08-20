;;; https://leetcode.com/problems/trapping-rain-water/
;;; by Alex Che

;; with reduce
(defn trap-rain-water [h]
  (let [max-pos (first (apply max-key second (map-indexed vector h)))
        [left-h  right-h] (split-at max-pos h)
        f (fn([[m v] x] (if (> x m) [x v] [m (+ v (- m x))])))]
    (+ (second (reduce f [0 0] left-h))
       (second (reduce f [0 0] (reverse right-h))))))
 
;; with loops
(defn trap-rain-water [h]
  (let [enumerated (map-indexed vector h)
        max-pos (first (apply max-key second enumerated))]
    (+ (loop [m 0 res 0 l enumerated]
         (let [[pos x] (first l)]
           (if (= pos max-pos)
             res
             (if (> x m)
               (recur x res (rest l))
               (recur m (+ res (- m x)) (rest l))))))
       (loop [m 0 res 0 l (reverse enumerated)]
         (let [[pos x] (first l)]
           (if (= pos max-pos)
             res
             (if (> x m)
               (recur x res (rest l))
               (recur m (+ res (- m x)) (rest l)))))))))
