package juego

import (
	"fmt"
	"strings"
)

type Turno struct {
	Rojo  int
	Azul  int
	Verde int
}

func parseTurno(cad string) Turno {
	var rojo, azul, verde int
	colores := strings.Split(cad, ",")

	for _, cad := range colores {
		var color string
		var n int
		if len(cad) == 0 {
			continue
		}
		fmt.Sscanf(cad[:], "%d %s", &n, &color)
		if len(color) == 0 {
			continue
		}
		// fmt.Println(cad)
		// fmt.Println(color)
		switch color[:] {
		case "red":
			rojo = n
		case "green":
			verde = n
		case "blue":
			azul = n
		}
	}

	return Turno{
		Rojo:  rojo,
		Verde: verde,
		Azul:  azul,
	}
}

type Juego struct {
	Turnos []Turno
	Id     int
}

func ParseJuego(cad string) Juego {
	var id int
	split := strings.Split(cad, ":")
	fmt.Sscanf(split[0], "Game %d:", &id)

	stringsTurnos := strings.Split(split[1], ";")

	var turnos []Turno
	for _, turno := range stringsTurnos {
		turnos = append(turnos, parseTurno(turno))
	}
	return Juego{
		Turnos: turnos,
		Id:     id,
	}
}

func ComprobarValido(juego Juego, r int, g int, b int) bool {
	for _, turno := range juego.Turnos {
		if turno.Rojo > r {
			return false
		}
		if turno.Verde > g {
			return false
		}
		if turno.Azul > b {
			return false
		}
	}
	return true
}

func PoderMinimo(juego Juego) int {
	var r, g, b int
	for _, turno := range juego.Turnos {
		if turno.Rojo > r {
			r = turno.Rojo
		}
		if turno.Verde > g {
			g = turno.Verde
		}
		if turno.Azul > b {
			b = turno.Azul
		}
	}

	return r * g * b
}
