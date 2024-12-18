```
pyrust-blockchain/
├── backend/             
│   ├── python/             # Python Backend (main backend logic)
│   │   ├── app.py          # Entry point for the Python application
│   │   ├── api/            # REST/GraphQL API routes
│   │   ├── models/         # Data models (e.g., transactions, blocks)
│   │   ├── utils/          # Helper functions
│   │   ├── services/       # Python services for blockchain logic
│   │   └── requirements.txt# Python dependencies
│   ├── rust/               # Rust Backend (performance-critical tasks)
│   │   ├── src/
│   │   │   ├── lib.rs      # Rust library entry point
│   │   │   ├── crypto/     # Cryptography utilities (hashing, signing)
│   │   │   ├── consensus/  # Consensus mechanism implementation
│   │   │   ├── ffi/        # Rust <-> Python FFI bindings
│   │   └── Cargo.toml      # Rust dependencies
│   ├── Dockerfile          # Unified backend Docker configuration
├── frontend/            
│   ├── app/                # Next.js app router
|   |   ├── page.tsx        # Next.js page component
│   │   ├── layout.tsx      # Next.js layout component
│   ├── components/         # UI components
│   ├── public/             # Static assets
|   ├── eslintrc.json          
│   ├── tailwind.config.ts  # Tailwind CSS configuration
│   └── package.json        # Node.js dependencies
├── scripts/             
│   ├── seed_network.py     # Network bootstrapping (Python)
│   └── analyze_chain.py    # Chain analysis (Python)
├── docker-compose.yml      # Orchestration for all services
└── README.md               # Documentation
```