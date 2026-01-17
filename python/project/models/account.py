from db import get_db

class Account:
    @staticmethod
    def create_table():
        db = get_db()
        db.execute('''
            CREATE TABLE IF NOT EXISTS accounts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                customer_id INTEGER NOT NULL,
                account_number TEXT NOT NULL,
                balance REAL NOT NULL,
                account_type TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (customer_id) REFERENCES customers (id)
            )
        ''')
        db.commit()

    @staticmethod
    def get_all():
        db = get_db()
        cursor = db.execute('SELECT * FROM accounts')
        return cursor.fetchall()

    @staticmethod
    def create(customer_id, account_number, balance, account_type):
        db = get_db()
        db.execute(
            'INSERT INTO accounts (customer_id, account_number, balance, account_type) VALUES (?, ?, ?, ?)',
            (customer_id, account_number, balance, account_type)
        )
        db.commit()

    @staticmethod
    def update(id, account_number, balance, account_type):
        db = get_db()
        db.execute(
            'UPDATE accounts SET account_number=?, balance=?, account_type=? WHERE id=?',
            (account_number, balance, account_type, id)
        )
        db.commit()

    @staticmethod
    def delete(id):
        db = get_db()
        db.execute('DELETE FROM accounts WHERE id=?', (id,))
        db.commit()
