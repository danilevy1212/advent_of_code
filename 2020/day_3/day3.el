;;; day3.el --- Solution to the advent of code 2020 day 3. -*- lexical-binding: t; -*-
;;
;; Copyright (C) 2020 Daniel Levy Moreno
;;
;; Author: Daniel Levy Moreno <http://github/danilevy1212>
;; Maintainer: Daniel Levy Moreno <daniellevymoreno@gmail.com>
;; Created: December 05, 2020
;; Modified: December 05, 2020
;; Version: 0.0.1
;; Keywords: Advent
;; Homepage: https://github.com/danilevy1212/advent_of_code
;; Package-Requires: ((dash "2.17.0") (s "1.12.0") (f "0.20.0"))
;;
;; This file is not part of GNU Emacs.
;;
;; This program is free software; you can redistribute it and/or modify
;; it under the terms of the GNU General Public License as published by
;; the Free Software Foundation, either version 3 of the License, or
;; (at your option) any later version.

;; This program is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;; GNU General Public License for more details.

;; You should have received a copy of the GNU General Public License
;; along with this program.  If not, see <http://www.gnu.org/licenses/>.
;;
;;; Commentary:
;; With the toboggan login problems resolved, you set off toward the airport.
;; While travel by toboggan might be easy, it's certainly not safe: there's very
;; minimal steering and the area is covered in trees. You'll need to see which
;; angles will take you near the fewest trees.;
;;
;;; Code:

(require 'dash)
(require 's)
(require 'f)
(require 'subr-x)
(require 'cl-lib)

(defvar day3-input-file-name "input.txt")
(defvar day3-script-dir load-file-name)
(defvar day3-valid-chars '("#" "."))


(defun day3-main ()
  "Entry to the Day 3 solution to the Advent Of Code 2020."
  (let* ((input-file-path (read-file-name "Input File?"
                                          (or day3-script-dir default-directory)
                                          day3-input-file-name))
         (input-text      (f-read-text input-file-path))
         (input-lines     (--remove (string-empty-p it) (s-lines input-text))))
    (when (day3--validate-lines input-lines)
      (let* ((slope-1 (--reduce-from (if (s-equals-p "#"
                                                     (nth (% (car it) (length (cdr it)))
                                                          (day3--line-to-list
                                                           (cdr it))))
                                         (+ 1 acc)
                                       acc)
                                     0
                                     (-zip-pair (--iterate
                                                 (+ 3 it) 0 (length input-lines))
                                                input-lines)))
             (slope-2 (--reduce-from (if (s-equals-p "#"
                                                     (nth (% (car it) (length (cdr it)))
                                                          (day3--line-to-list
                                                           (cdr it))))
                                         (+ 1 acc)
                                       acc)
                                     0
                                     (-zip-pair (-iterate
                                                 #'1+ 0 (length input-lines))
                                                input-lines)))
             (slope-3 (--reduce-from (if (s-equals-p "#"
                                                     (nth (% (car it) (length (cdr it)))
                                                          (day3--line-to-list
                                                           (cdr it))))
                                         (+ 1 acc)
                                       acc)
                                     0
                                     (-zip-pair (--iterate
                                                 (+ 5 it) 0 (length input-lines))
                                                input-lines)))
             (slope-4 (--reduce-from (if (s-equals-p "#"
                                                     (nth (% (car it) (length (cdr it)))
                                                          (day3--line-to-list
                                                           (cdr it))))
                                         (+ 1 acc)
                                       acc)
                                     0
                                     (-zip-pair (--iterate
                                                 (+ 7 it) 0 (length input-lines))
                                                input-lines)))
             (slope-5 (--reduce-from (if (and (cl-evenp (car it))
                                              (s-equals-p "#"
                                                          (nth (% (floor (/ (car it) 2))
                                                                  (length (cdr it)))
                                                               (day3--line-to-list
                                                                (cdr it)))))
                                         (+ 1 acc)
                                       acc)
                                     0
                                     (-zip-pair (-iterate #'1+
                                                          0 (length input-lines))
                                                input-lines))))
        (* slope-1 slope-2 slope-3 slope-4 slope-5)))))

(defun day3--validate-lines (lines)
  "Validates LINES from the input, returning t if the line is valid.

`LINES' must all have the same length and be composed of only # or .."
  (let* ((line-def-length (length (car lines)))
         (def-acc         `(:length ,line-def-length :valid-chars t))
         (validation      (-reduce-from  #'day3--validate-line def-acc lines))
         (valid-chars     (plist-get validation ':valid-chars))
         (valid-length    (> (plist-get validation ':length) 0)))
    (and valid-chars valid-length)))

(defun day3--validate-line (acc line)
  "Validate the line, ACC is property list and LINE the line.

Meant to be used as an argument function to `-reduce-from'."
  (let* ((prev-line-length (plist-get acc ':length))
         (prev-valid       (plist-get acc ':valid-chars))
         (curr-valid       (= 0 (length (s-replace-all
                                         '(("#" . "") ("." . "")) line)))))
    `(:length ,(if (and (= prev-line-length (length line) prev-line-length))
                   prev-line-length
                 -1)
      :valid-chars ,(and prev-valid curr-valid))))

(defun day3--line-to-list (line)
  "Transform a LINE to a list of chars."
  (--remove (s-equals-p it "") (split-string line "")))

(provide 'day3)
;;; day3.el ends here
