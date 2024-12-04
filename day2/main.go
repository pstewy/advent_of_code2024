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
		if checkReport(report) {
			total++
		}
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
