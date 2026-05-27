import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function Header(){
  return(
   <header>
     <h1>Cotizador ADSO</h1>
   </header>
  )
}
function Principal(){
  return(
    <main>
      <p>ingresa tu texto</p>
      <input type="text" className="entrada"/>

    </main>
  );
}
function BarraLateral(){
  return (
    <aside>
      <section>
        <strong>
          <p>tabla de datos</p>
        </strong>
        <ul >
          <li className="lista">hola</li>
          <li className="lista">hola</li>
          <li className="lista">hola</li>
        </ul>
      </section>
    </aside>
  )
}

function App() {
  // 3. El return con el HTML va dentro de App
  return (
    <>
      {/* 1. Contenedor general de la página */}
      <div className="contenedor-pagina">
        
        {/* El Header se queda arriba solo */}
        <Header />

        {/* 2. NUEVO CONTENEDOR: Solo para lo que va de lado a lado */}
        <div className="contenedor-body">
          <BarraLateral />
          <Principal />
        </div>

      </div>
    </>
  );
} // <-- Aquí es donde verdaderamente debe cerrar la función App
export default App;

