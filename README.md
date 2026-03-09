# La Viborita en Rust 🐍

Una implementación del clásico juego "Snake" (Viborita) para la terminal, desarrollada en **Rust**. Este proyecto utiliza **Ratatui** para el renderizado de la interfaz de usuario (TUI) y **Crossterm** para el manejo de eventos de teclado y el control de la terminal en modo crudo.

## 🛠️ Arquitectura del Proyecto

El código sigue principios de **Responsabilidad Única (SRP)** y una estructura modular para facilitar el mantenimiento y la legibilidad:

### 1. Modelos de Datos (`src/models.rs`)

Contiene las estructuras fundamentales y enumeradores que definen el estado lógico:

- **`Direccion`**: Enum para controlar el movimiento (`Arriba`, `Abajo`, `Izquierda`, `Derecha`).
- **`Posicion`**: Estructura de coordenadas `u16` para los segmentos de la serpiente y la comida.

### 2. Lógica del Juego (`src/main.rs` - `impl Juego`)

Encapsula las reglas del juego separadas de la representación visual:

- **Cálculo de Movimiento**: Uso de `saturating_sub` para evitar desbordamientos matemáticos.
- **Gestión de Cuerpo**: Implementación con `VecDeque` para inserciones eficientes en la cabeza (`push_front`) y eliminaciones en la cola (`pop_back`).
- **Detección de Colisiones**: Uso de iteradores (`iter()`, `skip(1)`, `any()`) para detectar choques con el propio cuerpo de forma eficiente y perezosa (lazy).

### 3. Interfaz de Usuario (`main.rs`)

Diseño visual basado en capas:

- **Layout**: División vertical de la pantalla mediante `Constraint`.
- **Canvas**: Renderizado basado en coordenadas geométricas para la serpiente y la comida.
- **Pop-ups**: Sistema de centrado dinámico para mensajes de "Game Over" con limpieza de fondo mediante `Clear`.

## 🕹️ Mecánicas del Juego

- **Crecimiento**: La serpiente aumenta su tamaño y el puntaje al colisionar con la comida.
- **Validación de Dirección**: Se impide el giro de 180° (por ejemplo, si vas hacia arriba, no puedes cambiar directamente a abajo) para evitar colisiones accidentales.
- **Fin del Juego**: Ocurre al tocar los bordes del mapa o colisionar con cualquier segmento del cuerpo.

## 🚀 Instalación y Ejecución

1. **Clonar el repositorio**:
   ```bash
   git clone [https://github.com/ffespechen/rust-ratatui-snake-viborita.git](https://github.com/ffespechen/rust-ratatui-snake-viborita.git)
   cd nibbles-rust
   ```
