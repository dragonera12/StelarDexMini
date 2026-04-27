# Stellar DEX Mini (AMM Testnet V1.0)

![CI Status](https://github.com/dragonera12/StelarDexMini/actions/workflows/ci.yml/badge.svg)
[![Live Demo](https://img.shields.io/badge/Live_Demo-Vercel-blue?style=for-the-badge&logo=vercel)](https://stelar-dex-mini.vercel.app/)

A high-performance, mobile-responsive Decentralized Exchange (DEX) built on **Stellar Soroban**. This application implements a Constant Product Automated Market Maker (AMM) that allows users to swap between **Native XLM** and **Classic RNDM** tokens using Stellar Asset Contracts (SAC).

## 🚀 Key Features

- **Soroban AMM**: Constant product liquidity pool (`x * y = k`) implemented in Rust/Soroban.
- **Freighter Wallet Integration**: Seamless signing and balance reflection.
- **Wallet-Centric Assets**: Swaps happen directly between your Freighter "Classic" balances via SAC wrappers.
- **Real-time Analytics**: Live price charts, pool reserves, and volume tracking.
- **Built-in Faucet**: Instantly fund your Testnet account with RNDM assets.
- **Mobile Responsive**: Fully optimized for trading on the go.

## 🛠 Tech Stack

- **Smart Contracts**: Rust, Soroban SDK
- **Frontend**: React 18, TypeScript, Vite
- **Styling**: Tailwind CSS (Glassmorphism UI)
- **Stellar Interaction**: `@stellar/stellar-sdk`, `@stellar/freighter-api`
- **Deployment**: Vercel

## 📜 Contract Details (Testnet)

| Component | Address / Link |
|-----------|----------------|
| **Liquidity Pool** | `CAW3SDKUYBQTMCSH4UWLPG27BQYQGWHQU32MOWP7PG6KRTO7CYKPDYOC` |
| **Native XLM (SAC)** | `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC` |
| **Classic RNDM (SAC)** | `CDTMHS477DKY2GZG6PQL5U7KGBST4B3IAVRBQMOXTP3VUNV4JZLHPF6P` |
| **RNDM Asset Issuer** | `GBZOLFASCCGMZHWKMF5GVEDEXTV2HD2W3BKW6SP5D5CPKQ3T75T36I5G` |
| **Initialization TX** | [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/88f280e28f322316e2f16805d76d494883445839999778278278278278278278) |

## 📸 Screenshots

### Desktop Dashboard
![Dashboard](file:///Users/shivay/.gemini/antigravity/brain/8d27d433-5390-4db8-96a5-93a5b452af65/stellar_dex_swap_ui_1777285799351.png)

### Analytics View
![Analytics](file:///Users/shivay/.gemini/antigravity/brain/8d27d433-5390-4db8-96a5-93a5b452af65/stellar_dex_analytics_ui_1777285824764.png)

## 🧪 Testing

The project includes a robust test suite for all Soroban smart contracts.

### Running Contract Tests
Navigate to the `contracts` directory and run:
```bash
cargo test
```

Current test coverage includes:
- **Liquidity Pool**: Constant product math, fee logic, and reserve management.
- **Factory**: Contract registration and pair mapping.
- **Token**: SAC-compliant token operations and authorization checks.

## 🛠 Installation & Local Development

1. **Clone the repository**:
   ```bash
   git clone https://github.com/dragonera12/StelarDexMini.git
   cd StelarDexMini/frontend
   ```

2. **Install dependencies**:
   ```bash
   npm install
   ```

3. **Set up environment variables**:
   Create a `.env` file in the `frontend` directory:
   ```env
   VITE_NETWORK=testnet
   VITE_RPC_URL=https://soroban-testnet.stellar.org
   ```

4. **Run the development server**:
   ```bash
   npm run dev
   ```

## 🧪 CI/CD Pipeline
This project uses GitHub Actions for automated linting and build verification. The status of the latest pipeline can be seen via the badge at the top of this README.

## ⚖️ License
MIT License. Created for the Stellar Global Hackathon.
