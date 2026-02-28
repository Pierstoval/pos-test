# POS System — Specifications

## Overview

A lightweight Point of Sale desktop application for a convention booth, built with Tauri v2 (Rust backend) and Svelte 5. The system tracks sales of snacks and drinks. It does **not
** handle any payment processing.

Data is stored locally in SQLite (managed by Tauri). No remote database or internet connectivity is required.

---

## Features

### 1. Product Management

A screen to manage the catalog of products available for sale.

- Each product has: **name**, **price** (in cents, displayed as euros), **category** (`snack` or `drink`), an optional **emoji**, and an **available** boolean.
- Products can be added, edited, and toggled available/unavailable.
- Unavailable products do not appear on the sales screen.
- Products cannot be deleted (to preserve transaction history integrity), only marked unavailable.

### 2. Sales Screen (New Order)

The main screen used during the event to create and submit orders.

- Displays all **available** products as a grid of buttons, each showing the product name and price.
- Tapping a product adds one unit to the current order.
- The current order is displayed alongside the product grid, showing:
    - Each selected product with its name, unit price, quantity, and line subtotal.
    - The **total amount** for the order.
- For each item in the current order, the user can:
    - **Increase** the quantity (+1).
    - **Decrease** the quantity (-1, removing the line when it reaches 0).
- A **Checkout** button opens a checkout step where the user selects a payment method (**cash** or **card**).
    - For cash payments, the user can enter the amount received and see the **change** to return.
    - Confirming checkout records the order as a completed transaction (including the payment method).
    - After checkout, the current order is cleared and a new one can be started.
    - Payment method is recorded for reporting purposes only — no actual payment processing is involved.
- A **Clear** button resets the current order without recording anything.

### 3. Dashboard (Sales Summary)

A read-only screen displaying a summary of all completed transactions.

- **Per-product summary table** showing, for each product that has been sold at least once:
    - Product name.
    - Total quantity sold.
    - Total revenue (quantity x unit price at time of sale).
- **Grand total** of all sales revenue.
- **Breakdown by payment method** (cash vs. card).
- **Total number of transactions** completed.

### 4. Transaction Log

A screen (or section within the dashboard) listing all completed transactions.

- Each entry shows: timestamp, number of items, total amount, and payment method.
- Provides a way to **export** the full transaction log (e.g., as CSV or JSON).

### 5. Basic Inventory (Optional)

Optional stock tracking for products.

- Set a **starting stock** quantity per product.
- Stock decrements automatically on each sale.
- Display a **low-stock warning** when a product's remaining quantity falls below a configurable threshold.

---

## Data Model

### Product

| Field     | Type    | Description                                              |
|-----------|---------|----------------------------------------------------------|
| id        | string  | Unique identifier (UUID or similar)                      |
| name      | string  | Display name                                             |
| price     | number  | Price in cents (integer)                                 |
| category  | string  | `"snack"`, `"soft_drink"`; `"alcohol_drink"`, `"sweets"` |
| available | boolean | Whether the product is sellable                          |

### Order

| Field | Type     | Description                               |
|-------|----------|-------------------------------------------|
| id    | string   | Unique identifier (UUID or similar)       |
| items | relation | A list of OrderItem related to this order |
| total | number   | total value of this order                 |

### OrderItem

| Field       | Type   | Description                                                                  |
|-------------|--------|------------------------------------------------------------------------------|
| id          | string | Unique identifier (UUID or similar)                                          |
| productId   | string | Reference to a Product                                                       |
| productName | string | Product name snapshot at sale time                                           |
| unitPrice   | number | Price in cents snapshot at sale time                                         |
| quantity    | number | Number of units                                                              |
| total       | number | Total price of this Order Item (unit price multiplied by quantity, in cents) |

---

## Navigation

The app has three top-level routes:

| Route        | Screen             |
|--------------|--------------------|
| `/`          | Sales Screen       |
| `/products`  | Product Management |
| `/orders`    | Product Management |
| `/dashboard` | Dashboard          |

A persistent navigation bar allows switching between the three screens.

---

## Constraints

- All prices are stored and computed as **integers in cents** to avoid floating-point errors. Displayed as euros (e.g., `150` -> `1.50 EUR`).
- No authentication, no user accounts.
- No payment processing. Payment method (cash/card) is recorded for reporting only.
- No internet connectivity required for core functionality.
- The app is packaged as a desktop binary via Tauri. All data stays local.
- The UI must be usable on a tablet (large touch targets, readable fonts).
- Data must be stored in an SQLite database that is saved alongside the binary file. 
