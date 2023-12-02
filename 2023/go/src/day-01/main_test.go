package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const testData = `1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet`

const testData2 = `two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen`

func TestProcessPart1(t *testing.T) {

	assert.Equal(t, "142", processPart1(testData))
}

func TestProcessLine(t *testing.T) {
	var expected int64 = 84
	assert.Equal(t, processLine("nqninenmvnpsz874"), expected)
}

func TestProcessPart2(t *testing.T) {
	assert.Equal(t, "281", processPart2(testData2))
}