# Rust-React-Tauri-MovieProject
a rust React tauri movie library installable in your own pc

## project structure
```
my_media_manager/
├── src/
│   ├── main.rs          # Punto d'ingresso
│   ├── models/          # Modelli dati
│   │   ├── mod.rs       # Esposizione moduli
│   │   ├── movie.rs     # Struttura Film
│   │   └── tv_show.rs   # Struttura Serie TV
│   ├── scanner/         # Logica scansione file
│   │   ├── mod.rs
│   │   ├── movie.rs     # Parser per film
│   │   └── tv.rs        # Parser per serie
│   ├── db/              
│   │   ├── mod.rs       # Configurazione DB
│   │   └── handler.rs   # Operazioni DB
│   └── api/             # Endpoint API
│       ├── mod.rs
│       ├── movies.rs
│       └── tv.rs
├── migrations/          # Script migrazione DB
└── frontend/            # Cartella React
```
