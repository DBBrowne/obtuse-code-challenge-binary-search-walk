package main

import (
	"fmt"
	// "testing"
)

func counts(inputs []int, refs []int) []int {
	var output []int

	for i:= 0; i<len(refs);i++{
		counter := 0
		ref := refs[i]
		for j:= 0; j<len(inputs);j++{
			if inputs[j]<=ref{
				counter ++
			}
		}
		output = append(output, counter)
	}
	
	return output
}


// ********************************
// *** Tests
func main() {
	testCounts()
}

func Equal(a, b []int) bool {
    if len(a) != len(b) {
        return false
    }
    for i, v := range a {
        if v != b[i] {
            return false
        }
    }
    return true
}

func testCounts(){
	output := counts([]int {1, 2, 4, 4}, []int {3, 4})

	assert := Equal([]int{2,4}, output)
	if !assert {
		fmt.Println("Fail.  Output: ", output)
	}
}