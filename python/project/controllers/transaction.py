from flask import jsonify, request
from models.transaction import Transaction

def get_transactions():
    transactions = Transaction.get_all()
    return jsonify([dict(row) for row in transactions])

def create_transaction():
    data = request.get_json()
    try:
        Transaction.create(data['account_id'], data['amount'], data['transaction_type'], data.get('description'))
        return '', 201
    except Exception as e:
        return '', 500

def update_transaction(id):
    data = request.get_json()
    try:
        Transaction.update(id, data['account_id'], data['amount'], data['transaction_type'], data.get('description'))
        return '', 200
    except Exception as e:
        return '', 500

def delete_transaction(id):
    try:
        Transaction.delete(id)
        return '', 200
    except Exception as e:
        return '', 500
