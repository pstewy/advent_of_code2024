package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
)

var (
	reg1 = regexp.MustCompile(`mul\((\d{1,3}),\s*(\d{1,3})\)`)
	reg2 = regexp.MustCompile(`(?:(mul\((\d{1,3}),\s*(\d{1,3})\))|(do\(\)|don't\(\)))`)
)

func main() {
	f, err := os.ReadFile("input_day_3.txt")
	if err != nil {
		panic(err)
	}
	fmt.Println(part1(string(f)))
	fmt.Println(part2(string(f)))
}

func part1(s string) int {
	parts := reg1.FindAllStringSubmatch(s, -1)
	if len(parts) == 0 {
		return 0
	}
	sum := 0
	for _, match := range parts {
		x, _ := strconv.Atoi(match[1])
		y, _ := strconv.Atoi(match[2])
		sum += x * y
	}
	return sum
}

func part2(s string) int {
	parts := reg2.FindAllStringSubmatch(s, -1)
	if len(parts) == 0 {
		return 0
	}
	do := true
	sum := 0
	for _, match := range parts {
		switch {
		case match[4] == "do()":
			do = true
		case match[4] == "don't()":
			do = false
		case do:
			x, _ := strconv.Atoi(match[2])
			y, _ := strconv.Atoi(match[3])
			sum += x * y
		}
	}
	return sum

}
