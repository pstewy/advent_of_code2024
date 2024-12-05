package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	reports, err := os.ReadFile("input_day_2.txt")
	if err != nil {
		panic(err)
	}
	parsed := into(reports)
	total := 0
	for _, report := range parsed {
		safe := checkReportRemove(report)
		safeInPlace := checkReportWithRemoval(report, false)
		if safe != safeInPlace {
			fmt.Printf("Difference: %v\n", report)
		}
		// if safe {
		// 	total++
		// } else {
		// 	fmt.Printf("Report: %v; Safe: %v\n", report, safe)
		// }

	}
	fmt.Println(total)
}

func into(dat []byte) [][]int {
	out := [][]int{}
	for _, line := range strings.Split(string(dat), "\n") {
		row := []int{}
		for _, value := range strings.Split(line, " ") {
			v, _ := strconv.Atoi(value)
			row = append(row, v)
		}
		out = append(out, row)
	}
	return out
}

// part 1
func checkReport(level []int) bool {
	increasing := false
	decreasing := false

	previousLevel := level[0]
	for _, nextLevel := range level[1:] {
		if math.Abs(float64(previousLevel)-float64(nextLevel)) > 3 {
			return false
		}
		switch {
		case previousLevel > nextLevel && !increasing:
			decreasing = true
		case nextLevel > previousLevel && !decreasing:
			increasing = true
		default:
			return false
		}
		previousLevel = nextLevel
	}
	return increasing || decreasing
}

// part 2 trying in place - doesn't work and its hackery anyway so I'm tossing it
func checkReportWithRemoval(level []int, haveRemoved bool) bool {
	increasing := false
	decreasing := false

	previousLevel := level[0]
	for idx, nextLevel := range level[1:] {
		isFirst := idx == 0
		if math.Abs(float64(previousLevel)-float64(nextLevel)) > 3 {
			// If this is the first one, then we have to check removing the first one, and the "next" one
			if !haveRemoved && isFirst && checkReportWithRemoval(level[1:], true) {
				// It is safe without the first
				return true
			}
			if !haveRemoved {
				haveRemoved = true
				continue
			}
			return false
		}
		switch {
		case previousLevel > nextLevel && !increasing:
			decreasing = true
		case nextLevel > previousLevel && !decreasing:
			increasing = true
		default:
			if !haveRemoved {
				haveRemoved = true
				continue
			}
			// If this is the first one, then we have to check removing the first one, and the "next" one
			if !haveRemoved && isFirst && checkReportWithRemoval(level[1:], true) {
				// It is safe without the first
				return true
			}

			return false
		}
		previousLevel = nextLevel
	}
	return increasing || decreasing
}

// part 2 n2
func checkReportRemove(level []int) bool {
	// Check the report as a whole. If it fails, then try removing a value
	if checkReportWithRemovalSlow(level, false) {
		return true
	}
	for idx := range level {
		l := make([]int, len(level))
		copy(l, level)
		switch idx {
		case 0:
			l = l[1:]
		case len(l) - 1:
			l = l[:len(l)-1]
		default:
			first := l[:idx]
			second := l[idx+1:]
			l = append(first, second...)
		}
		if checkReportWithRemovalSlow(l, true) {
			return true
		}
	}
	return false
}

// part 2 n2 helper
func checkReportWithRemovalSlow(level []int, haveRemoved bool) bool {
	increasing := false
	decreasing := false

	previousLevel := level[0]
	for _, nextLevel := range level[1:] {
		if math.Abs(float64(previousLevel)-float64(nextLevel)) > 3 {
			if !haveRemoved {
				haveRemoved = true
				continue
			}
			return false
		}
		switch {
		case previousLevel > nextLevel && !increasing:
			decreasing = true
		case nextLevel > previousLevel && !decreasing:
			increasing = true
		default:
			if !haveRemoved {
				haveRemoved = true
				continue
			}
			return false
		}
		previousLevel = nextLevel
	}
	return increasing || decreasing
}
