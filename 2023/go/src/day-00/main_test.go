package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const testData = `1000
2000
3000

4000

5000
6000

7000
8000
9000

10000`

func TestProcessPart1(t *testing.T) {

	assert.Equal(t, processPart1(testData), "24000")
}

func TestProcessPart2(t *testing.T) {
	assert.Equal(t, processPart2(testData), "45000")
}