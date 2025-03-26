# Distribución de Poisson en Rust

Este repositorio contiene una implementación en Rust de la distribución de Poisson, una distribución de probabilidad discreta que expresa la probabilidad de que ocurra un número determinado de eventos durante un intervalo de tiempo o espacio fijo.

## Índice
- [Descripción](#descripción)
- [Fórmula Matemática](#fórmula-matemática)
- [Implementación](#implementación)
- [Uso del Programa](#uso-del-programa)
- [Ejemplo de Ejecución](#ejemplo-de-ejecución)
- [Requisitos](#requisitos)

## Descripción
La distribución de Poisson es una distribución de probabilidad discreta que expresa la probabilidad de que ocurra un determinado número de eventos en un intervalo de tiempo o espacio específico, si estos eventos ocurren con una tasa media constante conocida e independientemente del tiempo transcurrido desde el último evento.

Este programa implementa esta distribución matemática y proporciona una interfaz sencilla para calcular probabilidades con diferentes valores de λ (lambda) y n (número de ocurrencias).

## Fórmula Matemática
La distribución de Poisson se define mediante la siguiente fórmula:

$$P(X = n; \lambda) = \frac{\lambda^n e^{-\lambda}}{n!}$$

Donde:
- **λ (lambda)** es el número esperado de ocurrencias en el intervalo dado.
- **n** es el número de ocurrencias para el cual se calcula la probabilidad.
- **e** es la base del logaritmo natural (aproximadamente 2.71828).
- **n!** es el factorial de n.

## Implementación
El código implementa la distribución de Poisson mediante una estructura y un método asociado:
La función calcula la probabilidad de Poisson siguiendo estos pasos:
1. Calcula λ^n.
2. Calcula e^(-λ).
3. Multiplica estos valores para obtener el numerador.
4. Calcula el factorial de n.
5. Divide el numerador por el factorial para obtener la probabilidad.

## Uso del Programa
El programa ofrece un menú interactivo con las siguientes opciones:
1. **Agregar el valor de lambda (λ):** Este parámetro representa la tasa media de ocurrencia del evento.
2. **Agregar el valor de n:** Este parámetro representa el número de ocurrencias para el cual se desea calcular la probabilidad.
3. **Mostrar resultado de la función de Poisson:** Calcula y muestra la probabilidad P(X = n).
4. **Mostrar fórmula:** Muestra la fórmula de Poisson con los valores actuales de n y λ.
5. **Salir:** Finaliza el programa.

## Ejemplo de Ejecución
En este ejemplo:
- **λ = 2.5** (la tasa media de ocurrencia)
- **n = 3** (el número de ocurrencias)

La probabilidad **P(X = 3; λ = 2.5)** = 0.2138... o aproximadamente **21.39%**.

## Requisitos
- **Rust** (recomendado 1.54.0 o superior)
- **Cargo** (incluido con Rust)

### Para ejecutar el programa:
Este programa es una herramienta educativa para entender y trabajar con la distribución de Poisson, una distribución de probabilidad fundamental en estadística y teoría de la probabilidad.

---

# Instalación de Rust en Windows

Este archivo proporciona las instrucciones para instalar Rust en un sistema operativo Windows.

## Requisitos
- Sistema operativo: Windows 7 o superior.
- Conexión a Internet para descargar los paquetes necesarios.

## Paso a Paso

### 1. Instalar Rust utilizando rustup
Rust se instala a través de un instalador llamado `rustup`, que gestiona la instalación de Rust y las herramientas relacionadas.

#### Paso 1: Descargar el instalador
1. Ve al sitio oficial de Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2. En la página, haz clic en el enlace de descarga para el instalador de Windows (`rustup-init.exe`).

#### Paso 2: Ejecutar el instalador
1. Abre el archivo `rustup-init.exe` que descargaste.
2. Aparecerá una ventana del terminal (Command Prompt) en la que se ejecutará el instalador.
3. El instalador te preguntará si deseas proceder con la instalación. Presiona `1` para instalar Rust con la configuración predeterminada.
   
   > **Nota:** Si prefieres personalizar la instalación, puedes elegir otras opciones, pero la opción predeterminada funciona para la mayoría de los usuarios.

#### Paso 3: Esperar a que finalice la instalación
1. El instalador descargará y configurará Rust en tu sistema. Esto puede tomar unos minutos dependiendo de tu conexión a Internet.

#### Paso 4: Verificar la instalación
1. Una vez que la instalación haya terminado, abre una nueva ventana de terminal (símbolo del sistema o PowerShell) y escribe el siguiente comando para verificar que Rust está instalado correctamente:
   ```bash
   rustc --version
   ```
   Si la instalación fue exitosa, deberías ver la versión de Rust que acabas de instalar, algo como:
   ```
   rustc 1.54.0 (a178d0322 2021-07-26)
   ```

#### Paso 2: Agregar Rust al PATH (si no se hace automáticamente)
Si por alguna razón Rust no se agrega automáticamente al PATH, puedes hacerlo manualmente.

##### Paso 1: Buscar la carpeta de instalación
Por defecto, Rust se instala en el directorio `C:\Users\<TuNombreDeUsuario>\.cargo\bin`.

##### Paso 2: Modificar el PATH
1. Haz clic derecho en el botón de Inicio y selecciona Sistema.
2. Haz clic en Configuración avanzada del sistema.
3. En la ventana que se abre, selecciona Variables de entorno.
4. En Variables del sistema, selecciona la variable llamada Path y haz clic en Editar.
5. En la ventana de edición, agrega la siguiente ruta (ajústala si tienes una ubicación diferente para la instalación de Rust):
   ```
   C:\Users\<TuNombreDeUsuario>\.cargo\bin
   ```
6. Haz clic en Aceptar para cerrar todas las ventanas.

#### Paso 3: Instalar Cargo (si no se instala automáticamente)
Cargo es el gestor de paquetes de Rust y debería instalarse automáticamente con el paso anterior. Si no es así, puedes instalarlo manualmente ejecutando el siguiente comando:
```bash
rustup component add cargo
```

#### Paso 4: Actualizar Rust (opcional)
Para asegurarte de que tienes la última versión de Rust, puedes actualizar Rust y sus componentes utilizando el siguiente comando:
```bash
rustup update
```

#### Paso 5: Comprobar la instalación de Cargo
Después de la instalación, verifica que Cargo también está instalado correctamente ejecutando:
```bash
cargo --version
```
Si todo está en orden, deberías ver algo como:
```
cargo 1.54.0 (cb1e1e7d5 2021-07-26)
```

¡Listo!

Con esto, Rust debería estar correctamente instalado en tu sistema Windows. Ahora puedes comenzar a escribir y ejecutar programas en Rust.