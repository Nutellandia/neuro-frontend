# NEURO-GENESIS v5.0 (Hybrid Architecture)
        
## GUIDA DI AVVIO

### 1. FRONTEND (React)
La simulazione visiva e sensoriale.
- `npm install`
- `npm start`

### 2. BACKEND (Node.js - Cervello Remoto)
Gestisce la logica pesante e il DB.
- `cd backend`
- `npm install`
- `node server.js`

### 3. RUST PHYSICS (Wasm - Motore Fisico)
Accelera la simulazione a 60fps con >10k neuroni.
- `cd rust_core`
- `wasm-pack build --target web`
- Copia la cartella `pkg` generata dentro `src/wasm` del frontend.
