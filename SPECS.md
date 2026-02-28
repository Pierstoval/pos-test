# POS System — Specifications

## Overview

A lightweight Point of Sale system for a convention booth, built with SvelteKit and Svelte 5. The system tracks sales of snacks and drinks. It does **not** handle any payment processing.

Data is stored locally (localStorage or IndexedDB). No backend or remote database is required.

---

## Features

### 1. Product Management

A screen to manage the catalog of products available for sale.

- Each product has: **name**, **price** (in cents, displayed as euros), and an **available** boolean.
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
- A **Checkout** button validates and records the order as a completed transaction.
  - After checkout, the current order is cleared and a new one can be started.
  - No payment method selection or payment processing is involved.
- A **Clear** button resets the current order without recording anything.

### 3. Dashboard (Sales Summary)

A read-only screen displaying a summary of all completed transactions.

- **Per-product summary table** showing, for each product that has been sold at least once:
  - Product name.
  - Total quantity sold.
  - Total revenue (quantity x unit price at time of sale).
- **Grand total** of all sales revenue.
- **Total number of transactions** completed.

---

## Data Model

### Product

| Field       | Type    | Description                          |
|-------------|---------|--------------------------------------|
| id          | string  | Unique identifier (UUID or similar)  |
| name        | string  | Display name                         |
| price       | number  | Price in cents (integer)             |
| available   | boolean | Whether the product is sellable      |

### OrderItem

| Field       | Type    | Description                          |
|-------------|---------|--------------------------------------|
| productId   | string  | Reference to a Product               |
| productName | string  | Product name snapshot at sale time   |
| unitPrice   | number  | Price in cents snapshot at sale time  |
| quantity    | number  | Number of units                      |

### Transaction

| Field       | Type        | Description                        |
|-------------|-------------|------------------------------------|
| id          | string      | Unique identifier                  |
| timestamp   | string      | ISO 8601 date-time of the sale     |
| items       | OrderItem[] | List of items in the order         |
| total       | number      | Total amount in cents              |

---

## Navigation

The app has three top-level routes:

| Route          | Screen             |
|----------------|--------------------|
| `/`            | Sales Screen       |
| `/products`    | Product Management |
| `/dashboard`   | Dashboard          |

A persistent navigation bar allows switching between the three screens.

---

## Constraints

- All prices are stored and computed as **integers in cents** to avoid floating-point errors. Displayed as euros (e.g., `150` -> `1.50 EUR`).
- No authentication, no user accounts.
- No payment method tracking or payment processing.
- No internet connectivity required for core functionality.
- The UI must be usable on a tablet (large touch targets, readable fonts).
