# 🧠 Prediction Market on Stellar (Soroban)

## 📌 Project Description

This project is a basic decentralized prediction market built using Soroban smart contracts on the Stellar network. It allows users to create markets based on future events and resolve them once the outcome is known.

Prediction markets are powerful tools for aggregating collective intelligence and forecasting real-world events like elections, sports outcomes, or crypto prices.

---

## ⚙️ What It Does

- Allows users to create prediction markets with multiple outcomes
- Stores market data on-chain using Soroban smart contracts
- Enables resolution of markets by selecting a winning outcome
- Provides a way to fetch market details anytime

---

## ✨ Features

- 🏗️ Create a new prediction market  
- 📊 Multiple outcomes supported  
- ✅ Resolve markets with a winner  
- 🔍 Fetch market data on-chain  
- ⚡ Built using Soroban (Stellar Smart Contracts)  
- 🧩 Simple and extensible architecture  

---

## 🚀 Deployed Smart Contract Link

Prediction Market Contract:  
👉 [https://stellar.expert/explorer/public/contract/your_contract_id_here](https://stellar.expert/explorer/testnet/contract/CB3EN4VN2RTSTSQPSE42IHUIOUG5EWK3ESKI4WDFEAEZWT4SBU3EYOHK)

<img width="1919" height="905" alt="Screenshot 2026-03-19 143905" src=  "https://github.com/user-attachments/assets/1fd22c44-d43a-4be2-b786-35d1b4e65e56"/>

> Replace `your_contract_id_here` with your deployed contract ID.

---

## 🛠️ Tech Stack

- Rust (Soroban SDK)
- Stellar Blockchain
- Soroban CLI

---

## 📦 How to Use

1. Deploy the contract using Soroban CLI
2. Call `create_market` with:
   - Market ID
   - Question
   - List of outcomes
3. Call `resolve_market` to finalize result
4. Use `get_market` to fetch details

---

## 🔮 Future Improvements

- Add token-based betting
- User participation tracking
- Automated oracle-based resolution
- Rewards distribution system
- Frontend UI integration

---

## 📄 License

MIT License
