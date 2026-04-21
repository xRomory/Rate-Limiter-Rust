# Rust Rate Limiter API — Learning Roadmap

A beginner-friendly, step-by-step project for building a **production-inspired rate limiter API in Rust**.

This repository is not just a finished project—it’s a **guided roadmap** to help you understand how real-world backend systems are built.

***Disclaimer: This README file is AI-generated since I'm too lazy to write it on my own.***

---

## Project Goal

Build a rate limiter from scratch and evolve it into a **real-world API component**.

By the end, you will understand:
- Core rate limiting algorithms
- Rust concurrency (`Arc`, `Mutex`)
- Building APIs using a web framework
- How production systems enforce limits

---

## What I’ll Learn

- Systems thinking (how backend services control traffic)
- State management in Rust
- Thread safety and shared state
- API design basics
- Incremental development (build → test → improve)

---

## Project Phases

---

### Phase 1 — Core Rate Limiter Logic (No API Yet)
**Phase 1: DONE**

**Goal:** Understand how rate limiting works internally.

#### Tasks:
- [x] Implement a `TokenBucket` struct
- [x] Add:
  - [x] `new(capacity, refill_rate)`
  - [x] `allow_request() -> bool`
- [x] Simulate requests in `main.rs`
- [x] Print:
  - [x] `"Allowed"`
  - [x] `"Rate Limited"`

#### Key Concepts:
- Time tracking (`Instant`)
- Floating-point calculations
- Mutable state

#### Output Example:
```
Request 1 → Allowed
Request 2 → Allowed
Request 11 → Rate Limited
```

---

### Phase 2 — Expose as an HTTP API

**Goal:** Turn your logic into a usable web service.

#### Suggested Framework:
- Axum (lightweight and beginner-friendly)

#### Tasks:
- [ ] Create a basic HTTP server
- [ ] Add endpoint:
  - [ ] `GET /request`
- [ ] Return:
  - [ ] `200 OK` → allowed
  - [ ] `429 Too Many Requests` → blocked
- [ ] Store limiter in shared state using:
  - [ ] `Arc`
  - [ ] `Mutex`

#### Key Concepts:
- Shared state in Rust
- Basic routing
- HTTP status codes

---

### Phase 3 — Per-User / Per-IP Rate Limiting

**Goal:** Make the limiter more realistic.

#### Tasks:
- [ ] Replace single limiter with:
  - [ ] `HashMap<String, TokenBucket>`
- [ ] Identify users via:
  - [ ] IP address OR
  - [ ] API key
- [ ] Create buckets dynamically

#### Key Concepts:
- Hash maps
- Dynamic state management
- Multi-user systems

---

### Phase 4 — Configuration Support

**Goal:** Make limits configurable like real systems.

#### Tasks:
- [ ] Define rate limit rules (hardcoded first)
- [ ] Optionally load from JSON
- [ ] Support different tiers:
  - [ ] Free → low limit
  - [ ] Premium → higher limit

#### Example:
```json
{
  "free": { "capacity": 10, "refill_rate": 5 },
  "premium": { "capacity": 100, "refill_rate": 50 }
}
```

---

### Phase 5 — Middleware Integration

**Goal:** Apply rate limiting automatically to routes.

#### Tasks:
- [ ] Refactor limiter into middleware
- [ ] Apply to all or selected routes
- [ ] Remove limiter logic from handlers

#### Key Concepts:
- Middleware pattern
- Separation of concerns

---

### Phase 6 — Advanced (Optional but Impressive)

#### 🔹 Distributed Rate Limiting
- [ ] Use Redis for shared state
- [ ] Allow multiple server instances

#### 🔹 Alternative Algorithms
- [ ] Implement:
  - [ ] Leaky Bucket
  - [ ] Fixed Window
  - [ ] Sliding Window

#### 🔹 Observability
- [ ] Add logging
- [ ] Track:
  - [ ] Allowed requests
  - [ ] Blocked requests

#### 🔹 Benchmarking
- [ ] Measure requests per second
- [ ] Compare algorithms

---

## Example API Usage

```bash
curl http://localhost:3000/request
```

### Responses:

```
200 OK
Request allowed
```

```
429 Too Many Requests
Rate limit exceeded
```

---

## 🛠 Tech Stack

- Rust
- Axum (web framework)
- Tokio (async runtime)
- Serde (for JSON, optional)

---

## Project Structure (Suggested)

```
src/
├── main.rs
├── limiter/
│   ├── token_bucket.rs
│   └── mod.rs
├── api/
│   ├── routes.rs
│   └── middleware.rs
```

---

## Why This Project Matters

Rate limiting is used in:
- API gateways
- Authentication systems
- Payment services
- Cloud platforms

This project demonstrates **real backend engineering skills**, not just CRUD.

---

## Suggested Workflow

1. Build → test locally  
2. Break things → fix them  
3. Refactor → improve structure  
4. Repeat  

---

## Future Improvements

- Add authentication
- Integrate with a database
- Deploy using Docker
- Add a frontend dashboard

---

## Contributing

This project is a personal learning roadmap, but feel free to:
- Suggest improvements
- Add new algorithms
- Optimize performance

---

## License

MIT License

---

## Final Note

Don’t rush to “advanced” features.

Focus on:
> **Understanding why things work, not just making them work.**

That’s what turns this from a project into real skill.
