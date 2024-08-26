package main

import (
	"fmt"
	"os"
	"strings"
	"unicode"
)

func main() {
	input, err := os.ReadFile("./input/input")
	if err != nil {
		fmt.Println("No se ha podido leer la entrada.")
		return
	}
	puzzle1(input)
	puzzle2(input)
}

func puzzle1(input []byte) {
	texto := string(input)
	lineas := strings.Split(texto, "\n")

	total := 0
	for _, line := range lineas {
		var primero, ultimo int
		primeroAsignado := false
		for _, runa := range line {
			if unicode.IsDigit(runa) {
				n := int(runa - '0')
				if primeroAsignado == false {
					primero = n
					primeroAsignado = true
				}
				ultimo = n
			}
		}
		total += primero*10 + ultimo
	}
	fmt.Printf("La solución del primer problema es: %d\n", total)
}

func puzzle2(input []byte) {
	lineas := strings.Split(string(input), "\n")
	Numeros := [...]string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}

	total := 0
	for _, line := range lineas {
		var primero, ultimo int
		primeroAsignado := false
		buf := ""
		for _, runa := range line {
			if unicode.IsDigit(runa) {
				n := int(runa - '0')
				if primeroAsignado == false {
					primero = n
					primeroAsignado = true
				}
				ultimo = n
			} else {
				buf += string(runa)
				for i := 0; len(buf)-i >= 3; i++ {

					if len(buf)-i < 3 || len(buf)-i > 5 {
						continue
					}

					indice := indexOf(string(buf[i:]), Numeros[:])
					// fmt.Printf("[%d:%d],%s\n", i, len(buf), buf)
					if indice != -1 {
						buf = ""
						if primeroAsignado == false {
							primero = indice + 1
							primeroAsignado = true
						}
						ultimo = indice + 1
					}
				}
			}
		}
		total += primero*10 + ultimo
		// fmt.Printf("%d%d\n", primero, ultimo)
	}
	fmt.Printf("La solución del primer problema es: %d\n", total)
}

func indexOf[T comparable](element T, slice []T) int {
	for i, v := range slice {
		if v == element {
			return i
		}
	}
	return -1
}
