#!/bin/bash

# Configuración
ROWS=$(tput lines)
COLS=$(tput cols)
SLEEP=0.1

# Generar copos de nieve
while true; do
    tput cup $((RANDOM % ROWS)) $((RANDOM % COLS))  # Mueve el cursor a una posición aleatoria
    echo -n "*"
    sleep $SLEEP
done

