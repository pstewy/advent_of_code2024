package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	f, err := os.ReadFile("input_day_4.txt")
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(f), "\n")
	// fmt.Println(part1(lines))
	fmt.Println(part2(lines))
}

/*
  Find an A
  When you find an A search diagonal around it
*/

func part2(lines []string) int {
	xmas := 0
	for rowNum, line := range lines {
		for colNum, char := range line {
			if string(char) == "A" {
				if masTopLeftToRight(rowNum, colNum, lines) && masTopRightToLeft(rowNum, colNum, lines) {
					xmas++
				}
			}
		}
	}
	return xmas
}

func masTopLeftToRight(rowNum, colNum int, lines []string) bool {
	upRow := rowNum - 1
	if upRow < 0 {
		return false
	}
	upCol := colNum - 1
	if upCol < 0 {
		return false
	}
	downRow := rowNum + 1
	downCol := colNum + 1
	if downRow > len(lines)-1 {
		return false
	}
	if downCol > len(lines[downRow])-1 {
		return false
	}
	/*
		       Check for
			   M
			     A
				   S
			   AND
			   S
			     A
				   M
	*/
	topLeft := string(lines[upRow][upCol])
	bottomRight := string(lines[downRow][downCol])

	if topLeft == "M" && bottomRight == "S" {
		return true
	}
	if topLeft == "S" && bottomRight == "M" {
		return true
	}
	return false
}

func masTopRightToLeft(rowNum, colNum int, lines []string) bool {
	upRow := rowNum - 1
	if upRow < 0 {
		return false
	}
	upCol := colNum - 1
	if upCol < 0 {
		return false
	}
	downRow := rowNum + 1
	downCol := colNum + 1
	if downRow > len(lines)-1 {
		return false
	}
	if downCol > len(lines[downRow])-1 {
		return false
	}
	/*
		       Check for
			       M
			     A
				S
			   AND
			       S
			     A
				M
	*/
	topRight := string(lines[upRow][downCol])
	bottomLeft := string(lines[downRow][upCol])

	if topRight == "M" && bottomLeft == "S" {
		return true
	}
	if topRight == "S" && bottomLeft == "M" {
		return true
	}
	return false
}
