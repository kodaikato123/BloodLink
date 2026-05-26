# BloodLink

A blockchain-powered blood donor tracking and emergency matching system using Stellar Soroban.

---

# Problem

Hospitals manually search for compatible blood donors during emergencies, causing delays, fake records, and unreliable blood availability tracking.

# Solution

BloodLink uses Stellar Soroban smart contracts to verify donors, match urgent blood requests instantly, and maintain tamper-proof donation records.

---

# Timeline

- Week 1: Smart contract development
- Week 2: Donor registration frontend
- Week 3: Emergency request dashboard
- Week 4: Testing and deployment

---

# Stellar Features Used

- Soroban smart contracts
- Custom donor verification assets
- Trustlines
- XLM transfers

---

# Vision and Purpose

BloodLink aims to improve emergency healthcare coordination in Southeast Asia by providing transparent and trusted blood donor infrastructure.

---

# Prerequisites

- Rust
- Soroban CLI
- Stellar testnet account

Recommended version:

```bash
soroban --version
```

---

# Build Contract

```bash
soroban contract build
```

---

# Run Tests

```bash
cargo test
```

---

# Deploy to Testnet

```bash
soroban contract deploy \
--wasm target/wasm32-unknown-unknown/release/blood_link.wasm \
--source admin \
--network testnet
```

---

# Sample CLI Invocation

```bash
soroban contract invoke \
--id CONTRACT_ID \
--source hospital_admin \
--network testnet \
-- request_blood \
--patient_name "Juan Dela Cruz" \
--blood_type A+ \
--donor DONOR_ADDRESS
```

---

# License

MIT