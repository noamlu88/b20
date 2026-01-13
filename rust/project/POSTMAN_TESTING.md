### 1. Customers

**GET All Customers**
- **Method:** `GET`
- **URL:** `http://127.0.0.1:8080/customers`

**Create Customer**
- **Method:** `POST`
- **URL:** `http://127.0.0.1:8080/customers`
- **Body (JSON):**
  ```json
  {
    "full_name": "John Doe",
    "email": "john.doe@example.com",
    "phone": "+1234567890"
  }
  ```

**Update Customer**
- **Method:** `PUT`
- **URL:** `http://127.0.0.1:8080/customers/{id}` (Replace `{id}` with actual ID, e.g., `1`)
- **Body (JSON):**
  ```json
  {
    "full_name": "John Doe Updated",
    "email": "john.updated@example.com",
    "phone": "+0987654321"
  }
  ```

**Delete Customer**
- **Method:** `DELETE`
- **URL:** `http://127.0.0.1:8080/customers/{id}`

---

### 2. Accounts

**GET All Accounts**
- **Method:** `GET`
- **URL:** `http://127.0.0.1:8080/accounts`

**Create Account**
- **Method:** `POST`
- **URL:** `http://127.0.0.1:8080/accounts`
- **Body (JSON):**
  ```json
  {
    "customer_id": 1,
    "account_number": "ACC-1001",
    "balance": 100.50,
    "account_type": "Checking"
  }
  ```

**Update Account**
- **Method:** `PUT`
- **URL:** `http://127.0.0.1:8080/accounts/{id}`
- **Body (JSON):**
  ```json
  {
    "customer_id": 1,
    "account_number": "ACC-1001",
    "balance": 250.00,
    "account_type": "Savings"
  }
  ```

**Delete Account**
- **Method:** `DELETE`
- **URL:** `http://127.0.0.1:8080/accounts/{id}`

---

### 3. Transactions

**GET All Transactions**
- **Method:** `GET`
- **URL:** `http://127.0.0.1:8080/transactions`

**Create Transaction**
- **Method:** `POST`
- **URL:** `http://127.0.0.1:8080/transactions`
- **Body (JSON):**
  ```json
  {
    "account_id": 1,
    "amount": 50.00,
    "transaction_type": "deposit",
    "description": "Initial Deposit"
  }
  ```

**Update Transaction**
- **Method:** `PUT`
- **URL:** `http://127.0.0.1:8080/transactions/{id}`
- **Body (JSON):**
  ```json
  {
    "account_id": 1,
    "amount": 75.00,
    "transaction_type": "deposit",
    "description": "Updated Deposit Amount"
  }
  ```

**Delete Transaction**
- **Method:** `DELETE`
- **URL:** `http://127.0.0.1:8080/transactions/{id}`
