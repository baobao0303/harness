# E-Commerce Storefront — Product & System Architecture Specification 🛒

> **Spec ID**: `SPEC-EC-001`  
> **Intake ID**: `Intake #1`  
> **Status**: Accepted & Active  
> **Risk Lane**: `normal`  
> **Author**: Chief of Staff (Mina)  
> **Target Workspace**: [projects/ecommerce/](file:///Users/bao312/Desktop/harness/projects/ecommerce/)

---

## 1. Product Executive Summary & Vision

The **E-Commerce Storefront** is a modern, high-performance web shopping platform engineered for lightning-fast navigation, seamless checkout, and rich interactive visual aesthetics (Glassmorphism, dark/light theme support, responsive micro-animations).

### 🎯 Key Performance Targets & Core Web Vitals (CWV)
- **Largest Contentful Paint (LCP)**: `< 1.2s`
- **Interaction to Next Paint (INP)**: `< 100ms`
- **Cumulative Layout Shift (CLS)**: `< 0.05`
- **Client Offline Resilience**: Instant UI updates via optimistic rendering and local storage persistence.

---

## 2. System Architecture & Relational Data Schema (ERD)

```text
 ┌───────────────┐       ┌────────────────┐       ┌────────────────┐
 │     users     │1     *│     orders     │1     *│  order_items   │
 ├───────────────┤───────├────────────────┤───────├────────────────┤
 │ id (PK)       │       │ id (PK)        │       │ id (PK)        │
 │ email         │       │ user_id (FK)   │       │ order_id (FK)  │
 │ password_hash │       │ status         │       │ product_id (FK)│
 │ role          │       │ total_amount   │       │ quantity       │
 └───────────────┘       └────────────────┘       └────────────────┘
                                                           │*
 ┌───────────────┐1     *┌────────────────┐                │1
 │  categories   │───────│    products    │────────────────┘
 ├───────────────┤       ├────────────────┤
 │ id (PK)       │       │ id (PK)        │
 │ name          │       │ category_id(FK)│
 │ slug          │       │ price, stock   │
 └───────────────┘       └────────────────┘
```

### Database Tables Schema (`SQLite / Postgres`)

```sql
-- Users & Roles
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'customer', -- 'customer', 'admin'
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Product Categories
CREATE TABLE categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    parent_id TEXT REFERENCES categories(id)
);

-- Products Catalog
CREATE TABLE products (
    id TEXT PRIMARY KEY,
    category_id TEXT NOT NULL REFERENCES categories(id),
    title TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    description TEXT NOT NULL,
    price REAL NOT NULL,
    sale_price REAL,
    stock_quantity INTEGER NOT NULL DEFAULT 0,
    rating_avg REAL DEFAULT 5.0,
    rating_count INTEGER DEFAULT 0,
    images_json TEXT NOT NULL, -- JSON array of image URLs
    is_featured INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Order Engine
CREATE TABLE orders (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending','processing','shipped','delivered','cancelled'
    total_amount REAL NOT NULL,
    coupon_code TEXT,
    shipping_address_json TEXT NOT NULL,
    payment_method TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Order Items
CREATE TABLE order_items (
    id TEXT PRIMARY KEY,
    order_id TEXT NOT NULL REFERENCES orders(id),
    product_id TEXT NOT NULL REFERENCES products(id),
    quantity INTEGER NOT NULL,
    unit_price REAL NOT NULL
);

-- Discount Coupons
CREATE TABLE coupons (
    id TEXT PRIMARY KEY,
    code TEXT UNIQUE NOT NULL,
    discount_percent REAL NOT NULL,
    max_uses INTEGER NOT NULL,
    current_uses INTEGER DEFAULT 0,
    valid_until TIMESTAMP NOT NULL
);
```

---

## 3. Comprehensive Epic & User Story Breakdown

### Epic 1: Modern Storefront UI & Glassmorphism Theme (`EPIC-EC-01`)
- **`US-EC-001`**: Responsive Navigation Bar with Category Mega-Menu, Cart Badge, Theme Switcher.
- **`US-EC-002`**: Hero Carousel & Featured Collections Showcase with smooth HSL gradient cards.
- **`US-EC-003`**: Footer with Newsletter Subscription & Currency/Language selector.

### Epic 2: Product Catalog, Search & Filtering (`EPIC-EC-02`)
- **`US-EC-004`**: Catalog Page with Multi-Attribute Filter Sidebar (Price range, Category, Rating, Stock).
- **`US-EC-005`**: Instant Search Bar with Live Autocomplete Suggestions (<50ms debounce).
- **`US-EC-006`**: Detailed Product View Page with Image Gallery, Stock Status, and Star Reviews.

### Epic 3: Real-Time Shopping Cart & Dynamic Checkout (`EPIC-EC-03`)
- **`US-EC-007`**: Slide-over Shopping Cart Drawer with quantity increment/decrement & optimistic UI.
- **`US-EC-008`**: Promo Coupon Code Application System (Percent vs Fixed Discount validation).
- **`US-EC-009`**: 3-Step Checkout Wizard (Shipping Address ➔ Payment Gateway ➔ Order Confirmation).

### Epic 4: Auth, Order Engine & State Machine (`EPIC-EC-04`)
- **`US-EC-010`**: Customer Authentication (JWT Token Register/Login & OAuth Social Login).
- **`US-EC-011`**: Order State Machine Engine (`Pending` ➔ `Processing` ➔ `Shipped` ➔ `Delivered`).
- **`US-EC-012`**: Customer Dashboard with Real-time Order Tracking Timeline & Invoice PDF Export.

---

## 4. API Endpoints Specification (RESTful Contract)

| Method | Endpoint | Description | Auth Required |
| :--- | :--- | :--- | :--- |
| `GET` | `/api/products` | Query catalog products with filter/sort params | No |
| `GET` | `/api/products/:slug` | Fetch product detail by slug | No |
| `GET` | `/api/search` | Search product autocomplete suggestions | No |
| `POST` | `/api/cart/coupon` | Validate & apply coupon code | No |
| `POST` | `/api/auth/register` | Register new customer account | No |
| `POST` | `/api/auth/login` | Authenticate customer & return JWT bearer | No |
| `POST` | `/api/orders` | Create new order from active cart | Yes (Bearer) |
| `GET` | `/api/orders/my-orders` | Fetch user order history | Yes (Bearer) |

---

## 5. Order Lifecycle State Machine

```text
  [ Client Submits Cart ]
            │
            ▼
      ┌───────────┐      Payment Fails / Cancel
      │  Pending  │──────────────────────────────────┐
      └─────┬─────┘                                  │
            │ Payment Confirmed                      │
            ▼                                        ▼
      ┌───────────┐                        ┌───────────────────┐
      │ Processing│                        │     Cancelled     │
      └─────┬─────┘                        └───────────────────┘
            │ Shipped to Courier                     ▲
            ▼                                        │ Refund Issued
      ┌───────────┐                                  │
      │  Shipped  │──────────────────────────────────┘
      └─────┬─────┘
            │ Customer Receives Package
            ▼
      ┌───────────┐
      │ Delivered │  (Order Completed)
      └───────────┘
```

---

## 6. Harness Verification Plan & Quality Proof Matrix

All user stories are registered in Harness SQLite `harness.db` and verified via automated commands:

```bash
# 1. Verify Catalog & Filtering Story
./scripts/harness story verify --id US-EC-001

# 2. Verify Cart & Coupon Story
./scripts/harness story verify --id US-EC-003

# 3. Audit codebase entropy score
./scripts/harness audit
```
