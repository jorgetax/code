use std::io::{self, Write};

// Definimos los tipos de movimientos en el inventario
#[derive(Debug, Clone, PartialEq)]
pub enum TipoMovimiento {
    Entrada,
    Salida,
}

// Estructura de una transacción individual
#[derive(Debug)]
pub struct Transaccion {
    pub fecha: String,
    pub descripcion: String,
    pub tipo: TipoMovimiento,
    pub cantidad: u32,
    pub precio_unitario: f64,
}

// Estado central del Kardex
#[derive(Debug)]
pub struct Kardex {
    pub producto: String,
    pub transacciones: Vec<Transaccion>,
    pub stock_actual: u32,
    pub costo_promedio: f64,
}

impl Kardex {
    pub fn new(producto: String) -> Self {
        Self {
            producto,
            transacciones: Vec::new(),
            stock_actual: 0,
            costo_promedio: 0.0,
        }
    }

    // Método principal de lógica de negocio (Cálculo de Promedio Ponderado)
    pub fn registrar_movimiento(&mut self, transaccion: Transaccion) -> Result<(), String> {
        match transaccion.tipo {
            TipoMovimiento::Entrada => {
                let valor_actual = self.stock_actual as f64 * self.costo_promedio;
                let valor_nuevo = transaccion.cantidad as f64 * transaccion.precio_unitario;

                self.stock_actual += transaccion.cantidad;
                self.costo_promedio = (valor_actual + valor_nuevo) / self.stock_actual as f64;
            }
            TipoMovimiento::Salida => {
                if transaccion.cantidad > self.stock_actual {
                    return Err(format!(
                        "Stock insuficiente. Tienes {} unidades.",
                        self.stock_actual
                    ));
                }
                self.stock_actual -= transaccion.cantidad;
                // En una salida por costo promedio, el costo promedio no cambia.
            }
        }

        self.transacciones.push(transaccion);
        Ok(())
    }

    // Renderizado del Kardex en formato tabla
    pub fn imprimir_reporte(&self) {
        println!(
            "\n=========================================================================================="
        );
        println!(
            "KARDEX - Producto: {} | Método: Promedio Ponderado",
            self.producto
        );
        println!(
            "------------------------------------------------------------------------------------------"
        );
        println!(
            "{:<12} | {:<15} | {:<8} | {:<12} | {:<10} | {:<10}",
            "Fecha", "Descripción", "Tipo", "Cantidad", "P. Unit", "Valor Total"
        );
        println!(
            "------------------------------------------------------------------------------------------"
        );

        for t in &self.transacciones {
            let tipo_str = if t.tipo == TipoMovimiento::Entrada {
                "ENTRADA"
            } else {
                "SALIDA"
            };
            let valor_total = t.cantidad as f64 * t.precio_unitario;

            println!(
                "{:<12} | {:<15} | {:<8} | {:<12} | Q{:<9.2} | Q{:<10.2}",
                t.fecha, t.descripcion, tipo_str, t.cantidad, t.precio_unitario, valor_total
            );
        }
        println!(
            "------------------------------------------------------------------------------------------"
        );
        println!(
            "SALDO FINAL: {} unidades | Costo Promedio: Q{:.2} | Valor Total Inventario: Q{:.2}",
            self.stock_actual,
            self.costo_promedio,
            self.stock_actual as f64 * self.costo_promedio
        );
        println!(
            "==========================================================================================\n"
        );
    }
}

// Utilidad para capturar la entrada estándar de manera robusta
fn capturar_entrada(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("--- Sistema Automatizado de Kardex ---");
    let producto = capturar_entrada("Ingrese el nombre del producto para el Kardex: ");
    let mut mi_kardex = Kardex::new(producto);

    loop {
        println!("\n¿Qué desea registrar?");
        println!("1. Entrada (Compra)");
        println!("2. Salida (Venta)");
        println!("3. Imprimir Kardex y Salir");

        let opcion = capturar_entrada("Seleccione una opción (1-3): ");

        if opcion == "3" {
            break;
        }

        let fecha = capturar_entrada("Fecha (DD-MM-YYYY): ");
        let descripcion = capturar_entrada("Descripción (Ej. Factura 001): ");

        let cantidad: u32 = match capturar_entrada("Cantidad: ").parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Error: La cantidad debe ser un número entero.");
                continue;
            }
        };

        let tipo;
        let precio_unitario: f64;

        if opcion == "1" {
            tipo = TipoMovimiento::Entrada;
            precio_unitario = match capturar_entrada("Precio de Compra Unitario: ").parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("❌ Error: El precio debe ser un número válido.");
                    continue;
                }
            };
        } else if opcion == "2" {
            tipo = TipoMovimiento::Salida;
            // En salidas contables por promedio ponderado, el costo es el promedio actual
            precio_unitario = mi_kardex.costo_promedio;
            println!(
                "ℹ️  El costo unitario de salida se asume como Q{:.2} (Costo Promedio actual).",
                precio_unitario
            );
        } else {
            println!("❌ Opción no válida.");
            continue;
        }

        let transaccion = Transaccion {
            fecha,
            descripcion,
            tipo,
            cantidad,
            precio_unitario,
        };

        match mi_kardex.registrar_movimiento(transaccion) {
            Ok(_) => println!("✅ Movimiento registrado exitosamente."),
            Err(e) => println!("❌ Fallo al registrar: {}", e),
        }
    }

    mi_kardex.imprimir_reporte();
}
