from db import get_db

class Transaction:
    @staticmethod
    def create_table():
        db = get_db()
        db.execute('''
            CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                account_id INTEGER NOT NULL,
                amount REAL NOT NULL,
                transaction_type TEXT NOT NULL,
                description TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (account_id) REFERENCES accounts (id)
            )
        ''')
        db.commit()

    @staticmethod
    def get_all():
        db = get_db()
        cursor = db.execute('SELECT * FROM transactions')
        return cursor.fetchall()

    @staticmethod
    def create(account_id, amount, transaction_type, description=None):
        db = get_db()
        db.execute(
            'INSERT INTO transactions (account_id, amount, transaction_type, description) VALUES (?, ?, ?, ?)',
            (account_id, amount, transaction_type, description)
        )
        db.commit()

    @staticmethod
    def update(id, account_id, amount, transaction_type, description=None):
        db = get_db()
        db.execute(
            'UPDATE transactions SET account_id=?, amount=?, transaction_type=?, description=? WHERE id=?',
            (account_id, amount, transaction_type, description, id)
        )
        db.commit()

    @staticmethod
    def delete(id):
        db = get_db()
        db.execute('DELETE FROM transactions WHERE id=?', (id,))
        db.commit()
