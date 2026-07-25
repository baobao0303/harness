# Project: E-Commerce Platform (`SPEC-EC-001`)

## Architecture
- Full-stack E-Commerce Platform built with modern web storefront, backend API endpoints, SQLite durable data layer (`harness.db`), and order engine state machine.
- Verification and telemetry driven by Harness CLI (`./scripts/harness`).

## Milestones

| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| 1 | Exploration & Baseline | Codebase inspection, harness database status check, story registration check | none | DONE |
| 2 | R1: Storefront Catalog & Autocomplete Search | Navigation bar, hero carousel, footer, multi-attribute filter sidebar, autocomplete search (<50ms), product view | M1 | DONE |
| 3 | R2: Shopping Cart & Dynamic Checkout | Slide-over cart drawer, quantity controls, promo coupon validation, 3-step checkout wizard | M2 | DONE |
| 4 | R3: Order Engine State Machine & Customer Dashboard | Customer authentication, state machine transitions, order tracking timeline, invoice PDF export | M3 | DONE |
| 5 | E2E Verification & Harness Telemetry | `./scripts/harness story verify US-EC-001`, `./scripts/harness story verify US-EC-003`, `./scripts/harness audit`, `./scripts/harness export-trace --format tldraw` | M2, M3, M4 | IN_PROGRESS |

## Interface Contracts
### Storefront Catalog ↔ API
- `GET /api/products`: Query products with parameters (`category`, `price_min`, `price_max`, `rating_min`, `in_stock`, `sort`).
- `GET /api/products/:slug`: Fetch product details with category info, images array, rating, stock status.
- `GET /api/search`: Query autocomplete suggestions with debounce (<50ms response).

### Shopping Cart ↔ Promo & Checkout
- `POST /api/cart/coupon`: Validate coupon code (`code`), returns discount percentage/amount and total preview.
- `POST /api/orders`: Submit cart items, user address, and payment method to generate order in `pending` state.

### Order Engine State Machine
- States: `pending` ➔ `processing` ➔ `shipped` ➔ `delivered` / `cancelled`.
- Transitions validated by order engine rules.
- `GET /api/orders/my-orders`: Retrieve tracking history and status timeline for customer dashboard.

## Code Layout
- Frontend & Server source files in `/Users/bao312/Desktop/harness/projects/ecommerce`
- Harness CLI at `./scripts/harness` / `scripts/bin/harness-cli`
- SQLite telemetry database at `./harness.db`
