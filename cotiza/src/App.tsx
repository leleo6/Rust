import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
function App() {
  // 1. Declarar los estados dentro de App
  const [nombre, setNombre] = useState("");
  const [cantidad, setCantidad] = useState(1);
  const [precio, setPrecio] = useState(0.0);
  const [resultado, setResultado] = useState("");
  // 2. Definir la función de cotizar dentro de App
  async function cotizar() {
    try {
      const respuesta = await invoke<string>("calcular_cotizacion", {
        item: {
          nombre: nombre,
          cantidad: Number(cantidad),
          precio: Number(precio),
        },
      });
      setResultado(respuesta);
    } catch (error) {
      console.error("Error al calcular la cotización:", error);
    }
  }
  // 3. El return con el HTML va dentro de App
  return (
    <main className="container">
      <h3>Calcular Cotización</h3>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          cotizar();
        }}
        style={{ display: "flex", flexDirection: "column", gap: "10px", marginTop: "20px" }}
      >
        <input
          type="text"
          placeholder="Nombre del artículo"
          value={nombre}
          onChange={(e) => setNombre(e.target.value)}
        />
        <input
          type="number"
          placeholder="Cantidad"
          value={cantidad}
          onChange={(e) => setCantidad(Number(e.target.value))}
        />
        <input
          type="number"
          step="0.01"
          placeholder="Precio"
          value={precio}
          onChange={(e) => setPrecio(Number(e.target.value))}
        />
        <button type="submit">Calcular en Rust</button>
      </form>
      {resultado && <p style={{ fontWeight: "bold", marginTop: "10px" }}>{resultado}</p>}
    </main>
  );
} // <-- Aquí es donde verdaderamente debe cerrar la función App
export default App;