# Lab 4: Carga de modelos

Este programa carga [`assets/cubo_vista.obj`](assets/cubo_vista.obj), un cubo de 8 vértices y 12 caras triangulares. El cargador lee los vértices y convierte los índices OBJ (que comienzan en 1) a índices de Rust (que comienzan en 0).

Por cada grupo de tres índices, el programa busca sus vértices y dibuja el contorno con la función `triangle`, que utiliza líneas Bresenham y píxeles. Para esta primera parte, la posición en pantalla se calcula solo con `x` e `y`. El modelo ya está rotado en el archivo OBJ para que sus caras se distingan; el programa lo escala y centra automáticamente dentro de la ventana.

## Ejecutar

Desde la raíz del repositorio:

```sh
cargo run
```

Para generar de nuevo la captura:

```sh
cargo run -- --screenshot
```

## Resultado

![Cubo OBJ triangulado renderizado por el programa](captura.png)
