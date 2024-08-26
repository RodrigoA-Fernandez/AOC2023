package main

import (
	"dia_2/juego"
	"fmt"
	"os"
	"strings"
)

func main() {
	input, err := os.ReadFile("./input/input")
	if err != nil {
		fmt.Println("No se ha podido leer la entrada.")
		return
	}

	problema1(input)
	problema2(input)
}

func problema1(input []byte) {
	var juegos []juego.Juego
	lineas := strings.Split(string(input), "\n")
	for _, linea := range lineas[:len(lineas)-1] {
		juegos = append(juegos, juego.ParseJuego(linea))
	}

	total := 0
	for _, game := range juegos {
		if juego.ComprobarValido(game, 12, 13, 14) {
			total += game.Id
		}
	}
	fmt.Printf("Problema 1: %d\n", total)
}

func problema2(input []byte) {
	var juegos []juego.Juego
	lineas := strings.Split(string(input), "\n")
	for _, linea := range lineas[:len(lineas)-1] {
		juegos = append(juegos, juego.ParseJuego(linea))
	}

	total := 0
	for _, game := range juegos {
		total += juego.PoderMinimo(game)
	}

	fmt.Printf("Problema 2: %d\n", total)
}
