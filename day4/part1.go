package main

import "fmt"

/*
Search, looking at each value and stopping if it is an X
If it is an X, look in all ways it could spell xmas
1. horizontal left -> right / right -> left
2. diagonal (top left/ right, bottom left/right)
3. vertical up / down
*/

func part1(lines []string) int {
	xmas := 0
	for rowNum, line := range lines {
		for colNum, char := range line {
			if string(char) == "X" {
				xmas += searchForXMAS(rowNum, colNum, lines)
			}
		}
	}
	return xmas
}

func searchForXMAS(curRow, curCol int, lines []string) int {
	downLeft := diagonalDownLeft(curRow, curCol, lines)
	if downLeft > 0 {
		fmt.Printf("Found down left from (%d, %d)\n", curRow, curCol)
	}
	downRight := diagonalDownRight(curRow, curCol, lines)
	if downRight > 0 {
		fmt.Printf("Found down right from (%d, %d)\n", curRow, curCol)
	}
	upLeft := diagonalUpLeft(curRow, curCol, lines)
	if upLeft > 0 {
		fmt.Printf("Found up left from (%d, %d)\n", curRow, curCol)
	}
	upRight := diagonalUpRight(curRow, curCol, lines)
	if upRight > 0 {
		fmt.Printf("Found up right from (%d, %d)\n", curRow, curCol)
	}
	inLine := searchInLine(curCol, lines[curRow])
	if inLine > 0 {
		fmt.Printf("Found %d in line from (%d, %d)\n", inLine, curRow, curCol)
	}
	vertical := searchVertical(curRow, curCol, lines)
	if vertical > 0 {
		fmt.Printf("Found %d vertical from (%d, %d)\n", vertical, curRow, curCol)
	}
	return downLeft + downRight + upLeft + upRight + inLine + vertical
}

func diagonalUpLeft(curRow, curCol int, lines []string) int {
	lookingFor := "M"
	for {
		curRow--
		curCol--
		if curRow < 0 || curCol < 0 {
			return 0
		}
		if string(lines[curRow][curCol]) == lookingFor {
			switch lookingFor {
			case "M":
				lookingFor = "A"
			case "A":
				lookingFor = "S"
			case "S":
				return 1
			}
		} else {
			return 0
		}
	}
}

func diagonalUpRight(curRow, curCol int, lines []string) int {
	lookingFor := "M"
	for {
		curRow--
		curCol++
		if curRow < 0 {
			return 0
		}
		line := lines[curRow]
		if len(line)-1 < curCol {
			return 0
		}
		if string(line[curCol]) == lookingFor {
			switch lookingFor {
			case "M":
				lookingFor = "A"
			case "A":
				lookingFor = "S"
			case "S":
				return 1
			}
		} else {
			return 0
		}
	}
}

func diagonalDownLeft(curRow, curCol int, lines []string) int {
	lookingFor := "M"
	for {
		curRow++
		curCol--
		if curRow > len(lines)-1 {
			return 0
		}
		line := lines[curRow]
		if curCol < 0 {
			return 0
		}
		if string(line[curCol]) == lookingFor {
			switch lookingFor {
			case "M":
				lookingFor = "A"
			case "A":
				lookingFor = "S"
			case "S":
				return 1
			}
		} else {
			return 0
		}
	}
}

func diagonalDownRight(curRow, curCol int, lines []string) int {
	lookingFor := "M"
	for {
		curRow++
		curCol++
		if curRow > len(lines)-1 {
			return 0
		}
		line := lines[curRow]
		if curCol > len(line)-1 {
			return 0
		}
		if string(line[curCol]) == lookingFor {
			switch lookingFor {
			case "M":
				lookingFor = "A"
			case "A":
				lookingFor = "S"
			case "S":
				return 1
			}
		} else {
			return 0
		}
	}
}

func searchVertical(curRow, curCol int, lines []string) int {
	return searchUp(curRow, curCol, lines) + searchDown(curRow, curCol, lines)
}

func searchUp(curRow, curCol int, lines []string) int {
	lookingFor := "M"
	for i := curRow - 1; i >= 0; i-- {
		curVal := string(lines[i][curCol])
		if curVal == lookingFor {
			switch lookingFor {
			case "M":
				lookingFor = "A"
			case "A":
				lookingFor = "S"
			case "S":
				return 1
			}
		} else {
			return 0
		}
	}
	return 0
}

func searchDown(curRow, curCol int, lines []string) int {
	lookingFor := "M"
	for i := curRow + 1; i <= len(lines)-1; i++ {
		if string(lines[i][curCol]) == lookingFor {
			switch lookingFor {
			case "M":
				lookingFor = "A"
			case "A":
				lookingFor = "S"
			case "S":
				return 1
			}
		} else {
			return 0
		}
	}
	return 0
}

func searchInLine(curIDX int, line string) int {
	return walkBackwards(curIDX, line) + walkForwards(curIDX, line)
}

func walkBackwards(from int, line string) int {
	lookingFor := "M"
	for i := from - 1; i >= 0; i-- {
		if string(line[i]) == lookingFor {
			switch lookingFor {
			case "M":
				lookingFor = "A"
			case "A":
				lookingFor = "S"
			case "S":
				return 1
			}
		} else {
			return 0
		}
	}
	return 0
}

func walkForwards(from int, line string) int {
	lookingFor := "M"
	for i := from + 1; i <= len(line)-1; i++ {
		if string(line[i]) == lookingFor {
			switch lookingFor {
			case "M":
				lookingFor = "A"
			case "A":
				lookingFor = "S"
			case "S":
				return 1
			}
		} else {
			return 0
		}
	}
	return 0
}
