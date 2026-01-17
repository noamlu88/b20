from db import get_db

class Customer:
    @staticmethod
    def create_table():
        db = get_db()
        db.execute('''
            CREATE TABLE IF NOT EXISTS customers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                full_name TEXT NOT NULL,
                email TEXT NOT NULL,
                phone TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        ''')
        db.commit()

    @staticmethod
    def get_all():
        db = get_db()
        cursor = db.execute('SELECT * FROM customers')
        return cursor.fetchall()

    @staticmethod
    def create(full_name, email, phone):
        db = get_db()
        db.execute(
            'INSERT INTO customers (full_name, email, phone) VALUES (?, ?, ?)',
            (full_name, email, phone)
        )
        db.commit()

    @staticmethod
    def update(id, full_name, email, phone):
        db = get_db()
        db.execute(
            'UPDATE customers SET full_name=?, email=?, phone=? WHERE id=?',
            (full_name, email, phone, id)
        )
        db.commit()

    @staticmethod
    def delete(id):
        db = get_db()
        db.execute('DELETE FROM customers WHERE id=?', (id,))
        db.commit()
