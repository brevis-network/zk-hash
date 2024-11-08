package utils

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestSliceFlip(t *testing.T) {
	original := []int{1, 2, 3, 4, 5, 6, 7}
	flipped := Flip(original)
	fmt.Println(flipped)
	require.EqualValues(t, flipped, []int{7, 6, 5, 4, 3, 2, 1})
}
