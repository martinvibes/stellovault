# StelloVault

**A secure trade finance decentralized application (dApp) built on Stellar & Soroban**  
Tokenizing collateral (invoices, commodities, etc.) to unlock instant liquidity for African exporters and importers, bridging the massive trade finance gap.

[![Stellar](https://img.shields.io/badge/Built%20on-Stellar-blue?logo=stellar)](https://stellar.org)
[![Soroban](https://img.shields.io/badge/Smart%20Contracts-Soroban-orange)](https://soroban.stellar.org)
[![Next.js](https://img.shields.io/badge/Frontend-Next.js-black?logo=next.js)](https://nextjs.org)
[![Rust](https://img.shields.io/badge/Backend-Rust-orange?logo=rust)](https://www.rust-lang.org)

## 🚀 Overview

StelloVault is a trade finance dApp that enables African SMEs to tokenize real-world assets (e.g., invoices, commodities) as Stellar assets with embedded metadata, use them as collateral in multi-signature escrows managed by **Soroban smart contracts**, and unlock instant cross-border liquidity.

Key innovations:
- **Collateral Tokenization** — Real assets become fractional, traceable Stellar tokens.
- **Automated Escrows** — Multi-sig + conditional release triggered by shipment verification oracles (e.g., IoT/Maersk integration).
- **Dynamic Financing** — Algorithmic loans based on on-chain history and utilization.
- **Risk Scoring** — Backend uses transaction history for creditworthiness.
- **Governance** — Quadratic voting for stakeholders to decide accepted collateral types.

### Why It Matters

Africa faces a **trade finance gap of over $100–120 billion annually** (sources: Afreximbank, African Development Bank, World Bank estimates), disproportionately affecting SMEs — which represent >90% of businesses but are underserved by traditional finance. This stifles **$100B+ in potential exports** and intra-African trade under the **AfCFTA**.

StelloVault leverages:
- Stellar's low-cost, fast settlements and native asset issuance
- Soroban's Rust-based smart contracts for secure, programmable logic
- To reduce intermediary costs by up to **50%**, enable fractional ownership, and foster inclusive trade.

Target: Scalable to **1,000+ deals/month** with real-time oracle verification.

## ✨ Key Features

- **Collateral Tokenization** — Mint Stellar assets from invoices/commodities with provenance metadata.
- **Multi-Sig Escrows & Automated Release** — Soroban enforces release upon oracle confirmation (shipment delivered, quality verified).
- **Oracle Integration** — Real-time data feeds (planned: Maersk APIs, IoT devices, Chainlink-style oracles).
- **Risk Scoring Engine** — Rust backend analyzes on-chain history for dynamic loan terms.
- **Frontend Dashboard** — Next.js interface for deal origination, collateral upload, escrow monitoring, and repayments.
- **Governance Module** — On-chain voting (quadratic mechanisms) for protocol parameters and collateral acceptance.
- **Flash Settlements** — Instant cross-border payments using Stellar's built-in DEX/path payments.

## 📂 Repository Structure (Monorepo)

```
stellovault/
├── contracts/                    # Soroban Smart Contracts (Rust)
│   ├── Cargo.toml               # Rust dependencies for contracts
│   ├── rust-toolchain.toml      # Rust toolchain configuration
│   └── src/
│       └── lib.rs               # Main contract: StelloVault trade finance logic
│
├── frontend/                     # Next.js Frontend Application
│   ├── package.json             # Node.js dependencies
│   ├── next.config.ts           # Next.js configuration
│   ├── tailwind.config.js       # Tailwind CSS configuration
│   ├── src/
│   │   ├── app/                 # Next.js App Router
│   │   │   ├── layout.tsx       # Root layout
│   │   │   ├── page.tsx         # Home page
│   │   │   ├── dashboard/       # User dashboard
│   │   │   ├── escrows/         # Escrow management
│   │   │   ├── collateral/      # Collateral tokenization
│   │   │   └── profile/         # User profile
│   │   ├── components/          # Reusable React components
│   │   │   ├── ui/              # UI primitives (Button, etc.)
│   │   │   ├── forms/           # Form components
│   │   │   └── dashboard/       # Dashboard-specific components
│   │   ├── lib/                 # Library utilities and configurations
│   │   ├── hooks/               # Custom React hooks
│   │   ├── types/               # TypeScript type definitions
│   │   └── utils/               # Utility functions
│   └── public/                  # Static assets
│
├── server/                      # Rust Backend API Server
│   ├── Cargo.toml               # Rust dependencies for backend
│   ├── src/
│   │   ├── main.rs              # Server entry point
│   │   ├── lib.rs               # Library exports
│   │   ├── handlers.rs          # API route handlers
│   │   ├── models.rs            # Data models and types
│   │   ├── routes.rs            # Route definitions
│   │   ├── services.rs          # Business logic services
│   │   ├── middleware.rs        # HTTP middleware
│   │   └── utils.rs             # Utility functions
│   └── tests/                   # Integration tests
│
└── README.md                    # Project documentation
```

### Directory Details

#### Contracts (`/contracts`)
- **Purpose**: Soroban smart contracts for trade finance operations
- **Tech**: Rust with Soroban SDK
- **Key Contract**: `StelloVaultContract` - handles collateral tokenization and escrow management
- **Build**: `cargo build --release --target wasm32-unknown-unknown`

#### Frontend (`/frontend`)
- **Purpose**: User interface for the dApp
- **Tech**: Next.js 14+, TypeScript, Tailwind CSS
- **Features**: Dashboard, escrow management, collateral tokenization
- **Scripts**: `npm run dev` (development), `npm run build` (production)

#### Server (`/server`)
- **Purpose**: Backend API server for analytics, user management, and external integrations
- **Tech**: Rust with Axum web framework
- **Features**: REST API, database integration, risk scoring engine
- **Scripts**: `cargo run` (development), `cargo build --release` (production)

### Getting Started

#### Prerequisites
- Rust (latest stable)
- Node.js 18+
- PostgreSQL (for backend database)
- Soroban CLI (for contract development)

#### Quick Start

1. **Clone and setup contracts:**
   ```bash
   cd contracts
   cargo build --release --target wasm32-unknown-unknown
   ```

2. **Setup frontend:**
   ```bash
   cd frontend
   npm install
   npm run dev
   ```

3. **Setup backend:**
   ```bash
   cd server
   cargo run
   ```

### Development Workflow

1. **Contracts**: Modify smart contract logic in `contracts/src/lib.rs`
2. **Frontend**: Add UI components and pages in respective directories
3. **Backend**: Implement API endpoints and business logic in server modules
4. **Testing**: Run tests for each component separately
5. **Deployment**: Deploy contracts to Stellar, build and deploy frontend/backend

### Key Integration Points

- **Contract ↔ Frontend**: Direct Soroban contract calls from React components
- **Frontend ↔ Backend**: REST API calls for analytics and user data
- **Backend ↔ Contracts**: Indexer services to track on-chain events
- **External APIs**: Integration with shipping providers (Maersk) and oracles
